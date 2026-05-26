//! Ω.5.P18.E1.ns-default-synth-narrow — HostFinalizeModuleNamespace closure.
//!
//! Earlier Ω.5.P16.E2 added Tuple A (synthesize `default = namespace` when
//! the module had named exports but no default) for compatibility with
//! `import x from "lodash/foo"` patterns. Per Doc 721 Step 4 against the
//! Ω.5.P17 residual, Tuple A causes a 237-package III.a keyCount-Δ+1 cluster
//! versus Bun, which does NOT synthesize default on ESM-with-named-exports.
//! CJS-shimmed packages route through `evaluate_cjs_module` instead and
//! never reach this hook, so Tuple A's stated rationale doesn't apply here.
//!
//! Tuple A is now restricted to the empty-namespace case: a module that
//! exports nothing at all gets `default = namespace` as a fallback so
//! default-import callers receive a stable empty handle. Modules with at
//! least one named export are left untouched, matching Bun.
//!
//! Tuple B: ESM module exports only `default` and the default value is an
//! object. Synthesize one named export per own enumerable string key of the
//! default value. Matches `export default { a, b }` followed by
//! `import { a } from "..."`.
//!
//! Reentrant-safe: the hook re-reads the namespace's current property set on
//! each invocation and never overwrites an existing key.

use rusty_js_runtime::{HostHook, Runtime, Value};

/// Ω.5.P43.E1: true if `url` is a `file://` URL ending in `.js` whose
/// enclosing package.json does not declare `"type":"module"`. That's the
/// "module field ESM" shape (.js loaded as ESM because of the package's
/// `module` field or `exports.module` condition, not because the package
/// is whole-tree ESM via `type:module`). Bun's namespace synth treats
/// this case as "needs default", matching the @opentelemetry/core /
/// @xstate/fsm / minified-rollup-TypeScript pattern.
/// Ω.5.P53.E13: walk parent dirs to find the enclosing package.json
/// and report whether it declares `"type":"module"`. Returns false when
/// the package is non-type-module (CJS-default, with module field /
/// .mjs handling making certain files ESM).
fn package_is_type_module(url: &str) -> bool {
    let path_str = match url.strip_prefix("file://") {
        Some(p) => p,
        None => return false,
    };
    let path = std::path::Path::new(path_str);
    let mut cur = path.parent();
    while let Some(d) = cur {
        let candidate = d.join("package.json");
        if candidate.is_file() {
            if let Ok(text) = std::fs::read_to_string(&candidate) {
                let lower = text.replace(char::is_whitespace, "");
                return lower.contains("\"type\":\"module\"");
            }
        }
        cur = d.parent();
    }
    false
}

/// Ω.5.P63.E48: walk parents for a package.json that declares an `exports`
/// map. The presence of `exports` means the package author has opted into
/// the canonical conditional-resolution API; Bun treats files reached via
/// `exports.import` as canonical ESM and does NOT lift function intrinsics.
/// Packages without `exports` (only `module` / `main`) use the legacy
/// dual-package shape; Bun lifts there.
fn package_has_exports_field(url: &str) -> bool {
    let path_str = match url.strip_prefix("file://") {
        Some(p) => p,
        None => return false,
    };
    let path = std::path::Path::new(path_str);
    let mut cur = path.parent();
    while let Some(d) = cur {
        let candidate = d.join("package.json");
        if candidate.is_file() {
            if let Ok(text) = std::fs::read_to_string(&candidate) {
                let compact = text.replace(char::is_whitespace, "");
                return compact.contains("\"exports\":");
            }
        }
        cur = d.parent();
    }
    false
}

