//! CMLD Stage-L — derived static CJS export-name scanner (cjs-module-lexer equivalent).
//!
//! See `pilots/cjs-esm-namespace-pipeline/cjs-module-lexer-derived/{seed,design}.md`.
//!
//! CMLD-EXT 1 scope: detection rungs **R1** (direct member assignment:
//! `exports.NAME =`, `module.exports.NAME =`) and **R2** (object-literal:
//! `module.exports = { NAME: … }`), plus `__esModule` flag detection. Later
//! rungs add R3 (defineProperty names), R4 (transpiler prologues), R5 (reexport
//! stars), R6 (dynamic-guard hardening).
//!
//! This is a PURE function: source text in → static export-name set out. It has
//! no `Runtime` access and does not execute the module. It is currently UNUSED
//! by the engine; CENP-EXT 1 wires it into `module::populate_cjs_namespace_view_at`.
//! Fidelity is validated by the unit tests below against the Node named-export
//! view (the cohorts named in the CMLD seed: debug/semver detect, lodash/ms empty).

use rusty_js_ast::{
    AssignOp, DefaultExportBody, Expr, ExportDeclaration, MemberProperty, ModuleItem, ObjectKey,
    ObjectProperty, Stmt,
};

/// The static export set Node's cjs-module-lexer would attribute to a CJS source.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct CjsExportSet {
    /// Statically-detected named exports, in first-seen order.
    pub names: Vec<String>,
    /// Source declared itself a transpiled ES module (`__esModule`).
    pub esmodule: bool,
}

/// Scan CJS `source` for statically-detectable named exports (R1 + R2).
pub fn cjs_lex(source: &str) -> CjsExportSet {
    let mut set = CjsExportSet::default();
    let body = match extract_cjs_body(source) {
        Some(b) => b,
        None => return set,
    };
    let mut seen = std::collections::HashSet::new();
    walk_stmts(&body, &mut set.names, &mut seen, &mut set.esmodule);
    set
}

/// Wrap the source exactly as `module::evaluate_cjs_module` does, parse it, and
/// return the synthesized wrapper function's body statements. Returns None if the
/// source does not parse (the engine surfaces the parse error elsewhere).
fn extract_cjs_body(source: &str) -> Option<Vec<Stmt>> {
    let src = if source.starts_with("#!") {
        match source.find('\n') {
            Some(nl) => &source[nl + 1..],
            None => "",
        }
    } else {
        source
    };
    let wrapped = format!(
        "export default (function (exports, module, require, __filename, __dirname) {{\n{}\n}});\n",
        src
    );
    let module = rusty_js_parser::parse_module(&wrapped).ok()?;
    for item in &module.body {
        if let ModuleItem::Export(ExportDeclaration::Default { body, .. }) = item {
            if let DefaultExportBody::Expression { expr } = body {
                if let Some(b) = function_body(expr) {
                    return Some(b);
                }
            }
        }
    }
    None
}

fn function_body(expr: &Expr) -> Option<Vec<Stmt>> {
    match expr {
        Expr::Parenthesized { expr, .. } => function_body(expr),
        Expr::Function { body, .. } => Some(body.clone()),
        _ => None,
    }
}

fn unparen(e: &Expr) -> &Expr {
    if let Expr::Parenthesized { expr, .. } = e {
        unparen(expr)
    } else {
        e
    }
}

/// Walk a statement slice, descending into the control-flow nesting where CJS
/// export assignments commonly appear (blocks + if-branches). It does NOT descend
/// into nested function bodies: a name assigned inside an inner function is not a
/// statically-detectable top-level export (matches Node's lexer shallow scope).
fn walk_stmts(
    stmts: &[Stmt],
    names: &mut Vec<String>,
    seen: &mut std::collections::HashSet<String>,
    esmod: &mut bool,
) {
    for s in stmts {
        walk_stmt(s, names, seen, esmod);
    }
}

fn walk_stmt(
    s: &Stmt,
    names: &mut Vec<String>,
    seen: &mut std::collections::HashSet<String>,
    esmod: &mut bool,
) {
    match s {
        Stmt::Expression { expr, .. } => handle_expr(expr, names, seen, esmod),
        Stmt::Block { body, .. } => walk_stmts(body, names, seen, esmod),
        Stmt::If {
            consequent,
            alternate,
            ..
        } => {
            walk_stmt(consequent, names, seen, esmod);
            if let Some(a) = alternate {
                walk_stmt(a, names, seen, esmod);
            }
        }
        _ => {}
    }
}

fn handle_expr(
    expr: &Expr,
    names: &mut Vec<String>,
    seen: &mut std::collections::HashSet<String>,
    esmod: &mut bool,
) {
    match unparen(expr) {
        // `a = b, c = d` — multiple assignments in one statement.
        Expr::Sequence { expressions, .. } => {
            for e in expressions {
                handle_expr(e, names, seen, esmod);
            }
        }
        Expr::Assign {
            operator: AssignOp::Assign,
            target,
            value,
            ..
        } => {
            let target = unparen(target);
            // R2: whole-exports reassignment to an object literal →
            // each literal member name is a static export.
            if is_exports_root(target) {
                if let Expr::Object { properties, .. } = unparen(value) {
                    for p in properties {
                        if let ObjectProperty::Property { key, .. } = p {
                            if let Some(n) = object_key_name(key) {
                                push(names, seen, n);
                            }
                        }
                    }
                }
                return;
            }
            // R1: `exports.NAME = …` / `module.exports.NAME = …`.
            if let Some(name) = exports_member_name(target) {
                if name == "__esModule" {
                    *esmod = true;
                    return;
                }
                push(names, seen, name);
            }
        }
        // `Object.defineProperty(exports, "__esModule", { value: true })`:
        // detect the interop flag only (general R3 names are a later rung).
        Expr::Call {
            callee, arguments, ..
        } => {
            if is_object_define_property(callee) {
                if let Some(rusty_js_ast::Argument::Expr(arg0)) = arguments.first() {
                    if is_exports_root(unparen(arg0)) {
                        if let Some(rusty_js_ast::Argument::Expr(arg1)) = arguments.get(1) {
                            if let Expr::StringLiteral { value, .. } = unparen(arg1) {
                                if value == "__esModule" {
                                    *esmod = true;
                                }
                            }
                        }
                    }
                }
            }
        }
        _ => {}
    }
}

