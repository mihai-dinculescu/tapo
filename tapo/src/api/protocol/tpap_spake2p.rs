//! The prover side of SPAKE2+ (RFC 9383) on P-256 with SHA-256, HKDF-SHA256
//! and HMAC-SHA256, which is how the TPAP protocol logs in.

use p256::elliptic_curve::Generate;
use p256::elliptic_curve::ff::FromUniformBytes;
use p256::elliptic_curve::sec1::ToSec1Point;
use p256::{NonZeroScalar, ProjectivePoint, PublicKey, Scalar};

use crate::{Error, TapoResponseError};

use super::crypto;
use super::tapo_protocol::DeviceFamily;

/// The points `M` and `N` that RFC 9383 assigns to P-256, compressed.
const M: &str = "02886e2f97ace46e55ba9dd7242579f2993b64e16ef3dcab95afd497333d8fa12f";
const N: &str = "03d8bbd6c639c62937b04d997f38c3770719c629d7014d49a24b4f98baa1292b49";

/// What the prover holds once it has seen the verifier's share.
pub(super) struct Proof {
    /// The prover's share `X`, uncompressed.
    pub share: Vec<u8>,
    /// Proves to the verifier that the prover knows the password.
    pub confirm: [u8; 32],
    /// The secret that both sides derive the session keys from.
    pub shared_key: Vec<u8>,
    verifier_confirm_key: Vec<u8>,
}

impl Proof {
    /// Whether `confirm` proves that the verifier knows the password too.
    pub fn verify(&self, confirm: &[u8]) -> bool {
        crypto::hmac_sha256_verify(&self.verifier_confirm_key, &self.share, confirm)
    }
}

/// The string that stands in for the password, which depends on the
/// `extra_crypt` of the device's `pake_register` response and on the
/// `device_family`.
///
/// # Errors
///
/// Returns an error if the device asks for a password transform that is not
/// supported, or for none at all unless it is a camera.
pub(super) fn credential(
    password: &str,
    extra_crypt: Option<&serde_json::Value>,
    device_family: DeviceFamily,
) -> Result<String, Error> {
    let Some(extra_crypt) = extra_crypt else {
        return match device_family {
            // A camera names no transform and takes the MD5 of the password.
            DeviceFamily::SmartCam => Ok(crypto::md5_hex_lower(password.as_bytes())),
            DeviceFamily::Smart => Err(Error::Tapo(TapoResponseError::ResponseError {
                description:
                    "The device asks for no password transform (`extra_crypt`), which is not supported yet"
                        .to_string(),
            })),
        };
    };

    let kind = extra_crypt.get("type").and_then(|v| v.as_str());
    let password_id = extra_crypt
        .pointer("/params/passwd_id")
        .and_then(|v| v.as_i64());

    match (kind, password_id) {
        // The app hashes only as many bytes of the UTF-8 password as the
        // password has UTF-16 code units, which cuts a non-ASCII password
        // short. Which of the two hashes such a device holds is not known, so
        // the whole password is hashed.
        (Some("password_shadow"), Some(2)) => Ok(crypto::sha1_hex_lower(password.as_bytes())),
        _ => Err(Error::Tapo(TapoResponseError::ResponseError {
            description: format!(
                "The device asks for a password transform that is not supported yet: {extra_crypt}"
            ),
        })),
    }
}

/// Runs the prover with the `credential`, the `salt` and `iterations` of the
/// device, and the random values that both sides exchanged.
///
/// # Errors
///
/// Returns an error if `device_share` is not a point that the login can continue with.
pub(super) fn prove(
    credential: &str,
    salt: &[u8],
    iterations: u32,
    user_random: &[u8],
    device_random: &[u8],
    device_share: &[u8],
) -> Result<Proof, Error> {
    let (w0, w1) = derive_w0_w1(credential, salt, iterations);
    let x = NonZeroScalar::generate_from_rng(&mut rand::rng());
    let context = crypto::sha256(&[b"PAKE V1".as_slice(), user_random, device_random].concat());

    prove_with(&w0, &w1, &x, &context, b"", b"", device_share)
}

/// `w0` and `w1`, the two halves of the PBKDF2 output reduced modulo the
/// group order.
fn derive_w0_w1(credential: &str, salt: &[u8], iterations: u32) -> (Scalar, Scalar) {
    let key = crypto::pbkdf2_hmac_sha256(credential.as_bytes(), salt, iterations, 80);
    let (w0, w1) = key.split_at(40);

    (scalar_from_wide(w0), scalar_from_wide(w1))
}

/// The big-endian `bytes`, of at most 64 bytes, reduced modulo the group order.
fn scalar_from_wide(bytes: &[u8]) -> Scalar {
    let mut wide = [0u8; 64];
    wide[64 - bytes.len()..].copy_from_slice(bytes);

    Scalar::from_uniform_bytes(&wide)
}

