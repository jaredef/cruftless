//! Full-suite Test262 Pin-Art apparatus.
//!
//! Consumes a full upstream test262 run from the sidecar and projects each
//! non-passing result onto engine coordinates:
//!
//!   observed result -> structure surface -> resolver instance -> engine tier
//!     -> engine rung -> availability/cut kind -> failure mode
//!
//! The output is an apparatus interpretation, not a replacement for the raw
//! benchmark artifact. Raw JSONL stays in the sidecar; derived matrices land in
//! `pilots/apparatus/test262-categorize/full-suite/results/<run-id>/`.

use serde_json::Value;
use std::collections::{BTreeMap, HashMap};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
struct Record {
    path: String,
    status: String,
    reason: String,
}

#[derive(Debug, Clone)]
struct Coord {
    rel: String,
    status: String,
    reason: String,
    surface: String,
    resolver: String,
    tier: String,
    rung: String,
    axis: String,
    availability: String,
    cut_kind: String,
    projection: String,
    failure_mode: String,
    abstract_op: String,
    feature_shape: String,
    pin: String,
}

fn main() {
    let arg = std::env::args()
        .nth(1)
        .map(PathBuf::from)
        .or_else(find_latest_full);
    let results_path = match arg {
        Some(path) => path,
        None => {
            eprintln!("usage: t262-full-pinart <sidecar-results.jsonl>");
            std::process::exit(2);
        }
    };
    if !results_path.exists() {
        eprintln!("not found: {}", results_path.display());
        std::process::exit(2);
    }

    let repo_root = repo_root();
    let locale_dir = repo_root.join("pilots/apparatus/test262-categorize/full-suite");
    let run_id = results_path
        .parent()
        .and_then(|p| p.file_name())
        .and_then(|s| s.to_str())
        .unwrap_or("unknown-run")
        .to_string();
    let out_dir = locale_dir.join("results").join(&run_id);
    fs::create_dir_all(&out_dir).expect("mkdir output dir");

    let (records, malformed_fragments) = read_records(&results_path);
    let mut totals: BTreeMap<String, usize> = BTreeMap::new();
    let mut coords = Vec::new();
    for record in records {
        *totals.entry(record.status.clone()).or_insert(0) += 1;
        if record.status != "PASS" && record.status != "SKIP" {
            coords.push(classify(&record));
        }
    }

    write_outputs(
        &out_dir,
        &results_path,
        &run_id,
        &totals,
        &coords,
        malformed_fragments,
    );
    println!("wrote {}", out_dir.display());
    println!("{}", out_dir.join("summary.md").display());
}

fn read_records(path: &Path) -> (Vec<Record>, usize) {
    let text = fs::read_to_string(path).expect("read results.jsonl");
    let mut records = Vec::new();
    let mut malformed = 0usize;
    let mut buf = String::new();

    for line in text.lines() {
        let trimmed = line.trim_end();
        if trimmed.is_empty() {
            continue;
        }
        if buf.is_empty() {
            buf.push_str(trimmed);
        } else {
            buf.push('\n');
            buf.push_str(trimmed);
        }

        match serde_json::from_str::<Value>(&buf) {
            Ok(v) => {
                records.push(value_to_record(v));
                buf.clear();
            }
            Err(_) => {
                // A real split record usually becomes valid after one or more
                // following physical lines. If a new record begins before the
                // old buffer parsed, preserve the old fragment as malformed.
                if trimmed.starts_with("{\"path\":") && !buf.starts_with(trimmed) {
                    malformed += 1;
                    buf.clear();
                    buf.push_str(trimmed);
                }
            }
        }
    }
    if !buf.trim().is_empty() {
        malformed += 1;
    }
    (records, malformed)
}

fn value_to_record(v: Value) -> Record {
    Record {
        path: v
            .get("path")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string(),
        status: v
            .get("status")
            .and_then(|v| v.as_str())
            .unwrap_or("MALFORMED")
            .to_string(),
        reason: v
            .get("reason")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string(),
    }
}

