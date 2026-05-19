//! Spec-vs-IR drift linter per IR-DESIGN.md §6.
//!
//! Tier 1 stub: the data structures are in place; the spec-XML parser is
//! Tier 2 work. For Tier 1 the linter accepts a hand-authored
//! `SpecStepRecord` list and walks the IR against it.

use crate::ir::{Expr, IRFunction, IRNode, Step};

/// One canonical algorithm step from ECMA-262, as parsed from `<emu-alg>`.
/// Tier 1 takes this list as direct input; Tier 2 will parse it from XML.
#[derive(Debug, Clone)]
pub struct SpecStepRecord {
    /// The step identifier (e.g. "1", "6.c.ii").
    pub step_id: String,
    /// The named abstract operations the step invokes
    /// (e.g. ["ToObject"], ["IsCallable", "Throw"]).
    pub abstract_ops: Vec<&'static str>,
    /// If the step throws, the canonical class.
    pub throws: Option<&'static str>,
    /// The literal spec prose (for the lint message).
    pub prose: &'static str,
}

#[derive(Debug, Clone)]
pub struct LintFinding {
    pub spec_step: String,
    pub kind: FindingKind,
    pub message: String,
}

#[derive(Debug, Clone)]
pub enum FindingKind {
    MissingStep,
    ExtraStep,
    MissingAbstractOp(&'static str),
    ExtraAbstractOp(String),
    ThrowClassMismatch { spec: &'static str, ir: String },
}

#[derive(Debug, Clone, Default)]
pub struct LintReport {
    pub findings: Vec<LintFinding>,
}

impl LintReport {
    pub fn ok(&self) -> bool {
        self.findings.is_empty()
    }
}

/// Run the spec-vs-IR diff. Tier 1: a naive 1:1 walk by step_id.
pub fn lint(f: &IRFunction, spec: &[SpecStepRecord]) -> LintReport {
    let mut report = LintReport::default();

    // Index IR steps by spec_step.
    let mut ir_steps_by_id: std::collections::HashMap<&str, &Step> =
        std::collections::HashMap::new();
    collect_steps(&f.body, &mut ir_steps_by_id);

    // 1. Every spec step must have a corresponding IR step.
    for sp in spec {
        let Some(ir_step) = ir_steps_by_id.get(sp.step_id.as_str()) else {
            report.findings.push(LintFinding {
                spec_step: sp.step_id.clone(),
                kind: FindingKind::MissingStep,
                message: format!(
                    "IR for §{} step {} missing; spec says: {}",
                    f.spec_section, sp.step_id, sp.prose
                ),
            });
            continue;
        };

        // 2. Each abstract op the spec names must appear in the IR step.
        let ir_ops = collect_abstract_ops_in_step(ir_step);
        for op in &sp.abstract_ops {
            if !ir_ops.contains(op) {
                report.findings.push(LintFinding {
                    spec_step: sp.step_id.clone(),
                    kind: FindingKind::MissingAbstractOp(op),
                    message: format!(
                        "IR for §{} step {} lacks {} call; spec says: {}",
                        f.spec_section, sp.step_id, op, sp.prose
                    ),
                });
            }
        }

        // 3. Throw class must match.
        if let Some(spec_throw) = sp.throws {
            let ir_throw = find_throw_class(ir_step);
            match ir_throw {
                Some(ir_class) if ir_class == spec_throw => {}
                Some(ir_class) => report.findings.push(LintFinding {
                    spec_step: sp.step_id.clone(),
                    kind: FindingKind::ThrowClassMismatch {
                        spec: spec_throw,
                        ir: ir_class.to_string(),
                    },
                    message: format!(
                        "IR for §{} step {} throws {}; spec throws {}",
                        f.spec_section, sp.step_id, ir_class, spec_throw
                    ),
                }),
                None => report.findings.push(LintFinding {
                    spec_step: sp.step_id.clone(),
                    kind: FindingKind::MissingAbstractOp("Throw"),
                    message: format!(
                        "IR for §{} step {} does not throw; spec throws {}",
                        f.spec_section, sp.step_id, spec_throw
                    ),
                }),
            }
        }
    }

    // 4. Every IR step must have a corresponding spec step.
    let spec_ids: std::collections::HashSet<&str> =
        spec.iter().map(|s| s.step_id.as_str()).collect();
    for (id, _step) in &ir_steps_by_id {
        if !spec_ids.contains(id) {
            report.findings.push(LintFinding {
                spec_step: id.to_string(),
                kind: FindingKind::ExtraStep,
                message: format!(
                    "IR for §{} step {} not in spec",
                    f.spec_section, id
                ),
            });
        }
    }

    report
}

fn collect_steps<'a>(
    body: &'a [Step],
    out: &mut std::collections::HashMap<&'a str, &'a Step>,
) {
    for s in body {
        // Convention: sub-step IDs of the form "<parent>.throw" or
        // "<parent>.guard" are *inline-throw consequents* — the spec
        // doesn't distinguish them as separate steps, so the linter
        // doesn't either. They're already collected by walk_step_collecting
        // for the abstract-op set of the parent step. Skip them here so
        // they don't appear as "ExtraStep" findings.
        let is_synthetic_inline =
            s.spec_step.ends_with(".throw") ||
            s.spec_step.ends_with(".guard") ||
            s.spec_step.ends_with(".return") ||
            s.spec_step.ends_with(".adj") ||
            s.spec_step.ends_with(".seed");
        if !is_synthetic_inline {
            out.insert(s.spec_step.as_str(), s);
        }
        // Recurse into If/While bodies — the inner steps may have their
        // own spec-step IDs like "6.a", "6.c.i".
        match &s.node {
            IRNode::If { then_body, else_body, .. } => {
                collect_steps(then_body, out);
                collect_steps(else_body, out);
            }
            IRNode::While { body, .. } => collect_steps(body, out),
            _ => {}
        }
    }
}

fn collect_abstract_ops_in_step(step: &Step) -> std::collections::HashSet<&'static str> {
    let mut set = std::collections::HashSet::new();
    walk_step_collecting(&step.node, &mut set);
    set
}

