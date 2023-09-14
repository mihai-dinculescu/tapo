use base64::{engine::general_purpose, Engine as _};
use log::debug;
use openssl::symm::{decrypt, encrypt, Cipher};
use openssl::{pkey, rsa, sha::Sha1};

#[derive(Debug, Clone)]
pub(crate) struct PassthroughKeyPair {
    rsa: rsa::Rsa<pkey::Private>,
}

impl PassthroughKeyPair {
    pub fn new() -> anyhow::Result<Self> {
        debug!("Generating RSA key pair...");
        let rsa = rsa::Rsa::generate(1024)?;

        Ok(Self { rsa })
    }

    pub fn get_public_key(&self) -> anyhow::Result<String> {
        let public_key_pem = self.rsa.public_key_to_pem()?;
        let public_key = std::str::from_utf8(&public_key_pem)?.to_string();

        Ok(public_key)
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
        let mut buf = vec![0; key_pair.rsa.size() as usize];

        let decrypt_count =
            key_pair
                .rsa
                .private_decrypt(&key_bytes, &mut buf, rsa::Padding::PKCS1)?;

        if decrypt_count != 32 {
            return Err(anyhow::anyhow!("expected 32 bytes, got {decrypt_count}"));
        }

        Ok(PassthroughCipher {
            key: buf[0..16].to_vec(),
            iv: buf[16..32].to_vec(),
        })
    }

    pub fn encrypt(&self, data: &str) -> anyhow::Result<String> {
        let cipher_bytes = encrypt(
            Cipher::aes_128_cbc(),
            &self.key,
            Some(&self.iv),
            data.as_bytes(),
        )?;
        let cipher_base64 = general_purpose::STANDARD.encode(cipher_bytes);

        Ok(cipher_base64)
    }

    pub fn decrypt(&self, cipher_base64: &str) -> anyhow::Result<String> {
        let cipher_bytes = general_purpose::STANDARD.decode(cipher_base64)?;
        let decrypted_bytes = decrypt(
            Cipher::aes_128_cbc(),
            &self.key,
            Some(&self.iv),
            &cipher_bytes,
        )?;
        let decrypted = std::str::from_utf8(&decrypted_bytes)?.to_string();

        Ok(decrypted)
    }
}

impl PassthroughCipher {
    pub fn sha1_digest_username(username: String) -> String {
        let mut hasher = Sha1::new();
        hasher.update(username.as_bytes());
        let hash = hasher.finish();

        base16ct::lower::encode_string(&hash)
    }
}