fn classify(record: &Record) -> Coord {
    let rel = rel_path(&record.path);
    let surface = surface_axis(&rel);
    let resolver = resolver_instance(&rel, &surface);
    let tier = engine_tier(&resolver, &surface);
    let rung = engine_rung(&rel, &resolver, &surface);
    let axis = constraint_axis(&rel, &surface, &resolver);
    let availability = availability_axis(&rel, &record.reason, &surface);
    let cut_kind = cut_kind(&record.status, &record.reason, &availability);
    let projection = projection_axis(&rel, &record.reason, &surface);
    let failure_mode = failure_mode(&record.status, &record.reason);
    let abstract_op = abstract_op_candidate(&rel, &surface, &record.reason);
    let feature_shape = feature_shape(&record.path, &record.reason);
    let pin = format!(
        "{} :: {} :: {} :: {}",
        resolver, rung, projection, failure_mode
    );
    Coord {
        rel,
        status: record.status.clone(),
        reason: record.reason.clone(),
        surface,
        resolver,
        tier,
        rung,
        axis,
        availability,
        cut_kind,
        projection,
        failure_mode,
        abstract_op,
        feature_shape,
        pin,
    }
}

fn rel_path(path: &str) -> String {
    path.strip_prefix("/home/jaredef/test262/test/")
        .unwrap_or(path)
        .to_string()
}

fn surface_axis(rel: &str) -> String {
    let parts: Vec<&str> = rel.split('/').collect();
    match parts.as_slice() {
        ["built-ins", obj, "prototype", method, ..] => format!("{}.prototype.{}", obj, method),
        ["built-ins", obj, method, ..] if !method.ends_with(".js") => format!("{}.{}", obj, method),
        ["built-ins", obj, ..] => obj.to_string(),
        ["language", "expressions", item, ..] => format!("language.expressions.{}", item),
        ["language", "statements", item, ..] => format!("language.statements.{}", item),
        ["language", "declarations", item, ..] => format!("language.declarations.{}", item),
        ["language", "module-code", ..] => "language.module-code".into(),
        ["language", item, ..] => format!("language.{}", item),
        ["intl402", rest @ ..] => format!("intl402.{}", rest.first().unwrap_or(&"root")),
        ["annexB", rest @ ..] => format!("annexB.{}", rest.first().unwrap_or(&"root")),
        ["staging", rest @ ..] => format!("staging.{}", rest.first().unwrap_or(&"root")),
        _ => format!("other.{}", parts.first().unwrap_or(&"unknown")),
    }
}

fn resolver_instance(rel: &str, surface: &str) -> String {
    if rel.starts_with("language/") {
        if rel.contains("/early/") || rel.contains("invalid") || rel.contains("syntax") {
            "source-to-ast/parser-early-error".into()
        } else {
            "ast-to-bytecode/language-lowering".into()
        }
    } else if surface.starts_with("intl402.") {
        "host-intrinsic/intl402".into()
    } else if surface.starts_with("Atomics") || surface.contains("SharedArrayBuffer") {
        "runtime/agent-memory".into()
    } else if surface.starts_with("Promise") {
        "runtime/job-queue-promise".into()
    } else if surface.starts_with("RegExp") {
        "runtime/regexp".into()
    } else if surface.starts_with("ArrayBuffer")
        || surface.starts_with("TypedArray")
        || surface.contains("ArrayBuffer")
        || surface.starts_with("DataView")
        || surface.starts_with("Float")
        || surface.starts_with("Int")
        || surface.starts_with("Uint")
        || surface.starts_with("BigInt")
    {
        "runtime/buffer-typed-array".into()
    } else if surface.starts_with("Map")
        || surface.starts_with("Set")
        || surface.starts_with("WeakMap")
        || surface.starts_with("WeakSet")
    {
        "runtime/collection-intrinsics".into()
    } else if surface.starts_with("Object")
        || surface.starts_with("Reflect")
        || surface.starts_with("Proxy")
    {
        "runtime/object-internals".into()
    } else if surface.starts_with("Array") {
        "runtime/array-exotic".into()
    } else if rel.starts_with("built-ins/") {
        "runtime/spec-builtins".into()
    // PCR-EXT 1: annexB tests are legacy web-compat web-browser intrinsics
    // (per ECMA-262 Annex B). They were routed to uncategorized/resolver
    // because they live under `annexB/` not `built-ins/`. Re-route to the
    // appropriate resolver by walking the inner surface.
    } else if rel.starts_with("annexB/built-ins/") {
        let inner = &rel["annexB/built-ins/".len()..];
        if inner.starts_with("Date/")
            || inner.starts_with("String/")
            || inner.starts_with("Object/")
            || inner.starts_with("Function/")
            || inner.starts_with("RegExp/")
        {
            // Annex B legacy methods on these intrinsics.
            "runtime/spec-builtins".into()
        } else if inner.starts_with("escape/")
            || inner.starts_with("unescape/")
            || inner.starts_with("global/")
        {
            "runtime/spec-builtins".into()
        } else {
            "runtime/spec-builtins".into()
        }
    // PCR-EXT 1: annexB/language/ tests are the Annex B web-compat language
    // tweaks (B.3.* sections). Route to the language-tier resolver.
    } else if rel.starts_with("annexB/language/") {
        "ast-to-bytecode/language-lowering".into()
    // PCR-EXT 1: staging/ tests are stage-3 proposals not yet in core spec.
    // The resolver depends on the proposal; default to spec-builtins for
    // intrinsic-adding proposals, ast-to-bytecode for syntax proposals.
    } else if rel.starts_with("staging/") {
        if rel.contains("/Iterator/") || rel.contains("/AsyncIterator/") {
            "runtime/spec-builtins".into()
        } else if rel.contains("/language/") {
            "ast-to-bytecode/language-lowering".into()
        } else {
            "runtime/spec-builtins".into()
        }
    } else {
        "uncategorized/resolver".into()
    }
}

