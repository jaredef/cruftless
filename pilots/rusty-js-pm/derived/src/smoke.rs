//! PM-EXT 3 toolchain smoke test.
//!
//! Builds a synthetic tarball in memory (three files with known
//! contents), gzips it, verifies the gzip's SHA-512 round-trips through
//! the SRI verifier, decompresses, untars into a tmpdir, and asserts
//! the extracted files match what was put in. No network, no fixture
//! files on disk. Verifies the on-disk toolchain (flate2 + tar + sha2 +
//! base64) builds and runs on the aarch64-linux Pi target before any
//! registry-facing substrate lands.
//!
//! Equivalent in shape to JIT-EXT 2's `smoke_test_add()` for Cranelift.

use std::io::{Cursor, Read, Write};
use std::path::PathBuf;

use base64::engine::general_purpose::STANDARD as B64_STANDARD;
use base64::Engine as _;
use flate2::{read::GzDecoder, write::GzEncoder, Compression};
use sha2::{Digest as _, Sha512};
use tar::{Archive, Builder, Header};

use crate::integrity;

#[derive(Debug)]
pub enum SmokeError {
    Io(std::io::Error),
    Integrity(integrity::IntegrityError),
    Mismatch(String),
}

impl From<std::io::Error> for SmokeError {
    fn from(e: std::io::Error) -> Self {
        SmokeError::Io(e)
    }
}
impl From<integrity::IntegrityError> for SmokeError {
    fn from(e: integrity::IntegrityError) -> Self {
        SmokeError::Integrity(e)
    }
}

pub fn roundtrip_synthetic_tarball() -> Result<(), SmokeError> {
    // (1) Build a tarball in memory with three known files. Mirrors
    // the npm tarball convention of files living under `package/`.
    let files: &[(&str, &[u8])] = &[
        (
            "package/package.json",
            br#"{"name":"smoke","version":"0.0.0"}"#,
        ),
        ("package/index.js", b"module.exports = 42;\n"),
        ("package/README.md", b"# smoke\n"),
    ];
    let tar_bytes: Vec<u8> = {
        let mut buf = Vec::new();
        {
            let mut builder = Builder::new(&mut buf);
            for (path, contents) in files {
                let mut header = Header::new_gnu();
                header.set_size(contents.len() as u64);
                header.set_mode(0o644);
                header.set_cksum();
                builder.append_data(&mut header, path, *contents)?;
            }
            builder.finish()?;
        }
        buf
    };

    // (2) Gzip it.
    let gz_bytes: Vec<u8> = {
        let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(&tar_bytes)?;
        encoder.finish()?
    };

    // (3) Compute SHA-512 of the gzip bytes, emit as SRI, verify
    // through the same path PM-R2 will use on real registry responses.
    let digest = Sha512::digest(&gz_bytes);
    let sri = format!("sha512-{}", B64_STANDARD.encode(digest));
    integrity::verify_sri(&gz_bytes, &sri)?;

    // (4) Decompress + untar into a tmpdir.
    let dest: PathBuf = {
        let mut p = std::env::temp_dir();
        p.push(format!("rusty-js-pm-smoke-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&p);
        std::fs::create_dir_all(&p)?;
        p
    };
    {
        let mut decoder = GzDecoder::new(Cursor::new(&gz_bytes));
        let mut tar = Vec::new();
        decoder.read_to_end(&mut tar)?;
        let mut archive = Archive::new(Cursor::new(tar));
        archive.unpack(&dest)?;
    }

    // (5) Verify extracted contents match input. This is the
    // load-bearing assertion: the toolchain round-trip preserved
    // the byte stream end-to-end.
    for (path, expected) in files {
        let mut full = dest.clone();
        full.push(path);
        let got = std::fs::read(&full)
            .map_err(|e| SmokeError::Mismatch(format!("read {}: {e}", full.display())))?;
        if got != *expected {
            return Err(SmokeError::Mismatch(format!(
                "content mismatch at {}: expected {} bytes, got {} bytes",
                path,
                expected.len(),
                got.len()
            )));
        }
    }

    // (6) Cleanup. Cheap; failure is non-fatal because the tmpdir
    // is process-id-scoped.
    let _ = std::fs::remove_dir_all(&dest);
    Ok(())
}
