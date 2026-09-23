//! Decryption of encrypted media stream parts.
//!
//! The hub encrypts every media part (`X-If-Encrypt: 1`) with AES-128-CBC and
//! PKCS7 padding. The Tapo app derives the keys from the `Key-Exchange`
//! header of the `200` response, e.g.
//! `cipher="AES_128_CBC" username="admin" padding="PKCS7_16" algorithm="HKDF"
//! nonce="…" salt="…"`, and a secret:
//!
//! - AES key: `HKDF-SHA256(ikm = "<nonce>:<secret>", salt = "<salt>",
//!   info = "stream_hkdf_aes_key", 16 bytes)`.
//! - HMAC key: the same with `info = "stream_hkdf_hmac_key"`, also 16 bytes.
//!   Each part's `X-Data-Hmac` is the base64 HMAC-SHA256 of its ciphertext.
//! - IV: the part's `X-Nonce` header, hex-decoded.
//!
//! The `nonce` may be empty, which makes the ikm `":<secret>"`. An H200 on
//! firmware 1.7.5 sent `nonce=""` on every connection on 2026-09-22, and a
//! 32-hex nonce on every earlier one.
//!
//! The secret is the password as pre-hashed for the Digest handshake
//! (upper-case hex SHA-256), which the app reuses for the media cipher.

use std::collections::HashMap;

use anyhow::{Context, anyhow, bail};
use base64::{Engine as _, engine::general_purpose};

use crate::api::protocol::crypto;

const CIPHER: &str = "AES_128_CBC";
const ALGORITHM: &str = "HKDF";
const AES_KEY_INFO: &[u8] = b"stream_hkdf_aes_key";
const HMAC_KEY_INFO: &[u8] = b"stream_hkdf_hmac_key";
const AES_KEY_LENGTH: usize = 16;
const HMAC_KEY_LENGTH: usize = 16;

/// The parsed `Key-Exchange` header.
#[derive(Debug)]
pub(super) struct KeyExchange {
    pub nonce: String,
    pub salt: String,
}

impl KeyExchange {
    /// Parses the space-separated `name="value"` list the hub sends. Only the
    /// scheme an H200 sends is accepted: `cipher="AES_128_CBC"` and
    /// `algorithm="HKDF"`.
    pub fn parse(value: &str) -> anyhow::Result<Self> {
        let mut params: HashMap<String, String> = HashMap::new();
        for token in value.split_whitespace() {
            if let Some((name, value)) = token.split_once('=') {
                params.insert(
                    name.trim().to_ascii_lowercase(),
                    value.trim().trim_matches('"').to_string(),
                );
            }
        }

        match params.remove("cipher") {
            Some(cipher) if cipher.eq_ignore_ascii_case(CIPHER) => {}
            Some(cipher) => bail!("unsupported Key-Exchange cipher `{cipher}`"),
            None => bail!("Key-Exchange is missing the cipher"),
        }
        match params.remove("algorithm") {
            Some(algorithm) if algorithm.eq_ignore_ascii_case(ALGORITHM) => {}
            Some(algorithm) => bail!("unsupported Key-Exchange algorithm `{algorithm}`"),
            None => bail!("Key-Exchange is missing the algorithm"),
        }

        let nonce = params
            .remove("nonce")
            .ok_or_else(|| anyhow!("Key-Exchange is missing the nonce"))?;
        let salt = params
            .remove("salt")
            .ok_or_else(|| anyhow!("Key-Exchange is missing the salt"))?;

        Ok(Self { nonce, salt })
    }
}

/// The keys derived for one media stream session.
pub(super) struct MediaCipher {
    aes_key: Vec<u8>,
    hmac_key: Vec<u8>,
}

impl MediaCipher {
    /// Derives the keys from the key exchange and the secret.
    pub fn derive(key_exchange: &KeyExchange, secret: &str) -> anyhow::Result<Self> {
        let salt = &key_exchange.salt;
        let ikm = format!("{}:{secret}", key_exchange.nonce);

        Ok(Self {
            aes_key: crypto::hkdf_sha256(
                ikm.as_bytes(),
                salt.as_bytes(),
                AES_KEY_INFO,
                AES_KEY_LENGTH,
            )?,
            hmac_key: crypto::hkdf_sha256(
                ikm.as_bytes(),
                salt.as_bytes(),
                HMAC_KEY_INFO,
                HMAC_KEY_LENGTH,
            )?,
        })
    }

    /// Whether `hmac_base64` (the part's `X-Data-Hmac`) matches `ciphertext`.
    pub fn verify_hmac(&self, ciphertext: &[u8], hmac_base64: &str) -> bool {
        let expected = crypto::hmac_sha256(&self.hmac_key, ciphertext);
        general_purpose::STANDARD.encode(expected) == hmac_base64.trim()
    }

