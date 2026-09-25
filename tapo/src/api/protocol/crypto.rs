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

pub fn sha256(data: &[u8]) -> [u8; 32] {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.finalize().into()
}

pub fn sha256_hex(data: &[u8]) -> String {
    base16ct::upper::encode_string(&sha256(data))
}

pub fn md5_hex(data: &[u8]) -> String {
    use md5::Digest;
    let mut hasher = md5::Md5::new();
    hasher.update(data);
    let hash = hasher.finalize();
    base16ct::upper::encode_string(&hash)
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

#[cfg(test)]
mod tests {
    use super::*;

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
