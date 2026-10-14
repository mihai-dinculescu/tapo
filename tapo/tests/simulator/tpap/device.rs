//! The login and the sessions of a device in TPAP mode.

use std::collections::HashMap;
use std::sync::Mutex;

use base64::{Engine as _, engine::general_purpose::STANDARD};
use rand::RngExt as _;
use serde::Deserialize;
use serde_json::{Value, json};

use crate::simulator::crypto;
use crate::simulator::fixture::Fixture;

use super::cipher::Cipher;
use super::spake2p::Verifier;

/// What a P110 asks for in `pake_register`.
const ITERATIONS: u32 = 3000;
/// What a P110 grants a session, in seconds.
const SESSION_LIFETIME: u64 = 604_800;
/// What a plug answers `pake_share` with when the password is wrong.
const WRONG_PASSWORD: i64 = -2203;

pub struct Tpap {
    fixture: Fixture,
    /// The SHA-1 of the password, which is what a plug asks the client to
    /// prove it knows (`extra_crypt` of `password_shadow` id 2).
    credential: String,
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
        Self {
            fixture,
            credential: crypto::sha1_hex(password.as_bytes()),
            salt: rand::rng().random(),
            state: Mutex::default(),
        }
    }

    /// Answers the `login` request `body` with the body of the response.
    pub fn login(&self, body: &[u8]) -> Value {
        let request: LoginRequest =
            serde_json::from_slice(body).expect("a login request that the simulator knows");

        let result = match request.params {
            Login::Discover => Ok(self.fixture.discover_result().clone()),
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

        match result {
            Ok(result) => json!({ "error_code": 0, "result": result }),
            Err(error_code) => json!({ "error_code": error_code }),
        }
    }

    /// Answers the encrypted request `body` of the session `token`, encrypted
    /// in turn. A device serves only a request sent under the sequence number
    /// that follows the previous one, starting at the `start_seq` of the
    /// login.
    pub fn request(&self, token: &str, body: &[u8]) -> Vec<u8> {
        let mut state = self.state.lock().unwrap();
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
        let method = request["method"]
            .as_str()
            .expect("a session request names its method");

        let response = json!({ "error_code": 0, "result": self.fixture.result(method) });

        session.cipher.encrypt(seq, &response.to_string())
    }

    fn pake_register(&self, username: &str, user_random: &str, passcode_type: &str) -> Value {
        assert_eq!(
            username,
            crypto::md5_hex(b"admin"),
            "TPAP logs in as the `admin` user"
        );
        assert_eq!(passcode_type, "userpw", "a plug takes the account password");

        let device_random: [u8; 32] = rand::rng().random();
        let verifier = Verifier::new(
            &self.credential,
            &self.salt,
            ITERATIONS,
            &decode(user_random),
            &device_random,
        );

        let result = json!({
            "sub_method": "pake_register",
            "dev_random": STANDARD.encode(device_random),
            "dev_salt": STANDARD.encode(self.salt),
            "dev_share": STANDARD.encode(verifier.share()),
            "cipher_suites": 1,
            "iterations": ITERATIONS,
            "encryption": "aes_128_ccm",
            "extra_crypt": { "type": "password_shadow", "params": { "passwd_id": 2 } },
        });

        self.state.lock().unwrap().pending = Some(verifier);

        result
    }

    fn pake_share(&self, user_share: &str, user_confirm: &str) -> Result<Value, i64> {
        let mut state = self.state.lock().unwrap();
        let verifier = state
            .pending
            .take()
            .expect("`pake_share` follows a `pake_register`");

        let (user_share, user_confirm) = (decode(user_share), decode(user_confirm));
        let (confirm, shared_key) = verifier
            .confirm(&user_share, &user_confirm)
            .ok_or(WRONG_PASSWORD)?;

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

fn decode(base64: &str) -> Vec<u8> {
    STANDARD
        .decode(base64)
        .unwrap_or_else(|_| panic!("the client sends base64: {base64}"))
}

#[derive(Deserialize)]
struct LoginRequest {
    params: Login,
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
