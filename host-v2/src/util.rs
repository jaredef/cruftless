//! node:util intrinsic — Tier-Ω.5.s.
//!
//! Mixed: `inspect` / `format` / `inherits` / `deprecate` / `types` are
//! actually implemented enough for pathe + common callers; `promisify`
//! and `callbackify` are stubs.

use crate::register::{new_object, register_method, set_constant};
use rusty_js_runtime::abstract_ops;
use rusty_js_runtime::value::{InternalKind, Object};
use rusty_js_runtime::{Runtime, RuntimeError, Value};
use std::rc::Rc;

pub fn install(rt: &mut Runtime) {
    let util = new_object(rt);

    // Tier-Ω.5.nnnnnn: util.debuglog(section) → no-op callable logger.
    // Ω.5.P06.L3.util-style-text: Node 22+'s util.styleText(format, text)
    // applies ANSI color codes. Cruftless's v1 stub ignores the format
    // and returns text verbatim — no ANSI codes, but the function is
    // callable. Packages that pattern-match on `util.styleText` typeof
    // === 'function' as a runtime-capability probe (the @inquirer/core
    // theme.js:5 pattern) load cleanly. Bun renders the codes; cruftless
    // renders plain text. Output divergence is acceptable for the load-
    // and-shape probe; a real ANSI implementation is queued for when
    // terminal-output fidelity becomes load-bearing.
    register_method(rt, util, "styleText", |_rt, args| {
        let text = args.get(1).cloned().unwrap_or(Value::Undefined);
        match text {
            Value::String(_) => Ok(text),
            other => Ok(Value::String(Rc::new(format!("{:?}", other)))),
        }
    });
    register_method(rt, util, "debuglog", |rt, _args| {
        // Return a callable native function that ignores its args.
        // Reuse the global Function ctor pattern via make-stub-fn.
        let f = crate::register::make_callable(rt, "debuglog_fn", |_rt, _args| Ok(Value::Undefined));
        rt.object_set(f, "enabled".into(), Value::Boolean(false));
        Ok(Value::Object(f))
    });
    register_method(rt, util, "deprecate", |_rt, args| {
        // Returns the original function unchanged (skip deprecation wrapper).
        Ok(args.first().cloned().unwrap_or(Value::Undefined))
    });
    // inspect(v) → JSON.stringify(v, null, 2). Close enough for v1.
    // Ω.5.P53.E6: inspect carries a `custom` symbol property whose value
    // is Symbol.for('nodejs.util.inspect.custom'). ts-node and other
    // consumers test `util.inspect.custom` and re-export its typeof —
    // returning a real Symbol (not undefined → string-fallback) closes
    // the L5 cut surfaced by the ts-node probe.
    let inspect_fn = crate::register::make_callable(rt, "inspect", |rt, args| {
        let v = args.first().cloned().unwrap_or(Value::Undefined);
        let s = json_stringify_via_intrinsic(rt, &v)?;
        Ok(Value::String(Rc::new(s)))
    });
    rt.object_set(
        inspect_fn,
        "custom".into(),
        Value::Symbol(Rc::new("@@sym:nodejs.util.inspect.custom".to_string())),
    );
    rt.object_set(util, "inspect".into(), Value::Object(inspect_fn));

    // format(fmt, ...args) → printf-style substitution with %s/%d/%j.
    register_method(rt, util, "format", |rt, args| {
        if args.is_empty() {
            return Ok(Value::String(Rc::new(String::new())));
        }
        let fmt = match &args[0] {
            Value::String(s) => s.as_str().to_string(),
            other => abstract_ops::to_string(other).as_str().to_string(),
        };
        let mut out = String::new();
        let mut chars = fmt.chars().peekable();
        let mut arg_idx = 1usize;
        while let Some(c) = chars.next() {
            if c == '%' {
                match chars.next() {
                    Some('s') => {
                        let a = args.get(arg_idx).cloned().unwrap_or(Value::Undefined);
                        arg_idx += 1;
                        out.push_str(&abstract_ops::to_string(&a));
                    }
                    Some('d') | Some('i') => {
                        let a = args.get(arg_idx).cloned().unwrap_or(Value::Undefined);
                        arg_idx += 1;
                        let n = abstract_ops::to_number(&a);
                        if n.is_nan() {
                            out.push_str("NaN");
                        } else {
                            out.push_str(&(n.trunc() as i64).to_string());
                        }
                    }
                    Some('f') => {
                        let a = args.get(arg_idx).cloned().unwrap_or(Value::Undefined);
                        arg_idx += 1;
                        out.push_str(&abstract_ops::to_number(&a).to_string());
                    }
                    Some('j') => {
                        let a = args.get(arg_idx).cloned().unwrap_or(Value::Undefined);
                        arg_idx += 1;
                        let s = json_stringify_via_intrinsic(rt, &a)?;
                        out.push_str(&s);
                    }
                    Some('%') => out.push('%'),
                    Some(other) => {
                        out.push('%');
                        out.push(other);
                    }
                    None => out.push('%'),
                }
            } else {
                out.push(c);
            }
        }
        // Trailing args appended space-separated, per Node semantics.
        for i in arg_idx..args.len() {
            out.push(' ');
            out.push_str(&abstract_ops::to_string(&args[i]));
        }
        Ok(Value::String(Rc::new(out)))
    });

    // inherits(ctor, super_): ctor.super_ = super_;
    //   ctor.prototype = Object.create(super_.prototype, {constructor:{value:ctor}})
    register_method(rt, util, "inherits", |rt, args| {
        let ctor_id = match args.first() {
            Some(Value::Object(id)) => *id,
            _ => return Err(RuntimeError::TypeError(
                "util.inherits: ctor must be an object".into())),
        };
        let super_id = match args.get(1) {
            Some(Value::Object(id)) => *id,
            _ => return Err(RuntimeError::TypeError(
                "util.inherits: super must be an object".into())),
        };
        rt.object_set(ctor_id, "super_".into(), Value::Object(super_id));
        let super_proto = rt.object_get(super_id, "prototype");
        let new_proto = rt.alloc_object(Object::new_ordinary());
        if let Value::Object(sp) = super_proto {
            // Set [[Prototype]] of new_proto to super_proto.
            let _ = rt.obj_mut(new_proto);
            rt.obj_mut(new_proto).proto = Some(sp);
        }
        rt.obj_mut(new_proto).set_own_internal("constructor".into(), Value::Object(ctor_id));
        rt.obj_mut(ctor_id).set_own_frozen("prototype".into(), Value::Object(new_proto));
        Ok(Value::Undefined)
    });

    // Tier-Ω.5.ddd: promisify / callbackify v1 stub. Real semantics
    // (callback-style → Promise-returning wrapper) need a full Promise
    // implementation in the runtime; for module-load-time evaluation,
    // returning the input function unchanged lets dependent libraries
    // (node-fetch, etc.) at least load and probe their namespaces.
    register_method(rt, util, "promisify", |_rt, args| {
        Ok(args.first().cloned().unwrap_or(Value::Undefined))
    });
    register_method(rt, util, "callbackify", |_rt, args| {
        Ok(args.first().cloned().unwrap_or(Value::Undefined))
    });
    register_method(rt, util, "deprecate", |_rt, args| {
        // Return fn unchanged; v1 drops the deprecation warning.
        Ok(args.first().cloned().unwrap_or(Value::Undefined))
    });

    // types subobject with InternalKind-based checks.
    let types = new_object(rt);
    register_method(rt, types, "isPromise", |rt, args| {
        Ok(Value::Boolean(matches!(args.first(),
            Some(Value::Object(id)) if matches!(rt.obj(*id).internal_kind, InternalKind::Promise(_)))))
    });
    register_method(rt, types, "isRegExp", |rt, args| {
        Ok(Value::Boolean(matches!(args.first(),
            Some(Value::Object(id)) if matches!(rt.obj(*id).internal_kind, InternalKind::RegExp(_)))))
    });
    register_method(rt, types, "isMap", |_rt, _args| Ok(Value::Boolean(false)));
    register_method(rt, types, "isSet", |_rt, _args| Ok(Value::Boolean(false)));
    register_method(rt, types, "isDate", |_rt, _args| Ok(Value::Boolean(false)));
    register_method(rt, types, "isNativeError", |rt, args| {
        Ok(Value::Boolean(matches!(args.first(),
            Some(Value::Object(id)) if matches!(rt.obj(*id).internal_kind, InternalKind::Error))))
    });
    register_method(rt, types, "isArrayBuffer", |_rt, _args| Ok(Value::Boolean(false)));
    register_method(rt, types, "isTypedArray", |_rt, _args| Ok(Value::Boolean(false)));
    set_constant(rt, util, "types", Value::Object(types));

    set_constant(rt, util, "default", Value::Object(util));
    rt.globals.insert("util".into(), Value::Object(util));
}

fn json_stringify_via_intrinsic(rt: &mut Runtime, v: &Value) -> Result<String, RuntimeError> {
    let json = rt
        .globals
        .get("JSON")
        .cloned()
        .ok_or_else(|| RuntimeError::TypeError("JSON intrinsic missing".into()))?;
    let json_id = match json {
        Value::Object(id) => id,
        _ => return Err(RuntimeError::TypeError("JSON is not an object".into())),
    };
    let stringify = rt.object_get(json_id, "stringify");
    let s = rt.call_function(stringify, Value::Object(json_id), vec![v.clone()])?;
    Ok(match s {
        Value::String(s) => s.as_str().to_string(),
        _ => String::new(),
    })
}
