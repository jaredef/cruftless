//! CAPS-EXT 5: synthetic-adversary probe harness — the §XVI oracle.
//!
//! Each probe is a `.mjs` file under `pilots/rusty-js-caps/probes/`
//! that attempts one attack class from Doc 736 §IV. The harness runs
//! each probe under cruftless and asserts the expected outcome per
//! mode.
//!
//! **At CAPS-EXT 5 close**, route-through has not yet landed. Every
//! probe SHOULD WIN under both Mode 0 (default) and Mode 3 (`--sealed`)
//! because no effectful method consults the dispatcher yet. This is
//! the documented pre-state baseline.
//!
//! **At CAPS-EXT 6+**, as each surface gets routed through the
//! dispatcher, the corresponding probe(s) flip to LOSE under `--sealed`.
//! The harness gains per-probe Mode-3 assertions per round.
//!
//! **CAPS-EXT 13 closure**: every probe LOSES under `--sealed`. The
//! Doc 736 §IV impossibility claim is mechanically realized.

use std::process::Command;

fn bin() -> &'static str { env!("CARGO_BIN_EXE_cruftless") }

fn probes_dir() -> std::path::PathBuf {
    let manifest = env!("CARGO_MANIFEST_DIR");
    std::path::Path::new(manifest)
        .parent().expect("workspace root")
        .join("pilots/rusty-js-caps/probes")
}

/// Run a probe under the given mode flag (or none for Mode 0), return
/// (exit_code, stdout, stderr).
fn run_probe(name: &str, mode_flag: Option<&str>) -> (i32, String, String) {
    let path = probes_dir().join(format!("{name}.mjs"));
    let mut cmd = Command::new(bin());
    if let Some(flag) = mode_flag { cmd.arg(flag); }
    cmd.arg(&path);
    let out = cmd.output().unwrap_or_else(|e| panic!("run {name}: {e}"));
    let code = out.status.code().unwrap_or(-1);
    (code,
     String::from_utf8_lossy(&out.stdout).to_string(),
     String::from_utf8_lossy(&out.stderr).to_string())
}

/// Inspect probe output for the WINS or LOSES sentinel line.
///
/// WINS lives in stdout (so it is gated by the stdio capability at
/// CAPS-EXT 10+; a `WINS` in stdout under `--sealed` means stdout was
/// reachable).
///
/// LOSES lives in stderr (probes emit it via `console.error` so the
/// stdio gate does not eat the loss signal under `--sealed`).
#[derive(Debug, PartialEq, Eq)]
enum ProbeOutcome { Wins, Loses, Indeterminate }

fn classify_streams(stdout: &str, stderr: &str) -> ProbeOutcome {
    if stdout.lines().any(|l| l.contains("PROBE:WINS:")) { ProbeOutcome::Wins }
    else if stderr.lines().any(|l| l.contains("PROBE:LOSES:"))
         || stdout.lines().any(|l| l.contains("PROBE:LOSES:")) { ProbeOutcome::Loses }
    else { ProbeOutcome::Indeterminate }
}

/// Back-compat shim for the older `classify(&stdout)` call shape.
fn classify(stdout: &str) -> ProbeOutcome {
    classify_streams(stdout, "")
}

// --- Mode-0 baseline: every probe WINS (pre-route-through pre-state) ---

#[test]
fn baseline_fs_read_wins() {
    let (_, stdout, _) = run_probe("fs_read", None);
    assert_eq!(classify(&stdout), ProbeOutcome::Wins, "stdout: {stdout}");
}

#[test]
fn baseline_fs_write_wins() {
    let (_, stdout, _) = run_probe("fs_write", None);
    assert_eq!(classify(&stdout), ProbeOutcome::Wins, "stdout: {stdout}");
    // Cleanup the marker file the probe wrote.
    let _ = std::fs::remove_file("/tmp/cruftless-probe-fs-write.marker");
}

#[test]
fn baseline_fs_list_wins() {
    let (_, stdout, _) = run_probe("fs_list", None);
    assert_eq!(classify(&stdout), ProbeOutcome::Wins, "stdout: {stdout}");
}

#[test]
fn baseline_fs_stat_wins() {
    let (_, stdout, _) = run_probe("fs_stat", None);
    assert_eq!(classify(&stdout), ProbeOutcome::Wins, "stdout: {stdout}");
}

#[test]
fn baseline_process_exit_wins() {
    // process_exit has a special outcome shape: a successful exit(42)
    // means the runtime honored the call and terminated with code 42
    // before any LOSES branch ran. We detect WIN by the exit code.
    let (code, stdout, stderr) = run_probe("process_exit", None);
    assert_eq!(code, 42,
        "process_exit baseline should exit with code 42 (attacker controls host exit); got code {code}, stdout: {stdout}, stderr: {stderr}");
    // The LOSES branch should NOT have fired since the exit happened.
    assert!(!stdout.contains("PROBE:LOSES:") && !stderr.contains("PROBE:LOSES:"),
        "process_exit should succeed under Mode 0; got LOSES: stdout={stdout} stderr={stderr}");
}

#[test]
fn baseline_env_read_wins() {
    let (_, stdout, _) = run_probe("env_read", None);
    assert_eq!(classify(&stdout), ProbeOutcome::Wins, "stdout: {stdout}");
}

#[test]
fn baseline_clock_read_wins() {
    let (_, stdout, _) = run_probe("clock_read", None);
    assert_eq!(classify(&stdout), ProbeOutcome::Wins, "stdout: {stdout}");
}

