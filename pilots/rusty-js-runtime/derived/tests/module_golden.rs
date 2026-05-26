//! Module evaluation tests per Doc 717 §IX + design spec §VI–§VII.

use rusty_js_runtime::{HostHook, Runtime, Value};

#[test]
fn evaluate_empty_module() {
    let mut rt = Runtime::new();
    rt.install_intrinsics();
    let ns = rt.evaluate_module("", "test").expect("evaluate failed");
    assert!(rt.obj(ns).properties.is_empty());
}

#[test]
fn named_export_populates_namespace() {
    let mut rt = Runtime::new();
    rt.install_intrinsics();
    let src = r#"
        const greeting = 'hello';
        export { greeting };
    "#;
    let ns = rt.evaluate_module(src, "test").expect("evaluate failed");
    match rt.object_get(ns, "greeting") {
        Value::String(s) => assert_eq!(s.as_str(), "hello"),
        other => panic!("expected string, got {:?}", other),
    }
}

#[test]
fn multiple_named_exports() {
    let mut rt = Runtime::new();
    rt.install_intrinsics();
    let src = r#"
        const a = 1;
        const b = 2;
        const c = 3;
        export { a, b, c };
    "#;
    let ns = rt.evaluate_module(src, "test").expect("evaluate failed");
    for (name, expected) in &[("a", 1.0), ("b", 2.0), ("c", 3.0)] {
        match rt.object_get(ns, name) {
            Value::Number(n) => assert_eq!(n, *expected),
            other => panic!("{} expected number, got {:?}", name, other),
        }
    }
}

#[test]
fn rename_export() {
    let mut rt = Runtime::new();
    rt.install_intrinsics();
    let src = r#"
        const internal = 42;
        export { internal as exposed };
    "#;
    let ns = rt.evaluate_module(src, "test").expect("evaluate failed");
    let obj = rt.obj(ns);
    assert!(obj.has_own_str("exposed"));
    assert!(!obj.has_own_str("internal"));
}

// ─────────── Doc 717 Tuple-A closure via host hook ───────────

#[test]
fn host_hook_synthesizes_default_as_namespace() {
    let mut rt = Runtime::new();
    rt.install_intrinsics();
    rt.install_host_hook(HostHook::FinalizeModuleNamespace(Box::new(
        |rt, _ast, ns, _url| {
            let has_default = rt.obj(ns).has_own_str("default");
            if !has_default {
                rt.object_set(
                    ns,
                    "default".into(),
                    rusty_js_runtime::Value::String(std::rc::Rc::new(
                        "<synthesized-default>".to_string(),
                    )),
                );
            }
            Ok(())
        },
    )));
    let src = "const x = 1; export { x };";
    let ns = rt.evaluate_module(src, "test").expect("evaluate failed");
    let obj = rt.obj(ns);
    assert!(obj.has_own_str("x"));
    assert!(
        obj.has_own_str("default"),
        "Tuple-A closure: default synthesized by host hook"
    );
}

#[test]
fn host_hook_does_not_run_without_install() {
    let mut rt = Runtime::new();
    rt.install_intrinsics();
    let src = "const x = 1; export { x };";
    let ns = rt.evaluate_module(src, "test").expect("evaluate failed");
    let obj = rt.obj(ns);
    assert!(obj.has_own_str("x"));
    assert!(!obj.has_own_str("default"));
}

// ─────────── Doc 717 Tuple-B closure via host hook ───────────

#[test]
fn host_hook_synthesizes_named_exports_from_default() {
    let mut rt = Runtime::new();
    rt.install_intrinsics();
    rt.install_host_hook(HostHook::FinalizeModuleNamespace(Box::new(
        |rt, _ast, ns, _url| {
            let keys_to_spread: Vec<(String, rusty_js_runtime::Value)> = {
                let b = rt.obj(ns);
                if let Some(d) = b.get_own("__default_obj_props") {
                    if let rusty_js_runtime::Value::Object(id) = &d.value {
                        rt.obj(*id)
                            .string_keys()
                            .map(|k| (k.to_string(), rt.object_get(*id, k)))
                            .collect()
                    } else {
                        Vec::new()
                    }
                } else {
                    Vec::new()
                }
            };
            for (k, v) in keys_to_spread {
                if !rt.obj(ns).has_own_str(&k) {
                    rt.object_set(ns, k, v);
                }
            }
            Ok(())
        },
    )));
    let src = r#"
        const __default_obj_props = { Ls: 1, en: 2, extend: 3 };
        export { __default_obj_props };
    "#;
    let ns = rt.evaluate_module(src, "test").expect("evaluate failed");
    let obj = rt.obj(ns);
    assert!(
        obj.has_own_str("Ls"),
        "Tuple-B: 'Ls' spread from default's own props by host hook"
    );
    assert!(obj.has_own_str("en"));
    assert!(obj.has_own_str("extend"));
}
