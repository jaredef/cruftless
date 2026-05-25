//! `cruft` CLI entry point. The binary's name is `cruft`; the crate
//! and library remain `cruftless` (the architectural concept per
//! corpus Doc 729). Reads a JS source file, evaluates it through the
//! rusty-js-runtime engine, drives the event loop to completion, exits.

use cruftless::install_bun_host;
use rusty_js_runtime::{Runtime, Value};
use std::process::ExitCode;

// Ω.5.P46.E1.napi-v1: reference the keepalive array so the linker
// retains every napi_* C symbol. Without this Rust dead-code-strips the
// `#[no_mangle] pub extern "C"` shims (they're not referenced by any
// other Rust code) and dlopen'd .node modules can't resolve them via
// dlsym. The array itself is declared `#[no_mangle] pub static`, so
// just reading its length here is enough to anchor every entry.
#[used]
static _NAPI_RETAIN: usize = rusty_js_runtime::napi::NAPI_KEEPALIVE.len();

fn format_thrown(rt: &Runtime, v: &Value) -> String {
    match v {
        Value::String(s) => format!("Thrown: {}", s),
        Value::Object(id) => {
            let name = match rt.object_get(*id, "name") { Value::String(s) => (*s).clone(), _ => String::new() };
            let message = match rt.object_get(*id, "message") { Value::String(s) => (*s).clone(), _ => String::new() };
            if !name.is_empty() && !message.is_empty() {
                format!("Thrown: {}: {}", name, message)
            } else if !message.is_empty() {
                format!("Thrown: {}", message)
            } else if !name.is_empty() {
                format!("Thrown: {}", name)
            } else {
                format!("Thrown: {:?}", v)
            }
        }
        _ => format!("Thrown: {:?}", v),
    }
}

fn run_install_subcommand() -> ExitCode {
    // PM-EXT 12: `cruft install` — runs pm_install against the
    // current working directory using the engagement's default
    // registry (registry.npmmirror.com per Doc 730 §XVI Case-4 scope).
    let cwd = match std::env::current_dir() {
        Ok(p) => p,
        Err(e) => { eprintln!("cruft install: cannot read cwd: {e}"); return ExitCode::from(66); }
    };
    let registry = std::env::var("CRUFT_REGISTRY")
        .or_else(|_| std::env::var("CRUFTLESS_REGISTRY"))
        .unwrap_or_else(|_| rusty_js_pm::resolver::DEFAULT_REGISTRY.to_string());
    eprintln!("cruft install: project={} registry={}", cwd.display(), registry);
    match rusty_js_pm::install::pm_install(&cwd, &registry) {
        Ok(report) => {
            for (n, v) in &report.installed { println!("+ {n}@{v}"); }
            for (n, v) in &report.skipped { println!("= {n}@{v}"); }
            eprintln!("cruft install: {} installed, {} skipped",
                report.installed.len(), report.skipped.len());
            ExitCode::from(0)
        }
        Err(e) => {
            eprintln!("cruft install: {e:?}");
            ExitCode::from(70)
        }
    }
}

