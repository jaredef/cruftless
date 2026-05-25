//! CAPS-EXT 4: audit recorder + --audit CLI flag + log-drain test.
//!
//! Until CAPS-EXT 6+ wires effectful methods through the dispatcher,
//! no audit records appear from real JS code. This test exercises the
//! plumbing by:
//!   1. invoking `cruftless --audit fixture.mjs` with --audit-log set
//!   2. asserting the binary still exits 0 (PM-EXT-style smoke under
//!      Mode 1)
//!   3. asserting the audit log records the routed stdio effect.

use std::process::Command;

fn bin() -> &'static str { env!("CARGO_BIN_EXE_cruftless") }

fn tmp_path(tag: &str) -> std::path::PathBuf {
    let mut p = std::env::temp_dir();
    p.push(format!("cruftless-caps-{tag}-{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()));
    p
}

#[test]
fn audit_mode_smoke_compat_behavior() {
    // A trivial .mjs that does nothing dangerous. Under --audit the
    // runtime should run it identically to Mode 0.
    let src = tmp_path("src.mjs");
    std::fs::write(&src, b"console.log('hello');\n").unwrap();
    let log = tmp_path("audit.log");

    let out = Command::new(bin())
        .arg("--audit")
        .arg("--audit-log").arg(&log)
        .arg(&src)
        .output()
        .expect("run cruftless --audit");

    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(out.status.success(),
        "--audit run should exit 0 (Mode 1 == Mode 0 + logging)\nstdout: {stdout}\nstderr: {stderr}");
    assert!(stdout.contains("hello"), "expected 'hello' in stdout; got: {stdout}");

    let body = std::fs::read_to_string(&log).expect("audit log should be written");
    assert!(body.contains("# cruft audit log"), "log file malformed: {body}");
    assert!(body.contains("\tstdio\twrite(stdout)\t"), "expected stdout audit record: {body}");

    let _ = std::fs::remove_file(&src);
    let _ = std::fs::remove_file(&log);
}

#[test]
fn mode_flag_parsed_does_not_affect_compat_run() {
    // Sealed mode without any effectful calls should behave identically
    // to Compat. Console output is now routed through stdio and denied
    // under Mode 3, so keep this fixture genuinely effect-free.
    let src = tmp_path("seal-src.mjs");
    std::fs::write(&src, b"const sealedNoop = 1 + 1;\n").unwrap();

    let out = Command::new(bin())
        .arg("--sealed")
        .arg(&src)
        .output()
        .expect("run cruftless --sealed");
    assert!(out.status.success(),
        "Mode 3 on a no-effectful-call program should still exit 0; got {:?}", out.status);

    let _ = std::fs::remove_file(&src);
}

#[test]
fn env_var_mode_override() {
    let src = tmp_path("env-src.mjs");
    std::fs::write(&src, b"console.log('env-mode');\n").unwrap();

    let out = Command::new(bin())
        .env("CRUFTLESS_CAPS_MODE", "audit")
        .arg(&src)
        .output()
        .expect("run cruftless w/ env CRUFTLESS_CAPS_MODE=audit");
    assert!(out.status.success());

    let _ = std::fs::remove_file(&src);
}
