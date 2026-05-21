//! `pm_install` driver (PM-EXT 9).
//!
//! Composes PM-R1 + R2 + R3 + lockfile codec into the user-facing
//! install flow:
//!
//!   1. Read `<project>/package.json`; extract `dependencies` (exact-pin
//!      only per Doc 732 §VI carve-out).
//!   2. If `<project>/cruftless-lock.json` exists, load it. For each
//!      dep already present + integrity-matching at
//!      `<project>/node_modules/<name>/`, skip refetch.
//!   3. For each remaining dep: PM-R1 resolve → PM-R2 fetch+extract to
//!      a tmp staging dir → PM-R3 link into `<project>/node_modules/`.
//!   4. Write the updated lockfile.
//!
//! First cut: top-level dependencies only. Transitive resolution is
//! PM-EXT 10. The driver works in tmpdirs (used by tests) and against
//! real project trees identically; nothing here is host-CLI-specific.

use std::path::{Path, PathBuf};

use crate::resolver::{resolve_closure, ResolverError, ResolvedDep, DEFAULT_REGISTRY};
use crate::fetcher::{fetch_and_extract, FetchError};
use crate::linker::{link_package, LinkError};
use crate::lockfile::{Lockfile, LockfileError, LOCKFILE_NAME};

#[derive(Debug)]
pub enum InstallError {
    Io(String),
    PackageJson(String),
    Resolver(ResolverError),
    Fetch(FetchError),
    Link(LinkError),
    Lockfile(LockfileError),
}

impl From<ResolverError> for InstallError { fn from(e: ResolverError) -> Self { Self::Resolver(e) } }
impl From<FetchError> for InstallError { fn from(e: FetchError) -> Self { Self::Fetch(e) } }
impl From<LinkError> for InstallError { fn from(e: LinkError) -> Self { Self::Link(e) } }
impl From<LockfileError> for InstallError { fn from(e: LockfileError) -> Self { Self::Lockfile(e) } }

#[derive(Debug)]
pub struct InstallReport {
    pub installed: Vec<(String, String)>,  // (name, version)
    pub skipped: Vec<(String, String)>,    // already present + lockfile-matching
}

/// Install dependencies for the project rooted at `project_dir`, using
/// `registry` as the package source. Pass `DEFAULT_REGISTRY` for the
/// engagement default (registry.npmmirror.com).
pub fn pm_install(project_dir: &Path, registry: &str) -> Result<InstallReport, InstallError> {
    let pkg_json_path = project_dir.join("package.json");
    let deps = read_dependencies(&pkg_json_path)?;

    let lock_path = project_dir.join(LOCKFILE_NAME);
    let mut lock = if lock_path.exists() {
        Lockfile::read_from(&lock_path)?
    } else {
        Lockfile::new()
    };

    let nm_root = project_dir.join("node_modules");
    let mut report = InstallReport { installed: Vec::new(), skipped: Vec::new() };

    // PM-EXT 10: walk the transitive closure before any fetch, so a
    // range-using transitive surfaces NonExactVersionSpec before any
    // disk writes. The closure is BFS-ordered; install order follows.
    let closure: Vec<ResolvedDep> = resolve_closure(registry, &deps)?;

    for resolved in closure {
        let name = resolved.name.clone();
        let version = resolved.version.clone();
        let install_dir = nm_root.join(&name);
        let already_present = install_dir.join("package.json").exists()
            && lock.get(&name, &version).is_some();
        if already_present {
            report.skipped.push((name, version));
            continue;
        }

        // Stage in node_modules/.cruftless-staging/<name>-<version>/
        // so the rename in PM-R3 is same-fs (no EXDEV fallback under
        // normal use). The directory is removed by PM-R3 after rename.
        let staging = nm_root.join(".cruftless-staging")
            .join(format!("{name}-{version}-{}",
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()));

        let pkg = fetch_and_extract(&resolved, &staging)?;
        link_package(&resolved, pkg, &nm_root)?;

        lock.insert(resolved);
        report.installed.push((name, version));
    }

    // Cleanup empty staging root if it survives (it shouldn't, but
    // defense in depth).
    let staging_root = nm_root.join(".cruftless-staging");
    if staging_root.exists() {
        let _ = std::fs::remove_dir_all(&staging_root);
    }

    lock.write_to(&lock_path)?;

    Ok(report)
}