    /// Decrypts a part's body with the IV from its `X-Nonce` header.
    pub fn decrypt(&self, nonce_hex: &str, ciphertext: &[u8]) -> anyhow::Result<Vec<u8>> {
        let iv = base16ct::mixed::decode_vec(nonce_hex.trim())
            .map_err(|err| anyhow!("invalid X-Nonce `{nonce_hex}`: {err}"))?;
        crypto::aes128_cbc_decrypt_bytes(&self.aes_key, &iv, ciphertext)
            .context("decrypt media stream part")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const H200_KEY_EXCHANGE: &str = "cipher=\"AES_128_CBC\" username=\"admin\" padding=\"PKCS7_16\" algorithm=\"HKDF\" nonce=\"4514f88f1148a6735bdc6a7d7b93b0b0\" salt=\"f9192a9ee24bc7db8df141bf2bd56af4\"";
    /// Sent by an H200 (firmware 1.7.5) on 2026-09-22.
    const H200_KEY_EXCHANGE_EMPTY_NONCE: &str = "cipher=\"AES_128_CBC\" username=\"admin\" padding=\"PKCS7_16\" algorithm=\"HKDF\" nonce=\"\" salt=\"fd1a82ca71cb9e00be85f57ac69360f9\"";

    #[test]
    fn test_key_exchange_parse() {
        let key_exchange = KeyExchange::parse(H200_KEY_EXCHANGE).unwrap();

        assert_eq!(key_exchange.nonce, "4514f88f1148a6735bdc6a7d7b93b0b0");
        assert_eq!(key_exchange.salt, "f9192a9ee24bc7db8df141bf2bd56af4");
    }

    /// Only the `cipher` and `algorithm` an H200 sends are accepted, and both
    /// `nonce` and `salt` are required.
    #[test]
    fn test_key_exchange_parse_rejects_other_schemes() {
        for value in [
            "algorithm=\"HKDF\" nonce=\"N\" salt=\"S\"",
            "cipher=\"AES_256_GCM\" algorithm=\"HKDF\" nonce=\"N\" salt=\"S\"",
            "cipher=\"AES_128_CBC\" nonce=\"N\" salt=\"S\"",
            "cipher=\"AES_128_CBC\" algorithm=\"PBKDF2\" nonce=\"N\" salt=\"S\"",
            "cipher=\"AES_128_CBC\" algorithm=\"HKDF\" salt=\"S\"",
            "cipher=\"AES_128_CBC\" algorithm=\"HKDF\" nonce=\"N\"",
            "username=\"admin\" nonce=\"N\"",
        ] {
            assert!(KeyExchange::parse(value).is_err(), "{value}");
        }
    }

    #[test]
    fn test_media_cipher_round_trip_and_hmac() {
        let key_exchange = KeyExchange::parse(H200_KEY_EXCHANGE).unwrap();
        let cipher = MediaCipher::derive(&key_exchange, "SECRET").unwrap();

        // Encrypt with the derived key directly, the way the hub would.
        let iv_hex = "97b45a69521ceae88775a96dab9d6628";
        let iv = base16ct::lower::decode_vec(iv_hex).unwrap();
        let plaintext = b"\x47\x40\x00\x10 a transport stream packet";
        let ciphertext_base64 = crypto::aes128_cbc_encrypt(
            &cipher.aes_key,
            &iv,
            std::str::from_utf8(plaintext).unwrap(),
        )
        .unwrap();
        let ciphertext = general_purpose::STANDARD.decode(ciphertext_base64).unwrap();

        let hmac =
            general_purpose::STANDARD.encode(crypto::hmac_sha256(&cipher.hmac_key, &ciphertext));
        assert!(cipher.verify_hmac(&ciphertext, &hmac));
        assert!(!cipher.verify_hmac(&ciphertext, "AAAA"));

        let other = MediaCipher::derive(&key_exchange, "OTHER").unwrap();
        assert!(!other.verify_hmac(&ciphertext, &hmac));

        assert_eq!(cipher.decrypt(iv_hex, &ciphertext).unwrap(), plaintext);
        assert!(cipher.decrypt("not hex", &ciphertext).is_err());
    }

    /// Pins the derived keys, computed separately with an RFC 5869 HKDF, so
    /// that a change to the derivation (such as swapping the salt and the
    /// ikm) cannot pass unnoticed. The keys also differ per secret.
    #[test]
    fn test_media_cipher_derive_known_keys() {
        let key_exchange = KeyExchange::parse(H200_KEY_EXCHANGE).unwrap();
        let cipher = MediaCipher::derive(&key_exchange, "SECRET").unwrap();
        let other = MediaCipher::derive(&key_exchange, "secret").unwrap();

        assert_eq!(
            base16ct::lower::encode_string(&cipher.aes_key),
            "bb7cb3a56d64bd5238c8998f8c1dd7e8"
        );
        assert_eq!(
            base16ct::lower::encode_string(&cipher.hmac_key),
            "243f9ab129ce97a19e84f17b97a46fa5"
        );
        assert_ne!(cipher.aes_key, other.aes_key);
    }

    /// An empty `nonce` parses, and the keys derive from the ikm `":<secret>"`
    /// (pinned the same way as above).
    #[test]
    fn test_media_cipher_derive_empty_nonce() {
        let key_exchange = KeyExchange::parse(H200_KEY_EXCHANGE_EMPTY_NONCE).unwrap();
        assert_eq!(key_exchange.nonce, "");
        assert_eq!(key_exchange.salt, "fd1a82ca71cb9e00be85f57ac69360f9");

        let cipher = MediaCipher::derive(&key_exchange, "SECRET").unwrap();
        assert_eq!(
            base16ct::lower::encode_string(&cipher.aes_key),
            "bfcb321cbf40361ed342ea3f0a9d7203"
        );
        assert_eq!(
            base16ct::lower::encode_string(&cipher.hmac_key),
            "81ced300ae4ea0fa865d2c1dc074aec7"
        );
    }
}
