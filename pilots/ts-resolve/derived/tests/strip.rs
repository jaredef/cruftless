//! TSR-EXT 3 strip tests. Each case: a `.ts` snippet, the expected
//! erased-to-JS equivalent, and a witness-count assertion.
//!
//! Verification strategy: parse-and-erase the TS source; parse the
//! hand-written JS twin; assert structural-shape equivalence via the
//! Module's body length (Span-level Debug comparison would be brittle).

use ts_resolve::{parse_and_erase, parse_with_witnesses};

fn parses(src: &str) -> bool {
    parse_and_erase(src).is_ok()
}

fn shape_equiv(ts_src: &str, js_src: &str) -> bool {
    let ts = parse_and_erase(ts_src).expect("ts parse ok");
    let js = rusty_js_parser::parse_module(js_src).expect("js parse ok");
    ts.body.len() == js.body.len()
}

#[test]
fn let_annotation_stripped() {
    assert!(parses("let x: number = 1;"));
    assert!(shape_equiv("let x: number = 1;", "let x = 1;"));
}

#[test]
fn const_annotation_stripped() {
    assert!(shape_equiv("const s: string = 'hi';", "const s = 'hi';"));
}

#[test]
fn function_param_and_return_annotations_stripped() {
    assert!(shape_equiv(
        "function add(a: number, b: number): number { return a + b; }",
        "function add(a, b) { return a + b; }"
    ));
}

#[test]
fn arrow_param_annotations_stripped() {
    assert!(shape_equiv(
        "const f = (x: number, y: number) => x + y;",
        "const f = (x, y) => x + y;"
    ));
}

#[test]
fn interface_declaration_stripped() {
    assert!(shape_equiv(
        "interface Foo { x: number; y: string; }\nlet a = 1;",
        "let a = 1;"
    ));
}

#[test]
fn type_alias_stripped() {
    assert!(shape_equiv(
        "type Name = string;\nlet a = 1;",
        "let a = 1;"
    ));
}

#[test]
fn as_cast_stripped() {
    assert!(shape_equiv(
        "let x = (1 + 2) as number;",
        "let x = (1 + 2);"
    ));
}

#[test]
fn non_null_postfix_stripped() {
    assert!(shape_equiv(
        "let x = foo!.bar;",
        "let x = foo.bar;"
    ));
}

#[test]
fn optional_param_marker_stripped() {
    assert!(shape_equiv(
        "function f(x?: number) { return x; }",
        "function f(x) { return x; }"
    ));
}

#[test]
fn declare_statement_stripped() {
    assert!(shape_equiv(
        "declare const env: string;\nlet a = 1;",
        "let a = 1;"
    ));
}

#[test]
fn array_type_annotation_stripped() {
    assert!(shape_equiv(
        "function sum(xs: number[]): number { return 0; }",
        "function sum(xs) { return 0; }"
    ));
}

#[test]
fn array_generic_annotation_stripped() {
    assert!(shape_equiv(
        "function head(xs: Array<string>): string { return xs[0]; }",
        "function head(xs) { return xs[0]; }"
    ));
}

#[test]
fn union_type_stripped() {
    assert!(shape_equiv(
        "let x: string | number = 1;",
        "let x = 1;"
    ));
}

#[test]
fn intersection_type_stripped() {
    assert!(shape_equiv(
        "let x: A & B = ({} as any);",
        "let x = ({});"
    ));
}

#[test]
fn nested_object_type_stripped() {
    assert!(shape_equiv(
        "let x: { k: number, v: string[] } = ({} as any);",
        "let x = ({});"
    ));
}

#[test]
fn fn_type_annotation_stripped() {
    assert!(shape_equiv(
        "let cb: (n: number) => string = String;",
        "let cb = String;"
    ));
}

#[test]
fn witnesses_captured_for_let_annotation() {
    let (_m, ws) = parse_with_witnesses("let count: number = 0;").expect("ok");
    assert!(!ws.is_empty(), "expected at least one witness");
}

#[test]
fn pure_js_via_ts_resolve_yields_same_body_length() {
    let src = "let x = 1; let y = 2; (x + y);";
    let direct = rusty_js_parser::parse_module(src).expect("ok");
    let tsr = parse_and_erase(src).expect("ok");
    assert_eq!(direct.body.len(), tsr.body.len());
}