fn engine_tier(resolver: &str, surface: &str) -> String {
    if resolver.starts_with("source-to-ast") {
        "parser".into()
    } else if resolver.starts_with("ast-to-bytecode") {
        "bytecode".into()
    } else if resolver.contains("intl") {
        "host".into()
    } else if surface.contains("Promise") {
        "runtime+job-queue".into()
    } else {
        "runtime".into()
    }
}

fn engine_rung(rel: &str, resolver: &str, surface: &str) -> String {
    if resolver.starts_with("source-to-ast") {
        "E1/algorithm-step:syntactic-grammar".into()
    } else if resolver.starts_with("ast-to-bytecode") {
        "E2/internal-method:execution-semantics".into()
    } else if surface.starts_with("Intl") || surface.starts_with("intl402.") {
        "E3/intrinsic-object:ecma-402".into()
    } else if surface.starts_with("Temporal") {
        "E3/intrinsic-object:temporal".into()
    } else if surface.starts_with("Proxy")
        || surface.starts_with("Reflect")
        || surface.contains("getOwnProperty")
        || surface.contains("defineProperty")
    {
        "E2/internal-method:object-internals".into()
    } else if rel.contains("/realm/")
        || rel.contains("cross-realm")
        || rel.contains("proto-from-ctor")
    {
        "E5/realm".into()
    } else if surface.starts_with("Promise") || rel.contains("async") {
        "E4/execution-context:jobs".into()
    } else if rel.starts_with("built-ins/") {
        "E3/intrinsic-object:ecma-262".into()
    } else {
        "E2/internal-method:runtime".into()
    }
}

fn constraint_axis(rel: &str, surface: &str, resolver: &str) -> String {
    if resolver.starts_with("source-to-ast") {
        "R/parser-form".into()
    } else if resolver.starts_with("ast-to-bytecode") {
        "R/ast-to-bytecode".into()
    } else if surface.starts_with("Intl") || surface.starts_with("intl402.") {
        "H/host-builtins-ecma402".into()
    } else if surface.starts_with("Temporal") {
        "E/eval-runtime-semantics:temporal-chapter".into()
    } else if surface.contains("Symbol") || rel.contains("Symbol.") || rel.contains("@@") {
        "S/symbol-identity".into()
    } else if rel.contains("module") || rel.contains("import") || rel.contains("export") {
        "M/module-resolution".into()
    } else if surface.starts_with("Object")
        || surface.starts_with("Reflect")
        || surface.starts_with("Proxy")
        || surface.contains("prototype")
    {
        "N/namespace-object-surface".into()
    } else if rel.contains("operator") || rel.contains("expressions") {
        "O/operator-semantics".into()
    } else {
        "E/eval-runtime-semantics".into()
    }
}