/// Recursively visit an IRNode's body. The spec writes "If X is false,
/// throw a TypeError exception" as one step; the IR may represent it as
/// an If with a Throw consequent. The linter recognizes the inline throw
/// as part of the parent step's abstract-op set, so the spec-vs-IR diff
/// can match the spec's step granularity.
fn walk_step_collecting(node: &IRNode, set: &mut std::collections::HashSet<&'static str>) {
    match node {
        IRNode::Let { value, .. }
        | IRNode::LetIndex { value, .. }
        | IRNode::AssignIndex { value, .. }
        | IRNode::Assign { value, .. }
        | IRNode::Expr(value) => {
            collect_abstract_ops_in_expr(value, set);
        }
        IRNode::Return(e) => collect_abstract_ops_in_expr(e, set),
        IRNode::Throw { .. } => {
            set.insert("Throw");
        }
        IRNode::If { cond, then_body, else_body } => {
            collect_abstract_ops_in_expr(cond, set);
            // Inline-throw recognition: if the then_body is a single Throw
            // step (or contains only a Throw at top level), surface it as
            // part of this step's op set. This matches the spec's
            // single-step "If X, throw Y" convention.
            for s in then_body {
                if matches!(s.node, IRNode::Throw { .. }) {
                    walk_step_collecting(&s.node, set);
                }
            }
            for s in else_body {
                if matches!(s.node, IRNode::Throw { .. }) {
                    walk_step_collecting(&s.node, set);
                }
            }
        }
        IRNode::While { cond, .. } => {
            collect_abstract_ops_in_expr(cond, set);
        }
    }
}