fn prove_with(
    w0: &Scalar,
    w1: &Scalar,
    x: &Scalar,
    context: &[u8],
    prover_identity: &[u8],
    verifier_identity: &[u8],
    verifier_share: &[u8],
) -> Result<Proof, Error> {
    let m = point(&base16ct::lower::decode_vec(M).expect("M is hex"))
        .expect("M is a point on the curve");
    let n = point(&base16ct::lower::decode_vec(N).expect("N is hex"))
        .expect("N is a point on the curve");

    let share = ProjectivePoint::GENERATOR * x + m * w0;

    let verifier_share = point(verifier_share).ok_or_else(|| {
        Error::Tapo(TapoResponseError::ResponseError {
            description: "The device's share (`dev_share`) is not a point on the P-256 curve"
                .to_string(),
        })
    })?;
    let unmasked = verifier_share - n * w0;
    let z = unmasked * x;
    let v = unmasked * w1;

    if z == ProjectivePoint::IDENTITY || v == ProjectivePoint::IDENTITY {
        return Err(Error::Tapo(TapoResponseError::ResponseError {
            description: "The device's share (`dev_share`) leads to a key that is not secret"
                .to_string(),
        }));
    }

    let share = uncompressed(&share);
    let verifier_share = uncompressed(&verifier_share);

    let transcript = transcript(&[
        context,
        prover_identity,
        verifier_identity,
        &uncompressed(&m),
        &uncompressed(&n),
        &share,
        &verifier_share,
        &uncompressed(&z),
        &uncompressed(&v),
        // Always the 32 bytes of the group order, however small `w0` is.
        &w0.to_bytes(),
    ]);
    let main_key = crypto::sha256(&transcript);

    let confirm_keys = crypto::hkdf_sha256(&main_key, &[], b"ConfirmationKeys", 64)?;
    let (prover_confirm_key, verifier_confirm_key) = confirm_keys.split_at(32);
    let shared_key = crypto::hkdf_sha256(&main_key, &[], b"SharedKey", 32)?;

    Ok(Proof {
        confirm: crypto::hmac_sha256(prover_confirm_key, &verifier_share),
        share,
        shared_key,
        verifier_confirm_key: verifier_confirm_key.to_vec(),
    })
}

/// The `items` one after the other, each behind its length as 8 little-endian bytes.
fn transcript(items: &[&[u8]]) -> Vec<u8> {
    let mut transcript = Vec::new();
    for item in items {
        transcript.extend_from_slice(&(item.len() as u64).to_le_bytes());
        transcript.extend_from_slice(item);
    }
    transcript
}

/// The point that the SEC1 `bytes` encode, unless they encode the identity
/// or no point of the curve at all.
fn point(bytes: &[u8]) -> Option<ProjectivePoint> {
    PublicKey::from_sec1_bytes(bytes)
        .ok()
        .map(|key| key.to_projective())
}