fn projection_axis(rel: &str, reason: &str, surface: &str) -> String {
    let r = reason.to_lowercase();
    if harness_surface(reason) {
        "runner-harness/$262-or-host-hook".into()
    } else if missing_surface(reason) {
        if r.contains("not defined") || r.contains("referenceerror") {
            "availability/missing-global-or-binding".into()
        } else {
            "availability/missing-method-or-intrinsic".into()
        }
    } else if rel.starts_with("language/") && (r.contains("syntaxerror") || rel.contains("invalid"))
    {
        "parser-form/early-error".into()
    // TECR-EXT 2: lift the missing-X-feature family above the generic
    // assertion-text matching below. cruft self-tags these reasons
    // (`parse: lex error:`, `parse:`, `compile:`, `not yet supported`,
    // `not implemented`, `(stub)`, `webassembly`) — they are unambiguous
    // tier discriminators that must beat the catch-all `r.contains("expected")`
    // value-semantics rule that otherwise siphons records like
    // "bangla is not yet supported ... Expected a RangeError" away from
    // the runtime-tier coordinate they belong to.
    } else if r.starts_with("parse: lex error:") {
        "availability/missing-lex-feature".into()
    } else if r.starts_with("parse: ") || r.contains("parse error") {
        "availability/missing-syntax-feature".into()
    } else if r.starts_with("compile: ") {
        "availability/missing-lowering-feature".into()
    } else if r.contains("not yet supported")
        || r.contains("not implemented")
        || r.contains("(stub)")
        || r.contains("webassembly")
    {
        "availability/missing-runtime-feature".into()
    } else if r.contains("expected") && r.contains("thrown") {
        "abrupt-completion/throw-missing".into()
    } else if r.contains("wrong error")
        || r.contains("expected a typeerror")
        || r.contains("expected a syntaxerror")
    {
        "abrupt-completion/wrong-throw-type".into()
    } else if r.contains("not callable") || r.contains("is not a function") {
        "callability/brand-or-method-missing".into()
    } else if r.contains("property descriptor")
        || r.contains("enumerable")
        || r.contains("configurable")
        || r.contains("writable")
        || surface.contains("defineProperty")
        || surface.contains("getOwnProperty")
        || surface.contains("Object.create")
    {
        "descriptor-shape/property-semantics".into()
    } else if r.contains("iterator") || r.contains("@@iterator") || surface.contains("Iterator") {
        "iteration/iterator-protocol".into()
    } else if r.contains("species") || surface.contains("species") {
        "constructor-species/species-constructor".into()
    } else if r.contains("prototype") || r.contains("[[prototype]]") {
        "realm-prototype/prototype-chain".into()
    } else if r.contains("samevalue") || r.contains("expected") {
        "value-semantics/wrong-result".into()
    // PCR-EXT 1+2: cruft runtime errors of "Cannot read/index property"
    // shape — missing-internal-slot or missing-method-access patterns.
    } else if r.contains("cannot read property") || r.contains("cannot index") {
        "availability/missing-internal-slot".into()
    // PCR-EXT 2: cruft runtime rejections of the form "not coercible to
    // Object" / "is not coercible" / "is not a constructor" — Annex B
    // String html-method tests (.anchor/.big/.blink/etc.) hit this when
    // cruft refuses introspection. Route to availability/missing-method-
    // or-intrinsic because the underlying Annex B method isn't implemented.
    } else if r.contains("not coercible to object")
        || r.contains("is not coercible")
        || r.contains("is not a constructor")
        || r.contains("isconstructor invoked")
    {
        "availability/missing-method-or-intrinsic".into()
    // PCR-EXT 1+2: descriptor-shape assertions in more phrasings than the
    // earlier rule caught.
    } else if r.contains("should be an own property")
        || r.contains("should have own property")
        || r.contains("descriptor value should")
        || r.contains("length descriptor")
    {
        "descriptor-shape/missing-own-property".into()
    // PCR-EXT 1: regex-engine partial carve-outs.
    } else if r.contains("unsupported by the v1 regex engine") {
        "partial/regex-features-missing".into()
    // PCR-EXT 1+2: regex lex/parse errors.
    } else if r.contains("unterminated regex")
        || (r.contains("lex error") && surface.contains("RegExp"))
        || r.contains("missing from character class")
    {
        "regexp-semantics/lex-error".into()
    // PCR-EXT 2: literal Test262Error throws (the test framework's own
    // assertion class, when raised without a more specific reason).
    // Route to value-semantics/wrong-result :: assertion/expected-
    // mismatch since the framework fires Test262Error when an assert_*
    // helper fails its comparison.
    } else if r.starts_with("test262error:") || r.starts_with("test262error") {
        "value-semantics/wrong-result".into()
    // PCR-EXT 2: identity/equality assertions in shorthand cruft-trace
    // phrasing (e.g., "testResult !== true", "X === false", "result !==
    // expected"). These are value-semantics wrong-results in test262's
    // older-style phrasing.
    } else if r.contains("!== true")
        || r.contains("!== false")
        || r.contains("=== false")
        || r.contains("=== true")
    {
        "value-semantics/wrong-result".into()
    // PCR-EXT 2: spec-numbered older-style assertions like "#1: ...".
    // Predominantly value-semantics in T_PA-era tests.
    } else if r.starts_with("#") && r.contains(":") {
        "value-semantics/wrong-result".into()
    // PCR-EXT 2: cruft runtime traces showing "(in-method=X)" or
    // "(in-call=X)" markers — these are runtime-tier errors that occurred
    // during method-dispatch. Most are availability gaps; route as such.
    } else if r.contains("(in-method=") || r.contains("(in-call=") {
        "availability/missing-method-or-intrinsic".into()
    // PCR-EXT 2: URIError-class results (encodeURI/decodeURI edge cases).
    } else if r.contains("urierror") || r.contains("uri error") {
        "value-semantics/wrong-result".into()
    } else if surface.contains("RegExp") {
        "regexp-semantics".into()
    } else {
        "uncategorized/projection".into()
    }
}

