//! TCC-EXT 2 measurement harness.
//!
//! Walks `pilots/ts-consumer-corpus/fixtures/**/*.ts`, runs each file
//! through `ts_resolve::parse_and_erase`, categorizes the outcome,
//! and writes:
//!   - `results/<date>/results.jsonl` — per-file record
//!   - `results/<date>/failure-table.md` — failure-kind frequency
//!     table ranked by file count
//!   - `results/<date>/summary.md` — overall headline numbers
//!
//! Usage (from repo root):
//!   cargo run --release -p ts-consumer-corpus --bin tcc-measure
//!
//! No arguments. Outputs land under `pilots/ts-consumer-corpus/results/<today>/`.

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::Instant;
use walkdir::WalkDir;

#[derive(Debug, Clone, PartialEq)]
enum Outcome {
    Ok,
    /// strip step itself errored (lex error during pre-scan)
    StripError(String),
    /// parse of stripped output failed — TSR emitted invalid JS
    ParseError(String),
    /// Rust panic — caught via catch_unwind
    Panic(String),
}

fn outcome_kind(o: &Outcome) -> &'static str {
    match o {
        Outcome::Ok => "OK",
        Outcome::StripError(_) => "STRIP",
        Outcome::ParseError(_) => "PARSE",
        Outcome::Panic(_) => "PANIC",
    }
}

/// Extract a coarse "structural cause" tag from a failure message so
/// the failure-table can rank by structural concept rather than by
/// unique error text. Heuristic — pattern-matches against common
/// substrings in rusty-js-parser error messages.
/// Inspect the file's source text near the error span to identify the
/// structural cause. Pattern-matches against constructs known to break
/// TSR's strip-and-parse pipeline; categorizes into a fixed taxonomy
/// so the failure-table groups by feature, not by error text.
fn structural_tag(msg: &str, src: &str) -> String {
    // Extract a span hint from `Span { start: N }` if present.
    let near: String = if let Some(idx) = msg.find("Span { start: ") {
        let rest = &msg[idx + 14..];
        let n: usize = rest
            .chars()
            .take_while(|c| c.is_ascii_digit())
            .collect::<String>()
            .parse()
            .unwrap_or(0);
        let lo = n.saturating_sub(80).min(src.len());
        let hi = (n + 80).min(src.len());
        src.get(lo..hi).unwrap_or("").to_string()
    } else {
        String::new()
    };

    // Patterns most likely to dominate the corpus, inspected at the
    // SOURCE near the error rather than in the error message itself
    // (which is downstream of TSR's strip and may be misleading).
    if near.contains("import type") || near.contains("export type") {
        return "import-export-type".into();
    }
    if near.contains("enum ") {
        return "enum".into();
    }
    if near.contains("@")
        && (near.contains("class") || near.contains("@inject") || near.contains("@override"))
    {
        return "decorator".into();
    }
    if near.contains("namespace ") || near.contains("declare module") {
        return "namespace-or-module".into();
    }
    if near.contains("abstract class") || near.contains("abstract ") {
        return "abstract-modifier".into();
    }
    if near.contains("readonly ") {
        return "readonly-modifier".into();
    }
    if near.contains("public ") || near.contains("private ") || near.contains("protected ") {
        return "access-modifier".into();
    }
    if near.contains("override ") {
        return "override-modifier".into();
    }
    if near.contains("satisfies ") {
        return "satisfies-op".into();
    }
    if near.contains("as const") {
        return "as-const".into();
    }
    if near.contains(" infer ") || near.contains("<infer ") {
        return "infer-type".into();
    }
    if near.contains("keyof ") {
        return "keyof-type".into();
    }
    // Class method return-type annotation: pattern `): TYPE {` or
    // `): TYPE =>`. Common in class bodies.
    if near.contains("):") && (near.contains("{") || near.contains("=>")) {
        return "method-return-annotation".into();
    }
    // Generic call expression `f<T>(...)`: a `<` preceded by Ident,
    // followed eventually by `>(`. Hard to detect from message alone;
    // approximate via source pattern.
    if near.contains("<") && near.contains(">(") {
        return "generic-call".into();
    }
    // Template-literal-type `${...}` in type position.
    if near.contains("`") && near.contains("${") {
        return "template-literal-type".into();
    }
    // Tuple/labeled-tuple `[name: T, ...]`
    if near.contains("[") && near.contains(":") && near.contains("...") {
        return "labeled-tuple".into();
    }
    // Fallback: by error-message kind.
    let m = msg.to_lowercase();
    if m.contains("expected colon") {
        return "uncategorized-expected-colon".into();
    }
    if m.contains("expected rparen") {
        return "uncategorized-expected-rparen".into();
    }
    if m.contains("expected rbrace") {
        return "uncategorized-expected-rbrace".into();
    }
    if m.contains("expected class member") {
        return "uncategorized-class-member".into();
    }
    if m.contains("unexpected token") {
        return "uncategorized-unexpected-token".into();
    }
    if m.contains("unterminated") {
        return "lex-unterminated".into();
    }
    if m.contains("invalididentifier") {
        return "lex-invalid-identifier".into();
    }
    let snippet: String = msg.chars().take(40).collect();
    format!("other: {}", snippet)
}

