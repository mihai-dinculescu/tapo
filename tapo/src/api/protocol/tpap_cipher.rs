use std::sync::atomic::{AtomicI32, Ordering};

use aes::Aes128;
use ccm::Ccm;
use ccm::aead::{Aead, KeyInit};
use ccm::consts::{U12, U16};

use super::crypto;

type Aes128Ccm = Ccm<Aes128, U16, U12>;

const SEQ_LENGTH: usize = 4;
const TAG_LENGTH: usize = 16;

pub(super) struct TpapCipher {
    cipher: Aes128Ccm,
    base_nonce: Vec<u8>,
    seq: AtomicI32,
}

impl TpapCipher {
    pub fn new(shared_key: &[u8], start_seq: i32) -> anyhow::Result<Self> {
        let key = crypto::hkdf_sha256(
            shared_key,
            b"tp-kdf-salt-aes128-key",
            b"tp-kdf-info-aes128-key",
            16,
        )?;
        let base_nonce = crypto::hkdf_sha256(
            shared_key,
            b"tp-kdf-salt-aes128-iv",
            b"tp-kdf-info-aes128-iv",
            12,
        )?;

        Ok(Self {
            cipher: Aes128Ccm::new_from_slice(&key)?,
            base_nonce,
            seq: AtomicI32::new(start_seq),
        })
    }

    /// Encrypts `data` under the next sequence number. Returns the sequence
    /// number followed by the ciphertext and its tag, and the sequence number
    /// on its own.
    pub fn encrypt(&self, data: &str) -> anyhow::Result<(Vec<u8>, i32)> {
        let seq = self.seq.fetch_add(1, Ordering::Relaxed);

        let cipher_bytes = self
            .cipher
            .encrypt(&self.nonce(seq).into(), data.as_bytes())
            .map_err(|e| anyhow::anyhow!("Encryption error: {e:?}"))?;

        let result = [seq.to_be_bytes().as_slice(), &cipher_bytes].concat();

        Ok((result, seq))
    }

    /// Decrypts the `body` of the response to the request that was encrypted
    /// under `seq`. The device answers under the sequence number of the
    /// request, so any other response is not the answer to it.
    pub fn decrypt(&self, seq: i32, body: &[u8]) -> anyhow::Result<String> {
        let Some((body_seq, cipher_bytes)) = body
            .split_first_chunk::<SEQ_LENGTH>()
            .filter(|(_, cipher_bytes)| cipher_bytes.len() >= TAG_LENGTH)
        else {
            anyhow::bail!(
                "Response of {} bytes is shorter than its {SEQ_LENGTH}-byte sequence number and {TAG_LENGTH}-byte tag",
                body.len()
            );
        };
        let body_seq = i32::from_be_bytes(*body_seq);
        if body_seq != seq {
            anyhow::bail!("Response has the sequence number {body_seq}, but the request has {seq}");
        }

        let decrypted_bytes = self
            .cipher
            .decrypt(&self.nonce(seq).into(), cipher_bytes)
            .map_err(|e| anyhow::anyhow!("Decryption error: {e:?}"))?;
        let decrypted = std::str::from_utf8(&decrypted_bytes)?.to_string();

        Ok(decrypted)
    }
}

impl TpapCipher {
    /// The base nonce with its last 4 bytes replaced by the sequence number,
    /// so only the first 8 of its 12 bytes are used.
    fn nonce(&self, seq: i32) -> [u8; 12] {
        let mut nonce = [0u8; 12];
        nonce[..8].copy_from_slice(&self.base_nonce[..8]);
        nonce[8..].copy_from_slice(&seq.to_be_bytes());
        nonce
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const MESSAGE: &str = r#"{"method":"get_device_info"}"#;

    fn cipher(start_seq: i32) -> TpapCipher {
        let shared_key: Vec<u8> = (0..32).collect();
        TpapCipher::new(&shared_key, start_seq).unwrap()
    }

    #[test]
    fn test_encrypt_matches_a_known_answer() {
        let (body, seq) = cipher(0x01020304).encrypt(MESSAGE).unwrap();

        assert_eq!(seq, 0x01020304);
        assert_eq!(
            base16ct::lower::encode_string(&body),
            "01020304c0331c1c36a5df4ce0c68e5a27b2667ebf842dc994a3fd24b9e860a60fab76d4\
             b66b9b669d95cca10c012f13"
        );
    }

    #[test]
    fn test_encrypt_decrypt_round_trip() {
        let cipher = cipher(42);

        let (body, seq) = cipher.encrypt(MESSAGE).unwrap();

        assert_eq!(body.len(), SEQ_LENGTH + MESSAGE.len() + TAG_LENGTH);
        assert_eq!(cipher.decrypt(seq, &body).unwrap(), MESSAGE);
    }

    #[test]
    fn test_decrypt_rejects_the_response_to_another_request() {
        let cipher = cipher(7);
        let (first_body, first_seq) = cipher.encrypt(MESSAGE).unwrap();
        let (_, second_seq) = cipher.encrypt(MESSAGE).unwrap();

        assert_eq!(cipher.decrypt(first_seq, &first_body).unwrap(), MESSAGE);
        assert!(cipher.decrypt(second_seq, &first_body).is_err());
    }

    #[test]
    fn test_decrypt_rejects_a_body_shorter_than_its_sequence_number_and_tag() {
        let cipher = cipher(0);

        assert!(cipher.decrypt(0, &[0; 19]).is_err());
        assert!(cipher.decrypt(0, &[]).is_err());
    }
}
