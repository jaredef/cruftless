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
fn function_decl_head_generics_stripped() {
    assert!(shape_equiv(
        "function id<T>(x: T): T { return x; }",
        "function id(x) { return x; }"
    ));
}

#[test]
fn function_decl_multi_generics_stripped() {
    assert!(shape_equiv(
        "function pair<A, B>(a: A, b: B): [A, B] { return [a, b]; }",
        "function pair(a, b) { return [a, b]; }"
    ));
}

#[test]
fn class_decl_head_generics_stripped() {
    assert!(shape_equiv(
        "class Box<T> { v: T; constructor(v: T) { this.v = v; } }",
        "class Box { v; constructor(v) { this.v = v; } }"
    ));
}

#[test]
fn template_literal_with_substitution_then_tail_text_lexes() {
    // TRSLS-EXT 1 regression: template-substitution goal switching.
    // Pre-fix: the `}` closing ${b} would be lexed as plain Punct
    // under LexerGoal::Div, then the `"` and `` ` `` after would
    // mis-lex (UnterminatedString reported elsewhere).
    let src = r#"function f(a, b) { return `${a.slice(0, -1)}${b}"`; }"#;
    assert!(parses(src), "template with multiple substitutions + literal text should parse");
}

#[test]
fn template_literal_no_substitution_unchanged() {
    assert!(parses("const x = `simple template`;"));
}

#[test]
fn template_literal_single_substitution() {
    assert!(parses("const x = `pre${y}post`;"));
}

#[test]
fn template_literal_nested_braces_inside_substitution() {
    assert!(parses("const x = `${(() => { return y; })()}`;"));
}

#[test]
fn destructured_object_param_annotation_stripped() {
    assert!(shape_equiv(
        "function f({a, b}: T): void { return; }",
        "function f({a, b}) { return; }"
    ));
}

#[test]
fn destructured_array_param_annotation_stripped() {
    assert!(shape_equiv(
        "function f([x, y]: [number, number]): void { return; }",
        "function f([x, y]) { return; }"
    ));
}

#[test]
fn extends_clause_generics_stripped() {
    assert!(shape_equiv(
        "class Sub<T> extends Base<T> { }",
        "class Sub extends Base { }"
    ));
}

#[test]
fn implements_clause_stripped() {
    assert!(shape_equiv(
        "class C extends Base implements I, J { x = 1; }",
        "class C extends Base { x = 1; }"
    ));
}

#[test]
fn implements_clause_with_generics_stripped() {
    assert!(shape_equiv(
        "class C<T> extends B<T> implements I<T>, J { x = 1; }",
        "class C extends B { x = 1; }"
    ));
}

#[test]
fn class_member_modifiers_stripped() {
    assert!(shape_equiv(
        "class C { public x = 1; private y = 2; readonly z = 3; protected w = 4; }",
        "class C { x = 1; y = 2; z = 3; w = 4; }"
    ));
}

#[test]
fn class_member_modifiers_with_annotations_stripped() {
    assert!(shape_equiv(
        "class C { public x: number = 1; readonly y: string = 'a'; }",
        "class C { x = 1; y = 'a'; }"
    ));
}

#[test]
fn regex_literal_at_var_init_position_parses() {
    // TRCAPS-EXT 1 regex-goal regression: /pat/ at expression-start
    // (after `=`) must lex as regex, not division.
    assert!(parses("const TRAILING_SLASH_HASH = /#\\/?$/;"));
}

#[test]
fn regex_literal_at_call_arg_position_parses() {
    assert!(parses("s.match(/foo/g);"));
}

#[test]
fn division_after_ident_still_works() {
    // Negative: a `/` after an Ident (expression-terminator) is
    // division, NOT a regex. Verify we didn't break basic arithmetic.
    assert!(parses("const x = a / b;"));
}

#[test]
fn generic_arrow_with_single_type_param_stripped() {
    assert!(shape_equiv(
        "const f = <T>(x: T): T => x;",
        "const f = (x) => x;"
    ));
}

