//! T262C-EXT 1 categorization apparatus.
//!
//! Reads a `results.jsonl` produced by `scripts/test262-sample/run-sample.sh`
//! and emits a failure-frequency matrix indexed by two orthogonal axes:
//!
//!   - **Structure axis** (per Doc 720 static pipeline DAG): which
//!     pipeline does the failing test exercise? Derived from the test
//!     file path's prefix under `test262/test/...`.
//!
//!   - **Data axis** (per the data-axis missing-coordinate framing):
//!     what input value-shapes or feature-set does the test exercise?
//!     Derived from the test file's frontmatter `features:` list +
//!     test-source heuristics (typeof / negative / async / etc.).
//!
//! Output: `pilots/test262-categorize/results/<date>/{matrix.md,
//! categorized.jsonl, summary.md}`.
//!
//! Usage (from repo root):
//!   cargo run --release -p test262-categorize --bin t262c -- <results.jsonl>
//!
//! Argument: path to the results.jsonl produced by the test262-sample
//! runner. If absent, defaults to the most-recent
//! `results/test262-sample-<DATE>/results.jsonl`.

use std::collections::{BTreeMap, HashMap};
use std::fs;
use std::path::{Path, PathBuf};

fn main() {
    let repo_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent().unwrap().parent().unwrap().parent().unwrap().to_path_buf();
    let pilot_dir = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap().to_path_buf();
    let arg = std::env::args().nth(1);
    let results_path = match arg {
        Some(p) => PathBuf::from(p),
        None => find_latest_results(&repo_root).unwrap_or_else(|| {
            eprintln!("usage: t262c <results.jsonl> (or no arg to auto-find)");
            std::process::exit(1);
        }),
    };
    if !results_path.exists() {
        eprintln!("not found: {}", results_path.display());
        std::process::exit(1);
    }
    eprintln!("t262c: reading {}", results_path.display());

    let text = fs::read_to_string(&results_path).expect("read results.jsonl");
    let mut failures: Vec<(String, String)> = Vec::new(); // (path, reason)
    let mut total = 0usize;
    let mut pass = 0usize;
    let mut fail = 0usize;
    let mut skip = 0usize;
    for line in text.lines() {
        if line.trim().is_empty() { continue; }
        let v: serde_json::Value = match serde_json::from_str(line) {
            Ok(v) => v,
            Err(_) => continue,
        };
        total += 1;
        let status = v.get("status").and_then(|s| s.as_str()).unwrap_or("");
        let path = v.get("path").and_then(|s| s.as_str()).unwrap_or("").to_string();
        let reason = v.get("reason").and_then(|s| s.as_str()).unwrap_or("").to_string();
        match status {
            "PASS" => pass += 1,
            "FAIL" => { fail += 1; failures.push((path, reason)); }
            "SKIP" => skip += 1,
            _ => {}
        }
    }
    eprintln!("t262c: {} total / {} PASS / {} FAIL / {} SKIP",
        total, pass, fail, skip);
    let runnable = pass + fail;
    let pct = if runnable > 0 { 100.0 * pass as f64 / runnable as f64 } else { 0.0 };
    eprintln!("t262c: runnable pass-rate {:.1}% ({}/{})", pct, pass, runnable);

    // Categorize each failure.
    let mut cells: HashMap<(String, String), Vec<(String, String)>> = HashMap::new();
    let mut pipeline_freq: BTreeMap<String, usize> = BTreeMap::new();
    let mut data_freq: BTreeMap<String, usize> = BTreeMap::new();

    for (path, reason) in &failures {
        let pipeline = structure_axis_pipeline(path);
        let data = data_axis_shape(path, reason);
        *pipeline_freq.entry(pipeline.clone()).or_insert(0) += 1;
        *data_freq.entry(data.clone()).or_insert(0) += 1;
        cells.entry((pipeline, data)).or_insert_with(Vec::new)
            .push((path.clone(), reason.clone()));
    }

    // Rank cells.
    let mut ranked_cells: Vec<((String, String), usize)> = cells.iter()
        .map(|(k, v)| (k.clone(), v.len()))
        .collect();
    ranked_cells.sort_by(|a, b| b.1.cmp(&a.1));

    // Rank pipelines (structure-axis marginal).
    let mut ranked_pipelines: Vec<(String, usize)> = pipeline_freq.iter()
        .map(|(k, &v)| (k.clone(), v)).collect();
    ranked_pipelines.sort_by(|a, b| b.1.cmp(&a.1));

    // Rank data-axis (data marginal).
    let mut ranked_data: Vec<(String, usize)> = data_freq.iter()
        .map(|(k, &v)| (k.clone(), v)).collect();
    ranked_data.sort_by(|a, b| b.1.cmp(&a.1));

    // Output.
    let date = chrono_today();
    let out_dir = pilot_dir.join("results").join(&date);
    fs::create_dir_all(&out_dir).expect("mkdir");

    // summary.md
    let top_15_files: usize = ranked_cells.iter().take(15).map(|(_, c)| *c).sum();
    let top_15_pct = if fail > 0 { 100.0 * top_15_files as f64 / fail as f64 } else { 0.0 };
    let mut summary = String::new();
    summary.push_str(&format!("# T262C summary — {}\n\n", date));
    summary.push_str(&format!("Source: `{}`\n\n", results_path.display()));
    summary.push_str(&format!("- total tests measured: {}\n", total));
    summary.push_str(&format!("- PASS: **{}**\n", pass));
    summary.push_str(&format!("- FAIL: **{}**\n", fail));
    summary.push_str(&format!("- SKIP: {}\n", skip));
    summary.push_str(&format!("- runnable pass-rate: **{:.1}%** ({}/{})\n\n", pct, pass, runnable));
    summary.push_str(&format!("- distinct (pipeline × data) cells: {}\n", ranked_cells.len()));
    summary.push_str(&format!("- top-15 cells account for **{:.1}%** of failures ({}/{})\n", top_15_pct, top_15_files, fail));
    summary.push_str("\n");
    summary.push_str("See `matrix.md` for the ranked cell distribution + structure-axis and data-axis marginals.\n");
    fs::write(out_dir.join("summary.md"), &summary).expect("write summary");

    // matrix.md
    let mut matrix = String::new();
    matrix.push_str(&format!("# T262C failure matrix — {}\n\n", date));
    matrix.push_str(&format!("Source: `{}`. Total FAIL: **{}** ({:.1}% of runnable {}).\n\n",
        results_path.display(), fail, 100.0 - pct, runnable));

    matrix.push_str("## Ranked (pipeline × data-shape) cells\n\n");
    matrix.push_str("| Rank | Pipeline (structure-axis) | Data-shape (data-axis) | Count | Example test |\n");
    matrix.push_str("|---:|---|---|---:|---|\n");
    for (i, ((pipeline, data), count)) in ranked_cells.iter().enumerate().take(40) {
        let example = cells.get(&(pipeline.clone(), data.clone()))
            .and_then(|v| v.first())
            .map(|(p, _)| p.clone())
            .unwrap_or_default();
        let example_rel = example.strip_prefix("/home/jaredef/test262/test/")
            .unwrap_or(&example).to_string();
        matrix.push_str(&format!("| {} | `{}` | `{}` | {} | `{}` |\n",
            i + 1, pipeline, data, count, example_rel));
    }

    matrix.push_str("\n## Structure-axis marginal (per-pipeline failure counts)\n\n");
    matrix.push_str("| Pipeline | Failures |\n|---|---:|\n");
    for (p, c) in ranked_pipelines.iter().take(30) {
        matrix.push_str(&format!("| `{}` | {} |\n", p, c));
    }

    matrix.push_str("\n## Data-axis marginal (per-feature failure counts)\n\n");
    matrix.push_str("| Data-shape | Failures |\n|---|---:|\n");
    for (d, c) in ranked_data.iter().take(40) {
        matrix.push_str(&format!("| `{}` | {} |\n", d, c));
    }

    fs::write(out_dir.join("matrix.md"), &matrix).expect("write matrix");

    // categorized.jsonl — full per-failure record
    let mut jsonl = String::new();
    for (path, reason) in &failures {
        let pipeline = structure_axis_pipeline(path);
        let data = data_axis_shape(path, reason);
        let path_rel = path.strip_prefix("/home/jaredef/test262/test/")
            .unwrap_or(path);
        let r = reason.replace('"', "'").replace('\n', " ");
        jsonl.push_str(&format!(
            "{{\"file\":\"{}\",\"pipeline\":\"{}\",\"data\":\"{}\",\"reason\":\"{}\"}}\n",
            path_rel, pipeline, data, &r.chars().take(200).collect::<String>(),
        ));
    }
    fs::write(out_dir.join("categorized.jsonl"), &jsonl).expect("write categorized");

    println!("{}", summary);
    println!("wrote: {}", out_dir.join("summary.md").display());
    println!("wrote: {}", out_dir.join("matrix.md").display());
    println!("wrote: {}", out_dir.join("categorized.jsonl").display());
}

