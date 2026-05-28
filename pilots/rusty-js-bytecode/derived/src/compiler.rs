//! Compiler from rusty_js_ast typed AST to bytecode. Per design spec §IV–§V.
//!
//! v1 (round 3.c.b): single-pass walk of expressions + minimal statement
//! support (ExpressionStatement + Return). Variable references compile to
//! LOAD_GLOBAL by default; local scope resolution + upvalue binding land in
//! round 3.c.c. Control-flow opcodes land in 3.c.c, function/closure in 3.c.d.

use crate::constants::{Constant, ConstantsPool};
use crate::op::*;
use rusty_js_ast::*;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct CompileError {
    pub span: Span,
    pub message: String,
}

#[derive(Debug, Clone)]
pub struct LocalDescriptor {
    pub name: String,
    pub kind: VariableKind,
    pub depth: u32,
}

#[derive(Debug, Clone)]
pub struct UpvalueDescriptor {
    pub source: UpvalueSource,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UpvalueSource {
    Local(u16),
    Upvalue(u16),
}

#[derive(Debug, Clone)]
pub struct FunctionProto {
    pub bytecode: Vec<u8>,
    pub constants: ConstantsPool,
    pub params: u16,
    /// ECMA-262 §10.2.9: the function's [[InitialName]] surfaced as the
    /// own .name property. Comes from the function-decl / function-expr's
    /// identifier; empty for unnamed expressions and arrows unless the
    /// compiler applied a NamedEvaluation hint at the binding site.
    pub display_name: String,
    /// ECMA-262 §10.2.10 FunctionLength: count of formal parameters before
    /// the first rest or default-valued parameter. Surfaced as the own
    /// .length property. Distinct from `params` (which counts the raw
    /// parameter slots including rest and defaults).
    pub function_length: u16,
    pub locals: Vec<LocalDescriptor>,
    pub upvalues: Vec<UpvalueDescriptor>,
    /// Tier-Ω.5.l: if the last parameter is a rest parameter (`...name`),
    /// this is its local slot index. The runtime collects all arguments
    /// from this index onward into a single Array bound to this slot.
    /// None for ordinary parameter lists.
    pub rest_param_slot: Option<u16>,
    /// Tier-Ω.5.zzz: slot for the magic `arguments` local. Populated by
    /// call_function with an Array of the actual args. None when not
    /// allocated (arrow bodies skip this; only non-arrow function-decl /
    /// function-expression bodies get it). Indexed reads `arguments[i]`
    /// resolve via Array indexing; .length, .slice, etc. work via the
    /// Array prototype chain.
    pub arguments_slot: Option<u16>,
    /// Tier-Ω.5.kkkkk: slot for the named-function-expression self-binding.
    /// When the function is `function NAME() { ... }` AS AN EXPRESSION (not
    /// a declaration), NAME is bound inside the body to the function itself
    /// per ECMA-262 §15.2.5. call_function populates this slot with the
    /// closure object on entry. just-curry-it's recursive `function curried()
    /// { ... curried.apply(...) }` pattern depends on this.
    pub self_name_slot: Option<u16>,
    /// Byte offset immediately after the formal-parameter initialization
    /// prologue. Generator calls execute this prefix at call time before
    /// returning the generator object, then resume eager body collection
    /// after the boundary.
    pub param_prologue_end: usize,
    /// Tier-Ω.5.eeeeee: marker for generator functions. The runtime
    /// returns an iterator over an eagerly-collected yields array
    /// instead of running the body to its return. v1 deviation: real
    /// generators are coroutines (suspend on yield, resume on next).
    /// The eager-collect path matches observable semantics for forward-
    /// only generators with no value-passed-back (the dominant idiom in
    /// superstruct, p-map, ts-pattern's iteration helpers).
    pub is_generator: bool,
    /// Ω.5.P51.E1: per-function source-line index. Same shape as
    /// CompiledModule.line_starts. Populated for module-compiled
    /// FunctionProtos; empty for synthesized hand-built ones.
    pub line_starts: Vec<u32>,
    /// Ω.5.P51.E1: per-function bytecode→span map. (bytecode_offset, Span)
    /// pairs covering this function's body. Lets runtime error enrichment
    /// derive the line:col for a fault inside any function frame, not just
    /// the top-level module frame.
    pub source_map: Vec<(usize, Span)>,
    /// Ω.5.P53.E2: per-function AST-construct probe tags. (bytecode_offset,
    /// construct_name) pairs marking the entry of bug-prone compile sites
    /// (optional-chain member, optional call, try/catch handler, loop
    /// bodies). Runtime error enrichment looks up the most recent tag
    /// with offset <= failing_pc to attribute faults to a named construct
    /// at the AST-to-bytecode resolver boundary. Doc 729 §VIII property-
    /// class: §2 boundary-integrity instrumentation.
    pub construct_tags: Vec<(usize, &'static str)>,
    /// Ω.5.P51.E1: URL of the source this function was compiled from.
    /// Propagated from the parent module / compiler so closure-frame
    /// errors can emit `@url:line:col`. Empty when not threaded.
    pub source_url: String,
    /// Ω.5.P50.E1: async-function marker per ECMA-262 §15.7.5. AsyncFunction
    /// objects do not have a `prototype` own property (unlike base function
    /// declarations). MakeClosure consults this to skip the auto-prototype
    /// allocation; the CJS-as-ESM namespace builder then sees no `prototype`
    /// to leak. Async functions with explicit `.prototype = X` assignment
    /// remain supported via the normal property-set path.
    pub is_async: bool,
    /// EXT 73: ECMA-262 §10.2.1.2 OrdinaryCallBindThis discriminator.
    /// True when the function body begins with a `"use strict"` directive
    /// prologue, when the function is a class method (always strict per
    /// §15.7), or when the enclosing context is already strict (modules,
    /// or a strict outer function). Read at call time in call_function:
    /// strict functions receive thisArg unchanged; non-strict (sloppy)
    /// functions coerce null/undefined → globalThis and primitives via
    /// ToObject.
    pub strict: bool,
}

#[derive(Debug, Clone)]
pub struct CompiledModule {
    pub bytecode: Vec<u8>,
    pub constants: ConstantsPool,
    pub locals: Vec<LocalDescriptor>,
    pub source_map: Vec<(usize, Span)>,
    /// Ω.5.P04.E2.strict-write-enforcement: module-level strict flag.
    /// Set by compile_module from the directive-prologue scan plus the
    /// .mjs / has_module_syntax checks. Module frames built via
    /// Frame::new_module read this into Frame::strict so SetProp /
    /// SetIndex / StoreGlobal can enforce strict-mode rejection of
    /// write-to-non-writable and write-to-undeclared per ECMA §10.1.9.4
    /// step 4 and §13.15.4 step 1.f.
    pub strict: bool,
    /// Tier-Ω.5.b: ESM static imports. Each entry binds a local slot to a
    /// value drawn from another module's namespace. The runtime resolves
    /// `module_request` and populates `slot` BEFORE running the module body.
    pub imports: Vec<ImportBinding>,
    /// Tier-Ω.5.b: ESM static exports. After running the module body, the
    /// runtime reads each `local` slot and writes it to namespace[`exported`].
    /// `default` exports use the synthetic local "<module.default>".
    pub exports: Vec<ExportBinding>,
    /// Tier-Ω.5.h: ESM re-export source dependencies. Each entry is the
    /// `from "..."` specifier of an `export ... from "..."` form. The
    /// runtime loads these modules eagerly (like ImportDeclaration sources)
    /// so their namespaces are populated in the module cache before the
    /// namespace-build phase reads from them.
    pub reexport_sources: Vec<String>,
    /// Tier-Ω.5.IIIIIIII: side-effect ImportDeclaration sources — i.e.
    /// `import "X"` (no default / namespace / named bindings). Per ECMA-262
    /// §16.2.1.5 these still require module evaluation; the runtime loads
    /// them before running the module body. Previously the compiler tracked
    /// only bound imports in `self.imports`, so side-effect imports were
    /// silently no-ops (autoprefixer / many node_modules use this for
    /// CSS / runtime-side-effect setup).
    pub side_effect_imports: Vec<String>,
    /// Ω.5.P53.E2: module-level construct tags. Same shape as
    /// FunctionProto.construct_tags; covers the module-body bytecode.
    pub construct_tags: Vec<(usize, &'static str)>,
    /// Ω.5.P51.E1: byte offsets of the start of each line in the source
    /// (line 0 starts at 0; line N starts at line_starts[N]). Computed
    /// once at compile time. Lets the runtime convert a Span's byte offset
    /// to (line, col) for error-message enrichment without re-scanning the
    /// source. Empty for hand-built frames or modules compiled without a
    /// source-line index (legacy callers).
    pub line_starts: Vec<u32>,
    /// ES-EXT/EVFEI pipeline discriminator: true for Script-mode frames
    /// whose variable environment is the global environment record.
    pub eval_var_env_is_global: bool,
}

/// One ESM import binding. Compiled from ImportDeclaration entries.
#[derive(Debug, Clone)]
pub struct ImportBinding {
    /// Local-slot index this binding writes to.
    pub slot: u16,
    /// Specifier from `from "..."`. Either `node:*` or a relative path
    /// after Tier-Ω.5.b's resolver.
    pub module_request: String,
    /// What to read from the imported module's namespace.
    pub kind: ImportBindingKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ImportBindingKind {
    /// `import x from "..."` — read namespace["default"].
    Default,
    /// `import * as x from "..."` — bind the namespace object itself.
    Namespace,
    /// `import { name } from "..."` — read namespace[name].
    Named(String),
}

/// One ESM export binding. Compiled from ExportDeclaration entries.
///
/// Tier-Ω.5.h widened this from a single (exported, local) struct to a
/// four-variant enum to accommodate the four re-export forms. The
/// runtime's namespace-build phase iterates these and reads from either
/// the local-slot table (Local) or from a previously-loaded source
/// module's namespace (Named / Star / StarAs). Snapshot semantics: source
/// modules are loaded eagerly during evaluate_module's link phase so
/// their namespaces are populated by the time the namespace-build phase
/// runs. Cyclic re-exports may observe partial namespaces (v1 deviation
/// from spec live-bindings — see module.rs banner).
#[derive(Debug, Clone)]
pub enum ExportBinding {
    /// `export { x }` (no `from`) and `export default ...` — the namespace
    /// entry is populated from a local slot in this module's frame.
    Local {
        /// Name as it appears in the namespace.
        exported: String,
        /// Local-slot index whose value populates namespace[`exported`].
        local: u16,
    },
    /// `export { x } from "..."` or `export { x as y } from "..."`. The
    /// runtime reads `source_specifier`'s namespace at `imported`, writes
    /// the value to this module's namespace under `exported`. Either
    /// name may be `"default"` to express default<->named conversions.
    Named {
        exported: String,
        source_specifier: String,
        imported: String,
    },
    /// `export * from "..."`. The runtime iterates the source's namespace
    /// own properties and copies each (except `"default"` per spec
    /// §16.2.3.7) into this namespace under the same name.
    Star { source_specifier: String },
    /// `export * as ns from "..."`. The runtime writes the source's whole
    /// namespace object to this namespace under `exported`.
    StarAs {
        exported: String,
        source_specifier: String,
    },
}

pub struct Compiler {
    bytecode: Vec<u8>,
    constants: ConstantsPool,
    locals: Vec<LocalDescriptor>,
    source_map: Vec<(usize, Span)>,
    /// Stack of loop frames. Each frame collects patch sites for break
    /// jumps and the bytecode offset of the loop's continue target.
    /// Push on loop entry, pop on loop exit.
    loop_stack: Vec<LoopFrame>,
    /// FACL-EXT 1: function-level stack of pending finalizer AST nodes.
    /// Pushed at try-finally entry, popped at try-finally exit. Return
    /// statements compile each pending finalizer inline (TryExit + body)
    /// before the Return opcode so that finally blocks execute on return.
    fn_finalizer_stack: Vec<rusty_js_ast::Stmt>,
    /// FACL-EXT 1: true while compiling an inline-duplicated finally body
    /// for a break/continue/return. Prevents re-entrant finalizer emission
    /// when a return inside a finally block would otherwise recurse.
    in_finalizer_emission: bool,
    /// Tier-Ω.5.o: frames for LabelledStatement wrapping non-loop bodies
    /// (e.g. `outer: { ... break outer; }`). Loop labels live on the
    /// LoopFrame's `label` field instead.
    label_stack: Vec<LabelFrame>,
    /// Tier-Ω.5.o: pending label name to attach to the next pushed
    /// LoopFrame. Set by compile_stmt(Stmt::Labelled { body: <loop> })
    /// and cleared at frame-push by the loop's compile site.
    pending_label: Option<String>,
    /// Tier-Ω.5.c: each enclosing-function level's locals + accumulated
    /// upvalues, walked when resolving identifiers inside nested functions.
    /// Innermost outer is at the back. Empty at the top-level module.
    enclosing: Vec<EnclosingFrame>,
    /// Ω.5.P03.E2.enclosing-locals-rc: cached Rc snapshot of `self.locals`
    /// reused across nested function compiles. Invalidated (set to None)
    /// at every mutation of `self.locals` (alloc_local + scope-end name
    /// rewrites). Rebuilt lazily on next `locals_snapshot()` call.
    locals_snapshot: Option<Rc<Vec<LocalDescriptor>>>,
    /// This proto's own upvalue descriptors (only meaningful when this
    /// Compiler is compiling a nested function, i.e. enclosing.is_empty()
    /// is false).
    upvalues: Vec<UpvalueDescriptor>,
    /// Tier-Ω.5.f: class lowering context. Pushed when entering a class
    /// constructor / instance method / static method body. Read by
    /// Expr::Super and super(...) / super.method() lowerings to resolve
    /// the synthetic hidden bindings (`<super.ctor>` / `<super.proto>`)
    /// allocated by the class-emission site.
    class_stack: Vec<ClassFrame>,
    /// Counter for synthesizing unique local names across nested classes.
    class_seq: u32,
    /// Tier-Ω.5.b: ESM import bindings collected from the module's
    /// ImportDeclarations. Each binding allocates a local slot at the
    /// pre-body lowering step; references to the local name resolve to
    /// that slot via resolve_local. The runtime populates these slots
    /// from the imported module's namespace before run_frame_module.
    imports: Vec<ImportBinding>,
    /// Tier-Ω.5.b: ESM export bindings populated as ExportDeclarations are
    /// lowered. Filled lazily at compile time (Named export specifiers
    /// resolve their `local` -> slot at end-of-module).
    exports: Vec<ExportBinding>,
    /// Tier-Ω.5.h: re-export source dependencies (`from "..."` specifiers).
    reexport_sources: Vec<String>,
    /// Tier-Ω.5.IIIIIIII: side-effect ImportDeclaration specifiers
    /// (`import "X"` with no bindings).
    side_effect_imports: Vec<String>,
    /// Tier-Ω.5.b: snapshot of named local-or-default exports seen so far,
    /// pending slot lookup. For `export { name }` the slot is the local
    /// previously declared by `const name = ...` / `function name() {}`.
    /// Resolved at the end of compile_module.
    pending_named_exports: Vec<(String, String)>, // (exported, local_name)
    /// Tier-Ω.5.ee: names pre-allocated by the function-decl hoisting pass.
    /// VariableDecl compile path consults this to reuse the pre-allocated
    /// slot instead of allocating a fresh one. Cleared after the body pass.
    pre_allocated_slots: std::collections::HashMap<String, u16>,
    /// Ω.5.P51.E1: source-line index, populated once at top-level
    /// compile_module entry. Propagated to every CompiledModule and
    /// FunctionProto so runtime errors can derive line:col from any pc.
    source_line_starts: Vec<u32>,
    /// Ω.5.P51.E1: URL of the source being compiled. Propagated to
    /// every FunctionProto for closure-frame error enrichment.
    source_url: String,
    /// Ω.5.P53.E2: accumulator for per-construct probe tags emitted via
    /// emit_construct_tag(). Drained into CompiledModule.construct_tags
    /// or FunctionProto.construct_tags at compile-finish.
    construct_tags: Vec<(usize, &'static str)>,
    /// Ω.5.P52.E3: nesting depth of block-scoped constructs (Stmt::Block
    /// and friends). 0 = function-body top level; >0 inside any block.
    /// Gates pre_allocated_slots reuse for let/const: top-level let/const
    /// share Phase A's pre-allocated slot (necessary for closure capture
    /// across hoisted function decls — the chalk/supports-color pattern),
    /// but inside a nested block a fresh let/const MUST get a fresh slot
    /// per ES §13.2 lexical scope. Without the gate, an inner-block const
    /// reused the outer slot index from pre_allocated_slots, writing the
    /// inner value into the outer's slot and breaking shadow semantics.
    block_depth: u32,
    /// IR-EXT 31 (first piece of Constraint β ScopeRecord LIFT): per-block
    /// pre-allocated let/const slot maps, pushed on Stmt::Block entry and
    /// popped on exit. Stmt::Variable identifier branch consults the top
    /// of the stack to reuse a pre-allocated slot instead of allocating
    /// fresh. Enables block-scope TDZ enforcement: the slots are
    /// allocated + TDZ-seeded at block-entry, the decl line's InitLocal
    /// overwrites the sentinel.
    block_pre_slots: Vec<std::collections::HashMap<String, u16>>,
    /// IR-EXT 40: set by compile_class right before invoking
    /// compile_function_proto for a derived-class constructor body so
    /// the proto emits SetThisTDZ at body entry. Read + cleared at the
    /// start of compile_function_proto_with_name_hint so nested function
    /// or arrow compilations inside the ctor body don't inherit the
    /// flag. Avoids the class_stack-clone false-positive that
    /// IR-EXT 38/39 attempts hit.
    next_compile_is_derived_ctor: bool,
    /// WBMS-EXT 2: dynamic identifier mode for statement bodies executing
    /// inside one or more `with` object environments.
    with_depth: u32,
    /// EXT 73: strict-mode context for this compiler scope. Modules are
    /// always strict; a function body inherits the enclosing strictness
    /// and may upgrade itself to strict via a `"use strict"` directive
    /// prologue. Set before compiling the body; read at proto build time.
    strict: bool,
    /// ES-EXT 2 (eval-scope-binding-chain): when true, top-level `var`
    /// declarations route to `StoreGlobal` (against the realm's global
    /// object) instead of `StoreLocal`. Per ECMA-262 §19.2.1.3 PerformEval
    /// indirect-eval branch and §16.1 Scripts, a Script's variable env
    /// IS the global env — `eval("var foo = 42")` must attach `foo` to
    /// globalThis. Set via `set_script_mode(true)` before `compile_module`.
    /// Default false (Module semantics). See is_script_top_var().
    script_mode: bool,
}

#[derive(Debug, Clone)]
struct ClassFrame {
    /// Synthetic outer-local name holding the parent constructor (None
    /// when the class has no `extends` clause — super-references are a
    /// compile-time error in that case).
    super_ctor_name: Option<String>,
    /// Synthetic outer-local name holding the parent prototype.
    super_proto_name: Option<String>,
    /// Synthetic outer-local name holding an object-literal HomeObject.
    /// Object-method `super` resolves through HomeObject.[[Prototype]]
    /// at call time, so later Object.setPrototypeOf calls are visible.
    super_home_name: Option<String>,
    /// True inside the constructor body (only place where bare `super(...)`
    /// is valid). False inside instance / static methods.
    in_constructor: bool,
    /// True for static methods — bare `super(...)` not allowed; super.x
    /// resolves to the parent constructor, not the parent prototype.
    is_static: bool,
}

// Ω.5.P03.E2.enclosing-locals-rc: `locals` is held by `Rc` so a single
// snapshot of the parent's locals can be shared across every nested
// function compile inside a given parent. Pre-substrate this field was
// `Vec<LocalDescriptor>` and every call to `compile_function_proto_*`
// did `self.locals.clone()` — O(L) per child, O(L*N) for N siblings.
// The snapshot is read-only inside the sub-compiler (resolve_upvalue
// only iterates names). Parent mutations to `self.locals` invalidate
// the cached snapshot (see `Compiler::locals_snapshot`).
#[derive(Debug, Clone)]
struct EnclosingFrame {
    locals: Rc<Vec<LocalDescriptor>>,
    /// Upvalues that this enclosing frame itself captured. Needed when an
    /// inner function references a name owned by an even-outer level — the
    /// intermediate frames each get a transitive upvalue.
    upvalues: Vec<UpvalueDescriptor>,
}

fn emit_captures(buf: &mut Vec<u8>, captures: &[UpvalueDescriptor]) {
    for u in captures {
        match u.source {
            UpvalueSource::Local(slot) => {
                encode_op(buf, Op::CaptureLocal);
                encode_u16(buf, slot);
            }
            UpvalueSource::Upvalue(idx) => {
                encode_op(buf, Op::CaptureUpvalue);
                encode_u16(buf, idx);
            }
        }
    }
}

fn add_upvalue_to(table: &mut Vec<UpvalueDescriptor>, src: UpvalueSource, name: String) -> u16 {
    if let Some(i) = table.iter().position(|u| u.source == src) {
        return i as u16;
    }
    let idx = table.len() as u16;
    table.push(UpvalueDescriptor { source: src, name });
    idx
}

#[derive(Debug)]
struct LoopFrame {
    /// Bytecode offset where `continue` should jump to. For while / do-while
    /// this is fixed up front. For C-style for, the target is the update
    /// position which isn't known when the body compiles — `continue` then
    /// records a patch site in continue_patches instead of emitting a
    /// back-jump immediately.
    continue_target: usize,
    /// True while continue_target is provisional; `continue` records a
    /// patch site instead of emitting a known back-jump.
    continue_pending: bool,
    /// Operand-byte offsets of unresolved continue forward-jumps.
    continue_patches: Vec<usize>,
    /// Operand-byte offsets of unresolved break forward-jumps.
    break_patches: Vec<usize>,
    /// Tier-Ω.5.m: true for switch frames. `break` still targets this
    /// frame, but `continue` skips past it to the enclosing loop —
    /// switch is a break-only construct per ECMA-262 §14.12.4.
    is_switch: bool,
    /// FACL-EXT 1: count of active try blocks between this loop's entry
    /// and the current compilation point. Incremented at TryEnter inside
    /// the loop body; decremented at TryExit. When break/continue exits
    /// this loop, emit TryExit + inline finally bodies before the jump
    /// so that finally blocks execute per §14.15.3.
    try_depth: u32,
    /// FACL-EXT 1: pending finalizer AST nodes for active try-finally
    /// blocks enclosing this loop's body. Pushed at try-entry, popped
    /// at try-exit. On break/continue, each is compiled inline before
    /// the jump (duplicating the finally body in bytecode).
    pending_finalizers: Vec<rusty_js_ast::Stmt>,
    /// Tier-Ω.5.o: label name attached to this frame by an enclosing
    /// LabelledStatement. `break LABEL` / `continue LABEL` match the
    /// innermost frame with this label. None for unlabelled loops.
    label: Option<String>,
}

/// Tier-Ω.5.o: frame for a LabelledStatement wrapping a non-loop body.
/// Only `break LABEL` targets it; `continue LABEL` matches loop frames.
#[derive(Debug)]
struct LabelFrame {
    label: String,
    break_patches: Vec<usize>,
}

/// EXT 73: ECMA-262 §11.2.1 directive-prologue scan. A directive prologue
/// is the longest sequence of leading ExpressionStatements whose expression
/// is a StringLiteral. The "use strict" directive enables strict mode for
/// the function it heads. Returns true iff any directive in the prologue
/// has the canonical value `"use strict"` (case-sensitive, no escapes per
/// §11.2.1 — but for v1 we accept either spelling cheap-and-cheerfully).
fn directive_has_use_strict(body: &[Stmt]) -> bool {
    for stmt in body {
        match stmt {
            Stmt::Expression {
                expr: Expr::StringLiteral { value, .. },
                ..
            } => {
                if value == "use strict" {
                    return true;
                }
            }
            _ => return false,
        }
    }
    false
}

fn expr_contains_optional_chain(expr: &Expr) -> bool {
    match expr {
        Expr::Member {
            object, optional, ..
        } => *optional || expr_contains_optional_chain(object),
        Expr::Call {
            callee, optional, ..
        } => *optional || expr_contains_optional_chain(callee),
        Expr::Parenthesized { expr, .. }
        | Expr::Update { argument: expr, .. }
        | Expr::Unary { argument: expr, .. } => expr_contains_optional_chain(expr),
        Expr::Binary { left, right, .. }
        | Expr::Assign {
            target: left,
            value: right,
            ..
        } => expr_contains_optional_chain(left) || expr_contains_optional_chain(right),
        Expr::Conditional {
            test,
            consequent,
            alternate,
            ..
        } => {
            expr_contains_optional_chain(test)
                || expr_contains_optional_chain(consequent)
                || expr_contains_optional_chain(alternate)
        }
        Expr::Sequence { expressions, .. } | Expr::TemplateLiteral { expressions, .. } => {
            expressions.iter().any(expr_contains_optional_chain)
        }
        _ => false,
    }
}

fn body_contains_direct_eval(body: &[Stmt]) -> bool {
    body.iter().any(stmt_contains_direct_eval)
}

fn stmt_contains_direct_eval(stmt: &Stmt) -> bool {
    match stmt {
        Stmt::Variable(var) => var
            .declarators
            .iter()
            .any(|d| d.init.as_ref().is_some_and(expr_contains_direct_eval)),
        Stmt::Expression { expr, .. } => expr_contains_direct_eval(expr),
        Stmt::Block { body, .. } => body_contains_direct_eval(body),
        Stmt::FunctionDecl { .. } | Stmt::ClassDecl { .. } | Stmt::Empty { .. } => false,
        Stmt::If {
            test,
            consequent,
            alternate,
            ..
        } => {
            expr_contains_direct_eval(test)
                || stmt_contains_direct_eval(consequent)
                || alternate
                    .as_ref()
                    .is_some_and(|s| stmt_contains_direct_eval(s))
        }
        Stmt::For {
            init,
            test,
            update,
            body,
            ..
        } => {
            init.as_ref().is_some_and(for_init_contains_direct_eval)
                || test.as_ref().is_some_and(expr_contains_direct_eval)
                || update.as_ref().is_some_and(expr_contains_direct_eval)
                || stmt_contains_direct_eval(body)
        }
        Stmt::ForIn {
            left, right, body, ..
        }
        | Stmt::ForOf {
            left, right, body, ..
        } => {
            for_binding_contains_direct_eval(left)
                || expr_contains_direct_eval(right)
                || stmt_contains_direct_eval(body)
        }
        Stmt::While { test, body, .. } | Stmt::DoWhile { body, test, .. } => {
            expr_contains_direct_eval(test) || stmt_contains_direct_eval(body)
        }
        Stmt::With { object, body, .. } => {
            expr_contains_direct_eval(object) || stmt_contains_direct_eval(body)
        }
        Stmt::Switch {
            discriminant,
            cases,
            ..
        } => {
            expr_contains_direct_eval(discriminant)
                || cases.iter().any(|c| {
                    c.test.as_ref().is_some_and(expr_contains_direct_eval)
                        || body_contains_direct_eval(&c.consequent)
                })
        }
        Stmt::Try {
            block,
            handler,
            finalizer,
            ..
        } => {
            stmt_contains_direct_eval(block)
                || handler
                    .as_ref()
                    .is_some_and(|h| stmt_contains_direct_eval(&h.body))
                || finalizer
                    .as_ref()
                    .is_some_and(|s| stmt_contains_direct_eval(s))
        }
        Stmt::Return { argument, .. } => argument.as_ref().is_some_and(expr_contains_direct_eval),
        Stmt::Throw { argument, .. } => expr_contains_direct_eval(argument),
        Stmt::Break { .. }
        | Stmt::Continue { .. }
        | Stmt::Debugger { .. }
        | Stmt::Opaque { .. } => false,
        Stmt::Labelled { body, .. } => stmt_contains_direct_eval(body),
    }
}

fn for_init_contains_direct_eval(init: &ForInit) -> bool {
    match init {
        ForInit::Variable(var) => var
            .declarators
            .iter()
            .any(|d| d.init.as_ref().is_some_and(expr_contains_direct_eval)),
        ForInit::Expression(expr) => expr_contains_direct_eval(expr),
    }
}

fn for_binding_contains_direct_eval(binding: &ForBinding) -> bool {
    match binding {
        ForBinding::Decl { target, .. } | ForBinding::Pattern(target) => {
            binding_pattern_contains_direct_eval(target)
        }
        ForBinding::AssignmentTarget(expr) => expr_contains_direct_eval(expr),
    }
}

fn binding_pattern_contains_direct_eval(pattern: &BindingPattern) -> bool {
    match pattern {
        BindingPattern::Identifier(_) => false,
        BindingPattern::Array(array) => {
            array.elements.iter().flatten().any(|el| {
                binding_pattern_contains_direct_eval(&el.target)
                    || el.default.as_ref().is_some_and(expr_contains_direct_eval)
            }) || array
                .rest
                .as_ref()
                .is_some_and(|p| binding_pattern_contains_direct_eval(p))
        }
        BindingPattern::Object(object) => object.properties.iter().any(|prop| {
            matches!(&prop.key, PropertyKey::Computed(expr) if expr_contains_direct_eval(expr))
                || binding_pattern_contains_direct_eval(&prop.value.target)
                || prop
                    .value
                    .default
                    .as_ref()
                    .is_some_and(expr_contains_direct_eval)
        }),
    }
}

fn expr_contains_direct_eval(expr: &Expr) -> bool {
    match expr {
        Expr::Call {
            callee,
            arguments,
            optional,
            ..
        } => {
            (!*optional
                && matches!(callee.as_ref(), Expr::Identifier { name, .. } if name == "eval"))
                || expr_contains_direct_eval(callee)
                || arguments.iter().any(argument_contains_direct_eval)
        }
        Expr::Parenthesized { expr, .. }
        | Expr::Update { argument: expr, .. }
        | Expr::Unary { argument: expr, .. } => expr_contains_direct_eval(expr),
        Expr::Array { elements, .. } => elements.iter().any(|el| match el {
            ArrayElement::Elision { .. } => false,
            ArrayElement::Expr(expr) | ArrayElement::Spread { expr, .. } => {
                expr_contains_direct_eval(expr)
            }
        }),
        Expr::Object { properties, .. } => properties.iter().any(|prop| match prop {
            ObjectProperty::Property { key, value, .. } => {
                matches!(key, ObjectKey::Computed { expr, .. } if expr_contains_direct_eval(expr))
                    || expr_contains_direct_eval(value)
            }
            ObjectProperty::Spread { expr, .. } => expr_contains_direct_eval(expr),
        }),
        Expr::Member {
            object, property, ..
        } => {
            expr_contains_direct_eval(object)
                || matches!(property.as_ref(), MemberProperty::Computed { expr, .. } if expr_contains_direct_eval(expr))
        }
        Expr::New {
            callee, arguments, ..
        } => {
            expr_contains_direct_eval(callee) || arguments.iter().any(argument_contains_direct_eval)
        }
        Expr::Binary { left, right, .. }
        | Expr::Assign {
            target: left,
            value: right,
            ..
        } => expr_contains_direct_eval(left) || expr_contains_direct_eval(right),
        Expr::Conditional {
            test,
            consequent,
            alternate,
            ..
        } => {
            expr_contains_direct_eval(test)
                || expr_contains_direct_eval(consequent)
                || expr_contains_direct_eval(alternate)
        }
        Expr::Sequence { expressions, .. } | Expr::TemplateLiteral { expressions, .. } => {
            expressions.iter().any(expr_contains_direct_eval)
        }
        Expr::Class {
            super_class,
            members,
            ..
        } => {
            super_class
                .as_ref()
                .is_some_and(|e| expr_contains_direct_eval(e))
                || members
                    .iter()
                    .any(class_member_key_or_field_contains_direct_eval)
        }
        Expr::Function { .. } | Expr::Arrow { .. } => false,
        Expr::NullLiteral { .. }
        | Expr::BoolLiteral { .. }
        | Expr::NumberLiteral { .. }
        | Expr::BigIntLiteral { .. }
        | Expr::StringLiteral { .. }
        | Expr::Identifier { .. }
        | Expr::This { .. }
        | Expr::Super { .. }
        | Expr::MetaProperty { .. }
        | Expr::TemplateObject { .. }
        | Expr::RegExp { .. }
        | Expr::Opaque { .. } => false,
    }
}

fn argument_contains_direct_eval(argument: &Argument) -> bool {
    match argument {
        Argument::Expr(expr) | Argument::Spread { expr, .. } => expr_contains_direct_eval(expr),
    }
}

fn class_member_key_or_field_contains_direct_eval(member: &ClassMember) -> bool {
    match member {
        ClassMember::Method { name, .. } => class_member_name_contains_direct_eval(name),
        ClassMember::Field { name, init, .. } => {
            class_member_name_contains_direct_eval(name)
                || init.as_ref().is_some_and(expr_contains_direct_eval)
        }
        ClassMember::StaticBlock { body, .. } => body_contains_direct_eval(body),
    }
}

fn class_member_name_contains_direct_eval(name: &ClassMemberName) -> bool {
    matches!(name, ClassMemberName::Computed { expr, .. } if expr_contains_direct_eval(expr))
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            bytecode: Vec::new(),
            constants: ConstantsPool::new(),
            locals: Vec::new(),
            source_map: Vec::new(),
            loop_stack: Vec::new(),
            fn_finalizer_stack: Vec::new(),
            in_finalizer_emission: false,
            label_stack: Vec::new(),
            pending_label: None,
            enclosing: Vec::new(),
            upvalues: Vec::new(),
            class_stack: Vec::new(),
            class_seq: 0,
            imports: Vec::new(),
            exports: Vec::new(),
            reexport_sources: Vec::new(),
            side_effect_imports: Vec::new(),
            pending_named_exports: Vec::new(),
            pre_allocated_slots: std::collections::HashMap::new(),
            source_line_starts: Vec::new(),
            source_url: String::new(),
            block_depth: 0,
            block_pre_slots: Vec::new(),
            next_compile_is_derived_ctor: false,
            with_depth: 0,
            construct_tags: Vec::new(),
            strict: false,
            locals_snapshot: None,
            script_mode: false,
        }
    }

    /// ES-EXT 2 (eval-scope-binding-chain): mark this Compiler as compiling
    /// a Script (per ECMA-262 §16.1) rather than a Module (§16.2). The flag
    /// is consulted at the top-level VariableStatement compile path; for
    /// `var` declarations at module-top-level (block_depth == 0 and not
    /// inside an enclosing function), Script mode skips local-slot
    /// allocation and emits `StoreGlobal` so the binding attaches to the
    /// realm's global object — matching §19.2.1.3 PerformEval's indirect-
    /// eval branch where the global env IS the variable env.
    ///
    /// Only `var` is rerouted; `let`/`const` remain lexical (live in the
    /// realm's Lexical Environment, not GlobalThis) per §16.1 ScriptBody.
    /// Nested function bodies always get a fresh sub-Compiler with
    /// script_mode default-false, so function-local `var` keeps its
    /// standard semantics.
    pub fn set_script_mode(&mut self, m: bool) {
        self.script_mode = m;
    }

    /// ES-EXT 2 helper: at a Stmt::Variable compile site, does the
    /// declaration qualify as a Script-top-level `var` that should
    /// attach to globalThis instead of getting a local slot?
    fn is_script_top_var(&self, kind: rusty_js_ast::VariableKind) -> bool {
        self.script_mode
            && self.enclosing.is_empty()
            && self.block_depth == 0
            && matches!(kind, rusty_js_ast::VariableKind::Var)
    }

    /// Ω.5.P03.E2.enclosing-locals-rc: get-or-build the shared snapshot of
    /// `self.locals` for handing to a nested function compile. The Rc is
    /// reused across all sibling sub-compiles until `self.locals` is next
    /// mutated (alloc_local or scope-end name rewrite). Converts the
    /// per-child O(L) clone into an O(L) build amortized across all
    /// children that don't intervene with a parent-locals mutation —
    /// the dominant pattern in bundled CJS output, where N sibling
    /// `function (){…}` expressions are emitted between parent
    /// alloc_locals (often zero between them).
    fn locals_snapshot(&mut self) -> Rc<Vec<LocalDescriptor>> {
        if let Some(snap) = &self.locals_snapshot {
            if snap.len() == self.locals.len() {
                return Rc::clone(snap);
            }
        }
        let snap = Rc::new(self.locals.clone());
        self.locals_snapshot = Some(Rc::clone(&snap));
        snap
    }

    /// Invalidate the cached locals snapshot. Call after any mutation to
    /// `self.locals` (push, in-place name rewrite, or replace).
    fn invalidate_locals_snapshot(&mut self) {
        self.locals_snapshot = None;
    }

    /// Ω.5.P53.E2: mark the current bytecode offset with an AST-construct
    /// tag. Runtime error enrichment looks up the most recent tag at the
    /// failing pc to attribute faults to a named construct ("optional-chain
    /// member", "optional-chain call", etc.). Light-weight probe; one tag
    /// per construct entry, no per-op tracking.
    fn emit_construct_tag(&mut self, name: &'static str) {
        let off = self.bytecode.len();
        // Dedupe consecutive identical tags at the same offset (nested
        // constructs that share an entry point only need one tag).
        if let Some(&(prev_off, prev_name)) = self.construct_tags.last() {
            if prev_off == off && prev_name == name {
                return;
            }
        }
        self.construct_tags.push((off, name));
    }

    /// Ω.5.P51.E1: store the source URL for this compilation. Threaded to
    /// every FunctionProto for closure-frame error enrichment.
    pub fn set_source_url(&mut self, url: String) {
        self.source_url = url;
    }

    /// Ω.5.P51.E1: store the source-line index for this compilation. Called
    /// from the top-level compile_module(src) entry before c.compile_module(ast).
    /// The index is moved (not cloned) onto the resulting CompiledModule and
    /// every FunctionProto produced under this Compiler, threading runtime
    /// error enrichment with line:col data without re-scanning source.
    pub fn set_source_line_starts(&mut self, line_starts: Vec<u32>) {
        self.source_line_starts = line_starts;
    }

    pub fn compile_module(&mut self, m: &Module) -> Result<CompiledModule, CompileError> {
        // EXT 73: ECMA-262 §11.2.1 directive-prologue scan at the top level.
        // A real module is always strict (§11.2.2), and a script becomes
        // strict iff it opens with a `"use strict"` directive. cruftless
        // routes both through this same entry, so we only auto-enable
        // strict when the top of the body actually carries the directive
        // OR when imports/exports are present (a syntactic module).
        let has_module_syntax = m
            .body
            .iter()
            .any(|i| matches!(i, ModuleItem::Import(_) | ModuleItem::Export(_)));
        let leading_stmts: Vec<Stmt> = m
            .body
            .iter()
            .filter_map(|i| {
                if let ModuleItem::Statement(s) = i {
                    Some(s.clone())
                } else {
                    None
                }
            })
            .collect();
        // Ω.5.P05.L0.module-mjs-strict: .mjs files are ECMAScript modules
        // by file-extension convention (Node + browsers + Bun all enforce
        // this); ES modules are strict by default per ECMA-262 §15.5
        // ModuleDeclarationLinking step 9 (Module Environment Records
        // run code in strict mode). Pre-substrate, cruftless only enabled
        // strict mode when the body either carried a "use strict"
        // directive or contained import/export syntax — a .mjs file
        // without imports (e.g. a pure-script .mjs probe) ran as sloppy
        // mode, producing `this = globalThis` for unbound method calls
        // where the spec mandates `this = undefined`.
        let is_mjs_url = self.source_url.ends_with(".mjs");
        // Ω.5.P05.L0.cjs-wrapper-sloppy-default: cruftless wraps each
        // CJS module body in `export default (function (exports, module,
        // require, __filename, __dirname) { <source> })` for bytecode-
        // delivery purposes (see evaluate_cjs_module). That outer wrapper
        // is an ES module syntactically (it has `export default`), which
        // pre-substrate triggered the has_module_syntax → strict
        // promotion. Per spec, CJS bodies in Node default to SLOPPY
        // mode unless the body itself opens with a "use strict"
        // directive prologue. The inner function's compile then picks
        // strict up from its own body if the prologue is present.
        // Detect the CJS wrapper by exact-signature match on the
        // generated wrapping form, and override strict=false for the
        // outer module. The inner function's own directive-prologue
        // scan (compile_function_proto_with_name_hint:strict) continues
        // to flip strict on if the body opts in via "use strict".
        //
        // Without this, packages whose CJS bodies depend on sloppy-mode
        // global creation (icalendar's bare `PROP_NAME = 0` at module
        // top; ethereumjs-* family; many older npm packages) crashed on
        // ReferenceError after Ω.5.P04.E2.strict-write-enforcement.
        let is_cjs_wrapper = m.body.len() == 1
            && match m.body.first() {
                Some(ModuleItem::Export(rusty_js_ast::ExportDeclaration::Default {
                    body: rusty_js_ast::DefaultExportBody::Expression { expr },
                    ..
                })) => {
                    // Peel one Parenthesized layer — the wrap is
                    // `export default (function ...)` and the parser
                    // preserves the parens around the function expression.
                    let inner = match expr {
                        rusty_js_ast::Expr::Parenthesized { expr: e, .. } => e.as_ref(),
                        other => other,
                    };
                    matches!(inner, rusty_js_ast::Expr::Function { params, .. }
                    if params.len() == 5
                    && params.iter().zip(["exports","module","require","__filename","__dirname"].iter())
                        .all(|(p, expected)| matches!(&p.target,
                            rusty_js_ast::BindingPattern::Identifier(id) if id.name == **expected)))
                }
                _ => false,
            };
        self.strict = (!is_cjs_wrapper)
            && (is_mjs_url || has_module_syntax || directive_has_use_strict(&leading_stmts));
        // Tier-Ω.5.b phase A: pre-allocate locals for every import binding
        // so references to imported names in the body resolve to LoadLocal
        // (not LoadGlobal). The runtime populates these slots before
        // run_frame_module by reading from each module-request's namespace.
        for item in &m.body {
            if let ModuleItem::Import(imp) = item {
                let module_request = imp.specifier.value.clone();
                // Tier-Ω.5.IIIIIIII: track side-effect imports for evaluation.
                let has_bindings = imp.default_binding.is_some()
                    || imp.namespace_binding.is_some()
                    || !imp.named_imports.is_empty();
                if !has_bindings {
                    self.side_effect_imports.push(module_request.clone());
                }
                if let Some(def) = &imp.default_binding {
                    let slot = self.alloc_local(LocalDescriptor {
                        name: def.name.clone(),
                        kind: VariableKind::Const,
                        depth: 0,
                    });
                    self.imports.push(ImportBinding {
                        slot,
                        module_request: module_request.clone(),
                        kind: ImportBindingKind::Default,
                    });
                }
                if let Some(ns) = &imp.namespace_binding {
                    let slot = self.alloc_local(LocalDescriptor {
                        name: ns.name.clone(),
                        kind: VariableKind::Const,
                        depth: 0,
                    });
                    self.imports.push(ImportBinding {
                        slot,
                        module_request: module_request.clone(),
                        kind: ImportBindingKind::Namespace,
                    });
                }
                for spec in &imp.named_imports {
                    let imported_name = match &spec.imported {
                        ModuleExportName::Ident(b) => b.name.clone(),
                        ModuleExportName::String { value, .. } => value.clone(),
                    };
                    let slot = self.alloc_local(LocalDescriptor {
                        name: spec.local.name.clone(),
                        kind: VariableKind::Const,
                        depth: 0,
                    });
                    self.imports.push(ImportBinding {
                        slot,
                        module_request: module_request.clone(),
                        kind: ImportBindingKind::Named(imported_name),
                    });
                }
            }
        }

        // Phase A.5: function-declaration hoisting (Tier-Ω.5.ee). Per
        // ECMA-262 §10.2.1.3 / §14.1.22, FunctionDeclaration is hoisted to
        // the top of the enclosing function (or module). The dense CJS
        // idiom `exports = module.exports = objectHash; ... function
        // objectHash() {...}` depends on this. Pre-allocate the function's
        // local slot AND emit MakeClosure + StoreLocal so the name is bound
        // before any other statement runs.
        // Tier-Ω.5.zz / Ω.5.aaa: three-phase module-level hoisting.
        //
        //   A.4 pre-allocates slots for ALL top-level bindings —
        //        function-decl names AND const/let/var names (incl.
        //        names under `export ...`). Must run BEFORE any
        //        function body is compiled, otherwise a hoisted
        //        function-decl body that references a later top-level
        //        `var X = ...` resolves X as a free global rather than
        //        a local upvalue (acorn's `function binop(){ return
        //        new TokenType() }` over `var TokenType = ...` failed
        //        because TokenType's slot didn't yet exist).
        //
        //   A.5 (this block) compiles each function-decl body and
        //        emits MakeClosure + StoreLocal into its A.4 slot.
        //
        //   Phase A.6 (Ω.5.qq) is now folded into A.4.
        let mut fn_pre_slots: std::collections::HashMap<String, u16> =
            std::collections::HashMap::new();
        // Helper: collect function-decl name from a body item, including
        // export-wrapped form `export function f(){}`.
        for item in &m.body {
            // Function-decl names. Tier-Ω.5.mmm: also recognize
            // `export function f(){}` so f's slot is pre-allocated before
            // f's body compiles — required for self-recursion to capture
            // f as a local upvalue rather than a missing global.
            let fn_name: Option<&BindingIdentifier> = match item {
                ModuleItem::Statement(Stmt::FunctionDecl { name: Some(n), .. }) => Some(n),
                ModuleItem::Export(ExportDeclaration::Declaration {
                    decl_stmt: Some(stmt),
                    ..
                }) => {
                    if let Stmt::FunctionDecl { name: Some(n), .. } = stmt.as_ref() {
                        Some(n)
                    } else {
                        None
                    }
                }
                _ => None,
            };
            if let Some(n) = fn_name {
                if !fn_pre_slots.contains_key(&n.name) {
                    let slot = self.alloc_local(LocalDescriptor {
                        name: n.name.clone(),
                        kind: VariableKind::Var,
                        depth: 0,
                    });
                    fn_pre_slots.insert(n.name.clone(), slot);
                }
                continue;
            }
            // Tier-Ω.5.qqq: also pre-allocate class-decl names so a
            // top-level arrow that references a later-declared class
            // resolves it as a local upvalue rather than a missing
            // global. minimatch's `export const minimatch = (...) =>
            // { ... new Minimatch(...) ... }` over `export class
            // Minimatch` depends on this. Classes evaluate at their
            // declaration point in Phase B, so the slot is only
            // pre-allocated here (no MakeClosure / class-build emit).
            let class_name: Option<&BindingIdentifier> = match item {
                ModuleItem::Statement(Stmt::ClassDecl { name: Some(n), .. }) => Some(n),
                ModuleItem::Export(ExportDeclaration::Declaration {
                    decl_stmt: Some(stmt),
                    ..
                }) => {
                    if let Stmt::ClassDecl { name: Some(n), .. } = stmt.as_ref() {
                        Some(n)
                    } else {
                        None
                    }
                }
                _ => None,
            };
            if let Some(n) = class_name {
                if !self.pre_allocated_slots.contains_key(&n.name)
                    && !fn_pre_slots.contains_key(&n.name)
                    && self.resolve_local(&n.name).is_none()
                {
                    let slot = self.alloc_local(LocalDescriptor {
                        name: n.name.clone(),
                        kind: VariableKind::Let,
                        depth: 0,
                    });
                    self.pre_allocated_slots.insert(n.name.clone(), slot);
                }
                continue;
            }
            // Top-level variable bindings (incl. under `export`).
            let v_opt: Option<&rusty_js_ast::VariableStatement> = match item {
                ModuleItem::Statement(Stmt::Variable(v)) => Some(v),
                ModuleItem::Export(ExportDeclaration::Declaration {
                    decl_stmt: Some(stmt),
                    ..
                }) => {
                    if let Stmt::Variable(v) = stmt.as_ref() {
                        Some(v)
                    } else {
                        None
                    }
                }
                _ => None,
            };
            if let Some(v) = v_opt {
                // ES-EXT 2 v2 (post-GBSU): pre-allocation passes UNCHANGED in
                // script_mode. IC.1 (top-level inner-function upvalue capture)
                // requires the local slot to exist; the v1 skip violated this
                // and caused the 33.2% regression. v2 keeps the local slot
                // AND emits an additional StoreGlobal at declaration to attach
                // the binding to globalThis per ECMA-262 §16.1.
                for d in &v.declarators {
                    // Tier-Ω.5.dddd: pre-allocate every identifier the
                    // declarator's pattern binds, including destructure
                    // patterns ({a,b} = ..., [a,b] = ...). chalk's
                    // supports-color uses `const {env} = process;`
                    // followed by a function-decl that references `env`
                    // as upvalue — without pre-allocation, the function's
                    // body resolved `env` as a missing global.
                    for id in d.target.collect_names() {
                        if !self.pre_allocated_slots.contains_key(&id.name)
                            && !fn_pre_slots.contains_key(&id.name)
                            && self.resolve_local(&id.name).is_none()
                        {
                            let slot = self.alloc_local(LocalDescriptor {
                                name: id.name.clone(),
                                kind: v.kind,
                                depth: 0,
                            });
                            self.pre_allocated_slots.insert(id.name.clone(), slot);
                        }
                    }
                }
            }
        }
        self.pre_allocated_slots
            .extend(fn_pre_slots.iter().map(|(k, v)| (k.clone(), *v)));
        // IR-EXT 34 (re-attempt of EXT 29 with EXT 31-33 substrate now in
        // place): module/script top-level TDZ for let/const. Mirrors the
        // function-body Phase H1.5 emit + block-scope EXT 31 emit. Seed
        // each pre-allocated let/const slot with TDZ sentinel.
        for (_name, slot) in self.pre_allocated_slots.clone().iter() {
            let kind = self
                .locals
                .get(*slot as usize)
                .map(|d| d.kind)
                .unwrap_or(VariableKind::Var);
            if matches!(kind, VariableKind::Let | VariableKind::Const) {
                encode_op(&mut self.bytecode, Op::PushTDZ);
                encode_op(&mut self.bytecode, Op::InitLocal);
                encode_u16(&mut self.bytecode, *slot);
            }
        }
        for item in &m.body {
            // Pull the inner FunctionDecl out (whether direct or
            // wrapped under `export function f(){}`).
            let fn_parts: Option<(
                &Option<BindingIdentifier>,
                bool,
                bool,
                &Vec<Parameter>,
                &Vec<Stmt>,
            )> = match item {
                ModuleItem::Statement(Stmt::FunctionDecl {
                    name,
                    is_async,
                    is_generator,
                    params,
                    body,
                    ..
                }) => Some((name, *is_async, *is_generator, params, body)),
                ModuleItem::Export(ExportDeclaration::Declaration {
                    decl_stmt: Some(stmt),
                    ..
                }) => {
                    if let Stmt::FunctionDecl {
                        name,
                        is_async,
                        is_generator,
                        params,
                        body,
                        ..
                    } = stmt.as_ref()
                    {
                        Some((name, *is_async, *is_generator, params, body))
                    } else {
                        None
                    }
                }
                _ => None,
            };
            if let Some((name, is_async, is_generator, params, body)) = fn_parts {
                if let Some(n) = name {
                    let proto = self.compile_function_proto(
                        Some(n.clone()),
                        is_async,
                        is_generator,
                        params,
                        body,
                    )?;
                    let captures = proto.upvalues.clone();
                    let idx = self.constants.intern(Constant::Function(Box::new(proto)));
                    encode_op(&mut self.bytecode, Op::MakeClosure);
                    encode_u16(&mut self.bytecode, idx);
                    emit_captures(&mut self.bytecode, &captures);
                    let slot = *fn_pre_slots
                        .get(&n.name)
                        .expect("function-decl slot pre-allocated above");
                    encode_op(&mut self.bytecode, Op::StoreLocal);
                    encode_u16(&mut self.bytecode, slot);
                }
            }
        }

        // Phase A.6 (Tier-Ω.5.qq): pre-allocate slots for top-level
        // let/const/var bindings (including those under `export`). Without
        // this, an arrow defined earlier that references a later top-level
        // const captures it as a free name rather than a local upvalue, so
        // the call observes undefined. arktype's @ark/util/strings.js
        // depends on this (anchoredRegex references anchoredSource declared
        // two lines below).
        for item in &m.body {
            match item {
                ModuleItem::Statement(Stmt::Variable(v)) => {
                    // ES-EXT 2 v2: pre-allocation UNCHANGED in script_mode
                    // (IC.1 protected). The global attachment is emitted
                    // additionally at the Stmt::Variable identifier branch.
                    for d in &v.declarators {
                        if let rusty_js_ast::BindingPattern::Identifier(id) = &d.target {
                            if !self.pre_allocated_slots.contains_key(&id.name)
                                && self.resolve_local(&id.name).is_none()
                            {
                                let slot = self.alloc_local(LocalDescriptor {
                                    name: id.name.clone(),
                                    kind: v.kind,
                                    depth: 0,
                                });
                                self.pre_allocated_slots.insert(id.name.clone(), slot);
                            }
                        }
                    }
                }
                ModuleItem::Export(ExportDeclaration::Declaration {
                    decl_stmt: Some(stmt),
                    ..
                }) => {
                    if let Stmt::Variable(v) = stmt.as_ref() {
                        for d in &v.declarators {
                            if let rusty_js_ast::BindingPattern::Identifier(id) = &d.target {
                                if !self.pre_allocated_slots.contains_key(&id.name)
                                    && self.resolve_local(&id.name).is_none()
                                {
                                    let slot = self.alloc_local(LocalDescriptor {
                                        name: id.name.clone(),
                                        kind: v.kind,
                                        depth: 0,
                                    });
                                    self.pre_allocated_slots.insert(id.name.clone(), slot);
                                }
                            }
                        }
                    }
                }
                _ => {}
            }
        }

        // VHTB-EXT 1: per ECMA-262 §15.2.10 VarScopedDeclarations, `var`
        // declarations hoist to the enclosing function/script scope across
        // every non-function syntactic boundary (try/catch/finally, if,
        // for, for-in, for-of, while, do-while, switch, block, labelled).
        // The phase-A.6 walk above only covers TOP-LEVEL Stmt::Variable;
        // a `var` inside `try { for(...){ var x = ... } } catch{}` was
        // not pre-allocated, so post-loop reads of `x` (which §13.3.2
        // requires to be resolvable as Undefined) resolved as
        // unresolvable. REOU-EXT 1 made that path throw ReferenceError,
        // which surfaced the bug at 2 test262 fixtures
        // (for-in/S12.6.4_A{1,2}). Recursive walker fixes it at the
        // hoisting tier (the right tier per spec).
        let mut nested_var_names: Vec<(String, rusty_js_ast::VariableKind)> = Vec::new();
        for item in &m.body {
            match item {
                ModuleItem::Statement(s) => collect_hoisted_var_names(s, &mut nested_var_names),
                ModuleItem::Export(ExportDeclaration::Declaration {
                    decl_stmt: Some(stmt),
                    ..
                }) => collect_hoisted_var_names(stmt.as_ref(), &mut nested_var_names),
                _ => {}
            }
        }
        for (name, kind) in nested_var_names {
            if !self.pre_allocated_slots.contains_key(&name) && self.resolve_local(&name).is_none()
            {
                let slot = self.alloc_local(LocalDescriptor {
                    name: name.clone(),
                    kind,
                    depth: 0,
                });
                self.pre_allocated_slots.insert(name, slot);
            }
        }

        // Phase B: walk the body in order. Imports already lowered.
        // Statements compile normally. Exports are recorded for phase C
        // (default-export expressions are lowered inline into a synthetic
        // "<module.default>" local). FunctionDecl statements are skipped
        // here because they were hoisted in Phase A.5 above.
        for item in &m.body {
            match item {
                ModuleItem::Import(_) => { /* lowered in phase A */ }
                ModuleItem::Statement(Stmt::FunctionDecl { .. }) => { /* hoisted in A.5 */ }
                ModuleItem::Statement(s) => self.compile_stmt(s)?,
                ModuleItem::Export(e) => self.compile_export(e)?,
            }
        }

        // Phase C: resolve pending named-export specifiers to slot indices.
        // For `export { name }` after a local declaration, the slot is the
        // local previously bound by the declaration.
        for (exported, local_name) in std::mem::take(&mut self.pending_named_exports) {
            if let Some(slot) = self.resolve_local(&local_name) {
                self.exports.push(ExportBinding::Local {
                    exported,
                    local: slot,
                });
            }
            // Silently drop unresolved names; the namespace builder yields
            // Undefined for missing exports.
        }

        encode_op(&mut self.bytecode, Op::ReturnUndef);
        Ok(CompiledModule {
            bytecode: std::mem::take(&mut self.bytecode),
            constants: std::mem::take(&mut self.constants),
            locals: std::mem::take(&mut self.locals),
            source_map: std::mem::take(&mut self.source_map),
            imports: std::mem::take(&mut self.imports),
            exports: std::mem::take(&mut self.exports),
            reexport_sources: std::mem::take(&mut self.reexport_sources),
            side_effect_imports: std::mem::take(&mut self.side_effect_imports),
            construct_tags: std::mem::take(&mut self.construct_tags),
            line_starts: std::mem::take(&mut self.source_line_starts),
            strict: self.strict,
            eval_var_env_is_global: self.script_mode,
        })
    }

    /// Tier-Ω.5.b: lower one ExportDeclaration. Named local exports are
    /// recorded for end-of-module slot resolution. Default exports lower
    /// the underlying expression / hoistable-function / class to bytecode
    /// and store the result in the synthetic "<module.default>" local.
    /// Re-export forms (StarFrom / StarAsFrom / Named-with-source) are
    /// deferred to a follow-on round per the scope ceiling.
    fn compile_export(&mut self, e: &ExportDeclaration) -> Result<(), CompileError> {
        match e {
            ExportDeclaration::Named {
                specifiers,
                source: None,
                ..
            } => {
                for spec in specifiers {
                    let local_name = match &spec.local {
                        ModuleExportName::Ident(b) => b.name.clone(),
                        ModuleExportName::String { value, .. } => value.clone(),
                    };
                    let exported_name = match &spec.exported {
                        ModuleExportName::Ident(b) => b.name.clone(),
                        ModuleExportName::String { value, .. } => value.clone(),
                    };
                    self.pending_named_exports.push((exported_name, local_name));
                }
            }
            // Tier-Ω.5.h: re-export forms. Each records its source-module
            // specifier in `reexport_sources` so the runtime loads the
            // dependency eagerly, then emits one or more ExportBinding
            // entries that the namespace-build phase resolves against the
            // source module's namespace.
            ExportDeclaration::Named {
                source: Some(src),
                specifiers,
                ..
            } => {
                let source_specifier = src.value.clone();
                if !self.reexport_sources.iter().any(|s| s == &source_specifier) {
                    self.reexport_sources.push(source_specifier.clone());
                }
                for spec in specifiers {
                    let imported = match &spec.local {
                        ModuleExportName::Ident(b) => b.name.clone(),
                        ModuleExportName::String { value, .. } => value.clone(),
                    };
                    let exported = match &spec.exported {
                        ModuleExportName::Ident(b) => b.name.clone(),
                        ModuleExportName::String { value, .. } => value.clone(),
                    };
                    self.exports.push(ExportBinding::Named {
                        exported,
                        source_specifier: source_specifier.clone(),
                        imported,
                    });
                }
            }
            ExportDeclaration::StarFrom { source, .. } => {
                let source_specifier = source.value.clone();
                if !self.reexport_sources.iter().any(|s| s == &source_specifier) {
                    self.reexport_sources.push(source_specifier.clone());
                }
                self.exports.push(ExportBinding::Star { source_specifier });
            }
            ExportDeclaration::StarAsFrom {
                source, exported, ..
            } => {
                let source_specifier = source.value.clone();
                if !self.reexport_sources.iter().any(|s| s == &source_specifier) {
                    self.reexport_sources.push(source_specifier.clone());
                }
                let exported_name = match exported {
                    ModuleExportName::Ident(b) => b.name.clone(),
                    ModuleExportName::String { value, .. } => value.clone(),
                };
                self.exports.push(ExportBinding::StarAs {
                    exported: exported_name,
                    source_specifier,
                });
            }
            ExportDeclaration::Default { body, span } => {
                // Synthesize a local slot for the default binding. Reuse
                // across modules with multiple defaults isn't legal ECMAScript,
                // but we accept duplicate slot allocation (the last write wins).
                let slot = self.alloc_local(LocalDescriptor {
                    name: "<module.default>".to_string(),
                    kind: VariableKind::Const,
                    depth: 0,
                });
                match body {
                    DefaultExportBody::Expression { expr } => {
                        self.compile_expr(expr)?;
                    }
                    DefaultExportBody::HoistableFunction {
                        name,
                        is_async,
                        is_generator,
                        params,
                        body,
                    } => {
                        let proto = self.compile_function_proto(
                            name.clone(),
                            *is_async,
                            *is_generator,
                            params,
                            body,
                        )?;
                        let captures = proto.upvalues.clone();
                        let idx = self.constants.intern(Constant::Function(Box::new(proto)));
                        encode_op(&mut self.bytecode, Op::MakeClosure);
                        encode_u16(&mut self.bytecode, idx);
                        emit_captures(&mut self.bytecode, &captures);
                        // 2026-05-24: per ECMA-262 §16.2.3.6, `export
                        // default function NAME(...) {...}` ALSO binds
                        // NAME in the module's lexical environment, in
                        // addition to making the function the default
                        // export. Without this, subsequent module-scope
                        // references like `NAME.prop = ...` see NAME as
                        // undefined. Surfaced via TXC long-tail
                        // (ajv/quote.ts: `quote.code = '...'`).
                        if let Some(n) = name {
                            let name_str = n.name.clone();
                            let name_slot = if let Some(s) = self.resolve_local(&name_str) {
                                s
                            } else {
                                self.alloc_local(LocalDescriptor {
                                    name: name_str,
                                    kind: VariableKind::Var,
                                    depth: 0,
                                })
                            };
                            encode_op(&mut self.bytecode, Op::Dup);
                            encode_op(&mut self.bytecode, Op::StoreLocal);
                            encode_u16(&mut self.bytecode, name_slot);
                        }
                    }
                    DefaultExportBody::Class {
                        name,
                        super_class,
                        members,
                    } => {
                        // Tier-Ω.5.v: lower `export default class [Name?] ...`
                        // by synthesizing a class expression and letting the
                        // existing compile_expr path emit it; the resulting
                        // value is then stored into the module's default slot
                        // (the StoreLocal below).
                        let class_expr = Expr::Class {
                            name: name.clone(),
                            super_class: super_class.clone().map(Box::new),
                            members: members.clone(),
                            span: *span,
                        };
                        self.compile_expr(&class_expr)?;
                    }
                }
                encode_op(&mut self.bytecode, Op::StoreLocal);
                encode_u16(&mut self.bytecode, slot);
                self.exports.push(ExportBinding::Local {
                    exported: "default".to_string(),
                    local: slot,
                });
            }
            ExportDeclaration::Declaration {
                names, decl_stmt, ..
            } => {
                // Tier-Ω.5.kk: if the parser captured the typed inner
                // declaration, compile it as a normal statement so its
                // initializers / function bodies / class bodies run and
                // bind their slots. arktype + @ark/util need this so
                // `export const noSuggest = (s) => ...` actually creates
                // the binding's value rather than leaving the slot at
                // undefined (Ω.5.ii fixed slot allocation but not
                // initializer execution).
                if let Some(stmt) = decl_stmt {
                    self.compile_stmt(stmt)?;
                } else {
                    // Fallback path: alloc slots so references resolve as
                    // locals (matches Ω.5.ii). Initializers were discarded.
                    for n in names {
                        if self.resolve_local(&n.name).is_none() {
                            self.alloc_local(LocalDescriptor {
                                name: n.name.clone(),
                                kind: VariableKind::Var,
                                depth: 0,
                            });
                        }
                    }
                }
                for n in names {
                    self.pending_named_exports
                        .push((n.name.clone(), n.name.clone()));
                }
            }
        }
        Ok(())
    }

    fn compile_stmt(&mut self, s: &Stmt) -> Result<(), CompileError> {
        let span = s.span();
        self.record_span(span);
        match s {
            Stmt::Expression { expr, .. } => {
                self.compile_expr(expr)?;
                encode_op(&mut self.bytecode, Op::Pop);
            }
            Stmt::Return { argument, .. } => {
                if let Some(e) = argument {
                    self.compile_expr(e)?;
                } else {
                    encode_op(&mut self.bytecode, Op::PushUndef);
                }
                if !self.in_finalizer_emission {
                    let finalizers = self.fn_finalizer_stack.clone();
                    self.in_finalizer_emission = true;
                    for fin in finalizers.iter().rev() {
                        encode_op(&mut self.bytecode, Op::TryExit);
                        self.compile_stmt(fin)?;
                    }
                    self.in_finalizer_emission = false;
                }
                encode_op(&mut self.bytecode, Op::Return);
            }
            Stmt::Empty { .. } => {}
            Stmt::Block { body, .. } => {
                // Tier-Ω.5.ttttt: block-scope let/const. Snapshot the
                // local-table depth on entry; on exit, rename locals
                // added inside the block so resolve_local no longer
                // matches them. Slot is preserved (closures that
                // captured them still find their cell), but later
                // identifier resolution falls through to upvalue /
                // global as if the binding had gone out of scope.
                let snapshot = self.locals.len();
                self.block_depth += 1;
                // IR-EXT 31: block-scope TDZ. Walk body for top-level
                // let/const Identifier declarators; pre-allocate slots;
                // emit PushTDZ + InitLocal at block entry. The
                // Stmt::Variable identifier branch reuses these slots
                // via the block_pre_slots stack. Closes the block
                // surface of TDZ enforcement (point iii partial,
                // matching the pattern of EXT 23 function-body + EXT 24
                // for-head). Skips Function/Arrow/Class bodies in walk
                // (their let/const are their own scope's responsibility).
                let mut pre: std::collections::HashMap<String, u16> =
                    std::collections::HashMap::new();
                for s in body.iter() {
                    if let Stmt::Variable(v) = s {
                        if !matches!(v.kind, VariableKind::Let | VariableKind::Const) {
                            continue;
                        }
                        for d in &v.declarators {
                            if let rusty_js_ast::BindingPattern::Identifier(id) = &d.target {
                                if pre.contains_key(&id.name) {
                                    continue;
                                }
                                let slot = self.alloc_local(LocalDescriptor {
                                    name: id.name.clone(),
                                    kind: v.kind,
                                    depth: 0,
                                });
                                pre.insert(id.name.clone(), slot);
                                encode_op(&mut self.bytecode, Op::PushTDZ);
                                encode_op(&mut self.bytecode, Op::InitLocal);
                                encode_u16(&mut self.bytecode, slot);
                            }
                        }
                    }
                }
                self.block_pre_slots.push(pre);
                for s in body {
                    self.compile_stmt(s)?;
                }
                self.block_pre_slots.pop();
                self.block_depth -= 1;
                for i in snapshot..self.locals.len() {
                    // Don't rename pre-allocated names (let/const var
                    // hoisted from outer); they're meant to outlive.
                    let nm = &self.locals[i].name;
                    if !nm.starts_with('<') {
                        self.locals[i].name = format!("<scoped@{}>{}", i, nm);
                        self.locals_snapshot = None;
                    }
                }
            }
            Stmt::Variable(v) => {
                // Tier-Ω.5.WWWWWWW: dedupe same-name declarators within a
                // single `var`/`let`/`const` declarator list per ECMA §13.3.
                // `var x = a, x = x` is a single binding `x` whose init runs
                // twice (last write wins); prior code alloc_local'd a fresh
                // slot per declarator, producing two `x` slots and breaking
                // the second declarator's RHS lookup (it resolved to the
                // freshly-allocated slot, undefined, before the store).
                // Babel's for-of transpilation hits this exact pattern at
                // every iteration site (`var _iter = arr, _isArr = ..., _i =
                // 0, _iter = _isArr ? _iter : getIterator(_iter)`).
                let mut local_slots: std::collections::HashMap<String, u16> =
                    std::collections::HashMap::new();
                let script_top = self.is_script_top_var(v.kind);
                for d in &v.declarators {
                    match &d.target {
                        rusty_js_ast::BindingPattern::Identifier(id) => {
                            // ES-EXT 2 v2 (post-GBSU): pre-allocation +
                            // StoreLocal is unchanged; an additional StoreGlobal
                            // is emitted AFTER the StoreLocal to attach the
                            // binding to globalThis. The mirror is one-shot at
                            // declaration time — subsequent reassignments
                            // (resolved as LoadLocal/StoreLocal via the local
                            // slot) DON'T flow back to globalThis. This matches
                            // common patterns (eval declares var, outer reads
                            // via name or via globalThis.X) at declaration cost
                            // of one extra LoadLocal+StoreGlobal pair per
                            // top-level script var.
                            // Reuse pre-allocated slot if the H1 hoisting
                            // pass already created one; else reuse a slot
                            // from earlier-in-this-list; else alloc.
                            //
                            // Ω.5.P52.E3: gate pre_allocated_slots reuse by
                            // block_depth + v.kind. Pre-allocation is the
                            // function-body top-level Phase A — its slots
                            // belong to the function scope. For let/const
                            // inside a nested block, the spec mandates a
                            // FRESH binding (block scope, §13.2); reusing
                            // the pre-allocated slot would write the inner
                            // value into the outer's slot, defeating the
                            // existing Stmt::Block rename-on-exit machinery.
                            // var always hoists to function scope, so its
                            // reuse remains correct at any depth.
                            let pre_reuse_ok =
                                self.block_depth == 0 || matches!(v.kind, VariableKind::Var);
                            let slot = if pre_reuse_ok {
                                if let Some(s) = self.pre_allocated_slots.get(&id.name).copied() {
                                    s
                                } else if let Some(s) = local_slots.get(&id.name).copied() {
                                    s
                                } else {
                                    let s = self.alloc_local(LocalDescriptor {
                                        name: id.name.clone(),
                                        kind: v.kind,
                                        depth: 0,
                                    });
                                    local_slots.insert(id.name.clone(), s);
                                    s
                                }
                            } else if let Some(s) = local_slots.get(&id.name).copied() {
                                s
                            } else if let Some(s) = self
                                .block_pre_slots
                                .last()
                                .and_then(|m| m.get(&id.name).copied())
                            {
                                // IR-EXT 31: reuse slot pre-allocated at
                                // block-entry for block-scope TDZ.
                                s
                            } else {
                                let s = self.alloc_local(LocalDescriptor {
                                    name: id.name.clone(),
                                    kind: v.kind,
                                    depth: 0,
                                });
                                local_slots.insert(id.name.clone(), s);
                                s
                            };
                            if let Some(init) = &d.init {
                                // IR-EXT 21: TDZ self-init guard per §13.3.1.1.
                                // For let/const, if the initializer expression
                                // contains a free reference to the binding's
                                // own name, the binding is in TDZ during init
                                // eval — emit a synthetic ReferenceError throw.
                                // Var is exempt (var hoists with undefined init).
                                if matches!(v.kind, VariableKind::Let | VariableKind::Const)
                                    && self.expr_refs_free(init, &id.name)
                                {
                                    self.emit_throw_referenceerror(&format!(
                                        "Cannot access '{}' before initialization",
                                        id.name
                                    ));
                                    // Throw unwinds; remaining init+store
                                    // bytecode is unreachable. Still emit a
                                    // PushUndef + StoreLocal so the bytecode
                                    // verifier sees a well-formed sequence.
                                    encode_op(&mut self.bytecode, Op::PushUndef);
                                } else {
                                    // Tier-Ω.5.P15.E1: NamedEvaluation hint for
                                    // anonymous function expressions and arrows
                                    // bound by `let`/`const`/`var x = ...`.
                                    self.compile_expr_with_name_hint(init, Some(&id.name))?;
                                }
                            } else {
                                encode_op(&mut self.bytecode, Op::PushUndef);
                            }
                            // IR-EXT 25: variable-decl init site uses
                            // InitLocal so it can overwrite the TDZ
                            // sentinel that scope-entry seeded. Var also
                            // uses InitLocal — harmless since var slots
                            // aren't TDZ-seeded.
                            encode_op(&mut self.bytecode, Op::InitLocal);
                            encode_u16(&mut self.bytecode, slot);
                            // ES-EXT 2 v2: mirror to globalThis at declaration
                            // time (script-top var only, per ECMA-262 §16.1).
                            if script_top {
                                encode_op(&mut self.bytecode, Op::LoadLocal);
                                encode_u16(&mut self.bytecode, slot);
                                let name_idx =
                                    self.constants.intern(Constant::String(id.name.clone()));
                                encode_op(&mut self.bytecode, Op::StoreGlobal);
                                encode_u16(&mut self.bytecode, name_idx);
                            }
                        }
                        pat @ (rusty_js_ast::BindingPattern::Array(_)
                        | rusty_js_ast::BindingPattern::Object(_)) => {
                            // Tier-Ω.5.g.3: destructure declarator. Evaluate
                            // init into a hidden source slot, allocate every
                            // bound name as a local under v.kind, then walk
                            // the pattern.
                            //
                            // Tier-Ω.5.dddd: reuse pre-allocated slots if
                            // Phase A.4 already created them — otherwise
                            // hoisted function decls' upvalue captures
                            // point at the wrong slot (the pre-allocated
                            // one stays Undefined; this fresh one gets
                            // the value).
                            // Ω.5.P52.E3: same block-scope gate as the
                            // identifier branch above. Inside a nested block,
                            // let/const destructure introduces fresh bindings
                            // per spec — pre-allocated slot reuse would
                            // shadow-write the outer slot.
                            let pre_reuse_ok =
                                self.block_depth == 0 || matches!(v.kind, VariableKind::Var);
                            for id in pat.collect_names() {
                                if pre_reuse_ok && self.pre_allocated_slots.contains_key(&id.name) {
                                    // Slot already exists; skip re-alloc.
                                    continue;
                                }
                                self.alloc_local(LocalDescriptor {
                                    name: id.name.clone(),
                                    kind: v.kind,
                                    depth: 0,
                                });
                            }
                            let src_slot = self.alloc_temp("<destr.src>");
                            if let Some(init) = &d.init {
                                // IR-EXT 22: TDZ self-init guard for destructure
                                // bindings. For let/const, if the initializer
                                // references any of the pattern's own bound
                                // names, those bindings are in TDZ during init
                                // eval per §13.3.1.1. Throws ReferenceError.
                                let tdz_hit =
                                    if matches!(v.kind, VariableKind::Let | VariableKind::Const) {
                                        pat.collect_names().iter().find_map(|id| {
                                            if self.expr_refs_free(init, &id.name) {
                                                Some(id.name.clone())
                                            } else {
                                                None
                                            }
                                        })
                                    } else {
                                        None
                                    };
                                if let Some(name) = tdz_hit {
                                    self.emit_throw_referenceerror(&format!(
                                        "Cannot access '{}' before initialization",
                                        name
                                    ));
                                    encode_op(&mut self.bytecode, Op::PushUndef);
                                } else {
                                    self.compile_expr(init)?;
                                }
                            } else {
                                encode_op(&mut self.bytecode, Op::PushUndef);
                            }
                            encode_op(&mut self.bytecode, Op::StoreLocal);
                            encode_u16(&mut self.bytecode, src_slot);
                            self.emit_destructure(pat, src_slot)?;
                        }
                    }
                }
            }
            Stmt::Throw { argument, .. } => {
                self.compile_expr(argument)?;
                encode_op(&mut self.bytecode, Op::Throw);
            }
            Stmt::Debugger { .. } => {
                encode_op(&mut self.bytecode, Op::Debugger);
            }
            Stmt::If {
                test,
                consequent,
                alternate,
                ..
            } => {
                self.compile_expr(test)?;
                let jump_if_false = self.emit_jump(Op::JumpIfFalse);
                self.compile_stmt(consequent)?;
                if let Some(alt) = alternate {
                    let jump_end = self.emit_jump(Op::Jump);
                    self.patch_jump(jump_if_false);
                    self.compile_stmt(alt)?;
                    self.patch_jump(jump_end);
                } else {
                    self.patch_jump(jump_if_false);
                }
            }
            Stmt::While { test, body, .. } => {
                let loop_start = self.bytecode.len();
                self.loop_stack.push(LoopFrame {
                    continue_target: loop_start,
                    continue_pending: false,
                    continue_patches: Vec::new(),
                    break_patches: Vec::new(),
                    is_switch: false,
                    try_depth: 0,
                    pending_finalizers: Vec::new(),
                    label: self.pending_label.take(),
                });
                self.compile_expr(test)?;
                let jump_if_false = self.emit_jump(Op::JumpIfFalse);
                self.compile_stmt(body)?;
                self.emit_back_jump(loop_start);
                self.patch_jump(jump_if_false);
                let frame = self.loop_stack.pop().unwrap();
                for site in frame.break_patches {
                    self.patch_jump_at(site);
                }
            }
            Stmt::With { object, body, .. } => {
                self.compile_expr(object)?;
                encode_op(&mut self.bytecode, Op::EnterWith);
                self.with_depth += 1;
                self.compile_stmt(body)?;
                self.with_depth -= 1;
                encode_op(&mut self.bytecode, Op::ExitWith);
            }
            Stmt::DoWhile { body, test, .. } => {
                let loop_start = self.bytecode.len();
                self.loop_stack.push(LoopFrame {
                    continue_target: 0,
                    continue_pending: true,
                    continue_patches: Vec::new(),
                    break_patches: Vec::new(),
                    is_switch: false,
                    try_depth: 0,
                    pending_finalizers: Vec::new(),
                    label: self.pending_label.take(),
                });
                self.compile_stmt(body)?;
                let test_pos = self.bytecode.len();
                // Finalize continue target to test_pos and patch any
                // pending continue sites.
                {
                    let frame = self.loop_stack.last_mut().unwrap();
                    frame.continue_target = test_pos;
                    frame.continue_pending = false;
                }
                let patches =
                    std::mem::take(&mut self.loop_stack.last_mut().unwrap().continue_patches);
                for site in patches {
                    self.patch_jump_at(site);
                }
                self.compile_expr(test)?;
                let jump_back = self.emit_jump(Op::JumpIfTrue);
                self.patch_jump_to(jump_back, loop_start);
                let frame = self.loop_stack.pop().unwrap();
                for site in frame.break_patches {
                    self.patch_jump_at(site);
                }
            }
            Stmt::For {
                init,
                test,
                update,
                body,
                ..
            } => {
                // Ω.5.P52.E4: scope let/const declared in the for-header to
                // the entire for-statement, not the enclosing function.
                // Mirrors Stmt::Block's snapshot+rename pattern so resolve_local
                // stops matching the header bindings after loop exit.
                let scope_snapshot = self.locals.len();
                self.block_depth += 1;
                // Track which header bindings need PerIterationBindings per
                // ECMA-262 §13.7.4 step 11 (only let/const, not var).
                let mut per_iter_slots: Vec<u16> = Vec::new();
                if let Some(init) = init {
                    match init {
                        ForInit::Variable(v) => {
                            let needs_per_iter = matches!(
                                v.kind,
                                rusty_js_ast::VariableKind::Let | rusty_js_ast::VariableKind::Const
                            );
                            self.compile_stmt(&Stmt::Variable(v.clone()))?;
                            if needs_per_iter {
                                // Header bindings were just appended to locals.
                                for slot in scope_snapshot..self.locals.len() {
                                    per_iter_slots.push(slot as u16);
                                }
                            }
                        }
                        ForInit::Expression(e) => {
                            self.compile_expr(e)?;
                            encode_op(&mut self.bytecode, Op::Pop);
                        }
                    }
                }
                let test_pos = self.bytecode.len();
                self.loop_stack.push(LoopFrame {
                    continue_target: 0,
                    continue_pending: true,
                    continue_patches: Vec::new(),
                    break_patches: Vec::new(),
                    is_switch: false,
                    try_depth: 0,
                    pending_finalizers: Vec::new(),
                    label: self.pending_label.take(),
                });
                let jump_if_false = if let Some(t) = test {
                    self.compile_expr(t)?;
                    Some(self.emit_jump(Op::JumpIfFalse))
                } else {
                    None
                };
                self.compile_stmt(body)?;
                // ECMA-262 §13.7.4 step 11 PerIterationBindings: at end of
                // iteration body, for each let/const header binding,
                // detach the upvalue cell so the next iteration's
                // CaptureLocal promotes fresh. Closures created in THIS
                // iteration retain their cell; the next iteration's
                // closures get a fresh cell. Value is preserved by
                // round-trip through the slot (LoadLocal → ResetLocalCell
                // → StoreLocal). Without this, `for (let i...) fns.push(()=>i)`
                // closures all capture the same final i.
                for slot in &per_iter_slots {
                    encode_op(&mut self.bytecode, Op::LoadLocal);
                    encode_u16(&mut self.bytecode, *slot);
                    encode_op(&mut self.bytecode, Op::ResetLocalCell);
                    encode_u16(&mut self.bytecode, *slot);
                    encode_op(&mut self.bytecode, Op::StoreLocal);
                    encode_u16(&mut self.bytecode, *slot);
                }
                let update_pos = self.bytecode.len();
                // Finalize continue target and patch pending continue sites.
                {
                    let frame = self.loop_stack.last_mut().unwrap();
                    frame.continue_target = update_pos;
                    frame.continue_pending = false;
                }
                let patches =
                    std::mem::take(&mut self.loop_stack.last_mut().unwrap().continue_patches);
                for site in patches {
                    self.patch_jump_at(site);
                }
                if let Some(u) = update {
                    self.compile_expr(u)?;
                    encode_op(&mut self.bytecode, Op::Pop);
                }
                self.emit_back_jump(test_pos);
                if let Some(j) = jump_if_false {
                    self.patch_jump(j);
                }
                let frame = self.loop_stack.pop().unwrap();
                for site in frame.break_patches {
                    self.patch_jump_at(site);
                }
                self.block_depth -= 1;
                for i in scope_snapshot..self.locals.len() {
                    let nm = &self.locals[i].name;
                    if !nm.starts_with('<') {
                        self.locals[i].name = format!("<scoped@{}>{}", i, nm);
                        self.locals_snapshot = None;
                    }
                }
            }
            Stmt::ForOf {
                left,
                right,
                body,
                await_,
                ..
            } => {
                // PPAE-EXT 1: §14.7.1.2 Static Semantics:Early Errors —
                // BoundNames of ForDeclaration must not also occur in
                // VarDeclaredNames of Statement. Reject at compile
                // (surfaces as SyntaxError via eval's
                // CompileError->SyntaxError mapping per PPA-EXT 1).
                if let rusty_js_ast::ForBinding::Decl { kind, target, .. } = left {
                    if matches!(kind, VariableKind::Let | VariableKind::Const) {
                        let head_names: Vec<String> = target
                            .collect_names()
                            .iter()
                            .map(|id| id.name.clone())
                            .collect();
                        // PPAE-EXT 3 (§14.7.1.2): BoundNames of ForDeclaration
                        // must not contain duplicates (the destructure-leaf-
                        // duplicate variant — for (const [x, x] of [])).
                        let mut seen: std::collections::HashSet<&str> =
                            std::collections::HashSet::new();
                        for n in &head_names {
                            if !seen.insert(n.as_str()) {
                                return Err(self.err(
                                    span,
                                    &format!("duplicate lexical binding `{}` in for-of head", n),
                                ));
                            }
                        }
                        let mut body_vars: Vec<(String, rusty_js_ast::VariableKind)> = Vec::new();
                        collect_hoisted_var_names(body.as_ref(), &mut body_vars);
                        for (vname, _) in &body_vars {
                            if head_names.iter().any(|h| h == vname) {
                                return Err(self.err(span, &format!(
                                    "lexical binding `{}` in for-of head conflicts with `var {}` in body", vname, vname)));
                            }
                        }
                    }
                }
                // Ω.5.P52.E4: scope the for-of binding (`for (const x of arr)`)
                // to the for-statement, not the function. Snapshot + rename
                // mirrors Stmt::Block.
                let scope_snapshot = self.locals.len();
                self.block_depth += 1;
                // Allocate hidden slot for the iterator and a binding slot
                // for the loop variable.
                let iter_slot = self.alloc_local(LocalDescriptor {
                    name: "<iter>".into(),
                    kind: VariableKind::Let,
                    depth: 0,
                });
                let forawait_tmp = if *await_ {
                    Some(self.alloc_temp("<forawait.await>"))
                } else {
                    None
                };
                // Per ECMA-262 §14.7.5.5, `let`/`const` heads receive a fresh
                // binding per iteration; `var` heads remain function-scoped
                // and share a single slot across iterations. We track this
                // with `per_iter_fresh`: when true, emit Op::ResetLocalCell at
                // iteration entry so closures captured in iteration N keep
                // their handle to that iteration's cell. Tier-Ω.5.g.1.
                // Returns (slot_to_store_value_into, destructure_pattern_or_none, per_iter_fresh).
                // When destructure_pattern is Some, the body prologue will
                // run the pattern lowering using slot_to_store_value_into
                // as the hidden source.
                let (bind_slot, destr_pat, per_iter_fresh, assign_target): (
                    u16,
                    Option<rusty_js_ast::BindingPattern>,
                    bool,
                    Option<rusty_js_ast::Expr>,
                ) = match left {
                    rusty_js_ast::ForBinding::Decl { kind, target, .. } => {
                        match target {
                            rusty_js_ast::BindingPattern::Identifier(id) => {
                                let s = self.alloc_local(LocalDescriptor {
                                    name: id.name.clone(),
                                    kind: *kind,
                                    depth: 0,
                                });
                                let fresh = matches!(kind, VariableKind::Let | VariableKind::Const);
                                (s, None, fresh, None)
                            }
                            pat @ (rusty_js_ast::BindingPattern::Array(_)
                            | rusty_js_ast::BindingPattern::Object(_)) => {
                                // Allocate every bound name as a local under kind,
                                // then a hidden source slot for the per-iter value.
                                for id in pat.collect_names() {
                                    self.alloc_local(LocalDescriptor {
                                        name: id.name.clone(),
                                        kind: *kind,
                                        depth: 0,
                                    });
                                }
                                let s = self.alloc_temp("<forof.src>");
                                let fresh = matches!(kind, VariableKind::Let | VariableKind::Const);
                                (s, Some(pat.clone()), fresh, None)
                            }
                        }
                    }
                    rusty_js_ast::ForBinding::Pattern(pat) => {
                        match pat {
                            rusty_js_ast::BindingPattern::Identifier(id) => {
                                if let Some(s) = self.resolve_local(&id.name) {
                                    (s, None, false, None)
                                } else {
                                    let s = self.alloc_local(LocalDescriptor {
                                        name: id.name.clone(),
                                        kind: VariableKind::Let,
                                        depth: 0,
                                    });
                                    (s, None, false, None)
                                }
                            }
                            other => {
                                // SMDR-EXT 1 (2026-05-24, strict-mode-
                                // destructuring-refs locale): standalone
                                // pattern in for-of head (no var/let/const)
                                // is an AssignmentPattern per ECMA-262
                                // §13.7.5.5 + §13.15.2 — bound names are
                                // assignment-target REFERENCES, not new
                                // bindings. In strict mode, unresolvable
                                // targets must throw ReferenceError per
                                // §9.1.1.4.4 step 6. Pre-fix the compiler
                                // pre-allocated each name as a let-local,
                                // silently creating bindings.
                                //
                                // Fix: emit no pre-allocation; route the
                                // destructure through emit_destructure_
                                // assign (which uses emit_store_ident →
                                // StoreGlobal → strict ReferenceError
                                // check) at the body emission site below.
                                // Signaled via the new tuple field
                                // `destr_assign_pat` carrying the
                                // BindingPattern-as-Expr conversion.
                                let s = self.alloc_temp("<forof.src>");
                                (s, Some(other.clone()), false, None)
                            }
                        }
                    }
                    rusty_js_ast::ForBinding::AssignmentTarget(target) => {
                        let s = self.alloc_temp("<forof.assignment>");
                        (s, None, false, Some(target.clone()))
                    }
                };
                // IR-EXT 24 (TDZ candidate A.iii — for-of head): the
                // for-head let/const bound names are in TDZ during
                // evaluation of the iterable expression per §13.7.5.5.
                // Emit PushTDZ + StoreLocal for every head let/const slot
                // allocated above so `let x = 1; for (let x in { x }) {}`
                // throws ReferenceError when the inner `x` shorthand
                // resolves to the head-binding (still in TDZ).
                for i in scope_snapshot..self.locals.len() {
                    let nm = self.locals[i].name.clone();
                    let kind = self.locals[i].kind;
                    if nm.starts_with('<') {
                        continue;
                    }
                    if matches!(kind, VariableKind::Let | VariableKind::Const) {
                        encode_op(&mut self.bytecode, Op::PushTDZ);
                        encode_op(&mut self.bytecode, Op::StoreLocal);
                        encode_u16(&mut self.bytecode, i as u16);
                    }
                }
                // Compute iterable[@@iterator]() and store into iter_slot.
                self.compile_expr(right)?;
                encode_op(&mut self.bytecode, Op::Dup);
                let iter_key = self.constants.intern(Constant::String("@@iterator".into()));
                encode_op(&mut self.bytecode, Op::GetProp);
                encode_u16(&mut self.bytecode, iter_key);
                encode_op(&mut self.bytecode, Op::CallMethod);
                encode_u8(&mut self.bytecode, 0);
                encode_op(&mut self.bytecode, Op::StoreLocal);
                encode_u16(&mut self.bytecode, iter_slot);

                let loop_start = self.bytecode.len();
                self.loop_stack.push(LoopFrame {
                    continue_target: loop_start,
                    continue_pending: false,
                    continue_patches: Vec::new(),
                    break_patches: Vec::new(),
                    is_switch: false,
                    try_depth: 0,
                    pending_finalizers: Vec::new(),
                    label: self.pending_label.take(),
                });
                // IPBR-EXT 2 (2026-05-24, iter-protocol-bytecode-rewrite
                // locale): emit Op::ForOfFastNext as the first op of the
                // loop body. Fast-paths Array-iterator instances inline;
                // falls through to the slow-path emission below on shape
                // mismatch. Two patch sites: done_offset (i32 at +4) and
                // next_iter_offset (i16 at +8). Patched after the slow
                // path + body are emitted.
                let ipbr_op_off = if per_iter_fresh || assign_target.is_some() {
                    // The fused fast path writes the loop binding and jumps
                    // directly to the body. Lexical for-of heads need the
                    // ResetLocalCell prologue below so closures capture a
                    // fresh per-iteration cell; assignment targets need the
                    // post-store PutValue bridge below.
                    None
                } else {
                    let off = self.bytecode.len();
                    encode_op(&mut self.bytecode, Op::ForOfFastNext);
                    encode_u16(&mut self.bytecode, iter_slot);
                    encode_u16(&mut self.bytecode, bind_slot);
                    encode_i32(&mut self.bytecode, 0); // done_offset patch site
                    self.bytecode.extend_from_slice(&[0u8, 0u8]); // next_iter_offset patch site (i16)
                    Some(off)
                };
                // result = iter.next()
                encode_op(&mut self.bytecode, Op::LoadLocal);
                encode_u16(&mut self.bytecode, iter_slot);
                encode_op(&mut self.bytecode, Op::Dup);
                let next_key = self.constants.intern(Constant::String("next".into()));
                encode_op(&mut self.bytecode, Op::GetProp);
                encode_u16(&mut self.bytecode, next_key);
                encode_op(&mut self.bytecode, Op::CallMethod);
                encode_u8(&mut self.bytecode, 0);
                // AGFA-EXT 1: for-await-of consumes an async iterator result
                // promise before reading `done` / `value`. This is still a
                // synchronous await stand-in via the existing __await helper,
                // but it routes async-generator `.next()` results through the
                // correct observable shape instead of reading properties from
                // the Promise object itself.
                if let Some(slot) = forawait_tmp {
                    encode_op(&mut self.bytecode, Op::StoreLocal);
                    encode_u16(&mut self.bytecode, slot);
                    let await_nm = self.constants.intern(Constant::String("__await".into()));
                    encode_op(&mut self.bytecode, Op::LoadGlobal);
                    encode_u16(&mut self.bytecode, await_nm);
                    encode_op(&mut self.bytecode, Op::LoadLocal);
                    encode_u16(&mut self.bytecode, slot);
                    encode_op(&mut self.bytecode, Op::Call);
                    encode_u8(&mut self.bytecode, 1);
                }
                // [result]
                encode_op(&mut self.bytecode, Op::Dup);
                let done_key = self.constants.intern(Constant::String("done".into()));
                encode_op(&mut self.bytecode, Op::GetProp);
                encode_u16(&mut self.bytecode, done_key);
                // [result, done] — JumpIfTrue pops done
                let j_done = self.emit_jump(Op::JumpIfTrue);
                // [result]
                let value_key = self.constants.intern(Constant::String("value".into()));
                encode_op(&mut self.bytecode, Op::GetProp);
                encode_u16(&mut self.bytecode, value_key);
                // AsyncFromSyncIteratorContinuation awaits the `value`
                // component. The full wrapper protocol remains a deeper
                // runtime target, but awaiting here covers already-Promise
                // values and keeps for-await lowering from storing raw
                // pending Promise objects into the loop binding.
                if let Some(slot) = forawait_tmp {
                    encode_op(&mut self.bytecode, Op::StoreLocal);
                    encode_u16(&mut self.bytecode, slot);
                    let await_nm = self.constants.intern(Constant::String("__await".into()));
                    encode_op(&mut self.bytecode, Op::LoadGlobal);
                    encode_u16(&mut self.bytecode, await_nm);
                    encode_op(&mut self.bytecode, Op::LoadLocal);
                    encode_u16(&mut self.bytecode, slot);
                    encode_op(&mut self.bytecode, Op::Call);
                    encode_u8(&mut self.bytecode, 1);
                }
                // Per-iteration fresh binding for let/const heads: detach the
                // previous iteration's upvalue cell from this frame slot so
                // the body's CaptureLocal promotes to a new one. ECMA-262
                // §14.7.5.5 / Tier-Ω.5.g.1.
                if per_iter_fresh {
                    encode_op(&mut self.bytecode, Op::ResetLocalCell);
                    encode_u16(&mut self.bytecode, bind_slot);
                    // Tier-Ω.5.ffffff: also reset cells for destructured
                    // names. Without this, `for (const [k,v] of ...) {
                    // arr.push(() => v); }` would have every closure
                    // capture the SAME upvalue cell across iterations,
                    // collapsing to the last value (upath's `propValue`
                    // capture pattern).
                    if let Some(pat) = &destr_pat {
                        for id in pat.collect_names() {
                            if let Some(s) = self.resolve_local(&id.name) {
                                encode_op(&mut self.bytecode, Op::ResetLocalCell);
                                encode_u16(&mut self.bytecode, s);
                            }
                        }
                    }
                }
                // IR-EXT 25: for-iter binding write uses InitLocal so it
                // overwrites the TDZ sentinel seeded by IR-EXT 24.
                encode_op(&mut self.bytecode, Op::InitLocal);
                encode_u16(&mut self.bytecode, bind_slot);
                if let Some(target) = &assign_target {
                    encode_op(&mut self.bytecode, Op::LoadLocal);
                    encode_u16(&mut self.bytecode, bind_slot);
                    self.assign_target_from_stack(target)?;
                }
                // IPBR-EXT 2: patch ForOfFastNext's next_iter_offset to
                // this point (after slow-path StoreLocal completes; body
                // expects empty stack).
                if let Some(ipbr_op_off) = ipbr_op_off {
                    let next_iter_target = self.bytecode.len();
                    let ipbr_after_operand = ipbr_op_off + 11;
                    let next_disp = (next_iter_target as i32) - (ipbr_after_operand as i32);
                    let next_iter_disp_i16 = next_disp as i16;
                    self.bytecode[ipbr_op_off + 9..ipbr_op_off + 11]
                        .copy_from_slice(&next_iter_disp_i16.to_le_bytes());
                }
                if let Some(pat) = &destr_pat {
                    // SMDR-EXT 1 (2026-05-24): if the for-of head was
                    // a standalone pattern (ForBinding::Pattern, no
                    // var/let/const), the pattern's bound names are
                    // assignment-target REFERENCES per ECMA-262
                    // §13.7.5.5 + §13.15.2 — route through
                    // emit_destructure_assign (which uses StoreGlobal
                    // for non-locals, triggering strict-mode
                    // ReferenceError per §9.1.1.4.4). The decl-path
                    // (var/let/const) continues to use emit_destructure
                    // since pre-allocated locals are correct there.
                    let is_assignment_pattern = matches!(
                        left,
                        rusty_js_ast::ForBinding::Pattern(rusty_js_ast::BindingPattern::Array(_))
                            | rusty_js_ast::ForBinding::Pattern(
                                rusty_js_ast::BindingPattern::Object(_)
                            )
                    );
                    if is_assignment_pattern {
                        let target_expr = binding_pattern_to_assignment_expr(pat);
                        match target_expr {
                            Some(expr) => self.emit_destructure_assign(&expr, bind_slot)?,
                            None => {
                                // Conversion failed (rule 14 conservative
                                // fallback): fall back to binding-pattern
                                // emission. Will pre-allocate locals as
                                // before; semantically lossy in strict
                                // mode but parse-safe.
                                self.emit_destructure(pat, bind_slot)?;
                            }
                        }
                    } else {
                        self.emit_destructure(pat, bind_slot)?;
                    }
                }
                self.compile_stmt(body)?;
                self.emit_back_jump(loop_start);
                self.patch_jump(j_done);
                // At the exit, the result object is on the stack — pop it.
                encode_op(&mut self.bytecode, Op::Pop);
                // IPBR-EXT 2: patch ForOfFastNext's done_offset to here
                // (after the slow-path Pop; fast-path-done skips the Pop
                // because it has nothing on the stack to discard).
                if let Some(ipbr_op_off) = ipbr_op_off {
                    let done_target = self.bytecode.len();
                    let ipbr_after_operand = ipbr_op_off + 11;
                    let done_disp = (done_target as i32) - (ipbr_after_operand as i32);
                    self.bytecode[ipbr_op_off + 5..ipbr_op_off + 9]
                        .copy_from_slice(&done_disp.to_le_bytes());
                }
                let frame = self.loop_stack.pop().unwrap();
                for site in frame.break_patches {
                    self.patch_jump_at(site);
                }
                self.block_depth -= 1;
                for i in scope_snapshot..self.locals.len() {
                    let nm = &self.locals[i].name;
                    if !nm.starts_with('<') {
                        self.locals[i].name = format!("<scoped@{}>{}", i, nm);
                        self.locals_snapshot = None;
                    }
                }
            }
            Stmt::Break { label, .. } => match label {
                None => {
                    if let Some(frame) = self.loop_stack.last() {
                        let finalizers = frame.pending_finalizers.clone();
                        if !self.in_finalizer_emission {
                            self.in_finalizer_emission = true;
                            for fin in finalizers.iter().rev() {
                                encode_op(&mut self.bytecode, Op::TryExit);
                                self.compile_stmt(fin)?;
                            }
                            self.in_finalizer_emission = false;
                        }
                        let patch_site = encode_op(&mut self.bytecode, Op::Jump);
                        encode_i32(&mut self.bytecode, 0);
                        self.loop_stack
                            .last_mut()
                            .unwrap()
                            .break_patches
                            .push(patch_site);
                    } else {
                        return Err(self.err(span, "break outside of loop"));
                    }
                }
                Some(name) => {
                    let needle = name.name.clone();
                    if let Some(idx) = self
                        .loop_stack
                        .iter()
                        .rposition(|f| f.label.as_deref() == Some(needle.as_str()))
                    {
                        let finalizers = self.loop_stack[idx].pending_finalizers.clone();
                        for fin in finalizers.iter().rev() {
                            encode_op(&mut self.bytecode, Op::TryExit);
                            self.compile_stmt(fin)?;
                        }
                        let patch_site = encode_op(&mut self.bytecode, Op::Jump);
                        encode_i32(&mut self.bytecode, 0);
                        self.loop_stack[idx].break_patches.push(patch_site);
                    } else if let Some(idx) =
                        self.label_stack.iter().rposition(|f| f.label == needle)
                    {
                        let patch_site = encode_op(&mut self.bytecode, Op::Jump);
                        encode_i32(&mut self.bytecode, 0);
                        self.label_stack[idx].break_patches.push(patch_site);
                    } else {
                        return Err(self.err(
                            span,
                            &format!("break label '{}' not found in enclosing scopes", needle),
                        ));
                    }
                }
            },
            Stmt::FunctionDecl {
                name,
                is_async,
                is_generator,
                params,
                body,
                ..
            } => {
                let proto = self.compile_function_proto(
                    name.clone(),
                    *is_async,
                    *is_generator,
                    params,
                    body,
                )?;
                let captures = proto.upvalues.clone();
                let idx = self.constants.intern(Constant::Function(Box::new(proto)));
                encode_op(&mut self.bytecode, Op::MakeClosure);
                encode_u16(&mut self.bytecode, idx);
                emit_captures(&mut self.bytecode, &captures);
                // Bind to a local slot under the function's name.
                if let Some(n) = name {
                    let slot = self.alloc_local(LocalDescriptor {
                        name: n.name.clone(),
                        kind: VariableKind::Var, // functions are var-scoped per spec
                        depth: 0,
                    });
                    encode_op(&mut self.bytecode, Op::StoreLocal);
                    encode_u16(&mut self.bytecode, slot);
                } else {
                    encode_op(&mut self.bytecode, Op::Pop);
                }
            }
            Stmt::Try {
                block,
                handler,
                finalizer,
                ..
            } => {
                self.emit_construct_tag("try-block");
                // v1 minimal: encode TRY_ENTER with catch offset, compile block,
                // TRY_EXIT, jump past handler/finalizer; emit handler/finalizer
                // bodies. No exception-value binding to catch parameter yet
                // (would require a CATCH_BIND opcode). Body content compiles
                // normally.
                let try_enter = self.bytecode.len();
                encode_op(&mut self.bytecode, Op::TryEnter);
                let catch_off_patch = self.bytecode.len();
                encode_u32(&mut self.bytecode, 0);
                if let Some(fin) = finalizer {
                    let fin_clone = fin.as_ref().clone();
                    self.fn_finalizer_stack.push(fin_clone.clone());
                    for lf in self.loop_stack.iter_mut() {
                        lf.try_depth += 1;
                        lf.pending_finalizers.push(fin_clone.clone());
                    }
                } else {
                    for lf in self.loop_stack.iter_mut() {
                        lf.try_depth += 1;
                    }
                }
                self.compile_stmt(block)?;
                if finalizer.is_none() {
                    for lf in self.loop_stack.iter_mut() {
                        lf.try_depth = lf.try_depth.saturating_sub(1);
                    }
                }
                encode_op(&mut self.bytecode, Op::TryExit);
                let jump_to_end = self.emit_jump(Op::Jump);
                // Patch the catch offset to point here (start of handler).
                let catch_pos = self.bytecode.len();
                let _ = try_enter;
                self.bytecode[catch_off_patch..catch_off_patch + 4]
                    .copy_from_slice(&(catch_pos as u32).to_le_bytes());
                if let Some(h) = handler {
                    // Ω.5.P52.E4: scope the catch parameter to the handler
                    // body. Pre-fix the catch-param slot persisted in the
                    // function's locals, leaking the thrown value into outer
                    // resolve_local lookups.
                    let scope_snapshot = self.locals.len();
                    self.block_depth += 1;
                    // Bind the thrown value to the catch parameter. Identifier
                    // params store directly; patterned params spill to a
                    // hidden source slot and use the shared binding-pattern
                    // destructuring path so nullish object patterns throw
                    // at handler entry.
                    if let Some(p) = &h.param {
                        match p {
                            rusty_js_ast::BindingPattern::Identifier(id) => {
                                let slot = self.alloc_local(LocalDescriptor {
                                    name: id.name.clone(),
                                    kind: VariableKind::Let,
                                    depth: 0,
                                });
                                encode_op(&mut self.bytecode, Op::StoreLocal);
                                encode_u16(&mut self.bytecode, slot);
                            }
                            pat @ (rusty_js_ast::BindingPattern::Array(_)
                            | rusty_js_ast::BindingPattern::Object(_)) => {
                                for id in pat.collect_names() {
                                    self.alloc_local(LocalDescriptor {
                                        name: id.name.clone(),
                                        kind: VariableKind::Let,
                                        depth: 0,
                                    });
                                }
                                let src_slot = self.alloc_temp("<catch.destr.src>");
                                encode_op(&mut self.bytecode, Op::StoreLocal);
                                encode_u16(&mut self.bytecode, src_slot);
                                self.emit_destructure(pat, src_slot)?;
                            }
                        }
                    } else {
                        encode_op(&mut self.bytecode, Op::Pop);
                    }
                    self.compile_stmt(&h.body)?;
                    self.block_depth -= 1;
                    for i in scope_snapshot..self.locals.len() {
                        let nm = &self.locals[i].name;
                        if !nm.starts_with('<') {
                            self.locals[i].name = format!("<scoped@{}>{}", i, nm);
                            self.locals_snapshot = None;
                        }
                    }
                }
                self.patch_jump(jump_to_end);
                if let Some(fin) = finalizer {
                    self.fn_finalizer_stack.pop();
                    for lf in self.loop_stack.iter_mut() {
                        lf.try_depth = lf.try_depth.saturating_sub(1);
                        lf.pending_finalizers.pop();
                    }
                    self.compile_stmt(fin)?;
                }
            }
            Stmt::Continue { label, .. } => {
                // Find the target loop frame (skipping switch frames per
                // §14.12.4). Unlabelled: innermost loop. Labelled: nearest
                // loop frame whose `label` matches — switch frames are
                // skipped on the way up; labelled non-loop frames cannot
                // be `continue`d into and are skipped silently (a label
                // attached to a block doesn't support continue).
                let loop_idx = match label {
                    None => self.loop_stack.iter().rposition(|f| !f.is_switch),
                    Some(name) => {
                        let needle = name.name.clone();
                        let r = self.loop_stack.iter().rposition(|f| {
                            !f.is_switch && f.label.as_deref() == Some(needle.as_str())
                        });
                        if r.is_none() {
                            return Err(self.err(
                                span,
                                &format!(
                                    "continue label '{}' does not match an enclosing loop",
                                    needle
                                ),
                            ));
                        }
                        r
                    }
                };
                let Some(idx) = loop_idx else {
                    return Err(self.err(span, "continue outside of loop"));
                };
                let finalizers = self.loop_stack[idx].pending_finalizers.clone();
                for fin in finalizers.iter().rev() {
                    encode_op(&mut self.bytecode, Op::TryExit);
                    self.compile_stmt(fin)?;
                }
                let pending = self.loop_stack[idx].continue_pending;
                if pending {
                    let patch_site = encode_op(&mut self.bytecode, Op::Jump);
                    encode_i32(&mut self.bytecode, 0);
                    self.loop_stack[idx].continue_patches.push(patch_site);
                } else {
                    let target = self.loop_stack[idx].continue_target;
                    self.emit_back_jump(target);
                }
            }
            Stmt::ClassDecl {
                name,
                super_class,
                members,
                span,
            } => {
                // Tier-Ω.5.x: pre-allocate the class-name local BEFORE
                // compile_class emits the method bodies. Method bodies that
                // reference the class by name (e.g. `static get() { return
                // D.#count }`) resolve `D` via upvalue capture; the slot
                // is uninitialized at compile-time emission but holds the
                // constructor by the time any method actually runs.
                // Tier-Ω.5.qqq: reuse pre-allocated slot if Phase A.4
                // already created one for this class name.
                let class_slot = if let Some(n) = name {
                    if let Some(s) = self.pre_allocated_slots.get(&n.name).copied() {
                        Some(s)
                    } else {
                        Some(self.alloc_local(LocalDescriptor {
                            name: n.name.clone(),
                            kind: VariableKind::Let,
                            depth: 0,
                        }))
                    }
                } else {
                    None
                };
                // IR-EXT 28: class-name TDZ in extends (statement form).
                // See Expr::Class branch for full rationale.
                if let (Some(n), Some(sc)) = (name, super_class) {
                    if self.expr_refs_free(sc, &n.name) {
                        self.emit_throw_referenceerror(&format!(
                            "Cannot access '{}' before initialization",
                            n.name
                        ));
                        // Unreachable but keep stack/bytecode well-formed.
                        if let Some(slot) = class_slot {
                            encode_op(&mut self.bytecode, Op::PushUndef);
                            encode_op(&mut self.bytecode, Op::InitLocal);
                            encode_u16(&mut self.bytecode, slot);
                        }
                        return Ok(());
                    }
                }
                self.compile_class(*span, name.as_ref(), super_class.as_ref(), members)?;
                if let Some(slot) = class_slot {
                    // IR-EXT 27: class-decl outer-slot write uses InitLocal
                    // so it overwrites the TDZ sentinel that EXT 23's
                    // function-body scope-entry seeded.
                    encode_op(&mut self.bytecode, Op::InitLocal);
                    encode_u16(&mut self.bytecode, slot);
                } else {
                    encode_op(&mut self.bytecode, Op::Pop);
                }
            }
            Stmt::Switch {
                discriminant,
                cases,
                ..
            } => {
                // Tier-Ω.5.m: switch lowering per ECMA-262 §14.12.4.
                // IR-EXT 32: switch body is a single block-scope per §14.12.4
                // step 1 — let/const declared in any case are block-scoped to
                // the switch, visible across all cases (including fall-through
                // and non-matching cases), and in TDZ until their declaration
                // line executes. Pre-walk all cases' consequents for let/const
                // Identifier decls; pre-allocate slots + emit PushTDZ; push
                // map onto block_pre_slots stack. Mirrors EXT 31 Stmt::Block
                // exactly, applied to the switch's implicit block.
                self.block_depth += 1;
                let mut switch_pre: std::collections::HashMap<String, u16> =
                    std::collections::HashMap::new();
                for case in cases.iter() {
                    for s in &case.consequent {
                        if let Stmt::Variable(v) = s {
                            if !matches!(v.kind, VariableKind::Let | VariableKind::Const) {
                                continue;
                            }
                            for d in &v.declarators {
                                if let rusty_js_ast::BindingPattern::Identifier(id) = &d.target {
                                    if switch_pre.contains_key(&id.name) {
                                        continue;
                                    }
                                    let slot = self.alloc_local(LocalDescriptor {
                                        name: id.name.clone(),
                                        kind: v.kind,
                                        depth: 0,
                                    });
                                    switch_pre.insert(id.name.clone(), slot);
                                    encode_op(&mut self.bytecode, Op::PushTDZ);
                                    encode_op(&mut self.bytecode, Op::InitLocal);
                                    encode_u16(&mut self.bytecode, slot);
                                }
                            }
                        }
                    }
                }
                self.block_pre_slots.push(switch_pre);
                // 1. Spill the discriminant into a hidden local so the
                //    per-case StrictEq compares always use the same value.
                let disc_slot = self.alloc_temp("<switch.disc>");
                self.compile_expr(discriminant)?;
                encode_op(&mut self.bytecode, Op::StoreLocal);
                encode_u16(&mut self.bytecode, disc_slot);

                // 2. Dispatch chain. For each non-default case, emit a
                //    StrictEq test that conditionally jumps to that case's
                //    body. Record one patch site per case (None for the
                //    default — its body label is patched via default_jump).
                let mut case_body_patches: Vec<Option<usize>> = Vec::with_capacity(cases.len());
                let mut default_idx: Option<usize> = None;
                for (i, case) in cases.iter().enumerate() {
                    match &case.test {
                        Some(val) => {
                            encode_op(&mut self.bytecode, Op::LoadLocal);
                            encode_u16(&mut self.bytecode, disc_slot);
                            self.compile_expr(val)?;
                            encode_op(&mut self.bytecode, Op::StrictEq);
                            let j = self.emit_jump(Op::JumpIfTrue);
                            case_body_patches.push(Some(j));
                        }
                        None => {
                            if default_idx.is_some() {
                                return Err(
                                    self.err(span, "switch has more than one default clause")
                                );
                            }
                            default_idx = Some(i);
                            // Body label patched after default fall-through
                            // jump below.
                            case_body_patches.push(None);
                        }
                    }
                }

                // 3. If no case matched: jump to default body (if any) or
                //    past the switch end.
                let default_jump = self.emit_jump(Op::Jump);

                // 4. Push a switch frame so `break` targets the end. We
                //    leave continue_pending=false and continue_target=0:
                //    Continue handling skips switch frames explicitly.
                self.loop_stack.push(LoopFrame {
                    continue_target: 0,
                    continue_pending: false,
                    continue_patches: Vec::new(),
                    break_patches: Vec::new(),
                    is_switch: true,
                    try_depth: 0,
                    pending_finalizers: Vec::new(),
                    label: None,
                });

                // 5. Emit each case body in textual order. Patch its
                //    dispatch site (or default_jump for the default case)
                //    to the body start so fall-through flows naturally
                //    into the next body.
                for (i, case) in cases.iter().enumerate() {
                    let body_start = self.bytecode.len();
                    match case_body_patches[i] {
                        Some(p) => self.patch_jump_to(p, body_start),
                        None => self.patch_jump_to(default_jump, body_start),
                    }
                    for s in &case.consequent {
                        self.compile_stmt(s)?;
                    }
                }

                // 6. End label. If no default clause existed, the
                //    default_jump still needs a target — wire it to here.
                if default_idx.is_none() {
                    self.patch_jump(default_jump);
                }
                let frame = self.loop_stack.pop().unwrap();
                for site in frame.break_patches {
                    self.patch_jump_at(site);
                }
                // IR-EXT 32: pop the switch's block_pre_slots entry.
                self.block_pre_slots.pop();
                self.block_depth -= 1;
            }
            Stmt::ForIn {
                left, right, body, ..
            } => {
                // PPAE-EXT 1: §14.7.1.2 Early Errors (same as for-of).
                if let rusty_js_ast::ForBinding::Decl { kind, target, .. } = left {
                    if matches!(kind, VariableKind::Let | VariableKind::Const) {
                        let head_names: Vec<String> = target
                            .collect_names()
                            .iter()
                            .map(|id| id.name.clone())
                            .collect();
                        // PPAE-EXT 3 (§14.7.1.2): BoundNames-dup check (same
                        // as for-of branch above; symmetric across head shapes).
                        let mut seen: std::collections::HashSet<&str> =
                            std::collections::HashSet::new();
                        for n in &head_names {
                            if !seen.insert(n.as_str()) {
                                return Err(self.err(
                                    span,
                                    &format!("duplicate lexical binding `{}` in for-in head", n),
                                ));
                            }
                        }
                        let mut body_vars: Vec<(String, rusty_js_ast::VariableKind)> = Vec::new();
                        collect_hoisted_var_names(body.as_ref(), &mut body_vars);
                        for (vname, _) in &body_vars {
                            if head_names.iter().any(|h| h == vname) {
                                return Err(self.err(span, &format!(
                                    "lexical binding `{}` in for-in head conflicts with `var {}` in body", vname, vname)));
                            }
                        }
                    }
                }
                // Tier-Ω.5.m: for-in lowering. Spec deviations:
                //  - Own enumerable string keys only (no proto-chain walk).
                //  - No Symbol-key exclusion (we don't ship real Symbols).
                //  - Enumeration order matches Object.keys (integer-like
                //    indices in ascending order, then string keys in
                //    insertion order, per ECMA-262 §7.3.22).
                //
                // Lower as: keys = Object.keys(obj); for (i=0; i<keys.length; i++)
                //   bind = keys[i]; body.
                //
                // Ω.5.P52.E4: scope the for-in binding (`for (const k in obj)`).
                let scope_snapshot = self.locals.len();
                self.block_depth += 1;
                let keys_slot = self.alloc_temp("<forin.keys>");
                let len_slot = self.alloc_temp("<forin.len>");
                let idx_slot = self.alloc_temp("<forin.idx>");

                // Decide the per-iteration binding slot (and per_iter_fresh
                // for let/const heads, mirroring Ω.5.g.1 for-of semantics).
                let (bind_slot, per_iter_fresh, assign_target): (
                    u16,
                    bool,
                    Option<rusty_js_ast::Expr>,
                ) =
                    match left {
                        rusty_js_ast::ForBinding::Decl { kind, target, .. } => match target {
                            rusty_js_ast::BindingPattern::Identifier(id) => {
                                let s = self.alloc_local(LocalDescriptor {
                                    name: id.name.clone(),
                                    kind: *kind,
                                    depth: 0,
                                });
                                let fresh = matches!(kind, VariableKind::Let | VariableKind::Const);
                                (s, fresh, None)
                            }
                            _ => {
                                return Err(self
                                    .err(span, "for-in with destructure head not yet supported"))
                            }
                        },
                        rusty_js_ast::ForBinding::Pattern(pat) => match pat {
                            rusty_js_ast::BindingPattern::Identifier(id) => {
                                if let Some(s) = self.resolve_local(&id.name) {
                                    (s, false, None)
                                } else {
                                    let s = self.alloc_local(LocalDescriptor {
                                        name: id.name.clone(),
                                        kind: VariableKind::Let,
                                        depth: 0,
                                    });
                                    (s, false, None)
                                }
                            }
                            _ => {
                                return Err(self
                                    .err(span, "for-in with destructure head not yet supported"))
                            }
                        },
                        rusty_js_ast::ForBinding::AssignmentTarget(target) => {
                            let s = self.alloc_temp("<forin.assignment>");
                            (s, false, Some(target.clone()))
                        }
                    };

                // IR-EXT 24 (TDZ candidate A.iii — for-in head): symmetric
                // with for-of above; for-head let/const bindings are in TDZ
                // during evaluation of the iterable expression per §13.7.5.6.
                for i in scope_snapshot..self.locals.len() {
                    let nm = self.locals[i].name.clone();
                    let kind = self.locals[i].kind;
                    if nm.starts_with('<') {
                        continue;
                    }
                    if matches!(kind, VariableKind::Let | VariableKind::Const) {
                        encode_op(&mut self.bytecode, Op::PushTDZ);
                        encode_op(&mut self.bytecode, Op::StoreLocal);
                        encode_u16(&mut self.bytecode, i as u16);
                    }
                }
                // Ω.5.P04.E1.for-in-nullish-skip: route through
                // __for_in_keys helper instead of Object.keys directly.
                // The helper returns [] for undefined/null receivers per
                // ECMA §14.7.5.6 step 6; Object.keys would throw on those
                // and mask the spec-mandated short-circuit. The cluster of
                // packages depending on `for (const k in maybeUndef)` (joi,
                // 14 packages on the post-substrate top500 sweep) hit this.
                let helper = self
                    .constants
                    .intern(Constant::String("__for_in_keys".into()));
                encode_op(&mut self.bytecode, Op::LoadGlobal);
                encode_u16(&mut self.bytecode, helper);
                self.compile_expr(right)?;
                encode_op(&mut self.bytecode, Op::Call);
                encode_u8(&mut self.bytecode, 1);
                encode_op(&mut self.bytecode, Op::StoreLocal);
                encode_u16(&mut self.bytecode, keys_slot);

                // len = keys.length
                encode_op(&mut self.bytecode, Op::LoadLocal);
                encode_u16(&mut self.bytecode, keys_slot);
                let len_key = self.constants.intern(Constant::String("length".into()));
                encode_op(&mut self.bytecode, Op::GetProp);
                encode_u16(&mut self.bytecode, len_key);
                encode_op(&mut self.bytecode, Op::StoreLocal);
                encode_u16(&mut self.bytecode, len_slot);

                // i = 0
                encode_op(&mut self.bytecode, Op::PushI32);
                encode_i32(&mut self.bytecode, 0);
                encode_op(&mut self.bytecode, Op::StoreLocal);
                encode_u16(&mut self.bytecode, idx_slot);

                // loop_start: if (i >= len) break
                let loop_start = self.bytecode.len();
                self.loop_stack.push(LoopFrame {
                    continue_target: 0,
                    continue_pending: true,
                    continue_patches: Vec::new(),
                    break_patches: Vec::new(),
                    is_switch: false,
                    try_depth: 0,
                    pending_finalizers: Vec::new(),
                    label: self.pending_label.take(),
                });
                encode_op(&mut self.bytecode, Op::LoadLocal);
                encode_u16(&mut self.bytecode, idx_slot);
                encode_op(&mut self.bytecode, Op::LoadLocal);
                encode_u16(&mut self.bytecode, len_slot);
                encode_op(&mut self.bytecode, Op::Lt);
                let j_done = self.emit_jump(Op::JumpIfFalse);

                // bind = keys[i]
                if per_iter_fresh {
                    encode_op(&mut self.bytecode, Op::ResetLocalCell);
                    encode_u16(&mut self.bytecode, bind_slot);
                }
                encode_op(&mut self.bytecode, Op::LoadLocal);
                encode_u16(&mut self.bytecode, keys_slot);
                encode_op(&mut self.bytecode, Op::LoadLocal);
                encode_u16(&mut self.bytecode, idx_slot);
                encode_op(&mut self.bytecode, Op::GetIndex);
                // IR-EXT 25: for-in iter binding write uses InitLocal so it
                // overwrites the TDZ sentinel seeded by IR-EXT 24.
                encode_op(&mut self.bytecode, Op::InitLocal);
                encode_u16(&mut self.bytecode, bind_slot);
                if let Some(target) = &assign_target {
                    encode_op(&mut self.bytecode, Op::LoadLocal);
                    encode_u16(&mut self.bytecode, bind_slot);
                    self.assign_target_from_stack(target)?;
                }

                self.compile_stmt(body)?;

                // continue target: i++
                let cont_pos = self.bytecode.len();
                {
                    let frame = self.loop_stack.last_mut().unwrap();
                    frame.continue_target = cont_pos;
                    frame.continue_pending = false;
                }
                let patches =
                    std::mem::take(&mut self.loop_stack.last_mut().unwrap().continue_patches);
                for site in patches {
                    self.patch_jump_at(site);
                }
                encode_op(&mut self.bytecode, Op::LoadLocal);
                encode_u16(&mut self.bytecode, idx_slot);
                encode_op(&mut self.bytecode, Op::PushI32);
                encode_i32(&mut self.bytecode, 1);
                encode_op(&mut self.bytecode, Op::Add);
                encode_op(&mut self.bytecode, Op::StoreLocal);
                encode_u16(&mut self.bytecode, idx_slot);
                self.emit_back_jump(loop_start);
                self.patch_jump(j_done);
                let frame = self.loop_stack.pop().unwrap();
                for site in frame.break_patches {
                    self.patch_jump_at(site);
                }
                self.block_depth -= 1;
                for i in scope_snapshot..self.locals.len() {
                    let nm = &self.locals[i].name;
                    if !nm.starts_with('<') {
                        self.locals[i].name = format!("<scoped@{}>{}", i, nm);
                        self.locals_snapshot = None;
                    }
                }
            }
            Stmt::Labelled { label, body, .. } => {
                // Tier-Ω.5.o: LabelledStatement. If the body is a loop,
                // the label rides on the loop's LoopFrame (via
                // pending_label) and break/continue resolve there. For a
                // non-loop body, push a LabelFrame so labelled `break`
                // still works; labelled `continue` is rejected at the
                // continue site.
                let is_loop_body = matches!(
                    &**body,
                    Stmt::While { .. }
                        | Stmt::DoWhile { .. }
                        | Stmt::For { .. }
                        | Stmt::ForIn { .. }
                        | Stmt::ForOf { .. }
                );
                if is_loop_body {
                    self.pending_label = Some(label.name.clone());
                    self.compile_stmt(body)?;
                    // pending_label is consumed by the loop's frame-push.
                } else {
                    self.label_stack.push(LabelFrame {
                        label: label.name.clone(),
                        break_patches: Vec::new(),
                    });
                    self.compile_stmt(body)?;
                    let frame = self.label_stack.pop().unwrap();
                    for site in frame.break_patches {
                        self.patch_jump_at(site);
                    }
                }
            }
            Stmt::Opaque { .. } => {
                // Tier-Ω.5.cc: parser-produced Stmt::Opaque is a v1 marker
                // for statement forms not yet first-class in the AST
                // (currently: top-level `yield` expression as statement,
                // `with` statement). Generators don't actually suspend in
                // v1 (await is also no-op'd per Ω.5.x), so dropping the
                // statement entirely is semantically equivalent for the
                // dominant idiom that produced this Opaque: a yielding
                // statement inside a generator function body whose
                // suspension is never observed because we never enter
                // generator dispatch.
            }
            other => {
                let tag = match other {
                    _ => "<other>",
                };
                return Err(self.err(
                    span,
                    &format!("statement form not yet supported in compiler v1: {}", tag),
                ));
            }
        }
        Ok(())
    }

    // ───────────────── Tier-Ω.5.g.3: destructuring lowering ─────────────────

    /// Emit bytecode that destructures the value currently in `src_slot`
    /// into the bindings named by `pat`. Pattern leaves (Identifier) emit
    /// LoadLocal+StoreLocal into the leaf binding's slot, which was
    /// pre-allocated by the caller via pat.collect_names().
    fn emit_destructure(
        &mut self,
        pat: &rusty_js_ast::BindingPattern,
        src_slot: u16,
    ) -> Result<(), CompileError> {
        match pat {
            rusty_js_ast::BindingPattern::Identifier(id) => {
                let slot = self
                    .resolve_local(&id.name)
                    .expect("destructure leaf: binding slot pre-allocated by caller");
                encode_op(&mut self.bytecode, Op::LoadLocal);
                encode_u16(&mut self.bytecode, src_slot);
                // IR-EXT 26: destructure-decl leaf write uses InitLocal
                // so it can overwrite the TDZ sentinel that scope-entry
                // seeded into the bound name's slot.
                encode_op(&mut self.bytecode, Op::InitLocal);
                encode_u16(&mut self.bytecode, slot);
            }
            rusty_js_ast::BindingPattern::Array(arr) => {
                // IPEP-EXT 1 (2026-05-25): ECMA-262 §14.4.2.4
                // IteratorBindingInitialization opens an IteratorRecord
                // from the source via GetIterator(value) and reads each
                // element through iterator.next(). Previously the Array
                // path shortcut to GetIndex, bypassing @@iterator entirely
                // — iterables whose @@iterator getter threw, or whose
                // next() threw, never propagated the throw. The 40-test
                // for-of-dstr iterator-protocol cluster traces here.
                let iter_slot = self.alloc_temp("<destr.iter>");
                let done_slot = self.alloc_temp("<destr.iter.done>");
                let open_idx = self
                    .constants
                    .intern(Constant::String("__destr_iter_open".into()));
                encode_op(&mut self.bytecode, Op::LoadGlobal);
                encode_u16(&mut self.bytecode, open_idx);
                encode_op(&mut self.bytecode, Op::LoadLocal);
                encode_u16(&mut self.bytecode, src_slot);
                encode_op(&mut self.bytecode, Op::Call);
                encode_u8(&mut self.bytecode, 1);
                encode_op(&mut self.bytecode, Op::StoreLocal);
                encode_u16(&mut self.bytecode, iter_slot);
                // Initialize done_slot to false.
                encode_op(&mut self.bytecode, Op::PushFalse);
                encode_op(&mut self.bytecode, Op::StoreLocal);
                encode_u16(&mut self.bytecode, done_slot);
                for slot_opt in arr.elements.iter() {
                    self.emit_iter_step_value(iter_slot, Some(done_slot))?;
                    match slot_opt {
                        Some(elem) => {
                            self.emit_element_with_default(&elem.target, elem.default.as_ref())?
                        }
                        None => {
                            encode_op(&mut self.bytecode, Op::Pop);
                        } // elision: advance iter, discard
                    }
                }
                if let Some(rest_pat) = &arr.rest {
                    let rest_idx = self
                        .constants
                        .intern(Constant::String("__destr_iter_rest".into()));
                    encode_op(&mut self.bytecode, Op::LoadGlobal);
                    encode_u16(&mut self.bytecode, rest_idx);
                    encode_op(&mut self.bytecode, Op::LoadLocal);
                    encode_u16(&mut self.bytecode, iter_slot);
                    encode_op(&mut self.bytecode, Op::Call);
                    encode_u8(&mut self.bytecode, 1);
                    self.emit_element_with_default(rest_pat, None)?;
                    // Rest exhausts iter; no close needed.
                } else {
                    // §13.15.5.3 step 5: IteratorClose if not exhausted.
                    self.emit_iter_close_if_not_done(iter_slot, done_slot)?;
                }
            }
            rusty_js_ast::BindingPattern::Object(obj) => {
                self.emit_destructure_object_check(src_slot);
                let mut static_excluded: Vec<String> = Vec::new();
                for prop in &obj.properties {
                    // Push src on stack and get the property.
                    encode_op(&mut self.bytecode, Op::LoadLocal);
                    encode_u16(&mut self.bytecode, src_slot);
                    match &prop.key {
                        rusty_js_ast::PropertyKey::Identifier(id) => {
                            let k = self.constants.intern(Constant::String(id.name.clone()));
                            encode_op(&mut self.bytecode, Op::GetProp);
                            encode_u16(&mut self.bytecode, k);
                            static_excluded.push(id.name.clone());
                        }
                        rusty_js_ast::PropertyKey::String(s) => {
                            let k = self.constants.intern(Constant::String((**s).clone()));
                            encode_op(&mut self.bytecode, Op::GetProp);
                            encode_u16(&mut self.bytecode, k);
                            static_excluded.push((**s).clone());
                        }
                        rusty_js_ast::PropertyKey::Number(n) => {
                            let name = if n.fract() == 0.0 {
                                format!("{}", *n as i64)
                            } else {
                                format!("{}", n)
                            };
                            let k = self.constants.intern(Constant::String(name.clone()));
                            encode_op(&mut self.bytecode, Op::GetProp);
                            encode_u16(&mut self.bytecode, k);
                            static_excluded.push(name);
                        }
                        rusty_js_ast::PropertyKey::Computed(expr) => {
                            // src[expr]
                            self.compile_expr(expr)?;
                            encode_op(&mut self.bytecode, Op::GetIndex);
                            // Computed key excludes nothing reliably; rest
                            // pattern with computed keys above it isn't
                            // well-supported in our subset.
                        }
                    }
                    self.emit_element_with_default(
                        &prop.value.target,
                        prop.value.default.as_ref(),
                    )?;
                }
                if let Some(rest_id) = &obj.rest {
                    // Call shape: [callee, src, excluded_array]
                    let name_idx = self
                        .constants
                        .intern(Constant::String("__destr_object_rest".into()));
                    encode_op(&mut self.bytecode, Op::LoadGlobal);
                    encode_u16(&mut self.bytecode, name_idx);
                    encode_op(&mut self.bytecode, Op::LoadLocal);
                    encode_u16(&mut self.bytecode, src_slot);
                    encode_op(&mut self.bytecode, Op::NewArray);
                    encode_u16(&mut self.bytecode, static_excluded.len() as u16);
                    for (i, k) in static_excluded.iter().enumerate() {
                        let idx = self.constants.intern(Constant::String(k.clone()));
                        encode_op(&mut self.bytecode, Op::PushConst);
                        encode_u16(&mut self.bytecode, idx);
                        encode_op(&mut self.bytecode, Op::InitIndex);
                        encode_u32(&mut self.bytecode, i as u32);
                    }
                    encode_op(&mut self.bytecode, Op::Call);
                    encode_u8(&mut self.bytecode, 2);
                    let slot = self
                        .resolve_local(&rest_id.name)
                        .expect("object-rest binding slot pre-allocated by caller");
                    encode_op(&mut self.bytecode, Op::StoreLocal);
                    encode_u16(&mut self.bytecode, slot);
                }
            }
        }
        Ok(())
    }

    /// IPEP-EXT 1: emit one iterator-step + done-check sequence; leaves
    /// either the .value or Undefined on the operand stack. Used by both
    /// emit_destructure and emit_destructure_assign Array paths.
    /// ICOA-EXT 1: optional done_slot records the most-recent step's
    /// .done value so the caller can decide whether to emit
    /// IteratorClose per §13.15.5.3 step 5.
    fn emit_iter_step_value(
        &mut self,
        iter_slot: u16,
        done_slot: Option<u16>,
    ) -> Result<(), CompileError> {
        let step_idx = self
            .constants
            .intern(Constant::String("__destr_iter_step".into()));
        encode_op(&mut self.bytecode, Op::LoadGlobal);
        encode_u16(&mut self.bytecode, step_idx);
        encode_op(&mut self.bytecode, Op::LoadLocal);
        encode_u16(&mut self.bytecode, iter_slot);
        encode_op(&mut self.bytecode, Op::Call);
        encode_u8(&mut self.bytecode, 1);
        // [r]
        encode_op(&mut self.bytecode, Op::Dup);
        let done_key = self.constants.intern(Constant::String("done".into()));
        encode_op(&mut self.bytecode, Op::GetProp);
        encode_u16(&mut self.bytecode, done_key);
        // [r, done]
        if let Some(slot) = done_slot {
            encode_op(&mut self.bytecode, Op::Dup);
            encode_op(&mut self.bytecode, Op::StoreLocal);
            encode_u16(&mut self.bytecode, slot);
            // [r, done] still on stack
        }
        let j_done = self.emit_jump(Op::JumpIfTrue);
        // not-done: [r] → get value
        let value_key = self.constants.intern(Constant::String("value".into()));
        encode_op(&mut self.bytecode, Op::GetProp);
        encode_u16(&mut self.bytecode, value_key);
        let j_end = self.emit_jump(Op::Jump);
        // done: [r] → pop + push undef
        self.patch_jump(j_done);
        encode_op(&mut self.bytecode, Op::Pop);
        encode_op(&mut self.bytecode, Op::PushUndef);
        self.patch_jump(j_end);
        Ok(())
    }

    /// ICOA-EXT 1: emit IteratorClose check + call per §13.15.5.3 step 5.
    /// If done_slot is false at end-of-destructure (iter was not exhausted),
    /// call __destr_iter_close(iter_slot) to invoke iter.return().
    fn emit_iter_close_if_not_done(
        &mut self,
        iter_slot: u16,
        done_slot: u16,
    ) -> Result<(), CompileError> {
        // if (done_slot) skip
        encode_op(&mut self.bytecode, Op::LoadLocal);
        encode_u16(&mut self.bytecode, done_slot);
        let j_skip = self.emit_jump(Op::JumpIfTrue);
        let close_idx = self
            .constants
            .intern(Constant::String("__destr_iter_close".into()));
        encode_op(&mut self.bytecode, Op::LoadGlobal);
        encode_u16(&mut self.bytecode, close_idx);
        encode_op(&mut self.bytecode, Op::LoadLocal);
        encode_u16(&mut self.bytecode, iter_slot);
        encode_op(&mut self.bytecode, Op::Call);
        encode_u8(&mut self.bytecode, 1);
        encode_op(&mut self.bytecode, Op::Pop);
        self.patch_jump(j_skip);
        Ok(())
    }

    /// Consume the value currently on top of the operand stack, apply an
    /// optional default if it is === undefined, then bind it into `target`.
    /// For non-Identifier targets, spills into a fresh hidden local and
    /// recurses.
    fn emit_element_with_default(
        &mut self,
        target: &rusty_js_ast::BindingPattern,
        default: Option<&Expr>,
    ) -> Result<(), CompileError> {
        if let Some(def_expr) = default {
            // Dup; PushUndef; StrictEq; JumpIfFalse skip_default; Pop; <default>; skip:
            encode_op(&mut self.bytecode, Op::Dup);
            encode_op(&mut self.bytecode, Op::PushUndef);
            encode_op(&mut self.bytecode, Op::StrictEq);
            let j_skip = self.emit_jump(Op::JumpIfFalse);
            encode_op(&mut self.bytecode, Op::Pop);
            // ECMA-262 §13.15.5.3 NamedEvaluation: when the binding target
            // is an Identifier and the default expression is an anonymous
            // function/class/arrow/parenthesized-cover, the function's
            // own .name receives the identifier's text. Without the hint
            // the default compiles to name="" — test262 surfaces this
            // across the dstr cluster as 50 fn-name-inference failures.
            let name_hint = if let rusty_js_ast::BindingPattern::Identifier(id) = target {
                Some(id.name.as_str())
            } else {
                None
            };
            if name_hint.is_some() {
                self.compile_expr_with_name_hint(def_expr, name_hint)?;
            } else {
                self.compile_expr(def_expr)?;
            }
            self.patch_jump(j_skip);
        }
        match target {
            rusty_js_ast::BindingPattern::Identifier(id) => {
                let slot = self
                    .resolve_local(&id.name)
                    .expect("destructure leaf: binding slot pre-allocated by caller");
                // IR-EXT 26: destructure-decl leaf write bypasses TDZ check.
                encode_op(&mut self.bytecode, Op::InitLocal);
                encode_u16(&mut self.bytecode, slot);
            }
            nested => {
                let tmp = self.alloc_temp("<destr.tmp>");
                encode_op(&mut self.bytecode, Op::StoreLocal);
                encode_u16(&mut self.bytecode, tmp);
                self.emit_destructure(nested, tmp)?;
            }
        }
        Ok(())
    }

    /// Emit a forward jump with placeholder operand; return the operand
    /// offset for later patching via `patch_jump`.
    fn emit_jump(&mut self, op: Op) -> usize {
        encode_op(&mut self.bytecode, op);
        let operand_off = self.bytecode.len();
        encode_i32(&mut self.bytecode, 0);
        operand_off
    }

    /// Patch a forward-jump's operand so the jump targets the current
    /// bytecode offset (i.e., the place where emission has currently
    /// advanced to).
    fn patch_jump(&mut self, operand_off: usize) {
        let here = self.bytecode.len() as i32;
        let from = (operand_off + 4) as i32;
        let disp = here - from;
        self.bytecode[operand_off..operand_off + 4].copy_from_slice(&disp.to_le_bytes());
    }

    fn patch_jump_at(&mut self, operand_off: usize) {
        self.patch_jump(operand_off);
    }

    /// Patch a forward-jump to a specific absolute target offset.
    fn patch_jump_to(&mut self, operand_off: usize, target: usize) {
        let from = (operand_off + 4) as i32;
        let disp = target as i32 - from;
        self.bytecode[operand_off..operand_off + 4].copy_from_slice(&disp.to_le_bytes());
    }

    /// Emit an unconditional backward Jump to the given absolute offset.
    fn emit_back_jump(&mut self, target: usize) {
        encode_op(&mut self.bytecode, Op::Jump);
        let from = (self.bytecode.len() + 4) as i32;
        let disp = target as i32 - from;
        encode_i32(&mut self.bytecode, disp);
    }

    /// Allocate a local-slot for a binding. Returns the slot index.
    fn alloc_local(&mut self, desc: LocalDescriptor) -> u16 {
        let idx = self.locals.len();
        assert!(idx < u16::MAX as usize, "too many locals");
        self.locals.push(desc);
        // Ω.5.P03.E2.enclosing-locals-rc: locals changed → drop cache.
        self.locals_snapshot = None;
        idx as u16
    }

    /// Resolve an identifier to a local-slot index, if any.
    fn resolve_local(&self, name: &str) -> Option<u16> {
        for (i, l) in self.locals.iter().enumerate().rev() {
            if l.name == name {
                return Some(i as u16);
            }
        }
        None
    }

    /// Check whether an identifier is currently bound as `const` in this
    /// frame's locals (including outer-frame upvalues). Used by user-level
    /// assignment sites to reject const-reassignment per ECMA-262 §13.15.4
    /// at compile time. Declaration sites bypass this check (the initial
    /// `const x = 1` binding writes into the slot via emit_store_ident too).
    fn is_const_binding(&self, name: &str) -> bool {
        for l in self.locals.iter().rev() {
            if l.name == name {
                return matches!(l.kind, VariableKind::Const);
            }
        }
        for enc in self.enclosing.iter().rev() {
            for l in (*enc.locals).iter().rev() {
                if l.name == name {
                    return matches!(l.kind, VariableKind::Const);
                }
            }
        }
        false
    }

    fn upvalue_is_const(&self, idx: u16) -> bool {
        let Some(desc) = self.upvalues.get(idx as usize) else {
            return false;
        };
        let Some(parent_idx) = self.enclosing.len().checked_sub(1) else {
            return false;
        };
        self.upvalue_source_is_const_at(parent_idx, &desc.source)
    }

    fn upvalue_source_is_const_at(&self, frame_idx: usize, source: &UpvalueSource) -> bool {
        let Some(frame) = self.enclosing.get(frame_idx) else {
            return false;
        };
        match source {
            UpvalueSource::Local(slot) => frame
                .locals
                .get(*slot as usize)
                .is_some_and(|l| matches!(l.kind, VariableKind::Const)),
            UpvalueSource::Upvalue(idx) => frame.upvalues.get(*idx as usize).is_some_and(|u| {
                frame_idx.checked_sub(1).is_some_and(|parent_idx| {
                    self.upvalue_source_is_const_at(parent_idx, &u.source)
                })
            }),
        }
    }

    /// Tier-Ω.5.c: resolve an identifier to an upvalue slot in this proto.
    /// Walks the enclosing chain bottom-up. If the name resolves to a local
    /// in an outer frame, an upvalue is created in this proto (and in every
    /// intermediate enclosing frame as a transitive upvalue).
    ///
    /// Returns the upvalue index in `self.upvalues` (0-based).
    fn resolve_upvalue(&mut self, name: &str) -> Option<u16> {
        if self.enclosing.is_empty() {
            return None;
        }
        // Walk from innermost enclosing (back) to outermost (front).
        // Innermost-first lets us emit the chain of transitive upvalues
        // in the right order.
        let levels = self.enclosing.len();
        for depth in (0..levels).rev() {
            // Check locals of this enclosing level.
            let local_slot = self.enclosing[depth]
                .locals
                .iter()
                .enumerate()
                .rev()
                .find(|(_, l)| l.name == name)
                .map(|(i, _)| i as u16);
            if let Some(slot) = local_slot {
                // Build the upvalue chain top-down from `depth` toward
                // current proto. Topmost (this proto) ends up referencing
                // an Upvalue of the immediate parent unless `depth` is
                // levels-1 (immediate parent), in which case it references
                // a Local.
                let mut src = UpvalueSource::Local(slot);
                let name_s = name.to_string();
                for d in (depth + 1)..levels {
                    let up_idx =
                        add_upvalue_to(&mut self.enclosing[d].upvalues, src, name_s.clone());
                    src = UpvalueSource::Upvalue(up_idx);
                }
                let idx = add_upvalue_to(&mut self.upvalues, src, name.to_string());
                return Some(idx);
            }
            // Else check upvalues of this enclosing level — name might be
            // already-captured at this depth from an even-outer level.
            let up_at_depth = self.enclosing[depth]
                .upvalues
                .iter()
                .enumerate()
                .find(|(_, u)| u.name == name)
                .map(|(i, _)| i as u16);
            if let Some(up_idx) = up_at_depth {
                let mut src = UpvalueSource::Upvalue(up_idx);
                for d in (depth + 1)..levels {
                    let i = add_upvalue_to(&mut self.enclosing[d].upvalues, src, name.to_string());
                    src = UpvalueSource::Upvalue(i);
                }
                let idx = add_upvalue_to(&mut self.upvalues, src, name.to_string());
                return Some(idx);
            }
        }
        None
    }

    /// Tier-Ω.5.P15.E1: variant that propagates a NamedEvaluation hint.
    /// Anonymous function-expression / arrow on the RHS of a binding take
    /// the binding name as their .name property per ECMA-262 §13.2.5.5;
    /// every other expression falls through to plain compile_expr.
    fn compile_expr_with_name_hint(
        &mut self,
        e: &Expr,
        hint: Option<&str>,
    ) -> Result<(), CompileError> {
        match e {
            Expr::Function {
                name: None,
                is_async,
                is_generator,
                params,
                body,
                span,
            } => {
                self.record_span(*span);
                let proto = self.compile_function_proto_with_name_hint(
                    None,
                    hint,
                    *is_async,
                    *is_generator,
                    params,
                    body,
                )?;
                let captures = proto.upvalues.clone();
                let idx = self.constants.intern(Constant::Function(Box::new(proto)));
                encode_op(&mut self.bytecode, Op::MakeClosure);
                encode_u16(&mut self.bytecode, idx);
                emit_captures(&mut self.bytecode, &captures);
                Ok(())
            }
            Expr::Arrow {
                is_async,
                params,
                body,
                span,
            } => {
                self.record_span(*span);
                let body_stmts: Vec<Stmt> = match body {
                    ArrowBody::Block(stmts) => stmts.clone(),
                    ArrowBody::Expression(expr) => vec![Stmt::Return {
                        argument: Some((**expr).clone()),
                        span: expr.span(),
                    }],
                };
                let proto = self.compile_function_proto_with_name_hint(
                    None,
                    hint,
                    *is_async,
                    false,
                    params,
                    &body_stmts,
                )?;
                let captures = proto.upvalues.clone();
                let idx = self.constants.intern(Constant::Function(Box::new(proto)));
                encode_op(&mut self.bytecode, Op::MakeArrow);
                encode_u16(&mut self.bytecode, idx);
                emit_captures(&mut self.bytecode, &captures);
                Ok(())
            }
            Expr::Class {
                name: None,
                super_class,
                members,
                span,
            } => {
                self.record_span(*span);
                self.compile_class_with_name_hint(
                    *span,
                    None,
                    hint,
                    super_class.as_ref().map(|b| b.as_ref()),
                    members,
                )
            }
            Expr::Parenthesized { expr, .. } => self.compile_expr_with_name_hint(expr, hint),
            _ => self.compile_expr(e),
        }
    }

    fn compile_expr(&mut self, e: &Expr) -> Result<(), CompileError> {
        self.record_span(e.span());
        match e {
            Expr::NullLiteral { .. } => {
                encode_op(&mut self.bytecode, Op::PushNull);
            }
            Expr::BoolLiteral { value, .. } => {
                encode_op(
                    &mut self.bytecode,
                    if *value { Op::PushTrue } else { Op::PushFalse },
                );
            }
            Expr::NumberLiteral { value, .. } => {
                // Integer-fast-path: if the number fits in i32 exactly, emit PushI32.
                if value.fract() == 0.0 && *value >= i32::MIN as f64 && *value <= i32::MAX as f64 {
                    let iv = *value as i32;
                    encode_op(&mut self.bytecode, Op::PushI32);
                    encode_i32(&mut self.bytecode, iv);
                } else {
                    let idx = self.constants.intern(Constant::Number(*value));
                    encode_op(&mut self.bytecode, Op::PushConst);
                    encode_u16(&mut self.bytecode, idx);
                }
            }
            Expr::StringLiteral { value, .. } => {
                let idx = self.constants.intern(Constant::String(value.clone()));
                encode_op(&mut self.bytecode, Op::PushConst);
                encode_u16(&mut self.bytecode, idx);
            }
            Expr::BigIntLiteral { digits, .. } => {
                let idx = self.constants.intern(Constant::BigInt(digits.clone()));
                encode_op(&mut self.bytecode, Op::PushConst);
                encode_u16(&mut self.bytecode, idx);
            }
            Expr::Identifier { name, .. } => {
                if self.with_depth > 0 {
                    let name_idx = self.constants.intern(Constant::String(name.clone()));
                    encode_op(&mut self.bytecode, Op::LoadWithName);
                    encode_u16(&mut self.bytecode, name_idx);
                } else if let Some(slot) = self.resolve_local(name) {
                    encode_op(&mut self.bytecode, Op::LoadLocal);
                    encode_u16(&mut self.bytecode, slot);
                } else if let Some(up) = self.resolve_upvalue(name) {
                    encode_op(&mut self.bytecode, Op::LoadUpvalue);
                    encode_u16(&mut self.bytecode, up);
                } else {
                    let name_idx = self.constants.intern(Constant::String(name.clone()));
                    encode_op(&mut self.bytecode, Op::LoadGlobal);
                    encode_u16(&mut self.bytecode, name_idx);
                }
            }
            Expr::Unary {
                operator, argument, ..
            } => {
                // Tier-Ω.5.gggggg: yield / yield* lower to a call into
                // the host __yield_push__ / __yield_delegate__ globals.
                // The runtime maintains a thread-local yields vector
                // around generator-function invocations; these helpers
                // append the argument's value(s). The expression's
                // result is left on the stack (yield-as-expression
                // returns undefined in this v1; real coroutines would
                // return the value passed to .next()).
                if matches!(operator, UnaryOp::Yield) {
                    let nm = self
                        .constants
                        .intern(Constant::String("__yield_push__".into()));
                    encode_op(&mut self.bytecode, Op::LoadGlobal);
                    encode_u16(&mut self.bytecode, nm);
                    self.compile_expr(argument)?;
                    encode_op(&mut self.bytecode, Op::Call);
                    encode_u8(&mut self.bytecode, 1);
                    return Ok(());
                }
                if matches!(operator, UnaryOp::YieldDelegate) {
                    let nm = self
                        .constants
                        .intern(Constant::String("__yield_delegate__".into()));
                    encode_op(&mut self.bytecode, Op::LoadGlobal);
                    encode_u16(&mut self.bytecode, nm);
                    self.compile_expr(argument)?;
                    encode_op(&mut self.bytecode, Op::Call);
                    encode_u8(&mut self.bytecode, 1);
                    return Ok(());
                }
                // Tier-Ω.5.BBBBBBBB: `delete obj.prop` / `delete obj[key]`
                // now actually removes the property per ECMA §13.5.1.2.
                // Detect Member-expression arguments and emit DeleteProp /
                // DeleteIndex instead of the stub Op::Delete which always
                // returns true without mutating.
                if matches!(operator, UnaryOp::Delete) {
                    if let Expr::Member {
                        object, property, ..
                    } = argument.as_ref()
                    {
                        match property.as_ref() {
                            rusty_js_ast::MemberProperty::Identifier { name, .. }
                            | rusty_js_ast::MemberProperty::Private { name, .. } => {
                                self.compile_expr(object)?;
                                let idx = self.constants.intern(Constant::String(name.clone()));
                                encode_op(&mut self.bytecode, Op::DeleteProp);
                                encode_u16(&mut self.bytecode, idx);
                                return Ok(());
                            }
                            rusty_js_ast::MemberProperty::Computed { expr, .. } => {
                                self.compile_expr(object)?;
                                self.compile_expr(expr)?;
                                encode_op(&mut self.bytecode, Op::DeleteIndex);
                                return Ok(());
                            }
                        }
                    }
                }
                // REOU-EXT 1: `typeof <Ident>` and `delete <Ident>` take
                // the silent-undef path per §13.5.3 step 3.b.iii (typeof
                // of unresolvable reference returns "undefined") and
                // §13.5.1.2 (delete of unresolvable reference returns
                // true in sloppy mode). Emit Op::LoadGlobalOrUndef in
                // place of the throwing Op::LoadGlobal that Identifier
                // compilation produces by default.
                if matches!(operator, UnaryOp::Typeof | UnaryOp::Delete) {
                    if let Expr::Identifier { name, .. } = argument.as_ref() {
                        if self.resolve_local(name).is_none()
                            && self.resolve_upvalue(name).is_none()
                        {
                            let name_idx = self.constants.intern(Constant::String(name.clone()));
                            encode_op(&mut self.bytecode, Op::LoadGlobalOrUndef);
                            encode_u16(&mut self.bytecode, name_idx);
                            let op = if matches!(operator, UnaryOp::Typeof) {
                                Op::Typeof
                            } else {
                                Op::Delete
                            };
                            encode_op(&mut self.bytecode, op);
                            return Ok(());
                        }
                    }
                }
                // Tier-Ω.5.P17.E1: `await expr` lowers to `__await(expr)` —
                // a global intrinsic that synchronously unwraps already-settled
                // Promises (resolved → value; rejected → throw) and passes
                // non-Promises through. Full suspension semantics still
                // deferred; sufficient for the parity probe and any other
                // await-of-settled-Promise pattern.
                if matches!(operator, UnaryOp::Await) {
                    let nm = self.constants.intern(Constant::String("__await".into()));
                    encode_op(&mut self.bytecode, Op::LoadGlobal);
                    encode_u16(&mut self.bytecode, nm);
                    self.compile_expr(argument)?;
                    encode_op(&mut self.bytecode, Op::Call);
                    encode_u8(&mut self.bytecode, 1);
                    return Ok(());
                }
                self.compile_expr(argument)?;
                let op = match operator {
                    UnaryOp::Plus => Op::Pos,
                    UnaryOp::Minus => Op::Neg,
                    UnaryOp::BitNot => Op::BitNot,
                    UnaryOp::LogicalNot => Op::Not,
                    UnaryOp::Typeof => Op::Typeof,
                    UnaryOp::Void => Op::Void,
                    UnaryOp::Delete => Op::Delete,
                    UnaryOp::Await | UnaryOp::Yield | UnaryOp::YieldDelegate => unreachable!(),
                };
                encode_op(&mut self.bytecode, op);
            }
            Expr::Binary {
                operator,
                left,
                right,
                ..
            } => {
                match operator {
                    BinaryOp::LogicalAnd => {
                        // emit left; JumpIfFalseKeep end; Pop; emit right; end:
                        self.compile_expr(left)?;
                        let j = self.emit_jump(Op::JumpIfFalseKeep);
                        encode_op(&mut self.bytecode, Op::Pop);
                        self.compile_expr(right)?;
                        self.patch_jump(j);
                    }
                    BinaryOp::LogicalOr => {
                        self.compile_expr(left)?;
                        let j = self.emit_jump(Op::JumpIfTrueKeep);
                        encode_op(&mut self.bytecode, Op::Pop);
                        self.compile_expr(right)?;
                        self.patch_jump(j);
                    }
                    BinaryOp::NullishCoalesce => {
                        // Push LHS. Dup. JumpIfNullish to fallback (pops the
                        // top copy; the remaining LHS is the result). Else
                        // fall-through: same — Pop the dup, then we want LHS
                        // as result. Use the cleaner form:
                        //   emit LHS                            [a]
                        //   Dup                                 [a, a]
                        //   JumpIfNullish fb (pops top)          [a]   (jumps if nullish)
                        //   Jump end                            [a]
                        //   fb: Pop                              []
                        //       emit RHS                         [b]
                        //   end:                                 [result]
                        self.compile_expr(left)?;
                        encode_op(&mut self.bytecode, Op::Dup);
                        let j_fb = self.emit_jump(Op::JumpIfNullish);
                        let j_end = self.emit_jump(Op::Jump);
                        self.patch_jump(j_fb);
                        encode_op(&mut self.bytecode, Op::Pop);
                        self.compile_expr(right)?;
                        self.patch_jump(j_end);
                    }
                    _ => {
                        self.compile_expr(left)?;
                        self.compile_expr(right)?;
                        let op = match operator {
                            BinaryOp::Add => Op::Add,
                            BinaryOp::Sub => Op::Sub,
                            BinaryOp::Mul => Op::Mul,
                            BinaryOp::Div => Op::Div,
                            BinaryOp::Mod => Op::Mod,
                            BinaryOp::Pow => Op::Pow,
                            BinaryOp::Shl => Op::Shl,
                            BinaryOp::Shr => Op::Shr,
                            BinaryOp::UShr => Op::UShr,
                            BinaryOp::Lt => Op::Lt,
                            BinaryOp::Gt => Op::Gt,
                            BinaryOp::Le => Op::Le,
                            BinaryOp::Ge => Op::Ge,
                            BinaryOp::Eq => Op::Eq,
                            BinaryOp::Ne => Op::Ne,
                            BinaryOp::StrictEq => Op::StrictEq,
                            BinaryOp::StrictNe => Op::StrictNe,
                            BinaryOp::Instanceof => Op::Instanceof,
                            BinaryOp::In => Op::In,
                            BinaryOp::BitAnd => Op::BitAnd,
                            BinaryOp::BitOr => Op::BitOr,
                            BinaryOp::BitXor => Op::BitXor,
                            _ => unreachable!(),
                        };
                        encode_op(&mut self.bytecode, op);
                    }
                }
            }
            Expr::Parenthesized { expr, .. } => self.compile_expr(expr)?,
            Expr::Conditional {
                test,
                consequent,
                alternate,
                ..
            } => {
                self.compile_expr(test)?;
                let j_else = self.emit_jump(Op::JumpIfFalse);
                self.compile_expr(consequent)?;
                let j_end = self.emit_jump(Op::Jump);
                self.patch_jump(j_else);
                self.compile_expr(alternate)?;
                self.patch_jump(j_end);
            }
            Expr::Sequence { expressions, .. } => {
                // Tier-Ω.5.n: comma expression `a, b, c`. Evaluate each;
                // discard all but the last; final value remains on stack.
                let n = expressions.len();
                if n == 0 {
                    encode_op(&mut self.bytecode, Op::PushUndef);
                } else {
                    for (i, ex) in expressions.iter().enumerate() {
                        self.compile_expr(ex)?;
                        if i + 1 < n {
                            encode_op(&mut self.bytecode, Op::Pop);
                        }
                    }
                }
            }
            Expr::Assign {
                operator,
                target,
                value,
                ..
            } => {
                self.compile_assign(e.span(), *operator, target, value)?;
            }
            Expr::This { .. } => {
                // Tier-Ω.5.a: this now threads through the frame.
                encode_op(&mut self.bytecode, Op::PushThis);
            }
            Expr::Member {
                object,
                property,
                optional,
                ..
            } => {
                // Tier-Ω.5.f: super.x read — load from the parent prototype
                // (or parent constructor in a static context). The lookup
                // does NOT thread `this` for a bare member read; only when
                // wrapped in a Call does receiver-as-this matter.
                if matches!(object.as_ref(), Expr::Super { .. }) {
                    self.compile_super_member_load(e.span(), property)?;
                    return Ok(());
                }
                if *optional {
                    self.emit_construct_tag("optional-chain member");
                }
                self.compile_expr(object)?;
                // Tier-Ω.5.cc: optional chaining (`obj?.prop`). If `obj` is
                // null or undefined, short-circuit: pop it, push undefined,
                // skip the property access. Implemented via two strict-eq
                // checks (null + undefined) + JumpIfTrue to the short-circuit
                // sink. The check happens on a Dup so the value remains on
                // stack for the normal-path GetProp/GetIndex.
                let short_jumps = if *optional {
                    let mut sinks = Vec::new();
                    // Check === undefined
                    encode_op(&mut self.bytecode, Op::Dup);
                    encode_op(&mut self.bytecode, Op::PushUndef);
                    encode_op(&mut self.bytecode, Op::StrictEq);
                    sinks.push(self.emit_jump(Op::JumpIfTrue));
                    // Check === null
                    encode_op(&mut self.bytecode, Op::Dup);
                    encode_op(&mut self.bytecode, Op::PushNull);
                    encode_op(&mut self.bytecode, Op::StrictEq);
                    sinks.push(self.emit_jump(Op::JumpIfTrue));
                    Some(sinks)
                } else {
                    None
                };
                match property.as_ref() {
                    MemberProperty::Identifier { name, .. } => {
                        let idx = self.constants.intern(Constant::String(name.clone()));
                        encode_op(&mut self.bytecode, Op::GetProp);
                        encode_u16(&mut self.bytecode, idx);
                    }
                    MemberProperty::Computed { expr, .. } => {
                        self.compile_expr(expr)?;
                        encode_op(&mut self.bytecode, Op::GetIndex);
                    }
                    MemberProperty::Private { name, .. } => {
                        if expr_contains_optional_chain(object) {
                            self.emit_construct_tag("optional-chain private-continuation");
                        }
                        let idx = self
                            .constants
                            .intern(Constant::String(format!("#{}", name)));
                        encode_op(&mut self.bytecode, Op::GetProp);
                        encode_u16(&mut self.bytecode, idx);
                    }
                }
                if let Some(sinks) = short_jumps {
                    let end = self.emit_jump(Op::Jump);
                    for site in sinks {
                        self.patch_jump_at(site);
                    }
                    // Short-circuit landing: pop the leftover object,
                    // push undefined.
                    encode_op(&mut self.bytecode, Op::Pop);
                    encode_op(&mut self.bytecode, Op::PushUndef);
                    self.patch_jump_at(end);
                }
            }
            Expr::Call {
                callee,
                arguments,
                optional: call_optional,
                ..
            } => {
                if *call_optional {
                    self.emit_construct_tag("optional-chain call");
                }
                let n = arguments.len();
                if n > 255 {
                    return Err(self.err(e.span(), "too many call arguments (>255)"));
                }
                // Tier-Ω.5.f: super(...) call inside a derived-class
                // constructor. Lowers to a method-call on the parent
                // constructor with the current `this` as receiver.
                if matches!(callee.as_ref(), Expr::Super { .. }) {
                    self.compile_super_call(e.span(), arguments)?;
                    return Ok(());
                }
                // Tier-Ω.5.f: super.method(...) call inside an instance or
                // static method. Lowers to a method-call on the parent
                // prototype's (or parent constructor's) named slot with
                // the current `this` as receiver.
                if let Expr::Member {
                    object, property, ..
                } = callee.as_ref()
                {
                    if matches!(object.as_ref(), Expr::Super { .. }) {
                        self.compile_super_member_call(e.span(), property, arguments)?;
                        return Ok(());
                    }
                }
                // Tier-Ω.5.a: when callee is a MemberExpression, emit a
                // method-call form so `this` threads as the receiver.
                let is_method = matches!(callee.as_ref(), Expr::Member { .. });
                let has_spread = Self::args_has_spread(arguments);
                if is_method {
                    if let Expr::Member {
                        object,
                        property,
                        optional: mem_optional,
                        ..
                    } = callee.as_ref()
                    {
                        if has_spread {
                            // Tier-Ω.5.k: lower `obj.f(...args)` as
                            //   __apply(method, receiver, argsArray)
                            // Stack:
                            //   LoadGlobal __apply        [__apply]
                            //   <object>                  [__apply, recv]
                            //   Dup                       [__apply, recv, recv]
                            //   GetProp/GetIndex name     [__apply, recv, method]
                            //   Swap                      [__apply, method, recv]
                            //   <argsArray>               [__apply, method, recv, arr]
                            //   Call 3                    [result]
                            let apply_name = self
                                .constants
                                .intern(Constant::String("__apply".to_string()));
                            encode_op(&mut self.bytecode, Op::LoadGlobal);
                            encode_u16(&mut self.bytecode, apply_name);
                            self.compile_expr(object)?;
                            encode_op(&mut self.bytecode, Op::Dup);
                            match property.as_ref() {
                                MemberProperty::Identifier { name, .. } => {
                                    let idx = self.constants.intern(Constant::String(name.clone()));
                                    encode_op(&mut self.bytecode, Op::GetProp);
                                    encode_u16(&mut self.bytecode, idx);
                                }
                                MemberProperty::Computed { expr, .. } => {
                                    self.compile_expr(expr)?;
                                    encode_op(&mut self.bytecode, Op::GetIndex);
                                }
                                MemberProperty::Private { name, .. } => {
                                    let idx = self
                                        .constants
                                        .intern(Constant::String(format!("#{}", name)));
                                    encode_op(&mut self.bytecode, Op::GetProp);
                                    encode_u16(&mut self.bytecode, idx);
                                }
                            }
                            encode_op(&mut self.bytecode, Op::Swap);
                            self.emit_args_array(arguments)?;
                            encode_op(&mut self.bytecode, Op::Call);
                            encode_u8(&mut self.bytecode, 3);
                        } else {
                            // Push receiver, then method (looked up via GetProp /
                            // GetIndex), then args, then CallMethod n.
                            self.compile_expr(object)?;
                            // Tier-Ω.5.rr: optional-chain method call `obj?.m(...)`.
                            // If obj is null/undefined, short-circuit the entire
                            // call expression to undefined. The receiver is
                            // already on the stack; check it, branch to a sink
                            // that pops + pushes undef + skips past CallMethod.
                            let opt_sinks: Vec<usize> = if *mem_optional {
                                encode_op(&mut self.bytecode, Op::Dup);
                                encode_op(&mut self.bytecode, Op::PushUndef);
                                encode_op(&mut self.bytecode, Op::StrictEq);
                                let s1 = self.emit_jump(Op::JumpIfTrue);
                                encode_op(&mut self.bytecode, Op::Dup);
                                encode_op(&mut self.bytecode, Op::PushNull);
                                encode_op(&mut self.bytecode, Op::StrictEq);
                                let s2 = self.emit_jump(Op::JumpIfTrue);
                                vec![s1, s2]
                            } else {
                                Vec::new()
                            };
                            // Duplicate receiver so we can use it for the method
                            // lookup without losing it for the CallMethod consumer.
                            encode_op(&mut self.bytecode, Op::Dup);
                            match property.as_ref() {
                                MemberProperty::Identifier { name, .. } => {
                                    let idx = self.constants.intern(Constant::String(name.clone()));
                                    encode_op(&mut self.bytecode, Op::GetProp);
                                    encode_u16(&mut self.bytecode, idx);
                                }
                                MemberProperty::Computed { expr, .. } => {
                                    self.compile_expr(expr)?;
                                    encode_op(&mut self.bytecode, Op::GetIndex);
                                }
                                MemberProperty::Private { name, .. } => {
                                    let idx = self
                                        .constants
                                        .intern(Constant::String(format!("#{}", name)));
                                    encode_op(&mut self.bytecode, Op::GetProp);
                                    encode_u16(&mut self.bytecode, idx);
                                }
                            }
                            // Ω.5.P51.E7: optional-call `obj.method?.(args)`.
                            // Distinct from `obj?.method(args)` handled above —
                            // here the `?` gates whether to invoke the method,
                            // not whether the receiver was nullish. Per ECMA-262
                            // §13.3.7: if the method value is null/undefined,
                            // short-circuit the call to undefined. arktype's
                            // @ark/schema/parse.js:57 has `impl.applyConfig?.(...)`
                            // — when applyConfig isn't on impl, the access yields
                            // undefined, the `?.()` must not invoke. v1 lowered
                            // CallMethod regardless and surfaced 'callee not
                            // callable: undefined'.
                            let call_opt_sinks: Vec<usize> = if *call_optional {
                                // Stack at this point: [receiver, method].
                                // Test method's nullishness without consuming it.
                                encode_op(&mut self.bytecode, Op::Dup);
                                encode_op(&mut self.bytecode, Op::PushUndef);
                                encode_op(&mut self.bytecode, Op::StrictEq);
                                let s1 = self.emit_jump(Op::JumpIfTrue);
                                encode_op(&mut self.bytecode, Op::Dup);
                                encode_op(&mut self.bytecode, Op::PushNull);
                                encode_op(&mut self.bytecode, Op::StrictEq);
                                let s2 = self.emit_jump(Op::JumpIfTrue);
                                vec![s1, s2]
                            } else {
                                Vec::new()
                            };
                            // Now stack: [receiver, method]. Compile args.
                            for a in arguments {
                                match a {
                                    Argument::Expr(e) => self.compile_expr(e)?,
                                    Argument::Spread { .. } => unreachable!(),
                                }
                            }
                            encode_op(&mut self.bytecode, Op::CallMethod);
                            encode_u8(&mut self.bytecode, n as u8);
                            // Ω.5.P53.E1: separate landing pads for member-?.
                            // vs call-?. short-circuits. Pre-fix the two sink
                            // sets fed a single landing that always popped as
                            // if it came from the call-?. site (stack
                            // [receiver, method]). When BOTH were optional and
                            // the member-?. short-circuit fired (stack
                            // [receiver]), the landing popped one too many
                            // → 'operand stack underflow'. Manifested in
                            // execa's transitive yoctocolors/base.js:6 chain
                            // tty?.WriteStream?.prototype?.hasColors?.().
                            //
                            // Fix: jump to two distinct labels per sink class.
                            // member-?. sinks land at a pad that pops [recv]
                            // + pushes undef. call-?. sinks land at a pad
                            // that pops [recv, method] + pushes undef.
                            let has_mem_sinks = !opt_sinks.is_empty();
                            let has_call_sinks = !call_opt_sinks.is_empty();
                            if has_mem_sinks || has_call_sinks {
                                let done = self.emit_jump(Op::Jump);
                                // Member-?. landing: stack here is [receiver].
                                if has_mem_sinks {
                                    for s in opt_sinks {
                                        self.patch_jump_at(s);
                                    }
                                    encode_op(&mut self.bytecode, Op::Pop); // receiver
                                    encode_op(&mut self.bytecode, Op::PushUndef);
                                }
                                // Call-?. landing: stack here is [receiver, method].
                                if has_call_sinks {
                                    let skip_call_pad = if has_mem_sinks {
                                        // After the member pad lands we'd
                                        // fall through into the call pad,
                                        // which would pop a non-existent
                                        // method. Jump past it.
                                        Some(self.emit_jump(Op::Jump))
                                    } else {
                                        None
                                    };
                                    for s in call_opt_sinks {
                                        self.patch_jump_at(s);
                                    }
                                    encode_op(&mut self.bytecode, Op::Pop); // method
                                    encode_op(&mut self.bytecode, Op::Pop); // receiver
                                    encode_op(&mut self.bytecode, Op::PushUndef);
                                    if let Some(j) = skip_call_pad {
                                        self.patch_jump_at(j);
                                    }
                                }
                                self.patch_jump_at(done);
                            }
                        }
                    }
                } else if has_spread {
                    // Tier-Ω.5.k: lower `f(...args)` as
                    //   __apply(callee, undefined, argsArray)
                    let apply_name = self
                        .constants
                        .intern(Constant::String("__apply".to_string()));
                    encode_op(&mut self.bytecode, Op::LoadGlobal);
                    encode_u16(&mut self.bytecode, apply_name);
                    self.compile_expr(callee)?;
                    encode_op(&mut self.bytecode, Op::PushUndef);
                    self.emit_args_array(arguments)?;
                    encode_op(&mut self.bytecode, Op::Call);
                    encode_u8(&mut self.bytecode, 3);
                } else {
                    let is_direct_eval = !*call_optional
                        && matches!(
                            callee.as_ref(),
                            Expr::Identifier { name, .. } if name == "eval"
                        );
                    self.compile_expr(callee)?;
                    // ECMA-262 §13.3.7 OptionalChain: bare-callee optional
                    // call `f?.(args)` — if f is null/undefined, skip the
                    // call and yield undefined. Pre-fix the call site fell
                    // through to Op::Call which then threw "callee not
                    // callable: undefined". Matters for arktype's
                    // `reduceMapped?.(mappedBranches)` pattern at
                    // root.js:65 (distribute).
                    let opt_call_sinks: Vec<usize> = if *call_optional {
                        encode_op(&mut self.bytecode, Op::Dup);
                        encode_op(&mut self.bytecode, Op::PushUndef);
                        encode_op(&mut self.bytecode, Op::StrictEq);
                        let s1 = self.emit_jump(Op::JumpIfTrue);
                        encode_op(&mut self.bytecode, Op::Dup);
                        encode_op(&mut self.bytecode, Op::PushNull);
                        encode_op(&mut self.bytecode, Op::StrictEq);
                        let s2 = self.emit_jump(Op::JumpIfTrue);
                        vec![s1, s2]
                    } else {
                        Vec::new()
                    };
                    for a in arguments {
                        match a {
                            Argument::Expr(e) => self.compile_expr(e)?,
                            Argument::Spread { .. } => unreachable!(),
                        }
                    }
                    encode_op(
                        &mut self.bytecode,
                        if is_direct_eval {
                            Op::DirectEval
                        } else {
                            Op::Call
                        },
                    );
                    encode_u8(&mut self.bytecode, n as u8);
                    if !opt_call_sinks.is_empty() {
                        let done = self.emit_jump(Op::Jump);
                        for s in opt_call_sinks {
                            self.patch_jump_at(s);
                        }
                        // Short-circuit landing: pop the leftover callee,
                        // push undefined.
                        encode_op(&mut self.bytecode, Op::Pop);
                        encode_op(&mut self.bytecode, Op::PushUndef);
                        self.patch_jump_at(done);
                    }
                }
            }
            Expr::New {
                callee, arguments, ..
            } => {
                let n = arguments.len();
                if n > 255 {
                    return Err(self.err(e.span(), "too many new arguments (>255)"));
                }
                if Self::args_has_spread(arguments) {
                    // Tier-Ω.5.k: lower `new C(...args)` as
                    //   __construct(callee, argsArray)
                    let ctor_name = self
                        .constants
                        .intern(Constant::String("__construct".to_string()));
                    encode_op(&mut self.bytecode, Op::LoadGlobal);
                    encode_u16(&mut self.bytecode, ctor_name);
                    self.compile_expr(callee)?;
                    self.emit_args_array(arguments)?;
                    encode_op(&mut self.bytecode, Op::Call);
                    encode_u8(&mut self.bytecode, 2);
                } else {
                    self.compile_expr(callee)?;
                    for a in arguments {
                        match a {
                            Argument::Expr(e) => self.compile_expr(e)?,
                            Argument::Spread { .. } => unreachable!(),
                        }
                    }
                    encode_op(&mut self.bytecode, Op::New);
                    encode_u8(&mut self.bytecode, n as u8);
                }
            }
            Expr::Array { elements, .. } => {
                let has_spread = elements
                    .iter()
                    .any(|el| matches!(el, ArrayElement::Spread { .. }));
                if !has_spread {
                    let len = elements.len();
                    encode_op(&mut self.bytecode, Op::NewArray);
                    encode_u16(&mut self.bytecode, len.min(u16::MAX as usize) as u16);
                    let mut idx = 0u32;
                    for el in elements {
                        match el {
                            ArrayElement::Elision { .. } => {
                                idx += 1;
                            }
                            ArrayElement::Expr(ex) => {
                                self.compile_expr(ex)?;
                                encode_op(&mut self.bytecode, Op::InitIndex);
                                encode_u32(&mut self.bytecode, idx);
                                idx += 1;
                            }
                            ArrayElement::Spread { .. } => unreachable!(),
                        }
                    }
                } else {
                    // Tier-Ω.5.l: array literal with spread. Build incrementally
                    // via __array_push_single / __array_extend, matching the
                    // shape of emit_args_array (Ω.5.k).
                    encode_op(&mut self.bytecode, Op::NewArray);
                    encode_u16(&mut self.bytecode, 0);
                    let push_name = self
                        .constants
                        .intern(Constant::String("__array_push_single".to_string()));
                    let extend_name = self
                        .constants
                        .intern(Constant::String("__array_extend".to_string()));
                    for el in elements {
                        match el {
                            ArrayElement::Elision { .. } => {
                                encode_op(&mut self.bytecode, Op::LoadGlobal);
                                encode_u16(&mut self.bytecode, push_name);
                                encode_op(&mut self.bytecode, Op::Swap);
                                encode_op(&mut self.bytecode, Op::PushUndef);
                                encode_op(&mut self.bytecode, Op::Call);
                                encode_u8(&mut self.bytecode, 2);
                            }
                            ArrayElement::Expr(ex) => {
                                encode_op(&mut self.bytecode, Op::LoadGlobal);
                                encode_u16(&mut self.bytecode, push_name);
                                encode_op(&mut self.bytecode, Op::Swap);
                                self.compile_expr(ex)?;
                                encode_op(&mut self.bytecode, Op::Call);
                                encode_u8(&mut self.bytecode, 2);
                            }
                            ArrayElement::Spread { expr, .. } => {
                                encode_op(&mut self.bytecode, Op::LoadGlobal);
                                encode_u16(&mut self.bytecode, extend_name);
                                encode_op(&mut self.bytecode, Op::Swap);
                                self.compile_expr(expr)?;
                                encode_op(&mut self.bytecode, Op::Call);
                                encode_u8(&mut self.bytecode, 2);
                            }
                        }
                    }
                }
            }
            Expr::Object { properties, .. } => {
                encode_op(&mut self.bytecode, Op::NewObject);
                let object_home_name = format!("<object${}.home>", self.locals.len());
                let object_home_slot = self.alloc_temp(&object_home_name);
                encode_op(&mut self.bytecode, Op::Dup);
                encode_op(&mut self.bytecode, Op::StoreLocal);
                encode_u16(&mut self.bytecode, object_home_slot);
                for p in properties {
                    match p {
                        ObjectProperty::Property {
                            key,
                            value,
                            kind,
                            span,
                            ..
                        } => {
                            let is_object_method_definition = matches!(
                                value,
                                Expr::Function {
                                    span: fn_span, ..
                                } if fn_span.start == span.start
                            );
                            // Ω.5.P52.E1: getter/setter object-literal shorthand
                            // (`{get name(){...}, set name(v){...}}`) installs an
                            // accessor descriptor pair on the target via the
                            // __install_accessor__ global helper. The data-property
                            // path remains unchanged for `kind == Init`. Mirrors
                            // the class-member getter/setter compile path at
                            // Ω.5.kkkkkk; consumers writing `o.name = v` now hit
                            // the setter instead of overwriting the accessor with
                            // a fresh data property.
                            if matches!(
                                kind,
                                rusty_js_ast::ObjectPropertyKind::Get
                                    | rusty_js_ast::ObjectPropertyKind::Set
                            ) {
                                // Object-literal accessors are enumerable per
                                // ECMA-262 sec 13.2.5.5; class accessors are
                                // not. Use the obj-specific helper so the
                                // resulting descriptor is enumerable: true.
                                let helper = self
                                    .constants
                                    .intern(Constant::String("__install_accessor_obj__".into()));
                                let kind_str =
                                    if matches!(kind, rusty_js_ast::ObjectPropertyKind::Get) {
                                        "get"
                                    } else {
                                        "set"
                                    };
                                let kind_idx =
                                    self.constants.intern(Constant::String(kind_str.into()));
                                // Stack: [target] -> dup so target survives the call.
                                encode_op(&mut self.bytecode, Op::Dup); // [t, t]
                                encode_op(&mut self.bytecode, Op::LoadGlobal); // [t, t, helper]
                                encode_u16(&mut self.bytecode, helper);
                                encode_op(&mut self.bytecode, Op::Swap); // [t, helper, t]
                                                                         // Push the key — static keys as a PushConst string;
                                                                         // computed keys via compile_expr (the key value at
                                                                         // runtime gets ToString'd by __install_accessor__).
                                match key {
                                    ObjectKey::Identifier { name, .. } => {
                                        let key_idx =
                                            self.constants.intern(Constant::String(name.clone()));
                                        encode_op(&mut self.bytecode, Op::PushConst);
                                        encode_u16(&mut self.bytecode, key_idx);
                                    }
                                    ObjectKey::String { value: name, .. } => {
                                        let key_idx =
                                            self.constants.intern(Constant::String(name.clone()));
                                        encode_op(&mut self.bytecode, Op::PushConst);
                                        encode_u16(&mut self.bytecode, key_idx);
                                    }
                                    ObjectKey::Number { value: num, .. } => {
                                        let s = if num.fract() == 0.0 {
                                            format!("{}", *num as i64)
                                        } else {
                                            format!("{}", num)
                                        };
                                        let key_idx = self.constants.intern(Constant::String(s));
                                        encode_op(&mut self.bytecode, Op::PushConst);
                                        encode_u16(&mut self.bytecode, key_idx);
                                    }
                                    ObjectKey::Computed { expr: key_expr, .. } => {
                                        self.compile_expr(key_expr)?;
                                    }
                                }
                                encode_op(&mut self.bytecode, Op::PushConst); // [t, helper, t, key, kind]
                                encode_u16(&mut self.bytecode, kind_idx);
                                self.class_stack.push(ClassFrame {
                                    super_ctor_name: None,
                                    super_proto_name: None,
                                    super_home_name: Some(object_home_name.clone()),
                                    in_constructor: false,
                                    is_static: false,
                                });
                                self.compile_expr(value)?; // [t, helper, t, key, kind, fn]
                                self.class_stack.pop();
                                encode_op(&mut self.bytecode, Op::Call); // [t, result]
                                encode_u8(&mut self.bytecode, 4);
                                encode_op(&mut self.bytecode, Op::Pop); // [t]
                                continue;
                            }
                            match key {
                                ObjectKey::Identifier { name, .. }
                                | ObjectKey::String { value: name, .. } => {
                                    // Tier-Ω.5.ssssss: `{__proto__: X}` sets
                                    // [[Prototype]] per ECMA-262 §13.2.5.5
                                    // (not a normal own property). graceful-fs
                                    // / fs-extra clone via `var copy = {
                                    // __proto__: getPrototypeOf(obj) }`.
                                    if name == "__proto__" {
                                        encode_op(&mut self.bytecode, Op::Dup);
                                        self.compile_expr(value)?;
                                        encode_op(&mut self.bytecode, Op::SetPrototype);
                                    } else {
                                        // Tier-Ω.5.P15.E1: NamedEvaluation
                                        // for method shorthand + anonymous
                                        // function-valued properties.
                                        if is_object_method_definition {
                                            let helper = self.constants.intern(Constant::String(
                                                "__install_method_obj__".into(),
                                            ));
                                            encode_op(&mut self.bytecode, Op::Dup);
                                            encode_op(&mut self.bytecode, Op::LoadGlobal);
                                            encode_u16(&mut self.bytecode, helper);
                                            encode_op(&mut self.bytecode, Op::Swap);
                                            let idx = self
                                                .constants
                                                .intern(Constant::String(name.clone()));
                                            encode_op(&mut self.bytecode, Op::PushConst);
                                            encode_u16(&mut self.bytecode, idx);
                                            self.class_stack.push(ClassFrame {
                                                super_ctor_name: None,
                                                super_proto_name: None,
                                                super_home_name: Some(object_home_name.clone()),
                                                in_constructor: false,
                                                is_static: false,
                                            });
                                            self.compile_expr_with_name_hint(value, Some(name))?;
                                            self.class_stack.pop();
                                            encode_op(&mut self.bytecode, Op::Call);
                                            encode_u8(&mut self.bytecode, 3);
                                            encode_op(&mut self.bytecode, Op::Pop);
                                        } else {
                                            self.compile_expr_with_name_hint(value, Some(name))?;
                                            let idx = self
                                                .constants
                                                .intern(Constant::String(name.clone()));
                                            encode_op(&mut self.bytecode, Op::InitProp);
                                            encode_u16(&mut self.bytecode, idx);
                                        }
                                    }
                                }
                                ObjectKey::Number { value: num, .. } => {
                                    let name = if num.fract() == 0.0 {
                                        format!("{}", *num as i64)
                                    } else {
                                        format!("{}", num)
                                    };
                                    if is_object_method_definition {
                                        let helper = self.constants.intern(Constant::String(
                                            "__install_method_obj__".into(),
                                        ));
                                        encode_op(&mut self.bytecode, Op::Dup);
                                        encode_op(&mut self.bytecode, Op::LoadGlobal);
                                        encode_u16(&mut self.bytecode, helper);
                                        encode_op(&mut self.bytecode, Op::Swap);
                                        let idx = self.constants.intern(Constant::String(name));
                                        encode_op(&mut self.bytecode, Op::PushConst);
                                        encode_u16(&mut self.bytecode, idx);
                                        self.class_stack.push(ClassFrame {
                                            super_ctor_name: None,
                                            super_proto_name: None,
                                            super_home_name: Some(object_home_name.clone()),
                                            in_constructor: false,
                                            is_static: false,
                                        });
                                        self.compile_expr(value)?;
                                        self.class_stack.pop();
                                        encode_op(&mut self.bytecode, Op::Call);
                                        encode_u8(&mut self.bytecode, 3);
                                        encode_op(&mut self.bytecode, Op::Pop);
                                    } else {
                                        self.compile_expr(value)?;
                                        let idx = self.constants.intern(Constant::String(name));
                                        encode_op(&mut self.bytecode, Op::InitProp);
                                        encode_u16(&mut self.bytecode, idx);
                                    }
                                }
                                ObjectKey::Computed { expr: key_expr, .. } => {
                                    // Tier-Ω.5.o: computed object key `{[k]: v}`.
                                    // Stack: [target] -> Dup -> [target, target]
                                    // -> compile key -> [target, target, key]
                                    // -> compile value -> [target, target, key, value]
                                    // -> SetIndex -> [target, value]
                                    // -> Pop -> [target].
                                    encode_op(&mut self.bytecode, Op::Dup);
                                    self.compile_expr(key_expr)?;
                                    if is_object_method_definition {
                                        let key_slot = self.alloc_temp("<object.method.key>");
                                        encode_op(&mut self.bytecode, Op::StoreLocal);
                                        encode_u16(&mut self.bytecode, key_slot);
                                        let helper = self.constants.intern(Constant::String(
                                            "__install_method_obj__".into(),
                                        ));
                                        encode_op(&mut self.bytecode, Op::LoadGlobal);
                                        encode_u16(&mut self.bytecode, helper);
                                        encode_op(&mut self.bytecode, Op::Swap);
                                        encode_op(&mut self.bytecode, Op::LoadLocal);
                                        encode_u16(&mut self.bytecode, key_slot);
                                        self.class_stack.push(ClassFrame {
                                            super_ctor_name: None,
                                            super_proto_name: None,
                                            super_home_name: Some(object_home_name.clone()),
                                            in_constructor: false,
                                            is_static: false,
                                        });
                                        self.compile_expr(value)?;
                                        self.class_stack.pop();
                                        encode_op(&mut self.bytecode, Op::Call);
                                        encode_u8(&mut self.bytecode, 3);
                                    } else {
                                        self.compile_expr(value)?;
                                        encode_op(&mut self.bytecode, Op::SetIndex);
                                    }
                                    encode_op(&mut self.bytecode, Op::Pop);
                                }
                            }
                        }
                        ObjectProperty::Spread { expr, .. } => {
                            // Tier-Ω.5.k: lower `{...src}` as
                            //   Dup; LoadGlobal __object_spread; Swap;
                            //   <compile src>; Call 2; Pop
                            // Pre: [target]. Post: [target]. The helper
                            // copies own-enumerable props of src into
                            // target left-to-right and returns target.
                            encode_op(&mut self.bytecode, Op::Dup);
                            let helper = self
                                .constants
                                .intern(Constant::String("__object_spread".to_string()));
                            encode_op(&mut self.bytecode, Op::LoadGlobal);
                            encode_u16(&mut self.bytecode, helper);
                            encode_op(&mut self.bytecode, Op::Swap);
                            self.compile_expr(expr)?;
                            encode_op(&mut self.bytecode, Op::Call);
                            encode_u8(&mut self.bytecode, 2);
                            encode_op(&mut self.bytecode, Op::Pop);
                        }
                    }
                }
            }
            Expr::Function {
                name,
                is_async,
                is_generator,
                params,
                body,
                ..
            } => {
                let proto = self.compile_function_proto(
                    name.clone(),
                    *is_async,
                    *is_generator,
                    params,
                    body,
                )?;
                let captures = proto.upvalues.clone();
                let idx = self.constants.intern(Constant::Function(Box::new(proto)));
                encode_op(&mut self.bytecode, Op::MakeClosure);
                encode_u16(&mut self.bytecode, idx);
                emit_captures(&mut self.bytecode, &captures);
            }
            Expr::Arrow {
                is_async,
                params,
                body,
                ..
            } => {
                let body_stmts: Vec<Stmt> = match body {
                    ArrowBody::Block(stmts) => stmts.clone(),
                    ArrowBody::Expression(expr) => vec![Stmt::Return {
                        argument: Some((**expr).clone()),
                        span: expr.span(),
                    }],
                };
                let proto =
                    self.compile_function_proto(None, *is_async, false, params, &body_stmts)?;
                let captures = proto.upvalues.clone();
                let idx = self.constants.intern(Constant::Function(Box::new(proto)));
                encode_op(&mut self.bytecode, Op::MakeArrow);
                encode_u16(&mut self.bytecode, idx);
                emit_captures(&mut self.bytecode, &captures);
            }
            Expr::Update {
                operator,
                argument,
                prefix,
                ..
            } => {
                self.compile_update(e.span(), *operator, argument, *prefix)?;
            }
            Expr::Class {
                name,
                super_class,
                members,
                span,
            } => {
                // IR-EXT 28: class-name TDZ in extends per §15.7.14 step 12 —
                // the class binding is created at class evaluation but TDZ
                // until class initialization completes; reads of the class
                // name in the extends expression throw ReferenceError.
                // Compile-time guard: if the extends expression contains a
                // free reference to the class name, emit a synthetic throw.
                // Skips into Function/Arrow/Class bodies per expr_refs_free
                // (closure capture is fine — the binding only needs to be
                // initialized by the time the closure is called).
                if let (Some(n), Some(sc)) = (name, super_class) {
                    if self.expr_refs_free(sc, &n.name) {
                        self.emit_throw_referenceerror(&format!(
                            "Cannot access '{}' before initialization",
                            n.name
                        ));
                        encode_op(&mut self.bytecode, Op::PushUndef);
                        return Ok(());
                    }
                }
                self.compile_class(
                    *span,
                    name.as_ref(),
                    super_class.as_ref().map(|b| b.as_ref()),
                    members,
                )?;
            }
            Expr::Super { span } => {
                return Err(self.err(
                    *span,
                    "bare `super` reference is only valid as `super(...)` or `super.method(...)`",
                ));
            }
            Expr::TemplateLiteral {
                quasis,
                expressions,
                ..
            } => {
                // Tier-Ω.5.g.3: lower to left-to-right Add chain. op_add
                // coerces non-string operands when the LHS is a String, so
                // explicit ToString is unnecessary: the first quasi (a
                // String constant) seeds the chain, after which every Add
                // produces a String result.
                debug_assert_eq!(quasis.len(), expressions.len() + 1);
                let first = self
                    .constants
                    .intern(Constant::String((**quasis.first().unwrap()).clone()));
                encode_op(&mut self.bytecode, Op::PushConst);
                encode_u16(&mut self.bytecode, first);
                for (i, expr) in expressions.iter().enumerate() {
                    self.compile_expr(expr)?;
                    encode_op(&mut self.bytecode, Op::Add);
                    let q = self
                        .constants
                        .intern(Constant::String((*quasis[i + 1]).clone()));
                    encode_op(&mut self.bytecode, Op::PushConst);
                    encode_u16(&mut self.bytecode, q);
                    encode_op(&mut self.bytecode, Op::Add);
                }
            }
            Expr::RegExp { pattern, flags, .. } => {
                // Tier-Ω.5.i: lower regex literal to a call into the hidden
                // global `__createRegExp(pattern, flags)`. Avoids adding a
                // new opcode; trades one bytecode slot for two symbol-table
                // lookups at install_intrinsics time. The runtime helper
                // allocates an Object with InternalKind::RegExp wired to
                // %RegExp.prototype% via the alloc-time proto seam.
                let helper_name = self
                    .constants
                    .intern(Constant::String("__createRegExp".to_string()));
                encode_op(&mut self.bytecode, Op::LoadGlobal);
                encode_u16(&mut self.bytecode, helper_name);
                let pat_idx = self.constants.intern(Constant::String((**pattern).clone()));
                encode_op(&mut self.bytecode, Op::PushConst);
                encode_u16(&mut self.bytecode, pat_idx);
                let flags_idx = self.constants.intern(Constant::String((**flags).clone()));
                encode_op(&mut self.bytecode, Op::PushConst);
                encode_u16(&mut self.bytecode, flags_idx);
                encode_op(&mut self.bytecode, Op::Call);
                encode_u8(&mut self.bytecode, 2u8);
            }
            Expr::MetaProperty { meta, property, .. } if meta == "import" && property == "meta" => {
                // Tier-Ω.5.r: `import.meta` lowers to a single opcode. The
                // runtime threads the per-module import_meta object into the
                // frame at evaluate_module entry; PushImportMeta reads it.
                // `import.meta.X` member access works naturally because the
                // parser parses `import.meta.url` as Member{ MetaProperty, "url" }.
                encode_op(&mut self.bytecode, Op::PushImportMeta);
            }
            Expr::MetaProperty { meta, property, .. } if meta == "new" && property == "target" => {
                // Tier-Ω.5.s: `new.target` lowers to a single opcode. The
                // runtime populates Frame::new_target inside Op::New before
                // dispatching the constructor body; plain-Call frames leave
                // the slot None and PushNewTarget yields Undefined.
                encode_op(&mut self.bytecode, Op::PushNewTarget);
            }
            Expr::MetaProperty {
                meta,
                property,
                span,
            } => {
                return Err(self.err(
                    *span,
                    &format!("unsupported MetaProperty: {}.{}", meta, property),
                ));
            }
            other => {
                let tag = match other {
                    Expr::Sequence { .. } => "Sequence",
                    Expr::Conditional { .. } => "Conditional",
                    Expr::MetaProperty { .. } => "MetaProperty",
                    Expr::Opaque { .. } => "Opaque",
                    Expr::Class { .. } => "ClassExpression",
                    Expr::Super { .. } => "Super(standalone)",
                    Expr::Function { .. } => "Function",
                    Expr::Arrow { .. } => "Arrow",
                    _ => "<other>",
                };
                return Err(self.err(
                    e.span(),
                    &format!("expression form not yet supported in compiler v1: {}", tag),
                ));
            }
        }
        Ok(())
    }

    /// Compile a nested function body into a FunctionProto. Tier-Ω.5.c
    /// threads the outer-scope chain in so identifiers in the body that
    /// resolve to an enclosing local are captured as upvalues.
    fn compile_function_proto(
        &mut self,
        name: Option<BindingIdentifier>,
        _is_async: bool,
        is_generator: bool,
        params: &[Parameter],
        body: &[Stmt],
    ) -> Result<FunctionProto, CompileError> {
        self.compile_function_proto_with_name_hint(
            name,
            None,
            _is_async,
            is_generator,
            params,
            body,
        )
    }

    /// Tier-Ω.5.P15.E1: NamedEvaluation pathway per ECMA-262 §13.2.5.5 +
    /// §10.2.9. When an anonymous function-expression or arrow appears as
    /// the RHS of `const x = ...`, `let x = ...`, `x = ...`, or a property
    /// shorthand, the binding name flows in as `display_name_hint` and
    /// surfaces as the function's own .name property. The hint does NOT
    /// create a self-name slot inside the body (that's reserved for
    /// genuinely named function expressions per §15.2.5).
    fn compile_function_proto_with_name_hint(
        &mut self,
        name: Option<BindingIdentifier>,
        display_name_hint: Option<&str>,
        is_async: bool,
        is_generator: bool,
        params: &[Parameter],
        body: &[Stmt],
    ) -> Result<FunctionProto, CompileError> {
        // IR-EXT 40: consume the parent compiler's next_compile_is_
        // derived_ctor flag before forking the sub. Nested function/
        // arrow compiles inside the sub's body will see the cleared
        // flag and skip the SetThisTDZ emit.
        let derived_ctor_emit_needed = self.next_compile_is_derived_ctor;
        self.next_compile_is_derived_ctor = false;
        // Build the sub-compiler's enclosing chain from self's enclosing
        // plus self's own locals/upvalues snapshot. The snapshot is
        // immutable from the sub's perspective EXCEPT the sub may
        // back-fill upvalues into intermediate frames (handled by writing
        // back to self after the sub finishes).
        let mut sub_enclosing: Vec<EnclosingFrame> = self.enclosing.iter().cloned().collect();
        // Ω.5.P03.E2.enclosing-locals-rc: reuse the cached Rc snapshot
        // instead of cloning `self.locals` per child.
        let locals_snap = self.locals_snapshot();
        let direct_eval_parent_locals: Vec<String> = if body_contains_direct_eval(body) {
            locals_snap
                .iter()
                .filter(|local| !local.name.starts_with('<'))
                .map(|local| local.name.clone())
                .collect()
        } else {
            Vec::new()
        };
        sub_enclosing.push(EnclosingFrame {
            locals: locals_snap,
            upvalues: self.upvalues.clone(),
        });
        let mut sub = Compiler {
            bytecode: Vec::new(),
            constants: ConstantsPool::new(),
            locals: Vec::new(),
            source_map: Vec::new(),
            loop_stack: Vec::new(),
            fn_finalizer_stack: Vec::new(),
            in_finalizer_emission: false,
            label_stack: Vec::new(),
            pending_label: None,
            enclosing: sub_enclosing,
            upvalues: Vec::new(),
            class_stack: self.class_stack.clone(),
            class_seq: self.class_seq,
            imports: Vec::new(),
            exports: Vec::new(),
            reexport_sources: Vec::new(),
            side_effect_imports: Vec::new(),
            pending_named_exports: Vec::new(),
            pre_allocated_slots: std::collections::HashMap::new(),
            // Ω.5.P51.E1: share the parent's source-line index. Sub-compilers
            // emit bytecode whose pcs map to the SAME source — line:col
            // lookup uses the same line_starts vector. Cheap clone; the
            // vector is bounded by source line count.
            source_line_starts: self.source_line_starts.clone(),
            source_url: self.source_url.clone(),
            // Ω.5.P52.E3: sub-compilers reset block_depth — the function body
            // is its own top level for scope tracking.
            block_depth: 0,
            block_pre_slots: Vec::new(),
            next_compile_is_derived_ctor: false,
            with_depth: 0,
            // Ω.5.P53.E2: each function has its own tag list.
            construct_tags: Vec::new(),
            // EXT 73: inherit enclosing strictness; upgrade below if the
            // body opens with a `"use strict"` directive prologue.
            strict: self.strict || directive_has_use_strict(body),
            locals_snapshot: None,
            // ES-EXT 2: nested function bodies always have their own
            // scope — `var` inside a function is function-local regardless
            // of whether the enclosing top-level was Script or Module.
            script_mode: false,
        };
        for name in direct_eval_parent_locals {
            let _ = sub.resolve_upvalue(&name);
        }
        let param_count = params.len() as u16;
        // Tier-Ω.5.l: track the rest-parameter slot. Per spec only the
        // last parameter can be a rest parameter; the runtime uses this
        // to collect `args[slot..]` into an Array at call time.
        let mut rest_param_slot: Option<u16> = None;
        // Allocate one local per parameter position (slots 0..N receive
        // the args at call time per Runtime::call_function). For destructure
        // params, the param slot is the hidden source and additional locals
        // for inner names are allocated below; a prologue then runs the
        // pattern lowering at function entry.
        // Ω.5.P51.E2: allocate ALL parameter source slots first in declaration
        // order (slots 0..N-1 = arg positions), then allocate inner-destructured
        // names afterward. Previously the code interleaved per-param: a
        // destructure pattern at position 0 would consume slot 0 for the source
        // PLUS slots 1..K for its inner names, pushing the next positional
        // parameter to slot K+1. call_function stores args into slots 0..N-1,
        // so positional params after a destructure pattern received the wrong
        // value (or undefined when the inner-name slots ran past args).
        // Manifested in axios as ({value}, key) reading undefined for key.
        let mut destr_prologue: Vec<(rusty_js_ast::BindingPattern, u16, Option<Expr>)> = Vec::new();
        // Phase 1: one slot per parameter position. Destructure patterns get
        // a hidden <param$i> slot; identifier params get their identifier name
        // directly. This keeps slots 0..N-1 = arg positions exactly.
        for (i, p) in params.iter().enumerate() {
            match &p.target {
                rusty_js_ast::BindingPattern::Identifier(n) => {
                    let slot = sub.alloc_local(LocalDescriptor {
                        name: n.name.clone(),
                        kind: VariableKind::Let,
                        depth: 0,
                    });
                    if p.rest {
                        rest_param_slot = Some(slot);
                    }
                    if p.default.is_some() {
                        destr_prologue.push((
                            rusty_js_ast::BindingPattern::Identifier(n.clone()),
                            i as u16,
                            p.default.clone(),
                        ));
                    }
                }
                pat @ (rusty_js_ast::BindingPattern::Array(_)
                | rusty_js_ast::BindingPattern::Object(_)) => {
                    let slot = sub.alloc_local(LocalDescriptor {
                        name: format!("<param${}>", i),
                        kind: VariableKind::Let,
                        depth: 0,
                    });
                    // Ω.5.P51.E8: rest parameter with destructure binding
                    // (`...[opts]` / `...{a, b}`). Previously rest_param_slot
                    // was only set in the Identifier branch, so call_function's
                    // rest-collection ran only for `...name` rest params. With
                    // `...[opts]`, the rest slot received args[i] directly
                    // (a single value), and the array-destructure then iterated
                    // a non-Array value, yielding undefined for inner names.
                    // arktype's @ark/util Callable uses `(fn, ...[opts])` as
                    // its constructor signature.
                    if p.rest {
                        rest_param_slot = Some(slot);
                    }
                    destr_prologue.push((pat.clone(), i as u16, p.default.clone()));
                }
            }
        }
        // Phase 2: inner-destructured names allocated after all param sources.
        // Their slot indices live beyond N-1 so they don't shadow argument
        // positions. emit_destructure (called from the prologue below) writes
        // to these by name, which resolves via the locals table.
        for p in params.iter() {
            if let pat @ (rusty_js_ast::BindingPattern::Array(_)
            | rusty_js_ast::BindingPattern::Object(_)) = &p.target
            {
                for id in pat.collect_names() {
                    sub.alloc_local(LocalDescriptor {
                        name: id.name.clone(),
                        kind: VariableKind::Let,
                        depth: 0,
                    });
                }
            }
        }
        // IR-EXT 36 (TDZ point iii.param-expression): per-param sequential
        // TDZ per §10.2.10. During eval of param i's default expr, params
        // j > i are not yet initialized and must throw ReferenceError if
        // referenced. Compile-time guard via expr_refs_free pattern
        // (Rule 26 — params are captured by inner closures via upvalue
        // chains in test fixtures like `(a = (b = expr), b) => ...`).
        let later_param_names: Vec<Vec<String>> = (0..params.len())
            .map(|i| {
                let mut out = Vec::new();
                for j in (i + 1)..params.len() {
                    for id in params[j].target.collect_names() {
                        out.push(id.name.clone());
                    }
                }
                out
            })
            .collect();
        // Emit per-parameter default-application + destructure prologue.
        for (pat, slot, default) in &destr_prologue {
            if let Some(def_expr) = default {
                // IR-EXT 36 compile-time TDZ guard on param default.
                let pos = *slot as usize;
                let later = later_param_names
                    .get(pos)
                    .map(|v| v.as_slice())
                    .unwrap_or(&[]);
                let tdz_hit = later.iter().find(|name| sub.expr_refs_free(def_expr, name));
                if let Some(name) = tdz_hit {
                    sub.emit_throw_referenceerror(&format!(
                        "Cannot access '{}' before initialization",
                        name
                    ));
                    // Throw unwinds; the dead default-or-arg store path
                    // still emits PushUndef + StoreLocal to keep bytecode
                    // verifier-clean.
                    encode_op(&mut sub.bytecode, Op::PushUndef);
                    encode_op(&mut sub.bytecode, Op::StoreLocal);
                    encode_u16(&mut sub.bytecode, *slot);
                } else {
                    // if args[slot] === undefined: args[slot] = default
                    encode_op(&mut sub.bytecode, Op::LoadLocal);
                    encode_u16(&mut sub.bytecode, *slot);
                    encode_op(&mut sub.bytecode, Op::PushUndef);
                    encode_op(&mut sub.bytecode, Op::StrictEq);
                    let j_skip = sub.emit_jump(Op::JumpIfFalse);
                    sub.compile_expr(def_expr)?;
                    encode_op(&mut sub.bytecode, Op::StoreLocal);
                    encode_u16(&mut sub.bytecode, *slot);
                    sub.patch_jump(j_skip);
                }
            }
            if !matches!(pat, rusty_js_ast::BindingPattern::Identifier(_)) {
                sub.emit_destructure(pat, *slot)?;
            }
        }
        // IR-EXT 40: emit SetThisTDZ only when the parent compiler
        // explicitly set next_compile_is_derived_ctor (i.e., compile_class
        // is invoking us for the ctor body). The flag is consumed here
        // so nested function/arrow compiles never observe it via the
        // class_stack inheritance path (which would mis-fire — class_stack
        // entries persist through nested sub-compilers).
        let is_derived_ctor = derived_ctor_emit_needed;
        if is_derived_ctor {
            encode_op(&mut sub.bytecode, Op::SetThisTDZ);
        }
        let param_prologue_end = sub.bytecode.len();
        // Tier-Ω.5.zzz: allocate the `arguments` slot. Populated by
        // call_function at invocation with an Array of the actual
        // received arguments. Per ECMA-262 §10.2.4 the slot exists
        // for non-arrow functions; v1 always allocates for any
        // function — arrow bodies will resolve `arguments` via
        // upvalue from the enclosing function's slot anyway, and
        // an unused local in an arrow's own frame costs one Value.
        let arguments_slot = Some(sub.alloc_local(LocalDescriptor {
            name: "arguments".to_string(),
            kind: VariableKind::Var,
            depth: 0,
        }));
        // Tier-Ω.5.kkkkk: self-name slot for named function expressions /
        // declarations. Populated by call_function with the closure object.
        // Per ECMA-262 §15.2.5 the body sees its own name bound to itself.
        let self_name_slot = if let Some(n) = &name {
            // Skip if a parameter already shadows the name — the param wins.
            let already = sub.locals.iter().any(|l| l.name == n.name);
            if !already {
                Some(sub.alloc_local(LocalDescriptor {
                    name: n.name.clone(),
                    kind: VariableKind::Const,
                    depth: 0,
                }))
            } else {
                None
            }
        } else {
            None
        };
        // Tier-Ω.5.ee: function-declaration hoisting per ECMA-262 §10.2.1.3.
        // Two-phase to preserve upvalue resolution: phase H1 pre-allocates
        // ALL top-level var/let/const slots so nested function bodies that
        // capture them via upvalues find the slots during compilation;
        // phase H2 emits MakeClosure + StoreLocal for FunctionDecls so the
        // names are bound before any other statement runs.
        //
        // Without H1, hoisting `function inner() { return x }` before its
        // enclosing function compiles `let x = ...` would make inner's
        // upvalue resolution miss `x` (it would be `let`-allocated later).
        // Tier-Ω.5.vvvvvv: nested-var hoisting per ECMA-262 §9.2.12 (VarScopedDeclarations).
        // `var` is function-scoped, so `var x` inside if/else/loops/try/switch
        // must hoist to the function body — otherwise sibling `var x = ...`
        // declarations in if and else branches allocate separate locals,
        // and the read-back picks whichever was alloc'd last.
        // graceful-fs/fs-extra clone.js does
        //     if (obj instanceof Object) var copy = {...};
        //     else var copy = Object.create(null);
        // and `copy` came back undefined from the if-branch.
        fn collect_var_hoists(stmts: &[Stmt], out: &mut Vec<(String, VariableKind)>) {
            for s in stmts {
                match s {
                    Stmt::Variable(v) if matches!(v.kind, VariableKind::Var) => {
                        for d in &v.declarators {
                            for id in d.target.collect_names() {
                                out.push((id.name.clone(), VariableKind::Var));
                            }
                        }
                    }
                    Stmt::Block { body, .. } => collect_var_hoists(body, out),
                    Stmt::If {
                        consequent,
                        alternate,
                        ..
                    } => {
                        collect_var_hoists(std::slice::from_ref(consequent.as_ref()), out);
                        if let Some(a) = alternate {
                            collect_var_hoists(std::slice::from_ref(a.as_ref()), out);
                        }
                    }
                    Stmt::While { body, .. }
                    | Stmt::With { body, .. }
                    | Stmt::DoWhile { body, .. } => {
                        collect_var_hoists(std::slice::from_ref(body.as_ref()), out);
                    }
                    Stmt::For { init, body, .. } => {
                        // Ω.5.P03.E2.var-hoist-for-init: var declarations in
                        // the for-loop init position (`for (var i = 0; ...)`)
                        // are var-scoped per ECMA-262 §10.2.11
                        // FunctionDeclarationInstantiation step 25-26 and
                        // must be hoisted to the function top. Pre-substrate
                        // the collector recursed only into body, missing the
                        // init. The miss surfaced after Ω.5.P04.E2.strict-
                        // write-enforcement: bn.js's `_parseHex` declares
                        // `var i` in one branch's for-init and reuses `i`
                        // bare in another branch; pre-strict-write the bare
                        // write silently created a global; post-strict-write
                        // it threw ReferenceError because the hoist had
                        // missed the var declaration. Same shape for ForIn
                        // and ForOf left bindings.
                        if let Some(ForInit::Variable(v)) = init {
                            if matches!(v.kind, VariableKind::Var) {
                                for d in &v.declarators {
                                    for id in d.target.collect_names() {
                                        out.push((id.name.clone(), VariableKind::Var));
                                    }
                                }
                            }
                        }
                        collect_var_hoists(std::slice::from_ref(body.as_ref()), out);
                    }
                    Stmt::ForIn { left, body, .. } | Stmt::ForOf { left, body, .. } => {
                        if let ForBinding::Decl {
                            kind: VariableKind::Var,
                            target,
                            ..
                        } = left
                        {
                            for id in target.collect_names() {
                                out.push((id.name.clone(), VariableKind::Var));
                            }
                        }
                        collect_var_hoists(std::slice::from_ref(body.as_ref()), out);
                    }
                    Stmt::Switch { cases, .. } => {
                        for c in cases {
                            collect_var_hoists(&c.consequent, out);
                        }
                    }
                    Stmt::Try {
                        block,
                        handler,
                        finalizer,
                        ..
                    } => {
                        collect_var_hoists(std::slice::from_ref(block.as_ref()), out);
                        if let Some(h) = handler {
                            collect_var_hoists(std::slice::from_ref(&h.body), out);
                        }
                        if let Some(f) = finalizer {
                            collect_var_hoists(std::slice::from_ref(f.as_ref()), out);
                        }
                    }
                    Stmt::Labelled { body, .. } => {
                        collect_var_hoists(std::slice::from_ref(body.as_ref()), out);
                    }
                    _ => {}
                }
            }
        }
        let pre_alloc_names: Vec<(String, VariableKind)> = {
            let mut out = Vec::new();
            for s in body {
                match s {
                    Stmt::Variable(v) => {
                        for d in &v.declarators {
                            for id in d.target.collect_names() {
                                out.push((id.name.clone(), v.kind));
                            }
                        }
                    }
                    Stmt::FunctionDecl { name: Some(n), .. } => {
                        out.push((n.name.clone(), VariableKind::Var));
                    }
                    // Tier-Ω.5.qqqqqq: pre-allocate class-decl names within
                    // function bodies. Without this, function-decls compiled
                    // in Phase H2 that reference module-scope classes via
                    // upvalue can't find the slot — class slots get allocated
                    // in Phase H3 when the class is executed, but H2's MakeClosure
                    // captures freezed at compile time. ajv's CJS wrapper had
                    // `class _Code` + `function _(){ return new _Code(...) }`
                    // both at body scope; `_` lost its `_Code` upvalue.
                    Stmt::ClassDecl { name: Some(n), .. } => {
                        out.push((n.name.clone(), VariableKind::Let));
                    }
                    _ => {}
                }
                // Tier-Ω.5.vvvvvv: also descend into nested control flow
                // and collect any `var`-kinded declarations. Skip the top
                // statement itself (already handled above).
                match s {
                    Stmt::Block { body, .. } => collect_var_hoists(body, &mut out),
                    Stmt::If {
                        consequent,
                        alternate,
                        ..
                    } => {
                        collect_var_hoists(std::slice::from_ref(consequent.as_ref()), &mut out);
                        if let Some(a) = alternate {
                            collect_var_hoists(std::slice::from_ref(a.as_ref()), &mut out);
                        }
                    }
                    Stmt::While { body, .. } | Stmt::DoWhile { body, .. } => {
                        collect_var_hoists(std::slice::from_ref(body.as_ref()), &mut out);
                    }
                    Stmt::For { init, body, .. } => {
                        // Ω.5.P03.E2.var-hoist-for-init: outer-collector
                        // mirror of the same fix in collect_var_hoists —
                        // var declarations in for-loop init are hoisted to
                        // the function top per ECMA §10.2.11 step 25-26.
                        if let Some(ForInit::Variable(v)) = init {
                            if matches!(v.kind, VariableKind::Var) {
                                for d in &v.declarators {
                                    for id in d.target.collect_names() {
                                        out.push((id.name.clone(), VariableKind::Var));
                                    }
                                }
                            }
                        }
                        collect_var_hoists(std::slice::from_ref(body.as_ref()), &mut out);
                    }
                    Stmt::ForIn { left, body, .. } | Stmt::ForOf { left, body, .. } => {
                        if let ForBinding::Decl {
                            kind: VariableKind::Var,
                            target,
                            ..
                        } = left
                        {
                            for id in target.collect_names() {
                                out.push((id.name.clone(), VariableKind::Var));
                            }
                        }
                        collect_var_hoists(std::slice::from_ref(body.as_ref()), &mut out);
                    }
                    Stmt::Switch { cases, .. } => {
                        for c in cases {
                            collect_var_hoists(&c.consequent, &mut out);
                        }
                    }
                    Stmt::Try {
                        block,
                        handler,
                        finalizer,
                        ..
                    } => {
                        collect_var_hoists(std::slice::from_ref(block.as_ref()), &mut out);
                        if let Some(h) = handler {
                            collect_var_hoists(std::slice::from_ref(&h.body), &mut out);
                        }
                        if let Some(f) = finalizer {
                            collect_var_hoists(std::slice::from_ref(f.as_ref()), &mut out);
                        }
                    }
                    Stmt::Labelled { body, .. } => {
                        collect_var_hoists(std::slice::from_ref(body.as_ref()), &mut out);
                    }
                    _ => {}
                }
            }
            out
        };
        let mut pre_slots: std::collections::HashMap<String, u16> =
            std::collections::HashMap::new();
        for (n, kind) in &pre_alloc_names {
            if !pre_slots.contains_key(n) && sub.resolve_local(n).is_none() {
                let slot = sub.alloc_local(LocalDescriptor {
                    name: n.clone(),
                    kind: *kind,
                    depth: 0,
                });
                pre_slots.insert(n.clone(), slot);
            }
        }
        // IR-EXT 23 (TDZ candidate A): at function-body entry, for every
        // pre-allocated let/const slot, emit PushTDZ + StoreLocal so any
        // LoadLocal that fires before the binding's declaration line
        // throws ReferenceError per §13.3.1.1. Var slots stay defaulted
        // to Undefined (var hoists with undefined init); function-decl
        // slots get overwritten by Phase H2's MakeClosure StoreLocal
        // immediately below, so the TDZ init is harmless for them.
        for (_n, slot) in pre_slots.iter() {
            let kind = sub
                .locals
                .get(*slot as usize)
                .map(|d| d.kind)
                .unwrap_or(VariableKind::Var);
            if matches!(kind, VariableKind::Let | VariableKind::Const) {
                encode_op(&mut sub.bytecode, Op::PushTDZ);
                encode_op(&mut sub.bytecode, Op::StoreLocal);
                encode_u16(&mut sub.bytecode, *slot);
            }
        }
        // Phase H2: emit closure-bind for each FunctionDecl into its
        // pre-allocated slot.
        for s in body {
            if let Stmt::FunctionDecl {
                name: Some(n),
                is_async,
                is_generator,
                params,
                body: fn_body,
                ..
            } = s
            {
                let proto = sub.compile_function_proto(
                    Some(n.clone()),
                    *is_async,
                    *is_generator,
                    params,
                    fn_body,
                )?;
                let captures = proto.upvalues.clone();
                let idx = sub.constants.intern(Constant::Function(Box::new(proto)));
                encode_op(&mut sub.bytecode, Op::MakeClosure);
                encode_u16(&mut sub.bytecode, idx);
                emit_captures(&mut sub.bytecode, &captures);
                if let Some(slot) = pre_slots.get(&n.name).copied() {
                    encode_op(&mut sub.bytecode, Op::StoreLocal);
                    encode_u16(&mut sub.bytecode, slot);
                }
            }
        }
        // Phase H3: compile remaining statements. FunctionDecls were already
        // hoisted; skip them. Variable declarations re-bind into their
        // pre-allocated slot rather than allocating a fresh one.
        sub.pre_allocated_slots = pre_slots;
        for s in body {
            if matches!(s, Stmt::FunctionDecl { name: Some(_), .. }) {
                continue;
            }
            sub.compile_stmt(s)?;
        }
        sub.pre_allocated_slots.clear();
        encode_op(&mut sub.bytecode, Op::ReturnUndef);

        // Back-propagate any new upvalues the sub added to intermediate
        // enclosing frames. The innermost enclosing-of-sub is this proto
        // itself, so its upvalues -> self.upvalues. Even-outer frames -> self.enclosing[i].
        let mut frames = sub.enclosing;
        let inner = frames.pop().expect("sub had at least one enclosing");
        self.upvalues = inner.upvalues;
        for (i, ef) in frames.into_iter().enumerate() {
            self.enclosing[i].upvalues = ef.upvalues;
        }

        // ECMA-262 §10.2.10 FunctionLength: stop at the first rest or
        // default-valued parameter. Destructure patterns count as ordinary
        // parameters unless they carry a default.
        let mut function_length: u16 = 0;
        for p in params {
            if p.rest || p.default.is_some() {
                break;
            }
            function_length += 1;
        }
        let display_name = name
            .as_ref()
            .map(|n| n.name.clone())
            .unwrap_or_else(|| display_name_hint.map(|s| s.to_string()).unwrap_or_default());
        Ok(FunctionProto {
            bytecode: sub.bytecode,
            constants: sub.constants,
            params: param_count,
            display_name,
            function_length,
            locals: sub.locals,
            upvalues: sub.upvalues,
            rest_param_slot,
            arguments_slot,
            self_name_slot,
            param_prologue_end,
            is_generator,
            line_starts: sub.source_line_starts,
            source_map: sub.source_map,
            construct_tags: sub.construct_tags,
            source_url: sub.source_url,
            is_async,
            strict: sub.strict,
        })
    }

    fn record_span(&mut self, span: Span) {
        let off = self.bytecode.len();
        if self.source_map.last().map_or(true, |&(_, s)| s != span) {
            self.source_map.push((off, span));
        }
    }

    fn err(&self, span: Span, msg: &str) -> CompileError {
        CompileError {
            span,
            message: msg.to_string(),
        }
    }

    // ───────────────── Tier-Ω.5.k: spread-argument lowering ─────────────────

    /// True if any argument is a spread (`...x`). Drives the choice between
    /// the direct Op::Call/Op::CallMethod/Op::New emit path and the
    /// __apply / __construct helper path.
    fn args_has_spread(arguments: &[Argument]) -> bool {
        arguments
            .iter()
            .any(|a| matches!(a, Argument::Spread { .. }))
    }

    /// Emit code that builds a fresh Array containing the call arguments,
    /// with spread elements expanded via @@iterator. Stack delta: pushes
    /// one Array.
    fn emit_args_array(&mut self, arguments: &[Argument]) -> Result<(), CompileError> {
        encode_op(&mut self.bytecode, Op::NewArray);
        encode_u16(&mut self.bytecode, 0);
        let push_name = self
            .constants
            .intern(Constant::String("__array_push_single".to_string()));
        let extend_name = self
            .constants
            .intern(Constant::String("__array_extend".to_string()));
        for a in arguments {
            match a {
                Argument::Expr(expr) => {
                    // Pre: [.., arr]. Post: [.., arr].
                    encode_op(&mut self.bytecode, Op::LoadGlobal);
                    encode_u16(&mut self.bytecode, push_name);
                    encode_op(&mut self.bytecode, Op::Swap);
                    self.compile_expr(expr)?;
                    encode_op(&mut self.bytecode, Op::Call);
                    encode_u8(&mut self.bytecode, 2);
                }
                Argument::Spread { expr, .. } => {
                    encode_op(&mut self.bytecode, Op::LoadGlobal);
                    encode_u16(&mut self.bytecode, extend_name);
                    encode_op(&mut self.bytecode, Op::Swap);
                    self.compile_expr(expr)?;
                    encode_op(&mut self.bytecode, Op::Call);
                    encode_u8(&mut self.bytecode, 2);
                }
            }
        }
        Ok(())
    }

    // ───────────────── Tier-Ω.5.d: compound assignment + update ─────────────────

    /// Map a compound AssignOp (e.g. AddAssign) to its arithmetic/bitwise
    /// binary opcode. Returns None for the plain `=` form and for the three
    /// short-circuit logical/nullish variants, which are lowered separately.
    fn assign_op_binop(op: AssignOp) -> Option<Op> {
        Some(match op {
            AssignOp::AddAssign => Op::Add,
            AssignOp::SubAssign => Op::Sub,
            AssignOp::MulAssign => Op::Mul,
            AssignOp::DivAssign => Op::Div,
            AssignOp::ModAssign => Op::Mod,
            AssignOp::PowAssign => Op::Pow,
            AssignOp::ShlAssign => Op::Shl,
            AssignOp::ShrAssign => Op::Shr,
            AssignOp::UShrAssign => Op::UShr,
            AssignOp::BitAndAssign => Op::BitAnd,
            AssignOp::BitOrAssign => Op::BitOr,
            AssignOp::BitXorAssign => Op::BitXor,
            AssignOp::Assign
            | AssignOp::LogicalAndAssign
            | AssignOp::LogicalOrAssign
            | AssignOp::NullishAssign => return None,
        })
    }

    fn alloc_temp(&mut self, name: &str) -> u16 {
        self.alloc_local(LocalDescriptor {
            name: name.to_string(),
            kind: VariableKind::Let,
            depth: 0,
        })
    }

    /// Emit load/store for a bare identifier resolved against locals,
    /// upvalues, then globals (in that order).
    fn emit_load_ident(&mut self, name: &str) {
        if self.with_depth > 0 {
            let idx = self.constants.intern(Constant::String(name.to_string()));
            encode_op(&mut self.bytecode, Op::LoadWithName);
            encode_u16(&mut self.bytecode, idx);
        } else if let Some(s) = self.resolve_local(name) {
            encode_op(&mut self.bytecode, Op::LoadLocal);
            encode_u16(&mut self.bytecode, s);
        } else if let Some(u) = self.resolve_upvalue(name) {
            encode_op(&mut self.bytecode, Op::LoadUpvalue);
            encode_u16(&mut self.bytecode, u);
        } else {
            let idx = self.constants.intern(Constant::String(name.to_string()));
            encode_op(&mut self.bytecode, Op::LoadGlobal);
            encode_u16(&mut self.bytecode, idx);
        }
    }

    /// Emit code that throws `new TypeError(msg)` at runtime. Used at
    /// compile-time-detectable spec violations (const reassignment) so
    /// IR-EXT 21 (TDZ self-init): emit `throw new ReferenceError(msg)`
    /// as bytecode. Used at compile-time when a let/const initializer
    /// references its own binding name (e.g. `let y = y`) and the binding
    /// is therefore in TDZ during init evaluation per §13.3.1.1.
    fn emit_throw_referenceerror(&mut self, msg: &str) {
        let ctor_name = self
            .constants
            .intern(Constant::String("ReferenceError".to_string()));
        let msg_idx = self.constants.intern(Constant::String(msg.to_string()));
        encode_op(&mut self.bytecode, Op::LoadGlobal);
        encode_u16(&mut self.bytecode, ctor_name);
        encode_op(&mut self.bytecode, Op::PushConst);
        encode_u16(&mut self.bytecode, msg_idx);
        encode_op(&mut self.bytecode, Op::New);
        self.bytecode.push(1u8);
        encode_op(&mut self.bytecode, Op::Throw);
    }

    /// IR-EXT 21: detect free reference to `name` in an expression at
    /// compile time, skipping nested function/arrow/class scopes (which
    /// shadow). Used to recognize the let/const self-init TDZ pattern.
    fn expr_refs_free(&self, expr: &rusty_js_ast::Expr, name: &str) -> bool {
        use rusty_js_ast::Expr as E;
        match expr {
            E::Identifier { name: n, .. } => n == name,
            E::Parenthesized { expr, .. } => self.expr_refs_free(expr, name),
            E::Member { object, .. } => self.expr_refs_free(object, name),
            E::Call {
                callee, arguments, ..
            }
            | E::New {
                callee, arguments, ..
            } => {
                if self.expr_refs_free(callee, name) {
                    return true;
                }
                arguments.iter().any(|a| match a {
                    rusty_js_ast::Argument::Expr(e) => self.expr_refs_free(e, name),
                    rusty_js_ast::Argument::Spread { expr, .. } => self.expr_refs_free(expr, name),
                })
            }
            E::Update { argument, .. } | E::Unary { argument, .. } => {
                self.expr_refs_free(argument, name)
            }
            E::Binary { left, right, .. } => {
                self.expr_refs_free(left, name) || self.expr_refs_free(right, name)
            }
            E::Conditional {
                test,
                consequent,
                alternate,
                ..
            } => {
                self.expr_refs_free(test, name)
                    || self.expr_refs_free(consequent, name)
                    || self.expr_refs_free(alternate, name)
            }
            E::Assign { target, value, .. } => {
                self.expr_refs_free(target, name) || self.expr_refs_free(value, name)
            }
            E::Sequence { expressions, .. } => {
                expressions.iter().any(|e| self.expr_refs_free(e, name))
            }
            E::Array { elements, .. } => elements.iter().any(|el| match el {
                rusty_js_ast::ArrayElement::Expr(e) => self.expr_refs_free(e, name),
                rusty_js_ast::ArrayElement::Spread { expr, .. } => self.expr_refs_free(expr, name),
                rusty_js_ast::ArrayElement::Elision { .. } => false,
            }),
            E::Object { properties, .. } => properties.iter().any(|p| match p {
                rusty_js_ast::ObjectProperty::Property { value, .. } => {
                    self.expr_refs_free(value, name)
                }
                rusty_js_ast::ObjectProperty::Spread { expr, .. } => {
                    self.expr_refs_free(expr, name)
                }
            }),
            E::TemplateLiteral { expressions, .. } => {
                expressions.iter().any(|e| self.expr_refs_free(e, name))
            }
            // Function / Arrow / Class create their own scopes; their bodies
            // may reference `name` legitimately as a closure capture of the
            // outer binding, which is fine — the binding only needs to be
            // initialized by the time the function is *called*, not by the
            // time the initializer expression evaluates. Skip them.
            E::Function { .. } | E::Arrow { .. } | E::Class { .. } => false,
            _ => false,
        }
    }

    /// the resulting error is JS-catchable rather than a host-level
    /// CompileError. Leaves nothing on the stack (Throw consumes its
    /// operand and unwinds).
    fn emit_throw_typeerror(&mut self, msg: &str) {
        let ctor_name = self
            .constants
            .intern(Constant::String("TypeError".to_string()));
        let msg_idx = self.constants.intern(Constant::String(msg.to_string()));
        encode_op(&mut self.bytecode, Op::LoadGlobal);
        encode_u16(&mut self.bytecode, ctor_name);
        encode_op(&mut self.bytecode, Op::PushConst);
        encode_u16(&mut self.bytecode, msg_idx);
        encode_op(&mut self.bytecode, Op::New);
        self.bytecode.push(1u8); // argc
        encode_op(&mut self.bytecode, Op::Throw);
    }

    fn emit_store_ident(&mut self, name: &str) {
        // with-scoping (remote): with-binding lookup takes precedence over
        // lexical resolution per ECMA-262 §13.11; remote's WithScoping arc
        // introduced Op::StoreWithName for this.
        if self.with_depth > 0 {
            let idx = self.constants.intern(Constant::String(name.to_string()));
            encode_op(&mut self.bytecode, Op::StoreWithName);
            encode_u16(&mut self.bytecode, idx);
        } else if let Some(s) = self.resolve_local(name) {
            // CSC-EXT 2 (compartment-spec-conformance factor 8; also closes
            // ESBC standing-rec ARC.M.7): when in script_mode and the
            // resolved local is a top-level `var` slot, mirror the store
            // to globalThis so subsequent reads via globalThis.X see the
            // assigned value. Caller's outer Dup left one value on the
            // stack for this fn to consume; an extra Dup here preserves
            // the assignment-expression value AFTER both stores.
            let mirror = self.script_mode
                && self.enclosing.is_empty()
                && self.locals.get(s as usize).map_or(false, |ld| {
                    ld.depth == 0 && matches!(ld.kind, rusty_js_ast::VariableKind::Var)
                });
            if mirror {
                encode_op(&mut self.bytecode, Op::Dup);
                encode_op(&mut self.bytecode, Op::StoreLocal);
                encode_u16(&mut self.bytecode, s);
                let idx = self.constants.intern(Constant::String(name.to_string()));
                encode_op(&mut self.bytecode, Op::StoreGlobal);
                encode_u16(&mut self.bytecode, idx);
                return;
            }
            encode_op(&mut self.bytecode, Op::StoreLocal);
            encode_u16(&mut self.bytecode, s);
        } else if let Some(u) = self.resolve_upvalue(name) {
            encode_op(&mut self.bytecode, Op::StoreUpvalue);
            encode_u16(&mut self.bytecode, u);
        } else {
            let idx = self.constants.intern(Constant::String(name.to_string()));
            encode_op(&mut self.bytecode, Op::StoreGlobal);
            encode_u16(&mut self.bytecode, idx);
        }
    }

    fn compile_assign(
        &mut self,
        span: Span,
        operator: AssignOp,
        target: &Expr,
        value: &Expr,
    ) -> Result<(), CompileError> {
        // ── Plain assignment: pre-existing semantics, fast path. ──
        if matches!(operator, AssignOp::Assign) {
            return self.compile_plain_assign(span, target, value);
        }

        // ── Logical / nullish: short-circuit lowering. ──
        if matches!(
            operator,
            AssignOp::LogicalAndAssign | AssignOp::LogicalOrAssign | AssignOp::NullishAssign
        ) {
            return self.compile_logical_assign(span, operator, target, value);
        }

        // ── Arithmetic / bitwise compound: read-modify-write. ──
        let binop = Self::assign_op_binop(operator)
            .expect("non-logical compound assign must map to a binop");

        match target {
            Expr::Identifier { name, .. } => {
                if self.is_const_binding(name) {
                    // Evaluate RHS for side effects, discard, throw TypeError.
                    self.compile_expr(value)?;
                    encode_op(&mut self.bytecode, Op::Pop);
                    self.emit_throw_typeerror(&format!(
                        "Assignment to constant variable '{}'",
                        name
                    ));
                    encode_op(&mut self.bytecode, Op::PushUndef);
                    let _ = span;
                    return Ok(());
                }
                if self.with_depth > 0 {
                    let idx = self.constants.intern(Constant::String(name.clone()));
                    encode_op(&mut self.bytecode, Op::LoadWithNameRef);
                    encode_u16(&mut self.bytecode, idx); // [base, old]
                    self.compile_expr(value)?; // [base, old, v]
                    encode_op(&mut self.bytecode, binop); // [base, new]
                    encode_op(&mut self.bytecode, Op::StoreWithNameRef);
                    encode_u16(&mut self.bytecode, idx); // [new]
                } else {
                    self.emit_load_ident(name); // [old]
                    self.compile_expr(value)?; // [old, v]
                    encode_op(&mut self.bytecode, binop); // [new]
                    encode_op(&mut self.bytecode, Op::Dup); // [new, new]
                    self.emit_store_ident(name); // [new]
                }
            }
            Expr::Member {
                object, property, ..
            } => {
                self.compile_compound_member(span, &**object, property, value, binop)?;
            }
            _ => return Err(self.err(span, "complex assignment target not yet supported")),
        }
        Ok(())
    }

    fn compile_plain_assign(
        &mut self,
        span: Span,
        target: &Expr,
        value: &Expr,
    ) -> Result<(), CompileError> {
        match target {
            Expr::Identifier { name, .. } => {
                // ECMA-262 §13.15.4 + §15.2.7: assignment to a const-bound
                // identifier is a TypeError. Emit code that evaluates the
                // value expression for side-effects, then throws a
                // TypeError. Runtime-emitted so try/catch can catch it
                // (matches bun's surface behavior even though bun catches
                // at parse time).
                if self.is_const_binding(name) {
                    self.compile_expr(value)?;
                    encode_op(&mut self.bytecode, Op::Pop); // discard value
                    self.emit_throw_typeerror(&format!(
                        "Assignment to constant variable '{}'",
                        name
                    ));
                    // Push undefined so any stack-balance assumption downstream
                    // (assign-as-expression yields the assigned value) holds.
                    encode_op(&mut self.bytecode, Op::PushUndef);
                    let _ = span;
                    return Ok(());
                }
                if self.with_depth > 0 {
                    let idx = self.constants.intern(Constant::String(name.clone()));
                    encode_op(&mut self.bytecode, Op::ResolveWithName);
                    encode_u16(&mut self.bytecode, idx);
                    self.compile_expr(value)?;
                    encode_op(&mut self.bytecode, Op::StoreWithNameRef);
                    encode_u16(&mut self.bytecode, idx);
                } else {
                    self.compile_expr(value)?;
                    encode_op(&mut self.bytecode, Op::Dup);
                    if let Some(slot) = self.resolve_local(name) {
                        encode_op(&mut self.bytecode, Op::StoreLocal);
                        encode_u16(&mut self.bytecode, slot);
                    } else if let Some(up) = self.resolve_upvalue(name) {
                        if self.upvalue_is_const(up) {
                            encode_op(&mut self.bytecode, Op::Pop);
                            self.emit_throw_typeerror(&format!(
                                "Assignment to constant variable '{}'",
                                name
                            ));
                            encode_op(&mut self.bytecode, Op::PushUndef);
                        } else {
                            encode_op(&mut self.bytecode, Op::StoreUpvalue);
                            encode_u16(&mut self.bytecode, up);
                        }
                    } else {
                        let idx = self.constants.intern(Constant::String(name.to_string()));
                        encode_op(&mut self.bytecode, Op::StoreGlobal);
                        encode_u16(&mut self.bytecode, idx);
                    }
                }
            }
            Expr::Member {
                object, property, ..
            } => {
                // Tier-Ω.5.f: super.x = v — receiver is `this` (spec: lookup
                // walks HomeObject's proto but the [[Set]] receiver is the
                // calling `this`). Mirror compile_super_member_load's
                // simplification: write directly to `this`. Setters on the
                // proto chain are not invoked; consistent with the load path
                // which doesn't invoke getters with the proper receiver.
                if matches!(object.as_ref(), Expr::Super { .. }) {
                    // Validate we're inside a class with a super reference.
                    let _frame = self
                        .class_stack
                        .last()
                        .cloned()
                        .ok_or_else(|| self.err(span, "super reference outside of a class"))?;
                    encode_op(&mut self.bytecode, Op::PushThis);
                } else {
                    self.compile_expr(object)?;
                }
                match property.as_ref() {
                    MemberProperty::Identifier { name, .. } => {
                        self.compile_expr(value)?;
                        let idx = self.constants.intern(Constant::String(name.clone()));
                        encode_op(&mut self.bytecode, Op::SetProp);
                        encode_u16(&mut self.bytecode, idx);
                    }
                    MemberProperty::Computed { expr, .. } => {
                        self.compile_expr(expr)?;
                        self.compile_expr(value)?;
                        encode_op(&mut self.bytecode, Op::SetIndex);
                    }
                    MemberProperty::Private { name, .. } => {
                        self.compile_expr(value)?;
                        let idx = self
                            .constants
                            .intern(Constant::String(format!("#{}", name)));
                        encode_op(&mut self.bytecode, Op::SetProp);
                        encode_u16(&mut self.bytecode, idx);
                    }
                }
            }
            // Tier-Ω.5.v: parenthesized assignment target — unwrap.
            Expr::Parenthesized { expr, .. } => {
                return self.compile_plain_assign(span, expr, value);
            }
            // Tier-Ω.5.v: destructuring assignment — array/object pattern as
            // an AssignmentTarget (distinct from a binding declaration).
            // The leaves are themselves AssignmentTargets (Identifier /
            // Member / nested pattern), not binding declarations, so we
            // route each leaf through `assign_target_from_value_on_stack`.
            Expr::Array { .. } | Expr::Object { .. } => {
                // Spill RHS into a temp; destructure into the leaves; leave
                // the source value on the stack as the assignment-expr result.
                let src = self.alloc_temp("<destr-assign.src>");
                self.compile_expr(value)?;
                encode_op(&mut self.bytecode, Op::StoreLocal);
                encode_u16(&mut self.bytecode, src);
                self.emit_destructure_assign(target, src)?;
                // Result of the assignment expression is the source value.
                encode_op(&mut self.bytecode, Op::LoadLocal);
                encode_u16(&mut self.bytecode, src);
            }
            _ => return Err(self.err(span, "complex assignment target not yet supported")),
        }
        Ok(())
    }

    /// Tier-Ω.5.v: lower a destructuring **assignment** (LHS is an array or
    /// object literal acting as an AssignmentTarget). Reads from the value
    /// in `src_slot`; each leaf is itself an AssignmentTarget (Identifier,
    /// Member, Parenthesized, or nested Array/Object pattern).
    fn emit_destructure_assign(
        &mut self,
        target: &Expr,
        src_slot: u16,
    ) -> Result<(), CompileError> {
        match target {
            Expr::Parenthesized { expr, .. } => self.emit_destructure_assign(expr, src_slot),
            Expr::Array { elements, .. } => {
                use rusty_js_ast::ArrayElement;
                // IPEP-EXT 1: open iterator from src via __destr_iter_open;
                // per element use __destr_iter_step (symmetric with
                // emit_destructure Array path). §13.15.5.3 RS:DAE.
                let iter_slot = self.alloc_temp("<destr-assign.iter>");
                let done_slot = self.alloc_temp("<destr-assign.iter.done>");
                let open_idx = self
                    .constants
                    .intern(Constant::String("__destr_iter_open".into()));
                encode_op(&mut self.bytecode, Op::LoadGlobal);
                encode_u16(&mut self.bytecode, open_idx);
                encode_op(&mut self.bytecode, Op::LoadLocal);
                encode_u16(&mut self.bytecode, src_slot);
                encode_op(&mut self.bytecode, Op::Call);
                encode_u8(&mut self.bytecode, 1);
                encode_op(&mut self.bytecode, Op::StoreLocal);
                encode_u16(&mut self.bytecode, iter_slot);
                encode_op(&mut self.bytecode, Op::PushFalse);
                encode_op(&mut self.bytecode, Op::StoreLocal);
                encode_u16(&mut self.bytecode, done_slot);
                let mut has_rest = false;
                for el in elements.iter() {
                    match el {
                        ArrayElement::Elision { .. } => {
                            self.emit_iter_step_value(iter_slot, Some(done_slot))?;
                            encode_op(&mut self.bytecode, Op::Pop);
                        }
                        ArrayElement::Expr(leaf) => {
                            self.emit_iter_step_value(iter_slot, Some(done_slot))?;
                            if let Expr::Assign {
                                target: lt,
                                value: dv,
                                operator,
                                ..
                            } = leaf
                            {
                                if matches!(operator, AssignOp::Assign) {
                                    encode_op(&mut self.bytecode, Op::Dup);
                                    encode_op(&mut self.bytecode, Op::PushUndef);
                                    encode_op(&mut self.bytecode, Op::StrictEq);
                                    let j_skip = self.emit_jump(Op::JumpIfFalse);
                                    encode_op(&mut self.bytecode, Op::Pop);
                                    // §13.15.5.3 NamedEvaluation hint.
                                    let hint = if let Expr::Identifier { name, .. } = lt.as_ref() {
                                        Some(name.as_str())
                                    } else {
                                        None
                                    };
                                    if hint.is_some() {
                                        self.compile_expr_with_name_hint(dv, hint)?;
                                    } else {
                                        self.compile_expr(dv)?;
                                    }
                                    self.patch_jump(j_skip);
                                    self.assign_target_from_stack(lt)?;
                                    continue;
                                }
                            }
                            self.assign_target_from_stack(leaf)?;
                        }
                        ArrayElement::Spread {
                            expr: rest_target, ..
                        } => {
                            has_rest = true;
                            let rest_idx = self
                                .constants
                                .intern(Constant::String("__destr_iter_rest".into()));
                            encode_op(&mut self.bytecode, Op::LoadGlobal);
                            encode_u16(&mut self.bytecode, rest_idx);
                            encode_op(&mut self.bytecode, Op::LoadLocal);
                            encode_u16(&mut self.bytecode, iter_slot);
                            encode_op(&mut self.bytecode, Op::Call);
                            encode_u8(&mut self.bytecode, 1);
                            self.assign_target_from_stack(rest_target)?;
                        }
                    }
                }
                if !has_rest {
                    self.emit_iter_close_if_not_done(iter_slot, done_slot)?;
                }
                Ok(())
            }
            Expr::Object { properties, .. } => {
                use rusty_js_ast::{ObjectKey, ObjectProperty};
                self.emit_destructure_object_check(src_slot);
                let mut static_excluded: Vec<String> = Vec::new();
                for prop in properties {
                    match prop {
                        ObjectProperty::Property {
                            key, value: leaf, ..
                        } => {
                            // value = src[key]
                            encode_op(&mut self.bytecode, Op::LoadLocal);
                            encode_u16(&mut self.bytecode, src_slot);
                            match key {
                                ObjectKey::Identifier { name, .. } => {
                                    let k = self.constants.intern(Constant::String(name.clone()));
                                    encode_op(&mut self.bytecode, Op::GetProp);
                                    encode_u16(&mut self.bytecode, k);
                                    static_excluded.push(name.clone());
                                }
                                ObjectKey::String { value, .. } => {
                                    let k = self.constants.intern(Constant::String(value.clone()));
                                    encode_op(&mut self.bytecode, Op::GetProp);
                                    encode_u16(&mut self.bytecode, k);
                                    static_excluded.push(value.clone());
                                }
                                ObjectKey::Number { value, .. } => {
                                    let name = if value.fract() == 0.0 {
                                        format!("{}", *value as i64)
                                    } else {
                                        format!("{}", value)
                                    };
                                    let k = self.constants.intern(Constant::String(name.clone()));
                                    encode_op(&mut self.bytecode, Op::GetProp);
                                    encode_u16(&mut self.bytecode, k);
                                    static_excluded.push(name);
                                }
                                ObjectKey::Computed { expr, .. } => {
                                    self.compile_expr(expr)?;
                                    encode_op(&mut self.bytecode, Op::GetIndex);
                                }
                            }
                            // Default-value support on the leaf.
                            if let Expr::Assign {
                                target: lt,
                                value: dv,
                                operator,
                                ..
                            } = leaf
                            {
                                if matches!(operator, AssignOp::Assign) {
                                    encode_op(&mut self.bytecode, Op::Dup);
                                    encode_op(&mut self.bytecode, Op::PushUndef);
                                    encode_op(&mut self.bytecode, Op::StrictEq);
                                    let j_skip = self.emit_jump(Op::JumpIfFalse);
                                    encode_op(&mut self.bytecode, Op::Pop);
                                    // §13.15.5.3 NamedEvaluation, symmetric with array path.
                                    let hint = if let Expr::Identifier { name, .. } = lt.as_ref() {
                                        Some(name.as_str())
                                    } else {
                                        None
                                    };
                                    if hint.is_some() {
                                        self.compile_expr_with_name_hint(dv, hint)?;
                                    } else {
                                        self.compile_expr(dv)?;
                                    }
                                    self.patch_jump(j_skip);
                                    self.assign_target_from_stack(lt)?;
                                    continue;
                                }
                            }
                            self.assign_target_from_stack(leaf)?;
                        }
                        ObjectProperty::Spread {
                            expr: rest_target, ..
                        } => {
                            // rest = __destr_object_rest(src, excluded)
                            let name_idx = self
                                .constants
                                .intern(Constant::String("__destr_object_rest".into()));
                            encode_op(&mut self.bytecode, Op::LoadGlobal);
                            encode_u16(&mut self.bytecode, name_idx);
                            encode_op(&mut self.bytecode, Op::LoadLocal);
                            encode_u16(&mut self.bytecode, src_slot);
                            encode_op(&mut self.bytecode, Op::NewArray);
                            encode_u16(&mut self.bytecode, static_excluded.len() as u16);
                            for (i, k) in static_excluded.iter().enumerate() {
                                let idx = self.constants.intern(Constant::String(k.clone()));
                                encode_op(&mut self.bytecode, Op::PushConst);
                                encode_u16(&mut self.bytecode, idx);
                                encode_op(&mut self.bytecode, Op::InitIndex);
                                encode_u32(&mut self.bytecode, i as u32);
                            }
                            encode_op(&mut self.bytecode, Op::Call);
                            encode_u8(&mut self.bytecode, 2);
                            self.assign_target_from_stack(rest_target)?;
                        }
                    }
                }
                Ok(())
            }
            // A scalar leaf in destructuring position should not appear here
            // — the caller dispatches on the literal forms. Defensive store.
            _ => self.assign_target_from_stack(target),
        }
    }

    fn emit_destructure_object_check(&mut self, src_slot: u16) {
        let check_idx = self
            .constants
            .intern(Constant::String("__destr_object_check".into()));
        encode_op(&mut self.bytecode, Op::LoadGlobal);
        encode_u16(&mut self.bytecode, check_idx);
        encode_op(&mut self.bytecode, Op::LoadLocal);
        encode_u16(&mut self.bytecode, src_slot);
        encode_op(&mut self.bytecode, Op::Call);
        encode_u8(&mut self.bytecode, 1);
        encode_op(&mut self.bytecode, Op::Pop);
    }

    /// Consume the value on top of the operand stack and assign it to the
    /// given AssignmentTarget. For nested patterns (Array/Object), spills
    /// into a fresh temp and recurses through `emit_destructure_assign`.
    fn assign_target_from_stack(&mut self, target: &Expr) -> Result<(), CompileError> {
        match target {
            Expr::Identifier { name, .. } => {
                self.emit_store_ident(name);
                Ok(())
            }
            Expr::Member {
                object, property, ..
            } => {
                // stack on entry: [value]. We need [object, key?, value] to
                // emit SetProp/SetIndex. Spill value into a temp first.
                let tmp_v = self.alloc_temp("<assign-tgt.v>");
                encode_op(&mut self.bytecode, Op::StoreLocal);
                encode_u16(&mut self.bytecode, tmp_v);
                self.compile_expr(object)?;
                match property.as_ref() {
                    MemberProperty::Identifier { name, .. } => {
                        encode_op(&mut self.bytecode, Op::LoadLocal);
                        encode_u16(&mut self.bytecode, tmp_v);
                        let idx = self.constants.intern(Constant::String(name.clone()));
                        encode_op(&mut self.bytecode, Op::SetProp);
                        encode_u16(&mut self.bytecode, idx);
                    }
                    MemberProperty::Computed { expr, .. } => {
                        self.compile_expr(expr)?;
                        encode_op(&mut self.bytecode, Op::LoadLocal);
                        encode_u16(&mut self.bytecode, tmp_v);
                        encode_op(&mut self.bytecode, Op::SetIndex);
                    }
                    MemberProperty::Private { name, .. } => {
                        encode_op(&mut self.bytecode, Op::LoadLocal);
                        encode_u16(&mut self.bytecode, tmp_v);
                        let idx = self
                            .constants
                            .intern(Constant::String(format!("#{}", name)));
                        encode_op(&mut self.bytecode, Op::SetProp);
                        encode_u16(&mut self.bytecode, idx);
                    }
                }
                // SetProp / SetIndex leaves the assigned value on top — pop it.
                encode_op(&mut self.bytecode, Op::Pop);
                Ok(())
            }
            Expr::Parenthesized { expr, .. } => self.assign_target_from_stack(expr),
            Expr::Array { .. } | Expr::Object { .. } => {
                // Nested pattern: spill into a temp and recurse.
                let tmp = self.alloc_temp("<destr-assign.nested>");
                encode_op(&mut self.bytecode, Op::StoreLocal);
                encode_u16(&mut self.bytecode, tmp);
                self.emit_destructure_assign(target, tmp)
            }
            _ => Err(self.err(target.span(), "complex assignment target not yet supported")),
        }
    }

    /// Compound assignment with a `MemberExpression` target. Spills the
    /// object (and, for computed/index, the key) into temporary locals so
    /// each sub-expression is evaluated exactly once.
    fn compile_compound_member(
        &mut self,
        span: Span,
        object: &Expr,
        property: &MemberProperty,
        value: &Expr,
        binop: Op,
    ) -> Result<(), CompileError> {
        if matches!(object, Expr::Super { .. }) {
            if let Some(frame) = self.class_stack.last().cloned() {
                if let Some(home_name) = frame.super_home_name {
                    return self
                        .compile_super_compound_member(span, &home_name, property, value, binop);
                }
            }
        }

        let tmp_obj = self.alloc_temp("<compound.obj>");
        self.compile_expr(object)?;
        encode_op(&mut self.bytecode, Op::StoreLocal);
        encode_u16(&mut self.bytecode, tmp_obj);

        match property {
            MemberProperty::Identifier { name, .. } => {
                let key_idx = self.constants.intern(Constant::String(name.clone()));
                // read old
                encode_op(&mut self.bytecode, Op::LoadLocal);
                encode_u16(&mut self.bytecode, tmp_obj);
                encode_op(&mut self.bytecode, Op::GetProp);
                encode_u16(&mut self.bytecode, key_idx);
                // compute new
                self.compile_expr(value)?;
                encode_op(&mut self.bytecode, binop);
                // write: [obj, new] then SetProp → [new]
                let tmp_new = self.alloc_temp("<compound.new>");
                encode_op(&mut self.bytecode, Op::StoreLocal);
                encode_u16(&mut self.bytecode, tmp_new);
                encode_op(&mut self.bytecode, Op::LoadLocal);
                encode_u16(&mut self.bytecode, tmp_obj);
                encode_op(&mut self.bytecode, Op::LoadLocal);
                encode_u16(&mut self.bytecode, tmp_new);
                encode_op(&mut self.bytecode, Op::SetProp);
                encode_u16(&mut self.bytecode, key_idx);
            }
            MemberProperty::Computed { expr, .. } => {
                let tmp_key = self.alloc_temp("<compound.key>");
                self.compile_expr(expr)?;
                encode_op(&mut self.bytecode, Op::StoreLocal);
                encode_u16(&mut self.bytecode, tmp_key);
                // read old
                encode_op(&mut self.bytecode, Op::LoadLocal);
                encode_u16(&mut self.bytecode, tmp_obj);
                encode_op(&mut self.bytecode, Op::LoadLocal);
                encode_u16(&mut self.bytecode, tmp_key);
                encode_op(&mut self.bytecode, Op::GetIndex);
                // compute new
                self.compile_expr(value)?;
                encode_op(&mut self.bytecode, binop);
                let tmp_new = self.alloc_temp("<compound.new>");
                encode_op(&mut self.bytecode, Op::StoreLocal);
                encode_u16(&mut self.bytecode, tmp_new);
                // write
                encode_op(&mut self.bytecode, Op::LoadLocal);
                encode_u16(&mut self.bytecode, tmp_obj);
                encode_op(&mut self.bytecode, Op::LoadLocal);
                encode_u16(&mut self.bytecode, tmp_key);
                encode_op(&mut self.bytecode, Op::LoadLocal);
                encode_u16(&mut self.bytecode, tmp_new);
                encode_op(&mut self.bytecode, Op::SetIndex);
            }
            MemberProperty::Private { name, .. } => {
                let key_idx = self
                    .constants
                    .intern(Constant::String(format!("#{}", name)));
                encode_op(&mut self.bytecode, Op::LoadLocal);
                encode_u16(&mut self.bytecode, tmp_obj);
                encode_op(&mut self.bytecode, Op::GetProp);
                encode_u16(&mut self.bytecode, key_idx);
                self.compile_expr(value)?;
                encode_op(&mut self.bytecode, binop);
                let tmp_new = self.alloc_temp("<compound.new>");
                encode_op(&mut self.bytecode, Op::StoreLocal);
                encode_u16(&mut self.bytecode, tmp_new);
                encode_op(&mut self.bytecode, Op::LoadLocal);
                encode_u16(&mut self.bytecode, tmp_obj);
                encode_op(&mut self.bytecode, Op::LoadLocal);
                encode_u16(&mut self.bytecode, tmp_new);
                encode_op(&mut self.bytecode, Op::SetProp);
                encode_u16(&mut self.bytecode, key_idx);
            }
        }
        let _ = span;
        Ok(())
    }

    fn compile_super_compound_member(
        &mut self,
        span: Span,
        home_name: &str,
        property: &MemberProperty,
        value: &Expr,
        binop: Op,
    ) -> Result<(), CompileError> {
        let tmp_base = self.alloc_temp("<super.compound.base>");
        let base_helper = self
            .constants
            .intern(Constant::String("__super_base_home".into()));
        encode_op(&mut self.bytecode, Op::LoadGlobal);
        encode_u16(&mut self.bytecode, base_helper);
        self.emit_load_ident(home_name);
        encode_op(&mut self.bytecode, Op::Call);
        encode_u8(&mut self.bytecode, 1);
        encode_op(&mut self.bytecode, Op::StoreLocal);
        encode_u16(&mut self.bytecode, tmp_base);

        enum SuperKey {
            Static(u16),
            Computed(u16),
        }
        let key = match property {
            MemberProperty::Identifier { name, .. } => {
                SuperKey::Static(self.constants.intern(Constant::String(name.clone())))
            }
            MemberProperty::Computed { expr, .. } => {
                let tmp_key = self.alloc_temp("<super.compound.key>");
                self.compile_expr(expr)?;
                encode_op(&mut self.bytecode, Op::StoreLocal);
                encode_u16(&mut self.bytecode, tmp_key);
                SuperKey::Computed(tmp_key)
            }
            MemberProperty::Private { .. } => {
                return Err(self.err(span, "private super assignment not yet supported"));
            }
        };

        let get_helper = self
            .constants
            .intern(Constant::String("__super_get_base".into()));
        encode_op(&mut self.bytecode, Op::LoadGlobal);
        encode_u16(&mut self.bytecode, get_helper);
        encode_op(&mut self.bytecode, Op::PushThis);
        encode_op(&mut self.bytecode, Op::LoadLocal);
        encode_u16(&mut self.bytecode, tmp_base);
        match key {
            SuperKey::Static(idx) => {
                encode_op(&mut self.bytecode, Op::PushConst);
                encode_u16(&mut self.bytecode, idx);
            }
            SuperKey::Computed(slot) => {
                encode_op(&mut self.bytecode, Op::LoadLocal);
                encode_u16(&mut self.bytecode, slot);
            }
        }
        encode_op(&mut self.bytecode, Op::Call);
        encode_u8(&mut self.bytecode, 3);

        self.compile_expr(value)?;
        encode_op(&mut self.bytecode, binop);
        let tmp_new = self.alloc_temp("<super.compound.new>");
        encode_op(&mut self.bytecode, Op::StoreLocal);
        encode_u16(&mut self.bytecode, tmp_new);

        let set_helper = self
            .constants
            .intern(Constant::String("__super_set".into()));
        encode_op(&mut self.bytecode, Op::LoadGlobal);
        encode_u16(&mut self.bytecode, set_helper);
        encode_op(&mut self.bytecode, Op::LoadLocal);
        encode_u16(&mut self.bytecode, tmp_base);
        encode_op(&mut self.bytecode, Op::PushThis);
        match key {
            SuperKey::Static(idx) => {
                encode_op(&mut self.bytecode, Op::PushConst);
                encode_u16(&mut self.bytecode, idx);
            }
            SuperKey::Computed(slot) => {
                encode_op(&mut self.bytecode, Op::LoadLocal);
                encode_u16(&mut self.bytecode, slot);
            }
        }
        encode_op(&mut self.bytecode, Op::LoadLocal);
        encode_u16(&mut self.bytecode, tmp_new);
        encode_op(&mut self.bytecode, Op::Call);
        encode_u8(&mut self.bytecode, 4);
        Ok(())
    }

    /// Logical / nullish compound assignment. Short-circuits: only
    /// evaluates the RHS and performs the store when the LHS reads as
    /// (truthy / falsy / nullish) appropriate for the operator.
    fn compile_logical_assign(
        &mut self,
        span: Span,
        operator: AssignOp,
        target: &Expr,
        value: &Expr,
    ) -> Result<(), CompileError> {
        // For an identifier target the lowering is:
        //
        //   LoadX                 [x]
        //   Dup                   [x, x]
        //   J<short-circuit> end  (pops top; keeps the other x as result on the
        //                          short-circuit branch)
        //   Pop                   []      (drop the kept copy; we'll replace)
        //   <eval value>          [v]
        //   Dup                   [v, v]
        //   StoreX                [v]
        //   end:                  [result]
        //
        // The trick: the `keep` jump opcodes (JumpIfTrueKeep/JumpIfFalseKeep)
        // keep on jump-taken and pop on fall-through. JumpIfNullish always
        // pops; for ??= we instead route via an unconditional Jump on the
        // not-nullish branch (matching the existing ?? lowering above).

        match target {
            Expr::Identifier { name, .. } => {
                self.emit_load_ident(name);
                let j_end = match operator {
                    AssignOp::LogicalAndAssign => {
                        // Ω.5.P53.E7: JumpIfFalseKeep peeks (doesn't pop).
                        // Truthy path falls through with [x]; falsy path
                        // jumps to end with [x] kept. Either way the stack
                        // ends with one value, so no leading Dup/trailing
                        // Pop is needed (previous lowering left a residual,
                        // breaking f(x ||= v) — the spare value became the
                        // callee). For &&= the assign branch fires on truthy.
                        Some(self.emit_jump(Op::JumpIfFalseKeep))
                    }
                    AssignOp::LogicalOrAssign => Some(self.emit_jump(Op::JumpIfTrueKeep)),
                    AssignOp::NullishAssign => None, // handled below with custom flow
                    _ => unreachable!(),
                };

                if let Some(j) = j_end {
                    // assign branch: drop the kept x, evaluate value,
                    // store with one residual copy for the expression result.
                    encode_op(&mut self.bytecode, Op::Pop);
                    self.compile_expr(value)?;
                    encode_op(&mut self.bytecode, Op::Dup);
                    self.emit_store_ident(name);
                    self.patch_jump(j);
                } else {
                    // NullishAssign: pattern matches the `??` operator in compile_expr.
                    //   [x, x] JumpIfNullish do_assign  (pops top)  → [x]
                    //   Jump end
                    //   do_assign: Pop → []; eval v; Dup; Store     → [v]
                    //   end:                                         → [result]
                    // Ω.5.P53.E7: the leading Dup the parent used to emit
                    // unconditionally was removed; ??= still needs [x, x]
                    // because JumpIfNullish pops its operand. Re-emit it
                    // locally so the non-nullish path retains [x].
                    encode_op(&mut self.bytecode, Op::Dup);
                    let j_assign = self.emit_jump(Op::JumpIfNullish);
                    let j_end2 = self.emit_jump(Op::Jump);
                    self.patch_jump(j_assign);
                    encode_op(&mut self.bytecode, Op::Pop);
                    self.compile_expr(value)?;
                    encode_op(&mut self.bytecode, Op::Dup);
                    self.emit_store_ident(name);
                    self.patch_jump(j_end2);
                }
            }
            Expr::Member {
                object, property, ..
            } => {
                self.compile_logical_assign_member(span, operator, object, property, value)?;
            }
            _ => return Err(self.err(span, "complex assignment target not yet supported")),
        }
        Ok(())
    }

    fn compile_logical_assign_member(
        &mut self,
        span: Span,
        operator: AssignOp,
        object: &Expr,
        property: &MemberProperty,
        value: &Expr,
    ) -> Result<(), CompileError> {
        // Spill object (and key) once, read old, branch, then write iff
        // the short-circuit predicate selects the assign path.
        let tmp_obj = self.alloc_temp("<lcompound.obj>");
        self.compile_expr(object)?;
        encode_op(&mut self.bytecode, Op::StoreLocal);
        encode_u16(&mut self.bytecode, tmp_obj);

        // After this block, the old-value is on the stack as the result
        // on the short-circuit (keep-old) path. We'll then branch.
        enum Key {
            Static(u16),
            Computed(u16),
            Private(u16),
        }
        let key = match property {
            MemberProperty::Identifier { name, .. } => {
                let idx = self.constants.intern(Constant::String(name.clone()));
                encode_op(&mut self.bytecode, Op::LoadLocal);
                encode_u16(&mut self.bytecode, tmp_obj);
                encode_op(&mut self.bytecode, Op::GetProp);
                encode_u16(&mut self.bytecode, idx);
                Key::Static(idx)
            }
            MemberProperty::Computed { expr, .. } => {
                let tmp_key = self.alloc_temp("<lcompound.key>");
                self.compile_expr(expr)?;
                encode_op(&mut self.bytecode, Op::StoreLocal);
                encode_u16(&mut self.bytecode, tmp_key);
                encode_op(&mut self.bytecode, Op::LoadLocal);
                encode_u16(&mut self.bytecode, tmp_obj);
                encode_op(&mut self.bytecode, Op::LoadLocal);
                encode_u16(&mut self.bytecode, tmp_key);
                encode_op(&mut self.bytecode, Op::GetIndex);
                Key::Computed(tmp_key)
            }
            MemberProperty::Private { name, .. } => {
                let idx = self
                    .constants
                    .intern(Constant::String(format!("#{}", name)));
                encode_op(&mut self.bytecode, Op::LoadLocal);
                encode_u16(&mut self.bytecode, tmp_obj);
                encode_op(&mut self.bytecode, Op::GetProp);
                encode_u16(&mut self.bytecode, idx);
                Key::Private(idx)
            }
        };
        // stack: [old]
        encode_op(&mut self.bytecode, Op::Dup);
        // stack: [old, old]

        let j_skip_assign = match operator {
            AssignOp::LogicalAndAssign => Some(self.emit_jump(Op::JumpIfFalseKeep)),
            AssignOp::LogicalOrAssign => Some(self.emit_jump(Op::JumpIfTrueKeep)),
            AssignOp::NullishAssign => None,
            _ => unreachable!(),
        };

        // Emit one "assign branch": pop the kept old copy, eval RHS, write
        // through the member. Leaves the new value on the stack.
        let emit_assign_branch =
            |c: &mut Self, value: &Expr, key: &Key, tmp_obj: u16| -> Result<(), CompileError> {
                encode_op(&mut c.bytecode, Op::Pop);
                c.compile_expr(value)?;
                let tmp_new = c.alloc_temp("<lcompound.new>");
                encode_op(&mut c.bytecode, Op::StoreLocal);
                encode_u16(&mut c.bytecode, tmp_new);
                match key {
                    Key::Static(idx) => {
                        encode_op(&mut c.bytecode, Op::LoadLocal);
                        encode_u16(&mut c.bytecode, tmp_obj);
                        encode_op(&mut c.bytecode, Op::LoadLocal);
                        encode_u16(&mut c.bytecode, tmp_new);
                        encode_op(&mut c.bytecode, Op::SetProp);
                        encode_u16(&mut c.bytecode, *idx);
                    }
                    Key::Computed(tmp_key) => {
                        encode_op(&mut c.bytecode, Op::LoadLocal);
                        encode_u16(&mut c.bytecode, tmp_obj);
                        encode_op(&mut c.bytecode, Op::LoadLocal);
                        encode_u16(&mut c.bytecode, *tmp_key);
                        encode_op(&mut c.bytecode, Op::LoadLocal);
                        encode_u16(&mut c.bytecode, tmp_new);
                        encode_op(&mut c.bytecode, Op::SetIndex);
                    }
                    Key::Private(idx) => {
                        encode_op(&mut c.bytecode, Op::LoadLocal);
                        encode_u16(&mut c.bytecode, tmp_obj);
                        encode_op(&mut c.bytecode, Op::LoadLocal);
                        encode_u16(&mut c.bytecode, tmp_new);
                        encode_op(&mut c.bytecode, Op::SetProp);
                        encode_u16(&mut c.bytecode, *idx);
                    }
                }
                Ok(())
            };

        if let Some(j) = j_skip_assign {
            emit_assign_branch(self, value, &key, tmp_obj)?;
            self.patch_jump(j);
        } else {
            let j_assign = self.emit_jump(Op::JumpIfNullish);
            let j_end = self.emit_jump(Op::Jump);
            self.patch_jump(j_assign);
            emit_assign_branch(self, value, &key, tmp_obj)?;
            self.patch_jump(j_end);
        }
        let _ = span;
        Ok(())
    }

    /// Compile a prefix or postfix update expression. Handles identifier,
    /// static member, computed member, and private member targets.
    fn compile_update(
        &mut self,
        span: Span,
        operator: UpdateOp,
        argument: &Expr,
        prefix: bool,
    ) -> Result<(), CompileError> {
        let op = match operator {
            UpdateOp::Inc => Op::Inc,
            UpdateOp::Dec => Op::Dec,
        };
        match argument {
            Expr::Identifier { name, .. } => {
                self.emit_load_ident(name); // [old]
                if !prefix {
                    encode_op(&mut self.bytecode, Op::Dup); // [old, old]
                }
                encode_op(&mut self.bytecode, op); // prefix:[new]  postfix:[old, new]
                if prefix {
                    encode_op(&mut self.bytecode, Op::Dup); // [new, new]
                }
                // Store consumes top: prefix leaves [new]; postfix leaves [old].
                self.emit_store_ident(name);
            }
            Expr::Member {
                object, property, ..
            } => {
                if matches!(&**object, Expr::Super { .. }) {
                    if let Some(frame) = self.class_stack.last().cloned() {
                        if let Some(home_name) = frame.super_home_name {
                            return self.compile_super_update_member(
                                span, &home_name, property, op, prefix,
                            );
                        }
                    }
                }

                let tmp_obj = self.alloc_temp("<update.obj>");
                self.compile_expr(object)?;
                encode_op(&mut self.bytecode, Op::StoreLocal);
                encode_u16(&mut self.bytecode, tmp_obj);

                match property.as_ref() {
                    MemberProperty::Identifier { name, .. } => {
                        let key_idx = self.constants.intern(Constant::String(name.clone()));
                        encode_op(&mut self.bytecode, Op::LoadLocal);
                        encode_u16(&mut self.bytecode, tmp_obj);
                        encode_op(&mut self.bytecode, Op::GetProp);
                        encode_u16(&mut self.bytecode, key_idx);
                        // [old]
                        let tmp_old = self.alloc_temp("<update.old>");
                        if !prefix {
                            encode_op(&mut self.bytecode, Op::Dup);
                            encode_op(&mut self.bytecode, Op::StoreLocal);
                            encode_u16(&mut self.bytecode, tmp_old);
                        }
                        encode_op(&mut self.bytecode, op); // [new]
                        let tmp_new = self.alloc_temp("<update.new>");
                        encode_op(&mut self.bytecode, Op::StoreLocal);
                        encode_u16(&mut self.bytecode, tmp_new);
                        // write through member
                        encode_op(&mut self.bytecode, Op::LoadLocal);
                        encode_u16(&mut self.bytecode, tmp_obj);
                        encode_op(&mut self.bytecode, Op::LoadLocal);
                        encode_u16(&mut self.bytecode, tmp_new);
                        encode_op(&mut self.bytecode, Op::SetProp);
                        encode_u16(&mut self.bytecode, key_idx);
                        // SetProp pushes new; drop it and load expression result.
                        encode_op(&mut self.bytecode, Op::Pop);
                        encode_op(&mut self.bytecode, Op::LoadLocal);
                        encode_u16(&mut self.bytecode, if prefix { tmp_new } else { tmp_old });
                    }
                    MemberProperty::Computed { expr, .. } => {
                        let tmp_key = self.alloc_temp("<update.key>");
                        self.compile_expr(expr)?;
                        encode_op(&mut self.bytecode, Op::StoreLocal);
                        encode_u16(&mut self.bytecode, tmp_key);

                        encode_op(&mut self.bytecode, Op::LoadLocal);
                        encode_u16(&mut self.bytecode, tmp_obj);
                        encode_op(&mut self.bytecode, Op::LoadLocal);
                        encode_u16(&mut self.bytecode, tmp_key);
                        encode_op(&mut self.bytecode, Op::GetIndex);
                        // [old]
                        let tmp_old = self.alloc_temp("<update.old>");
                        if !prefix {
                            encode_op(&mut self.bytecode, Op::Dup);
                            encode_op(&mut self.bytecode, Op::StoreLocal);
                            encode_u16(&mut self.bytecode, tmp_old);
                        }
                        encode_op(&mut self.bytecode, op);
                        let tmp_new = self.alloc_temp("<update.new>");
                        encode_op(&mut self.bytecode, Op::StoreLocal);
                        encode_u16(&mut self.bytecode, tmp_new);
                        encode_op(&mut self.bytecode, Op::LoadLocal);
                        encode_u16(&mut self.bytecode, tmp_obj);
                        encode_op(&mut self.bytecode, Op::LoadLocal);
                        encode_u16(&mut self.bytecode, tmp_key);
                        encode_op(&mut self.bytecode, Op::LoadLocal);
                        encode_u16(&mut self.bytecode, tmp_new);
                        encode_op(&mut self.bytecode, Op::SetIndex);
                        encode_op(&mut self.bytecode, Op::Pop);
                        encode_op(&mut self.bytecode, Op::LoadLocal);
                        encode_u16(&mut self.bytecode, if prefix { tmp_new } else { tmp_old });
                    }
                    MemberProperty::Private { name, .. } => {
                        let key_idx = self
                            .constants
                            .intern(Constant::String(format!("#{}", name)));
                        encode_op(&mut self.bytecode, Op::LoadLocal);
                        encode_u16(&mut self.bytecode, tmp_obj);
                        encode_op(&mut self.bytecode, Op::GetProp);
                        encode_u16(&mut self.bytecode, key_idx);
                        let tmp_old = self.alloc_temp("<update.old>");
                        if !prefix {
                            encode_op(&mut self.bytecode, Op::Dup);
                            encode_op(&mut self.bytecode, Op::StoreLocal);
                            encode_u16(&mut self.bytecode, tmp_old);
                        }
                        encode_op(&mut self.bytecode, op);
                        let tmp_new = self.alloc_temp("<update.new>");
                        encode_op(&mut self.bytecode, Op::StoreLocal);
                        encode_u16(&mut self.bytecode, tmp_new);
                        encode_op(&mut self.bytecode, Op::LoadLocal);
                        encode_u16(&mut self.bytecode, tmp_obj);
                        encode_op(&mut self.bytecode, Op::LoadLocal);
                        encode_u16(&mut self.bytecode, tmp_new);
                        encode_op(&mut self.bytecode, Op::SetProp);
                        encode_u16(&mut self.bytecode, key_idx);
                        encode_op(&mut self.bytecode, Op::Pop);
                        encode_op(&mut self.bytecode, Op::LoadLocal);
                        encode_u16(&mut self.bytecode, if prefix { tmp_new } else { tmp_old });
                    }
                }
            }
            _ => {
                return Err(self.err(
                    span,
                    "update on non-identifier non-member target not yet supported",
                ))
            }
        }
        Ok(())
    }

    fn compile_super_update_member(
        &mut self,
        span: Span,
        home_name: &str,
        property: &MemberProperty,
        op: Op,
        prefix: bool,
    ) -> Result<(), CompileError> {
        let tmp_base = self.alloc_temp("<super.update.base>");
        let base_helper = self
            .constants
            .intern(Constant::String("__super_base_home".into()));
        encode_op(&mut self.bytecode, Op::LoadGlobal);
        encode_u16(&mut self.bytecode, base_helper);
        self.emit_load_ident(home_name);
        encode_op(&mut self.bytecode, Op::Call);
        encode_u8(&mut self.bytecode, 1);
        encode_op(&mut self.bytecode, Op::StoreLocal);
        encode_u16(&mut self.bytecode, tmp_base);

        enum SuperKey {
            Static(u16),
            Computed(u16),
        }
        let key = match property {
            MemberProperty::Identifier { name, .. } => {
                SuperKey::Static(self.constants.intern(Constant::String(name.clone())))
            }
            MemberProperty::Computed { expr, .. } => {
                let tmp_key = self.alloc_temp("<super.update.key>");
                self.compile_expr(expr)?;
                encode_op(&mut self.bytecode, Op::StoreLocal);
                encode_u16(&mut self.bytecode, tmp_key);
                SuperKey::Computed(tmp_key)
            }
            MemberProperty::Private { .. } => {
                return Err(self.err(span, "private super update not yet supported"));
            }
        };

        let get_helper = self
            .constants
            .intern(Constant::String("__super_get_base".into()));
        encode_op(&mut self.bytecode, Op::LoadGlobal);
        encode_u16(&mut self.bytecode, get_helper);
        encode_op(&mut self.bytecode, Op::PushThis);
        encode_op(&mut self.bytecode, Op::LoadLocal);
        encode_u16(&mut self.bytecode, tmp_base);
        match key {
            SuperKey::Static(idx) => {
                encode_op(&mut self.bytecode, Op::PushConst);
                encode_u16(&mut self.bytecode, idx);
            }
            SuperKey::Computed(slot) => {
                encode_op(&mut self.bytecode, Op::LoadLocal);
                encode_u16(&mut self.bytecode, slot);
            }
        }
        encode_op(&mut self.bytecode, Op::Call);
        encode_u8(&mut self.bytecode, 3);

        let tmp_old = self.alloc_temp("<super.update.old>");
        if !prefix {
            encode_op(&mut self.bytecode, Op::Dup);
            encode_op(&mut self.bytecode, Op::StoreLocal);
            encode_u16(&mut self.bytecode, tmp_old);
        }
        encode_op(&mut self.bytecode, op);
        let tmp_new = self.alloc_temp("<super.update.new>");
        encode_op(&mut self.bytecode, Op::StoreLocal);
        encode_u16(&mut self.bytecode, tmp_new);

        let set_helper = self
            .constants
            .intern(Constant::String("__super_set".into()));
        encode_op(&mut self.bytecode, Op::LoadGlobal);
        encode_u16(&mut self.bytecode, set_helper);
        encode_op(&mut self.bytecode, Op::LoadLocal);
        encode_u16(&mut self.bytecode, tmp_base);
        encode_op(&mut self.bytecode, Op::PushThis);
        match key {
            SuperKey::Static(idx) => {
                encode_op(&mut self.bytecode, Op::PushConst);
                encode_u16(&mut self.bytecode, idx);
            }
            SuperKey::Computed(slot) => {
                encode_op(&mut self.bytecode, Op::LoadLocal);
                encode_u16(&mut self.bytecode, slot);
            }
        }
        encode_op(&mut self.bytecode, Op::LoadLocal);
        encode_u16(&mut self.bytecode, tmp_new);
        encode_op(&mut self.bytecode, Op::Call);
        encode_u8(&mut self.bytecode, 4);
        encode_op(&mut self.bytecode, Op::Pop);
        encode_op(&mut self.bytecode, Op::LoadLocal);
        encode_u16(&mut self.bytecode, if prefix { tmp_new } else { tmp_old });
        Ok(())
    }

    // ───────────────── Tier-Ω.5.f: class lowering ─────────────────

    /// Lower a class declaration / expression. Leaves the class's
    /// constructor function on the operand stack.
    ///
    /// Strategy: a class is sugar over function + prototype + property
    /// installation. The class body emits:
    ///   1. (if extends) evaluate super-class, stash in a hidden local
    ///      `<super.ctor>` and the super prototype in `<super.proto>`.
    ///   2. Allocate the prototype object; if extends, wire its
    ///      [[Prototype]] to the parent's prototype via SetPrototype.
    ///   3. Build the constructor closure (default no-op if absent).
    ///      Bind to a hidden local so methods that capture super can
    ///      land. Wire ctor.prototype = <proto>; proto.constructor = ctor.
    ///      If extends, wire ctor.[[Prototype]] = super-ctor for static
    ///      inheritance.
    ///   4. Install each instance / static method onto its target with
    ///      ClassFrame pushed so super-references resolve to the
    ///      synthesized outer-local names.
    ///
    /// Method-shorthand `super.method` / `super(...)` references in the
    /// method body resolve via the existing upvalue machinery — the
    /// synthesized outer-local names are real entries in `self.locals`,
    /// and the sub-compiler captures them as upvalues per Tier-Ω.5.c.
    fn compile_class(
        &mut self,
        span: Span,
        name: Option<&BindingIdentifier>,
        super_class: Option<&Expr>,
        members: &[ClassMember],
    ) -> Result<(), CompileError> {
        self.compile_class_with_name_hint(span, name, None, super_class, members)
    }

    fn compile_class_with_name_hint(
        &mut self,
        span: Span,
        name: Option<&BindingIdentifier>,
        display_name_hint: Option<&str>,
        super_class: Option<&Expr>,
        members: &[ClassMember],
    ) -> Result<(), CompileError> {
        let seq = self.class_seq;
        self.class_seq += 1;

        // Tier-Ω.5.NNNNNNNN: include source-identifier suffix in the
        // super-ctor slot name so the diagnostic receiver-tag at the
        // GetProp(prototype) failure names the textual identifier from
        // `class X extends Y`. The whole string is used uniformly at
        // both alloc and super-call resolution sites (line ~3753 / ~4006 /
        // ~4020 / ~4029), so exact-name match still works.
        let super_ctor_suffix = match super_class {
            Some(rusty_js_ast::Expr::Identifier { name, .. }) => format!(":{}", name),
            Some(rusty_js_ast::Expr::Member { property, .. }) => match property.as_ref() {
                rusty_js_ast::MemberProperty::Identifier { name, .. }
                | rusty_js_ast::MemberProperty::Private { name, .. } => format!(":.{}", name),
                _ => String::new(),
            },
            _ => String::new(),
        };
        let super_ctor_name = format!("<class${}.super.ctor{}>", seq, super_ctor_suffix);
        let super_proto_name = format!("<class${}.super.proto>", seq);
        let proto_name = format!("<class${}.proto>", seq);
        let ctor_name = format!("<class${}.ctor>", seq);

        // Tier-Ω.5.uuuuu: class-expression self-name binding per ECMA-262
        // §15.6.7. A named class expression binds its name inside the body
        // (methods + static methods) to the class itself. Pre-allocate the
        // slot here so methods compiled below resolve the name to an
        // upvalue pointing at this slot; we'll store the constructed class
        // into the slot once it's built. Marked's b=class l{static parse(){
        // return new l(t).parse(e); }} pattern depends on this.
        let self_name_slot = if let Some(n) = name {
            // Skip if the outer scope already has this name resolving to
            // something — class declarations get handled by the caller
            // and write to the outer binding; here we only handle class
            // EXPRESSIONS, where caller doesn't pre-bind.
            let slot = self.alloc_local(LocalDescriptor {
                name: n.name.clone(),
                kind: VariableKind::Const,
                depth: 0,
            });
            // IR-EXT 27: TDZ-init reverted to Undefined-init at this site.
            // The inner self_name_slot is captured by method-body upvalues
            // during class build; TDZ-initing it broke the slot's resolution
            // semantics during execution of class build. The class-name TDZ
            // during extends is deferred to a future rung that uses a
            // separate scratch slot for the extends-clause TDZ probe instead
            // of repurposing the self_name_slot.
            encode_op(&mut self.bytecode, Op::PushUndef);
            encode_op(&mut self.bytecode, Op::InitLocal);
            encode_u16(&mut self.bytecode, slot);
            Some(slot)
        } else {
            None
        };

        // ── 1. extends evaluation ──────────────────────────────────
        let super_ctor_slot = if let Some(sc) = super_class {
            // Tier-Ω.5.MMMMMMMM (reverted): keeping the synthetic slot name
            // <class$N.super.ctor> here. An earlier draft renamed the slot
            // after the source identifier ("Y [extends]") to enrich the
            // receiver tag at the GetProp(prototype) failure site; the
            // rename triggered a cascade of regressions in derived-class
            // call paths that look up the super-ctor by exact-name match
            // against `<class$N.super.ctor>`. The receiver-tag enrichment
            // is moved to a per-emission probe at the GetProp site instead.
            let slot = self.alloc_temp(&super_ctor_name);
            self.compile_expr(sc)?;
            encode_op(&mut self.bytecode, Op::StoreLocal);
            encode_u16(&mut self.bytecode, slot);
            // <super.proto> = <super.ctor>.prototype
            let proto_slot = self.alloc_temp(&super_proto_name);
            encode_op(&mut self.bytecode, Op::LoadLocal);
            encode_u16(&mut self.bytecode, slot);
            let key_proto = self.constants.intern(Constant::String("prototype".into()));
            encode_op(&mut self.bytecode, Op::GetProp);
            encode_u16(&mut self.bytecode, key_proto);
            encode_op(&mut self.bytecode, Op::StoreLocal);
            encode_u16(&mut self.bytecode, proto_slot);
            Some((slot, proto_slot))
        } else {
            None
        };

        // ── 2. prototype object allocation + extends-wiring ────────
        let proto_slot = self.alloc_temp(&proto_name);
        encode_op(&mut self.bytecode, Op::NewObject);
        encode_op(&mut self.bytecode, Op::StoreLocal);
        encode_u16(&mut self.bytecode, proto_slot);
        if let Some((_sc, sp)) = super_ctor_slot {
            encode_op(&mut self.bytecode, Op::LoadLocal);
            encode_u16(&mut self.bytecode, proto_slot);
            encode_op(&mut self.bytecode, Op::LoadLocal);
            encode_u16(&mut self.bytecode, sp);
            encode_op(&mut self.bytecode, Op::SetPrototype);
        }

        // ── 3. constructor closure ─────────────────────────────────
        //
        // Find an explicit `constructor` member, else synthesize a no-op.
        let mut ctor_params: Vec<Parameter> = Vec::new();
        let mut ctor_body: Vec<Stmt> = Vec::new();
        let mut has_explicit_ctor = false;
        for m in members {
            if let ClassMember::Method {
                kind: MethodKind::Constructor,
                params,
                body,
                ..
            } = m
            {
                ctor_params = params.clone();
                ctor_body = body.clone();
                has_explicit_ctor = true;
                break;
            }
        }

        // Tier-Ω.5.o: synthesize `this.<name> = <init>` statements from
        // instance Field members. Insert at the START of the constructor
        // body. For derived classes without an explicit constructor, also
        // synthesize `super(...args)` ahead of field inits so the parent
        // constructor (and its own field inits) runs first.
        let mut field_init_stmts: Vec<Stmt> = Vec::new();
        for m in members {
            if let ClassMember::Field {
                name: f_name,
                is_static,
                init,
                span: f_span,
            } = m
            {
                if *is_static {
                    continue;
                }
                if let ClassMemberName::Private { name, .. } = f_name {
                    let value = match init {
                        Some(e) => e.clone(),
                        None => Expr::Identifier {
                            name: "undefined".to_string(),
                            span: *f_span,
                        },
                    };
                    let helper_call = Expr::Call {
                        callee: Box::new(Expr::Identifier {
                            name: "__init_private_field__".to_string(),
                            span: *f_span,
                        }),
                        arguments: vec![
                            Argument::Expr(Expr::This { span: *f_span }),
                            Argument::Expr(Expr::StringLiteral {
                                value: format!("#{}", name),
                                span: *f_span,
                            }),
                            Argument::Expr(value),
                        ],
                        optional: false,
                        span: *f_span,
                    };
                    field_init_stmts.push(Stmt::Expression {
                        expr: helper_call,
                        span: *f_span,
                    });
                    continue;
                }
                let key_expr_prop: MemberProperty = match f_name {
                    ClassMemberName::Identifier { name, span } => MemberProperty::Identifier {
                        name: name.clone(),
                        span: *span,
                    },
                    ClassMemberName::String { value, span } => MemberProperty::Computed {
                        expr: Expr::StringLiteral {
                            value: value.clone(),
                            span: *span,
                        },
                        span: *span,
                    },
                    ClassMemberName::Number { value, span } => MemberProperty::Computed {
                        expr: Expr::NumberLiteral {
                            value: *value,
                            span: *span,
                        },
                        span: *span,
                    },
                    ClassMemberName::Computed { expr, span } => MemberProperty::Computed {
                        expr: expr.clone(),
                        span: *span,
                    },
                    ClassMemberName::Private { name, span } => {
                        // Tier-Ω.5.w: private fields v1 — name-mangled to
                        // "__private$<name>" so they're addressable from
                        // mangled paths only. Privacy isn't enforced —
                        // `obj["__private$x"]` would access from outside.
                        // Spec-faithful name mangling + WeakMap-backed
                        // privacy is queued for a substrate round.
                        MemberProperty::Identifier {
                            name: format!("#{}", name),
                            span: *span,
                        }
                    }
                };
                let target = Expr::Member {
                    object: Box::new(Expr::This { span: *f_span }),
                    property: Box::new(key_expr_prop),
                    optional: false,
                    span: *f_span,
                };
                let value = match init {
                    Some(e) => e.clone(),
                    None => Expr::Identifier {
                        name: "undefined".to_string(),
                        span: *f_span,
                    },
                };
                let assign = Expr::Assign {
                    operator: AssignOp::Assign,
                    target: Box::new(target),
                    value: Box::new(value),
                    span: *f_span,
                };
                field_init_stmts.push(Stmt::Expression {
                    expr: assign,
                    span: *f_span,
                });
            }
        }
        if !has_explicit_ctor && super_class.is_some() {
            // Synthesize `constructor(...__args) { super(...__args); <fields>; }`.
            let s = span;
            let args_id = BindingIdentifier {
                name: "__args".to_string(),
                span: s,
            };
            ctor_params = vec![Parameter {
                target: BindingPattern::Identifier(args_id.clone()),
                default: None,
                rest: true,
                span: s,
            }];
            let super_call = Expr::Call {
                callee: Box::new(Expr::Super { span: s }),
                arguments: vec![Argument::Spread {
                    expr: Expr::Identifier {
                        name: "__args".to_string(),
                        span: s,
                    },
                    span: s,
                }],
                optional: false,
                span: s,
            };
            let mut synth: Vec<Stmt> = Vec::new();
            synth.push(Stmt::Expression {
                expr: super_call,
                span: s,
            });
            synth.extend(field_init_stmts.clone());
            ctor_body = synth;
        } else if !field_init_stmts.is_empty() {
            // Ω.5.P03.E2.class-field-after-super: derived-class field
            // initializers must run AFTER super(), not before. Per
            // ECMA-262 §15.7.13 step 11 (and SuperCall step 7), `this`
            // is uninitialized in a derived constructor until super()
            // returns; field initializers reference `this` and the
            // spec inserts them at the InitializeInstanceElements step
            // which runs as part of SuperCall after the parent
            // constructor returns. Pre-substrate cruftless prepended
            // fields to the entire body — for a derived class with an
            // explicit constructor, that placed `this.field = value`
            // expressions BEFORE super(), so writes landed on the
            // pre-allocated `this` which super() then replaced when
            // its parent returned an object (per §15.4.5.4 step 9 /
            // the Callable-pattern in @ark/util). The writes vanished.
            //
            // Fix: if the class has `extends`, find the first top-level
            // statement in the explicit ctor body that contains the
            // super(...) call and insert field inits IMMEDIATELY AFTER
            // it. If no super-call statement is found (a derived ctor
            // that never calls super is a runtime ReferenceError on
            // first `this` access; the bytecode is still well-formed),
            // fall back to prepending so the field inits at least throw
            // on access in the spec-mandated way.
            //
            // For non-derived classes (super_class is None), the
            // legacy prepend is correct: there's no super() to wait
            // for; fields run at the start of construction.
            if super_class.is_some() {
                fn stmt_contains_super_call(s: &Stmt) -> bool {
                    fn expr_contains_super_call(e: &Expr) -> bool {
                        matches!(e, Expr::Call { callee, .. }
                            if matches!(callee.as_ref(), Expr::Super { .. }))
                    }
                    match s {
                        Stmt::Expression { expr, .. } => expr_contains_super_call(expr),
                        _ => false,
                    }
                }
                let mut inserted = false;
                let mut new_body: Vec<Stmt> =
                    Vec::with_capacity(ctor_body.len() + field_init_stmts.len());
                for s in ctor_body.into_iter() {
                    let is_super = !inserted && stmt_contains_super_call(&s);
                    new_body.push(s);
                    if is_super {
                        new_body.extend(field_init_stmts.iter().cloned());
                        inserted = true;
                    }
                }
                if !inserted {
                    // No top-level super(); prepend so any later `this`
                    // access throws as the spec requires.
                    let mut prepended: Vec<Stmt> = field_init_stmts.clone();
                    prepended.extend(new_body.into_iter());
                    new_body = prepended;
                }
                ctor_body = new_body;
            } else {
                let mut new_body: Vec<Stmt> = field_init_stmts.clone();
                new_body.extend(ctor_body.into_iter());
                ctor_body = new_body;
            }
        }

        // Push class context for the constructor body.
        self.class_stack.push(ClassFrame {
            super_ctor_name: super_ctor_slot.map(|_| super_ctor_name.clone()),
            super_proto_name: super_ctor_slot.map(|_| super_proto_name.clone()),
            super_home_name: None,
            in_constructor: true,
            is_static: false,
        });
        let class_display_name = name
            .as_ref()
            .map(|n| n.name.clone())
            .or_else(|| display_name_hint.map(|s| s.to_string()));
        // IR-EXT 40: signal to compile_function_proto that this body is
        // a derived-class ctor that should emit SetThisTDZ at entry.
        // Only set for derived ctors (super_class is Some).
        if super_class.is_some() {
            self.next_compile_is_derived_ctor = true;
        }
        let ctor_proto = self.compile_function_proto_with_name_hint(
            None,
            class_display_name.as_deref(),
            false,
            false,
            &ctor_params,
            &ctor_body,
        )?;
        self.class_stack.pop();
        let ctor_captures = ctor_proto.upvalues.clone();
        let ctor_idx = self
            .constants
            .intern(Constant::Function(Box::new(ctor_proto)));
        encode_op(&mut self.bytecode, Op::MakeClosure);
        encode_u16(&mut self.bytecode, ctor_idx);
        emit_captures(&mut self.bytecode, &ctor_captures);
        let ctor_slot = self.alloc_temp(&ctor_name);
        encode_op(&mut self.bytecode, Op::StoreLocal);
        encode_u16(&mut self.bytecode, ctor_slot);

        // Ω.5.P58.E10: the class binding must be available to static field
        // initializers per ECMA §15.7.10 ClassDefinitionEvaluation step
        // 26 (the classScope binds the class name to F before
        // InitializeBoundName is called on the outer scope). bson's
        // Long.fromInt access inside `static ZERO = Long.fromInt(0)`
        // would otherwise resolve via the outer class-decl slot which
        // gets written only after compile_class returns.
        // Copy ctor_slot → outer class-name slot here so static-init
        // expressions referencing the class name LoadLocal find the
        // ctor object. The outer slot was pre-allocated at line ~1399.
        if let Some(n) = name {
            if let Some(outer_slot) = self.resolve_local(&n.name) {
                if outer_slot != ctor_slot {
                    encode_op(&mut self.bytecode, Op::LoadLocal);
                    encode_u16(&mut self.bytecode, ctor_slot);
                    encode_op(&mut self.bytecode, Op::StoreLocal);
                    encode_u16(&mut self.bytecode, outer_slot);
                }
            }
        }

        // ctor.prototype = <proto>
        let key_proto = self.constants.intern(Constant::String("prototype".into()));
        encode_op(&mut self.bytecode, Op::LoadLocal);
        encode_u16(&mut self.bytecode, ctor_slot);
        encode_op(&mut self.bytecode, Op::LoadLocal);
        encode_u16(&mut self.bytecode, proto_slot);
        encode_op(&mut self.bytecode, Op::SetProp);
        encode_u16(&mut self.bytecode, key_proto);
        encode_op(&mut self.bytecode, Op::Pop);

        // <proto>.constructor = ctor
        // Ω.5.P03.E2.class-method-non-enumerable: constructor is also
        // non-enumerable on a class prototype per ECMA §15.7. Use the
        // __install_method__ helper.
        let key_constructor = self
            .constants
            .intern(Constant::String("constructor".into()));
        let install_helper = self
            .constants
            .intern(Constant::String("__install_method__".into()));
        encode_op(&mut self.bytecode, Op::LoadGlobal);
        encode_u16(&mut self.bytecode, install_helper);
        encode_op(&mut self.bytecode, Op::LoadLocal);
        encode_u16(&mut self.bytecode, proto_slot);
        encode_op(&mut self.bytecode, Op::PushConst);
        encode_u16(&mut self.bytecode, key_constructor);
        encode_op(&mut self.bytecode, Op::LoadLocal);
        encode_u16(&mut self.bytecode, ctor_slot);
        encode_op(&mut self.bytecode, Op::Call);
        encode_u8(&mut self.bytecode, 3);
        encode_op(&mut self.bytecode, Op::Pop);

        // ctor.[[Prototype]] = <super.ctor> for static-method inheritance.
        if let Some((sc, _sp)) = super_ctor_slot {
            encode_op(&mut self.bytecode, Op::LoadLocal);
            encode_u16(&mut self.bytecode, ctor_slot);
            encode_op(&mut self.bytecode, Op::LoadLocal);
            encode_u16(&mut self.bytecode, sc);
            encode_op(&mut self.bytecode, Op::SetPrototype);
        }

        // ── 4. methods + static-init ───────────────────────────────
        //
        // Ω.5.P58.E10: two-pass member processing per ECMA §15.7.10 step
        // 30-31. Methods (including static methods) are installed first;
        // static field initializers and static blocks then run in source
        // order. Pre-P58.E10 a single source-order pass meant a class
        // body like
        //   class Long {
        //     static TWO_PWR_24 = Long.fromInt(...);  // line 1263
        //     ...
        //     static fromInt(value, unsigned) { ... }  // line 1275
        //   }
        // had the static field init run before fromInt was installed.
        // bson.cjs's Long, and any class with field-before-method order,
        // tripped on this. Two passes — pass A installs methods, pass B
        // evaluates fields + static blocks — matches the spec ordering.
        //
        // Two-pass: pass=0 installs methods, pass=1 evaluates static
        // fields + static blocks in source order. Skips at the arm head.
        for pass in 0u8..2 {
            for m in members {
                // Pass-A skip for non-methods; pass-B skip for methods.
                match m {
                    ClassMember::Method { .. } => {
                        if pass != 0 {
                            continue;
                        }
                    }
                    ClassMember::Field { .. } | ClassMember::StaticBlock { .. } => {
                        if pass != 1 {
                            continue;
                        }
                    }
                }
                match m {
                    ClassMember::Method {
                        kind,
                        params,
                        body,
                        name: m_name,
                        is_static,
                        is_async,
                        is_generator,
                        span: m_span,
                    } => {
                        if matches!(kind, MethodKind::Constructor) {
                            continue;
                        }
                        // Tier-Ω.5.u (v1 deviation): getter / setter class members
                        // are lowered as plain function-valued properties on the
                        // prototype (instance) or constructor (static). Real
                        // accessor-descriptor semantics — calling the getter on
                        // property read, calling the setter on property write —
                        // are deferred to the substrate round that wires
                        // Object.defineProperty's get/set fields end-to-end.
                        // Mirrors the object-literal treatment landed in Ω.5.p.parse.
                        // PFRS-EXT 4: class methods preserve async and
                        // generator flags through the shared function-proto
                        // path so async class methods return Promises and
                        // async generators surface the async-generator bridge.
                        let method_key: Option<String> = match m_name {
                            ClassMemberName::Identifier { name, .. } => Some(name.clone()),
                            ClassMemberName::String { value, .. } => Some(value.clone()),
                            ClassMemberName::Number { value, .. } => {
                                Some(if value.fract() == 0.0 {
                                    format!("{}", *value as i64)
                                } else {
                                    format!("{}", value)
                                })
                            }
                            ClassMemberName::Private { name, .. } => {
                                // Tier-Ω.5.w: private method names use the same
                                // "#name" key convention as private fields. The
                                // member-access path on Private already reads
                                // via this key (see MemberProperty::Private
                                // compile sites).
                                Some(format!("#{}", name))
                            }
                            ClassMemberName::Computed { .. } => None,
                        };

                        // Push class context: not the constructor, so super(...)
                        // is forbidden inside the method; super.x is allowed
                        // and resolves through the prototype.
                        self.class_stack.push(ClassFrame {
                            super_ctor_name: super_ctor_slot.map(|_| super_ctor_name.clone()),
                            super_proto_name: super_ctor_slot.map(|_| super_proto_name.clone()),
                            super_home_name: None,
                            in_constructor: false,
                            is_static: *is_static,
                        });
                        let m_proto = self.compile_function_proto_with_name_hint(
                            None,
                            method_key.as_deref(),
                            *is_async,
                            *is_generator,
                            params,
                            body,
                        )?;
                        self.class_stack.pop();
                        let captures = m_proto.upvalues.clone();
                        let m_idx = self.constants.intern(Constant::Function(Box::new(m_proto)));

                        // Push the target object on the stack first, then the
                        // method closure, then SetProp / SetIndex.
                        let target_slot = if *is_static { ctor_slot } else { proto_slot };
                        // Tier-Ω.5.kkkkkk: getter / setter class members install
                        // as real accessor descriptors via __install_accessor__.
                        // Previously they were SetProp'd as data values; reading
                        // `c.value` returned the function instead of calling it.
                        let is_accessor = matches!(kind, MethodKind::Getter | MethodKind::Setter);
                        if is_accessor {
                            if let Some(key) = method_key.as_ref() {
                                // __install_accessor__(target, key, "get"|"set", fn)
                                let helper = self
                                    .constants
                                    .intern(Constant::String("__install_accessor__".into()));
                                encode_op(&mut self.bytecode, Op::LoadGlobal);
                                encode_u16(&mut self.bytecode, helper);
                                encode_op(&mut self.bytecode, Op::LoadLocal);
                                encode_u16(&mut self.bytecode, target_slot);
                                let key_idx = self.constants.intern(Constant::String(key.clone()));
                                encode_op(&mut self.bytecode, Op::PushConst);
                                encode_u16(&mut self.bytecode, key_idx);
                                let kind_str = if matches!(kind, MethodKind::Getter) {
                                    "get"
                                } else {
                                    "set"
                                };
                                let kind_idx =
                                    self.constants.intern(Constant::String(kind_str.into()));
                                encode_op(&mut self.bytecode, Op::PushConst);
                                encode_u16(&mut self.bytecode, kind_idx);
                                encode_op(&mut self.bytecode, Op::MakeClosure);
                                encode_u16(&mut self.bytecode, m_idx);
                                emit_captures(&mut self.bytecode, &captures);
                                encode_op(&mut self.bytecode, Op::Call);
                                encode_u8(&mut self.bytecode, 4);
                                encode_op(&mut self.bytecode, Op::Pop);
                                continue;
                            }
                        }
                        // Ω.5.P03.E2.class-method-non-enumerable: install via
                        // __install_method__(target, key, fn) so the resulting
                        // property descriptor is {w:true, e:false, c:true} per
                        // ECMA-262 §15.7. Pre-substrate, SetProp / SetIndex
                        // installed with enumerable=true, so Object.keys on a
                        // class prototype returned all method names instead of
                        // [], and any code iterating a class prototype's
                        // enumerables picked up methods the spec excludes —
                        // the proximate cause of arktype's wall-4 prototype-as-
                        // this state (Object.values over a registry whose
                        // values were classes would leak the prototype's own
                        // method functions as iteration items).
                        let install_helper = self
                            .constants
                            .intern(Constant::String("__install_method__".into()));
                        encode_op(&mut self.bytecode, Op::LoadGlobal);
                        encode_u16(&mut self.bytecode, install_helper);
                        encode_op(&mut self.bytecode, Op::LoadLocal);
                        encode_u16(&mut self.bytecode, target_slot);
                        match method_key {
                            Some(key) => {
                                let key_idx = self.constants.intern(Constant::String(key));
                                encode_op(&mut self.bytecode, Op::PushConst);
                                encode_u16(&mut self.bytecode, key_idx);
                            }
                            None => {
                                if let ClassMemberName::Computed { expr, .. } = m_name {
                                    self.compile_expr(expr)?;
                                } else {
                                    unreachable!();
                                }
                            }
                        }
                        encode_op(&mut self.bytecode, Op::MakeClosure);
                        encode_u16(&mut self.bytecode, m_idx);
                        emit_captures(&mut self.bytecode, &captures);
                        encode_op(&mut self.bytecode, Op::Call);
                        encode_u8(&mut self.bytecode, 3);
                        encode_op(&mut self.bytecode, Op::Pop);
                    }
                    ClassMember::Field {
                        name: f_name,
                        is_static,
                        init,
                        span: _,
                    } => {
                        // Ω.5.P58.E10 pass B start (see end of pass A's loop
                        // for the spec rationale). Static fields run AFTER
                        // all static methods are installed, in source order.
                        // The pass A→B split keeps the original code below
                        // intact; the same body runs unchanged. Per
                        // ECMA §15.7.10 step 30 (methods) precedes step 31
                        // (fields/blocks).
                        if !*is_static {
                            continue;
                        }
                        encode_op(&mut self.bytecode, Op::LoadLocal);
                        encode_u16(&mut self.bytecode, ctor_slot);
                        let static_key: Option<String> = match f_name {
                            ClassMemberName::Identifier { name, .. }
                            | ClassMemberName::String { value: name, .. } => Some(name.clone()),
                            ClassMemberName::Private { name, .. } => Some(format!("#{}", name)),
                            ClassMemberName::Number { value, .. } => {
                                Some(if value.fract() == 0.0 {
                                    format!("{}", *value as i64)
                                } else {
                                    format!("{}", value)
                                })
                            }
                            ClassMemberName::Computed { .. } => None,
                        };
                        match static_key {
                            Some(key) => {
                                if key.starts_with('#') {
                                    let helper = self
                                        .constants
                                        .intern(Constant::String("__init_private_field__".into()));
                                    encode_op(&mut self.bytecode, Op::LoadGlobal);
                                    encode_u16(&mut self.bytecode, helper);
                                    encode_op(&mut self.bytecode, Op::LoadLocal);
                                    encode_u16(&mut self.bytecode, ctor_slot);
                                    let key_idx = self.constants.intern(Constant::String(key));
                                    encode_op(&mut self.bytecode, Op::PushConst);
                                    encode_u16(&mut self.bytecode, key_idx);
                                    match init {
                                        Some(e) => {
                                            self.class_stack.push(ClassFrame {
                                                super_ctor_name: super_ctor_slot
                                                    .map(|_| super_ctor_name.clone()),
                                                super_proto_name: super_ctor_slot
                                                    .map(|_| super_proto_name.clone()),
                                                super_home_name: None,
                                                in_constructor: false,
                                                is_static: true,
                                            });
                                            let init_body = vec![rusty_js_ast::Stmt::Return {
                                                argument: Some(e.clone()),
                                                span: e.span(),
                                            }];
                                            let init_proto = self.compile_function_proto(
                                                None,
                                                false,
                                                false,
                                                &[],
                                                &init_body,
                                            )?;
                                            self.class_stack.pop();
                                            let captures = init_proto.upvalues.clone();
                                            let idx_proto = self
                                                .constants
                                                .intern(Constant::Function(Box::new(init_proto)));
                                            encode_op(&mut self.bytecode, Op::LoadLocal);
                                            encode_u16(&mut self.bytecode, ctor_slot);
                                            encode_op(&mut self.bytecode, Op::MakeClosure);
                                            encode_u16(&mut self.bytecode, idx_proto);
                                            emit_captures(&mut self.bytecode, &captures);
                                            encode_op(&mut self.bytecode, Op::CallMethod);
                                            encode_u8(&mut self.bytecode, 0);
                                        }
                                        None => {
                                            encode_op(&mut self.bytecode, Op::PushUndef);
                                        }
                                    }
                                    encode_op(&mut self.bytecode, Op::Call);
                                    encode_u8(&mut self.bytecode, 3);
                                    encode_op(&mut self.bytecode, Op::Pop);
                                    continue;
                                }
                                // Order: ctor on stack, then value, then SetProp.
                                // Per ECMA-262 §15.7.10 step 31.b, `this` inside
                                // a static field initializer is the class itself.
                                // Lower the init expression as a 0-arg method
                                // called on the ctor so `this` (and any arrow
                                // capture of `this` therein) resolves correctly.
                                // Mirrors the static-block lowering at L4884.
                                match init {
                                    Some(e) => {
                                        self.class_stack.push(ClassFrame {
                                            super_ctor_name: super_ctor_slot
                                                .map(|_| super_ctor_name.clone()),
                                            super_proto_name: super_ctor_slot
                                                .map(|_| super_proto_name.clone()),
                                            super_home_name: None,
                                            in_constructor: false,
                                            is_static: true,
                                        });
                                        let init_body = vec![rusty_js_ast::Stmt::Return {
                                            argument: Some(e.clone()),
                                            span: e.span(),
                                        }];
                                        let init_proto = self.compile_function_proto(
                                            None,
                                            false,
                                            false,
                                            &[],
                                            &init_body,
                                        )?;
                                        self.class_stack.pop();
                                        let captures = init_proto.upvalues.clone();
                                        let idx_proto = self
                                            .constants
                                            .intern(Constant::Function(Box::new(init_proto)));
                                        encode_op(&mut self.bytecode, Op::LoadLocal);
                                        encode_u16(&mut self.bytecode, ctor_slot);
                                        encode_op(&mut self.bytecode, Op::MakeClosure);
                                        encode_u16(&mut self.bytecode, idx_proto);
                                        emit_captures(&mut self.bytecode, &captures);
                                        encode_op(&mut self.bytecode, Op::CallMethod);
                                        encode_u8(&mut self.bytecode, 0);
                                    }
                                    None => {
                                        encode_op(&mut self.bytecode, Op::PushUndef);
                                    }
                                }
                                let idx = self.constants.intern(Constant::String(key));
                                encode_op(&mut self.bytecode, Op::SetProp);
                                encode_u16(&mut self.bytecode, idx);
                            }
                            None => {
                                // Tier-Ω.5.y: computed class field name —
                                // `class C { static [k] = v }`. SetIndex order:
                                // [ctor, key, value]. ctor is already on stack.
                                if let ClassMemberName::Computed { expr, .. } = f_name {
                                    self.compile_expr(expr)?;
                                } else {
                                    unreachable!();
                                }
                                match init {
                                    Some(e) => self.compile_expr(e)?,
                                    None => {
                                        encode_op(&mut self.bytecode, Op::PushUndef);
                                    }
                                }
                                encode_op(&mut self.bytecode, Op::SetIndex);
                            }
                        }
                        encode_op(&mut self.bytecode, Op::Pop);
                    }
                    ClassMember::StaticBlock {
                        body,
                        span: _b_span,
                    } => {
                        // Ω.5.P49.E5: static initializer blocks per ECMA 2022.
                        // Spec: body runs once at class-evaluation time with `this`
                        // bound to the class constructor. Lower as an anonymous
                        // 0-arg closure called with the ctor as receiver via
                        // CallMethod; this routes the body's `this`-references
                        // (e.g. playwright's `static { this.Events = {...} }`)
                        // through the standard CallMethod receiver path.
                        self.class_stack.push(ClassFrame {
                            super_ctor_name: super_ctor_slot.map(|_| super_ctor_name.clone()),
                            super_proto_name: super_ctor_slot.map(|_| super_proto_name.clone()),
                            super_home_name: None,
                            in_constructor: false,
                            is_static: true,
                        });
                        let block_proto =
                            self.compile_function_proto(None, false, false, &[], body)?;
                        self.class_stack.pop();
                        let captures = block_proto.upvalues.clone();
                        let idx = self
                            .constants
                            .intern(Constant::Function(Box::new(block_proto)));
                        // Stack layout for CallMethod: [receiver, method, ...args].
                        encode_op(&mut self.bytecode, Op::LoadLocal);
                        encode_u16(&mut self.bytecode, ctor_slot);
                        encode_op(&mut self.bytecode, Op::MakeClosure);
                        encode_u16(&mut self.bytecode, idx);
                        emit_captures(&mut self.bytecode, &captures);
                        encode_op(&mut self.bytecode, Op::CallMethod);
                        encode_u8(&mut self.bytecode, 0);
                        encode_op(&mut self.bytecode, Op::Pop);
                    }
                }
            }
        } // end two-pass loop

        // Tier-Ω.5.uuuuu: write the finalized constructor into the
        // self-name slot before pushing as expression value, so methods
        // (which captured the slot as an upvalue) see the real class.
        if let Some(slot) = self_name_slot {
            encode_op(&mut self.bytecode, Op::LoadLocal);
            encode_u16(&mut self.bytecode, ctor_slot);
            // IR-EXT 27: InitLocal to overwrite the TDZ sentinel seeded
            // at the start of class compile.
            encode_op(&mut self.bytecode, Op::InitLocal);
            encode_u16(&mut self.bytecode, slot);
        }
        // ── result: leave the constructor on the stack ─────────────
        encode_op(&mut self.bytecode, Op::LoadLocal);
        encode_u16(&mut self.bytecode, ctor_slot);
        let _ = span;
        Ok(())
    }

    /// Lower `super(args...)` inside a derived-class constructor body.
    /// Emits a method-call on the parent constructor with the current
    /// `this` as receiver. The result is left on the stack (Pop'd by
    /// the surrounding ExpressionStatement).
    fn compile_super_call(
        &mut self,
        span: Span,
        arguments: &[Argument],
    ) -> Result<(), CompileError> {
        let frame = self
            .class_stack
            .last()
            .cloned()
            .ok_or_else(|| self.err(span, "super(...) outside of a class"))?;
        if !frame.in_constructor {
            return Err(self.err(
                span,
                "super(...) is only valid inside a derived-class constructor",
            ));
        }
        let super_ctor_name = frame
            .super_ctor_name
            .clone()
            .ok_or_else(|| self.err(span, "super(...) used in a class with no `extends` clause"))?;
        let n = arguments.len();
        if n > 255 {
            return Err(self.err(span, "too many super-call arguments (>255)"));
        }
        if Self::args_has_spread(arguments) {
            // Ω.5.P03.E2.super-new-target: spread super(...) →
            // __super_apply(super_ctor, this, args). __super_apply is
            // __apply that forwards the calling frame's new.target so
            // the parent ctor invocation gets construct semantics. The
            // PropagateNewTarget op below seeds __super_apply's own
            // current_new_target at frame entry; __super_apply re-emits
            // it into pending_new_target before its inner dispatch.
            let apply_name = self
                .constants
                .intern(Constant::String("__super_apply".to_string()));
            encode_op(&mut self.bytecode, Op::LoadGlobal);
            encode_u16(&mut self.bytecode, apply_name);
            self.emit_load_ident(&super_ctor_name);
            // IR-EXT 40: PushThisRaw re-enabled with the SetThisTDZ emit.
            encode_op(&mut self.bytecode, Op::PushThisRaw);
            self.emit_args_array(arguments)?;
            encode_op(&mut self.bytecode, Op::PropagateNewTarget);
            encode_op(&mut self.bytecode, Op::Call);
            encode_u8(&mut self.bytecode, 3);
        } else {
            // IR-EXT 40: PushThisRaw re-enabled.
            encode_op(&mut self.bytecode, Op::PushThisRaw);
            // Method = parent constructor.
            self.emit_load_ident(&super_ctor_name);
            for a in arguments {
                match a {
                    Argument::Expr(e) => self.compile_expr(e)?,
                    Argument::Spread { .. } => unreachable!(),
                }
            }
            // Ω.5.P03.E2.super-new-target: forward current frame's
            // new.target so the parent ctor invocation gets construct
            // semantics. Without this, super(...) routed via CallMethod
            // with nt_for_this_call=None, the parent saw new.target as
            // undefined, AND the implicit-return-this rule (interp.rs
            // line 7639) didn't fire — so a parent whose body ended in
            // ReturnUndef returned Undefined to the super-call sequence,
            // SetThis ignored the Undefined, and any rebinding done in
            // the parent (Callable-style return-of-non-this) was lost
            // for the derived's this_value chain.
            encode_op(&mut self.bytecode, Op::PropagateNewTarget);
            encode_op(&mut self.bytecode, Op::CallMethod);
            encode_u8(&mut self.bytecode, n as u8);
        }
        // Tier-Ω.5.nnnnn: rebind `this` if super() returned an Object.
        // Per ECMA-262 §15.4.5.4 step 9, when the parent constructor
        // returns an object, that object replaces `this` for the rest
        // of the derived ctor body. Stack flow: CallMethod left [result]
        // → Dup [result, result] → SetThis (pops top, conditionally
        // rebinds) [result]. Final: result on stack as the expression
        // value of `super(...)`.
        encode_op(&mut self.bytecode, Op::Dup);
        encode_op(&mut self.bytecode, Op::SetThis);
        Ok(())
    }

    /// Lower `super.x` (bare read) inside a class method body. Resolves
    /// against the parent prototype (instance methods) or the parent
    /// constructor (static methods).
    fn compile_super_member_load(
        &mut self,
        span: Span,
        property: &MemberProperty,
    ) -> Result<(), CompileError> {
        let frame = self
            .class_stack
            .last()
            .cloned()
            .ok_or_else(|| self.err(span, "super reference outside of a class"))?;
        if let Some(home_name) = frame.super_home_name.clone() {
            let helper = self
                .constants
                .intern(Constant::String("__super_get_home".into()));
            encode_op(&mut self.bytecode, Op::LoadGlobal);
            encode_u16(&mut self.bytecode, helper);
            encode_op(&mut self.bytecode, Op::PushThis);
            self.emit_load_ident(&home_name);
            match property {
                MemberProperty::Identifier { name, .. } => {
                    let idx = self.constants.intern(Constant::String(name.clone()));
                    encode_op(&mut self.bytecode, Op::PushConst);
                    encode_u16(&mut self.bytecode, idx);
                }
                MemberProperty::Computed { expr, .. } => {
                    self.compile_expr(expr)?;
                }
                MemberProperty::Private { name, .. } => {
                    let idx = self
                        .constants
                        .intern(Constant::String(format!("#{}", name)));
                    encode_op(&mut self.bytecode, Op::PushConst);
                    encode_u16(&mut self.bytecode, idx);
                }
            }
            encode_op(&mut self.bytecode, Op::Call);
            encode_u8(&mut self.bytecode, 3);
            return Ok(());
        }
        let target_name = if frame.is_static {
            frame.super_ctor_name.clone()
        } else {
            frame.super_proto_name.clone()
        }
        .ok_or_else(|| self.err(span, "super reference in a class with no `extends` clause"))?;
        // Ω.5.P03.E2.super-get-this: super.X reads dispatch through
        // __super_get(this, super_base, key) so any accessor on the
        // super-base's chain runs with `this = original method's this`
        // per ECMA §13.3.7.3 + §10.1.7.2. Pre-substrate, super.X compiled
        // to `LoadIdent <super.proto>; GetProp X` and Op::GetProp uses
        // the popped object as the accessor receiver — a `get x() {
        // return super.x; }` pattern produced this = super-base inside
        // the inherited getter instead of this = the instance.
        let helper = self
            .constants
            .intern(Constant::String("__super_get".into()));
        encode_op(&mut self.bytecode, Op::LoadGlobal);
        encode_u16(&mut self.bytecode, helper);
        encode_op(&mut self.bytecode, Op::PushThis);
        self.emit_load_ident(&target_name);
        match property {
            MemberProperty::Identifier { name, .. } => {
                let idx = self.constants.intern(Constant::String(name.clone()));
                encode_op(&mut self.bytecode, Op::PushConst);
                encode_u16(&mut self.bytecode, idx);
                encode_op(&mut self.bytecode, Op::Call);
                encode_u8(&mut self.bytecode, 3);
                return Ok(());
            }
            MemberProperty::Computed { expr, .. } => {
                self.compile_expr(expr)?;
                encode_op(&mut self.bytecode, Op::Call);
                encode_u8(&mut self.bytecode, 3);
                return Ok(());
            }
            MemberProperty::Private { name, .. } => {
                let idx = self
                    .constants
                    .intern(Constant::String(format!("#{}", name)));
                encode_op(&mut self.bytecode, Op::PushConst);
                encode_u16(&mut self.bytecode, idx);
                encode_op(&mut self.bytecode, Op::Call);
                encode_u8(&mut self.bytecode, 3);
                return Ok(());
            }
        }
        Ok(())
    }

    /// Lower `super.method(args...)` — a super member-call with the
    /// current `this` as receiver. The method lookup goes through the
    /// parent prototype (instance) or constructor (static).
    fn compile_super_member_call(
        &mut self,
        span: Span,
        property: &MemberProperty,
        arguments: &[Argument],
    ) -> Result<(), CompileError> {
        let n = arguments.len();
        if n > 255 {
            return Err(self.err(span, "too many super-call arguments (>255)"));
        }
        if Self::args_has_spread(arguments) {
            // Tier-Ω.5.k: spread super.m(...) → __apply(method, this, args).
            let apply_name = self
                .constants
                .intern(Constant::String("__apply".to_string()));
            encode_op(&mut self.bytecode, Op::LoadGlobal);
            encode_u16(&mut self.bytecode, apply_name);
            self.compile_super_member_load(span, property)?;
            encode_op(&mut self.bytecode, Op::PushThis);
            self.emit_args_array(arguments)?;
            encode_op(&mut self.bytecode, Op::Call);
            encode_u8(&mut self.bytecode, 3);
        } else {
            // Receiver = current `this`.
            encode_op(&mut self.bytecode, Op::PushThis);
            // Method = (parent prototype | parent ctor) [.property].
            self.compile_super_member_load(span, property)?;
            for a in arguments {
                match a {
                    Argument::Expr(e) => self.compile_expr(e)?,
                    Argument::Spread { .. } => unreachable!(),
                }
            }
            encode_op(&mut self.bytecode, Op::CallMethod);
            encode_u8(&mut self.bytecode, n as u8);
        }
        Ok(())
    }
}

/// SMDR-EXT 1 (2026-05-24, strict-mode-destructuring-refs locale):
/// convert a `BindingPattern` to its equivalent assignment-target
/// `Expr` form. The compiler uses this to route `ForBinding::Pattern`
/// (standalone for-of head, no var/let/const) through the
/// AssignmentPattern semantics (emit_destructure_assign) instead of
/// the BindingPattern semantics (emit_destructure). Returns None when
/// the conversion cannot be safely performed (e.g. nested defaults
/// with rest — rule 14 conservative bail).
fn binding_pattern_to_assignment_expr(
    pat: &rusty_js_ast::BindingPattern,
) -> Option<rusty_js_ast::Expr> {
    use rusty_js_ast::*;
    match pat {
        BindingPattern::Identifier(id) => Some(Expr::Identifier {
            name: id.name.clone(),
            span: id.span,
        }),
        BindingPattern::Array(arr) => {
            let mut elements: Vec<ArrayElement> = Vec::with_capacity(arr.elements.len());
            for elem in &arr.elements {
                match elem {
                    None => elements.push(ArrayElement::Elision { span: arr.span }),
                    Some(be) => {
                        let leaf = binding_pattern_to_assignment_expr(&be.target)?;
                        let expr = if let Some(default) = &be.default {
                            Expr::Assign {
                                target: Box::new(leaf),
                                operator: AssignOp::Assign,
                                value: Box::new(default.clone()),
                                span: be.span,
                            }
                        } else {
                            leaf
                        };
                        elements.push(ArrayElement::Expr(expr));
                    }
                }
            }
            if let Some(rest_pat) = &arr.rest {
                let rest_expr = binding_pattern_to_assignment_expr(rest_pat)?;
                elements.push(ArrayElement::Spread {
                    expr: rest_expr,
                    span: arr.span,
                });
            }
            Some(Expr::Array {
                elements,
                trailing_comma_after_spread: false,
                span: arr.span,
            })
        }
        BindingPattern::Object(obj) => {
            let mut properties: Vec<ObjectProperty> = Vec::with_capacity(obj.properties.len());
            for prop in &obj.properties {
                let value_expr = binding_pattern_to_assignment_expr(&prop.value.target)?;
                let value = if let Some(default) = &prop.value.default {
                    Expr::Assign {
                        target: Box::new(value_expr),
                        operator: AssignOp::Assign,
                        value: Box::new(default.clone()),
                        span: prop.value.span,
                    }
                } else {
                    value_expr
                };
                let key = match &prop.key {
                    PropertyKey::Identifier(id) => ObjectKey::Identifier {
                        name: id.name.clone(),
                        span: id.span,
                    },
                    PropertyKey::String(s) => ObjectKey::String {
                        value: s.as_str().to_string(),
                        span: prop.span,
                    },
                    PropertyKey::Number(n) => ObjectKey::Number {
                        value: *n,
                        span: prop.span,
                    },
                    PropertyKey::Computed(expr) => ObjectKey::Computed {
                        expr: expr.clone(),
                        span: prop.span,
                    },
                };
                properties.push(ObjectProperty::Property {
                    key,
                    value,
                    shorthand: prop.shorthand,
                    kind: ObjectPropertyKind::Init,
                    span: prop.span,
                });
            }
            if let Some(rest_id) = &obj.rest {
                let rest_expr = Expr::Identifier {
                    name: rest_id.name.clone(),
                    span: rest_id.span,
                };
                properties.push(ObjectProperty::Spread {
                    expr: rest_expr,
                    span: obj.span,
                });
            }
            Some(Expr::Object {
                properties,
                span: obj.span,
            })
        }
    }
}

/// VHTB-EXT 1: collect `var` declarators that hoist to the enclosing
/// function/script scope per ECMA-262 §15.2.10 VarScopedDeclarations.
/// Recurses through every non-function syntactic boundary. Skips
/// FunctionDecl + class bodies (those start fresh hoisting scopes) and
/// `let`/`const` (block-scoped, not hoisted).
fn collect_hoisted_var_names(
    stmt: &rusty_js_ast::Stmt,
    out: &mut Vec<(String, rusty_js_ast::VariableKind)>,
) {
    use rusty_js_ast::{ForBinding, ForInit, Stmt, VariableKind};
    match stmt {
        Stmt::Variable(v) if matches!(v.kind, VariableKind::Var) => {
            for d in &v.declarators {
                for id in d.target.collect_names() {
                    out.push((id.name.clone(), v.kind));
                }
            }
        }
        Stmt::Variable(_) => { /* let/const are block-scoped */ }
        Stmt::Block { body, .. } => {
            for s in body {
                collect_hoisted_var_names(s, out);
            }
        }
        Stmt::If {
            consequent,
            alternate,
            ..
        } => {
            collect_hoisted_var_names(consequent, out);
            if let Some(a) = alternate {
                collect_hoisted_var_names(a, out);
            }
        }
        Stmt::For { init, body, .. } => {
            if let Some(ForInit::Variable(v)) = init {
                if matches!(v.kind, VariableKind::Var) {
                    for d in &v.declarators {
                        for id in d.target.collect_names() {
                            out.push((id.name.clone(), v.kind));
                        }
                    }
                }
            }
            collect_hoisted_var_names(body, out);
        }
        Stmt::ForIn { left, body, .. } | Stmt::ForOf { left, body, .. } => {
            if let ForBinding::Decl {
                kind: VariableKind::Var,
                target,
                ..
            } = left
            {
                for id in target.collect_names() {
                    out.push((id.name.clone(), VariableKind::Var));
                }
            }
            collect_hoisted_var_names(body, out);
        }
        Stmt::While { body, .. } | Stmt::With { body, .. } | Stmt::DoWhile { body, .. } => {
            collect_hoisted_var_names(body, out)
        }
        Stmt::Switch { cases, .. } => {
            for c in cases {
                for s in &c.consequent {
                    collect_hoisted_var_names(s, out);
                }
            }
        }
        Stmt::Try {
            block,
            handler,
            finalizer,
            ..
        } => {
            collect_hoisted_var_names(block, out);
            if let Some(h) = handler {
                collect_hoisted_var_names(&h.body, out);
            }
            if let Some(f) = finalizer {
                collect_hoisted_var_names(f, out);
            }
        }
        Stmt::Labelled { body, .. } => collect_hoisted_var_names(body, out),
        // FunctionDecl + ClassDecl start fresh hoisting scopes; do not recurse.
        // Other leaf statements have no nested vars.
        _ => {}
    }
}
