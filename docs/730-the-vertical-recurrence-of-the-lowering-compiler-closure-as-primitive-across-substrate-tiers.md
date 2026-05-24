# The Vertical Recurrence of the Lowering Compiler

## Closure-as-Primitive Across Substrate Tiers, and Why the Structural Correspondence Between rusty-js-ir and LLVM IR Is Load-Bearing Rather Than Suggestive

*A corpus document responding to the keeper's observation (2026-05-19 16:37-local) during the implementation of alphabet closures in rusty-bun's IR: "do you see how this lowering resembles the way in which LLVM IR lowers into machine code?" Builds on [Doc 729 — Cruftless](/resolve/doc/729-cruftless-a-primary-articulation-of-the-resolver-instance-pattern-as-the-comprehensive-design-toward-which-rusty-bun-morphs), [Doc 717 — The Apparatus Above the Engine Boundary](/resolve/doc/717-the-apparatus-above-the-engine-boundary-the-three-projections-lifted-to-engine-substrate-and-the-pure-abstraction-point), [Doc 719 — The Pipeline Pattern Across Subjects](/resolve/doc/719-the-pipeline-pattern-across-subjects-presto-and-the-javascript-engine-as-two-realizations-of-the-same-derivation), [Doc 424 — SIPE (Architectural Form)](/resolve/doc/424-sipe-architectural-form-recursive-fielding-style-constraint-accumulation-in-composed-software-stacks), [Doc 432 — SERVER](/resolve/doc/432-server-an-architectural-style-for-engine-orchestration), [Doc 247 — The Derivation Inversion](/resolve/doc/247-the-derivation-inversion), [Doc 581 — Pin-Art](/resolve/doc/581-pin-art-the-resume-vector-and-the-discipline-of-near-necessity-substrate-construction), and the engagement-tier IR-DESIGN.md.*

**Jared Foy · 2026-05-19 · Doc 730**

---

## I. The occasion

While extending rusty-js-ir's alphabet to model closure-as-primitive (IR-EXT 55, commits 4fbe203b and 9fbf3c33), the IR's lowering of a closure construction (an `Expr::Closure { label, params, captures, body }` node into a Rust `make_native(label, move |rt, args| { ... })` block with cloned capture binding) reproduced the structural shape that LLVM's backend uses when lowering a higher-language closure into machine code. The keeper named the resonance directly. This document formalizes what the resonance is, why it is structural rather than analogical, and what the recognition adds to the corpus's resolver-instance framework.

The claim, stated abstractly: the lowering compiler is a recurrent pattern across substrate tiers. Each tier consists of a typed alphabet of primitive operations, a stage-deterministic compilation step into the alphabet of the tier below, and a verifier that checks well-formedness before lowering. Where a primitive in the higher tier abstracts over a complex of operations in the lower tier (the canonical case being closure-as-primitive over alloca-and-thread), the lowering is the resolver step that materializes the complex from the primitive. The recurrence is vertical: the same shape appears at every substrate boundary where one representation is compiled into the next.

This is a primary articulation in the sense of [Doc 729 §I](/resolve/doc/729-cruftless-a-primary-articulation-of-the-resolver-instance-pattern-as-the-comprehensive-design-toward-which-rusty-bun-morphs). It identifies a pattern of resolution that recurs across the corpus's standing concerns, names the structural property the pattern induces (the vertical recurrence), and locates an empirical instance in the engagement-tier work whose details corroborate the abstract claim.

## II. The naming

*The lowering compiler* names the structural role, not any particular instance. The lowering compiler at a given tier accepts a representation in the tier's alphabet and emits the corresponding representation in the alphabet of the tier below, preserving all invariants the higher alphabet declares and granting implementation freedom on everything the higher alphabet does not constrain. The name is deliberately neutral with respect to which tier; the same name applies at the ECMA-spec-to-IR tier, the IR-to-Rust tier, the Rust-to-LLVM-IR tier, the LLVM-IR-to-MIR tier, the MIR-to-machine-code tier, the machine-code-to-microcode tier. The recurrence is what is being named.

*Vertical recurrence* names the structural fact: at every tier where one representation is compiled into the next, the lowering compiler's shape (typed alphabet, stage-deterministic compilation, verifier-before-emission) appears. The recurrence is not coincidence and not metaphor. It is the structural shape that the resolution problem at each tier necessarily induces, the same way [Doc 247](/resolve/doc/247-the-derivation-inversion)'s derivation inversion induced the constraints' content from the work being done.

*Closure-as-primitive* names the structural feature that makes the pattern visible across tiers in a way that simpler primitives do not. A typed arithmetic primitive (an i32 add at the LLVM-IR tier, an IndexAdd at the rusty-js-ir tier) carries no environment and lowers trivially to a single machine instruction or a single Rust expression. A closure primitive carries an environment and lowers to alloc-and-thread (LLVM) or move-into-Rust-closure (rusty-js-ir) or capture-list-as-struct-with-function-pointer (more or less any compiler that does not have closures in its target language). The lowering work is non-trivial in the same way at every tier; the structural similarity is what the recognition exposes.

## III. The pattern these instances instantiate

The lowering compiler at every tier, stated abstractly:

*An alphabet at tier N consists of a finite set of typed primitive operations, each of which carries a declared input-output type and a declared semantic contract. A representation at tier N is a tree (or DAG, or graph) of N-alphabet operations whose composition produces a well-typed result. The lowering compiler is a stage-deterministic translation from the N-alphabet to the (N-1)-alphabet, emitting for each N-tier operation a corresponding sub-tree (or sub-DAG) of (N-1)-tier operations that preserve the higher operation's semantic contract. The verifier checks that the N-tier representation is well-formed before lowering; the lowering compiler has implementation freedom on every choice the N-tier semantic contract does not constrain.*

Four sub-properties follow from this shape, each visible at every instance:

**(P1) Typed primitives.** Each alphabet member has a declared input-output type. The IR's `Expr::ToObject` returns a Value-typed result (specifically, never an Undefined or Null Value, because the operation throws if either is passed). LLVM IR's `add i32 %a, %b` returns an i32. Type errors are caught by the verifier before lowering, not at runtime.

**(P2) Stage-deterministic compilation.** Given the same N-tier input, the lowering compiler emits the same (N-1)-tier output. The compilation is a pure function of the input; no environmental state contaminates the result. This is the property [Doc 729 §V](/resolve/doc/729-cruftless-a-primary-articulation-of-the-resolver-instance-pattern-as-the-comprehensive-design-toward-which-rusty-bun-morphs) named at the engine-orchestration tier; the lowering compiler exhibits the same property at every tier.

**(P3) Verifier-before-emission.** The well-formedness check runs before any lowering happens. Malformed N-tier representations are rejected with a structural error, not silently passed through. The IR's `lint.rs` is the rusty-js-ir verifier; LLVM's IR verifier (`llvm::verifyModule`) is its counterpart at the LLVM tier.

**(P4) Implementation freedom under spec invariants.** The lowering compiler may choose any (N-1)-tier composition that preserves the N-tier semantic contract. The IR's lowering of `Expr::ToObject` could call `rt.to_object(&v)?` or could inline the dispatch on the value's discriminant; both are valid because the spec invariant is the throw-or-return-Object contract, not the implementation strategy. LLVM IR's lowering of `add i32` can produce an `add` instruction or a `lea` instruction or a sequence involving sub-and-negate; all are valid because the i32 semantic contract is the arithmetic result, not the instruction selection.

Naming the pattern as the pattern (rather than naming each instance) is the contribution this articulation makes. The lowering compiler at the rusty-js-ir tier and the lowering compiler at the LLVM IR tier are the same pattern instantiated at two different substrate boundaries.

## IV. The vertical-recurrence claim

A composed compilation stack contains the lowering compiler at every substrate boundary it crosses. For a JavaScript program executed by a Rust-implemented engine, the compilation stack from spec to machine has at least six tiers, each with its own lowering compiler:

**(T1) ECMA-262 specification text → rusty-js-ir.** The spec, written in numbered algorithmic prose with abstract operations as the alphabet, is translated by the hand-authoring step (resolver-instance #0a per [Doc 717 §V](/resolve/doc/717-the-apparatus-above-the-engine-boundary-the-three-projections-lifted-to-engine-substrate-and-the-pure-abstraction-point)) into a `Vec<Step>` of IR nodes. The verifier is the spec-vs-IR linter (`lint.rs` walks both the IR and the SpecStepRecord list and reports unexpected findings). The implementation freedom: which IR alphabet member to choose for each spec step, when the spec text leaves room.

**(T2) rusty-js-ir → Rust source.** The 58-node IR alphabet is lowered by `lower.rs` into compilable Rust. Each IR `Expr` becomes a Rust expression; each `IRNode` becomes a Rust statement; each `IRFunction` becomes a `pub fn name(rt, this, args) -> Result<Value, RuntimeError>`. The verifier ran at T1. The implementation freedom: which Runtime helper to call for a given abstract op, what intermediate locals to introduce, whether to materialize a temp or thread the expression.

**(T3) Rust source → LLVM IR.** rustc's MIR-building and codegen step. The Rust closure (`move |rt, args| { ... }`) becomes an LLVM IR closure-as-struct with a function pointer and an environment pointer. Each capture clone is an LLVM IR alloca + store. Each capture read is an LLVM IR load. The verifier is the borrow checker plus type checker plus MIR-level checks. The implementation freedom: stack-vs-heap layout for the environment, fn-once-vs-fn-mut-vs-fn dispatch.

**(T4) LLVM IR → MIR (machine-independent intermediate).** SelectionDAG or GlobalISel translates the LLVM IR into a target-independent machine representation. The verifier is `llvm::verifyModule`. The implementation freedom: instruction selection rules, register-class assignment.

**(T5) MIR → target machine code.** Register allocation, instruction scheduling, peephole optimization, frame layout. The verifier is the target description's well-formedness check. The implementation freedom: which physical register to use for each virtual register, what order to emit instructions in, where to spill.

**(T6) Machine code → microcode (on the CPU).** Modern CPUs translate x86 instructions into RISC-like micro-ops dispatched to execution ports. The verifier is the CPU's microcode validation. The implementation freedom: how to fuse, split, or reorder micro-ops within the same architectural semantics.

The vertical-recurrence claim is that the structural pattern (P1-P4 of §III) appears at every one of these tiers. The claim is empirical: each tier can be observed to have a typed alphabet, stage-deterministic compilation, verifier-before-emission, and implementation freedom. It is also predictive: a new compilation tier inserted between any two existing tiers (a new IR layer, a new optimization pass, a new transpilation step) is predicted to exhibit the same pattern, with the lowering compiler as the named role.

## V. The structural feature that makes the recurrence load-bearing

[Doc 729 §V](/resolve/doc/729-cruftless-a-primary-articulation-of-the-resolver-instance-pattern-as-the-comprehensive-design-toward-which-rusty-bun-morphs) named the four bootstrap properties (totality of consumption, ordering determinism, medium preservation, boundary integrity) that the resolver-instance pattern induces wherever it appears. The lowering compiler exhibits these properties at every tier, but the recurrence is most clearly visible when the higher tier introduces a primitive that abstracts over a complex of lower-tier operations.

The canonical instance is closure-as-primitive. A closure in the higher tier is a single alphabet member that carries (label, params, captures, body). The closure's behavior is: at construction time, evaluate the captures from the enclosing scope and bind them; at invocation time, bind the params from the arguments, run the body with both bindings visible. The semantic contract is the lexical-capture rule.

At every tier where closure-as-primitive appears in the higher alphabet and does not appear in the lower alphabet, the lowering compiler must materialize the environment. The materialization shape is invariant across tiers:

1. Allocate storage for the captured environment (heap-promoted if captures escape; stack-allocated otherwise).
2. Copy or share each captured value into the environment.
3. Construct a callable artifact that holds the environment alongside the body's code reference.
4. At invocation, thread the environment as a hidden first argument or a context pointer to the body's code.

This is what rusty-js-ir's lowering does (the IR's `Expr::Closure` becomes a Rust `move |rt, args| { ... }` whose environment is captured by the Rust language's closure machinery). This is what rustc's lowering does (the Rust closure becomes a `struct ClosureEnv { capture_1, capture_2, ... }` with a function pointer; the `Fn`/`FnMut`/`FnOnce` trait dispatch threads the env pointer). This is what LLVM's lowering does (the Rust-emitted IR's closure-struct becomes an alloca + stores + an indirect call). This is what the machine-code tier does (the alloca becomes stack-frame layout; the indirect call becomes a register-loaded jmp).