fn availability_axis(rel: &str, reason: &str, surface: &str) -> String {
    let r = reason.to_lowercase();
    if harness_surface(reason) {
        "runner-deferred".into()
    } else if rel.contains("staging/") || surface.starts_with("staging.") {
        "policy-deferred".into()
    } else if surface.starts_with("Temporal")
        || surface == "intl402.Temporal"
        || surface.starts_with("intl402.Temporal")
    {
        "absent-chapter".into()
    } else if surface.starts_with("Intl") || surface.starts_with("intl402.") {
        if missing_surface(reason) {
            "partial-chapter"
        } else {
            "partial"
        }
        .into()
    } else if r.contains("not defined")
        || r.contains("is not a constructor")
        || r.contains("is not a function")
        || r.contains("not callable")
    {
        "absent-or-partial-surface".into()
    } else if rel.contains("annexB/") || surface.starts_with("annexB.") {
        "policy-or-partial".into()
    } else {
        "available-surface".into()
    }
}

fn cut_kind(status: &str, reason: &str, availability: &str) -> String {
    if status == "NO_OUTPUT" || status == "TIMEOUT" || availability == "runner-deferred" {
        return "measurement-residue".into();
    }
    let r = reason.to_lowercase();
    if availability.contains("absent") {
        "K1/throw-on-use".into()
    } else if availability.contains("policy") {
        "version-or-policy-cut".into()
    } else if r.contains("expected") && r.contains("thrown") {
        "widening/abrupt-completion".into()
    } else if r.contains("samevalue") || r.contains("expected") {
        "widening/value-semantics".into()
    } else if r.contains("syntaxerror") {
        "successor/parser-acceptance".into()
    } else {
        "successor/semantic-refinement".into()
    }
}

fn missing_surface(reason: &str) -> bool {
    let r = reason.to_lowercase();
    r.contains("not defined")
        || r.contains("is not a function")
        || r.contains("not callable")
        || r.contains("is not a constructor")
        || r.contains("cannot read properties of undefined")
        || r.contains("cannot read property")
}

fn harness_surface(reason: &str) -> bool {
    let r = reason.to_lowercase();
    r.contains("$262 is not defined")
        || r.contains("create realm")
        || r.contains("detacharraybuffer")
}

