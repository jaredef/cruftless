//! PM-EXT 13 reconnaissance: classify a sample of popular npm packages
//! by their exact-pin-reachability under the closure walker.
//!
//! For each candidate name:
//!   1. Fetch dist-tags.latest from the registry root endpoint.
//!   2. Walk `resolve_closure(name, latest)`.
//!   3. Classify the outcome:
//!      - `Reachable(n)`: full closure resolves; n total packages
//!      - `RangeAt(transitive, range)`: a range was hit (NonExactVersionSpec)
//!      - `HttpError(...)`: registry returned non-2xx (e.g., 403 on
//!        scoped/private)
//!      - `Other(...)`: anything else
//!
//! **No tarballs are downloaded.** This round only fetches per-package
//! and per-version JSON manifests (each <100 KB) — no significant disk
//! footprint. Any scratch artifacts land on the mounted T7 drive per
//! the engagement constraint.
//!
//! Run with `nice` to avoid hammering the Pi:
//!
//!     nice -n 19 cargo run --release -p rusty-js-pm --example npm_coverage \
//!         > /media/jaredef/T7/cruftless-pm-recon/raw.txt
//!
//! Output: stdout = per-package classification + summary table; the
//! committed markdown report at
//! `pilots/rusty-js-pm/docs/npm-coverage-recon.md` is hand-curated
//! from the raw output.

use rusty_js_pm::resolver::{
    fetch_latest_version, resolve_closure, ResolverError, DEFAULT_REGISTRY,
};

/// Curated candidate list. Mix of leaf utilities, framework cores,
/// and tools known to historically exact-pin or range-pin
/// transitives. Top-of-mind names from the npm popularity rankings.
const CANDIDATES: &[&str] = &[
    "lodash",
    "debug",
    "ms",
    "chalk",
    "uuid",
    "axios",
    "express",
    "commander",
    "minimist",
    "semver",
    "yargs",
    "glob",
    "mkdirp",
    "rimraf",
    "fs-extra",
    "dotenv",
    "classnames",
    "prop-types",
    "ansi-styles",
    "is-number",
    "color-name",
    "ms", // dup to test dedup behavior; should resolve to same
    "tslib",
    "yallist",
    "lru-cache",
];

#[derive(Debug)]
enum Outcome {
    Reachable { count: usize },
    RangeAt { spec: String },
    NotFound,
    HttpError(String),
    Other(String),
}

fn classify(name: &str) -> Outcome {
    let version = match fetch_latest_version(DEFAULT_REGISTRY, name) {
        Ok(v) => v,
        Err(ResolverError::Http(rusty_js_pm::http::HttpError::Status { code, body_prefix })) => {
            if code == 404 {
                return Outcome::NotFound;
            }
            return Outcome::HttpError(format!("status {code}: {body_prefix}"));
        }
        Err(e) => return Outcome::Other(format!("latest: {e:?}")),
    };

    match resolve_closure(DEFAULT_REGISTRY, &[(name.to_string(), version.clone())]) {
        Ok(closure) => Outcome::Reachable {
            count: closure.len(),
        },
        Err(ResolverError::NonExactVersionSpec(s)) => Outcome::RangeAt { spec: s },
        Err(e) => Outcome::Other(format!("closure: {e:?}")),
    }
}

fn main() {
    let mut seen = std::collections::BTreeSet::new();
    let mut tallies: std::collections::BTreeMap<&'static str, usize> = [
        ("Reachable", 0usize),
        ("RangeAt", 0),
        ("NotFound", 0),
        ("HttpError", 0),
        ("Other", 0),
    ]
    .into_iter()
    .collect();

    println!("# npm-coverage reconnaissance (PM-EXT 13)");
    println!("# registry = {DEFAULT_REGISTRY}");
    println!();
    println!("| package | outcome | detail |");
    println!("|---|---|---|");
    for &name in CANDIDATES {
        if !seen.insert(name) {
            continue;
        }
        let t0 = std::time::Instant::now();
        let out = classify(name);
        let dt = t0.elapsed();
        let (cat, detail) = match &out {
            Outcome::Reachable { count } => ("Reachable", format!("closure size {count}")),
            Outcome::RangeAt { spec } => ("RangeAt", format!("`{spec}`")),
            Outcome::NotFound => ("NotFound", String::new()),
            Outcome::HttpError(s) => ("HttpError", s.clone()),
            Outcome::Other(s) => ("Other", s.clone()),
        };
        *tallies.get_mut(cat).unwrap() += 1;
        println!("| {name} | {cat} | {detail} ({}ms) |", dt.as_millis());
    }

    println!();
    println!("## Summary");
    let total: usize = tallies.values().sum();
    for (cat, n) in &tallies {
        let pct = if total > 0 {
            100.0 * (*n as f64) / (total as f64)
        } else {
            0.0
        };
        println!("- **{cat}**: {n}/{total} ({pct:.0}%)");
    }
}
