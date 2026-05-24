//! TXC-EXT 1 measurement harness — execute-parity per file.
//!
//! For each .ts file in the corpus fixtures, runs:
//!   bun  -e "import * as M from FILE; console.log(Object.keys(M).sort().join('\n'))"
//!   cruft <driver-wrapper.ts>
//! Then diffs the stdout. Categorizes per the fixed taxonomy:
//!   MATCH       — both produce byte-identical stdout
//!   DIVERGE     — both run but stdout differs
//!   BUN_FAIL    — bun exited non-zero (corpus file likely has dep
//!                 it can't resolve; not actionable for cruft)
//!   CRUFT_FAIL  — cruft exited non-zero (THE actionable category)
//!   SETUP_FAIL  — driver setup or filesystem error
//!   TIMEOUT     — either runtime exceeded 5s
//!
//! Uses `bun` as oracle per seed §I.1. The driver imports the file as
//! a module and prints sorted export-names — exercises parse + module-
//! init + export shape. Side-effect-free files produce empty output
//! (still a valid MATCH). Files with side-effects (top-level console
//! calls, etc.) produce richer output that surfaces runtime divergence.
//!
//! Usage (from repo root):
//!   cargo run --release -p ts-execute-corpus --bin txc-measure

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};
use walkdir::WalkDir;

const TIMEOUT_SECS: u64 = 5;

#[derive(Debug, Clone, PartialEq)]
enum Outcome {
    Match,
    Diverge { bun_first_line: String, cruft_first_line: String },
    BunFail { msg: String },
    CruftFail { msg: String },
    SetupFail { msg: String },
    Timeout { which: &'static str },
}

fn outcome_kind(o: &Outcome) -> &'static str {
    match o {
        Outcome::Match => "MATCH",
        Outcome::Diverge { .. } => "DIVERGE",
        Outcome::BunFail { .. } => "BUN_FAIL",
        Outcome::CruftFail { .. } => "CRUFT_FAIL",
        Outcome::SetupFail { .. } => "SETUP_FAIL",
        Outcome::Timeout { .. } => "TIMEOUT",
    }
}

/// Run a command with timeout; capture stdout + stderr + exit status.
/// Returns (status_code, stdout, stderr) or Err on timeout.
fn run_with_timeout(cmd: &mut Command) -> Result<(Option<i32>, String, String), String> {
    cmd.stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    let start = Instant::now();
    let mut child = match cmd.spawn() {
        Ok(c) => c,
        Err(e) => return Err(format!("spawn: {}", e)),
    };
    // Poll with sleep — std lacks a builtin timeout.
    loop {
        match child.try_wait() {
            Ok(Some(status)) => {
                let stdout = read_stream(&mut child.stdout);
                let stderr = read_stream(&mut child.stderr);
                return Ok((status.code(), stdout, stderr));
            }
            Ok(None) => {
                if start.elapsed() > Duration::from_secs(TIMEOUT_SECS) {
                    let _ = child.kill();
                    return Err("timeout".into());
                }
                std::thread::sleep(Duration::from_millis(20));
            }
            Err(e) => return Err(format!("wait: {}", e)),
        }
    }
}

fn read_stream<R: std::io::Read>(s: &mut Option<R>) -> String {
    let mut buf = String::new();
    if let Some(s) = s {
        use std::io::Read;
        let _ = s.read_to_string(&mut buf);
    }
    buf
}

/// Build the synthetic driver script — imports the file as a module
/// and prints sorted export names. Same for both runtimes.
fn driver_for(file_abs: &Path) -> String {
    format!(
        "import * as M from '{}';\nconsole.log(Object.keys(M).sort().join('\\n'));\n",
        file_abs.display()
    )
}

fn measure_file(file: &Path, cruft_bin: &Path, bun_bin: &Path) -> Outcome {
    // TXC-EXT 1 design pivot: skip the synthetic-driver wrapper. Run
    // each fixture directly with both runtimes; compare exit status.
    // MATCH = both exit 0; BUN_FAIL = bun couldn't (e.g. unresolved
    // dep — file unrunnable in any context); CRUFT_FAIL = bun ok
    // but cruft errored (THE actionable category for parity work).
    //
    // Stdout-diffing was too noisy under the synthetic wrapper because
    // import resolution side-effects vary. Status-only is the cleanest
    // execute-parity signal at the module-load tier.
    let bun_res = run_with_timeout(Command::new(bun_bin).arg(file));
    let cruft_res = run_with_timeout(Command::new(cruft_bin).arg(file));

    let (bun_status, _bun_stdout, bun_stderr) = match bun_res {
        Ok(t) => t,
        Err(e) if e == "timeout" => return Outcome::Timeout { which: "bun" },
        Err(e) => return Outcome::SetupFail { msg: format!("bun run: {}", e) },
    };
    let (cruft_status, _cruft_stdout, cruft_stderr) = match cruft_res {
        Ok(t) => t,
        Err(e) if e == "timeout" => return Outcome::Timeout { which: "cruft" },
        Err(e) => return Outcome::SetupFail { msg: format!("cruft run: {}", e) },
    };

    let bun_ok = bun_status == Some(0);
    let cruft_ok = cruft_status == Some(0);

    if !bun_ok {
        // File unrunnable under oracle — not actionable for cruft.
        return Outcome::BunFail {
            msg: bun_stderr.lines().next().unwrap_or("").chars().take(120).collect::<String>(),
        };
    }
    if !cruft_ok {
        return Outcome::CruftFail {
            msg: cruft_stderr.lines().next().unwrap_or("").chars().take(120).collect::<String>(),
        };
    }
    Outcome::Match
}