/// `exports` or `module.exports`.
fn is_exports_root(e: &Expr) -> bool {
    match unparen(e) {
        Expr::Identifier { name, .. } => name == "exports",
        Expr::Member {
            object, property, ..
        } => {
            matches!(unparen(object), Expr::Identifier { name, .. } if name == "module")
                && member_prop_name(property).as_deref() == Some("exports")
        }
        _ => false,
    }
}

/// `exports.NAME` or `module.exports.NAME` → Some(NAME).
fn exports_member_name(target: &Expr) -> Option<String> {
    if let Expr::Member {
        object, property, ..
    } = target
    {
        let name = member_prop_name(property)?;
        if is_exports_root(object) {
            return Some(name);
        }
    }
    None
}

fn member_prop_name(p: &MemberProperty) -> Option<String> {
    match p {
        MemberProperty::Identifier { name, .. } => Some(name.clone()),
        MemberProperty::Computed { expr, .. } => match unparen(expr) {
            Expr::StringLiteral { value, .. } => Some(value.clone()),
            _ => None,
        },
        MemberProperty::Private { .. } => None,
    }
}

fn object_key_name(k: &ObjectKey) -> Option<String> {
    match k {
        ObjectKey::Identifier { name, .. } => Some(name.clone()),
        ObjectKey::String { value, .. } => Some(value.clone()),
        // Numeric and computed keys are not statically-named exports here.
        ObjectKey::Number { .. } | ObjectKey::Computed { .. } => None,
    }
}

fn is_object_define_property(callee: &Expr) -> bool {
    if let Expr::Member {
        object, property, ..
    } = unparen(callee)
    {
        return matches!(unparen(object), Expr::Identifier { name, .. } if name == "Object")
            && member_prop_name(property).as_deref() == Some("defineProperty");
    }
    false
}

fn push(names: &mut Vec<String>, seen: &mut std::collections::HashSet<String>, name: String) {
    if seen.insert(name.clone()) {
        names.push(name);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn set(src: &str) -> (Vec<String>, bool) {
        let s = cjs_lex(src);
        let mut names = s.names.clone();
        names.sort();
        (names, s.esmodule)
    }

    #[test]
    fn r1_member_assignment() {
        // debug-class: exports.X = … and module.exports.Y = …
        let (n, esm) = set("exports.foo = 1;\nmodule.exports.bar = function(){};\n");
        assert_eq!(n, vec!["bar".to_string(), "foo".to_string()]);
        assert!(!esm);
    }

    #[test]
    fn r1_computed_string_key() {
        let (n, _) = set("exports['baz'] = 1; module.exports[\"qux\"] = 2;");
        assert_eq!(n, vec!["baz".to_string(), "qux".to_string()]);
    }

    #[test]
    fn r2_object_literal() {
        // semver-class: module.exports = { … } literal members.
        let (n, _) = set("module.exports = { SemVer: 1, parse: function(){}, 'clean': 3 };");
        assert_eq!(
            n,
            vec!["SemVer".to_string(), "clean".to_string(), "parse".to_string()]
        );
    }

    #[test]
    fn dynamic_assignment_is_empty() {
        // lodash-class: members assigned to a local, not to exports → no static names.
        let (n, _) = set("var f = {}; f.map = 1; f.filter = 2; module.exports = f;");
        assert!(n.is_empty(), "expected empty, got {:?}", n);
    }

    #[test]
    fn function_export_is_empty() {
        // ms-class: module.exports = function(){} → no named exports.
        let (n, _) = set("module.exports = function(val){ return val; };");
        assert!(n.is_empty(), "expected empty, got {:?}", n);
    }

    #[test]
    fn esmodule_flag_and_default_name() {
        // transpiled: __esModule flag set, plus default + named.
        let (n, esm) = set(
            "Object.defineProperty(exports, \"__esModule\", { value: true });\nexports.default = x;\nexports.thing = y;\n",
        );
        assert!(esm);
        assert_eq!(n, vec!["default".to_string(), "thing".to_string()]);
    }

    #[test]
    fn esmodule_via_assignment() {
        let (n, esm) = set("exports.__esModule = true; exports.only = 1;");
        assert!(esm);
        assert_eq!(n, vec!["only".to_string()]);
    }

    #[test]
    fn names_inside_if_block() {
        let (n, _) = set("if (true) { exports.a = 1; } else { exports.b = 2; }");
        assert_eq!(n, vec!["a".to_string(), "b".to_string()]);
    }

    #[test]
    fn nested_function_body_not_scanned() {
        // An assignment inside an inner function is not a static top-level export.
        let (n, _) = set("function f(){ exports.hidden = 1; }\nexports.real = 2;");
        assert_eq!(n, vec!["real".to_string()]);
    }

    #[test]
    fn unparseable_source_is_empty() {
        let (n, esm) = set("this is not ){ valid javascript");
        assert!(n.is_empty());
        assert!(!esm);
    }
}
