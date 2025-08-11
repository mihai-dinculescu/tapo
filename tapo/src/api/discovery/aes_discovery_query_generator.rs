use anyhow::Context;
use crc32fast::Hasher;
use rsa::pkcs1::EncodeRsaPublicKey;
use rsa::rand_core::{OsRng, RngCore};
use rsa::{RsaPrivateKey, RsaPublicKey};
use serde_json::json;

struct KeyPair {
    public_key: RsaPublicKey,
}

impl KeyPair {
    fn new(key_size: usize) -> anyhow::Result<KeyPair> {
        let private_key = RsaPrivateKey::new(&mut OsRng, key_size)
            .context("Failed to generate RSA private key")?;
        let public_key = RsaPublicKey::from(&private_key);
        Ok(KeyPair { public_key })
    }

    fn get_public_pem(&self) -> anyhow::Result<String> {
        self.public_key
            .to_pkcs1_pem(rsa::pkcs8::LineEnding::LF)
            .context("Failed to convert public key to PEM")
    }
}

pub(crate) struct AesDiscoveryQueryGenerator {
    key_pair: KeyPair,
}

impl AesDiscoveryQueryGenerator {
    pub(crate) fn new() -> anyhow::Result<Self> {
        let key_pair = KeyPair::new(1024)?;
        Ok(AesDiscoveryQueryGenerator { key_pair })
    }

    pub(crate) fn generate(&mut self) -> anyhow::Result<Vec<u8>> {
        let mut secret = [0u8; 4];
        OsRng.fill_bytes(&mut secret);

        let key_payload = json!({
            "params": {
                "rsa_key": self.key_pair.get_public_pem()?
            }
        });

        let key_payload_bytes =
            serde_json::to_vec(&key_payload).context("Failed to serialize the key payload Json")?;
        let version = 2u8;
        let msg_type = 0u8;
        let op_code = 1u16;
        let msg_size = key_payload_bytes.len() as u16;
        let flags = 17u8;
        let padding_byte = 0u8;
        let device_serial = u32::from_be_bytes(secret);
        let initial_crc = 0x5A6B7C8Di32;

        let mut disco_header = vec![];
        disco_header.extend_from_slice(&version.to_be_bytes());
        disco_header.extend_from_slice(&msg_type.to_be_bytes());
        disco_header.extend_from_slice(&op_code.to_be_bytes());
        disco_header.extend_from_slice(&msg_size.to_be_bytes());
        disco_header.extend_from_slice(&flags.to_be_bytes());
        disco_header.extend_from_slice(&padding_byte.to_be_bytes());
        disco_header.extend_from_slice(&device_serial.to_be_bytes());
        disco_header.extend_from_slice(&initial_crc.to_be_bytes());

        let mut query = disco_header;
        query.extend_from_slice(&key_payload_bytes);

        let mut hasher = Hasher::new();
        hasher.update(&query);
        let crc = hasher.finalize().to_be_bytes();
        query[12..16].copy_from_slice(&crc);

        Ok(query)
    }
}
