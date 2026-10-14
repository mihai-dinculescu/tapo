//! The device side of the request cipher of a TPAP session: AES-128-CCM under
//! a key and a nonce that HKDF derives from the shared key of the login. The
//! nonce ends in the sequence number of the request, which the response is
//! encrypted under as well.

use aes::Aes128;
use ccm::Ccm;
use ccm::aead::{Aead, KeyInit};
use ccm::consts::{U12, U16};

use crate::simulator::crypto;

pub struct Cipher {
    cipher: Ccm<Aes128, U16, U12>,
    base_nonce: Vec<u8>,
}

impl Cipher {
    pub fn new(shared_key: &[u8]) -> Self {
        let key = crypto::hkdf_sha256(
            shared_key,
            b"tp-kdf-salt-aes128-key",
            b"tp-kdf-info-aes128-key",
            16,
        );
        let base_nonce = crypto::hkdf_sha256(
            shared_key,
            b"tp-kdf-salt-aes128-iv",
            b"tp-kdf-info-aes128-iv",
            12,
        );

        Self {
            cipher: Ccm::new_from_slice(&key).expect("the key is 16 bytes"),
            base_nonce,
        }
    }

    /// The sequence number and the plaintext of the request `body`, unless
    /// it does not decrypt.
    pub fn decrypt(&self, body: &[u8]) -> Option<(i32, String)> {
        let (seq, ciphertext) = body.split_first_chunk::<4>()?;
        let seq = i32::from_be_bytes(*seq);

        let plaintext = self
            .cipher
            .decrypt(&self.nonce(seq).into(), ciphertext)
            .ok()?;

        Some((seq, String::from_utf8(plaintext).ok()?))
    }

    /// Encrypts the response to the request that was sent under `seq`.
    pub fn encrypt(&self, seq: i32, plaintext: &str) -> Vec<u8> {
        let ciphertext = self
            .cipher
            .encrypt(&self.nonce(seq).into(), plaintext.as_bytes())
            .expect("AES-CCM encrypts any message");

        [&seq.to_be_bytes(), ciphertext.as_slice()].concat()
    }

    fn nonce(&self, seq: i32) -> [u8; 12] {
        let mut nonce = [0u8; 12];
        nonce[..8].copy_from_slice(&self.base_nonce[..8]);
        nonce[8..].copy_from_slice(&seq.to_be_bytes());
        nonce
    }
}
