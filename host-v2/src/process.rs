//! process intrinsic — minimal v1 surface.

use crate::register::{new_object, register_method, set_constant};
use rusty_js_runtime::{Runtime, RuntimeError, Value};
use rusty_js_runtime::caps as caps;
use rusty_js_runtime::caps::{ModuleId, ModuleProvenance};
use std::rc::Rc;

/// CAPS-EXT 11: gate a clock operation through the dispatcher.
fn check_clock(rt: &Runtime, op: caps::ClockOp) -> Result<(), RuntimeError> {
    let url = rt.current_module_url.last().cloned().unwrap_or_default();
    let provenance = if url.contains("/node_modules/") {
        ModuleProvenance::Dependency
    } else if url.starts_with("node:") {
        ModuleProvenance::Builtin
    } else {
        ModuleProvenance::Application
    };
    let caller = ModuleId { url, provenance };
    rt.caps.require_clock(&caps::Clock::disabled(), op, &caller)
        .map_err(|e| RuntimeError::TypeError(e.to_string()))
}

/// CAPS-EXT 8: gate a process operation through the capability
/// dispatcher. Same shape as host-v2/src/fs.rs's check_fs.
fn check_process(rt: &Runtime, op: caps::ProcessOp) -> Result<(), RuntimeError> {
    let url = rt.current_module_url.last().cloned().unwrap_or_default();
    let provenance = if url.contains("/node_modules/") {
        ModuleProvenance::Dependency
    } else if url.starts_with("node:") {
        ModuleProvenance::Builtin
    } else {
        ModuleProvenance::Application
    };
    let caller = ModuleId { url, provenance };
    rt.caps.require_process(&caps::Process::none(), op, &caller)
        .map_err(|e| RuntimeError::TypeError(e.to_string()))
}

