use aes::Aes128;
use aes::cipher::{BlockDecryptMut, BlockEncryptMut, KeyIvInit, block_padding};
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
    let cipher_bytes = encryptor.encrypt_padded_vec_mut::<block_padding::Pkcs7>(data.as_bytes());
    Ok(general_purpose::STANDARD.encode(cipher_bytes))
}

pub fn aes128_cbc_decrypt(key: &[u8], iv: &[u8], cipher_base64: &str) -> anyhow::Result<String> {
    let decryptor = Decryptor::<Aes128>::new_from_slices(key, iv)?;
    let cipher_bytes = general_purpose::STANDARD.decode(cipher_base64)?;
    let decrypted_bytes = decryptor
        .decrypt_padded_vec_mut::<block_padding::Pkcs7>(&cipher_bytes)
        .map_err(|e| anyhow::anyhow!("Decryption error: {:?}", e))?;
    Ok(std::str::from_utf8(&decrypted_bytes)?.to_string())
}