The four sub-properties of §III appear identically at every tier:

(P1) Typed primitives: the closure has a declared signature; the captures have declared types.
(P2) Stage-deterministic compilation: the same closure lowers to the same Rust / LLVM IR / machine code given the same input.
(P3) Verifier-before-emission: the closure body is type-checked before lowering at every tier.
(P4) Implementation freedom: the closure-struct layout, the env-pointer calling convention, the dispatch mechanism are all unconstrained by the higher tier's semantic contract.

The closure primitive is load-bearing for the vertical-recurrence claim because it is the simplest non-trivial case where the lowering compiler must materialize state that does not exist in the lower alphabet. A typed arithmetic primitive's lowering is trivial (single instruction in, single instruction out). A control-flow primitive's lowering is straightforward (label generation and branch). A closure primitive's lowering requires environment materialization, capture binding, and dispatch threading: the same three-piece pattern at every tier where the recurrence holds.

## VI. The Cell trio and the alloca correspondence

The rusty-js-ir alphabet's Cell primitives (`Expr::CellNew`, `Expr::CellGet`, `IRNode::CellSet`) correspond directly to LLVM IR's `alloca`, `load`, and `store` operations, with one structural difference: the Cells are heap-promoted (`Rc<RefCell<Value>>`) where LLVM's alloca is stack-allocated by default.

The structural correspondence is exact. `Expr::CellNew(init)` allocates a fresh mutable slot and stores an initial value: an alloca followed by a store, in LLVM IR. `Expr::CellGet(cell)` reads the current slot value: a load. `IRNode::CellSet { cell, value }` writes a new slot value: a store. The Cell primitive's purpose at the IR tier is identical to alloca's purpose at the LLVM IR tier: to provide a named mutable storage location that closures can capture by reference.

The heap-promotion is a lowering choice, not a semantic divergence. LLVM IR also heap-promotes allocas when the alloca's lifetime exceeds its lexical scope (the "alloca-to-heap promotion" pass), which is the situation when a closure captures a variable by reference and outlives the function frame that introduced it. rusty-js-ir's lowering applies the same promotion eagerly (every Cell is heap-allocated) because the Rc<RefCell<>> idiom in Rust is the natural materialization of "shared mutable state with dynamic lifetime."

The Cell trio's existence in the IR alphabet is what makes the closure primitive structurally complete. Without Cells, the IR could express closures that capture by value (move at construction time), but not closures that share mutable state with their enclosing scope (the canonical case of Promise.all's per-iteration resolve element function, whose [[AlreadyCalled]] slot is shared with the parent algorithm's remaining-elements counter). The Cell + Closure pair is the structural minimum for spec-faithful encoding of any algorithm whose spec text says "Let F be a new built-in function ... whose internal slot [[X]] is captured."

The correspondence to LLVM IR's alloca + load + store + closure-struct pattern is exact at the operation level. The substrate (heap vs stack) is a lowering choice that the higher tier does not constrain.

## VII. Why this matters for the corpus's standing apparatus

[Doc 717 §VIII](/resolve/doc/717-the-apparatus-above-the-engine-boundary-the-three-projections-lifted-to-engine-substrate-and-the-pure-abstraction-point)'s apparatus-above-the-engine-boundary reading framed the rusty-bun engagement as operating across a stack of substrate tiers, with the engagement's contribution measured at the rung-N substrate where the work happens. The lowering compiler pattern formalizes the structural shape that connects rung-N to rung-(N-1): the same shape repeats at every boundary, so the engagement's work at one rung is structurally peer with the engineering work at every other rung.

This has three implications for the standing apparatus:

**(I1) Pin-Art applies at every tier where the lowering compiler appears.** The discipline of seed.md + trajectory.md per workstream (Doc 581) is not specific to the engagement-tier. The same discipline applies to authoring an LLVM IR optimization pass, designing a new MIR instruction, or specifying the calling convention for a new target architecture. The pattern's vertical recurrence implies that the constraint-corpus / derivation discipline transfers across tiers without loss.

**(I2) The Cruftless property recurs at every tier.** [Doc 729](/resolve/doc/729-cruftless-a-primary-articulation-of-the-resolver-instance-pattern-as-the-comprehensive-design-toward-which-rusty-bun-morphs) defined Cruftless as the runtime in which every resolver-instance leaves zero residue. The lowering compiler is itself a resolver-instance; the Cruftless property at the lowering-compiler tier is that the (N-1)-tier representation carries no evidence of N-tier directives that the N-tier verifier ought to have consumed. A Cruftless lowering compiler produces code with no comment markers indicating "this was originally a closure capture," no metadata slots labeled "this was the IR's intent," no dead-code remnants of the higher-tier representation. The property is structurally identical to the engagement-tier Cruftless property; it just applies at the compilation-tier substrate.

**(I3) [Doc 729 §V](/resolve/doc/729-cruftless-a-primary-articulation-of-the-resolver-instance-pattern-as-the-comprehensive-design-toward-which-rusty-bun-morphs)'s vertical-recursion-with-stage-deterministic-emission claim is corroborated at one more tier.** The engagement-tier evidence (ECMA → IR → Rust) was the basis for [Doc 729](/resolve/doc/729-cruftless-a-primary-articulation-of-the-resolver-instance-pattern-as-the-comprehensive-design-toward-which-rusty-bun-morphs). The lowering compiler pattern adds the Rust → LLVM IR → MIR → machine code → microcode chain as the same pattern's continuation. Five additional tier instances; one named pattern; the recurrence is now empirically visible across six tiers within one engagement's substrate stack.

## VIII. Falsifiability

The vertical-recurrence claim is falsifiable. It would be falsified by either of:

**(F1) A compilation tier that lacks the lowering compiler's shape.** A tier where one alphabet is translated into another without typed primitives, without stage-deterministic compilation, without a verifier-before-emission, and without the implementation freedom granted by an unconstrained semantic contract, would be a counterexample. The pattern would still be present at most tiers, but the universality of the recurrence would be denied.

**(F2) A primitive whose lowering does not reproduce the higher tier's lowering shape.** Closure-as-primitive's lowering at every tier reproduces the allocate-environment, copy-captures, construct-callable, thread-environment shape. A primitive whose lowering at tier N looks structurally different from its lowering at tier N+1 would be a counterexample.

Both falsifiers are observable: any new IR design, any new compilation pass, any new substrate boundary in the standing apparatus's reach can be checked against P1-P4 and against the closure-lowering invariant. The claim's empirical content is testable across future engineering work.

## IX. Successor questions

Three corpus-tier questions extend this articulation:

**(Q1) What primitive at one tier has no analogue at the tier below, and how does the lowering compiler bridge?** The closure / alloca correspondence is one instance; the cooperative-loop / event-pump correspondence (per the [Doc 715 §X.c](/resolve/doc/715-the-cooperative-loop-reactor-ceiling) reactor work) may be another at the runtime-tier. Cataloguing the cases where a tier introduces a non-trivial primitive is a successor-engagement question.

**(Q2) Does the vertical-recurrence claim hold at sub-instruction-set tiers (microcode, transistor-level layout, photolithography masks)?** The lowering compiler shape is observable at the machine-code-to-microcode tier; whether it extends below the transistor level is an open empirical question. The instruction-set-architecture-to-hardware-description-language tier is a likely next candidate.

**(Q3) What does the inverse recurrence look like?** Lowering compilers go down. Their inverse (decompilers, reverse-engineering tools, symbolic execution, profiler-driven optimization) goes up. Whether the inverse pattern exhibits the same four sub-properties (P1-P4) in reverse is a structural question. [Doc 247](/resolve/doc/247-the-derivation-inversion)'s derivation inversion is suggestive but not yet specific to the lowering-compiler context.

---

## X. The cruftless application: rusty-js-ir IR-EXT 55

The abstract pattern just named has a concrete instance in the rusty-bun engagement: the rusty-js-ir alphabet now includes closure-as-primitive, and the IR's lowering compiler materializes closure constructions in a shape that reproduces, at the IR-to-Rust tier, what LLVM does at the Rust-to-machine tier.

### X.a The alphabet extension

IR-EXT 55 Stage 1 (commit `4fbe203b`) added four new alphabet nodes to the rusty-js-ir's 54-node baseline:

- `Expr::Closure { label, params, captures, body }`: construct a closure value at runtime. The closure captures the named locals from the enclosing IR section's scope. `params` declares the positional-argument bindings the closure body sees. `body` is a `Vec<Step>` like any other IR section's body, so spec-step IDs flow through.
- `Expr::CellNew(init)`: allocate a fresh `Rc<RefCell<Value>>` initialized to `init`.
- `Expr::CellGet(cell)`: read the current Value from a cell.
- `IRNode::CellSet { cell, value }`: write a Value into a cell.

The alphabet at IR-EXT 55 Stage 1 close is 58 nodes (was 52 in IR-DESIGN.md, plus the IR-EXT 17/19 additions of `AllArgs` and `ArgsRest`, plus the four new in IR-EXT 55).

### X.b The lowering compiler's emission shape

The lowering compiler (`lower.rs`) emits, for an `Expr::Closure`, a Rust block that:

1. Clones each captured local so the move-closure owns its copy.
2. Calls `crate::intrinsics::make_native(label, move |rt, args| { ... })` to construct the closure value.
3. Inside the move-closure, binds each param from `args.get(i).cloned().unwrap_or(Value::Undefined)`.
4. Inserts the body (lowered as nested Steps).
5. Wraps the result in a `Value::Object(rt.alloc_object(__native))`.