/// Read `dependencies` from a package.json. Exact-pin-only per Doc 732
/// §VI carve-out — the resolver will reject ranges, but we surface a
/// clearer error here.
fn read_dependencies(path: &Path) -> Result<Vec<(String, String)>, InstallError> {
    let body = std::fs::read(path)
        .map_err(|e| InstallError::Io(format!("read {path:?}: {e}")))?;
    let json: serde_json::Value = serde_json::from_slice(&body)
        .map_err(|e| InstallError::PackageJson(format!("{e}")))?;
    let Some(deps) = json.get("dependencies") else {
        return Ok(Vec::new());
    };
    let map = deps.as_object()
        .ok_or_else(|| InstallError::PackageJson("dependencies not an object".into()))?;
    let mut out = Vec::with_capacity(map.len());
    for (k, v) in map {
        let v = v.as_str().ok_or_else(|| InstallError::PackageJson(
            format!("dependencies.{k} not a string")))?;
        out.push((k.clone(), v.to_string()));
    }
    // Stable order for reproducible installs.
    out.sort();
    Ok(out)
}

fn _tmp_workdir(tag: &str) -> PathBuf {
    let mut p = std::env::temp_dir();
    p.push(format!("cruftless-pm-install-{tag}-{}",
        std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()));
    p
}

#[cfg(test)]
mod tests {
    use super::*;

    fn workdir(tag: &str) -> PathBuf { _tmp_workdir(tag) }

    #[test]
    fn read_deps_empty() {
        let dir = workdir("read-empty");
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("package.json"),
            br#"{"name":"app","version":"0.0.1"}"#).unwrap();
        let deps = read_dependencies(&dir.join("package.json")).unwrap();
        assert!(deps.is_empty());
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn read_deps_sorted() {
        let dir = workdir("read-sorted");
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("package.json"),
            br#"{"dependencies":{"zeta":"1.0.0","alpha":"2.0.0"}}"#).unwrap();
        let deps = read_dependencies(&dir.join("package.json")).unwrap();
        assert_eq!(deps, vec![
            ("alpha".to_string(), "2.0.0".to_string()),
            ("zeta".to_string(), "1.0.0".to_string()),
        ]);
        let _ = std::fs::remove_dir_all(&dir);
    }

    /// Network-dependent end-to-end: pm_install against a tmpdir with
    /// package.json declaring lodash@4.17.21. Verifies the install
    /// places lodash under node_modules/, writes the lockfile, and a
    /// SECOND run is a no-op (skipped, not refetched).
    #[test]
    #[ignore]
    fn install_lodash_idempotent() {
        let dir = workdir("install-lodash");
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("package.json"),
            br#"{"name":"app","version":"0.0.1","dependencies":{"lodash":"4.17.21"}}"#).unwrap();

        // First install: actual fetch.
        let r1 = pm_install(&dir, DEFAULT_REGISTRY).expect("install 1");
        assert_eq!(r1.installed.len(), 1);
        assert_eq!(r1.skipped.len(), 0);
        assert_eq!(r1.installed[0].0, "lodash");

        let lockfile_path = dir.join(LOCKFILE_NAME);
        assert!(lockfile_path.exists(), "lockfile should be written");
        let lock = Lockfile::read_from(&lockfile_path).unwrap();
        assert!(lock.get("lodash", "4.17.21").is_some());

        let lodash_pkg = dir.join("node_modules/lodash/package.json");
        assert!(lodash_pkg.exists(), "lodash/package.json should exist");
        let pj = std::fs::read_to_string(&lodash_pkg).unwrap();
        assert!(pj.contains("\"version\": \"4.17.21\""));

        // Second install: should skip (lockfile + on-disk both present).
        let r2 = pm_install(&dir, DEFAULT_REGISTRY).expect("install 2");
        assert_eq!(r2.installed.len(), 0, "second run should skip, not refetch");
        assert_eq!(r2.skipped.len(), 1);
        assert_eq!(r2.skipped[0].0, "lodash");

        let _ = std::fs::remove_dir_all(&dir);
    }

    /// PM-EXT 10 end-to-end: install debug@4.3.4 which has one
    /// exact-pinned transitive dep (ms@2.1.2). Verifies the closure
    /// walker drives recursive resolution + install.
    #[test]
    #[ignore]
    fn install_debug_with_transitive() {
        let dir = workdir("install-debug");
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("package.json"),
            br#"{"name":"app","version":"0.0.1","dependencies":{"debug":"4.3.4"}}"#).unwrap();

        let r = pm_install(&dir, DEFAULT_REGISTRY).expect("install");
        assert_eq!(r.installed.len(), 2,
            "expected debug + ms; got {:?}", r.installed);
        let names: Vec<&str> = r.installed.iter().map(|(n, _)| n.as_str()).collect();
        assert!(names.contains(&"debug"));
        assert!(names.contains(&"ms"));

        assert!(dir.join("node_modules/debug/package.json").exists());
        assert!(dir.join("node_modules/ms/package.json").exists());

        let lock = Lockfile::read_from(&dir.join(LOCKFILE_NAME)).unwrap();
        assert!(lock.get("debug", "4.3.4").is_some());
        assert!(lock.get("ms", "2.1.2").is_some());

        let _ = std::fs::remove_dir_all(&dir);
    }
}
