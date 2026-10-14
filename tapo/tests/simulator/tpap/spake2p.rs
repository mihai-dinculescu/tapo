//! The verifier side of SPAKE2+ (RFC 9383) on P-256 with SHA-256, HKDF-SHA256
//! and HMAC-SHA256, which is the side of the TPAP login that a device runs.
//! Written from the RFC, independently of the prover in the library.

use p256::elliptic_curve::Generate;
use p256::elliptic_curve::ff::FromUniformBytes;
use p256::elliptic_curve::sec1::ToSec1Point;
use p256::{NonZeroScalar, ProjectivePoint, PublicKey, Scalar};

use crate::simulator::crypto;

/// The points `M` and `N` that RFC 9383 assigns to P-256, compressed.
const M: &str = "02886e2f97ace46e55ba9dd7242579f2993b64e16ef3dcab95afd497333d8fa12f";
const N: &str = "03d8bbd6c639c62937b04d997f38c3770719c629d7014d49a24b4f98baa1292b49";

/// One run of the verifier, which holds its secret until the prover's share
/// and confirmation arrive.
pub struct Verifier {
    w0: Scalar,
    /// `L = w1 * G`, which is all a device keeps of `w1`.
    l: ProjectivePoint,
    y: Scalar,
    /// The verifier's share `Y`, uncompressed.
    share: Vec<u8>,
    context: [u8; 32],
}

impl Verifier {
    /// Starts a run for the `credential` that the client is to prove it
    /// knows, with the random values that both sides exchanged.
    pub fn new(
        credential: &str,
        salt: &[u8],
        iterations: u32,
        user_random: &[u8],
        device_random: &[u8],
    ) -> Self {
        let key = crypto::pbkdf2_hmac_sha256(credential.as_bytes(), salt, iterations, 80);
        let (w0, w1) = key.split_at(40);
        let (w0, w1) = (scalar(w0), scalar(w1));

        let y = *NonZeroScalar::generate_from_rng(&mut rand::rng());
        let share = ProjectivePoint::GENERATOR * y + point(N) * w0;

        Self {
            w0,
            l: ProjectivePoint::GENERATOR * w1,
            y,
            share: uncompressed(&share),
            context: crypto::sha256(&[b"PAKE V1", user_random, device_random].concat()),
        }
    }

    pub fn share(&self) -> &[u8] {
        &self.share
    }

    /// The verifier's confirmation and the shared key, unless `prover_confirm`
    /// does not prove that the prover knows the password, or `prover_share` is
    /// not a point that the exchange can continue with.
    pub fn confirm(
        &self,
        prover_share: &[u8],
        prover_confirm: &[u8],
    ) -> Option<([u8; 32], Vec<u8>)> {
        let unmasked = PublicKey::from_sec1_bytes(prover_share)
            .ok()?
            .to_projective()
            - point(M) * self.w0;
        let z = unmasked * self.y;
        let v = self.l * self.y;

        if z == ProjectivePoint::IDENTITY || v == ProjectivePoint::IDENTITY {
            return None;
        }

        // Tapo uses no identities, so two empty strings stand in their place.
        let transcript = transcript(&[
            &self.context,
            b"",
            b"",
            &uncompressed(&point(M)),
            &uncompressed(&point(N)),
            prover_share,
            &self.share,
            &uncompressed(&z),
            &uncompressed(&v),
            &self.w0.to_bytes(),
        ]);
        let main_key = crypto::sha256(&transcript);

        let confirm_keys = crypto::hkdf_sha256(&main_key, &[], b"ConfirmationKeys", 64);
        let (prover_confirm_key, verifier_confirm_key) = confirm_keys.split_at(32);

        if crypto::hmac_sha256(prover_confirm_key, &self.share) != prover_confirm {
            return None;
        }

        Some((
            crypto::hmac_sha256(verifier_confirm_key, prover_share),
            crypto::hkdf_sha256(&main_key, &[], b"SharedKey", 32),
        ))
    }
}

/// The big-endian `bytes`, of at most 64 bytes, reduced modulo the group order.
fn scalar(bytes: &[u8]) -> Scalar {
    let mut wide = [0u8; 64];
    wide[64 - bytes.len()..].copy_from_slice(bytes);

    Scalar::from_uniform_bytes(&wide)
}

/// The `items` one after the other, each behind its length as 8 little-endian bytes.
fn transcript(items: &[&[u8]]) -> Vec<u8> {
    items
        .iter()
        .flat_map(|item| [&(item.len() as u64).to_le_bytes(), *item].concat())
        .collect()
}

fn point(hex: &str) -> ProjectivePoint {
    PublicKey::from_sec1_bytes(&base16ct::lower::decode_vec(hex).expect("the point is hex"))
        .expect("the point is on the curve")
        .to_projective()
}

fn uncompressed(point: &ProjectivePoint) -> Vec<u8> {
    point.to_sec1_point(false).as_bytes().to_vec()
}