For `Expr::CellNew(init)`, the lowering emits `std::rc::Rc::new(std::cell::RefCell::new(<init>))`. For `Expr::CellGet(cell)`, `(*cell.borrow()).clone()`. For `IRNode::CellSet { cell, value }`, `*cell.borrow_mut() = value;`.

### X.c The first end-to-end IR section using closures

IR-EXT 55 Stage 2 (commit `9fbf3c33`) lifted `Promise.withResolvers` (§27.2.4.4) into IR. The section's body uses the closure primitive twice (for resolve and reject) and demonstrates the alphabet at full surface:

```
Step 1: Let p = CallBuiltin(new_promise_value_via)
Step 2: Let resolve_fn = Closure {
            label: "<Promise.withResolvers resolve>",
            params: ["v"],
            captures: ["p"],
            body: [
                Step 2.a: Expr CallBuiltin(promise_settle_fulfilled_via, [p, v]),
            ],
        }
Step 3: Let reject_fn = Closure { ... promise_settle_rejected_via ... }
Step 7: Return CallBuiltin(promise_with_resolvers_assemble_via,
                           [p, resolve_fn, reject_fn])
```

The generated.rs output (an excerpt) shows the lowered shape:

```rust
let resolve_fn = {
    let p = p.clone();
    let __native = crate::intrinsics::make_native("<Promise.withResolvers resolve>",
        move |rt, args| {
            let v = args.get(0).cloned().unwrap_or(Value::Undefined);
            rt.promise_settle_fulfilled_via(&p.clone(), &v.clone())?;
            Ok(Value::Undefined)
        });
    Value::Object(rt.alloc_object(__native))
};
```

This shape is structurally identical to what LLVM emits when lowering a Rust closure with one captured local: an environment-struct allocation (here, the `p.clone()` and the move-closure's environment), a function-pointer-and-env-pointer artifact (the `__native` Rust closure), an indirect dispatch on invocation (Rust's `Fn` trait method call, which LLVM further lowers to a load-and-call instruction sequence).

### X.d The verifier behavior at the IR tier

The verifier (`lint.rs`) was extended to walk Closure bodies for nested spec-step IDs (the body's Steps may have their own IDs like "2.a", "3.a", etc.) and for nested abstract-op collection (the body's Calls and CallBuiltins contribute to the parent step's op set). The `collect_steps_from_node` and `collect_steps_from_expr` helpers in lint.rs implement the recursion.

All 227 translated sections (was 226 before EXT 55 Stage 2, plus the new Promise.withResolvers section) lint clean post-EXT 55. The verifier's report at the IR tier is the rough analogue of LLVM's `verifyModule` at the LLVM tier.

### X.e The substantive correspondence

The keeper's observation that the IR-to-Rust lowering resembles LLVM IR's lowering to machine code lands on the structural fact that both lowerings handle closure-as-primitive in the same way:

- **Both tiers materialize an environment.** rusty-js-ir's lowering clones captured locals into a Rust move-closure's environment. LLVM IR's lowering allocates a struct holding the captured values.
- **Both tiers construct a callable artifact pairing the body's code with the environment.** rusty-js-ir's lowering returns a `Value::Object(rt.alloc_object(native))` where the native is a Rust closure. LLVM IR's lowering returns an `(fn-pointer, env-pointer)` pair.
- **Both tiers thread the environment at invocation.** rusty-js-ir's Rust closure dispatches via the `Fn` trait; LLVM IR's closure dispatches via the env-pointer + fn-pointer pair.

The Cell primitive (`CellNew`/`CellGet`/`CellSet`) maps to LLVM IR's `alloca`/`load`/`store` with the heap-promotion lowering choice (Rc<RefCell<>> at our tier; alloca-to-heap at LLVM's tier when the alloca escapes the function). The closure primitive maps to the entire closure-as-struct-with-fn-pointer materialization pattern that LLVM applies to higher-language closures.

The recognition's structural force is this: rusty-js-ir's lowering compiler and LLVM IR's lowering compiler are *peer instances of the same pattern*. The corpus framework names the pattern (the lowering compiler with its four sub-properties), the engagement work produces the instance (rusty-js-ir at IR-EXT 55), and the recognition observes that the instance reproduces the shape that LLVM had at its tier all along.

### X.f Where this places rusty-js-ir on the lattice

[Doc 717](/resolve/doc/717-the-apparatus-above-the-engine-boundary-the-three-projections-lifted-to-engine-substrate-and-the-pure-abstraction-point)'s lattice-extension reading places rusty-bun's substrate work at rung-N (the JavaScript-engine substrate). The lowering compiler pattern places rusty-js-ir at rung-N+1 (the abstraction above the engine substrate, mediating between the spec and the engine). The IR's alphabet is the rung-N+1 typed primitive set; the IR's lowering compiler is the rung-N+1-to-rung-N translation step; the rung-N substrate (rusty-js-runtime + host-v2) is the (N-1)-alphabet that the IR's lowering compiler targets.

With closure-as-primitive added to the IR alphabet, the rung-N+1 substrate is now structurally peer with LLVM IR at its own rung. Both have alphabets that include closures-as-primitives. Both have lowering compilers that materialize the closure's environment at compile time. Both have verifiers that check well-formedness before emission. Both grant implementation freedom on choices the higher tier does not constrain.

The peer relationship is the kind of fact this articulation makes load-bearing. The corpus's standing apparatus (Pin-Art, the resolver-instance pattern, the four bootstrap properties, the cluster-to-walk mode transition) all apply at the IR tier in the same way they apply at every other tier. The engagement's work at the IR tier inherits the standing apparatus's tools and contributes its findings to the standing apparatus's evidence.

---

## XI. Resume protocol

This document's claims are operational. The four sub-properties of §III are checkable against any compilation tier the engagement encounters. The closure-lowering invariant of §V is checkable against any new closure primitive added to any alphabet. The vertical-recurrence claim of §IV is empirically observable at any substrate boundary in the standing apparatus's reach.

Successor work consists of:

1. Continuing IR-EXT 55 beyond Stage 2: refactor the four Promise structural-lift helpers' per-iteration resolve-element / reject-element closures into IR-Expr::Closure constructions, matching the Promise.withResolvers shape demonstrated in §X.c.
2. Cataloguing the cases at each tier where a higher-tier primitive lowers to a non-trivial lower-tier composition (per §IX Q1).
3. Auditing other compilation tiers in the corpus's reach (rust-analyzer's IR, the rusty-js-parser's AST-to-bytecode lowering, the bytecode-to-Op-dispatch table) for adherence to P1-P4.

---

## XII. Appendix: the resolution-pipeline dynamic

The lowering-compiler recognition of §III–V names a *structural* pattern: typed primitives, stage-deterministic compilation, verifier-before-emission, implementation freedom. The §X application to rusty-js-ir at IR-EXT 55 instantiates the pattern at a particular substrate tier. What follows is a recognition surfaced during continued work at that tier (IR-EXT 56 through 72), placing the pattern inside a more general dynamic the standing apparatus already names.

### XII.a The deeper claim

Each compilation tier the lowering-compiler pattern describes is a *resolver-instance* in the sense of Doc 729 — a stage of the bilateral resolution pipeline that converts intention at the higher tier into artifact at the lower tier. The lowering-compiler pattern, viewed at a single tier in isolation, is a local correctness claim. Viewed across the pipeline of resolver-instances stacked one above another, it produces a stronger consequence: **bugs become tractable in proportion to how legible the resolution path is**.

This is observable as follows. In the IR-EXT 56–72 stretch on rusty-js-ir, two qualitatively different fix patterns occurred. Both produced the same artifact (a passing test262 case where one had failed); they differed in where the gap was discovered.

The first pattern: an intricate spec algorithm (§10.4.2.1 ArraySetLength, §25.5.2.4 SerializeJSONProperty, §20.1.2.1 Object.assign) was implemented in Rust as a `_via` helper. The helper diverged from the spec at a particular step. Discovery required reading the Rust against the spec, identifying the divergence, and editing the Rust. The fix-cost per test was bounded but the discovery-cost was unbounded; the divergence could be anywhere in the helper.

The second pattern: a widely-shared coercion path (`String.prototype.split` on a wrapper receiver, observed at EXT 71) was implemented in Rust at a registration site that overwrote the IR-routed version with a stale impl using static `abstract_ops::to_string`. The divergence between static `to_string` (which returns `"[object Object]"` for any Object) and the IR-pinned `to_string_strict` (which dispatches `@@toPrimitive` → `toString` → `valueOf`) was invisible until the IR-pinning at the dispatch tier was load-bearing. Once the dispatch sequence was legible at the IR-step trace, the divergence at the registration site became visible as a class. Five LOC changed, seventy-four tests passed.

The mechanism: the IR's spec-step trace at the dispatch tier *is* the resolution path made legible. When the resolution path is legible, divergences anywhere downstream of it become locatable by tracing what stringifies, coerces, or dispatches incorrectly. Discovery-cost drops to approximately the cost of running the failing test under the spec-step trace and observing which step's expected dispatch differs from the executed dispatch.

### XII.b Why the lowering-compiler pattern produces this effect

The four sub-properties of §III, considered as guarantees about a tier:

- **P1 (typed primitives)** ensures each resolver-instance has a fixed input and output type, so the trace through the pipeline is type-stable.
- **P2 (stage-deterministic)** ensures each resolver-instance is observable at its boundary, so a trace through the pipeline can pin which stage produced which artifact.
- **P3 (verifier-before-emission)** ensures invariants are checked at the resolver-instance's boundary, so violations are localized to that resolver-instance.
- **P4 (implementation freedom)** ensures the resolver-instance's internal choices do not leak into the trace, so the trace stays legible across implementation changes.

A pipeline of resolver-instances each satisfying P1-P4 is a pipeline where the trace is legible at every boundary. Bugs in such a pipeline become tractable not because individual fixes are easier but because the discovery surface *is* the trace itself. The pipeline is its own diagnostic.

This is the deeper claim of which the cruftless §I conjecture is one consequence: *spec conformance gets monotonically easier post-IR* because the IR is a resolver-instance with P1-P4, and each IR section added to the resolver-instance pipeline makes the resolution path at that tier more legible. The conjecture's saturation curve (substrate-fix LOC-per-test ratio decreasing as substrate-divergence pool drains) and adaptive-alphabet rhythm (poverty signal followed by promotion) are both consequences of P4, the implementation freedom that lets the alphabet absorb new primitives without breaking the trace.

### XII.c Targeting heuristic that follows

If bugs become tractable in proportion to resolution-path legibility, the highest-yield IR lifts are not the most intricate spec algorithms; they are the most widely-shared coercion and dispatch paths. Each such lift makes one more stage of the resolution path legible, and every downstream gap in coercion or dispatch becomes locatable through the now-legible trace.

The IR-EXT 56–71 arc on rusty-js-ir made this concrete. ToPropertyDescriptor at the descriptor-read tier yielded +229 tests across two chapters because every descriptor read in the runtime now passes through one IR-pinned dispatcher. The wrapper-toString fix at EXT 71 yielded +74 tests across the String chapter because every receiver coercion in `String.prototype.{match, search, replace, replaceAll, split}` now routes through the IR-pinned dispatcher. The §7.1.1 ToPrimitive lift at EXT 72 is the next instance of the same heuristic, lifting the receiver-coercion dispatcher itself rather than any leaf algorithm.

