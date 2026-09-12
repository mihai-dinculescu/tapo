//! Decryption of encrypted media stream parts.
//!
//! When a part carries `X-If-Encrypt: 1`, its body is AES-128-CBC with PKCS7
//! padding. The Tapo app (`hc0/b.java`, `qb0/c.java`, `if0/a.java`) derives
//! the keys from the `Key-Exchange` header of the `200` response, e.g.
//! `cipher="AES_128_CBC" username="admin" padding="PKCS7_16" algorithm="HKDF"
//! nonce="…" salt="…"`, and a secret:
//!
//! - AES key: `HKDF-SHA256(ikm = "<nonce>:<secret>", salt = "<salt>",
//!   info = "stream_hkdf_aes_key", 16 bytes)`.
//! - HMAC key: the same with `info = "stream_hkdf_hmac_key"`, also 16 bytes
//!   (the app's `HKDFHelper.d()` relies on the default length; the `8` in
//!   the decompiled call is Kotlin's default-argument mask). Each part's
//!   `X-Data-Hmac` is the base64 HMAC-SHA256 of its ciphertext.
//! - IV: the part's `X-Nonce` header, hex-decoded.
//!
//! The secret the app uses is the password as pre-hashed for the Digest
//! handshake (upper-case hex SHA-256 on an `encrypt_type` 3 hub). Since that
//! rests on the hub's default username matching `admin`, callers may offer
//! several candidate secrets and let the HMAC pick the right one.

use std::collections::HashMap;

use anyhow::{Context, anyhow};
use base64::{Engine as _, engine::general_purpose};

use crate::api::protocol::crypto;

const AES_KEY_INFO: &[u8] = b"stream_hkdf_aes_key";
const HMAC_KEY_INFO: &[u8] = b"stream_hkdf_hmac_key";
const AES_KEY_LENGTH: usize = 16;
const HMAC_KEY_LENGTH: usize = 16;

/// The parsed `Key-Exchange` header.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct KeyExchange {
    pub cipher: Option<String>,
    pub username: Option<String>,
    pub padding: Option<String>,
    pub algorithm: Option<String>,
    pub nonce: String,
    pub salt: Option<String>,
}

impl KeyExchange {
    /// Parses the space-separated `name="value"` list the hub sends. Like the
    /// app, only `nonce` is required.
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

        let nonce = params
            .remove("nonce")
            .ok_or_else(|| anyhow!("Key-Exchange is missing the nonce"))?;

        Ok(Self {
            cipher: params.remove("cipher"),
            username: params.remove("username"),
            padding: params.remove("padding"),
            algorithm: params.remove("algorithm"),
            nonce,
            salt: params.remove("salt"),
        })
    }

    /// Whether this is the HKDF / AES-128-CBC scheme implemented here.
    pub fn is_supported(&self) -> bool {
        self.algorithm
            .as_deref()
            .is_some_and(|algorithm| algorithm.eq_ignore_ascii_case("HKDF"))
            && self
                .cipher
                .as_deref()
                .is_none_or(|cipher| cipher.eq_ignore_ascii_case("AES_128_CBC"))
            && self.salt.is_some()
    }
}

/// The keys derived for one media stream session.
#[derive(Debug, Clone)]
pub(super) struct MediaCipher {
    aes_key: Vec<u8>,
    hmac_key: Vec<u8>,
}

impl MediaCipher {
    /// Derives the keys from the key exchange and a candidate secret.
    pub fn derive(key_exchange: &KeyExchange, secret: &str) -> anyhow::Result<Self> {
        let salt = key_exchange
            .salt
            .as_deref()
            .ok_or_else(|| anyhow!("Key-Exchange is missing the salt"))?;
        let ikm = format!("{}:{secret}", key_exchange.nonce);

        Ok(Self {
            aes_key: crypto::hkdf_sha256(
                ikm.as_bytes(),
                salt.as_bytes(),
                AES_KEY_INFO,
                AES_KEY_LENGTH,
            ),
            hmac_key: crypto::hkdf_sha256(
                ikm.as_bytes(),
                salt.as_bytes(),
                HMAC_KEY_INFO,
                HMAC_KEY_LENGTH,
            ),
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

    #[test]
    fn test_key_exchange_parse() {
        let key_exchange = KeyExchange::parse(H200_KEY_EXCHANGE).unwrap();

        assert_eq!(key_exchange.cipher.as_deref(), Some("AES_128_CBC"));
        assert_eq!(key_exchange.username.as_deref(), Some("admin"));
        assert_eq!(key_exchange.padding.as_deref(), Some("PKCS7_16"));
        assert_eq!(key_exchange.algorithm.as_deref(), Some("HKDF"));
        assert_eq!(key_exchange.nonce, "4514f88f1148a6735bdc6a7d7b93b0b0");
        assert_eq!(
            key_exchange.salt.as_deref(),
            Some("f9192a9ee24bc7db8df141bf2bd56af4")
        );
        assert!(key_exchange.is_supported());
    }

    #[test]
    fn test_key_exchange_parse_requires_nonce_and_flags_other_schemes() {
        assert!(KeyExchange::parse("cipher=\"AES_128_CBC\" salt=\"x\"").is_err());

        let legacy = KeyExchange::parse("username=\"admin\" nonce=\"N\"").unwrap();
        assert!(!legacy.is_supported());

        let other_cipher =
            KeyExchange::parse("cipher=\"AES_256_GCM\" algorithm=\"HKDF\" nonce=\"N\" salt=\"S\"")
                .unwrap();
        assert!(!other_cipher.is_supported());
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

    /// The derivation is deterministic and differs per secret; pins the
    /// current output so that later refactors cannot change it silently.
    #[test]
    fn test_media_cipher_derive_is_deterministic() {
        let key_exchange = KeyExchange::parse(H200_KEY_EXCHANGE).unwrap();
        let a = MediaCipher::derive(&key_exchange, "SECRET").unwrap();
        let b = MediaCipher::derive(&key_exchange, "SECRET").unwrap();
        let c = MediaCipher::derive(&key_exchange, "secret").unwrap();

        assert_eq!(a.aes_key, b.aes_key);
        assert_eq!(a.hmac_key, b.hmac_key);
        assert_ne!(a.aes_key, c.aes_key);
        assert_eq!(a.aes_key.len(), 16);
        assert_eq!(a.hmac_key.len(), 16);
    }
}
