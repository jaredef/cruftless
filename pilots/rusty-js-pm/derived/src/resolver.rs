//! PM-R1 specifier resolver (PM-EXT 5).
//!
//! Per Doc 732 §VI first-cut scope: takes a `(name, version)` pair where
//! `version` is an exact pin (no semver ranges, no tags, no git, no
//! file, no workspace specifiers). Resolves it against a configurable
//! npm-protocol-compatible registry by fetching the per-version
//! manifest endpoint `/{name}/{version}` and extracting the four
//! Class-A fields enumerated in `docs/registry-response-schema.md`:
//!
//! - `versions.<v>` (the per-version manifest object — implicit at
//!   the per-version endpoint, which returns it directly)
//! - `versions.<v>.dist.tarball` — tarball URL
//! - `versions.<v>.dist.integrity` — SRI string (sha512-<b64>)
//! - `versions.<v>.dist.shasum` — legacy fallback (hex sha1)
//!
//! The default registry is `registry.npmmirror.com` per the Doc 730
//! §XVI Case-4 endpoint-substitution scope decision (npmjs.org's
//! Cloudflare edge enforces TLS 1.2 only; cruftless's pilot is TLS
//! 1.3 only per seed §IV carve-out). npmmirror.com is npm-protocol
//! compatible and TLS 1.3 reachable through cruftless's substrate.

use std::collections::{BTreeMap, BTreeSet, VecDeque};

use crate::http::{pm_http_get, HttpError};

/// Default registry endpoint for the PM first cut. Chosen per Doc 732
/// §VII §XVI Case-4 scope decision: npmmirror.com is npm-protocol
/// compatible and TLS 1.3 reachable; registry.npmjs.org is not
/// (TLS 1.2 only, beyond the seed §IV carve-out).
pub const DEFAULT_REGISTRY: &str = "https://registry.npmmirror.com";

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub struct ResolvedDep {
    pub name: String,
    pub version: String,
    pub tarball_url: String,
    pub integrity: Option<String>,  // SRI: sha512-<b64>
    pub shasum: Option<String>,     // hex sha1 (legacy fallback)
    /// Transitive deps as declared in the per-version manifest's
    /// `dependencies` object. Stored verbatim; ranges will be rejected
    /// by `resolve_specifier` when the closure walker recurses into
    /// them. Empty map for leaf packages.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub dependencies: BTreeMap<String, String>,
}

