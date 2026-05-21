//! PM-EXT 11: runtime smoke. End-to-end closure gate for the package-
//! manager workstream.
//!
//! Network-gated (`#[ignore]`): builds a tmpdir with package.json →
//! pm_install resolves + fetches + extracts + links lodash@4.17.21
//! into tmpdir/node_modules → writes app.mjs that requires lodash and
//! prints `identity(42)` → spawns the cruftless binary on app.mjs →
//! asserts the binary exits 0 with the expected stdout.
//!
//! This closes Doc 732 §VI's "first-cut success" definition: a user
//! with a package.json can run the package manager, then evaluate
//! installed code through the runtime, with no further intervention.

use std::path::PathBuf;
use std::process::Command;

use rusty_js_pm::install::pm_install;
use rusty_js_pm::resolver::DEFAULT_REGISTRY;

fn bin() -> &'static str { env!("CARGO_BIN_EXE_cruftless") }

fn tmpdir(tag: &str) -> PathBuf {
    let mut p = std::env::temp_dir();
    p.push(format!("cruftless-pm-runtime-{tag}-{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()));
    p
}

/// Closure gate: install lodash via the PM, then require + use it
/// through the cruftless runtime. PASS condition: exit 0 + stdout
/// contains "identity42=42".
#[test]
#[ignore]
fn pm_install_then_require_lodash() {
    let project = tmpdir("lodash-rt");
    std::fs::create_dir_all(&project).unwrap();
    std::fs::write(
        project.join("package.json"),
        br#"{"name":"app","version":"0.0.1","dependencies":{"lodash":"4.17.21"}}"#,
    ).unwrap();

    let report = pm_install(&project, DEFAULT_REGISTRY).expect("pm_install");
    assert_eq!(report.installed.len(), 1, "expected 1 install, got {report:?}");

    // app.mjs uses the require global that node_stubs.rs installs in
    // module contexts. Bare specifier "lodash" walks up node_modules
    // and finds project/node_modules/lodash.
    let app = project.join("app.mjs");
    std::fs::write(&app, br#"
const lodash = require('lodash');
const r = lodash.identity(42);
console.log('identity42=' + r);
console.log('keys=' + (Object.keys(lodash).length > 100 ? 'lots' : 'few'));
"#).unwrap();

    let out = Command::new(bin()).arg(&app).output()
        .expect("run cruftless binary");
    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(out.status.success(),
        "cruftless failed (status={:?})\nstdout: {}\nstderr: {}",
        out.status, stdout, stderr);
    assert!(stdout.contains("identity42=42"),
        "expected 'identity42=42' in stdout; got:\nstdout: {stdout}\nstderr: {stderr}");
    assert!(stdout.contains("keys=lots"),
        "expected 'keys=lots' (lodash exports >100 functions); got: {stdout}");

    let _ = std::fs::remove_dir_all(&project);
}

/// PM-EXT 12 CLI gate: `cruftless install` in a project dir, then
/// `cruftless app.mjs` requires the installed package. End-to-end via
/// the binary's own subcommand dispatcher, no Rust intermediary.
#[test]
#[ignore]
fn cli_install_then_run() {
    let project = tmpdir("cli");
    std::fs::create_dir_all(&project).unwrap();
    std::fs::write(
        project.join("package.json"),
        br#"{"name":"app","version":"0.0.1","dependencies":{"lodash":"4.17.21"}}"#,
    ).unwrap();

    let install_out = Command::new(bin())
        .arg("install")
        .current_dir(&project)
        .output()
        .expect("run cruftless install");
    let install_stdout = String::from_utf8_lossy(&install_out.stdout);
    let install_stderr = String::from_utf8_lossy(&install_out.stderr);
    assert!(install_out.status.success(),
        "install failed:\nstdout: {install_stdout}\nstderr: {install_stderr}");
    assert!(install_stdout.contains("+ lodash@4.17.21"),
        "expected '+ lodash@4.17.21' in stdout; got: {install_stdout}");
    assert!(project.join("node_modules/lodash/package.json").exists(),
        "lodash should be installed");
    assert!(project.join("cruftless-lock.json").exists(),
        "lockfile should be written");

    // Second install: idempotent, all skipped.
    let install2 = Command::new(bin())
        .arg("install")
        .current_dir(&project)
        .output()
        .expect("run cruftless install (2)");
    let s2 = String::from_utf8_lossy(&install2.stdout);
    assert!(install2.status.success());
    assert!(s2.contains("= lodash@4.17.21"),
        "second install should print '= lodash@4.17.21' (skipped); got: {s2}");

    // Run an .mjs that requires the installed package.
    let app = project.join("app.mjs");
    std::fs::write(&app, br#"
const lodash = require('lodash');
console.log('cli-identity=' + lodash.identity(7));
"#).unwrap();
    let run = Command::new(bin()).arg(&app).output().expect("run app.mjs");
    let rs = String::from_utf8_lossy(&run.stdout);
    let re = String::from_utf8_lossy(&run.stderr);
    assert!(run.status.success(),
        "run failed:\nstdout: {rs}\nstderr: {re}");
    assert!(rs.contains("cli-identity=7"),
        "expected 'cli-identity=7'; got: {rs}");

    let _ = std::fs::remove_dir_all(&project);
}