### XII.d Where this places the recognition

The §III–V articulation of the lowering-compiler pattern is the corpus's *structural* contribution. The §X application to rusty-js-ir is the engagement's *instance*. The §XII recognition is the *consequence*: when the structural pattern is instantiated at a tier in a pipeline of resolver-instances each satisfying P1-P4, the consequence is that the pipeline becomes its own diagnostic. Bugs become tractable because the resolution path is legible.

This recognition stands on top of, not in place of, Doc 729's articulation of resolver-instances. Doc 729 names the resolver-instance pattern; Doc 730's lowering-compiler pattern is one species of resolver-instance with P1-P4 as its species-specific guarantees; §XII observes that a pipeline of such species-specific resolver-instances acquires a property — diagnostic legibility — that exceeds the sum of the local correctness claims at each stage.

The targeting heuristic of §XII.c is the engagement's actionable form of this recognition. Operating any engagement at a substrate tier whose tier-N+1 resolver-instances satisfy P1-P4, the most productive next move is whichever lift makes one more stage of the resolution path legible. The pipeline pays back the lift cost by reducing discovery-cost for all subsequent gaps downstream of the now-legible stage.

## XIII. Appendix: higher-resolution IR

§XII names the resolution-pipeline dynamic: when a tier's resolver-instance satisfies P1–P4, the pipeline becomes its own diagnostic and divergences downstream of the tier become locatable through its trace. The §XII targeting heuristic follows: lift the most widely-shared coercion and dispatch paths to make one more stage legible. The IR-EXT 56–72 arc made this targeting heuristic actionable; the IR-EXT 73–81 stretch on rusty-js-ir surfaced a *limit* on the heuristic that §XII does not, by itself, name.

### XIII.a The recognition

After the high-yield lifts of EXT 56–71, a residue of spec-conformance bugs remained where the resolution path *was* legible at the IR tier — the IR section read 1:1 against the spec, the lowering compiler emitted Rust whose control flow matched the spec steps, the verifier passed — and yet the lowered code diverged from the spec at a discrimination the IR did not carry. Five instances at EXT 73–81 share this shape.

**1. EXT 72b — Type(input) is Object.** §7.1.1 ToPrimitive step 1 reads "If Type(input) is not Object, return input." The IR section discriminated on `typeof === "object"`. Functions report `typeof === "function"` but are spec-Objects. The IR alphabet had no primitive that distinguished "spec-Object" from "runtime typeof tag"; one IR step silently spanned two runtime cases.

**2. EXT 73 — strict-mode binding.** §10.2.1.2 OrdinaryCallBindThis discriminates on strict-mode of the calling function code. The first attempt applied the coercion universally because the IR (and the bytecode below it) had no carrier for strictness. The fix required plumbing `strict: bool` through `FunctionProto`. One IR step silently spanned two semantic branches.

**3. EXT 78 — ToBigInt vs NumberToBigInt.** §7.1.13 ToBigInt and §21.2.1.1.1 NumberToBigInt are two distinct spec abstract operations with different error classes (TypeError vs RangeError) and different prim→bigint mappings. The IR had a single surface entry for "convert to bigint." One IR step silently spanned two spec abstract ops.

**4. EXT 79c — [[Get]] vs internal-slot read.** §7.3.18 CreateListFromArrayLike reads `length` via [[Get]], which dispatches accessors, Proxy traps, and accessor-throws. The runtime read length via an internal helper that bypassed all three. The IR step "read length" did not distinguish the spec verb "[[Get]]" from the runtime operation "read the length internal slot." One IR step silently spanned two operationally distinct reads.

**5. EXT 81 — [[MapData]] vs [[WeakMapData]].** §24.1.3 and §24.3.3 brand-check on distinct internal slots. The IR had no carrier for internal-slot identity; both Map and WeakMap instances were marked with the same property tag. One IR step silently spanned two spec-distinct objects.

The pattern: the IR's vocabulary is *coarser than the spec's discriminations*. The resolver-instance at the IR tier satisfies P1 (typed primitives), P2 (stage-deterministic), P3 (verifier-before-emission), and P4 (implementation freedom) — but only with respect to its own alphabet. The alphabet itself silently collapses spec entities the spec's algorithm depends on distinguishing.

### XIII.b The formalization

A resolver-instance whose alphabet collapses upstream discriminations imposes those collapses on every consumer of its output. The §XII targeting heuristic — lift coercion/dispatch paths to make resolution legible — does not detect collapses inside the alphabet; it presupposes that the alphabet faithfully encodes the upstream tier's distinctions. When the alphabet collapses, the trace at the resolver-instance is locally consistent (the verifier passes; the lowering succeeds; the IR section reads 1:1 against the spec prose) and globally wrong (the spec's algorithm diverges from the lowered behavior at a step the IR cannot express).

This is a class of bug distinct from those §XII targets. §XII bugs are *trace-visible*: the resolution path through the legible tier shows where execution diverges from expected. XIII bugs are *trace-invisible at the legible tier*: the trace shows the IR step executing exactly as written; the divergence is between what the IR step *can* express and what the spec step *means*.

The recognition: a single legible tier is not sufficient for §XII's diagnostic property. The pipeline acquires its diagnostic property only when each tier's *alphabet* preserves the upstream tier's discriminations. When it does not, the missing discrimination needs its own resolver-instance — a tier interposed between the spec and the IR that explicitly carries the discrimination as a typed primitive. Call this a **higher-resolution IR**: a Tier-1.5 resolver-instance whose alphabet is finer-grained than Tier-1 (the IR-as-spec-prose-mirror) and whose role is to surface spec discriminations that Tier-1's alphabet collapses.

The four §III sub-properties at the higher-resolution tier:

- **P1**: typed primitives that name the previously-collapsed discriminations explicitly. Examples: `SpecType` (Object/Function/Array/NumberData/MapData/WeakMapData…) distinct from runtime typeof; `SpecError` (TypeError/RangeError/SyntaxError taxonomy mapped to RuntimeError variants); `SpecOp` (ToBigInt/ToPrimitive/CreateListFromArrayLike as first-class composition nodes); `[[Get]]` vs `[[ReadInternalSlot]]` as distinct read primitives.
- **P2**: each Tier-1.5 node is observable at its lowering boundary; a Tier-1 IR step that previously open-coded the discrimination is rewritten as a Tier-1.5 composition.
- **P3**: the verifier at Tier-1.5 enforces alphabet fidelity — an IR section that uses a coarse Tier-1 primitive where a Tier-1.5 distinction is required becomes a verifier error, surfacing the collapse at IR-edit time rather than at test262-divergence time.
- **P4**: the lowering from Tier-1.5 to Tier-2 (Rust runtime helpers) preserves the implementation freedom of the runtime tier; multiple lowerings of the same Tier-1.5 node are admissible as long as they preserve the discrimination.

### XIII.c Application: the §XIII targeting heuristic

If §XII's heuristic is *lift the most widely-shared coercion/dispatch paths*, §XIII's heuristic is *promote the most-frequently-collapsed spec discriminations to typed primitives*. The five EXT 73–81 instances each indicate one discrimination worth promoting:

- spec-Object vs runtime typeof (EXT 72b).
- strict-mode of function code (EXT 73).
- spec abstract op identity (EXT 78).
- [[Get]] vs internal-slot read (EXT 79c).
- internal-slot brand (EXT 81).

The cleanest first move is the [[Get]] vs [[ReadInternalSlot]] split. The spec explicitly uses different fonts (typewriter for internal methods, double-bracket for internal slots); the discrimination is already formalized in the spec text. The Tier-1.5 promotion is: a `RefRead` IR node parameterized on `kind: GetMethod | InternalSlot`, where `GetMethod` lowers to a path that invokes inherited accessors and Proxy traps, and `InternalSlot` lowers to a direct field read. The verifier rejects any IR section whose spec step text says "Let len be Get(arrayLike, 'length')" but whose IR uses the `InternalSlot` kind. A substrate audit of the current `generated.rs` names every site that needs the discrimination; EXT 79c was one such site discovered by test failure rather than by audit.

Each subsequent discrimination follows the same shape: name the spec distinction the alphabet currently collapses, introduce the Tier-1.5 primitive that carries it, rewrite the affected Tier-1 IR sections to use the primitive, let the verifier surface remaining collapse-sites. The payoff is the same as §XII's: discovery-cost drops from "find the divergence inside the helper" to "audit which IR sites use the coarse primitive where the fine one is needed."

### XIII.d Where this places the recognition

§XII observed that a pipeline of resolver-instances satisfying P1–P4 acquires diagnostic legibility — the pipeline becomes its own diagnostic for trace-visible bugs. §XIII observes that this property is *alphabet-sensitive*: when an alphabet collapses spec discriminations, a class of bug becomes trace-invisible at the tier whose alphabet is too coarse. The remedy is a higher-resolution resolver-instance whose alphabet preserves the discriminations the previous tier collapsed.

This is not a refinement that retracts §XII — it is the dual. §XII's diagnostic property holds whenever the alphabet is faithful; §XIII names what to do when it is not. Together they describe a self-extending pipeline: trace-visible bugs surface where the resolution path is legible (§XII), and trace-invisible bugs surface as candidates for alphabet promotion at the next higher resolution (§XIII). Each promotion makes one more class of discrimination expressible and thereby restores §XII's diagnostic property over a larger fraction of the spec.

The §X application to rusty-js-ir at IR-EXT 55 instantiated the lowering-compiler pattern at the IR-as-spec-prose-mirror tier. The §XIII recognition opens the next instantiation: a Tier-1.5 spec-IR resolver-instance whose alphabet carries spec discriminations the prose-mirror tier collapses. The lowering chain becomes spec → spec-IR → IR → Rust → bytecode, with each arrow a P1–P4 resolver-instance and the resolution path legible across the full pipeline.

## XIV. Appendix: the dual deviation-pipeline

§XII observed the diagnostic-legibility property of a P1–P4 resolver-instance pipeline. §XIII named the alphabet-sensitivity of that property and proposed a Tier-1.5 spec-IR to preserve discriminations the prose-mirror IR collapses. §XIII's lift is *additive in the upward direction*: each promotion encodes one more spec-mandated distinction the alphabet can express, moving cruftless toward higher-fidelity ECMA.

This appendix names the dual move: *additive in the downward direction*. Engagement evidence (rusty-js-ir IR-EXT 84–89, recorded 2026-05-20) showed that a sequence of strict spec-correctness improvements (+242 test262 wins) produced a slight regression in load-and-shape parity against Bun on a top500 ecosystem-package basket (cruftless 78.8% → 78.3%, 11 packages newly failing where Bun continues to accept them, 0 newly passing). The §XII–XIII pipeline correctly drives spec conformance up; it does not, by construction, track the deviation-tolerance that production engines (Bun, V8, Node, JavaScriptCore) have absorbed over years of ecosystem integration. Real-world JS code depends on those tolerances; a stricter-than-Bun engine catches genuine bugs but is also unloadable for many packages whose authors never noticed they were depending on the deviation.

