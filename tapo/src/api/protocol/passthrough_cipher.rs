use aes::cipher::{block_padding, BlockDecryptMut, BlockEncryptMut, KeyIvInit};
use aes::Aes128;
use base64::{engine::general_purpose, Engine as _};
use cbc::{Decryptor, Encryptor};
use log::debug;
use rsa::pkcs8::{EncodePublicKey, LineEnding};
use rsa::rand_core::CryptoRngCore;
use rsa::{Pkcs1v15Encrypt, RsaPrivateKey};
use sha1::{Digest, Sha1};

#[derive(Debug, Clone)]
pub(crate) struct PassthroughKeyPair {
    rsa: RsaPrivateKey,
}

impl PassthroughKeyPair {
    pub fn new<R>(mut rng: R) -> anyhow::Result<Self>
    where
        R: CryptoRngCore,
    {
        debug!("Generating RSA key pair...");
        let rsa = RsaPrivateKey::new(&mut rng, 1024)?;

        Ok(Self { rsa })
    }

    pub fn get_public_key(&self) -> anyhow::Result<String> {
        let public_key =
            rsa::RsaPublicKey::to_public_key_pem(&self.rsa.to_public_key(), LineEnding::LF)?;

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
        let buf = key_pair.rsa.decrypt(Pkcs1v15Encrypt, &key_bytes)?;

        if buf.len() != 32 {
            return Err(anyhow::anyhow!("Expected 32 bytes, got {}", buf.len()));
        }

        Ok(PassthroughCipher {
            key: buf[0..16].to_vec(),
            iv: buf[16..32].to_vec(),
        })
    }

    pub fn encrypt(&self, data: &str) -> anyhow::Result<String> {
        let encryptor = Encryptor::<Aes128>::new_from_slices(&self.key, &self.iv)?;

        let cipher_bytes =
            encryptor.encrypt_padded_vec_mut::<block_padding::Pkcs7>(data.as_bytes());
        let cipher_base64 = general_purpose::STANDARD.encode(cipher_bytes);

        Ok(cipher_base64)
    }

    pub fn decrypt(&self, cipher_base64: &str) -> anyhow::Result<String> {
        let decryptor = Decryptor::<Aes128>::new_from_slices(&self.key, &self.iv)?;

        let cipher_bytes = general_purpose::STANDARD.decode(cipher_base64)?;
        let decrypted_bytes = decryptor
            .decrypt_padded_vec_mut::<block_padding::Pkcs7>(&cipher_bytes)
            .map_err(|e| anyhow::anyhow!("Decryption error: {:?}", e))?;

        let decrypted = std::str::from_utf8(&decrypted_bytes)?.to_string();

        Ok(decrypted)
    }
}

impl PassthroughCipher {
    pub fn sha1_digest_username(username: String) -> String {
        let mut hasher = Sha1::new();
        hasher.update(username.as_bytes());
        let hash = hasher.finalize();

        base16ct::lower::encode_string(&hash)
    }
}

#[cfg(test)]
mod tests {
    use rand::{rngs::StdRng, Rng, SeedableRng};

    use super::*;

    #[test]
    fn test_passthrough_cipher() -> anyhow::Result<()> {
        let mut rng = StdRng::seed_from_u64(0);

        let key_pair = PassthroughKeyPair::new(&mut rng)?;

        let public_key = key_pair.get_public_key()?;
        assert_eq!(public_key.len(), 272);
        assert_eq!(
            public_key,
            "-----BEGIN PUBLIC KEY-----
MIGfMA0GCSqGSIb3DQEBAQUAA4GNADCBiQKBgQDimR6OafJxMw3NNPTE8fTGA+I5
Djrk5YCtTAcnGXPsWP8tgGfgJ/S5SI21CzMiNA0GrbA1a2fIYafR73a5Be3+DTWd
pg/BjhASlKZos6CbkkVsOMeVKQOkdToGrRtHW6cIofLM6ZvZvuzVTTPdMd+paEjq
waihnXBCkPwQndikfwIDAQAB
-----END PUBLIC KEY-----\n"
        );

        let key_bytes = vec![rng.gen(); 32];
        let key_encrypted_bytes =
            key_pair
                .rsa
                .to_public_key()
                .encrypt(&mut rng, Pkcs1v15Encrypt, &key_bytes)?;

        assert_eq!(key_encrypted_bytes.len(), 128);
        assert_eq!(
            key_encrypted_bytes,
            vec![
                166, 230, 248, 62, 63, 183, 64, 126, 54, 24, 66, 139, 130, 63, 33, 84, 139, 98, 55,
                39, 200, 61, 91, 180, 108, 199, 245, 34, 183, 145, 198, 211, 79, 76, 151, 50, 16,
                136, 184, 88, 9, 118, 167, 70, 106, 212, 38, 17, 146, 140, 177, 42, 146, 149, 70,
                129, 229, 16, 56, 138, 206, 24, 168, 167, 65, 225, 136, 188, 137, 208, 216, 31, 44,
                195, 218, 150, 61, 172, 36, 63, 66, 56, 144, 5, 80, 199, 223, 153, 201, 237, 187,
                243, 81, 255, 139, 78, 27, 126, 49, 186, 218, 135, 98, 88, 250, 254, 135, 155, 196,
                101, 62, 234, 63, 254, 184, 34, 195, 110, 70, 213, 237, 228, 199, 37, 101, 3, 33,
                157
            ]
        );
        println!("{:?}", key_encrypted_bytes);

        let key = general_purpose::STANDARD.encode(key_encrypted_bytes);

        let cipher = PassthroughCipher::new(&key, &key_pair)?;

        let message = "hello";
        let message_encrypted = cipher.encrypt(message)?;

        assert_eq!(message_encrypted.len(), 24);
        assert_eq!(message_encrypted, "qUAXr1/bAt6+pHYjns76KA==");

        assert_eq!(cipher.decrypt(&message_encrypted)?, message);

        Ok(())
    }
}