/// Structure-axis: derive the pipeline from the test path.
/// `test262/test/built-ins/JSON/parse/...` → `JSON.parse`
/// `test262/test/built-ins/Array/prototype/sort/...` → `Array.prototype.sort`
/// `test262/test/language/expressions/arrow-function/...` → `language.arrow-function`
/// Etc.
fn structure_axis_pipeline(path: &str) -> String {
    let stripped = path.strip_prefix("/home/jaredef/test262/test/")
        .unwrap_or(path);
    let parts: Vec<&str> = stripped.split('/').collect();
    match parts.as_slice() {
        ["built-ins", obj, "prototype", method, ..] =>
            format!("{}.prototype.{}", obj, method),
        ["built-ins", obj, method, ..] if !method.ends_with(".js") =>
            format!("{}.{}", obj, method),
        ["built-ins", obj, ..] =>
            format!("{}", obj),
        ["language", "expressions", construct, ..] =>
            format!("language.expressions.{}", construct),
        ["language", "statements", construct, ..] =>
            format!("language.statements.{}", construct),
        ["language", construct, ..] =>
            format!("language.{}", construct),
        ["harness", ..] => "harness".to_string(),
        ["intl402", ..] => "intl402".to_string(),
        ["staging", ..] => "staging".to_string(),
        ["annexB", ..] => "annexB".to_string(),
        _ => format!("(other) {}", parts.first().unwrap_or(&"unknown")),
    }
}

