//! Ω.5.P18/P43 module namespace synthesis coverage for the
//! current HostFinalizeModuleNamespace install.

use cruftless::module_ns;
use rusty_js_runtime::{Runtime, Value};
use std::fs;
use std::path::PathBuf;

fn new_rt() -> Runtime {
    let mut rt = Runtime::new();
    rt.install_intrinsics();
    module_ns::install(&mut rt);
    rt
}

fn package_url(file_name: &str, package_json: &str) -> String {
    let dir = std::env::temp_dir().join(format!(
        "cruftless-module-ns-{}-{}",
        std::process::id(),
        file_name.replace('.', "-"),
    ));
    fs::create_dir_all(&dir).expect("create module namespace fixture dir");
    fs::write(dir.join("package.json"), package_json).expect("write package.json fixture");
    let file: PathBuf = dir.join(file_name);
    format!("file://{}", file.display())
}

#[test]
fn pure_esm_named_exports_do_not_synthesize_default() {
    let mut rt = new_rt();
    let src = "const a = 1; const b = 2; export { a, b };";
    let ns = rt.evaluate_module(src, "file:///tmp/cruftless-pure-esm.mjs").expect("evaluate");
    let o = rt.obj(ns);
    assert!(o.has_own_str("a"));
    assert!(o.has_own_str("b"));
    assert!(matches!(rt.object_get(ns, "default"), Value::Undefined));
}

#[test]
fn module_field_js_named_exports_synthesize_default_namespace() {
    let mut rt = new_rt();
    let src = "const a = 1; const b = 2; export { a, b };";
    let url = package_url("mod.js", r#"{"name":"fixture"}"#);
    let ns = rt.evaluate_module(src, &url).expect("evaluate");
    let o = rt.obj(ns);
    assert!(o.has_own_str("a"));
    assert!(o.has_own_str("b"));
    match rt.object_get(ns, "default") {
        Value::Object(id) => assert_eq!(id, ns, "default points at namespace itself"),
        other => panic!("expected default to be namespace object, got {:?}", other),
    }
}

#[test]
fn default_object_does_not_spread_named_exports_on_esm_path() {
    let mut rt = new_rt();
    let src = "export default { x: 1, y: 2 };";
    let ns = rt.evaluate_module(src, "file:///tmp/cruftless-default-object.mjs").expect("evaluate");
    let o = rt.obj(ns);
    assert!(o.has_own_str("default"), "default still present");
    assert!(!o.has_own_str("x"), "Tuple B spread is intentionally dropped");
    assert!(!o.has_own_str("y"), "Tuple B spread is intentionally dropped");
}

#[test]
fn does_not_shadow_when_both_default_and_named_exist() {
    let mut rt = new_rt();
    let src = r#"
        const a = 10;
        export { a };
        export default { a: 999, z: 7 };
    "#;
    let ns = rt.evaluate_module(src, "test-c").expect("evaluate");
    // Named export 'a' must be preserved (10, not 999 from default).
    match rt.object_get(ns, "a") {
        Value::Number(n) => assert_eq!(n, 10.0, "named 'a' preserved, not shadowed by default.a"),
        other => panic!("expected named 'a' = 10, got {:?}", other),
    }
    // default still present unchanged.
    assert!(matches!(rt.object_get(ns, "default"), Value::Object(_)));
    // Tuple B branch must NOT fire (named exports already exist), so 'z'
    // from default should NOT have been spread.
    let o = rt.obj(ns);
    assert!(!o.has_own_str("z"),
        "Tuple B suppressed when named exports already present");
}
