//! The login and the sessions of a device in TPAP mode. The fixture says
//! what the device asks for in `pake_register`, and with it which transform
//! of the password the client has to prove it knows, and what the device
//! answers a wrong password with. A camera takes its methods inside a
//! `multipleRequest`.

use std::collections::HashMap;
use std::sync::{Mutex, MutexGuard};

use base64::{Engine as _, engine::general_purpose::STANDARD};
use rand::RngExt as _;
use serde::Deserialize;
use serde_json::{Value, json};

use crate::simulator::fixture::Fixture;
use crate::simulator::{self, crypto};

use super::cipher::Cipher;
use super::spake2p::Verifier;

/// What the device grants a session, in seconds: what a P110 grants. No
/// fixture records what a camera grants.
const SESSION_LIFETIME: u64 = 604_800;

pub struct Tpap {
    fixture: Fixture,
    /// The transform of the password that the device asks the client to
    /// prove it knows.
    credential: String,
    /// The error code that `pake_share` answers a wrong password with.
    wrong_password_error_code: i64,
    salt: [u8; 16],
    state: Mutex<State>,
}

#[derive(Default)]
struct State {
    /// The handshake that has registered and not yet shared. A device holds
    /// one at a time: the next `pake_register` replaces it, and the
    /// `pake_share` that follows consumes it whether or not the password
    /// holds.
    pending: Option<Verifier>,
    /// The sessions that have logged in, by token.
    sessions: HashMap<String, Session>,
}

struct Session {
    cipher: Cipher,
    /// The sequence number that the next request has to be sent under.
    next_seq: i32,
}

impl Tpap {
    pub fn new(password: &str, fixture: Fixture) -> Self {
        let credential = credential(
            password,
            &fixture.pake_register_result()["extra_crypt"],
            fixture.is_camera(),
        );
        let wrong_password_error_code = fixture.wrong_password_error_code();

        Self {
            fixture,
            credential,
            wrong_password_error_code,
            salt: rand::rng().random(),
            state: Mutex::default(),
        }
    }

    /// Answers the `login` request `body` with the body of the response, or
    /// with the reason that the simulator cannot answer it: a body that is
    /// not a login it knows, a login of AES SSL, which names no `sub_method`,
    /// unlike a TPAP login, or a `discover` sent to a camera, which the
    /// fixture of one has no result for.
    pub fn login(&self, body: &[u8]) -> Result<Value, String> {
        let request: Value = serde_json::from_slice(body)
            .map_err(|error| format!("a login request that is not JSON: {error}"))?;
        let params = request
            .get("params")
            .ok_or_else(|| format!("a login request without its params: {request}"))?;
        if params.get("sub_method").is_none() {
            return Err("the simulator does not speak AES SSL".to_string());
        }
        let login = Login::deserialize(params)
            .map_err(|error| format!("a TPAP login that the simulator does not know: {error}"))?;

        let result = match login {
            Login::Discover => Ok(self
                .fixture
                .discover_result()
                .ok_or("the simulator does not answer `discover` for a camera")?
                .clone()),
            Login::PakeRegister {
                username,
                user_random,
                passcode_type,
            } => Ok(self.pake_register(&username, &user_random, &passcode_type)),
            Login::PakeShare {
                user_share,
                user_confirm,
            } => self.pake_share(&user_share, &user_confirm),
        };

        Ok(match result {
            Ok(result) => json!({ "error_code": 0, "result": result }),
            Err(error_code) => json!({ "error_code": error_code }),
        })
    }

    /// Answers the encrypted request `body` of the session `token`, encrypted
    /// in turn. A device serves only a request sent under the sequence number
    /// that follows the previous one, starting at the `start_seq` of the
    /// login. A request, and each request of a `multipleRequest`, is
    /// answered with its result; for a method that the fixture lacks, the
    /// simulator panics.
    pub fn request(&self, token: &str, body: &[u8]) -> Vec<u8> {
        let mut state = self.state();
        let session = state
            .sessions
            .get_mut(token)
            .unwrap_or_else(|| panic!("a request on the unknown session {token}"));

        let (seq, request) = session
            .cipher
            .decrypt(body)
            .expect("a session request decrypts under the key of its session");
        assert_eq!(
            seq, session.next_seq,
            "a request on the session {token} is sent under the next sequence number"
        );
        session.next_seq = seq.wrapping_add(1);
        let request: Value = serde_json::from_str(&request).expect("a session request is JSON");

        let method = method_of(&request);
        let response = if method == "multipleRequest" {
            let responses: Vec<Value> = request["params"]["requests"]
                .as_array()
                .expect("a `multipleRequest` carries its requests")
                .iter()
                .map(|request| {
                    let method = method_of(request);
                    let mut response = self.response(method);
                    response["method"] = json!(method);
                    response
                })
                .collect();

            json!({ "error_code": 0, "result": { "responses": responses } })
        } else {
            self.response(method)
        };

        session.cipher.encrypt(seq, &response.to_string())
    }

