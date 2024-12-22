use std::sync::atomic::{AtomicI32, Ordering};

use aes::cipher::{block_padding, BlockDecryptMut, BlockEncryptMut, KeyIvInit};
use aes::Aes128;
use cbc::{Decryptor, Encryptor};

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
        self.seq.fetch_add(1, Ordering::Relaxed);
        let seq = self.seq.load(Ordering::Relaxed);
        let encryptor = Encryptor::<Aes128>::new_from_slices(&self.key, &self.iv_seq(seq))?;

        let cipher_bytes =
            encryptor.encrypt_padded_vec_mut::<block_padding::Pkcs7>(data.as_bytes());

        let signature = Self::sha256(
            &[
                self.sig.as_slice(),
                &seq.to_be_bytes(),
                cipher_bytes.as_slice(),
            ]
            .concat(),
        );

        let result = [&signature, cipher_bytes.as_slice()].concat();

        Ok((result, seq))
    }

    pub fn decrypt(&self, seq: i32, cipher_bytes: Vec<u8>) -> anyhow::Result<String> {
        let decryptor = Decryptor::<Aes128>::new_from_slices(&self.key, &self.iv_seq(seq))?;

        let decrypted_bytes = decryptor
            .decrypt_padded_vec_mut::<block_padding::Pkcs7>(&cipher_bytes[32..])
            .map_err(|e| anyhow::anyhow!("Decryption error: {:?}", e))?;
        let decrypted = std::str::from_utf8(&decrypted_bytes)?.to_string();

        Ok(decrypted)
    }
}

impl KlapCipher {
    fn key_derive(local_hash: &[u8]) -> Vec<u8> {
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
    pub fn sha1(value: &[u8]) -> [u8; 20] {
        use sha1::{Digest, Sha1};
        let mut hasher = Sha1::new();
        hasher.update(value);
        hasher.finalize().into()
    }

    pub fn sha256(value: &[u8]) -> [u8; 32] {
        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();
        hasher.update(value);
        hasher.finalize().into()
    }
}
