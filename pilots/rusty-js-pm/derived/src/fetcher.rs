//! PM-R2 fetcher/extractor (PM-EXT 6).
//!
//! Takes a `ResolvedDep` from PM-R1, downloads the tarball via
//! `pm_http_get`, verifies integrity (SRI sha-512 preferred, sha-1
//! shasum fallback), gunzips, untars into a staging directory.
//!
//! First-cut scope per Doc 732 §VI:
//! - Tarballs only (.tgz / .tar.gz). The npm registry serves these
//!   universally; no .tar.xz or .zip alternatives exist on the
//!   protocol.
//! - Integrity is REQUIRED: the resolved dep must have at least one of
//!   `integrity` or `shasum`. A missing-both case is a registry-side
//!   fault and is rejected loudly.
//! - Staging layout: every tarball entry has a `package/` prefix per
//!   the npm pack convention. The extractor strips that one leading
//!   path component and writes into `<staging_dir>/<rest>`.
//! - Symlinks and hardlinks in tarball entries are rejected (npm packs
//!   regular files + directories only; anything else is a packing
//!   bug or attempted symlink escape). Absolute paths and `..`
//!   components in entry paths are rejected (zip-slip / tar-slip
//!   defense).

use std::path::{Component, Path, PathBuf};

use flate2::read::GzDecoder;

use crate::http::{pm_http_get_follow, HttpError};
use crate::integrity::{verify_shasum, verify_sri, IntegrityError};
use crate::resolver::ResolvedDep;

#[derive(Debug)]
pub enum FetchError {
    Http(HttpError),
    NoIntegrity,
    Integrity(IntegrityError),
    Io(String),
    UnsafePath(String),
    UnsafeEntryKind(String),
    Tar(String),
}

impl From<HttpError> for FetchError {
    fn from(e: HttpError) -> Self {
        FetchError::Http(e)
    }
}

/// Outcome of a fetch+extract: the on-disk staging directory plus the
/// count of files written. The caller (PM-R3 linker) consumes the
/// staging directory.
#[derive(Debug)]
pub struct FetchedPackage {
    pub staging_dir: PathBuf,
    pub file_count: usize,
}

/// Download the tarball for `dep`, verify integrity, extract under
/// `staging_dir`. The staging dir must not yet exist (the caller is
/// responsible for picking an unused path, typically via tempdir).
pub fn fetch_and_extract(
    dep: &ResolvedDep,
    staging_dir: &Path,
) -> Result<FetchedPackage, FetchError> {
    if dep.integrity.is_none() && dep.shasum.is_none() {
        return Err(FetchError::NoIntegrity);
    }

    // 5 hops is generous: registry.npmmirror.com → cdn.npmmirror.com
    // is a single hop in practice.
    let bytes = pm_http_get_follow(&dep.tarball_url, 5)?;

    // SRI preferred (sha-512, modern); shasum fallback (sha-1, legacy).
    // Doc 732 §VI: integrity is mandatory; we reject the no-verify path.
    if let Some(sri) = &dep.integrity {
        verify_sri(&bytes, sri).map_err(FetchError::Integrity)?;
    } else if let Some(shasum) = &dep.shasum {
        verify_shasum(&bytes, shasum).map_err(FetchError::Integrity)?;
    }

    std::fs::create_dir_all(staging_dir)
        .map_err(|e| FetchError::Io(format!("create staging {staging_dir:?}: {e}")))?;

    let gz = GzDecoder::new(&bytes[..]);
    let mut archive = tar::Archive::new(gz);
    let mut count = 0usize;

    for entry in archive
        .entries()
        .map_err(|e| FetchError::Tar(format!("{e}")))?
    {
        let mut entry = entry.map_err(|e| FetchError::Tar(format!("{e}")))?;
        let kind = entry.header().entry_type();

        // First cut: regular files + directories only. npm packs nothing
        // else; symlink/hardlink entries are either a packing bug or a
        // tarslip attempt.
        if !(kind.is_file() || kind.is_dir()) {
            return Err(FetchError::UnsafeEntryKind(format!("{:?}", kind)));
        }

        let raw_path = entry
            .path()
            .map_err(|e| FetchError::Tar(format!("path: {e}")))?
            .into_owned();
        let safe = sanitize_entry_path(&raw_path)?;
        let dest = staging_dir.join(&safe);

        if kind.is_dir() {
            std::fs::create_dir_all(&dest)
                .map_err(|e| FetchError::Io(format!("mkdir {dest:?}: {e}")))?;
            continue;
        }
        if let Some(parent) = dest.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| FetchError::Io(format!("mkdir {parent:?}: {e}")))?;
        }
        let mut out = std::fs::File::create(&dest)
            .map_err(|e| FetchError::Io(format!("create {dest:?}: {e}")))?;
        std::io::copy(&mut entry, &mut out)
            .map_err(|e| FetchError::Io(format!("write {dest:?}: {e}")))?;
        count += 1;
    }

    Ok(FetchedPackage {
        staging_dir: staging_dir.to_path_buf(),
        file_count: count,
    })
}

