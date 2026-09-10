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

pub fn md5(data: &[u8]) -> [u8; 16] {
    use md5::Digest;
    let mut hasher = md5::Md5::new();
    hasher.update(data);
    hasher.finalize().into()
}

pub fn md5_hex(data: &[u8]) -> String {
    base16ct::upper::encode_string(&md5(data))
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

/// HMAC-SHA256 (RFC 2104).
#[cfg_attr(not(feature = "debug"), allow(dead_code))]
pub fn hmac_sha256(key: &[u8], data: &[u8]) -> [u8; 32] {
    const BLOCK_SIZE: usize = 64;

    let mut block = [0u8; BLOCK_SIZE];
    if key.len() > BLOCK_SIZE {
        block[..32].copy_from_slice(&sha256(key));
    } else {
        block[..key.len()].copy_from_slice(key);
    }

    let mut inner = Vec::with_capacity(BLOCK_SIZE + data.len());
    inner.extend(block.iter().map(|byte| byte ^ 0x36));
    inner.extend_from_slice(data);
    let inner_hash = sha256(&inner);

    let mut outer = Vec::with_capacity(BLOCK_SIZE + 32);
    outer.extend(block.iter().map(|byte| byte ^ 0x5c));
    outer.extend_from_slice(&inner_hash);
    sha256(&outer)
}

/// HKDF with HMAC-SHA256 (RFC 5869): extract with `salt`, then expand with
/// `info` to `length` bytes (at most 255 * 32).
#[cfg_attr(not(feature = "debug"), allow(dead_code))]
pub fn hkdf_sha256(ikm: &[u8], salt: &[u8], info: &[u8], length: usize) -> Vec<u8> {
    let prk = hmac_sha256(salt, ikm);

    let mut okm = Vec::with_capacity(length);
    let mut previous: Vec<u8> = Vec::new();
    let mut counter = 1u8;
    while okm.len() < length {
        let mut input = Vec::with_capacity(previous.len() + info.len() + 1);
        input.extend_from_slice(&previous);
        input.extend_from_slice(info);
        input.push(counter);
        let block = hmac_sha256(&prk, &input);
        okm.extend_from_slice(&block);
        previous = block.to_vec();
        counter = counter.wrapping_add(1);
    }
    okm.truncate(length);
    okm
}

#[cfg(test)]
mod tests {
    use super::*;

    /// RFC 4231, test case 2.
    #[test]
    fn test_hmac_sha256_rfc4231() {
        let mac = hmac_sha256(b"Jefe", b"what do ya want for nothing?");
        assert_eq!(
            base16ct::lower::encode_string(&mac),
            "5bdcc146bf60754e6a042426089575c75a003f089d2739839dec58b964ec3843"
        );
    }

    /// RFC 4231, test case 6: a key longer than the block size is hashed.
    #[test]
    fn test_hmac_sha256_rfc4231_long_key() {
        let mac = hmac_sha256(
            &[0xaa; 131],
            b"Test Using Larger Than Block-Size Key - Hash Key First",
        );
        assert_eq!(
            base16ct::lower::encode_string(&mac),
            "60e431591ee0b67f0d8a26aacbf5b77f8e0bc6213728c5140546040f0ee37f54"
        );
    }

    /// RFC 5869, test case 1.
    #[test]
    fn test_hkdf_sha256_rfc5869() {
        let okm = hkdf_sha256(
            &[0x0b; 22],
            &base16ct::lower::decode_vec("000102030405060708090a0b0c").unwrap(),
            &base16ct::lower::decode_vec("f0f1f2f3f4f5f6f7f8f9").unwrap(),
            42,
        );
        assert_eq!(
            base16ct::lower::encode_string(&okm),
            "3cb25f25faacd57a90434f64d0362f2a2d2d0a90cf1a5a4c5db02d56ecc4c5bf34007208d5b887185865"
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
