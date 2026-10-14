//! The hashes and key derivations of the logins and ciphers.

use hmac::{Hmac, KeyInit, Mac};
use sha2::{Digest, Sha256};

pub fn md5_hex(data: &[u8]) -> String {
    base16ct::lower::encode_string(&md5::Md5::digest(data))
}

pub fn sha1(data: &[u8]) -> [u8; 20] {
    sha1::Sha1::digest(data).into()
}

pub fn sha1_hex(data: &[u8]) -> String {
    base16ct::lower::encode_string(&sha1(data))
}

pub fn sha256(data: &[u8]) -> [u8; 32] {
    Sha256::digest(data).into()
}

pub fn hmac_sha256(key: &[u8], data: &[u8]) -> [u8; 32] {
    let mut mac = Hmac::<Sha256>::new_from_slice(key).expect("HMAC takes a key of any size");
    mac.update(data);
    mac.finalize().into_bytes().into()
}

/// HKDF with HMAC-SHA256 (RFC 5869): extract with `salt`, then expand with
/// `info` to `length` bytes.
pub fn hkdf_sha256(ikm: &[u8], salt: &[u8], info: &[u8], length: usize) -> Vec<u8> {
    let mut okm = vec![0u8; length];
    hkdf::Hkdf::<Sha256>::new(Some(salt), ikm)
        .expand(info, &mut okm)
        .expect("HKDF-SHA256 expands to at most 255 * 32 bytes");
    okm
}

/// PBKDF2 with HMAC-SHA256 (RFC 8018), `length` bytes of output.
pub fn pbkdf2_hmac_sha256(password: &[u8], salt: &[u8], iterations: u32, length: usize) -> Vec<u8> {
    let mut key = vec![0u8; length];
    pbkdf2::pbkdf2_hmac::<Sha256>(password, salt, iterations, &mut key);
    key
}