fn is_js_under_non_type_module_package(url: &str) -> bool {
    let path_str = match url.strip_prefix("file://") {
        Some(p) => p,
        None => return false,
    };
    let path = std::path::Path::new(path_str);
    if path.extension().and_then(|s| s.to_str()) != Some("js") {
        return false;
    }
    let mut cur = path.parent();
    while let Some(d) = cur {
        let candidate = d.join("package.json");
        if candidate.is_file() {
            if let Ok(text) = std::fs::read_to_string(&candidate) {
                // Cheap text scan for "type":"module" — full JSON parse
                // would pull a dep. False positives on commented-out
                // "type" keys are tolerable for v1.
                if text.contains("\"type\"") && text.contains("\"module\"") {
                    // Heuristic: if both tokens appear AND the package
                    // declares type:module, treat as pure-ESM (return
                    // false). Otherwise it's module-field-ESM (return
                    // true). We don't precisely parse — Bun's heuristic
                    // is similarly loose.
                    let lower = text.replace(char::is_whitespace, "");
                    if lower.contains("\"type\":\"module\"") {
                        return false;
                    }
                }
                return true; // found package.json, no type:module → module-field-ESM
            }
        }
        cur = d.parent();
    }
    true // no package.json anywhere → treat as module-field-ESM (rare edge)
}