    /// The response to a request for `method`: its result.
    fn response(&self, method: &str) -> Value {
        json!({ "error_code": 0, "result": self.fixture.result(method) })
    }

    fn state(&self) -> MutexGuard<'_, State> {
        simulator::lock(&self.state)
    }

    fn pake_register(&self, username: &str, user_random: &str, passcode_type: &str) -> Value {
        assert_eq!(
            username,
            crypto::md5_hex(b"admin"),
            "TPAP logs in as the `admin` user"
        );
        assert_eq!(
            passcode_type, "userpw",
            "the device takes the account password"
        );

        let mut result = self.fixture.pake_register_result().clone();
        let iterations = result["iterations"]
            .as_u64()
            .and_then(|iterations| u32::try_from(iterations).ok())
            .expect("the fixture asks for a number of PBKDF2 iterations");
        assert!(
            result["cipher_suites"] == 1 && result["encryption"] == "aes_128_ccm",
            "the simulator serves only cipher suite 1 with aes_128_ccm, not {} with {}",
            result["cipher_suites"],
            result["encryption"]
        );

        let device_random: [u8; 32] = rand::rng().random();
        let verifier = Verifier::new(
            &self.credential,
            &self.salt,
            iterations,
            &decode(user_random),
            &device_random,
        );

        result["dev_random"] = json!(STANDARD.encode(device_random));
        result["dev_salt"] = json!(STANDARD.encode(self.salt));
        result["dev_share"] = json!(STANDARD.encode(verifier.share()));

        self.state().pending = Some(verifier);

        result
    }

    fn pake_share(&self, user_share: &str, user_confirm: &str) -> Result<Value, i64> {
        let mut state = self.state();
        let verifier = state
            .pending
            .take()
            .expect("`pake_share` follows a `pake_register`");

        let (user_share, user_confirm) = (decode(user_share), decode(user_confirm));
        let (confirm, shared_key) = verifier
            .confirm(&user_share, &user_confirm)
            .ok_or(self.wrong_password_error_code)?;

        let token = base16ct::lower::encode_string(&rand::rng().random::<[u8; 16]>());
        let start_seq: i32 = rand::rng().random();
        let session = Session {
            cipher: Cipher::new(&shared_key),
            next_seq: start_seq,
        };
        state.sessions.insert(token.clone(), session);

        Ok(json!({
            "dev_confirm": STANDARD.encode(confirm),
            "stok": token,
            "start_seq": start_seq,
            "expired": SESSION_LIFETIME,
        }))
    }
}

/// The transform of `password` that the device holds and asks the client to
/// prove it knows. A plug or a bulb names it in the `extra_crypt` of
/// `pake_register`, and a P110 or an L535 that names `password_shadow` id 2
/// holds the SHA-1 of the password. A camera names none and holds the MD5 of
/// the password, as a C220 does. A device whose fixture lives in
/// `fixtures/smartcam/` is a camera.
fn credential(password: &str, extra_crypt: &Value, is_camera: bool) -> String {
    match extra_crypt {
        Value::Null if is_camera => crypto::md5_hex(password.as_bytes()),
        Value::Null => panic!(
            "the fixture of a device that is not a camera names the password transform in `extra_crypt`"
        ),
        extra_crypt
            if extra_crypt["type"] == "password_shadow"
                && extra_crypt["params"]["passwd_id"] == 2 =>
        {
            crypto::sha1_hex(password.as_bytes())
        }
        extra_crypt => unimplemented!("the simulator does not know the transform {extra_crypt}"),
    }
}

fn decode(base64: &str) -> Vec<u8> {
    STANDARD
        .decode(base64)
        .unwrap_or_else(|_| panic!("the client sends base64: {base64}"))
}

fn method_of(request: &Value) -> &str {
    request["method"]
        .as_str()
        .unwrap_or_else(|| panic!("a request names its method: {request}"))
}

#[derive(Deserialize)]
#[serde(tag = "sub_method", rename_all = "snake_case")]
enum Login {
    Discover,
    PakeRegister {
        username: String,
        user_random: String,
        passcode_type: String,
    },
    PakeShare {
        user_share: String,
        user_confirm: String,
    },
}
