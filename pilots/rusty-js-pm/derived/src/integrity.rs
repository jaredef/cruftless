//! Subresource Integrity (SRI) verification per the registry's
//! `dist.integrity` field. Format: `<algo>-<base64-digest>`.
//!
//! First-cut support: sha-512 (the only algorithm modern npm uses for
//! `dist.integrity`). sha-256 and sha-384 would be straightforward
//! additions but the registry does not emit them as of 2026.
//!
//! Legacy fallback: `dist.shasum` is hex sha-1; verified separately
//! by `verify_shasum`. See docs/registry-response-schema.md §Class A
//! for precedence rules.

use base64::engine::general_purpose::STANDARD as B64_STANDARD;
use base64::Engine as _;
use sha1::{Digest as Sha1Digest, Sha1};
use sha2::{Digest as Sha2Digest, Sha512};

#[derive(Debug)]
pub enum IntegrityError {
    UnsupportedAlgorithm(String),
    MalformedSri(String),
    Base64Decode(String),
    DigestMismatch {
        algorithm: &'static str,
        expected_len: usize,
        got_len: usize,
    },
    HexDecode(String),
    ShasumMismatch,
}

/// Verify `bytes` matches the SRI string `sri` of the form
/// `<algo>-<base64-digest>`. First cut accepts `sha512-` only.
pub fn verify_sri(bytes: &[u8], sri: &str) -> Result<(), IntegrityError> {
    let dash = sri
        .find('-')
        .ok_or_else(|| IntegrityError::MalformedSri(sri.to_string()))?;
    let (algo, rest) = sri.split_at(dash);
    let b64 = &rest[1..];
    let expected = B64_STANDARD
        .decode(b64)
        .map_err(|e| IntegrityError::Base64Decode(format!("{e:?}")))?;
    match algo {
        "sha512" => {
            let actual = Sha512::digest(bytes);
            if actual.as_slice() == expected.as_slice() {
                Ok(())
            } else {
                Err(IntegrityError::DigestMismatch {
                    algorithm: "sha512",
                    expected_len: expected.len(),
                    got_len: actual.len(),
                })
            }
        }
        other => Err(IntegrityError::UnsupportedAlgorithm(other.to_string())),
    }
}

/// Verify `bytes` matches the hex sha-1 string `shasum`. Used only as
/// a legacy fallback when the registry response has no `integrity`
/// field (packages published pre-2017 SRI rollout). SHA-1 is broken;
/// this path exists for backward compatibility, not security.
pub fn verify_shasum(bytes: &[u8], shasum: &str) -> Result<(), IntegrityError> {
    let expected = hex::decode(shasum).map_err(|e| IntegrityError::HexDecode(format!("{e:?}")))?;
    let actual = Sha1::digest(bytes);
    if actual.as_slice() == expected.as_slice() {
        Ok(())
    } else {
        Err(IntegrityError::ShasumMismatch)
    }
}
