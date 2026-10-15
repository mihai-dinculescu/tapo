use aes::Aes128;
use aes::cipher::{BlockModeDecrypt, BlockModeEncrypt, KeyIvInit, block_padding};
use base64::{Engine as _, engine::general_purpose};
use cbc::{Decryptor, Encryptor};

pub fn sha1(data: &[u8]) -> [u8; 20] {
    use sha1::{Digest, Sha1};
    let mut hasher = Sha1::new();
    hasher.update(data);
    hasher.finalize().into()
}

pub fn sha1_hex_lower(data: &[u8]) -> String {
    base16ct::lower::encode_string(&sha1(data))
}

pub fn sha256(data: &[u8]) -> [u8; 32] {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.finalize().into()
}

pub fn sha256_hex(data: &[u8]) -> String {
    base16ct::upper::encode_string(&sha256(data))
}

pub fn md5(data: &[u8]) -> [u8; 16] {
    use md5::{Digest, Md5};
    let mut hasher = Md5::new();
    hasher.update(data);
    hasher.finalize().into()
}

pub fn md5_hex(data: &[u8]) -> String {
    base16ct::upper::encode_string(&md5(data))
}

pub fn md5_hex_lower(data: &[u8]) -> String {
    base16ct::lower::encode_string(&md5(data))
}

/// The SHA-256-crypt (`$5$`) hash of `password` with `salt` and `rounds`
/// (5000 when `None`), as the `$5$salt$hash` string of crypt(3), which names
/// `rounds` only when it is not the default.
pub fn sha256_crypt(password: &[u8], salt: &str, rounds: Option<u32>) -> anyhow::Result<String> {
    const ALPHABET: &[u8; 64] = b"./0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
    // The bytes of the hash that crypt(3) encodes together, in its order.
    const GROUPS: [[usize; 3]; 10] = [
        [0, 10, 20],
        [21, 1, 11],
        [12, 22, 2],
        [3, 13, 23],
        [24, 4, 14],
        [15, 25, 5],
        [6, 16, 26],
        [27, 7, 17],
        [18, 28, 8],
        [9, 19, 29],
    ];

    let rounds = rounds.unwrap_or(sha_crypt::Params::RECOMMENDED_ROUNDS);
    // crypt(3) uses at most 16 characters of the salt.
    let salt = salt.get(..16).unwrap_or(salt);
    let params = sha_crypt::Params::new(rounds)
        .map_err(|e| anyhow::anyhow!("Invalid SHA-256-crypt rounds {rounds}: {e:?}"))?;
    let hash = sha_crypt::sha256_crypt(password, salt.as_bytes(), params);

    let mut encoded = String::with_capacity(43);
    let mut push = |value: u32, chars: usize| {
        for i in 0..chars {
            encoded.push(ALPHABET[((value >> (6 * i)) & 0x3f) as usize] as char);
        }
    };
    for [a, b, c] in GROUPS {
        push(
            (u32::from(hash[a]) << 16) | (u32::from(hash[b]) << 8) | u32::from(hash[c]),
            4,
        );
    }
    push((u32::from(hash[31]) << 8) | u32::from(hash[30]), 3);

    Ok(if rounds == sha_crypt::Params::RECOMMENDED_ROUNDS {
        format!("$5${salt}${encoded}")
    } else {
        format!("$5$rounds={rounds}${salt}${encoded}")
    })
}

pub fn aes128_cbc_encrypt(key: &[u8], iv: &[u8], data: &str) -> anyhow::Result<String> {
    let encryptor = Encryptor::<Aes128>::new_from_slices(key, iv)?;
    let cipher_bytes = encryptor.encrypt_padded_vec::<block_padding::Pkcs7>(data.as_bytes());
    Ok(general_purpose::STANDARD.encode(cipher_bytes))
}

