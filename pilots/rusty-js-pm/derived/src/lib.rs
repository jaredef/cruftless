//! rusty-js-pm — package manager for the Cruftless runtime.
//!
//! Doc 732 §VI (PM-R1 + PM-R2 + PM-R3) is the design target. This crate
//! owns:
//!
//! - The package.json parser for the Class-A + Class-B fields enumerated
//!   in `pilots/rusty-js-pm/docs/manifest-field-coverage.md`.
//! - The registry-response parser for the Class-A fields enumerated in
//!   `pilots/rusty-js-pm/docs/registry-response-schema.md`.
//! - The PM-R1 specifier resolver (specifier → ResolvedDep).
//! - The PM-R2 fetcher/extractor (URL+integrity → on-disk extracted tree).
//! - The PM-R3 linker (extracted staging → flat node_modules layout).
//! - The lockfile read/write codec.
//!
//! What this crate does NOT own (per Doc 732 §VI carve-outs):
//!
//! - Semver range resolution (deferred to second cut).
//! - Peer-dependency reconciliation (deferred).
//! - Lifecycle script execution (rejected in first cut per Class-D).
//! - Workspace support (deferred).
//! - Binary lockfile (`bun.lockb`) parity (deferred).
//! - Global cache / hardlink layer (deferred; first cut refetches).
//!
//! PM-EXT 3 scope: crate scaffold + toolchain smoke test. No registry
//! integration yet — the HTTP-client decision is its own PM-EXT (PM-EXT
//! 4) per the Cargo.toml comment block. The smoke test exercises the
//! on-disk toolchain (gunzip + untar + sha2 + serde) by round-tripping
//! a synthetic tarball end-to-end: build a tarball in-memory, gzip it,
//! hash the gzip, decompress, untar to a tmpdir, verify the extracted
//! file count and contents match what was put in. This verifies the
//! aarch64-linux Pi target builds and runs the toolchain crates before
//! any registry-facing substrate lands.

pub mod smoke;
pub mod integrity;
pub mod http;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke_roundtrip() {
        smoke::roundtrip_synthetic_tarball().expect("smoke test failed");
    }

    #[test]
    fn integrity_sri_sha512() {
        // Verify the SRI parser + verifier on a known vector.
        // SRI for empty input: sha512-z4PhNX7vuL3xVChQ1m2AB9Yg5AULVxXcg/SpIdNs6c5H0NE8XYXysP+DGNKHfuwvY7kxvUdBeoGlODJ6+SfaPg==
        let empty_sri = "sha512-z4PhNX7vuL3xVChQ1m2AB9Yg5AULVxXcg/SpIdNs6c5H0NE8XYXysP+DGNKHfuwvY7kxvUdBeoGlODJ6+SfaPg==";
        integrity::verify_sri(b"", empty_sri).expect("empty SRI must verify");
        integrity::verify_sri(b"x", empty_sri).expect_err("non-empty input must fail");
    }
}