fn failure_mode(status: &str, reason: &str) -> String {
    if status == "NO_OUTPUT" {
        return "runner/no-output".into();
    }
    if status == "TIMEOUT" {
        return "runner/timeout".into();
    }
    let r = reason.to_lowercase();
    if r.contains("syntaxerror") {
        "err:SyntaxError".into()
    } else if r.contains("typeerror") {
        "err:TypeError".into()
    } else if r.contains("referenceerror") {
        "err:ReferenceError".into()
    } else if r.contains("rangeerror") {
        "err:RangeError".into()
    } else if r.contains("test262error") {
        "err:Test262Error".into()
    } else if r.contains("not defined") {
        "err:ReferenceError-like".into()
    } else if r.contains("expected") {
        "assertion/expected-mismatch".into()
    } else {
        "failure/other".into()
    }
}

fn abstract_op_candidate(rel: &str, surface: &str, reason: &str) -> String {
    let r = reason.to_lowercase();
    if r.contains("samevalue") {
        "SameValue/SameValueZero".into()
    } else if r.contains("not callable") || r.contains("is not a function") {
        "IsCallable/GetMethod".into()
    } else if r.contains("constructor") || surface.contains("species") {
        "SpeciesConstructor/Construct".into()
    } else if r.contains("iterator") || rel.contains("iterator") {
        "GetIterator/IteratorClose".into()
    } else if r.contains("property descriptor")
        || r.contains("defineproperty")
        || surface.contains("defineProperty")
        || surface.contains("getOwnProperty")
    {
        "OrdinaryDefineOwnProperty/OrdinaryGetOwnProperty".into()
    } else if r.contains("prototype") || surface.contains("prototype") {
        "GetPrototypeFromConstructor/OrdinaryGetPrototypeOf".into()
    } else if rel.contains("to-string") || r.contains("to string") {
        "ToString".into()
    } else if rel.contains("to-number") || r.contains("to number") {
        "ToNumber".into()
    } else if rel.contains("to-object") || r.contains("to object") {
        "ToObject".into()
    } else if rel.starts_with("language/") {
        "RuntimeSemantics/Evaluation".into()
    } else {
        "(unmapped)".into()
    }
}

fn feature_shape(path: &str, reason: &str) -> String {
    let mut tags = Vec::new();
    if let Ok(src) = fs::read_to_string(path) {
        if let Some(fm) = frontmatter(&src) {
            if let Some(features) = field_value(fm, "features:") {
                for f in features.trim_matches(&['[', ']'][..]).split(',') {
                    let f = f.trim().trim_matches('"').trim_matches('\'');
                    if !f.is_empty() {
                        tags.push(format!("feat:{}", f));
                    }
                }
            }
            if fm.contains("negative:") {
                if let Some(t) = field_value(fm, "type:") {
                    tags.push(format!("negative:{}", t.trim()));
                } else {
                    tags.push("negative".into());
                }
            }
            for flag in ["module", "async", "onlyStrict", "noStrict", "raw"] {
                if fm.contains(flag) {
                    tags.push(format!("flag:{}", flag));
                }
            }
        }
    }
    let r = reason.to_lowercase();
    for (needle, tag) in [
        ("symbol", "shape:symbol"),
        ("proxy", "shape:proxy"),
        ("constructor", "shape:constructor"),
        ("species", "shape:species"),
        ("iterator", "shape:iterator"),
        ("detached", "shape:detached-buffer"),
        ("resizable", "shape:resizable-buffer"),
    ] {
        if r.contains(needle) {
            tags.push(tag.into());
        }
    }
    if tags.is_empty() {
        "(no-feature-tag)".into()
    } else {
        tags.sort();
        tags.dedup();
        tags.join(";")
    }
}

fn frontmatter(src: &str) -> Option<&str> {
    let start = src.find("/*---")?;
    let rest = &src[start + 5..];
    let end = rest.find("---*/")?;
    Some(&rest[..end])
}

fn field_value<'a>(fm: &'a str, field: &str) -> Option<&'a str> {
    let idx = fm.find(field)?;
    let rest = &fm[idx + field.len()..];
    Some(rest.lines().next().unwrap_or("").trim())
}