pub fn aes128_cbc_decrypt(key: &[u8], iv: &[u8], cipher_base64: &str) -> anyhow::Result<String> {
    let cipher_bytes = general_purpose::STANDARD.decode(cipher_base64)?;
    let decrypted_bytes = aes128_cbc_decrypt_bytes(key, iv, &cipher_bytes)?;
    Ok(std::str::from_utf8(&decrypted_bytes)?.to_string())
}

pub fn aes128_cbc_decrypt_bytes(key: &[u8], iv: &[u8], data: &[u8]) -> anyhow::Result<Vec<u8>> {
    let decryptor = Decryptor::<Aes128>::new_from_slices(key, iv)?;
    decryptor
        .decrypt_padded_vec::<block_padding::Pkcs7>(data)
        .map_err(|e| anyhow::anyhow!("Decryption error: {:?}", e))
}

pub fn hmac_sha256(key: &[u8], data: &[u8]) -> [u8; 32] {
    use hmac::{Hmac, KeyInit, Mac};
    use sha2::Sha256;
    let mut mac = Hmac::<Sha256>::new_from_slice(key).expect("HMAC takes a key of any size");
    mac.update(data);
    mac.finalize().into_bytes().into()
}

/// Whether `tag` is the HMAC-SHA256 of `data` under `key`, compared in
/// constant time.
pub fn hmac_sha256_verify(key: &[u8], data: &[u8], tag: &[u8]) -> bool {
    use hmac::{Hmac, KeyInit, Mac};
    use sha2::Sha256;
    let mut mac = Hmac::<Sha256>::new_from_slice(key).expect("HMAC takes a key of any size");
    mac.update(data);
    mac.verify_slice(tag).is_ok()
}

/// HKDF with HMAC-SHA256 (RFC 5869): extract with `salt`, then expand with
/// `info` to `length` bytes, which RFC 5869 caps at 255 * 32.
pub fn hkdf_sha256(ikm: &[u8], salt: &[u8], info: &[u8], length: usize) -> anyhow::Result<Vec<u8>> {
    use hkdf::Hkdf;
    use sha2::Sha256;
    let mut okm = vec![0u8; length];
    Hkdf::<Sha256>::new(Some(salt), ikm)
        .expand(info, &mut okm)
        .map_err(|e| anyhow::anyhow!("HKDF-SHA256 cannot expand to {length} bytes: {e}"))?;
    Ok(okm)
}

/// PBKDF2 with HMAC-SHA256 (RFC 8018), `length` bytes of output.
pub fn pbkdf2_hmac_sha256(password: &[u8], salt: &[u8], iterations: u32, length: usize) -> Vec<u8> {
    use sha2::Sha256;
    let mut key = vec![0u8; length];
    pbkdf2::pbkdf2_hmac::<Sha256>(password, salt, iterations, &mut key);
    key
}

#[cfg(test)]
mod tests {
    use super::*;

    // The test vectors of the SHA-crypt specification.
    #[test]
    fn test_sha256_crypt() {
        assert_eq!(
            sha256_crypt(b"Hello world!", "saltstring", None).unwrap(),
            "$5$saltstring$5B8vYYiY.CVt1RlTTf8KbXBH3hsxY/GNooZaBBGWEc5"
        );
        assert_eq!(
            sha256_crypt(b"Hello world!", "saltstringsaltstring", Some(10000)).unwrap(),
            "$5$rounds=10000$saltstringsaltst$3xv.VbSHBb41AL9AvLeujZkZRBAwqFMz2.opqey6IcA"
        );
    }

    #[test]
    fn test_aes128_cbc_decrypt_bytes_round_trip() {
        let key = [0x11; 16];
        let iv = [0x22; 16];
        let encrypted = aes128_cbc_encrypt(&key, &iv, "hello, hub").unwrap();
        let cipher_bytes = general_purpose::STANDARD.decode(encrypted).unwrap();

        let decrypted = aes128_cbc_decrypt_bytes(&key, &iv, &cipher_bytes).unwrap();
        assert_eq!(decrypted, b"hello, hub");

        assert!(aes128_cbc_decrypt_bytes(&[0x33; 16], &iv, &cipher_bytes).is_err());
    }
}