/// Strip the single leading `package/` component that npm packs into
/// every tarball entry, then validate that the result has no absolute
/// path, no `..` components, and no other Windows/UNC oddities. This is
/// the tarslip-defense gate.
fn sanitize_entry_path(p: &Path) -> Result<PathBuf, FetchError> {
    let mut components = p.components();
    let first = components.next();

    let mut out = PathBuf::new();
    // npm convention: every entry starts with `package/`. If the first
    // component isn't that, accept it anyway (some older packs use
    // alternative prefixes) — but never strip more than one component.
    match first {
        Some(Component::Normal(s)) if s == "package" => { /* strip */ }
        Some(Component::Normal(s)) => out.push(s),
        Some(Component::CurDir) => { /* skip leading "./" */ }
        Some(Component::RootDir) | Some(Component::Prefix(_)) => {
            return Err(FetchError::UnsafePath(p.display().to_string()));
        }
        Some(Component::ParentDir) => {
            return Err(FetchError::UnsafePath(p.display().to_string()));
        }
        None => return Err(FetchError::UnsafePath(p.display().to_string())),
    }

    for c in components {
        match c {
            Component::Normal(s) => out.push(s),
            Component::CurDir => {}
            Component::ParentDir | Component::RootDir | Component::Prefix(_) => {
                return Err(FetchError::UnsafePath(p.display().to_string()));
            }
        }
    }
    if out.as_os_str().is_empty() {
        return Err(FetchError::UnsafePath(p.display().to_string()));
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn tmp_staging(suffix: &str) -> PathBuf {
        let mut p = std::env::temp_dir();
        p.push(format!(
            "cruftless-pm-test-{}-{}",
            suffix,
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        p
    }

    #[test]
    fn sanitize_strips_package_prefix() {
        let out = sanitize_entry_path(Path::new("package/lib/index.js")).unwrap();
        assert_eq!(out, PathBuf::from("lib/index.js"));
    }

    #[test]
    fn sanitize_rejects_absolute() {
        assert!(sanitize_entry_path(Path::new("/etc/passwd")).is_err());
    }

    #[test]
    fn sanitize_rejects_parent_dir() {
        assert!(sanitize_entry_path(Path::new("package/../../etc/passwd")).is_err());
    }

    #[test]
    fn sanitize_keeps_first_component_when_not_package() {
        let out = sanitize_entry_path(Path::new("other/file.js")).unwrap();
        assert_eq!(out, PathBuf::from("other/file.js"));
    }

    /// Network-dependent end-to-end: resolve lodash 4.17.21, fetch its
    /// tarball, verify SRI, extract to a tmpdir, assert package.json is
    /// present with the expected version.
    #[test]
    #[ignore]
    fn fetch_lodash_end_to_end() {
        use crate::resolver::{resolve_specifier, DEFAULT_REGISTRY};
        let dep = resolve_specifier(DEFAULT_REGISTRY, "lodash", "4.17.21").expect("resolve");
        let staging = tmp_staging("lodash");
        let result = fetch_and_extract(&dep, &staging).expect("fetch+extract");
        assert!(
            result.file_count > 100,
            "expected >100 files in lodash, got {}",
            result.file_count
        );
        let pkg_json_path = result.staging_dir.join("package.json");
        let body = std::fs::read_to_string(&pkg_json_path)
            .unwrap_or_else(|e| panic!("read {pkg_json_path:?}: {e}"));
        assert!(
            body.contains("\"version\": \"4.17.21\""),
            "package.json missing expected version: {}",
            &body[..body.len().min(200)]
        );
        // cleanup
        let _ = std::fs::remove_dir_all(&staging);
    }
}