/// CAPS-EXT 4: parse capability-mode flags from argv. Recognized:
///   --audit         → CapMode::Audit (Mode 1)
///   --sealed-deps   → CapMode::SealedDeps (Mode 2)
///   --sealed        → CapMode::Sealed (Mode 3)
/// Default: CapMode::Compat (Mode 0). Returns (mode, audit_log_path,
/// allow_net_loopback, remaining_args) where remaining_args is argv with
/// the flag(s) consumed.
fn parse_cap_flags(args: Vec<String>) -> (rusty_js_runtime::caps::CapMode, Option<String>, bool, Vec<String>) {
    use rusty_js_runtime::caps::CapMode;
    let mut mode = CapMode::Compat;
    let mut audit_path: Option<String> = None;
    let mut allow_net_loopback = false;
    let mut out: Vec<String> = Vec::with_capacity(args.len());
    let mut it = args.into_iter();
    while let Some(a) = it.next() {
        match a.as_str() {
            "--audit" => mode = CapMode::Audit,
            "--sealed-deps" => mode = CapMode::SealedDeps,
            "--sealed" => mode = CapMode::Sealed,
            "--audit-log" => {
                if let Some(p) = it.next() { audit_path = Some(p); }
            }
            "--allow-net-loopback" => allow_net_loopback = true,
            _ => out.push(a),
        }
    }
    // CRUFT_CAPS_MODE / CRUFTLESS_CAPS_MODE env var as override fallback
    // (the CRUFTLESS_ prefix preserved for one release of backwards-compat).
    if mode == CapMode::Compat {
        let env = std::env::var("CRUFT_CAPS_MODE")
            .or_else(|_| std::env::var("CRUFTLESS_CAPS_MODE"));
        if let Ok(s) = env {
            if let Some(m) = CapMode::from_str(&s) { mode = m; }
        }
    }
    if !allow_net_loopback {
        allow_net_loopback = std::env::var("CRUFT_ALLOW_NET_LOOPBACK")
            .or_else(|_| std::env::var("CRUFTLESS_ALLOW_NET_LOOPBACK"))
            .map(|v| matches!(v.as_str(), "1" | "true" | "yes"))
            .unwrap_or(false);
    }
    (mode, audit_path, allow_net_loopback, out)
}

fn drain_audit_log(rt: &rusty_js_runtime::Runtime, dest: Option<&str>) {
    let records = rt.caps.drain_audit();
    if records.is_empty() { return; }
    let mut sink: Box<dyn std::io::Write> = match dest {
        Some(path) => match std::fs::File::create(path) {
            Ok(f) => Box::new(std::io::BufWriter::new(f)),
            Err(e) => {
                eprintln!("cruft: could not open audit log {path}: {e}; writing to stderr");
                Box::new(std::io::stderr())
            }
        },
        None => Box::new(std::io::stderr()),
    };
    use std::io::Write;
    let _ = writeln!(sink, "# cruft audit log — {} records", records.len());
    let _ = writeln!(sink, "# format: <caller>\\t<capability>\\t<operation>\\t<unix_micros>");
    for r in &records {
        let _ = writeln!(sink, "{}\t{}\t{}\t{}",
            r.caller, r.capability, r.operation, r.timestamp_micros);
    }
    let _ = sink.flush();
}

fn print_help() {
    eprintln!("cruft {}", env!("CARGO_PKG_VERSION"));
    eprintln!("The most architecturally pure JavaScript runtime ever built.");
    eprintln!("Unfortunately, your JavaScript is still full of cruft.");
    eprintln!();
    eprintln!("USAGE:");
    eprintln!("    cruft <subcommand> [OPTIONS]");
    eprintln!("    cruft <file.mjs>            # shorthand for `cruft run <file.mjs>`");
    eprintln!();
    eprintln!("SUBCOMMANDS:");
    eprintln!("    run        Run JavaScript full of cruft");
    eprintln!("    install    Install dependencies into ./node_modules (more cruft will arrive)");
    eprintln!("    help       Print this message");
    eprintln!();
    eprintln!("OPTIONS:");
    eprintln!("    --audit              Enable capability audit mode (log all I/O attempts)");
    eprintln!("    --sealed-deps        Treat node_modules as sealed (no new I/O)");
    eprintln!("    --sealed             Seal everything (project + deps)");
    eprintln!("    --audit-log <path>   Write audit records to <path> (default: stderr)");
    eprintln!("    --allow-net-loopback Grant loopback listen authority in sealed modes");
    eprintln!("    -h, --help           Print help");
    eprintln!("    -V, --version        Print version");
    eprintln!();
    eprintln!("Note: Zero cruft in the engine. Maximum cruft in your node_modules.");
}

fn print_version() {
    println!("cruft {}", env!("CARGO_PKG_VERSION"));
}