fn collect_abstract_ops_in_expr(
    e: &Expr,
    set: &mut std::collections::HashSet<&'static str>,
) {
    match e {
        Expr::RequireObjectCoercible(v) => {
            set.insert("RequireObjectCoercible");
            collect_abstract_ops_in_expr(v, set);
        }
        Expr::ToObject(v) => {
            set.insert("ToObject");
            collect_abstract_ops_in_expr(v, set);
        }
        Expr::ToPrimitive(v, _) => {
            set.insert("ToPrimitive");
            collect_abstract_ops_in_expr(v, set);
        }
        Expr::ToString(v) => {
            set.insert("ToString");
            collect_abstract_ops_in_expr(v, set);
        }
        Expr::ToNumber(v) => {
            set.insert("ToNumber");
            collect_abstract_ops_in_expr(v, set);
        }
        Expr::ToInteger(v) => {
            set.insert("ToInteger");
            collect_abstract_ops_in_expr(v, set);
        }
        Expr::ToLength(v) => {
            set.insert("ToLength");
            collect_abstract_ops_in_expr(v, set);
        }
        Expr::ToUint32(v) => {
            set.insert("ToUint32");
            collect_abstract_ops_in_expr(v, set);
        }
        Expr::ToBoolean(v) => {
            set.insert("ToBoolean");
            collect_abstract_ops_in_expr(v, set);
        }
        Expr::ToPropertyKey(v) => {
            set.insert("ToPropertyKey");
            collect_abstract_ops_in_expr(v, set);
        }
        Expr::IsCallable(v) => {
            set.insert("IsCallable");
            collect_abstract_ops_in_expr(v, set);
        }
        Expr::IsConstructor(v) => {
            set.insert("IsConstructor");
            collect_abstract_ops_in_expr(v, set);
        }
        Expr::IsArray(v) => {
            set.insert("IsArray");
            collect_abstract_ops_in_expr(v, set);
        }
        Expr::IsRegExp(v) => {
            set.insert("IsRegExp");
            collect_abstract_ops_in_expr(v, set);
        }
        Expr::Get(o, p) => {
            set.insert("Get");
            collect_abstract_ops_in_expr(o, set);
            collect_abstract_ops_in_expr(p, set);
        }
        Expr::HasProperty(o, p) => {
            set.insert("HasProperty");
            collect_abstract_ops_in_expr(o, set);
            collect_abstract_ops_in_expr(p, set);
        }
        Expr::HasOwnProperty(o, p) => {
            set.insert("HasOwnProperty");
            collect_abstract_ops_in_expr(o, set);
            collect_abstract_ops_in_expr(p, set);
        }
        Expr::OrdinaryObjectCreate { proto, slots } => {
            set.insert("OrdinaryObjectCreate");
            collect_abstract_ops_in_expr(proto, set);
            for (_, v) in slots {
                collect_abstract_ops_in_expr(v, set);
            }
        }
        Expr::ArraySpeciesCreate { o, length } => {
            set.insert("ArraySpeciesCreate");
            collect_abstract_ops_in_expr(o, set);
            collect_abstract_ops_in_expr(length, set);
        }
        Expr::Call { function, this, args } => {
            set.insert("Call");
            collect_abstract_ops_in_expr(function, set);
            collect_abstract_ops_in_expr(this, set);
            for a in args {
                collect_abstract_ops_in_expr(a, set);
            }
        }
        Expr::Construct { ctor, args } => {
            set.insert("Construct");
            collect_abstract_ops_in_expr(ctor, set);
            for a in args {
                collect_abstract_ops_in_expr(a, set);
            }
        }
        Expr::Invoke { object, method, args } => {
            set.insert("Invoke");
            collect_abstract_ops_in_expr(object, set);
            collect_abstract_ops_in_expr(method, set);
            for a in args {
                collect_abstract_ops_in_expr(a, set);
            }
        }
        Expr::CreateDataPropertyOrThrow(a, b, c) => {
            set.insert("CreateDataPropertyOrThrow");
            collect_abstract_ops_in_expr(a, set);
            collect_abstract_ops_in_expr(b, set);
            collect_abstract_ops_in_expr(c, set);
        }
        Expr::LengthOfArrayLike(o) => {
            set.insert("LengthOfArrayLike");
            collect_abstract_ops_in_expr(o, set);
        }
        Expr::SameValue(a, b) => {
            set.insert("SameValue");
            collect_abstract_ops_in_expr(a, set);
            collect_abstract_ops_in_expr(b, set);
        }
        Expr::SameValueZero(a, b) => {
            set.insert("SameValueZero");
            collect_abstract_ops_in_expr(a, set);
            collect_abstract_ops_in_expr(b, set);
        }
        Expr::StrictEq(a, b) => {
            set.insert("IsStrictlyEqual");
            collect_abstract_ops_in_expr(a, set);
            collect_abstract_ops_in_expr(b, set);
        }
        Expr::OpAdd(a, b) | Expr::OpSub(a, b) | Expr::OpMul(a, b) |
        Expr::LooseEq(a, b) |
        Expr::Lt(a, b) | Expr::Le(a, b) => {
            collect_abstract_ops_in_expr(a, set);
            collect_abstract_ops_in_expr(b, set);
        }
        Expr::Not(v) => collect_abstract_ops_in_expr(v, set),
        Expr::HasSlot(v, _) | Expr::GetSlot(v, _) => {
            set.insert("HasSlot");
            collect_abstract_ops_in_expr(v, set);
        }
        Expr::Var(_) | Expr::Undefined | Expr::Null | Expr::Bool(_) |
        Expr::Number(_) | Expr::Str(_) | Expr::Arg(_) | Expr::This |
        Expr::HasArg(_) | Expr::AllArgs | Expr::IntConst(_) => {}
        Expr::CallBuiltin { name, args } => {
            set.insert(name);
            for a in args { collect_abstract_ops_in_expr(a, set); }
        }
        Expr::AsIndex(v) => collect_abstract_ops_in_expr(v, set),
        Expr::IndexAsValue(v) => collect_abstract_ops_in_expr(v, set),
        Expr::IndexAsKey(v) => {
            // IndexAsKey(k) is the typed shortcut for ! ToString(𝔽(k)) —
            // the spec writes this as a ToString call, so the linter
            // counts it as one.
            set.insert("ToString");
            collect_abstract_ops_in_expr(v, set);
        }
        Expr::IndexAdd(a, b) => {
            collect_abstract_ops_in_expr(a, set);
            collect_abstract_ops_in_expr(b, set);
        }
    }
}

fn find_throw_class(step: &Step) -> Option<&'static str> {
    match &step.node {
        IRNode::Throw { class, .. } => Some(class.rust_variant()),
        IRNode::If { then_body, else_body, .. } => then_body
            .iter()
            .find_map(find_throw_class)
            .or_else(|| else_body.iter().find_map(find_throw_class)),
        IRNode::While { body, .. } => body.iter().find_map(find_throw_class),
        _ => None,
    }
}
