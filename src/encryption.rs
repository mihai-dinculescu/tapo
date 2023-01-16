use base64::{engine::general_purpose, Engine as _};
use log::debug;
use openssl::symm::{decrypt, encrypt, Cipher};
use openssl::{pkey, rsa, sha::Sha1};

pub struct KeyPair {
    pub rsa: rsa::Rsa<pkey::Private>,
    pub private_key: String,
    pub public_key: String,
}

impl KeyPair {
    pub fn new() -> anyhow::Result<Self> {
        debug!("Generating key...");
        let rsa = rsa::Rsa::generate(1024)?;

        let private_key_pem = rsa.private_key_to_pem()?;
        let public_key_pem = rsa.public_key_to_pem()?;

        let private_key = std::str::from_utf8(&private_key_pem)?.to_string();
        let public_key = std::str::from_utf8(&public_key_pem)?.to_string();

        Ok(Self {
            rsa,
            private_key,
            public_key,
        })
    }
}

#[derive(Debug)]
pub struct TpLinkCipher {
    key: Vec<u8>,
    iv: Vec<u8>,
}

impl TpLinkCipher {
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

pub fn decode_handshake_key(key: &str, key_pair: &KeyPair) -> anyhow::Result<TpLinkCipher> {
    let key_preview = &key[..5];
    debug!("Will decode handshake key {key_preview:?}...");

    let key_bytes = general_purpose::STANDARD.decode(key)?;
    let mut buf = vec![0; key_pair.rsa.size() as usize];

    let decrypt_count = key_pair
        .rsa
        .private_decrypt(&key_bytes, &mut buf, rsa::Padding::PKCS1)?;

    if decrypt_count != 32 {
        return Err(anyhow::anyhow!("expected 32 bytes, got {decrypt_count}"));
    }

    Ok(TpLinkCipher {
        key: buf[0..16].to_vec(),
        iv: buf[16..32].to_vec(),
    })
}

pub fn sha_digest_username(username: &str) -> anyhow::Result<String> {
    let mut hasher = Sha1::new();
    hasher.update(username.as_bytes());
    let hash = hasher.finish();

    let hex_hash = base16ct::lower::encode_string(&hash);

    Ok(hex_hash)
}