fn uncompressed(point: &ProjectivePoint) -> Vec<u8> {
    point.to_sec1_point(false).as_bytes().to_vec()
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    fn hex(value: &str) -> Vec<u8> {
        base16ct::lower::decode_vec(value).unwrap()
    }

    fn scalar(value: &str) -> Scalar {
        scalar_from_wide(&hex(value))
    }

    #[test]
    fn prove_matches_the_rfc_9383_vector() {
        let proof = prove_with(
            &scalar("bb8e1bbcf3c48f62c08db243652ae55d3e5586053fca77102994f23ad95491b3"),
            &scalar("7e945f34d78785b8a3ef44d0df5a1a97d6b3b460409a345ca7830387a74b1dba"),
            &scalar("d1232c8e8693d02368976c174e2088851b8365d0d79a9eee709c6a05a2fad539"),
            b"SPAKE2+-P256-SHA256-HKDF-SHA256-HMAC-SHA256 Test Vectors",
            b"client",
            b"server",
            &hex(
                "04c0f65da0d11927bdf5d560c69e1d7d939a05b0e88291887d679fcadea75810fb\
                 5cc1ca7494db39e82ff2f50665255d76173e09986ab46742c798a9a68437b048",
            ),
        )
        .unwrap();

        assert_eq!(
            proof.share,
            hex(
                "04ef3bd051bf78a2234ec0df197f7828060fe9856503579bb1733009042c15c0c1\
                 de127727f418b5966afadfdd95a6e4591d171056b333dab97a79c7193e341727"
            )
        );
        assert_eq!(
            proof.confirm.to_vec(),
            hex("926cc713504b9b4d76c9162ded04b5493e89109f6d89462cd33adc46fda27527")
        );
        assert_eq!(
            proof.shared_key,
            hex("0c5f8ccd1413423a54f6c1fb26ff01534a87f893779c6e68666d772bfd91f3e7")
        );
        assert_eq!(
            proof.verifier_confirm_key,
            hex("ccd53c7c1fa37b64a462b40db8be101cedcf838950162902054e644b400f1680")
        );

        let verifier_confirm =
            hex("9747bcc4f8fe9f63defee53ac9b07876d907d55047e6ff2def2e7529089d3e68");
        assert!(proof.verify(&verifier_confirm));
        assert!(!proof.verify(&proof.confirm));
    }

    #[test]
    fn prove_rejects_a_share_that_is_not_on_the_curve() {
        let mut share = vec![0x04];
        share.extend_from_slice(&[0x01; 64]);

        let result = prove("credential", &[0; 16], 1, &[0; 32], &[0; 32], &share);

        assert!(matches!(
            result.err(),
            Some(Error::Tapo(TapoResponseError::ResponseError { .. }))
        ));
    }

    #[test]
    fn prove_rejects_a_share_that_cancels_the_password() {
        // A share of `w0 * N` leaves the identity once the password is taken out of it.
        let (w0, _) = derive_w0_w1("credential", &[0; 16], 1);
        let share = uncompressed(&(point(&hex(N)).unwrap() * w0));

        let result = prove("credential", &[0; 16], 1, &[0; 32], &[0; 32], &share);

        assert!(matches!(
            result.err(),
            Some(Error::Tapo(TapoResponseError::ResponseError { .. }))
        ));
    }

    #[test]
    fn scalar_from_wide_reduces_modulo_the_group_order() {
        let order = "ffffffff00000000ffffffffffffffffbce6faada7179e84f3b9cac2fc632551";

        assert_eq!(scalar("05"), Scalar::from(5u64));
        assert_eq!(scalar(order), Scalar::ZERO);
        // The order plus 5.
        assert_eq!(
            scalar("ffffffff00000000ffffffffffffffffbce6faada7179e84f3b9cac2fc632556"),
            Scalar::from(5u64)
        );
        // 2^256, which no longer fits 32 bytes.
        assert_eq!(
            scalar(&format!("01{}", "00".repeat(32)))
                .to_bytes()
                .to_vec(),
            hex("00000000ffffffff00000000000000004319055258e8617b0c46353d039cdaaf")
        );
        // 2^320 - 1, the largest value that 40 bytes hold.
        assert_eq!(
            scalar(&"ff".repeat(40)).to_bytes().to_vec(),
            hex("fffffffe00000001431905529c0166cd22159165b6faae70f756a571fc632550")
        );
    }

    #[test]
    fn derive_w0_w1_reduces_each_half_of_the_pbkdf2_output() {
        let (w0, w1) = derive_w0_w1(
            "f3bbbd66a63d4bf1747940578ec3d0103530e21d",
            b"0123456789abcdef",
            3000,
        );

        assert_eq!(
            w0.to_bytes().to_vec(),
            hex("f01588c03f935783a7c530a83ab1ade1a3261b13005521b6946dc5c7ad259f2c")
        );
        assert_eq!(
            w1.to_bytes().to_vec(),
            hex("de9bd4a17ba3b23e2a26fbf5ae94010c68f5e0e34388fe6ec227fcb45bd0b42e")
        );
    }

    #[test]
    fn credential_without_extra_crypt_is_unsupported() {
        let error = credential("hunter2", None, DeviceFamily::Smart).unwrap_err();

        assert!(error.to_string().contains("extra_crypt"), "{error}");
    }

    #[test]
    fn credential_of_a_camera_without_extra_crypt_is_the_md5_of_the_password() {
        assert_eq!(
            credential("hunter2", None, DeviceFamily::SmartCam).unwrap(),
            "2ab96390c7dbe3439de74d0c9b0b1767"
        );
    }

    #[test]
    fn credential_for_password_shadow_2_is_the_sha1_of_the_password() {
        let extra_crypt = json!({ "type": "password_shadow", "params": { "passwd_id": 2 } });

        assert_eq!(
            credential("hunter2", Some(&extra_crypt), DeviceFamily::Smart).unwrap(),
            "f3bbbd66a63d4bf1747940578ec3d0103530e21d"
        );
    }

    #[test]
    fn credential_for_an_unsupported_transform_names_it() {
        let extra_crypt = json!({ "type": "password_shadow", "params": { "passwd_id": 5 } });

        let error = credential("hunter2", Some(&extra_crypt), DeviceFamily::Smart).unwrap_err();

        assert!(error.to_string().contains("password_shadow"), "{error}");
        assert!(error.to_string().contains(r#""passwd_id":5"#), "{error}");
    }
}