#[derive(Debug)]
pub enum ResolverError {
    Http(HttpError),
    Json(String),
    MissingField(&'static str),
    NonExactVersionSpec(String),
}

impl From<HttpError> for ResolverError {
    fn from(e: HttpError) -> Self { ResolverError::Http(e) }
}

/// Resolve a single `(name, version)` against the configured registry.
/// Returns a `ResolvedDep` with the tarball URL and integrity for the
/// PM-R2 fetcher.
pub fn resolve_specifier(
    registry: &str,
    name: &str,
    version: &str,
) -> Result<ResolvedDep, ResolverError> {
    // First-cut: only exact-pinned versions per Doc 732 §VI carve-out
    // and `docs/manifest-field-coverage.md` Class A. Range syntax
    // (caret, tilde, "*", complex) is rejected loudly.
    if version.starts_with('^') || version.starts_with('~') || version.contains('*')
        || version.contains(' ') || version.contains("||")
    {
        return Err(ResolverError::NonExactVersionSpec(version.to_string()));
    }

    let url = format!("{}/{}/{}", registry.trim_end_matches('/'), name, version);
    let body = pm_http_get(&url)?;
    let json: serde_json::Value = serde_json::from_slice(&body)
        .map_err(|e| ResolverError::Json(format!("{e:?}")))?;

    // Per docs/registry-response-schema.md §2 endpoint B: the per-
    // version manifest returns the version's manifest object directly
    // (no "versions" wrapping).
    let name_returned = json.get("name").and_then(|v| v.as_str())
        .ok_or(ResolverError::MissingField("name"))?;
    let version_returned = json.get("version").and_then(|v| v.as_str())
        .ok_or(ResolverError::MissingField("version"))?;
    let dist = json.get("dist")
        .ok_or(ResolverError::MissingField("dist"))?;
    let tarball = dist.get("tarball").and_then(|v| v.as_str())
        .ok_or(ResolverError::MissingField("dist.tarball"))?;
    let integrity = dist.get("integrity").and_then(|v| v.as_str()).map(String::from);
    let shasum = dist.get("shasum").and_then(|v| v.as_str()).map(String::from);

    // Parse transitive deps (Class A field per docs/registry-response-
    // schema.md). Absent means zero-transitive leaf.
    let mut dependencies = BTreeMap::new();
    if let Some(obj) = json.get("dependencies").and_then(|v| v.as_object()) {
        for (k, v) in obj {
            if let Some(s) = v.as_str() {
                dependencies.insert(k.clone(), s.to_string());
            }
        }
    }

    if name_returned != name {
        return Err(ResolverError::Json(format!(
            "registry returned name={} for requested {}", name_returned, name)));
    }
    if version_returned != version {
        return Err(ResolverError::Json(format!(
            "registry returned version={} for requested {}", version_returned, version)));
    }

    Ok(ResolvedDep {
        name: name.to_string(),
        version: version.to_string(),
        tarball_url: tarball.to_string(),
        integrity,
        shasum,
        dependencies,
    })
}

/// Fetch the `dist-tags.latest` version for `name` from the registry's
/// per-package root endpoint. Used by PM-EXT 13 reconnaissance to pin
/// each probed name to a concrete version without hardcoding versions
/// that drift. Not used by `pm_install` (which only accepts exact pins
/// in package.json).
pub fn fetch_latest_version(registry: &str, name: &str) -> Result<String, ResolverError> {
    let url = format!("{}/{}", registry.trim_end_matches('/'), name);
    let body = pm_http_get(&url)?;
    let json: serde_json::Value = serde_json::from_slice(&body)
        .map_err(|e| ResolverError::Json(format!("{e:?}")))?;
    let v = json.get("dist-tags").and_then(|d| d.get("latest")).and_then(|v| v.as_str())
        .ok_or(ResolverError::MissingField("dist-tags.latest"))?;
    Ok(v.to_string())
}

/// Walk the transitive-deps closure starting from `roots`, returning
/// the complete resolution set in BFS order. Dedup is by `(name,
/// version)` — if two paths in the graph need the same pinned version,
/// it appears once. **Conflicts** — two roots needing different
/// versions of the same name — are NOT handled in the first cut: the
/// later visit silently replaces the earlier in the BFS dedup. Doc 732
/// §VI carve-out: under exact-pin discipline, a conflict means the
/// dep graph is inconsistent and should be surfaced by the caller. The
/// closure walker keeps both ordering and dedup; conflict-detection is
/// PM-EXT N+1 work.
///
/// Any transitive dep specified with a semver range (caret, tilde,
/// `*`, etc.) causes the walker to error with `NonExactVersionSpec`.
/// That is the §VI carve-out behaving as designed — surfacing the
/// ecosystem-coverage boundary of the exact-pin first cut.
pub fn resolve_closure(
    registry: &str,
    roots: &[(String, String)],
) -> Result<Vec<ResolvedDep>, ResolverError> {
    let mut seen: BTreeSet<(String, String)> = BTreeSet::new();
    let mut queue: VecDeque<(String, String)> = VecDeque::new();
    let mut out = Vec::new();

    for (n, v) in roots {
        if seen.insert((n.clone(), v.clone())) {
            queue.push_back((n.clone(), v.clone()));
        }
    }

    while let Some((name, version)) = queue.pop_front() {
        let resolved = resolve_specifier(registry, &name, &version)?;
        for (dn, dv) in &resolved.dependencies {
            if seen.insert((dn.clone(), dv.clone())) {
                queue.push_back((dn.clone(), dv.clone()));
            }
        }
        out.push(resolved);
    }

    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_caret_range() {
        let r = resolve_specifier(DEFAULT_REGISTRY, "lodash", "^4.17.21");
        assert!(matches!(r, Err(ResolverError::NonExactVersionSpec(_))));
    }

    #[test]
    fn rejects_tilde_range() {
        let r = resolve_specifier(DEFAULT_REGISTRY, "lodash", "~4.17.21");
        assert!(matches!(r, Err(ResolverError::NonExactVersionSpec(_))));
    }

    #[test]
    #[ignore]
    fn closure_lodash_is_leaf() {
        let roots = vec![("lodash".to_string(), "4.17.21".to_string())];
        let closure = resolve_closure(DEFAULT_REGISTRY, &roots).expect("closure");
        assert_eq!(closure.len(), 1, "lodash 4.17.21 is zero-transitive");
        assert_eq!(closure[0].name, "lodash");
        assert!(closure[0].dependencies.is_empty());
    }

    #[test]
    #[ignore]
    fn closure_probe_small_transitive() {
        // Probe a small package with declared dependencies. We don't
        // know in advance whether all transitives are exact-pinned; the
        // expected outcome is one of: (a) success with closure.len() > 1,
        // demonstrating recursion, or (b) NonExactVersionSpec, surfacing
        // the §VI exact-pin boundary on real ecosystem packages. Either
        // is informative; this test prints and asserts only that the
        // closure walker terminates without panicking.
        let roots = vec![("debug".to_string(), "4.3.4".to_string())];
        let result = resolve_closure(DEFAULT_REGISTRY, &roots);
        match result {
            Ok(closure) => {
                eprintln!("debug@4.3.4 closure: {} packages", closure.len());
                for r in &closure { eprintln!("  {}@{}", r.name, r.version); }
                assert!(closure.iter().any(|r| r.name == "debug"));
            }
            Err(ResolverError::NonExactVersionSpec(v)) => {
                eprintln!("debug@4.3.4 transitive surfaced range: {v}");
            }
            Err(e) => panic!("unexpected error: {e:?}"),
        }
    }

    /// Network-dependent. Run via:
    ///   cargo test -p rusty-js-pm --release --lib resolver:: -- --ignored
    #[test]
    #[ignore]
    fn resolve_lodash_4_17_21() {
        let r = resolve_specifier(DEFAULT_REGISTRY, "lodash", "4.17.21")
            .expect("lodash 4.17.21 should resolve via npmmirror.com");
        assert_eq!(r.name, "lodash");
        assert_eq!(r.version, "4.17.21");
        assert!(r.tarball_url.ends_with("lodash-4.17.21.tgz"),
            "unexpected tarball URL: {}", r.tarball_url);
        // npmmirror provides integrity OR shasum (typically both for modern packages).
        assert!(r.integrity.is_some() || r.shasum.is_some(),
            "neither integrity nor shasum present");
    }
}