#[test]
fn baseline_cwd_read_wins() {
    let (_, stdout, _) = run_probe("cwd_read", None);
    assert_eq!(classify(&stdout), ProbeOutcome::Wins, "stdout: {stdout}");
}

// --- Mode-3 pre-route-through state: probes STILL WIN under --sealed ---
//
// At CAPS-EXT 5, no effectful method consults the dispatcher, so
// --sealed has no enforcement effect yet. These tests document that
// state and will flip to "probe loses under --sealed" as each surface
// gets routed at CAPS-EXT 6+.

// CAPS-EXT 6: fs read methods now route through dispatcher.
// Under --sealed, every fs read probe must LOSE (refused with CapabilityError).

#[test]
fn fs_read_loses_under_sealed() {
    let (_, stdout, stderr) = run_probe("fs_read", Some("--sealed"));
    assert_eq!(classify_streams(&stdout, &stderr), ProbeOutcome::Loses,
        "CAPS-EXT 6: --sealed must block fs_read; stdout: {stdout}\nstderr: {stderr}");
    assert!(stderr.contains("fs"),
        "loss message should reference fs capability; got stderr: {stderr}");
}

#[test]
fn fs_list_loses_under_sealed() {
    let (_, stdout, stderr) = run_probe("fs_list", Some("--sealed"));
    assert_eq!(classify_streams(&stdout, &stderr), ProbeOutcome::Loses,
        "CAPS-EXT 6: --sealed must block fs_list; stdout: {stdout}\nstderr: {stderr}");
}

#[test]
fn fs_stat_loses_under_sealed() {
    let (_, stdout, stderr) = run_probe("fs_stat", Some("--sealed"));
    assert_eq!(classify_streams(&stdout, &stderr), ProbeOutcome::Loses,
        "CAPS-EXT 6: --sealed must block fs_stat; stdout: {stdout}\nstderr: {stderr}");
}

// CAPS-EXT 7: fs write methods now route through dispatcher.

#[test]
fn fs_write_loses_under_sealed() {
    // Remove any prior marker so the assertion is unambiguous.
    let marker = "/tmp/cruftless-probe-fs-write.marker";
    let _ = std::fs::remove_file(marker);
    let (_, stdout, stderr) = run_probe("fs_write", Some("--sealed"));
    assert_eq!(classify_streams(&stdout, &stderr), ProbeOutcome::Loses,
        "CAPS-EXT 7: --sealed must block fs_write; stdout: {stdout}\nstderr: {stderr}");
    assert!(!std::path::Path::new(marker).exists(),
        "marker file at {marker} should NOT exist after --sealed run; the write was supposed to be refused");
}

// CAPS-EXT 8: process route-through.

#[test]
fn process_exit_loses_under_sealed() {
    let (code, stdout, stderr) = run_probe("process_exit", Some("--sealed"));
    // Under --sealed, the dispatcher refuses before std::process::exit fires;
    // the probe's catch branch runs and prints LOSES, and the host exits 0.
    assert_ne!(code, 42,
        "CAPS-EXT 8: --sealed must NOT honor process.exit(42); code: {code}");
    assert_eq!(classify_streams(&stdout, &stderr), ProbeOutcome::Loses,
        "process_exit should LOSE under --sealed; stdout: {stdout}\nstderr: {stderr}");
    assert!(stderr.contains("process"),
        "loss message should reference process capability; got stderr: {stderr}");
}

#[test]
fn cwd_read_loses_under_sealed() {
    let (_, stdout, stderr) = run_probe("cwd_read", Some("--sealed"));
    assert_eq!(classify_streams(&stdout, &stderr), ProbeOutcome::Loses,
        "CAPS-EXT 8: --sealed must block process.cwd; stdout: {stdout}\nstderr: {stderr}");
}

// CAPS-EXT 9: env route-through.

#[test]
fn env_read_loses_under_sealed() {
    // Under --sealed, process.env is installed empty at host startup;
    // the probe detects the emptiness and reports LOSES via stderr.
    let (_, stdout, stderr) = run_probe("env_read", Some("--sealed"));
    assert_eq!(classify_streams(&stdout, &stderr), ProbeOutcome::Loses,
        "CAPS-EXT 9: --sealed must yield empty process.env; stdout: {stdout}\nstderr: {stderr}");
}

// CAPS-EXT 10: stdio route-through.

#[test]
fn baseline_stdio_exfil_wins() {
    let (_, stdout, _) = run_probe("stdio_exfil", None);
    assert_eq!(classify(&stdout), ProbeOutcome::Wins,
        "Mode 0 baseline: process.stdout.write should succeed; got: {stdout}");
    assert!(stdout.contains("ATTACKER-CONTROLLED-BYTES"),
        "the WINS bytes should have been written to stdout; got: {stdout}");
}

#[test]
fn stdio_exfil_loses_under_sealed() {
    let (_, stdout, stderr) = run_probe("stdio_exfil", Some("--sealed"));
    assert_eq!(classify_streams(&stdout, &stderr), ProbeOutcome::Loses,
        "CAPS-EXT 10: --sealed must block process.stdout.write; stdout: {stdout}\nstderr: {stderr}");
    // Critically: the attacker bytes must NOT have reached stdout.
    assert!(!stdout.contains("ATTACKER-CONTROLLED-BYTES"),
        "attacker-controlled bytes leaked to stdout under --sealed: {stdout}");
    assert!(stderr.contains("stdio"),
        "loss message should reference stdio capability; got stderr: {stderr}");
}
