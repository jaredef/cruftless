# rusty-js-jit — Resume Vector / Seed

**Workstream**: a baseline JIT compiler at the bytecode-to-machine-code substrate boundary, structured per Doc 731 §VII (R1–R8).
**Author**: 2026-05-20 session (EXT 21 close, after the §XVI cluster recovery stretch).
**Parent**: cruftless engagement (`/home/jaredef/rusty-bun`).
**Composes with**:
- [Doc 730](../../../corpus-master/corpus/730-the-vertical-recurrence-of-the-lowering-compiler-closure-as-primitive-across-substrate-tiers.md) §III–§VII (P1–P4 lowering compiler) + §XII–§XVI (deviation-resolution pipeline + bidirectional engine-diff oracle).
- [Doc 731](../../../corpus-master/corpus/731-the-jit-as-a-lowering-compiler-tier-alphabet-purity-upstream-as-the-bound-on-jit-complexity.md) (the JIT as a lowering-compiler tier).
- `pilots/rusty-js-bytecode/derived/` — the upstream alphabet whose purity bounds the JIT's complexity.
- `pilots/rusty-js-ir/derived/` — Tier-1.5 spec-IR whose §XIII alphabet promotions further reduce the JIT's speculation surface.
- `pilots/rusty-js-runtime/derived/` — the consumer of the JIT-emitted code at the engine-internal boundary.

## I. Telos

Build a single-tier baseline JIT for cruftless's bytecode that demonstrates the Doc 731 structural claim empirically: a JIT operating downstream of a P1–P4-faithful alphabet has structural complexity bounded by upstream alphabet impurity. The expected outcome is a JIT whose line count, IC surface, and design legibility are an order of magnitude smaller than V8 TurboFan or JSC FTL, with no multi-tier hierarchy, no internal optimization passes, and no deoptimization sites beyond the small enumerable set of P4 dispatch points the alphabet declares.

The success criterion is *not* benchmark parity with V8. The success criterion is the structural shape match against Doc 731 §VII (R1–R8) and the falsifiability checks of §IX. Benchmark performance is downstream of the corpus claim; the canonical JITs spent years tuning, and tuning is not what this workstream demonstrates.

### I.1 Refined telos (after Doc 731 + EXT 21)

Two empirical refinements after the Doc 731 articulation:

**(a) Telos is bounded execution-time recovery, not raw machine-code throughput.** "JIT works" means: for a representative basket of cruftless's exemplar-43 packages (and a parallel set of test262 timing-relevant cases), the JIT-compiled hot functions execute faster than the bytecode interpreter by a factor large enough to make the compile-amortization positive, on the same sequence of inputs. Specific number is not the seed-tier claim; reach for "noticeably faster than interpreter on a function called >1000 times" as the first-cut quantitative target.

**(b) Telos is also alphabet-completeness corroboration.** The JIT's per-Op translation table (Doc 731's §XI step 2 artifact) becomes the empirical map of which bytecode ops are P1-pure (single Cranelift instruction or small composition) and which are P4 sites (call into runtime helper). The cardinality of P4 sites is the JIT's IC surface. A small cardinality (single digits to low tens) corroborates the strong form of Doc 731's conjecture. A large cardinality weakens it to the residual "smaller than canonical but not LuaJIT-class."

The conjunction of (a) + (b) gives a falsifiable termination condition for the first-cut JIT: termination reached when *(i)* the bytecode-to-Cranelift translation table covers every Op in `rusty-js-bytecode/src/op.rs`, *(ii)* every function called past the compile threshold runs through the JIT-emitted code, *(iii)* a small basket of npm-package loads with hot init functions shows measurable JIT benefit, and *(iv)* the P4-site enumeration produces a single-digit-or-low-tens cardinality.

## II. Apparatus

The JIT is **resolver-instance #N+1 below the bytecode tier** per Doc 730 §IV's vertical-recurrence reading. It composes with:

- **Resolver-instance #0 (rusty-js-ir Tier-1.5)**: each §XIII alphabet promotion in the IR reduces the JIT's speculation surface by one. Currently 1 promotion landed (IsSpecObject); residual TypeOf collapses across the IR sections are queued.
- **Resolver-instance #1 (bytecode compile)**: produces the JIT's input. The Op enum at `pilots/rusty-js-bytecode/derived/src/op.rs` is the JIT's alphabet.
- **Resolver-instance #2 (interpreter)**: the JIT's fallback. Deopt branches back into the interpreter at the bytecode-pc the speculation guard fired at.
- **Resolver-instance #N (Cranelift)**: the JIT's downstream. Cranelift handles instruction selection, register allocation, scheduling, peephole optimization, and machine-code emission. The JIT does not own any of these.

Per Doc 730 §XII–§XVI, the JIT's design and implementation operate under the §XVI bidirectional engine-diff oracle: each substrate move at the JIT tier is gated on the four-case categorization (cruftless-spec-correct vs Bun-spec-correct vs both-diverge vs implementation-freedom) per a probe against a reference engine. Performance-tuning moves use timing comparison rather than semantic comparison, but the methodology is the same shape.

## III. Methodology

The Doc 731 §XI step list is the operational template. Each step is its own substrate move under the standing Pin-Art discipline:

1. **Cranelift integration.** Add `cranelift-codegen`, `cranelift-frontend`, `cranelift-jit`, `cranelift-module` as workspace dependencies. New crate `pilots/rusty-js-jit/derived/` with one entry-point `compile_function(proto: &FunctionProto) -> Option<JitFn>`.