/// Data-axis: derive the input value-shape / feature-set from the test
/// file's frontmatter + the failure reason. Heuristic at v1; opens up
/// to richer per-test parsing as the apparatus matures.
fn data_axis_shape(path: &str, reason: &str) -> String {
    // Read the test file's frontmatter (if accessible).
    let src = fs::read_to_string(path).ok();
    let mut tags: Vec<String> = Vec::new();

    if let Some(s) = &src {
        // Extract features: list from frontmatter.
        if let Some(start) = s.find("/*---") {
            if let Some(end) = s[start..].find("---*/") {
                let frontmatter = &s[start..start + end];
                if let Some(idx) = frontmatter.find("features:") {
                    let rest = &frontmatter[idx + 9..];
                    if let Some(nl) = rest.find('\n') {
                        let features_line = rest[..nl].trim()
                            .trim_start_matches('[').trim_end_matches(']');
                        for f in features_line.split(',') {
                            let f = f.trim().trim_matches('"').trim_matches('\'').to_string();
                            if !f.is_empty() { tags.push(format!("feat:{}", f)); }
                        }
                    }
                }
                if frontmatter.contains("flags: [async]") {
                    tags.push("flag:async".into());
                }
                if frontmatter.contains("flags: [module]") {
                    tags.push("flag:module".into());
                }
                if frontmatter.contains("negative:") {
                    if let Some(idx) = frontmatter.find("type:") {
                        let rest = &frontmatter[idx + 5..];
                        if let Some(nl) = rest.find('\n') {
                            let t = rest[..nl].trim().to_string();
                            tags.push(format!("negative:{}", t));
                        }
                    } else {
                        tags.push("negative".into());
                    }
                }
            }
        }
    }

    // Heuristic from reason text — what error class did cruft surface?
    let r = reason.to_lowercase();
    if r.contains("typeerror") { tags.push("err:TypeError".into()); }
    else if r.contains("rangeerror") { tags.push("err:RangeError".into()); }
    else if r.contains("syntaxerror") { tags.push("err:SyntaxError".into()); }
    else if r.contains("referenceerror") { tags.push("err:ReferenceError".into()); }
    else if r.contains("uri") { tags.push("err:URIError".into()); }
    else if r.contains("test262error") { tags.push("err:Test262Error".into()); }
    if r.contains("expected") && r.contains("thrown") { tags.push("expected-throw-missing".into()); }
    if r.contains("not callable") { tags.push("not-callable".into()); }
    if r.contains("compileerror") { tags.push("err:CompileError".into()); }
    if r.contains("module not found") { tags.push("module-not-found".into()); }
    if r.contains("not yet implemented") || r.contains("unimplemented") {
        tags.push("not-implemented".into());
    }

    if tags.is_empty() {
        "(no-feature-tag)".to_string()
    } else {
        tags.join(";")
    }
}

fn find_latest_results(repo_root: &Path) -> Option<PathBuf> {
    let results_dir = repo_root.join("results");
    let mut latest: Option<PathBuf> = None;
    if let Ok(entries) = fs::read_dir(&results_dir) {
        for entry in entries.filter_map(|e| e.ok()) {
            let p = entry.path();
            let name = p.file_name().and_then(|n| n.to_str()).unwrap_or("");
            if name.starts_with("test262-sample-") && !name.ends_with("-bun") {
                let candidate = p.join("results.jsonl");
                if candidate.exists() {
                    match &latest {
                        None => latest = Some(candidate),
                        Some(prior) => {
                            if candidate > *prior { latest = Some(candidate); }
                        }
                    }
                }
            }
        }
    }
    latest
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
