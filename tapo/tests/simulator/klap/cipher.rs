//! The device side of the request cipher of a KLAP session: AES-128-CBC under
//! a key, an IV and a signing key that are all hashed out of the two seeds of
//! the handshake and the account hash. The hash of the IV ends in the sequence
//! number that the requests of the session count up from. The IV ends in the
//! sequence number of the request, which the response is encrypted under as
//! well, and every body starts with a signature over its sequence number and
//! ciphertext.

use aes::Aes128;
use aes::cipher::{BlockModeDecrypt, BlockModeEncrypt, KeyIvInit, block_padding::Pkcs7};

use crate::simulator::crypto;

pub struct Cipher {
    key: [u8; 16],
    iv: [u8; 12],
    start_seq: i32,
    signing_key: [u8; 28],
}

impl Cipher {
    pub fn new(local_seed: &[u8], remote_seed: &[u8], auth_hash: &[u8]) -> Self {
        let derive =
            |label: &[u8]| crypto::sha256(&[label, local_seed, remote_seed, auth_hash].concat());
        let iv_hash = derive(b"iv");

        Self {
            key: derive(b"lsk")[..16].try_into().expect("16 bytes"),
            iv: iv_hash[..12].try_into().expect("12 bytes"),
            start_seq: i32::from_be_bytes(iv_hash[28..].try_into().expect("4 bytes")),
            signing_key: derive(b"ldk")[..28].try_into().expect("28 bytes"),
        }
    }

    /// The sequence number that the handshake derives. The first request of
    /// the session is sent under the one after it.
    pub fn start_seq(&self) -> i32 {
        self.start_seq
    }

    /// The plaintext of the request `body` sent under `seq`, unless its
    /// signature or its ciphertext is not what the session's keys make of it.
    pub fn decrypt(&self, seq: i32, body: &[u8]) -> Option<String> {
        let (signature, ciphertext) = body.split_at_checked(32)?;
        if self.sign(seq, ciphertext) != signature {
            return None;
        }

        let plaintext = cbc::Decryptor::<Aes128>::new(&self.key.into(), &self.iv(seq).into())
            .decrypt_padded_vec::<Pkcs7>(ciphertext)
            .ok()?;

        String::from_utf8(plaintext).ok()
    }

    /// Encrypts the response to the request that was sent under `seq`.
    pub fn encrypt(&self, seq: i32, plaintext: &str) -> Vec<u8> {
        let ciphertext = cbc::Encryptor::<Aes128>::new(&self.key.into(), &self.iv(seq).into())
            .encrypt_padded_vec::<Pkcs7>(plaintext.as_bytes());

        [&self.sign(seq, &ciphertext), ciphertext.as_slice()].concat()
    }

    fn sign(&self, seq: i32, ciphertext: &[u8]) -> [u8; 32] {
        crypto::sha256(&[&self.signing_key, seq.to_be_bytes().as_slice(), ciphertext].concat())
    }

    fn iv(&self, seq: i32) -> [u8; 16] {
        let mut iv = [0u8; 16];
        iv[..12].copy_from_slice(&self.iv);
        iv[12..].copy_from_slice(&seq.to_be_bytes());
        iv
    }
}