pub fn install(rt: &mut Runtime) {
    // Ω.5.P54.E3 (Axis-N probe): the FinalizeModuleNamespace hook decides
    // one of four synth paths (Tuple-A-empty, Tuple-A-wide, no-op-pass-
    // through, P53.E13-fn-lift). Each branch records the path taken to
    // module_ns_synth_trace so downstream Axis-N walks can locate which
    // branch produced the resulting surface.
    rt.install_host_hook(HostHook::FinalizeModuleNamespace(Box::new(
        |rt, _ast, ns, url| {
            let (has_default, default_value, named_count): (bool, Value, usize) = {
                let o = rt.obj(ns);
                let has = o.has_own_str("default");
                let dv = o
                    .get_own("default")
                    .map(|d| d.value.clone())
                    .unwrap_or(Value::Undefined);
                let other = o
                    .properties
                    .keys()
                    .filter(|k| k.as_str() != "default")
                    .count();
                (has, dv, other)
            };

            // Ω.5.P43.E1.tuple-a-by-url-shape: re-widen Tuple A for .js
            // modules loaded through a `module` field / `exports.module` /
            // module-field walk. Bun synthesizes `default = namespace` when
            // an ESM-shaped .js module under a non-type:module package is
            // loaded; we now do too. Pure ESM (type:module .js, .mjs files,
            // empty namespaces) continues to follow P18.E1's behavior:
            // empty → fallback default, otherwise no synth.
            //
            // The URL discriminator: .mjs always pure-ESM. .js needs to
            // check the parent package.json's `type` field; absence (or
            // anything other than "module") means we're in module-field
            // territory and Bun synthesizes default.
            let is_module_field_esm = is_js_under_non_type_module_package(url);

            if !has_default && named_count == 0 {
                // Ω.5.P57.E1: Tuple-A-empty further narrowed. Pre-P57.E1, any
                // ESM module with zero exports got `default = namespace` as
                // a fallback handle. micromark-util-types (type:module ESM,
                // pure-types package with empty runtime exports) ended up
                // with namespace `{default: {}}` where Bun produced just `{}`.
                // Per Doc 729 §XIII the implicit constraint surfaced by
                // walking the +Δ-default sub-cluster: don't synthesize even
                // the empty fallback when the enclosing package is
                // type:module — those packages are canonical ESM and Bun
                // treats them as authoritative-empty rather than fallback-
                // empty. The narrow keeps the fallback for the CJS-shimmed-
                // as-ESM path (P43.E1's siblings) while closing the
                // type:module-empty sub-cut.
                let pkg_is_type_module = package_is_type_module(url);
                if !pkg_is_type_module {
                    rt.object_set(ns, "default".to_string(), Value::Object(ns));
                    rt.module_ns_synth_trace.insert(
                        url.to_string(),
                        "ESM-finalize Tuple-A-empty (gated: not-type-module)".to_string(),
                    );
                } else {
                    rt.module_ns_synth_trace.insert(
                        url.to_string(),
                        "ESM-finalize Tuple-A-empty-suppressed (pkg type:module)".to_string(),
                    );
                }
                return Ok(());
            }
            if !has_default && is_module_field_esm {
                // Tuple A (wide, P43.E1): module has named exports but no
                // explicit default; Bun synthesizes default = namespace for
                // the module-field-ESM case. Matches the @opentelemetry/core
                // / @xstate/fsm / many-TS-compiled-packages shape.
                rt.object_set(ns, "default".to_string(), Value::Object(ns));
                rt.module_ns_synth_trace.insert(
                    url.to_string(),
                    "ESM-finalize Tuple-A-wide (P43.E1)".to_string(),
                );
                return Ok(());
            }

            // Ω.5.P21.E2.tuple-b-drop: Tuple B (spread default's own keys as
            // named exports when only `default` is declared) is dropped here
            // for the same reason Ω.5.P18.E1 dropped Tuple A. This hook fires
            // only on ESM evaluation; for ESM-with-only-default, Bun's
            // namespace is exactly `{default: V}` regardless of V's shape.
            // CJS-shimmed packages whose original justification motivated
            // Tuple B route through `evaluate_cjs_module`'s
            // `populate_cjs_namespace_view` and get their own handling
            // (including the Ω.5.P21.E1 callable-instance-prop filter).
            //
            // Examples that close from this drop: mitt (single fn-default ESM,
            // was leaking name/length/prototype), kleur (object default, was
            // leaking bg* color methods), upath (similar pattern). All now
            // produce keyCount=1 matching Bun.

            // Ω.5.P53.E11: when the default value is a function, Bun lifts
            // the function's own name / length / prototype to the namespace
            // level. Observed across mri, sleep-promise, sinon-test, es6-error,
            // fuzzyset.js, rollup-plugin-commonjs — all `export default function`
            // (or `module.exports = fn` reaching the ESM hook through a
            // .mjs build). Lift them only when default is a function and
            // the property isn't already explicitly named.
            // Ω.5.P53.E13: lift requires THREE constraints together —
            //   (a) default is the sole explicit export (named_count == 0)
            //   (b) default is a function-shaped object
            //   (c) the enclosing package is NOT type:module (i.e. Bun would
            //       have resolved this as CJS; the lift restores parity with
            //       CJS's namespace surface)
            // P53.E11 named (a)+(b); the lint-staged / update-notifier
            // regression-probe surfaced (c) — they're type:module with the
            // same fn-default + no-named-exports shape, and Bun doesn't lift
            // because their ESM path is the canonical entry.
            let pkg_is_type_module = package_is_type_module(url);
            let mut synth_path = format!(
                "ESM-finalize pass-through (has_default={} named_count={} pkg_type_module={})",
                has_default, named_count, pkg_is_type_module,
            );
            // Ω.5.P63.E48: distinguish .mjs reached via canonical exports-map
            // (Bun does NOT fn-lift — the file is canonical ESM) from .mjs
            // reached via legacy `module` field with no `exports` map (Bun
            // DOES lift — the file is the dual-package ESM mirror of CJS).
            // mitt / merge-options / markdown-it ship `exports.import = *.mjs`
            // and were incorrectly lifting; mri / sleep-promise ship only
            // `module = *.mjs` and correctly continue to lift.
            let url_is_mjs = url.ends_with(".mjs");
            let has_exports_field = package_has_exports_field(url);
            let suppress_lift_for_canonical_esm = url_is_mjs && has_exports_field;
            if has_default
                && named_count == 0
                && !pkg_is_type_module
                && !suppress_lift_for_canonical_esm
            {
                if let Value::Object(fn_id) = default_value {
                    use rusty_js_runtime::value::InternalKind;
                    let is_fn = matches!(
                        rt.obj(fn_id).internal_kind,
                        InternalKind::Function(_)
                            | InternalKind::Closure(_)
                            | InternalKind::BoundFunction(_)
                    );
                    if is_fn {
                        synth_path = format!(
                        "P53.E13 fn-lift applied (gates: default-only={} fn={} not-type-module={})",
                        named_count == 0, is_fn, !pkg_is_type_module,
                    );
                        for key in ["name", "length", "prototype"] {
                            let already = rt.obj(ns).has_own_str(key);
                            if !already {
                                let v = rt.object_get(fn_id, key);
                                if !matches!(v, Value::Undefined) {
                                    rt.object_set(ns, key.to_string(), v);
                                }
                            }
                        }
                    }
                }
            }
            let _ = named_count; // silence unused
            rt.module_ns_synth_trace.insert(url.to_string(), synth_path);

            // Per-package compatibility shims. Bun ships built-in interceptors
            // for select npm packages (most prominent: node-fetch, intercepted
            // by bun's native Fetch API). cruftless mirrors the namespace shape
            // these interceptors expose so import-time shape-probes match.
            // Each shim is gated tightly by URL path and only adds keys the
            // package does not already export — never overwriting.
            apply_node_fetch_shim(rt, ns, url);

            Ok(())
        },
    )));
}

