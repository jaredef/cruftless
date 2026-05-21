//! Lockfile read/write codec (PM-EXT 8).
//!
//! On-disk shape: a top-level JSON object with `lockfileVersion: 1` and
//! a `packages` map keyed by `"<name>@<version>"` (lexicographically
//! ordered for byte-stable output across runs). Each value is the
//! `ResolvedDep` record from PM-R1.
//!
//! Why a map keyed by specifier rather than an array: lookup by
//! `(name, version)` during PM-EXT 9's install-skip check is O(1), and
//! a stably-ordered serializer (`BTreeMap`) gives byte-identical
//! lockfiles for the same input regardless of resolution order.
//!
//! Path convention (decided here, used by PM-EXT 9): the lockfile
//! lives next to package.json as `cruftless-lock.json`. Naming it
//! distinctly from `package-lock.json` / `bun.lockb` avoids
//! cross-tool confusion: cruftless's first-cut lock is not a drop-in
//! for either.

use std::collections::BTreeMap;
use std::path::Path;

use crate::resolver::ResolvedDep;

pub const LOCKFILE_NAME: &str = "cruftless-lock.json";
pub const LOCKFILE_VERSION: u32 = 1;

#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub struct Lockfile {
    #[serde(rename = "lockfileVersion")]
    pub version: u32,
    pub packages: BTreeMap<String, ResolvedDep>,
}

#[derive(Debug)]
pub enum LockfileError {
    Io(String),
    Json(String),
    UnsupportedVersion(u32),
}

impl Lockfile {
    pub fn new() -> Self {
        Self { version: LOCKFILE_VERSION, packages: BTreeMap::new() }
    }

    pub fn from_resolved(deps: impl IntoIterator<Item = ResolvedDep>) -> Self {
        let mut lock = Self::new();
        for dep in deps { lock.insert(dep); }
        lock
    }

    pub fn insert(&mut self, dep: ResolvedDep) {
        let key = format!("{}@{}", dep.name, dep.version);
        self.packages.insert(key, dep);
    }

    pub fn get(&self, name: &str, version: &str) -> Option<&ResolvedDep> {
        self.packages.get(&format!("{name}@{version}"))
    }

    pub fn write_to(&self, path: &Path) -> Result<(), LockfileError> {
        let json = serde_json::to_string_pretty(self)
            .map_err(|e| LockfileError::Json(format!("{e}")))?;
        // newline-terminated for diff-friendliness
        let bytes = format!("{json}\n");
        std::fs::write(path, bytes)
            .map_err(|e| LockfileError::Io(format!("write {path:?}: {e}")))
    }

    pub fn read_from(path: &Path) -> Result<Self, LockfileError> {
        let bytes = std::fs::read(path)
            .map_err(|e| LockfileError::Io(format!("read {path:?}: {e}")))?;
        let lock: Lockfile = serde_json::from_slice(&bytes)
            .map_err(|e| LockfileError::Json(format!("{e}")))?;
        if lock.version != LOCKFILE_VERSION {
            return Err(LockfileError::UnsupportedVersion(lock.version));
        }
        Ok(lock)
    }
}

impl Default for Lockfile {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_dep(name: &str, version: &str) -> ResolvedDep {
        ResolvedDep {
            name: name.into(),
            version: version.into(),
            tarball_url: format!("https://cdn.example/{name}-{version}.tgz"),
            integrity: Some(format!("sha512-{name}{version}=")),
            shasum: None,
        }
    }

    #[test]
    fn roundtrip_empty() {
        let lock = Lockfile::new();
        let path = std::env::temp_dir().join(format!("lock-empty-{}.json",
            std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()));
        lock.write_to(&path).unwrap();
        let back = Lockfile::read_from(&path).unwrap();
        assert_eq!(lock, back);
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn roundtrip_two_deps_stable_order() {
        let mut lock = Lockfile::new();
        // Insert in non-sorted order; BTreeMap should sort.
        lock.insert(sample_dep("lodash", "4.17.21"));
        lock.insert(sample_dep("@babel/core", "7.24.0"));

        let path = std::env::temp_dir().join(format!("lock-stable-{}.json",
            std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()));
        lock.write_to(&path).unwrap();
        let serialized = std::fs::read_to_string(&path).unwrap();
        let babel_pos = serialized.find("@babel/core").unwrap();
        let lodash_pos = serialized.find("lodash").unwrap();
        assert!(babel_pos < lodash_pos, "BTreeMap should sort @babel before lodash");

        let back = Lockfile::read_from(&path).unwrap();
        assert_eq!(lock, back);
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn get_by_name_version() {
        let mut lock = Lockfile::new();
        lock.insert(sample_dep("lodash", "4.17.21"));
        assert!(lock.get("lodash", "4.17.21").is_some());
        assert!(lock.get("lodash", "4.17.22").is_none());
        assert!(lock.get("underscore", "4.17.21").is_none());
    }

    #[test]
    fn rejects_unsupported_version() {
        let path = std::env::temp_dir().join(format!("lock-badver-{}.json",
            std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()));
        std::fs::write(&path, br#"{"lockfileVersion":999,"packages":{}}"#).unwrap();
        let r = Lockfile::read_from(&path);
        assert!(matches!(r, Err(LockfileError::UnsupportedVersion(999))));
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn byte_stable_across_runs() {
        // Two locks with the same deps inserted in different orders must
        // serialize to byte-identical output.
        let mut a = Lockfile::new();
        a.insert(sample_dep("z-pkg", "1.0.0"));
        a.insert(sample_dep("a-pkg", "1.0.0"));
        let mut b = Lockfile::new();
        b.insert(sample_dep("a-pkg", "1.0.0"));
        b.insert(sample_dep("z-pkg", "1.0.0"));

        let pa = std::env::temp_dir().join("lock-stable-a.json");
        let pb = std::env::temp_dir().join("lock-stable-b.json");
        a.write_to(&pa).unwrap();
        b.write_to(&pb).unwrap();
        let sa = std::fs::read_to_string(&pa).unwrap();
        let sb = std::fs::read_to_string(&pb).unwrap();
        assert_eq!(sa, sb, "lockfile must be byte-stable across insertion orders");
        let _ = std::fs::remove_file(&pa);
        let _ = std::fs::remove_file(&pb);
    }
}
