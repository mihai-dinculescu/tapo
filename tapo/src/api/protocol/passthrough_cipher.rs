use aes::Aes128;
use aes::cipher::{BlockDecryptMut, BlockEncryptMut, KeyIvInit, block_padding};
use base64::{Engine as _, engine::general_purpose};
use cbc::{Decryptor, Encryptor};
use log::debug;
use rand::{CryptoRng, Rng};
use rsa::pkcs8::{EncodePublicKey, LineEnding};
use rsa::{Pkcs1v15Encrypt, RsaPrivateKey};
use sha1::{Digest, Sha1};

#[derive(Debug, Clone)]
pub(crate) struct PassthroughKeyPair {
    rsa: RsaPrivateKey,
}

impl PassthroughKeyPair {
    pub fn new<R>(rng: &mut R) -> anyhow::Result<Self>
    where
        R: CryptoRng + Rng + ?Sized,
    {
        debug!("Generating RSA key pair...");
        let rsa = RsaPrivateKey::new(rng, 1024)?;

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
    use rand::{Rng, SeedableRng, rngs::StdRng};

    use super::*;

    #[test]
    fn test_passthrough_cipher() -> anyhow::Result<()> {
        let mut rng = StdRng::from_seed([0u8; 32]);

        let key_pair = PassthroughKeyPair::new(&mut rng)?;

        let public_key = key_pair.get_public_key()?;
        assert_eq!(
            public_key,
            "-----BEGIN PUBLIC KEY-----\nMIGfMA0GCSqGSIb3DQEBAQUAA4GNADCBiQKBgQDVBqhjRpIFPl0ee9v5fNxvf0z4\nebYgYfTQYNKELi50Ba4yDm7l3BMyvwGe8OWb+hqP8zZVHDCAQz0cMm4CplYdi4qA\nbrbdHXhkgo6+Y1J6Wo7xXkaH3IxuMTJ0LXphqFPkqvdApD8xfZMbQbL1D5HT6AVM\nKntJZXJnX9/czU/fdQIDAQAB\n-----END PUBLIC KEY-----\n"
        );

        let mut key_bytes = [0u8; 32];
        rng.fill_bytes(&mut key_bytes);
        let key_bytes = key_bytes.to_vec();
        let key_encrypted_bytes =
            key_pair
                .rsa
                .to_public_key()
                .encrypt(&mut rng, Pkcs1v15Encrypt, &key_bytes)?;

        assert_eq!(key_encrypted_bytes.len(), 128);
        assert_eq!(
            key_encrypted_bytes,
            vec![
                208, 156, 169, 76, 213, 173, 29, 141, 119, 45, 60, 47, 142, 214, 125, 2, 185, 30,
                32, 205, 2, 64, 148, 28, 130, 197, 152, 45, 115, 84, 14, 87, 61, 156, 88, 191, 56,
                122, 85, 27, 172, 49, 245, 224, 242, 14, 125, 195, 255, 94, 158, 112, 151, 172,
                197, 19, 82, 22, 207, 151, 80, 165, 51, 185, 189, 229, 78, 204, 107, 98, 211, 101,
                248, 157, 110, 55, 16, 97, 101, 93, 190, 223, 210, 193, 48, 147, 233, 106, 229,
                119, 54, 38, 36, 74, 106, 85, 44, 63, 1, 221, 71, 238, 183, 120, 232, 216, 66, 74,
                9, 77, 131, 37, 15, 72, 121, 25, 5, 245, 250, 134, 61, 126, 202, 144, 55, 15, 4,
                156
            ]
        );

        let key = general_purpose::STANDARD.encode(key_encrypted_bytes);

        let cipher = PassthroughCipher::new(&key, &key_pair)?;

        let message = "hello";
        let message_encrypted = cipher.encrypt(message)?;

        assert_eq!(message_encrypted.len(), 24);
        assert_eq!(message_encrypted, "xd8NUSekkgt3UdW6UHZCkA==");

        assert_eq!(cipher.decrypt(&message_encrypted)?, message);

        Ok(())
    }
}