fn write_outputs(
    out_dir: &Path,
    source: &Path,
    run_id: &str,
    totals: &BTreeMap<String, usize>,
    coords: &[Coord],
    malformed_fragments: usize,
) {
    let mut by_pin: HashMap<String, Vec<&Coord>> = HashMap::new();
    let mut by_resolver: HashMap<String, Vec<&Coord>> = HashMap::new();
    let mut by_surface: HashMap<String, Vec<&Coord>> = HashMap::new();
    let mut by_projection: HashMap<String, Vec<&Coord>> = HashMap::new();
    let mut by_rung: HashMap<String, Vec<&Coord>> = HashMap::new();
    let mut by_axis: HashMap<String, Vec<&Coord>> = HashMap::new();
    let mut by_availability: HashMap<String, Vec<&Coord>> = HashMap::new();
    let mut by_cut_kind: HashMap<String, Vec<&Coord>> = HashMap::new();
    let mut by_abstract_op: HashMap<String, Vec<&Coord>> = HashMap::new();
    for c in coords {
        by_pin.entry(c.pin.clone()).or_default().push(c);
        by_resolver.entry(c.resolver.clone()).or_default().push(c);
        by_surface.entry(c.surface.clone()).or_default().push(c);
        by_projection
            .entry(c.projection.clone())
            .or_default()
            .push(c);
        by_rung.entry(c.rung.clone()).or_default().push(c);
        by_axis.entry(c.axis.clone()).or_default().push(c);
        by_availability
            .entry(c.availability.clone())
            .or_default()
            .push(c);
        by_cut_kind.entry(c.cut_kind.clone()).or_default().push(c);
        by_abstract_op
            .entry(c.abstract_op.clone())
            .or_default()
            .push(c);
    }

    let pass = *totals.get("PASS").unwrap_or(&0);
    let fail = *totals.get("FAIL").unwrap_or(&0);
    let skip = *totals.get("SKIP").unwrap_or(&0);
    let no_output = *totals.get("NO_OUTPUT").unwrap_or(&0);
    let timeout = *totals.get("TIMEOUT").unwrap_or(&0);
    let malformed = *totals.get("MALFORMED").unwrap_or(&0) + malformed_fragments;
    let runnable = pass + fail;
    let pct = if runnable > 0 {
        100.0 * pass as f64 / runnable as f64
    } else {
        0.0
    };

    let mut summary = String::new();
    summary.push_str(&format!(
        "# Full Test262 Pin-Art Interpretation - {}\n\n",
        run_id
    ));
    summary.push_str("## Source\n\n");
    summary.push_str(&format!("- Raw results: `{}`\n", source.display()));
    summary
        .push_str("- Interpretation: observed result -> engine coordinate -> Pin-Art target\n\n");
    summary.push_str("## Raw Tallies\n\n");
    summary.push_str(&format!("- PASS: **{}**\n", pass));
    summary.push_str(&format!("- FAIL: **{}**\n", fail));
    summary.push_str(&format!("- SKIP: {}\n", skip));
    summary.push_str(&format!("- NO_OUTPUT: {}\n", no_output));
    summary.push_str(&format!("- TIMEOUT: {}\n", timeout));
    summary.push_str(&format!("- MALFORMED/fragments: {}\n", malformed));
    summary.push_str(&format!(
        "- Runnable pass rate: **{:.1}%** ({}/{})\n\n",
        pct, pass, runnable
    ));
    summary.push_str("## Apparatus Tallies\n\n");
    summary.push_str(&format!(
        "- Interpreted non-pass records: **{}**\n",
        coords.len()
    ));
    summary.push_str(&format!(
        "- Distinct Pin-Art coordinates: **{}**\n",
        by_pin.len()
    ));
    summary.push_str(&format!(
        "- Distinct resolver instances: **{}**\n",
        by_resolver.len()
    ));
    summary.push_str(&format!("- Distinct engine rungs: **{}**\n", by_rung.len()));
    summary.push_str(&format!(
        "- Distinct constraint axes: **{}**\n",
        by_axis.len()
    ));
    summary.push_str(&format!(
        "- Distinct availability classes: **{}**\n",
        by_availability.len()
    ));
    summary.push_str(&format!(
        "- Distinct cut kinds: **{}**\n",
        by_cut_kind.len()
    ));
    summary.push_str(&format!("- Distinct surfaces: **{}**\n", by_surface.len()));
    summary.push_str("\nSee `matrix.md` for ranked engine-coordinate distributions and `interpreted.jsonl` for per-test records.\n");
    fs::write(out_dir.join("summary.md"), summary).expect("write summary");

    let mut matrix = String::new();
    matrix.push_str(&format!("# Full Test262 Pin-Art Matrix - {}\n\n", run_id));
    write_ranked(
        &mut matrix,
        "Pin-Art coordinates",
        "Coordinate",
        &by_pin,
        |c| &c.rel,
    );
    write_ranked(
        &mut matrix,
        "Resolver-instance marginal",
        "Resolver",
        &by_resolver,
        |c| &c.rel,
    );
    write_ranked(&mut matrix, "Engine-rung marginal", "Rung", &by_rung, |c| {
        &c.rel
    });
    write_ranked(
        &mut matrix,
        "Constraint-axis marginal",
        "Axis",
        &by_axis,
        |c| &c.rel,
    );
    write_ranked(
        &mut matrix,
        "Availability marginal",
        "Availability",
        &by_availability,
        |c| &c.rel,
    );
    write_ranked(
        &mut matrix,
        "Cut-kind marginal",
        "Cut kind",
        &by_cut_kind,
        |c| &c.rel,
    );
    write_ranked(
        &mut matrix,
        "Abstract-operation candidate marginal",
        "Candidate",
        &by_abstract_op,
        |c| &c.rel,
    );
    write_ranked(
        &mut matrix,
        "Surface marginal",
        "Surface",
        &by_surface,
        |c| &c.rel,
    );
    write_ranked(
        &mut matrix,
        "Projection marginal",
        "Projection",
        &by_projection,
        |c| &c.rel,
    );
    fs::write(out_dir.join("matrix.md"), matrix).expect("write matrix");

    let mut jsonl = String::new();
    for c in coords {
        jsonl.push_str(&format!(
            "{{\"file\":\"{}\",\"status\":\"{}\",\"resolver\":\"{}\",\"tier\":\"{}\",\"rung\":\"{}\",\"axis\":\"{}\",\"surface\":\"{}\",\"availability\":\"{}\",\"cut_kind\":\"{}\",\"projection\":\"{}\",\"failure_mode\":\"{}\",\"abstract_op\":\"{}\",\"feature_shape\":\"{}\",\"pin\":\"{}\",\"reason\":\"{}\"}}\n",
            esc(&c.rel), esc(&c.status), esc(&c.resolver), esc(&c.tier), esc(&c.rung),
            esc(&c.axis), esc(&c.surface), esc(&c.availability), esc(&c.cut_kind),
            esc(&c.projection), esc(&c.failure_mode), esc(&c.abstract_op), esc(&c.feature_shape), esc(&c.pin),
            esc(&c.reason.chars().take(240).collect::<String>())
        ));
    }
    fs::write(out_dir.join("interpreted.jsonl"), jsonl).expect("write interpreted");
}