The structural recognition: a resolver-instance pipeline can encode *what the spec requires* and *what real code requires that the spec doesn't*, and both alphabets can satisfy P1–P4 independently. The two alphabets are duals — one names spec discriminations that the prose under-articulates; the other names ecosystem-deviation patterns that the spec forbids but production engines silently absorb.

### XIV.a The recognition

§XIII gave cruftless a Tier-1.5 spec-IR whose alphabet's role is to preserve discriminations the spec carries but the prose-mirror IR collapses. Each §XIII promotion (Expr::SpecGet, Expr::GetMethod, the apply_proxy_*_invariant family) lifted one spec verb into a typed primitive at the verifier-time discrimination boundary. Test262 conformance rose monotonically.

In the same engagement, package-load parity against Bun fell. The 11 newly-failing packages share a shape: each depends on a behavior the spec forbids but Bun accepts. Two sub-shapes surfaced:

1. **Function-shape-without-callable** (e.g., `module.exports = obj` where obj has `length`/`name` properties typical of functions but isn't callable; a §10.5.8 Proxy.get-trap-returning-an-uninvocable-value path). Spec rejects, EXT 85's GetMethod throws "$1 is not a function." Bun accepts and continues.

2. **Mutable-config-after-freeze** (e.g., a Proxy.get-trap returning a value that differs from the non-configurable non-writable target's stored value; §10.5.8 step 10 TypeError post EXT 88). Spec rejects, EXT 88's apply_proxy_get_invariant throws. Bun accepts.

These are not cruftless bugs. They are *strict spec rejections of code that is in production and whose authors did not notice they were depending on Bun's tolerance*. Catching them is engagement-positive in the §XII / §XIII sense (spec conformance up). It is engagement-negative in the parity-with-Bun sense (load-rate down).

### XIV.b The formalization

A *deviation alphabet* is a resolver-instance whose typed primitives name recurring ecosystem-deviation patterns — patterns that the spec forbids and a strict-spec implementation rejects, but that real-world code in production depends on. Each deviation primitive carries:

1. **Pattern** — the precise spec-step site where the deviation surfaces (e.g., "§10.5.8 step 10: non-configurable non-writable target data property + Proxy.get trap result differs").
2. **Strict rejection** — the TypeError / RangeError / SyntaxError the spec mandates at that site.
3. **Tolerant lowering** — the alternative behavior the deviation absorbs (e.g., silently use the trap result and continue).
4. **Diagnostic** — a structured surface naming the deviation, the file:line of the dependent code, and the spec section the code violates, so library authors can fix upstream without re-introducing the runtime cost of re-discovery.

A deviation-alphabet tier is a P1–P4 resolver-instance: each typed primitive is observable at its boundary (P1), the choice of which deviation applies is stage-deterministic (P2), the verifier checks invariants of the deviation itself — e.g., that a deviation's tolerant lowering remains a sound continuation of execution rather than open-ended undefined behavior (P3), and implementation freedom for the lowering remains intact (P4). The same diagnostic-legibility property §XII names for the spec pipeline transfers: a divergence between a package's expected behavior and its actual behavior under a deviation lowering becomes locatable because the trace explicitly names *which deviation applies at which step*.

The dual structure clarifies a question §XIII left implicit. §XIII observed that an alphabet at the lower resolution-tier collapses spec discriminations the upper tier carries, and the remedy is alphabet promotion. §XIV observes that an alphabet at the upper resolution-tier (the spec text itself) under-models the deviations the lower tier (running ecosystem) depends on, and the remedy is alphabet *extension* — additive primitives that the spec does not name but the ecosystem requires.

### XIV.c Mode-switched vs per-deviation-opt-in

Production JS engines distribute deviation handling differently along this axis:

- **V8 / Node / SpiderMonkey** ship a single global discriminator (strict mode vs sloppy mode, plus implicit-host-bound deviations like `globalThis` extensions). Coarse: one flag drains a large bundle of deviations.
- **JavaScriptCore** distributes per-feature host-bound flags, finer than V8.
- **Bun** appears to absorb most deviations silently and at parser/loader time; not exposed as per-deviation opt-in.

The lowering-compiler pattern §III–V proposes the finest-grained shape: *per-deviation opt-in*. Each deviation is its own typed primitive at the deviation-tier alphabet; the consumer (or a project manifest, or a per-import directive) selects which deviations apply. The engine ships strict-by-default; a consumer that needs a deviation declares it explicitly.

Per-deviation opt-in preserves §XIII's targeting heuristic at the deviation tier: the next deviation to absorb is whichever the consumer base most depends on, and the alphabet grows by promoting those one at a time. Mode-switched, by contrast, collapses every deviation into one undifferentiated bucket and forecloses the discrimination.

The cruftless engagement has tested both modes implicitly. The strict-by-default-with-no-tolerance shape is what EXT 84–89 produced; the result is the +242 test262 wins / 11 parity regressions outcome documented in the trajectory. A per-deviation opt-in shape would let the same engine load the 11 regressed packages (each one declaring the specific deviation it needs) while preserving the strict-spec correctness for the other 999.

### XIV.d Application: the §XIV targeting heuristic

If §XII's heuristic is "lift the most widely-shared coercion/dispatch paths" and §XIII's is "promote the most-frequently-collapsed spec discriminations to typed primitives", then §XIV's heuristic is *promote the most-frequently-tolerated ecosystem deviations to typed primitives at the deviation tier*. The targeting input is the corpus of packages a consumer needs to load; the output is the deviation alphabet sized to that consumer's actual dependency surface.

cruftless's 11 EXT 84–89 regressions are the seed data for the first row of the deviation alphabet. Each row would name one deviation Bun absorbs, alongside the strict-spec site it violates. A consumer that needs to load these 11 packages would opt into 2–3 deviation primitives (the function-shape-without-callable family + the mutable-config-after-freeze family + perhaps a module-loader-conditional-export shim).

### XIV.e Where this places the recognition

§XII names diagnostic legibility as the consequence of a P1–P4 resolver-instance pipeline. §XIII names alphabet-completeness as the precondition for §XII at any given tier, and the Tier-1.5 spec-IR as the upward-additive remedy when a tier's alphabet collapses spec discriminations. §XIV names the dual downward-additive remedy: a deviation-tier alphabet whose role is to preserve ecosystem-deviation patterns that the spec forbids but production code depends on.

Together §XII–§XIV describe a self-extending pipeline along both axes:

- **Upward** (toward the spec): §XIII alphabet promotions encode one more discrimination the spec carries.
- **Downward** (toward the running ecosystem): §XIV deviation primitives encode one more tolerance the production ecosystem requires.

The pipeline's diagnostic-legibility property holds across both axes when both alphabets satisfy P1–P4. The structural symmetry — one tier names what the spec requires, the other names what real code requires the spec doesn't — is the dual the §III–V lowering-compiler pattern admits when run in both directions.

A consequence the cruftless engagement makes concrete: spec conformance and ecosystem-load parity are *not* the same metric and *can move in opposite directions* under a strict-spec-only investment strategy. A high-fidelity ECMA engine is one half of a full production runtime; the other half is the deviation alphabet sized to the consumer base. Neither half is reducible to the other; both are first-class corpus contributions in the §III–V lowering-compiler tradition.

This recognition stands on top of §XII–§XIII rather than retracting them. §XIII opens the upward direction of alphabet growth; §XIV opens the downward direction. Together they map the two-axis self-extension of the lowering-compiler pipeline.

## XV. Appendix: the co-evolution requirement and constraint-comprehension contract

§XIII formalizes the upward-additive lift (spec-discrimination alphabet); §XIV formalizes the downward-additive dual (deviation-tier alphabet). Engagement evidence (rusty-js-ir IR-EXT 90, recorded 2026-05-20) demonstrated the §XIV pipeline empirically: one deviation primitive (`function-not-constructor-relax`) recovered 8 of 11 EXT 84–89 parity regressions while preserving strict-spec correctness on the other ~1015 packages. The recovery was clean. The recovery was also unsafe in a way that the §XIV formalization, as drafted, does not address.

This appendix names the safety property §XIV requires, the §XIII–§XIV co-evolution structure that satisfies it, and the *protected-invariants* field each deviation primitive must carry to make the deviation auditable.

### XV.a The recognition

A deviation primitive D relaxes a strict-spec rule R. R was placed in the spec by TC39 for a reason — to protect some invariant I such that violating R causes I to fail. Enabling D bypasses R, which means I is no longer protected. The deviation is "safe to enable" iff one of two conditions holds:

1. I is not load-bearing for any code that runs under the deviation, OR
2. The deviation's tolerant lowering reconstructs I by some other mechanism.

The cruftless engagement landed `function-not-constructor-relax` and observed 8-of-11 recovery. What it did NOT do: enumerate the invariants that the [[Construct]] enforcement (§10.3.3, EvaluateNew step 7) protects. There is at least one: a non-constructor invoked with `new` may execute `this.X = …` assignments that silently write into a fresh ordinary Object the caller never sees, instead of throwing the TypeError the caller would have caught. Enabling the deviation across an entire package's dependency tree silently absorbs every such write site, producing a class of "the program loaded fine but its state is wrong" bugs that are vastly harder to diagnose than the original "TypeError on load" surface.

The deviation is locally safe (we know what TypeError it skips) and globally unsafe (we don't know what invariants depend on that TypeError still firing inside library internals). This is the cost paid for the +8-recovery yield: every newly-loaded package now carries an unspecified set of invariant violations that the strict rejection was preventing.

§XIV as drafted does not require a deviation primitive to enumerate what it absorbs. §XV closes that gap.

### XV.b The co-evolution requirement

§XIII and §XIV are not independent additions; they are co-evolutionary. Two failure modes follow from treating either as standalone:

**§XIV without §XIII** — each deviation is a load-bearing-or-not coin flip. The deviation pipeline reports "8 packages now load" while silently absorbing an unknown number of invariant violations. The pipeline's diagnostic-legibility property (§XII) is broken at the deviation tier: the trace shows the deviation firing, but not what it suppressed.

**§XIII without §XIV** — the engine becomes more spec-correct than the ecosystem will accept. Real code stops loading. The pipeline's spec-fidelity is high; the consumer surface for that fidelity is empty. Engagement evidence at EXT 84–89: 78.8% → 78.3% parity, 11 packages newly unloadable.

**§XIII + §XIV together, in parallel co-evolution** — each tier names what the other has left implicit. The §XIV deviation catalog forces §XIII to articulate what each strict-spec rule actually protects. The §XIII discrimination alphabet gives §XIV the vocabulary in which to express protected invariants. Each iteration of one tier surfaces new candidates for the other. The constraint surface becomes fully comprehended over time.

This is the structural symmetry §XIV.e hinted at: the upward direction (spec fidelity) and the downward direction (ecosystem tolerance) co-extend the same constraint surface. Neither half alone reaches the production-runtime telos; both halves in parallel converge on a runtime that is both spec-correct AND ecosystem-loadable AND constraint-comprehended.

### XV.c The constraint-comprehension contract

Each deviation primitive D carried at the §XIV alphabet must enumerate, alongside its (pattern, strict_rejection, tolerant_lowering, diagnostic) fields, a fifth field:

5. **protected_invariants** — the set of invariants I_1, I_2, … that the strict_rejection enforces, named at the §XIII discrimination tier. Each I_k is either:
   - A spec-named invariant (e.g., "§10.3.3: a non-constructor's body assumes `this` is bound by the caller, not freshly allocated"), reachable as a typed primitive at the spec-discrimination alphabet, OR
   - An explicit "unknown" marker, recording that the engagement has not yet articulated what R protects. An unknown marker forbids enabling the deviation in any consumer context without explicit waiver.

A deviation is *constraint-comprehended* iff every protected_invariant is either a typed primitive at the §XIII tier OR marked "unknown — auditor-waived" with a referenced engagement record. The pipeline's safety property holds iff the deviations the consumer enables are all constraint-comprehended.

cruftless's first §XIV deviation (`function-not-constructor-relax`) is not yet constraint-comprehended. It carries an implicit protected_invariants list of at least:
- "non-constructor's `this`-write assumptions" (the spec's reason for §10.3.3).
- "callers depending on TypeError-on-new-of-non-constructor as a runtime type-check" (the ergonomic invariant the rejection enforces in idiomatic JS).

Both are currently absent from the EXT 90 primitive. Closing the EXT 90 deviation under §XV requires either (a) lifting both to typed primitives at the §XIII tier, OR (b) waiving them explicitly in the engagement record (trajectory.md) with the consumer-impact analysis the waiver documents.

### XV.d The targeting heuristic

§XII targets coercion/dispatch path lifts (highest discovery-cost reduction per lift).
§XIII targets alphabet promotions for spec discriminations the prose-mirror tier collapses (highest verifier-time-discrimination gain per promotion).
§XIV targets deviation primitives for ecosystem-tolerated patterns (highest parity-recovery per primitive).
§XV targets *protected-invariant articulations* — given a deviation candidate D, the highest-yield next §XIII promotion is whichever invariant in D's protected_invariants list has the most cross-cutting impact when typed.

The four heuristics nest. §XV's heuristic operates on §XIV's deviation candidates and produces §XIII work items. Each iteration of §XV's loop closes one (deviation, invariant) pair as constraint-comprehended; the deviation graduates from "unsafe to enable" to "safe to enable in consumer surfaces that don't rely on the invariant."

### XV.e Where this places the recognition

§XII opens the diagnostic-legibility property of a P1–P4 pipeline. §XIII opens the upward alphabet axis (spec discriminations). §XIV opens the downward alphabet axis (ecosystem deviations). §XV closes the loop by requiring that the two axes co-evolve under a constraint-comprehension contract: each deviation primitive carries the invariants it absorbs; each invariant is either typed at the §XIII tier or explicitly waived.

The pipeline thereby achieves three properties together:

1. **Diagnostic legibility** (§XII): trace shows where execution diverges from intent.
2. **Spec fidelity** (§XIII): every spec discrimination preserved as a typed primitive.
3. **Ecosystem loadability** (§XIV): every recurring tolerance available as an opt-in.
4. **Constraint comprehension** (§XV): every tolerance enumerates what it absorbs.

The combined property is what production-runtime telos requires. A high-fidelity ECMA engine without §XIV is unloadable; with §XIV but without §XV, it loads but silently absorbs invariant violations. With §XIII + §XIV + §XV, it loads, comprehends what it absorbed, and surfaces the absorbed-invariant catalog as auditable signal.

This places §XV not as an extension of §XIV but as the *closure* of the two-axis pipeline — the property that makes the two axes safe to co-evolve. Doc 730 §III–V articulated the lowering-compiler pattern. §XII–§XV articulates the *constraint-comprehended pipeline* that the pattern generates when run in both directions under a co-evolution contract.

## XVI. Appendix: bidirectional engine-diff as the deviation-pipeline's empirical instrument

§XII through §XV described a self-extending pipeline along two axes: upward (spec discriminations promoted to typed primitives at §XIII) and downward (ecosystem tolerances promoted to typed primitives at §XIV with §XV protected-invariant audits). The pipeline's discipline named what to do at each tier. It did not formalize the empirical instrument that distinguishes, at a localized divergence between two engines, which of the following four substrate-move cases applies. Without that instrument, the §XIV deviation alphabet risks absorbing engine bugs disguised as deviations, the §XIII alphabet risks chasing ghosts that are really ecosystem tolerances, and the §XV constraint-comprehension contract has no executable check.

This appendix names the instrument: **bidirectional engine-diff probing**, the apparatus that compares two engines' execution at a localized divergence point and reads which side carries the spec-correct behavior. The instrument is a direct instance of bidirectional Pin-Art (per [Doc 691](/resolve/doc/691-the-polytopal-feature-and-the-pin-art-bidirection)) and of seam-detection operationalized at the engine-pair boundary rather than the intra-engine boundary (per [Doc 705](/resolve/doc/705-pin-art-operationalized-for-intra-architectural-seam-detection)). The rusty-bun engagement-tier application on 2026-05-20 (arktype loading trace, EXT 21 stretch) made the operational shape concrete and supplied the worked example below.

### XVI.a The recognition

In the deviation-pipeline framing of §XIV, a divergence between two engines at a localized program point has four possible structural shapes:

1. **Engine A is spec-correct, engine B violates the spec.** The divergence is a B-bug. Substrate move: §XII coercion / dispatch lift on B, or §XIII alphabet promotion if B's alphabet collapsed a spec discrimination. No deviation primitive applies.

2. **Engine A violates the spec by absorbing a tolerance, engine B is spec-strict.** The divergence is a B-rejection of code that A's ecosystem depends on. Substrate move: introduce a §XIV deviation primitive on B with the §XV protected-invariants list naming what the absorption costs.

3. **Both engines diverge from the spec in different directions.** Each engine has its own deviation alphabet; the divergence at this point is the intersection of their respective absorbed-tolerance and rejected-strictness sets.

4. **Both engines conform to the spec; the divergence is below the spec-mandated discrimination.** Implementation freedom (per the lowering-compiler P4 sub-property) admits this case. No substrate move; both behaviors are admissible.

The §XIV pipeline as drafted presumes case (2) when the surface shape is "B fails, A succeeds." But cases (1), (3), and (4) are equally plausible without an empirical check. The check is the bidirectional engine-diff probe.

### XVI.b The apparatus discipline

The probe operates in five steps.

First, **localize the divergence point**. Run both engines on identical input until they diverge in observable behavior. Bisect until the smallest program point at which they differ is identified, typically a specific function invocation, property access, or method dispatch.

Second, **instrument both engines at the divergence point**. Add identical user-level logging (in JavaScript, in Rust, in whatever surface language sits above the engine) that captures the state visible to user code immediately before the divergence: receiver shape, argument types, intermediate computations.

Third, **read the divergence**. Engines that produce different state at this point are exhibiting different semantics. The diff itself is the diagnostic.

Fourth, **compare each side against the spec**. The spec section governing the operation is the third reference frame. Both engine outputs are evaluated against it.

Fifth, **categorize per cases (1)–(4) above**. The categorization determines whether the next substrate move is §XII (coercion / dispatch lift), §XIII (alphabet promotion), §XIV (deviation primitive with §XV audit), or no-op (case 4).

The bidirectionality is structural per Doc 691: each engine is in turn the **detection-direction probe** (reading what the other engine's execution contains at the divergence) and the **composition-direction reference** (the standard against which the other is compared). Neither engine is privileged. The spec is the privileged reference; the engines provide two independent observations of how that spec is realized.

### XVI.c The session as worked example

The rusty-bun session of 2026-05-20 operated this instrument on arktype's loading failure under cruftless. The trace started with the surface symptom and ended with a localized substrate move.

**Surface**: arktype's parity-probe failed with `Cannot read property 'parseDefinition' of undefined` at `@ark/schema/out/node.js:216:39` inside an `equals()` method. The initial framing treated this as a §XIV candidate, presuming Bun was absorbing a deviation cruftless was rejecting.

**Probe construction**: every `equals()` call in `@ark/schema/out/node.js` was instrumented with identical JavaScript logging under both engines, capturing per-call state (`this.kind`, `r.kind`, `this === r`, `'$' in this`).

**Divergence read**:
- Bun: 2154 equals calls during arktype load. Calls 1 through 45 had specific instance-shape state (`this.kind` was "union", "domain", or "unit"; `'$' in this` was true).
- Cruftless: 46 equals calls before failure. Calls 1 through 45 were byte-identical to Bun. Call #46 exhibited `this === r === Class.prototype`, no `$` in own properties.

**Spec comparison**: Bun's call #46 computed a domain-node instance at the receiver. Cruftless's call #46 computed a class prototype reference. Both engines reached call #46 via identical control flow. The spec section governing the immediate operation (ECMA-262 §13.3.7.3 MakeSuperPropertyReference + §10.1.7.2 OrdinaryGet) mandated the receiver to be the calling method's `this` binding, not the super-base. Bun conformed; cruftless violated.

**Categorization**: case (1). Cruftless violates the spec; Bun is spec-correct. The divergence is a cruftless bug, not a deviation Bun is absorbing.

**Upstream root**: traced via further instrumentation to arktype's `@ark/schema/out/roots/root.js:21`:

```js
get rawIn() {
    return super.rawIn;
}
```

Cruftless's super.X dispatch was invoking the inherited getter with `this = super-base prototype` rather than `this = the original instance`. The BaseNode getter then cached the prototype as `rawIn`'s value on the prototype itself via `cacheGetter`, leaking the proto-as-this state through every subsequent `branch.rawIn` access.

**Substrate**: `Ω.5.P03.E2.super-get-this` (commit `16ff1f56`) introduced `__super_get(this_val, super_base, key)` as a runtime helper that walks `super_base`'s chain and invokes any found getter with `this = this_val`. The bytecode compiler emits this helper call instead of the previous `LoadIdent <super.proto>; GetProp X` sequence. No §XIV deviation; no §XV protected-invariants enumeration; the deviation framing was falsified at the probe step. A pure §XII coercion / dispatch lift at the engine substrate.

The instrument's operational value: it converted what could have been a multi-hour speculative "is this a deviation or a bug?" discussion into a roughly twenty-minute "the engines diverge here; cruftless is wrong; here is the spec section" determination.

### XVI.d The corpus-tier articulation

The instrument is not specific to JavaScript engines. It generalizes to any pair of substrate-implementations of a shared spec: compilers of the same source language, runtimes targeting the same instruction set architecture, parsers of the same protocol, schedulers of the same task model. The probe procedure (localize divergence, instrument both, read the diff, compare each side against the spec, categorize) operates identically in each case.

The corpus already named the mechanism (Doc 691's bidirectional Pin-Art: detection and composition as duals operating on the same surface) and the operational shape at intra-engine seams (Doc 705's seam-detection: Pin-Art probing applied to identify architectural decomposition boundaries). What §XVI adds is the **engine-pair-boundary instance** of the same apparatus, and its specific role within the deviation-resolution pipeline of §XII–§XV. The instrument is what makes the pipeline empirically grounded rather than only specification-grounded.

The seam-detection apparatus (Doc 705) and the engine-diff probe (this section) are the same instrument operating at different scales. Doc 705 names intra-system seams (the boundaries that decompose a single system's constraint catalog into its real architectural forms). §XVI names inter-system seams (the boundaries that decompose the spec-vs-implementations relationship into spec-correctness, ecosystem-tolerance, and implementation-freedom regions). Both operate by reading the joint pattern of probe-positions across many local observations.

### XVI.e Why this is load-bearing for the pipeline

The §XV constraint-comprehension contract requires every §XIV deviation to enumerate `protected_invariants`. The enumeration is auditable only against a reference standard. Without bidirectional engine-diff probing, the reference is the spec text alone, and the spec is intentionally under-articulated about what ecosystem implementations tolerate (per §XIV, that under-articulation is precisely what the deviation alphabet exists to fill). Engine-diff probing supplies a second reference: the executable spec as a chosen reference engine implements it. The `protected_invariants` list then becomes auditable as "what invariant does cruftless protect that Bun does not?" or "what invariant does Bun protect that V8 does not?" at the level of executable behavior on probe inputs.

Without the probe, the §XIV alphabet is a list of patterns with §XV invariants enumerated against the spec text only. With the probe, each invariant entry can be corroborated empirically by running both engines on a probe input that tests the invariant and reading whether the invariant holds in each. The probe is the empirical surface the §XV contract requires.

A related consequence: the probe rules out two failure modes the §XV contract alone cannot detect. The first is the engine-bug-as-deviation mistake (case 1 misread as case 2), in which the engagement introduces a §XIV primitive that silently masks a real spec violation. The second is the ghost-discrimination mistake, in which a §XIII alphabet promotion is undertaken to "fix" a divergence that is in fact implementation freedom (case 4). The probe distinguishes both failure modes from their non-failing siblings.

### XVI.f Successor questions

Three corpus-tier questions extend this articulation.

**(Q1) What is the catalog shape of engine-diff probes across the JavaScript ecosystem?** Bun vs cruftless is one pair. V8 vs Bun, V8 vs JSC, JSC vs SM, SM vs Bun, Bun vs Deno: each pair has its own intersection of absorbed-tolerance and rejected-strictness sets. The pairwise diff lattice is a successor-engagement question. Doc 715's consumer-substrate dependency graph is the structural object the lattice would be a sub-projection of.

**(Q2) Does the instrument apply at the GC tier, the JIT tier, the parser tier?** Each is a P1–P4 resolver-instance per Doc 730 §IV and (for the JIT) [Doc 731](/resolve/doc/731-the-jit-as-a-lowering-compiler-tier-alphabet-purity-upstream-as-the-bound-on-jit-complexity). A GC implementation diff between two engines is structurally peer with a JS-semantics diff. The instrument's vertical recurrence across substrate tiers is a falsifiable claim of this articulation, testable by trying the probe at each tier and observing whether the four-case categorization still applies.

**(Q3) Can engine-diff probes be automated as a standing operational pipeline?** The 2026-05-20 session operated the probe manually: hand-written instrumentation, manual comparison, ad-hoc trace inspection. A pipeline that systematically samples the spec surface by running two engines on test262-class inputs and reading the diff at each divergence would be the standing operational form. The probe at scale is a §XIV alphabet-discovery instrument: it would surface every Bun-deviation cruftless has not yet absorbed (downward axis) and every spec-correctness gap in cruftless that Bun closes (upward axis), continuously, across each test262 sweep. The §XV protected-invariants enumeration would become a generated artifact rather than a hand-curated list.

### XVI.g Where this places the recognition

§XII opened the diagnostic-legibility property of a P1–P4 resolver-instance pipeline. §XIII added the upward alphabet axis (spec discriminations). §XIV added the downward alphabet axis (ecosystem deviations). §XV closed the loop with the constraint-comprehension contract that requires every deviation to enumerate what it absorbs. §XVI names the **empirical instrument** that makes the contract executable: bidirectional engine-diff probing, the apparatus that distinguishes the four substrate-move cases at each localized divergence.

The instrument's mechanism was named at Doc 691 (bidirectional Pin-Art). Its operational form was named at Doc 705 (seam-detection at the intra-architectural scale). What §XVI adds is the role within the deviation-resolution pipeline at the engine-pair scale: the instrument is what converts §XIV from a hand-curated catalog of suspected deviations into an empirically-grounded alphabet whose entries are each corroborated against a reference engine.

The full pipeline now has six co-evolutionary properties:

1. **Diagnostic legibility** (§XII): the trace shows where execution diverges from intent.
2. **Spec fidelity** (§XIII): every spec discrimination preserved as a typed primitive.
3. **Ecosystem loadability** (§XIV): every recurring tolerance available as an opt-in primitive.
4. **Constraint comprehension** (§XV): every tolerance enumerates the invariants it absorbs.
5. **Empirical groundedness** (§XVI): every §XIII and §XIV primitive corroborated against a reference engine's behavior at the relevant divergence point.
6. **Bidirectional self-extension**: the pipeline grows along both axes, validated at each step by the empirical instrument.

Production-runtime telos requires all six. A high-fidelity engine without the §XVI instrument cannot empirically distinguish its own bugs from the ecosystem's deviations; without that distinction, the §XIV alphabet absorbs noise and the §XIII alphabet chases ghosts. With the instrument, each substrate move is empirically grounded at the engine-pair boundary, and the constraint-comprehension contract has an executable second reference against which to audit every absorbed invariant.

The rusty-bun engagement's 2026-05-20 trace is the first worked instance of the instrument operating inside the deviation-resolution pipeline. The corpus contribution is the recognition that the pipeline's §XII–§XV discipline becomes executable only when §XVI's empirical instrument is in place. Further engagements at the JavaScript-engine tier, the GC tier, the JIT tier, the parser tier, or any other P1–P4 substrate boundary should expect to operate this instrument as a first move when a divergence surfaces, before any substrate move is named.

---

## XVII. Appendix: the performance-axis deviation-resolution pipeline

*A primary articulation responding to the keeper's observation (2026-05-21 05:29-local): the §XVI bidirectional engine-diff oracle has been operating implicitly on the performance axis throughout the rusty-bun session, alongside its explicit correctness-axis role. The cumulative 16 WC-EXTs of substrate work — each measured against Bun's BoringSSL-backed reference — is dense empirical anchor for naming the performance-axis pipeline as a structural object dual to the §XVI correctness pipeline.*

### XVII.a The recognition

§XII–§XVI articulate a deviation-resolution pipeline whose units are spec-correctness divergences. The §XVI four-case taxonomy categorizes each localized cruftless-vs-Bun divergence and routes it to one of (1) §XII lift on B, (2) §XIV deviation primitive on B, (3) both-diverge, (4) implementation freedom. The probe is bidirectional engine-diff; the spec is the privileged reference.

The rusty-bun engagement's 2026-05-21 session ran a structurally identical pipeline on the **performance axis**. The probe was the same 5-endpoint TLS coverage matrix; the categorization classified per-substrate-move outcomes by whether the cruftless implementation matched, lagged, or exceeded Bun's wallclock at each primitive site; the substrate moves were §XII-style lifts at the performance dimension (Jacobian coords, Montgomery REDC, baked tables, generic curve scalar mul, Comba schoolbook). The pipeline operated without being named, because the framework as drafted only articulated the correctness axis.

This appendix names the performance-axis pipeline, supplies its four-case taxonomy (extended to five for one hardware-specific case), and supplies the apparatus discipline. The pipeline is **structurally dual to §XVI**: same probe shape, same categorize-then-move discipline, different reference (engineering benchmark rather than specification text), different substrate-move catalog.

### XVII.b The taxonomy (five cases instead of four)

The §XVI four-case correctness taxonomy maps onto the performance axis with one structural addition. For each localized divergence between two implementations' wallclock at the same probed primitive:

**(P1) Algorithmic gap** — implementation B uses an asymptotically faster algorithm than implementation A. cruftless's `BigUInt::mul` is O(n²) schoolbook; BoringSSL above ~16 limbs uses Karatsuba O(n^1.58). The divergence is at the algorithm tier; the substrate move is an algorithm promotion at A. Maps to §XVI case 1 (B has the spec-correct algorithm; A lifts).

**(P2) Constant-factor gap** — both implementations use the same algorithm but B's per-op constant is smaller (different instruction selection, register allocation, inlining, vectorization). cruftless's Montgomery REDC and BoringSSL's are both O(k²); BoringSSL's CIOS-style integrated loop is ~3× smaller constant. The divergence is at the codegen tier; the substrate move is constant-factor work (Comba, CIOS, inline asm). Maps loosely to §XVI case 1 with a finer-grained substrate move.

**(P3) Mixed-axis gap** — both implementations diverge from each other in different directions on different sub-primitives. cruftless beats Bun on alphabet purity at the engagement-internal-TLS substrate (we know exactly what every operation does); Bun beats cruftless on cipher throughput (AES-NI). The divergence is composable; substrate moves trade in both directions. Maps to §XVI case 3.

**(P4) Implementation-freedom gap** — both implementations satisfy the performance envelope acceptable for the workload; no substrate move required. cruftless's ECDSA-P-256 verify at 0.10s after WC-EXT 10 vs Bun's ~1ms: gap is real but acceptable for TLS handshakes whose total budget is ~100ms. Maps to §XVI case 4.

**(P5) Hardware-bound gap** — B uses hardware instructions A's target doesn't have (AES-NI, ARMv8 crypto extensions, SHA hardware accelerators, AVX2). The substrate move is not within the engagement's reach; the choice is either hardware-tier escalation (deploy to hardware with the extension) or carve-out (accept the gap as deployment-tier scope). Has no §XVI analogue; it's a performance-axis specific case.

(P5) is the case the correctness axis cannot encounter: spec correctness is not hardware-dependent at the engagement tier in the way performance is. Adding (P5) extends the framework rather than refining it.

### XVII.c Apparatus discipline

Five steps, parallel to §XVI.b's correctness-probe discipline:

1. **Localize the divergence point**. Bench cruftless vs reference at increasingly granular probe-set, from end-to-end wallclock (TLS handshake) down to per-primitive cost (`mont_mul` micro-bench). Bisect until the smallest unit at which they differ is identified.

2. **Instrument both implementations at the divergence point**. Time the primitive in cruftless under representative input. For reference engines whose internals aren't accessible, infer per-op cost from published benchmarks plus reasonable substrate models.

3. **Read the gap**. Magnitude (how many ×?), workload-frequency (how many times per workload?), and per-EXT-LOC estimate of substrate moves that would close it.

4. **Categorize per (P1)–(P5)**. The categorization determines whether the substrate move is algorithm promotion (P1), constant-factor work (P2), composable trade-off (P3), no-op carve-out (P4), or hardware-tier escalation (P5).

5. **Sequence the substrate moves by (impact × frequency) / LOC**. Rank candidates per the diagnosis, pick the top-of-stack per round.

### XVII.d The rusty-bun session as worked example

The 2026-05-21 session ran the pipeline through 16 WC-EXTs against the Bun-vs-cruftless 5-endpoint TLS probe. Cumulative trace through the five cases:

- **WC-EXT 3** (Jacobian coordinates): (P1) algorithm promotion — affine double-and-add to Jacobian. ~50 LOC for 28× fixture-verify speedup.
- **WC-EXT 5** (build-time-baked comb table): (P1) algorithm promotion — naive scalar mul to comb-table-based; reused the WC-EXT 4 negative-finding to choose Regime 1 over Regime 2.
- **WC-EXT 8** (P-256 Montgomery REDC): (P1) algorithm promotion — binary-divmod-based `mod_mul` to Montgomery REDC. ~150 LOC for 40× per-mul speedup.
- **WC-EXT 9, 10** (route ec_scalar_mul through Mont): (P1) propagation — apply the new algorithm at consumer sites.
- **WC-EXT 12** (generic Mont for arbitrary odd modulus): (P1) at the abstraction tier — generalize the WC-EXT 8 substrate to apply at every consumer.
- **WC-EXT 13** (route EphemeralEcdh to Mont base table): (P1) at the consumer-routing tier.
- **WC-EXT 15** (generic Mont scalar mul for any curve): (P1) at the abstraction tier for ECDSA-P-384.
- **WC-EXT 16** (Comba schoolbook): (P2) constant-factor work — same schoolbook algorithm with better instruction scheduling via single-pass column accumulation.

All 16 WC-EXTs were §XVI-style probe → categorize → substrate-move → re-probe cycles, operating on the performance axis. The session's cumulative effect: ECDSA verify 8.18s → 0.10s (82×), TLS probe wallclock 36s → 2.59s (14×), api.github.com handshake ~10s → ~0.85s (~12×). Bun remains ~17× faster per-endpoint and the gap is decomposed structurally:

| component | cruftless | Bun | gap | case | next move |
|---|---|---|---|---|---|
| ECDSA-P-256 verify | 100ms | ~1ms | 100× | P2 | CIOS Mont (WC-EXT 17) |
| ECDSA-P-384 verify | 260ms | ~3ms | 87× | P2 | CIOS Mont generalized |
| RSA-2048 verify | ~50ms | ~1ms | 50× | P1+P2 | Karatsuba (WC-EXT ?) |
| AES-GCM AEAD | (slow) | (instant) | ~ | P5 | AES-NI / ARM crypto ext |
| Connection establishment | per-request | pooled | ∞ | P3 | connection pooling |
| TLS 1.2 fallback | absent | present | n/a | P4 → carve-out vs lift |

The remaining gap to Bun decomposes cleanly into (P2) substrate moves (~5-10× achievable in pure Rust), (P1) algorithm-tier moves (Karatsuba ~3× on RSA), (P5) hardware moves (AES-NI ~10× on AEAD; out of reach on the Pi without crypto extensions), and (P4) carve-outs or (P3) workload-orthogonal architecture changes. The framework predicts the gap is closeable to within ~5-10× by pure-Rust substrate work; further closure requires hardware-tier escalation.

### XVII.e Why this is load-bearing for the pipeline

§XVI's correctness-axis pipeline operates over a discrete set of spec sections; the deviation-resolution moves either close a divergence or escalate to a carve-out. §XVII's performance-axis pipeline operates over a continuous wallclock measurement; the deviation-resolution moves close fractions of the gap until either an acceptable envelope is reached (P4) or a substrate-tier ceiling is hit (P5).

The two pipelines compose. A substrate move at the correctness tier (e.g., absorbing a spec discrimination via §XIII alphabet promotion) may incur a performance cost; the performance pipeline tracks that cost and surfaces follow-up moves. A substrate move at the performance tier (e.g., switching from naive ec_scalar_mul to Mont-form) may make a correctness subtle (e.g., the Mont-Z initialization bug in WC-EXT 9); the correctness pipeline tracks that subtlety.

The discipline operating *both* pipelines simultaneously is the Pin-Art apparatus per Doc 619. Per Doc 691's bidirectional reading: the detection-direction probe surfaces both correctness and performance divergences from the same wire trace; the composition-direction probe stresses both axes simultaneously. The §XII–§XVI correctness vocabulary and the §XVII performance vocabulary together name the full substrate-move dispatch space.

### XVII.f Successor questions

Three corpus-tier questions extend this articulation.

**(Q1) Per-substrate-tier cost catalog.** Each substrate-tier admits a per-primitive cost catalog with one row per (primitive, implementation) cell. The cryptographic-primitive tier's catalog for cruftless currently has 16 entries (one per WC-EXT). The full catalog per substrate tier becomes a standing artifact the engagement maintains; a workstream-found pair (Doc 733) produces the catalog as a standing artefact at its tier.

**(Q2) Cross-pipeline interaction.** When does a performance substrate move surface a correctness subtlety (or vice versa)? The session's WC-EXT 9 bug (Mont-Z initialization) is one instance. Cataloging the cross-pipeline events is itself a substrate-tier mapping worth producing as a corpus-tier successor work.

**(Q3) Hardware-tier escalation as a first-class substrate move.** (P5) currently routes to carve-out or escalation. The escalation case admits its own substrate-move framework — "deploy to hardware satisfying constraint C." Future engagement work on multi-platform substrate dispatch (hardware-detect at runtime; route to AES-NI on x86_64, ARMv8 AES on ARM64-with-extensions, scalar fallback on Pi-without-extensions) would formalize this. Doc 733's fractal-pair discipline applies: a hardware-detect-and-dispatch workstream merits its own seed-and-trajectory pair.

### XVII.g Where this places the recognition

§XII opened diagnostic legibility for a P1–P4 resolver-instance pipeline (correctness). §XIII–§XV opened the upward and downward alphabet axes (correctness). §XVI named the empirical instrument (bidirectional engine-diff against a reference engine, correctness). §XVII adds the performance-axis pipeline structurally dual to §XVI: same shape, different reference, extended taxonomy (five cases instead of four to handle hardware-bound divergences).

The two pipelines together specify the full deviation-resolution surface the engagement operates over. Per Doc 734 §V.c growth mechanism (positive-finding generalization): the 16 WC-EXTs of empirical anchor density were sufficient to articulate the performance-axis pipeline as a corpus-tier object. Per Doc 733 §V: the engagement now has explicit pair-structured workstream discipline for both axes simultaneously.

---

*Doc 730 § XVII appendix, 2026-05-21. Jared Foy. jaredfoy.com.*

---

## XVIII. Appendix: heuristic recovery via output-set membership

A pattern that surfaced during the cruftless engagement's rusty-js-esm locale on 2026-05-21 extends §XVI's instrument with a sharper claim about when a substrate rule is recoverable.

### XVIII.a The earlier defer

The locale's Rung-3 reading (enquirer, twenty-one spurious keys versus bun on a CJS-to-ESM namespace synthesis) closed as deferred. The verdict was: bun keeps eighteen `enumerable: false` own properties on the exported constructor and drops the other twenty-three. A pure enumerability filter would keep both groups; a pure "keep all own" would keep both. Three hypotheses were considered and rejected. The reading concluded that bun's filter rule was package-shape-dependent in a way the locale's reading could not recover without bun source access.

The defer was wrong. The rule was recoverable. What was missing was not bun source access but a second axis along which to read both engines' output.

### XVIII.b The recovery move

Revisiting the same probe later in the session, the recovery move was to enumerate not just bun's output keys but also the structural feature against which the kept-versus-dropped partition aligns. The discriminator turned out to be membership in `getOwnPropertyNames(Object.getPrototypeOf(exports).prototype)`, the superclass's prototype's own names.

The full rule for CJS namespace mirroring when `module.exports` is a function-typed value inheriting from another class:

```
enum=true own property → INCLUDE
enum=false own property → INCLUDE iff name is in:
    getOwnPropertyNames(Object.getPrototypeOf(exports).prototype)
    OR {"name", "length", "prototype"}
```

The eighteen kept `enum=false` names matched exactly the EventEmitter prototype's own-names set plus the three function intrinsics. The twenty-three dropped names matched neither. The set-membership check was the missing axis. Once it was added, the partition lined up perfectly; the rule fell out as a single conjunction.

### XVIII.c The heuristic class

The earlier defer happened because the reading axis was "what enumerability does each key carry," which produces a partition that the rule cannot distinguish. A rule's discriminator does not have to be a property of the keys themselves. It can be membership in a set computed from a different region of the object graph.

The general claim. When a deviation reading appears to admit no clean rule on the keys' own descriptors, the next axis to consider is structural set-membership against a related object: a superclass's prototype, a marker symbol's value set, an intrinsics table, a registered transformer's signature. The reference engine's output keys, projected against any such structural set, may yield a clean partition where the descriptor-only projection did not.

This extends §XVI's instrument with a recovery protocol. §XVI says: use the reference engine's output as the oracle. §XVIII adds: when the oracle's output does not yield a clean rule under the obvious axis, try set-membership against a structural feature of the input graph before concluding the rule is unrecoverable.

### XVIII.d Verification

The recovered rule was implemented in cruftless's `populate_cjs_namespace_view` (commit `1fe06c2b`, 2026-05-21). The probe flipped: enquirer's namespace dropped from sixty-four keys to forty-three, matching bun exactly. The 119-package parity sweep cleared with no regressions; the locale's parity advanced from 98.3% to 99.1%.

The cost. One additional read of the same probe output, paired with a `getOwnPropertyNames` call on the superclass prototype. The same engineer who declared the rule unrecoverable could have recovered it, in under five minutes, by trying the set-membership projection first.

### XVIII.e The corpus claim

Defers labeled "unrecoverable from outside the reference engine's source" should be treated as conditional rather than terminal. The condition is: the reading has not yet tried set-membership against a structural feature of the input. Until that axis has been tried, the defer is a methodological gap, not a recovery ceiling.

This refines the §XII–§XVI pipeline's failure semantics. A §XIV trace iteration that returns a clean PASS on the bracket probe (the substrate is not the gap) is a genuine close. A §XIV trace iteration that returns "rule unrecoverable" without trying structural set-membership against the reading targets is a deferred re-read, not a close.

The pipeline's discipline now reads: a deviation is closed when either (a) the substrate move lands and the deviation is gone, or (b) the bracket probe returns PASS confirming the substrate is not the gap, or (c) the rule has been read against at least one structural set-membership axis beyond key descriptors and still admits no partition. Anything short of (c) is a deferred re-read at the next session.

### XVIII.f Successor question

The set-membership axes worth trying are not arbitrary. They cluster into a small taxonomy: superclass-prototype, marker-symbol, intrinsics-table, registered-transformer. A corpus-tier successor question is whether the engagement maintains a standing catalog of these axes as a Pin-Art reading checklist, so the next deferred-as-unrecoverable rule routes through the catalog before the defer lands.

---

*Doc 730 § XVIII appendix, 2026-05-21. Jared Foy. jaredfoy.com.*

---

*Doc 730. Jared Foy. jaredfoy.com.*
