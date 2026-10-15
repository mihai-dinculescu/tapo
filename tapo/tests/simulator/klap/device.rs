//! The handshake and the sessions of a device in KLAP mode.

use std::collections::HashMap;
use std::sync::{Mutex, MutexGuard};

use rand::RngExt as _;
use serde_json::{Value, json};

use crate::simulator::fixture::Fixture;
use crate::simulator::{self, crypto};

use super::cipher::Cipher;

pub struct Klap {
    fixture: Fixture,
    /// What both sides hash the account to.
    auth_hash: [u8; 32],
    /// The sessions, by the value of their `TP_SESSIONID` cookie.
    sessions: Mutex<HashMap<String, Session>>,
}

struct Session {
    local_seed: Vec<u8>,
    remote_seed: [u8; 16],
    cipher: Cipher,
    /// The sequence number that the next request has to be sent under.
    next_seq: i32,
    /// Whether `handshake2` has proved that the client knows the account.
    confirmed: bool,
}

impl Klap {
    pub fn new(username: &str, password: &str, fixture: Fixture) -> Self {
        let auth_hash = crypto::sha256(
            &[
                crypto::sha1(username.as_bytes()),
                crypto::sha1(password.as_bytes()),
            ]
            .concat(),
        );

        Self {
            fixture,
            auth_hash,
            sessions: Mutex::default(),
        }
    }

    /// Answers the `discover` login request, with which a client asks
    /// whether the device is in TPAP mode.
    pub fn discover(&self) -> Value {
        let result = self
            .fixture
            .discover_result()
            .expect("the fixture has a result for the discover login request");
        json!({ "error_code": 0, "result": result })
    }

    /// Opens a session for the client that sent `local_seed`: its cookie, and
    /// the device's seed followed by the device's proof that it knows the
    /// account. A client of another account sees that the proof is not what
    /// it computes itself. A device holds one handshake at a time, so a
    /// session that a client abandoned before `handshake2` is forgotten.
    pub fn handshake1(&self, local_seed: &[u8]) -> (String, Vec<u8>) {
        assert_eq!(local_seed.len(), 16, "`handshake1` carries a 16-byte seed");

        let remote_seed: [u8; 16] = rand::rng().random();
        let proof = crypto::sha256(&[local_seed, &remote_seed, &self.auth_hash].concat());
        let cookie = base16ct::upper::encode_string(&rand::rng().random::<[u8; 16]>());

        let cipher = Cipher::new(local_seed, &remote_seed, &self.auth_hash);
        let session = Session {
            local_seed: local_seed.to_vec(),
            remote_seed,
            next_seq: cipher.start_seq().wrapping_add(1),
            cipher,
            confirmed: false,
        };
        let mut sessions = self.sessions();
        sessions.retain(|_, session| session.confirmed);
        sessions.insert(cookie.clone(), session);

        (cookie, [remote_seed.as_slice(), &proof].concat())
    }

    /// Whether the client's `proof` that it knows the account holds. A
    /// device answers a proof that does not with 403 and forgets the
    /// session, so a request on it afterwards is one on an unknown session.
    pub fn handshake2(&self, cookie: &str, proof: &[u8]) -> bool {
        let mut sessions = self.sessions();
        let session = sessions
            .get_mut(cookie)
            .unwrap_or_else(|| panic!("`handshake2` on the unknown session {cookie}"));

        let expected = crypto::sha256(
            &[
                session.remote_seed.as_slice(),
                &session.local_seed,
                &self.auth_hash,
            ]
            .concat(),
        );

        let holds = expected == proof;
        if holds {
            session.confirmed = true;
        } else {
            sessions.remove(cookie);
        }

        holds
    }

    /// Answers the encrypted request `body` of the session with `cookie`, sent
    /// under `seq`, encrypted in turn. A device serves a session only once
    /// `handshake2` has confirmed it, and only a request sent under the
    /// sequence number that follows the previous one.
    pub fn request(&self, cookie: &str, seq: i32, body: &[u8]) -> Vec<u8> {
        let mut sessions = self.sessions();
        let session = sessions
            .get_mut(cookie)
            .unwrap_or_else(|| panic!("a request on the unknown session {cookie}"));
        assert!(
            session.confirmed,
            "a request on the session {cookie} before `handshake2` confirmed it"
        );
        assert_eq!(
            seq, session.next_seq,
            "a request on the session {cookie} is sent under the next sequence number"
        );
        session.next_seq = seq.wrapping_add(1);
        let cipher = &session.cipher;

        let request = cipher
            .decrypt(seq, body)
            .expect("a session request is signed and encrypted under the keys of its session");
        let request: Value = serde_json::from_str(&request).expect("a session request is JSON");
        let method = request["method"]
            .as_str()
            .expect("a session request names its method");

        let response = json!({ "error_code": 0, "result": self.fixture.result(method) });

        cipher.encrypt(seq, &response.to_string())
    }

    fn sessions(&self) -> MutexGuard<'_, HashMap<String, Session>> {
        simulator::lock(&self.sessions)
    }
}