fn measure_file(path: &Path) -> Outcome {
    let src = match fs::read_to_string(path) {
        Ok(s) => s,
        Err(e) => return Outcome::StripError(format!("read: {}", e)),
    };
    let res = std::panic::catch_unwind(|| ts_resolve::parse_and_erase(&src));
    match res {
        Ok(Ok(_)) => Outcome::Ok,
        Ok(Err(e)) => {
            let m = e.message.clone();
            if m.starts_with("strip:") {
                Outcome::StripError(m)
            } else {
                Outcome::ParseError(m)
            }
        }
        Err(p) => {
            let s = if let Some(s) = p.downcast_ref::<&str>() {
                (*s).to_string()
            } else if let Some(s) = p.downcast_ref::<String>() {
                s.clone()
            } else {
                "panic (non-string payload)".to_string()
            };
            Outcome::Panic(s)
        }
    }
}

fn main() {
    let pilot_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_path_buf();
    let fixtures = pilot_dir.join("fixtures");
    if !fixtures.exists() {
        eprintln!("tcc-measure: fixtures dir missing: {}", fixtures.display());
        eprintln!("run: pilots/ts-consumer-corpus/scripts/install.sh");
        std::process::exit(1);
    }

    let date = chrono_today();
    let out_dir = pilot_dir.join("results").join(&date);
    fs::create_dir_all(&out_dir).expect("create results dir");

    let mut files: Vec<PathBuf> = WalkDir::new(&fixtures)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter(|e| e.path().extension().and_then(|s| s.to_str()) == Some("ts"))
        .filter(|e| !e.path().to_string_lossy().ends_with(".d.ts"))
        .map(|e| e.path().to_path_buf())
        .collect();
    files.sort();

    let t0 = Instant::now();
    let mut results: Vec<(PathBuf, Outcome)> = Vec::with_capacity(files.len());
    for f in &files {
        let o = measure_file(f);
        results.push((f.clone(), o));
    }
    let elapsed_ms = t0.elapsed().as_millis();

    // Per-file JSONL.
    let mut jsonl = String::new();
    for (f, o) in &results {
        let rel = f.strip_prefix(&fixtures).unwrap_or(f).display();
        let kind = outcome_kind(o);
        let msg = match o {
            Outcome::Ok => "".into(),
            Outcome::StripError(m) | Outcome::ParseError(m) | Outcome::Panic(m) => {
                m.replace('\n', " ").replace('"', "'")
            }
        };
        jsonl.push_str(&format!(
            "{{\"file\": \"{}\", \"outcome\": \"{}\", \"message\": \"{}\"}}\n",
            rel, kind, msg,
        ));
    }
    fs::write(out_dir.join("results.jsonl"), &jsonl).expect("write jsonl");

    // Aggregate counts.
    let total = results.len();
    let ok = results
        .iter()
        .filter(|(_, o)| matches!(o, Outcome::Ok))
        .count();
    let strip = results
        .iter()
        .filter(|(_, o)| matches!(o, Outcome::StripError(_)))
        .count();
    let parse = results
        .iter()
        .filter(|(_, o)| matches!(o, Outcome::ParseError(_)))
        .count();
    let panic = results
        .iter()
        .filter(|(_, o)| matches!(o, Outcome::Panic(_)))
        .count();
    let success_rate = if total > 0 {
        100.0 * ok as f64 / total as f64
    } else {
        0.0
    };

    // Structural-cause frequency for failures.
    let mut tag_counts: HashMap<String, (usize, String)> = HashMap::new();
    let mut tag_examples: HashMap<String, PathBuf> = HashMap::new();
    for (f, o) in &results {
        let msg = match o {
            Outcome::Ok => continue,
            Outcome::StripError(m) | Outcome::ParseError(m) | Outcome::Panic(m) => m,
        };
        let src = fs::read_to_string(f).unwrap_or_default();
        let tag = structural_tag(msg, &src);
        let entry = tag_counts.entry(tag.clone()).or_insert((0, msg.clone()));
        entry.0 += 1;
        tag_examples.entry(tag).or_insert(f.clone());
    }
    let mut ranked: Vec<(&String, &(usize, String))> = tag_counts.iter().collect();
    ranked.sort_by(|a, b| b.1 .0.cmp(&a.1 .0));

    // Summary.
    let summary = format!(
        "# TCC measurement — {}\n\n\
        - files measured: **{}**\n\
        - OK: **{}** ({:.1}% parse-success)\n\
        - STRIP errors: {}\n\
        - PARSE errors: {}\n\
        - PANICs: {}\n\
        - elapsed: {} ms ({:.2} ms/file)\n\n\
        Per-file results: `results.jsonl`. Failure-frequency table: `failure-table.md`.\n",
        date,
        total,
        ok,
        success_rate,
        strip,
        parse,
        panic,
        elapsed_ms,
        if total > 0 {
            elapsed_ms as f64 / total as f64
        } else {
            0.0
        },
    );
    fs::write(out_dir.join("summary.md"), &summary).expect("write summary");

    // Failure table.
    let mut table = String::new();
    table.push_str(&format!("# TCC failure-frequency table — {}\n\n", date));
    table.push_str(&format!(
        "Total files measured: {}. OK: {} ({:.1}% parse-success).\n\n",
        total, ok, success_rate
    ));
    table.push_str("Rows ranked by file count. Each row's `structural tag` names a TS feature / shape concept; an example file is given so the failure can be inspected. Sub-locale priority order is set by this table.\n\n");
    table.push_str("| Rank | Structural tag | Files | Example | Sample message |\n");
    table.push_str("|---:|---|---:|---|---|\n");
    for (i, (tag, (count, msg))) in ranked.iter().enumerate().take(30) {
        let example = tag_examples
            .get(*tag)
            .and_then(|p| p.strip_prefix(&fixtures).ok())
            .map(|p| p.display().to_string())
            .unwrap_or_default();
        let msg_short: String = msg.chars().take(80).collect();
        table.push_str(&format!(
            "| {} | `{}` | {} | `{}` | `{}` |\n",
            i + 1,
            tag,
            count,
            example,
            msg_short.replace('|', "\\|"),
        ));
    }
    fs::write(out_dir.join("failure-table.md"), &table).expect("write table");

    // Stdout summary.
    println!("{}", summary);
    println!("wrote: {}", out_dir.join("results.jsonl").display());
    println!("wrote: {}", out_dir.join("summary.md").display());
    println!("wrote: {}", out_dir.join("failure-table.md").display());
}

