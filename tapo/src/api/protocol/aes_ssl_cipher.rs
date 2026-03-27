use std::sync::atomic::{AtomicI32, Ordering};

use super::crypto;

#[derive(Debug)]
pub(super) struct AesSslCipher {
    key: Vec<u8>,
    iv: Vec<u8>,
    tag_prefix: String,
    seq: AtomicI32,
}

impl AesSslCipher {
    pub fn new(
        local_nonce: String,
        server_nonce: &str,
        password_hash: String,
        start_seq: i32,
    ) -> Self {
        let hashed_key =
            crypto::sha256_hex(format!("{local_nonce}{password_hash}{server_nonce}").as_bytes());

        let lsk = crypto::sha256(format!("lsk{local_nonce}{server_nonce}{hashed_key}").as_bytes());
        let ivb = crypto::sha256(format!("ivb{local_nonce}{server_nonce}{hashed_key}").as_bytes());

        let tag_prefix = crypto::sha256_hex(format!("{password_hash}{local_nonce}").as_bytes());

        Self {
            key: lsk[..16].to_vec(),
            iv: ivb[..16].to_vec(),
            tag_prefix,
            seq: AtomicI32::new(start_seq),
        }
    }

    pub fn encrypt(&self, data: &str) -> anyhow::Result<String> {
        crypto::aes128_cbc_encrypt(&self.key, &self.iv, data)
    }

    pub fn generate_tag(&self, request_body: &str, sequence: i32) -> String {
        crypto::sha256_hex(format!("{}{request_body}{sequence}", self.tag_prefix).as_bytes())
    }

    pub fn next_sequence(&self) -> i32 {
        self.seq.fetch_add(1, Ordering::Relaxed)
    }
}

pub(super) fn generate_nonce() -> String {
    use rand::RngExt as _;
    let bytes: [u8; 8] = rand::rng().random();
    base16ct::upper::encode_string(&bytes)
}

pub(super) fn validate_device_confirm(
    local_nonce: &str,
    server_nonce: &str,
    password_hash: &str,
    device_confirm: &str,
) -> bool {
    let expected_hash =
        crypto::sha256_hex(format!("{local_nonce}{password_hash}{server_nonce}").as_bytes());
    let expected = format!("{expected_hash}{server_nonce}{local_nonce}");
    expected == device_confirm
}

pub(super) fn compute_password_digest(
    local_nonce: &str,
    server_nonce: &str,
    password_hash: &str,
) -> String {
    let digest =
        crypto::sha256_hex(format!("{password_hash}{local_nonce}{server_nonce}").as_bytes());
    format!("{digest}{local_nonce}{server_nonce}")
}

#[cfg(test)]
mod tests {
    use super::*;

    impl AesSslCipher {
        fn decrypt(&self, cipher_base64: &str) -> anyhow::Result<String> {
            crypto::aes128_cbc_decrypt(&self.key, &self.iv, cipher_base64)
        }
    }

    #[test]
    fn test_nonce_generation() {
        let nonce = generate_nonce();
        assert_eq!(nonce.len(), 16);
        assert!(nonce.chars().all(|c| c.is_ascii_hexdigit()));
        assert!(nonce.chars().all(|c| !c.is_ascii_lowercase()));
    }

    #[test]
    fn test_sha256_hex() {
        let result = crypto::sha256_hex(b"hello");
        assert_eq!(
            result,
            "2CF24DBA5FB0A30E26E83B2AC5B9E29E1B161E5C1FA7425E73043362938B9824"
        );
    }

    #[test]
    fn test_md5_hex() {
        let result = crypto::md5_hex(b"hello");
        assert_eq!(result, "5D41402ABC4B2A76B9719D911017C592");
    }

    #[test]
    fn test_encrypt_decrypt_round_trip() -> anyhow::Result<()> {
        let cipher = AesSslCipher::new(
            "ABCDEF0123456789".to_string(),
            "1234567890ABCDEF",
            "E3B0C44298FC1C149AFBF4C8996FB92427AE41E4649B934CA495991B7852B855".to_string(),
            0,
        );

        let message = r#"{"method":"get_device_info"}"#;
        let encrypted = cipher.encrypt(message)?;
        let decrypted = cipher.decrypt(&encrypted)?;
        assert_eq!(decrypted, message);

        Ok(())
    }

    #[test]
    fn test_validate_device_confirm() {
        let local_nonce = "AAAA1111BBBB2222";
        let server_nonce = "CCCC3333DDDD4444";
        let pwd_hash = crypto::sha256_hex(b"mypassword");

        let expected_hash =
            crypto::sha256_hex(format!("{local_nonce}{pwd_hash}{server_nonce}").as_bytes());
        let device_confirm = format!("{expected_hash}{server_nonce}{local_nonce}");

        assert!(validate_device_confirm(
            local_nonce,
            server_nonce,
            &pwd_hash,
            &device_confirm
        ));
        assert!(!validate_device_confirm(
            local_nonce,
            server_nonce,
            &pwd_hash,
            "wrong"
        ));
    }

    #[test]
    fn test_compute_digest_passwd() {
        let local_nonce = "AAAA1111BBBB2222";
        let server_nonce = "CCCC3333DDDD4444";
        let pwd_hash = "SOMEHASH";

        let result = compute_password_digest(local_nonce, server_nonce, pwd_hash);

        let expected_digest =
            crypto::sha256_hex(format!("{pwd_hash}{local_nonce}{server_nonce}").as_bytes());
        assert_eq!(
            result,
            format!("{expected_digest}{local_nonce}{server_nonce}")
        );
    }

    #[test]
    fn test_generate_tag() {
        let cipher = AesSslCipher::new(
            "ABCDEF0123456789".to_string(),
            "1234567890ABCDEF",
            "MYPWDHASH".to_string(),
            100,
        );

        let tag = cipher.generate_tag(r#"{"method":"test"}"#, 100);

        let pwd_nonce_hash = crypto::sha256_hex(b"MYPWDHASHABCDEF0123456789");
        let expected =
            crypto::sha256_hex(format!(r#"{pwd_nonce_hash}{{"method":"test"}}100"#).as_bytes());
        assert_eq!(tag, expected);
    }

    #[test]
    fn test_next_seq() {
        let cipher = AesSslCipher::new(
            "ABCDEF0123456789".to_string(),
            "1234567890ABCDEF",
            "HASH".to_string(),
            42,
        );

        assert_eq!(cipher.next_sequence(), 42);
        assert_eq!(cipher.next_sequence(), 43);
        assert_eq!(cipher.next_sequence(), 44);
    }
}