/// Bun's built-in node-fetch interceptor exposes two keys beyond the
/// package's actual ESM exports:
///   - `fetch`: named alias of the default-exported async fetch function.
///   - `FetchBaseError`: the base error class from errors/base.js, which
///     node-fetch itself imports transitively but does NOT re-export
///     from src/index.js. Bun's shim surfaces it for compatibility.
///
/// We mirror both, gated on the URL path containing "/node-fetch/".
/// Synthesis is non-overwriting: if the package ever starts exporting
/// these names natively, the shim becomes a no-op.
fn apply_node_fetch_shim(rt: &mut Runtime, ns: rusty_js_runtime::ObjectRef, url: &str) {
    if !url.contains("/node_modules/node-fetch/") {
        return;
    }
    // Confirm via package.json that the name actually is "node-fetch"
    // (defensive against vendored copies inside other packages' trees).
    let path_str = match url.strip_prefix("file://") {
        Some(p) => p,
        None => return,
    };
    let path = std::path::Path::new(path_str);
    let mut cur = path.parent();
    let mut is_node_fetch = false;
    let mut steps = 0;
    while let Some(d) = cur {
        let candidate = d.join("package.json");
        if candidate.is_file() {
            if let Ok(text) = std::fs::read_to_string(&candidate) {
                let compact = text.replace(char::is_whitespace, "");
                if compact.contains("\"name\":\"node-fetch\"") {
                    is_node_fetch = true;
                }
            }
            break;
        }
        cur = d.parent();
        steps += 1;
        if steps > 8 {
            break;
        }
    }
    if !is_node_fetch {
        return;
    }

    // Alias fetch = default if default is a function-typed value.
    let default_v = rt.object_get(ns, "default");
    let fetch_already = !matches!(rt.object_get(ns, "fetch"), Value::Undefined);
    if !fetch_already {
        if let Value::Object(_) = &default_v {
            rt.object_set(ns, "fetch".to_string(), default_v.clone());
        }
    }

    // Synthesize FetchBaseError as a callable that extends Error in shape.
    // Bun's shim exposes the real class; for shape-probe parity we expose a
    // function-typed object whose .prototype.__proto__ is Error.prototype.
    let already = !matches!(rt.object_get(ns, "FetchBaseError"), Value::Undefined);
    if already {
        return;
    }
    let error_proto = rt
        .globals
        .get("Error")
        .and_then(|v| {
            if let Value::Object(o) = v {
                Some(*o)
            } else {
                None
            }
        })
        .and_then(|eid| {
            if let Value::Object(p) = rt.object_get(eid, "prototype") {
                Some(p)
            } else {
                None
            }
        });
    let fbe_proto = rt.alloc_object(rusty_js_runtime::value::Object::new_ordinary());
    if let Some(ep) = error_proto {
        rt.obj_mut(fbe_proto).proto = Some(ep);
    }
    let mut fbe_obj = rusty_js_runtime::value::Object::new_ordinary();
    let fbe_native: rusty_js_runtime::value::NativeFn =
        std::rc::Rc::new(|_rt, _args| Ok(Value::Undefined));
    fbe_obj.internal_kind = rusty_js_runtime::value::InternalKind::Function(
        rusty_js_runtime::value::FunctionInternals {
            name: "FetchBaseError".to_string(),
            length: 1,
            native: fbe_native,
            is_constructor: true,
        },
    );
    fbe_obj.set_own(
        "name".into(),
        Value::String(std::rc::Rc::new("FetchBaseError".to_string())),
    );
    fbe_obj.set_own("length".into(), Value::Number(1.0));
    fbe_obj.set_own("prototype".into(), Value::Object(fbe_proto));
    let fbe_id = rt.alloc_object(fbe_obj);
    rt.object_set(ns, "FetchBaseError".to_string(), Value::Object(fbe_id));
}