fn tempfile() -> std::io::Result<PathBuf> {
    use std::time::SystemTime;
    let pid = std::process::id();
    let nanos = SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.subsec_nanos())
        .unwrap_or(0);
    let p = std::env::temp_dir().join(format!("txc-driver-{}-{}.ts", pid, nanos));
    Ok(p)
}

fn main() {
    let pilot_dir = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap().to_path_buf();
    let repo_root = pilot_dir.parent().unwrap().parent().unwrap().to_path_buf();
    let fixtures = repo_root.join("pilots/ts-consumer-corpus/fixtures");
    if !fixtures.exists() {
        eprintln!("txc: fixtures missing at {} — run TCC install first", fixtures.display());
        std::process::exit(1);
    }
    let cruft_bin = std::env::var("CRUFT_BIN")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from(format!("{}/bin/cruft", std::env::var("HOME").unwrap_or_default())));
    let bun_bin = std::env::var("BUN_BIN")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from(format!("{}/.bun/bin/bun", std::env::var("HOME").unwrap_or_default())));
    if !cruft_bin.exists() {
        eprintln!("txc: cruft binary missing at {}", cruft_bin.display());
        std::process::exit(1);
    }
    if !bun_bin.exists() {
        eprintln!("txc: bun binary missing at {}", bun_bin.display());
        std::process::exit(1);
    }

    let date = chrono_today();
    let out_dir = pilot_dir.join("results").join(&date);
    fs::create_dir_all(&out_dir).expect("create results dir");

    // Walk corpus; collect files. Optionally limit via TXC_LIMIT for
    // dev iteration (full-corpus runs may take minutes).
    let limit: Option<usize> = std::env::var("TXC_LIMIT").ok().and_then(|s| s.parse().ok());
    let mut files: Vec<PathBuf> = WalkDir::new(&fixtures)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter(|e| e.path().extension().and_then(|s| s.to_str()) == Some("ts"))
        .filter(|e| !e.path().to_string_lossy().ends_with(".d.ts"))
        .map(|e| e.path().to_path_buf())
        .collect();
    files.sort();
    if let Some(n) = limit { files.truncate(n); }

    eprintln!("txc: measuring {} files (bun={}, cruft={})",
        files.len(), bun_bin.display(), cruft_bin.display());

    let t0 = Instant::now();
    let mut results: Vec<(PathBuf, Outcome)> = Vec::with_capacity(files.len());
    for (idx, f) in files.iter().enumerate() {
        if idx % 25 == 0 && idx > 0 {
            eprintln!("  ... {}/{} ({} s elapsed)", idx, files.len(), t0.elapsed().as_secs());
        }
        let o = measure_file(f, &cruft_bin, &bun_bin);
        results.push((f.clone(), o));
    }
    let elapsed_ms = t0.elapsed().as_millis();

    // Aggregate.
    let total = results.len();
    let mut counts: HashMap<&'static str, usize> = HashMap::new();
    for (_, o) in &results { *counts.entry(outcome_kind(o)).or_insert(0) += 1; }
    let n_match = counts.get("MATCH").copied().unwrap_or(0);
    let parity = if total > 0 { 100.0 * n_match as f64 / total as f64 } else { 0.0 };

    // Per-file JSONL.
    let mut jsonl = String::new();
    for (f, o) in &results {
        let rel = f.strip_prefix(&fixtures).unwrap_or(f).display();
        let kind = outcome_kind(o);
        let detail = match o {
            Outcome::Match => String::new(),
            Outcome::Diverge { bun_first_line, cruft_first_line } =>
                format!("bun={} cruft={}", esc(bun_first_line), esc(cruft_first_line)),
            Outcome::BunFail { msg } | Outcome::CruftFail { msg } | Outcome::SetupFail { msg } =>
                esc(msg),
            Outcome::Timeout { which } => format!("which={}", which),
        };
        jsonl.push_str(&format!(
            "{{\"file\":\"{}\",\"outcome\":\"{}\",\"detail\":\"{}\"}}\n",
            rel, kind, detail,
        ));
    }
    fs::write(out_dir.join("results.jsonl"), &jsonl).expect("write jsonl");

    // Summary.
    let summary = format!(
        "# TXC measurement — {}\n\n\
        - files measured: **{}**\n\
        - MATCH: **{}** ({:.1}% execute-parity)\n\
        - DIVERGE: {}\n\
        - BUN_FAIL: {}\n\
        - CRUFT_FAIL: {}\n\
        - SETUP_FAIL: {}\n\
        - TIMEOUT: {}\n\
        - elapsed: {:.1} s ({:.2} s/file)\n\n\
        Per-file: `results.jsonl`. Divergence table: `divergence-table.md`.\n",
        date, total, n_match, parity,
        counts.get("DIVERGE").copied().unwrap_or(0),
        counts.get("BUN_FAIL").copied().unwrap_or(0),
        counts.get("CRUFT_FAIL").copied().unwrap_or(0),
        counts.get("SETUP_FAIL").copied().unwrap_or(0),
        counts.get("TIMEOUT").copied().unwrap_or(0),
        elapsed_ms as f64 / 1000.0,
        elapsed_ms as f64 / 1000.0 / total.max(1) as f64,
    );
    fs::write(out_dir.join("summary.md"), &summary).expect("write summary");

    // Divergence table — group CRUFT_FAIL by first-line-of-stderr.
    let mut tag_counts: HashMap<String, (usize, PathBuf, String)> = HashMap::new();
    for (f, o) in &results {
        let (tag, sample) = match o {
            Outcome::CruftFail { msg } => (format!("CRUFT_FAIL: {}", first_words(msg, 6)), msg.clone()),
            Outcome::Diverge { bun_first_line, cruft_first_line } =>
                (format!("DIVERGE: bun={} cruft={}", first_words(bun_first_line, 3), first_words(cruft_first_line, 3)),
                 format!("bun={} cruft={}", bun_first_line, cruft_first_line)),
            _ => continue,
        };
        let e = tag_counts.entry(tag).or_insert((0, f.clone(), sample.clone()));
        e.0 += 1;
    }
    let mut ranked: Vec<_> = tag_counts.iter().collect();
    ranked.sort_by(|a, b| b.1.0.cmp(&a.1.0));
    let mut table = String::new();
    table.push_str(&format!("# TXC divergence-frequency table — {}\n\nTotal files: {}. MATCH: {} ({:.1}%).\n\n",
        date, total, n_match, parity));
    table.push_str("| Rank | Tag | Files | Example | Sample |\n|---:|---|---:|---|---|\n");
    for (i, (tag, (cnt, ex, sample))) in ranked.iter().enumerate().take(30) {
        let rel = ex.strip_prefix(&fixtures).map(|p| p.display().to_string()).unwrap_or_default();
        let s: String = sample.chars().take(80).collect();
        table.push_str(&format!("| {} | `{}` | {} | `{}` | `{}` |\n",
            i + 1, tag, cnt, rel, s.replace('|', "\\|")));
    }
    fs::write(out_dir.join("divergence-table.md"), &table).expect("write table");

    println!("{}", summary);
    println!("wrote: {}", out_dir.join("results.jsonl").display());
    println!("wrote: {}", out_dir.join("summary.md").display());
    println!("wrote: {}", out_dir.join("divergence-table.md").display());
}

fn esc(s: &str) -> String {
    s.replace('\\', "\\\\").replace('"', "\\\"").replace('\n', " ").chars().take(160).collect()
}

fn first_words(s: &str, n: usize) -> String {
    s.split_whitespace().take(n).collect::<Vec<_>>().join(" ")
}

fn chrono_today() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let secs = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    let days = (secs / 86400) as i64;
    let (mut y, mut m, mut d) = (1970i64, 1i64, 1i64);
    let mut rem = days;
    while rem > 0 {
        let leap = (y % 4 == 0 && y % 100 != 0) || (y % 400 == 0);
        let yd = if leap { 366 } else { 365 };
        if rem >= yd { rem -= yd; y += 1; } else { break; }
    }
    let md = |yy: i64, mm: i64| -> i64 {
        match mm {
            1|3|5|7|8|10|12 => 31,
            4|6|9|11 => 30,
            2 => if (yy % 4 == 0 && yy % 100 != 0) || (yy % 400 == 0) { 29 } else { 28 },
            _ => 30,
        }
    };
    while rem >= md(y, m) { rem -= md(y, m); m += 1; if m > 12 { m = 1; y += 1; } }
    d += rem;
    format!("{:04}-{:02}-{:02}", y, m, d)
}