fn main() -> ExitCode {
    let raw_args: Vec<String> = std::env::args().collect();
    let (cap_mode, audit_log_path, allow_net_loopback, args) = parse_cap_flags(raw_args);

    // Global flags handled before subcommand dispatch.
    if args.iter().skip(1).any(|a| a == "-h" || a == "--help" || a == "help") {
        print_help();
        return ExitCode::SUCCESS;
    }
    if args.iter().skip(1).any(|a| a == "-V" || a == "--version") {
        print_version();
        return ExitCode::SUCCESS;
    }

    if args.len() < 2 {
        print_help();
        return ExitCode::from(64); // EX_USAGE
    }
    if args[1] == "install" {
        return run_install_subcommand();
    }
    // `cruft run <file>` and bare `cruft <file>` both treat args[N] as the source.
    let path = if args[1] == "run" {
        if args.len() < 3 {
            eprintln!("cruft run: missing file argument");
            return ExitCode::from(64);
        }
        args[2].clone()
    } else {
        args[1].clone()
    };
    let source = match std::fs::read_to_string(&path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("cruft: cannot read {}: {}", path, e);
            return ExitCode::from(66); // EX_NOINPUT
        }
    };
    // TSR-EXT 4 (2026-05-24): extension-based dispatch into the TS
    // source-language resolver. `.ts` files are routed through the
    // type-stripper before being handed to evaluate_module — the IR
    // sees pure ECMAScript per Doc 729's resolver-instance purity
    // (C3 in pilots/ts-resolve/seed.md §I.2).
    let source = if path.ends_with(".ts") || path.ends_with(".mts") || path.ends_with(".cts") {
        match ts_resolve::strip::strip_ts(&source) {
            Ok((stripped, _witnesses)) => stripped,
            Err(e) => {
                eprintln!("cruft: ts strip error in {}: {}", path, e);
                return ExitCode::from(65); // EX_DATAERR
            }
        }
    } else {
        source
    };

    let mut rt = Runtime::new();
    rt.set_cap_mode(cap_mode);
    if allow_net_loopback {
        rt.caps = std::sync::Arc::new(
            rusty_js_runtime::caps::CapDispatcher::new(cap_mode)
                .with_net_grant(rusty_js_runtime::caps::Net::loopback_server())
        );
    }
    rt.install_intrinsics();
    install_bun_host(&mut rt, args);

    let url = format!("file://{}", path);
    match rt.evaluate_module(&source, &url) {
        Ok(_namespace) => {}
        Err(e) => {
            // Tier-Ω.5.hhhhh: stringify thrown Error objects via their
            // `message` / `name` properties rather than `[Object #NNN]`.
            // Doc 723 Layer-A: the surface message should at least name
            // what happened.
            let msg = match &e {
                rusty_js_runtime::RuntimeError::Thrown(v) => format_thrown(&rt, v),
                _ => format!("{:?}", e),
            };
            eprintln!("cruft: evaluation error: {}", msg);
            return ExitCode::from(70);
        }
    }

    let t_loop = if std::env::var("CRUFT_PROFILE").is_ok()
        || std::env::var("CRUFTLESS_PROFILE").is_ok()
    {
        Some(std::time::Instant::now())
    } else { None };
    if let Err(e) = rt.run_to_completion() {
        eprintln!("cruft: event-loop error: {:?}", e);
        return ExitCode::from(70);
    }
    if let Some(t) = t_loop {
        use rusty_js_runtime::module::phase_profile as pp;
        let loop_ns = t.elapsed().as_nanos() as u64;
        let parse = pp::read(&pp::PARSE_NS);
        let compile = pp::read(&pp::COMPILE_NS);
        let eval = pp::read(&pp::EVAL_NS);
        let modules = pp::read(&pp::MODULE_COUNT);
        let ms = |n: u64| (n as f64) / 1.0e6;
        eprintln!(
            "cruft-profile: modules={} parse={:.1}ms compile={:.1}ms eval={:.1}ms event_loop={:.1}ms total_phases={:.1}ms",
            modules, ms(parse), ms(compile), ms(eval), ms(loop_ns),
            ms(parse + compile + eval + loop_ns),
        );
    }

    let unhandled = rt.drain_unhandled_rejections();
    if !unhandled.is_empty() {
        for (_id, reason) in &unhandled {
            eprintln!("cruft: unhandled promise rejection: {:?}", reason);
        }
        drain_audit_log(&rt, audit_log_path.as_deref());
        return ExitCode::from(70);
    }

    drain_audit_log(&rt, audit_log_path.as_deref());
    ExitCode::SUCCESS
}