#[test]
fn generic_arrow_assigned_to_class_static_field_stripped() {
    assert!(shape_equiv(
        "class C { static create = <T>(s?: (x: T) => void) => { return new C(); }; }",
        "class C { static create = (s) => { return new C(); }; }"
    ));
}

#[test]
fn generic_instantiation_in_new_stripped() {
    assert!(shape_equiv(
        "const o = new Observable<T>(subscribe);",
        "const o = new Observable(subscribe);"
    ));
}

#[test]
fn generic_method_decl_in_class_body_stripped() {
    assert!(shape_equiv(
        "class C { lift<R>(op?: Op<T, R>): Observable<R> { return this; } }",
        "class C { lift(op) { return this; } }"
    ));
}

#[test]
fn generic_function_call_stripped() {
    assert!(shape_equiv(
        "const x = parse<MyType>(input);",
        "const x = parse(input);"
    ));
}

#[test]
fn less_than_operator_not_mis_stripped() {
    // Negative regression: `a < b` is comparison, NOT generic. Verify
    // we don't strip a real `<` operator.
    assert!(shape_equiv(
        "const r = (a < b) && (c < d);",
        "const r = (a < b) && (c < d);"
    ));
}

#[test]
fn arrow_return_type_annotation_does_not_eat_fat_arrow() {
    // The `: Writable => {...}` shape: type ends at `Writable`; the
    // `=>` belongs to the arrow function, not the type. Regression
    // for the post-TRGC follow-on substrate fix.
    assert!(shape_equiv(
        "export default (): Writable => { return 1; };",
        "export default () => { return 1; };"
    ));
}

#[test]
fn fn_type_annotation_still_consumes_arrow() {
    // The `: (x: T) => U` shape: type IS the fn-type; arrow + return
    // type belong to the type. Both must be stripped together.
    assert!(shape_equiv(
        "let cb: (n: number) => string = String;",
        "let cb = String;"
    ));
}

#[test]
fn class_method_overload_signature_stripped() {
    assert!(shape_equiv(
        "class C { method(x: A): R; method(x: B): R; method(x: A | B): R { return x as R; } }",
        "class C { method(x) { return x; } }"
    ));
}

#[test]
fn class_field_annotation_no_init_stops_at_newline() {
    assert!(shape_equiv(
        "class C {\n  readonly str: string\n  constructor(s: string) { this.str = s; }\n}",
        "class C {\n  str\n  constructor(s) { this.str = s; }\n}"
    ));
}

#[test]
fn regex_call_arg_not_mis_treated_as_overload() {
    // Negative: `s.match(/foo/g);` is not an overload — the `match`
    // Ident is at expression position (after `.`), not class-body
    // member-start.
    assert!(parses("const r = s.match(/foo/g);"));
}

#[test]
fn plain_enum_stripped() {
    assert!(shape_equiv(
        "enum K { A, B, C }\nlet a = 1;",
        "let a = 1;"
    ));
}

#[test]
fn export_enum_stripped() {
    assert!(shape_equiv(
        "export enum K { A = 'a', B = 'b' }\nlet a = 1;",
        "let a = 1;"
    ));
}

#[test]
fn const_enum_stripped() {
    assert!(shape_equiv(
        "const enum K { A, B }\nlet a = 1;",
        "let a = 1;"
    ));
}

#[test]
fn export_const_enum_stripped() {
    assert!(shape_equiv(
        "export const enum K { A, B }\nlet a = 1;",
        "let a = 1;"
    ));
}

#[test]
fn declare_enum_stripped() {
    assert!(shape_equiv(
        "declare enum K { A, B }\nlet a = 1;",
        "let a = 1;"
    ));
}

#[test]
fn pure_js_via_ts_resolve_yields_same_body_length() {
    let src = "let x = 1; let y = 2; (x + y);";
    let direct = rusty_js_parser::parse_module(src).expect("ok");
    let tsr = parse_and_erase(src).expect("ok");
    assert_eq!(direct.body.len(), tsr.body.len());
}
