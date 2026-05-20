//! cruftless entry point. Reads a JS source file, evaluates it
//! through the rusty-js-runtime engine, drives the event loop to
//! completion, exits.

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

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("usage: {} <file.mjs>", args.get(0).map(|s| s.as_str()).unwrap_or("cruftless"));
        return ExitCode::from(64); // EX_USAGE
    }
    let path = args[1].clone();
    let source = match std::fs::read_to_string(&path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("cruftless: cannot read {}: {}", path, e);
            return ExitCode::from(66); // EX_NOINPUT
        }
    };

    let mut rt = Runtime::new();
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
            eprintln!("cruftless: evaluation error: {}", msg);
            return ExitCode::from(70);
        }
    }

    let t_loop = if std::env::var("CRUFTLESS_PROFILE").is_ok() {
        Some(std::time::Instant::now())
    } else { None };
    if let Err(e) = rt.run_to_completion() {
        eprintln!("cruftless: event-loop error: {:?}", e);
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
            "cruftless-profile: modules={} parse={:.1}ms compile={:.1}ms eval={:.1}ms event_loop={:.1}ms total_phases={:.1}ms",
            modules, ms(parse), ms(compile), ms(eval), ms(loop_ns),
            ms(parse + compile + eval + loop_ns),
        );
    }

    let unhandled = rt.drain_unhandled_rejections();
    if !unhandled.is_empty() {
        for (_id, reason) in &unhandled {
            eprintln!("cruftless: unhandled promise rejection: {:?}", reason);
        }
        return ExitCode::from(70);
    }

    ExitCode::SUCCESS
}