pub fn install(rt: &mut Runtime, argv: Vec<String>) {
    let process = new_object(rt);

    // argv: ["cruftless", <script>, ...rest]
    let argv_array = rt.alloc_object(rusty_js_runtime::value::Object::new_array());
    for (i, s) in argv.iter().enumerate() {
        rt.object_set(argv_array, i.to_string(), Value::String(Rc::new(s.clone())));
    }
    set_constant(rt, process, "argv", Value::Object(argv_array));

    // Ω.5.P06.L3.process-execargv: process.execArgv is documented as an
    // Array of strings (Node's CLI flags passed before the script name).
    // Bun reports []; cruftless previously returned undefined, which
    // packages that probe `process.execArgv.length` (resolve-package-path's
    // should-preserve-symlinks at lib/should-preserve-symlinks.js:4 is the
    // canonical case) crash on. Empty-array default matches Bun's shape
    // and the spec'd Node behavior when no flags were forwarded.
    let exec_argv = rt.alloc_object(rusty_js_runtime::value::Object::new_array());
    rt.object_set(exec_argv, "length".into(), Value::Number(0.0));
    set_constant(rt, process, "execArgv", Value::Object(exec_argv));

    // env: snapshot of std::env::vars() at startup.
    //
    // CAPS-EXT 9 mode-aware install: under Mode 3 (--sealed) and Mode 2
    // (--sealed-deps), install an empty object — JS code reading
    // `process.env.HOME` gets `undefined` and cannot exfiltrate. Under
    // Mode 0 / Mode 1 (default + --audit), install the full snapshot so
    // existing npm packages that read process.env at module load (PATH,
    // NODE_ENV, HOME, etc.) keep working.
    //
    // This is the install-time form of capability enforcement. A
    // future round will lift it to per-property getter semantics so
    // Mode 2 can give the application the full env while sealing deps;
    // for the first cut, the all-or-nothing install matches Mode 3's
    // semantics correctly and Mode 2's partial.
    let env_obj = new_object(rt);
    let mode = rt.caps.mode;
    if matches!(mode, rusty_js_runtime::caps::CapMode::Compat | rusty_js_runtime::caps::CapMode::Audit) {
        let vars: Vec<(String, String)> = std::env::vars().collect();
        for (k, v) in vars {
            rt.object_set(env_obj, k, Value::String(Rc::new(v)));
        }
    }
    set_constant(rt, process, "env", Value::Object(env_obj));

    set_constant(rt, process, "platform", Value::String(Rc::new(
        if cfg!(target_os = "linux") { "linux" }
        else if cfg!(target_os = "macos") { "darwin" }
        else { "unknown" }.to_string()
    )));
    set_constant(rt, process, "arch", Value::String(Rc::new(
        if cfg!(target_arch = "x86_64") { "x64" }
        else if cfg!(target_arch = "aarch64") { "arm64" }
        else { "unknown" }.to_string()
    )));
    // Ω.5.P45.E2: report a real Node major version. Many packages
    // pattern-match `process.version` against /^v\d+\.\d+\.\d+$/ for
    // platform-support gating (nx is the canonical case — its native
    // dependency loader prints "NX Missing Platform Dependency" if the
    // version doesn't parse as Node-shaped). Match the process.versions.node
    // value below so consumers cross-checking the two agree.
    set_constant(rt, process, "version", Value::String(Rc::new("v20.10.0".to_string())));
    // Tier-Ω.5.pppp: process.versions for fast-glob + many libs that gate
    // behavior on node major version.
    let versions = new_object(rt);
    rt.object_set(versions, "node".into(), Value::String(Rc::new("20.10.0".into())));
    rt.object_set(versions, "v8".into(), Value::String(Rc::new("11.3.244.8".into())));
    rt.object_set(versions, "uv".into(), Value::String(Rc::new("1.46.0".into())));
    rt.object_set(versions, "modules".into(), Value::String(Rc::new("115".into())));
    set_constant(rt, process, "versions", Value::Object(versions));
    set_constant(rt, process, "pid", Value::Number(std::process::id() as f64));
    // Tier-Ω.5.nnnnnn: process.stdout / stderr / stdin minimal shapes
    // — many libs check isTTY + fd at module-load to choose color/style.
    for (name, fd_num) in [("stdout", 1.0), ("stderr", 2.0), ("stdin", 0.0)] {
        let s = new_object(rt);
        rt.object_set(s, "isTTY".into(), Value::Boolean(false));
        rt.object_set(s, "fd".into(), Value::Number(fd_num));
        rt.object_set(s, "columns".into(), Value::Number(80.0));
        rt.object_set(s, "rows".into(), Value::Number(24.0));
        let fd = fd_num as u32;
        register_method(rt, s, "write", move |rt, args| {
            if let Some(Value::String(s)) = args.first() {
                // CAPS-EXT 10: gate stdout writes (fd=1) only; stderr
                // (fd=2) remains the probe-harness escape valve for now.
                if fd == 1 {
                    let bytes = s.as_bytes().to_vec();
                    let url = rt.current_module_url.last().cloned().unwrap_or_default();
                    let provenance = if url.contains("/node_modules/") {
                        ModuleProvenance::Dependency
                    } else if url.starts_with("node:") {
                        ModuleProvenance::Builtin
                    } else {
                        ModuleProvenance::Application
                    };
                    let caller = ModuleId { url, provenance };
                    rt.caps.require_stdio(&caps::Stdio::none(), caps::StdioOp::Stdout(bytes), &caller)
                        .map_err(|e| RuntimeError::TypeError(e.to_string()))?;
                    print!("{}", s);
                } else {
                    eprint!("{}", s);
                }
            }
            Ok(Value::Boolean(true))
        });
        register_method(rt, s, "on", |rt, _args| Ok(rt.current_this()));
        set_constant(rt, process, name, Value::Object(s));
    }

    register_method(rt, process, "cwd", |rt, _args| {
        check_process(rt, caps::ProcessOp::ReadCwd)?;
        let cwd = std::env::current_dir()
            .ok()
            .and_then(|p| p.to_str().map(|s| s.to_string()))
            .unwrap_or_else(|| "/".to_string());
        Ok(Value::String(Rc::new(cwd)))
    });

    register_method(rt, process, "exit", |rt, args| {
        let code = args.first().map(|v| {
            rusty_js_runtime::abstract_ops::to_number(v) as i32
        }).unwrap_or(0);
        check_process(rt, caps::ProcessOp::Exit(code))?;
        std::process::exit(code);
    });

    register_method(rt, process, "hrtime", |rt, _args| {
        use std::time::{SystemTime, UNIX_EPOCH};
        check_clock(rt, caps::ClockOp::HighResolution)?;
        let d = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default();
        let arr = rt.alloc_object(rusty_js_runtime::value::Object::new_array());
        rt.object_set(arr, "0".into(), Value::Number(d.as_secs() as f64));
        rt.object_set(arr, "1".into(), Value::Number(d.subsec_nanos() as f64));
        Ok(Value::Object(arr))
    });
    // Tier-Ω.5.DDDDDDDD: process.hrtime.bigint() returns nanosecond BigInt
    // since the unix epoch. pino / pino-http call this at module-init for
    // their time-stamping closures.
    if let rusty_js_runtime::Value::Object(hrtime_id) = rt.object_get(process, "hrtime") {
        let bigint_fn: rusty_js_runtime::value::NativeFn = std::rc::Rc::new(|rt, _args| {
            use std::time::{SystemTime, UNIX_EPOCH};
            check_clock(rt, caps::ClockOp::HighResolution)?;
            let ns = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_nanos() as i64;
            Ok(Value::BigInt(std::rc::Rc::new(rusty_js_runtime::bigint::JsBigInt::from_i64(ns))))
        });
        let mut bigint_props = indexmap::IndexMap::new();
        rusty_js_runtime::value::install_function_meta_props(&mut bigint_props, "bigint", 0.0);
        let bigint_obj = rusty_js_runtime::value::Object {
            proto: None, extensible: true, properties: bigint_props,
            internal_kind: rusty_js_runtime::value::InternalKind::Function(
                rusty_js_runtime::value::FunctionInternals { name: "bigint".into(), length: 0, native: bigint_fn , is_constructor: true }
            ),
        };
        let bigint_id = rt.alloc_object(bigint_obj);
        rt.object_set(hrtime_id, "bigint".into(), Value::Object(bigint_id));
    }

    // Tier-Ω.5.DDDDDDDD: process.binding(name) — legacy internal API.
    // mock-fs and a handful of low-level packages probe it. Returns
    // empty namespace object so module-init reads pass; downstream
    // .fs / .uv methods return undefined.
    register_method(rt, process, "binding", |rt, _args| {
        let o = rt.alloc_object(rusty_js_runtime::value::Object::new_ordinary());
        Ok(Value::Object(o))
    });
    // process.report — Node's reporting surface. nx + others touch it.
    let report = rt.alloc_object(rusty_js_runtime::value::Object::new_ordinary());
    rt.object_set(report, "reportOnFatalError".into(), Value::Boolean(false));
    rt.object_set(report, "reportOnSignal".into(), Value::Boolean(false));
    rt.object_set(report, "reportOnUncaughtException".into(), Value::Boolean(false));
    rt.object_set(report, "directory".into(), Value::String(std::rc::Rc::new(String::new())));
    register_method(rt, report, "writeReport", |_rt, _a| Ok(Value::String(std::rc::Rc::new(String::new()))));
    register_method(rt, report, "getReport", |rt, _a| Ok(Value::Object(rt.alloc_object(rusty_js_runtime::value::Object::new_ordinary()))));
    rt.object_set(process, "report".into(), Value::Object(report));

    // Tier-Ω.5.cccc: process.nextTick(cb, ...args) — synchronous-ish
    // queuing of the callback. v1 deviation: invokes immediately since
    // we don't yet have a microtask integration at the JS-callable
    // surface. pump and many CJS streams rely on its existence and
    // single-callback shape.
    register_method(rt, process, "nextTick", |rt, args| {
        if let Some(cb) = args.first().cloned() {
            let rest: Vec<Value> = args.iter().skip(1).cloned().collect();
            let _ = rt.call_function(cb, Value::Undefined, rest);
        }
        Ok(Value::Undefined)
    });
    register_method(rt, process, "emit", |_rt, _args| Ok(Value::Boolean(false)));
    register_method(rt, process, "on", |rt, _args| Ok(rt.current_this()));
    register_method(rt, process, "off", |rt, _args| Ok(rt.current_this()));
    register_method(rt, process, "once", |rt, _args| Ok(rt.current_this()));
    register_method(rt, process, "removeListener", |rt, _args| Ok(rt.current_this()));
    // EventEmitter alias surface — nx daemon client calls process.addListener.
    register_method(rt, process, "addListener", |rt, _args| Ok(rt.current_this()));
    register_method(rt, process, "removeAllListeners", |rt, _args| Ok(rt.current_this()));
    register_method(rt, process, "prependListener", |rt, _args| Ok(rt.current_this()));
    register_method(rt, process, "prependOnceListener", |rt, _args| Ok(rt.current_this()));
    register_method(rt, process, "listeners", |rt, _args| {
        Ok(Value::Object(rt.alloc_object(rusty_js_runtime::value::Object::new_array())))
    });
    register_method(rt, process, "rawListeners", |rt, _args| {
        Ok(Value::Object(rt.alloc_object(rusty_js_runtime::value::Object::new_array())))
    });
    register_method(rt, process, "listenerCount", |_rt, _args| Ok(Value::Number(0.0)));
    register_method(rt, process, "eventNames", |rt, _args| {
        Ok(Value::Object(rt.alloc_object(rusty_js_runtime::value::Object::new_array())))
    });
    register_method(rt, process, "setMaxListeners", |rt, _args| Ok(rt.current_this()));
    register_method(rt, process, "getMaxListeners", |_rt, _args| Ok(Value::Number(10.0)));

    // Tier-Ω.5.mmmm: process.getBuiltinModule(name) — Node 22+ API. ohash
    // calls it at module init to fetch node:crypto without going through
    // the loader.
    register_method(rt, process, "getBuiltinModule", |rt, args| {
        let name = match args.first() {
            Some(Value::String(s)) => s.as_str().to_string(),
            _ => return Ok(Value::Undefined),
        };
        let stripped = name.strip_prefix("node:").unwrap_or(&name);
        match rt.globals.get(stripped).cloned() {
            Some(v) => Ok(v),
            None => Ok(Value::Undefined),
        }
    });

    rt.globals.insert("process".into(), Value::Object(process));
}
