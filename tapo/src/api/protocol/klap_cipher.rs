use aes::Aes128;
use cbc::cipher::{BlockDecryptMut, BlockEncryptMut, KeyIvInit};
use sha2::Digest;
use std::sync::atomic::{AtomicI32, Ordering};

// Create AES-128 CBC type
type Aes128CbcEnc = cbc::Encryptor<Aes128>;
type Aes128CbcDec = cbc::Decryptor<Aes128>;

#[derive(Debug)]
pub(super) struct KlapCipher {
    key: Vec<u8>,
    iv: Vec<u8>,
    seq: AtomicI32,
    sig: Vec<u8>,
}

impl KlapCipher {
    pub fn new(
        local_seed: Vec<u8>,
        remote_seed: Vec<u8>,
        user_hash: Vec<u8>,
    ) -> anyhow::Result<Self> {
        let local_hash = &[local_seed, remote_seed, user_hash].concat();

        let (iv, seq) = Self::iv_derive(local_hash)?;

        Ok(Self {
            key: Self::key_derive(local_hash),
            iv,
            seq: AtomicI32::new(seq),
            sig: Self::sig_derive(local_hash),
        })
    }

    pub fn encrypt(&self, data: String) -> anyhow::Result<(Vec<u8>, i32)> {
        let seq = self.seq.fetch_add(1, Ordering::Relaxed) + 1;
        let iv_seq = self.iv_seq(seq);

        // Create CBC encryptor with the key and IV
        let cipher = Aes128CbcEnc::new_from_slices(&self.key, &iv_seq)
            .map_err(|e| anyhow::anyhow!("Encryption error: {:?}", e))?;

        let mut buffer = data.into_bytes();
        let data_len = buffer.len(); // Get the original length before padding

        // Add padding if necessary
        let pad_len = 16 - (buffer.len() % 16);
        buffer.extend(vec![pad_len as u8; pad_len]);

        // Encrypt the buffer, passing the original data length
        let encrypted = cipher
            .encrypt_padded_mut::<aes::cipher::block_padding::Pkcs7>(&mut buffer, data_len)
            .map_err(|e| anyhow::anyhow!("Encryption error: {:?}", e))?;

        let signature =
            Self::sha256(&[self.sig.as_slice(), &seq.to_be_bytes(), encrypted].concat());

        let result = [&signature, encrypted].concat();
        Ok((result, seq))
    }

    pub fn decrypt(&self, seq: i32, cipher_bytes: Vec<u8>) -> anyhow::Result<String> {
        let iv_seq = self.iv_seq(seq);

        // Create CBC decryptor with the key and IV
        let cipher = Aes128CbcDec::new_from_slices(&self.key, &iv_seq)
            .map_err(|e| anyhow::anyhow!("Decryption error: {:?}", e))?;

        let mut buffer = cipher_bytes[32..].to_vec(); // Skip the first 32 bytes (signature)

        let decrypted = cipher
            .decrypt_padded_mut::<aes::cipher::block_padding::Pkcs7>(&mut buffer)
            .map_err(|e| anyhow::anyhow!("Decryption error: {:?}", e))?;

        let decrypted_str = std::str::from_utf8(decrypted)
            .map_err(|e| anyhow::anyhow!("Decryption error: {:?}", e))?
            .to_string();

        Ok(decrypted_str)
    }

    // Derivation methods like iv_derive, key_derive, sig_derive, sha256, etc. would remain unchanged
}

impl KlapCipher {
    fn key_derive(local_hash: &Vec<u8>) -> Vec<u8> {
        let local_hash = &["lsk".as_bytes(), local_hash].concat();
        let hash = Self::sha256(local_hash);
        let key = &hash[..16];
        key.to_vec()
    }

    fn iv_derive(local_hash: &[u8]) -> anyhow::Result<(Vec<u8>, i32)> {
        let local_hash = &["iv".as_bytes(), local_hash].concat();
        let hash = Self::sha256(local_hash);
        let iv = &hash[..12];
        let seq: [u8; 4] = hash[hash.len() - 4..].try_into()?;
        let seq = i32::from_be_bytes(seq);
        Ok((iv.to_vec(), seq))
    }

    fn sig_derive(local_hash: &[u8]) -> Vec<u8> {
        let local_hash = &["ldk".as_bytes(), local_hash].concat();
        let hash = Self::sha256(local_hash);
        let key = &hash[..28];
        key.to_vec()
    }

    fn iv_seq(&self, seq: i32) -> Vec<u8> {
        let mut iv_seq = self.iv.clone();
        iv_seq.extend_from_slice(&seq.to_be_bytes());
        iv_seq
    }
}

impl KlapCipher {
    pub fn sha256(value: &[u8]) -> [u8; 32] {
        let mut hasher = sha2::Sha256::new();
        hasher.update(value);
        hasher.finalize().into()
    }

    pub fn sha1(value: &[u8]) -> [u8; 20] {
        let mut hasher = sha1::Sha1::new();
        hasher.update(value);
        hasher.finalize().into()
    }
}
