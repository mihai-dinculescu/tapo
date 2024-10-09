use base64::{engine::general_purpose, Engine as _};
use log::debug;
use sha1::Digest;

use aes::Aes128;
use cbc::cipher::{BlockDecryptMut, BlockEncryptMut, KeyIvInit};
use rsa::{pkcs1::EncodeRsaPublicKey, RsaPrivateKey, pkcs1v15::Pkcs1v15Encrypt};
use rand::rngs::OsRng;
use std::str;

type Aes128CbcEnc = cbc::Encryptor<Aes128>;
type Aes128CbcDec = cbc::Decryptor<Aes128>;

#[derive(Debug, Clone)]
pub(crate) struct PassthroughKeyPair {
    rsa: RsaPrivateKey,
}

impl PassthroughKeyPair {
    pub fn new() -> anyhow::Result<Self> {
        debug!("Generating RSA key pair...");
        let mut rng = OsRng;
        let rsa = RsaPrivateKey::new(&mut rng, 1024)?;

        Ok(Self { rsa })
    }

    pub fn get_public_key(&self) -> anyhow::Result<String> {
        let public_key_pem = self.rsa.to_public_key().to_pkcs1_pem(rsa::pkcs8::LineEnding::CR)?;
        Ok(public_key_pem)
    }
}

#[derive(Debug)]
pub(crate) struct PassthroughCipher {
    key: Vec<u8>,
    iv: Vec<u8>,
}

impl PassthroughCipher {
    pub fn new(key: &str, key_pair: &PassthroughKeyPair) -> anyhow::Result<Self> {
        debug!("Will decode handshake key {:?}...", &key[..5]);

        let key_bytes = general_purpose::STANDARD.decode(key)?;

        let padding = Pkcs1v15Encrypt;
        let decrypt = key_pair
            .rsa
            .decrypt(padding, &key_bytes)?;

        if decrypt.len() != 32 {
            return Err(anyhow::anyhow!("Expected 32 bytes, got {}", decrypt.len()));
        }

        Ok(PassthroughCipher {
            key: decrypt[0..16].to_vec(),
            iv: decrypt[16..32].to_vec(),
        })
    }

    pub fn encrypt(&self, data: &str) -> anyhow::Result<String> {
        let mut buffer = data.as_bytes().to_vec();
        let data_len = buffer.len();

        // Add padding if necessary
        let pad_len = 16 - (data_len % 16);
        buffer.extend(vec![pad_len as u8; pad_len]);

        // Create AES CBC encryptor
        let cipher = Aes128CbcEnc::new_from_slices(&self.key, &self.iv)
            .map_err(|e| anyhow::anyhow!("Encryption error: {:?}", e))?;

        let encrypted = cipher
            .encrypt_padded_mut::<aes::cipher::block_padding::Pkcs7>(&mut buffer, data_len)
            .map_err(|e| anyhow::anyhow!("Encryption error: {:?}", e))?;

        let cipher_base64 = general_purpose::STANDARD.encode(encrypted);
        Ok(cipher_base64)
    }

    pub fn decrypt(&self, cipher_base64: &str) -> anyhow::Result<String> {
        let cipher_bytes = general_purpose::STANDARD.decode(cipher_base64)?;

        // Create AES CBC decryptor
        let cipher = Aes128CbcDec::new_from_slices(&self.key, &self.iv)
            .map_err(|e| anyhow::anyhow!("Decryption error: {:?}", e))?;

        let mut buffer = cipher_bytes.to_vec();
        let decrypted = cipher
            .decrypt_padded_mut::<aes::cipher::block_padding::Pkcs7>(&mut buffer)
            .map_err(|e| anyhow::anyhow!("Decryption error: {:?}", e))?;

        let decrypted_str = str::from_utf8(decrypted)
            .map_err(|e| anyhow::anyhow!("Decryption error: {:?}", e))?
            .to_string();

        Ok(decrypted_str)
    }
}

impl PassthroughCipher {
    pub fn sha1_digest_username(username: String) -> String {
        let mut hasher = sha1::Sha1::new();
        hasher.update(username.as_bytes());
        let hash = hasher.finalize();

        base16ct::lower::encode_string(&hash)
    }
}