2. **P4 site enumeration.** Walk `Op` in `pilots/rusty-js-bytecode/derived/src/op.rs`. For each Op, classify:
   - **P1-pure**: receiver-type and operand-types are statically determinable from the bytecode alphabet (Op::PushI32, Op::Jump, Op::Add when operands are typed primitives at the IR tier).
   - **P4 site**: dispatch is genuinely free (Op::GetProp, Op::SetProp, Op::Call on a receiver of unknown shape).
   Document the classification as a table in `docs/op-p4-classification.md`. The cardinality of the P4 column is the JIT's IC surface upper bound.

3. **Per-Op translation table.** For each Op, propose the Cranelift IR composition:
   - **P1-pure ops** → single Cranelift instruction (`iadd`, `br`, `iconst`) or small composition.
   - **P4 site ops (first cut)** → Cranelift `call` to a runtime helper function that does the dispatch in interpreter-style. ICs deferred to second cut.

4. **Compilation threshold.** Counter field on `FunctionProto` (or a parallel map keyed on FunctionProto address): increment on each function entry. At threshold N (default 100 to start), the JIT compiles and links. Subsequent calls dispatch to the JIT-compiled version via the function table.

5. **Verifier at the boundary.** Before emitting Cranelift IR, verify the bytecode is well-typed under the alphabet's contract. The verifier is a separate function in `pilots/rusty-js-jit/derived/src/verifier.rs`; verification failure interprets-and-reports per Doc 731 R3.

6. **Deopt mechanism (queued for second cut).** Each P4 site declares its deopt reasons as a typed enum. The deopt path is a finite switch: read deopt reason from JIT frame, reconstruct interpreter frame from the JIT's stack map, resume interpretation at the recorded continuation bytecode pc.

7. **GC interaction.** Safepoints emitted as Cranelift IR pseudo-ops. Cranelift's framework threads stack maps and root info into machine code. Deferred to whenever rusty-js-gc gains a moving-GC tier; for now, the GC is conservative and treats the JIT's stack as opaque.

8. **No internal optimization passes.** The JIT does not run its own constant-folding, dead-code-elimination, or CSE. Cranelift handles those at the (N-1) tier.

## IV. Carve-outs and bounded scope

Per Doc 731 §VI's "what stays hard regardless":

- **No JIT-side ICs in the first cut.** P4 sites translate to runtime-helper calls. The first cut is a Sparkplug-style "compile bytecode to machine code that calls the same helpers the interpreter calls" — performance gain over the interpreter is the removal of the bytecode dispatch loop, not specialization.

- **No deopt in the first cut.** Without ICs, there is no speculation, so there is no deopt. The deopt mechanism is queued for the second cut alongside ICs.

- **No multi-tier hierarchy.** Doc 731 §VII R1 explicitly: one tier. Adding a second tier (Maglev-style or TurboFan-style) is out of scope; the corpus claim is that one tier is sufficient against a P1–P4-faithful alphabet.

- **No async / generator JIT support in the first cut.** Generator and async functions go through their state-machine bytecode in the interpreter; the JIT skips them.

- **No JIT for module top-level code.** Only functions called past the threshold get compiled. Module init runs through the interpreter regardless.

These carve-outs are spec-aligned: they correspond to areas where the JIT's complexity-vs-yield ratio is unfavorable for a first-cut baseline.

## V. Standing artefacts

Operational artefacts the workstream produces:

- `pilots/rusty-js-jit/derived/` — Cargo crate with `compile_function`, `JitFn`, the per-Op translation table, the verifier.
- `pilots/rusty-js-jit/derived/src/op_classification.rs` — the P1-pure / P4 classification table per Doc 731 §V S2.
- `pilots/rusty-js-jit/derived/src/translation.rs` — the bytecode-Op-to-Cranelift-IR table per Doc 731 §XI step 3.
- `pilots/rusty-js-jit/derived/src/verifier.rs` — the bytecode-well-typedness check per Doc 731 R3.
- `pilots/rusty-js-jit/docs/op-p4-classification.md` — human-readable version of the classification table.
- `trajectory.md` — time-ordered record of substrate moves and their yields.

## VI. Resume protocol

Read Doc 730 §III–§VII + §XII–§XVI, Doc 731 in full, this seed, then trajectory.md. The Doc 731 §VII (R1–R8) structural shape is the design target; the four-case §XVI categorization is the gate before any substrate move; the cadence target is the same ~10-minute diagnosis-to-landed substrate the EXT 21 work observed.

First substrate move (when implementation work begins): produce the P4 site enumeration table for the current Op enum. Cardinality is the JIT's IC surface bound. The enumeration is reading + classifying; no Cranelift integration required for this move. Output is `pilots/rusty-js-jit/derived/src/op_classification.rs` or, ahead of the crate, `pilots/rusty-js-jit/docs/op-p4-classification.md`.

Cruftless engine state at this workstream's start (EXT 21 close, 2026-05-20):
- Bytecode interpreter: complete, well-tested, ~80% Bun-load-parity.
- Bytecode alphabet: ~50 Ops, mostly P1-pure shape, several P4 sites (GetProp, SetProp, Call, CallMethod, New).
- IR alphabet: §XIII promotion underway (1 landed: IsSpecObject). Residual TypeOf collapses queued.
- Frame.strict tracking: just landed (EXT 21).
- 12 substrate moves landed in EXT 21 closing major spec-correctness gaps.

Pin-Art tag prefix for this workstream: `Ω.5.P03.??.jit-*` for compiler-side work, `Ω.5.P04.??.jit-*` for runtime-side work. Per host/tools/tag-grammar.md, the handle is the substrate node the move touches.