fn chrono_today() -> String {
    // Avoid the chrono crate dependency — emit YYYY-MM-DD from the
    // system clock via SystemTime + a small manual computation.
    use std::time::{SystemTime, UNIX_EPOCH};
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let days = secs / 86400;
    // 1970-01-01 was a Thursday; Zeller-ish conversion. Simpler: use
    // the "days since epoch -> Y/M/D" loop. Good enough for filenames.
    let (mut y, mut m, mut d) = (1970i64, 1i64, 1i64);
    let mut remaining = days as i64;
    while remaining > 0 {
        let is_leap = (y % 4 == 0 && y % 100 != 0) || (y % 400 == 0);
        let year_days = if is_leap { 366 } else { 365 };
        if remaining >= year_days {
            remaining -= year_days;
            y += 1;
        } else {
            break;
        }
    }
    let month_days = |yy: i64, mm: i64| -> i64 {
        match mm {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            2 => {
                if (yy % 4 == 0 && yy % 100 != 0) || (yy % 400 == 0) {
                    29
                } else {
                    28
                }
            }
            _ => 30,
        }
    };
    while remaining >= month_days(y, m) {
        remaining -= month_days(y, m);
        m += 1;
        if m > 12 {
            m = 1;
            y += 1;
        }
    }
    d += remaining;
    format!("{:04}-{:02}-{:02}", y, m, d)
}