fn write_ranked<F>(
    out: &mut String,
    title: &str,
    label: &str,
    map: &HashMap<String, Vec<&Coord>>,
    example: F,
) where
    F: Fn(&Coord) -> &str,
{
    let mut rows: Vec<(&String, usize, &Coord)> = map
        .iter()
        .filter_map(|(k, v)| v.first().map(|first| (k, v.len(), *first)))
        .collect();
    rows.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(b.0)));
    out.push_str(&format!("## {}\n\n", title));
    out.push_str(&format!("| Rank | {} | Count | Example |\n", label));
    out.push_str("|---:|---|---:|---|\n");
    for (i, (k, n, first)) in rows.into_iter().take(60).enumerate() {
        out.push_str(&format!(
            "| {} | `{}` | {} | `{}` |\n",
            i + 1,
            k,
            n,
            example(first)
        ));
    }
    out.push('\n');
}

fn esc(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf()
}

fn find_latest_full() -> Option<PathBuf> {
    let sidecar = std::env::var_os("CRUFTLESS_TEST262_RESULTS_ROOT")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("/home/jaredef/Developer/cruftless-sidecar/results"));
    let mut best = None;
    for entry in fs::read_dir(sidecar).ok()?.filter_map(|e| e.ok()) {
        let p = entry.path();
        let name = p.file_name().and_then(|s| s.to_str()).unwrap_or("");
        let candidate = p.join("results.jsonl");
        if name.starts_with("test262-full-") && candidate.exists() {
            if best
                .as_ref()
                .map(|b: &PathBuf| candidate > *b)
                .unwrap_or(true)
            {
                best = Some(candidate);
            }
        }
    }
    best
}
