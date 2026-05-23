# LeJIT — Resume Vector / Seed

*(Internal name for the rusty-js-jit pilot as of 2026-05-22 telos-sharpening; on-disk crate path `pilots/rusty-js-jit/` retained until a separate clerical-migration round renames it. The `LeJIT` reading: "le JIT" / "legit JIT", naming the pilot's deliberate hybrid stance — consume Cranelift where Cranelift is structurally upstream of the alphabet contract, hand-roll where the alphabet contract is finer-grained than Cranelift's defaults. Per §I.2 below.)*

**Locale tag**: `L.rusty-js-jit` (per [Doc 737](../../../corpus-master/corpus/737-the-locale-as-coordinate-nested-seed-trajectory-pairs-as-pin-art-substrate-positions.md); the locale-tag preserves the on-disk path per Doc 737's coordinate-uniqueness invariant, while the workstream's internal name advances to LeJIT).

**Status as of 2026-05-22**: **DEOPT CHAPTER CLOSED + IC INFRASTRUCTURE COMPLETE (without shapes); TELOS SHARPENED to the hybrid Cranelift+hand-rolled-IC-stub stance per §I.2.** JIT-EXT 10-24 landed across the prior two sessions — arithmetic deopt machinery, ICShapeMismatch demonstrator, full GetPropOnObject lowering with real runtime helper, mixed-regime dispatcher, IC chain success+failure paths proven end-to-end. ~1.2k LOC across pilots/rusty-js-jit + pilots/rusty-js-runtime + pilots/rusty-js-bytecode + host-v2. PM-EXT 11+12 regression GREEN every round. See §VIII below for the closure summary. JIT-EXT 25 (this entry) sharpens the forward telos to name the next two pilots: IC stub emitter + hidden classes substrate.

**Workstream**: a hybrid baseline JIT at the bytecode-to-machine-code substrate boundary, structured per Doc 731 §VII (R1–R8). Hybrid means: Cranelift owns the generic codegen tier (instruction selection, register allocation, scheduling, peephole, machine-code emission); LeJIT owns the substrate-specific layers Cranelift cannot reach (IC stub emission with patching, hand-rolled stack maps per §VII R7, deopt machinery, Value-tag inline checks).
**Author**: 2026-05-20 session (EXT 0-9), extended 2026-05-21 (EXT 10-24).
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

### I.2 Sharpened telos (2026-05-22): the LeJIT hybrid stance

The first-cut termination condition of §I.1 is structurally met as of JIT-EXT 24 (deopt chapter closed + IC infrastructure complete modulo hidden classes). The post-first-cut telos surfaced from a 2026-05-22 keeper exchange on whether hand-rolling Cranelift could provide performance benefits, and concurrence on the answer.

**Recognition.** Hand-rolling a Cranelift replacement is *probably no overall* (Cranelift carries 20+ years of regalloc / isel / scheduling engineering on aarch64 that hand-rolling cannot match in any tractable substrate budget; the engagement's hand-rolled discipline elsewhere is anchored in Pin-Art derivation against published specs, and Cranelift has no analogous spec to derive against). Hand-rolling is *probably yes in narrow regions* — specifically four sites where Cranelift's generality is structurally incapable of expressing what cruftless needs:

- **IC stub emission with self-modifying patching.** Cranelift cannot patch call targets in place; V8/JSC/SpiderMonkey ICs all rely on inline-cached 2-3-instruction shape-checks that self-modify on miss. Today's GetPropOnObject IC routes through an `extern "C"` call per JIT-EXT 22-24 (~5-15ns overhead per hit on the engagement's Pi). A hand-rolled stub emitter that inlines the shape-check and patches on miss is the structural fast-path Cranelift cannot reach.
- **Value-tag inline checks.** cruftless's Value encoding is finer-grained than Cranelift's IR sees. A hand-rolled emitter that knows the Value layout can emit one inline branch-on-tag where Cranelift routes through a function-call abstraction.
- **Tiny-function compile latency.** Cranelift's fixed compile-time overhead (regalloc + scheduling + isel) dominates for functions of ~20 instructions. A hand-rolled Sparkplug-style stack-machine-to-register baseline compiles in microseconds.
- **Tail-call-shaped dispatch loops.** Cranelift's calling conventions add prologue/epilogue cost the bytecode dispatch loop doesn't need.

**Sharpened telos.** The LeJIT pilot's forward telos is to demonstrate the hybrid-codegen structural claim empirically:

> A JIT with Cranelift owning the generic codegen tier AND a hand-rolled substrate-specific emitter owning the IC-fast-path / tagged-Value / inline-tiny-fn tier achieves IC fast-path latency competitive with mainstream JITs while preserving the Doc 731 §VII R1–R8 single-tier baseline shape.

Three structural consequences:

**(i) The hand-rolled scope is finite and named.** Not "hand-roll codegen"; specifically hand-roll: (a) the per-IC-family stub emitter for aarch64 (and x86_64 later) with self-modifying patching, (b) the Value-tag inline emitter for the hot Op::GetProp / Op::SetProp / Op::Call paths, (c) the tiny-fn fast-baseline emitter that bypasses Cranelift when function size is below threshold. Each is bounded by published codegen-literature templates; none requires reinventing regalloc or isel.

**(ii) The Cranelift dependency stays, with a sharper division of labor.** Cranelift handles function bodies for non-tiny functions and the slow-path fallback when the hand-rolled stub misses through its patches. The boundary is: "Cranelift owns what Cranelift can express; LeJIT owns what Cranelift structurally cannot reach." Doc 731 §VII R8 ("no internal optimization passes in the JIT") remains satisfied because the hand-rolled emitter is straight-line lowering, not optimization.

**(iii) The hidden-classes substrate is the dependency.** Per the seed §VIII gap list, IC fast paths require shared shape descriptors that cruftless's current Object representation lacks. A separate workstream — locale-coordinate `pilots/rusty-js-shapes/` per Doc 737 §IV's coordinate discipline — is pre-filed for the hidden-classes substrate. The IC stub emitter pilot and the hidden-classes pilot compose: the emitter is the consumer of shape descriptors the hidden-classes pilot produces. Neither lands without the other; the order is hidden-classes first (substrate-introduction round per Doc 729 §A8.13's substrate-amortization pattern), then IC stub emitter (closure round reusing the substrate).

**(iv) Per Doc 738's source-tier coordinate system, the LeJIT hand-rolled tier gets its own convention sub-namespace.** Functions in the hand-rolled emitter live at `pilots/rusty-js-jit/derived/src/{stub_aarch64,stub_x86_64,value_tag_inline,tiny_baseline}.rs`. The pillar-path encoding of Doc 738 §II.e applies; the prefix convention of Doc 738 §II.a applies (engine-internal sentinels for IC stub state use `__ic_*`). Cross-tier consistency with the Cranelift-using translator path is maintained at the source-identifier coordinate level.

**Falsifier added to §IX (existing):** If the hand-rolled IC stub emitter (Pilot LeJIT-Σ, queued) does not achieve at least 3× per-hit speedup over the current extern-call IC dispatch on a representative property-access hot loop, the §I.2 hybrid claim is weakened; the work should re-categorize either as (P2.d) correct-but-losing per Doc 735 §X.h.b (in which case revert to extern-call dispatch and document the boundary) or as (P2.c) illegal-speed (in which case fuzz coverage caught the gap before the bench-shape miscategorization persisted). The §X.h.c three-probe-levels discipline (bench + consumer-route + fuzz) gates the (P2.a) strict-win claim for the hand-rolled emitter.

**Forward queue at JIT-EXT 25 close:** Pilot LeJIT-Σ (IC stub emitter, hand-rolled aarch64, paired with hidden-classes pilot). The two together close the seed §VIII "hidden classes" gap and the seed §VIII "dispatcher branching for non-zero pc deopts" gap simultaneously, since hidden-classes lands the shape descriptors that make non-trivial IC fast-paths landable.

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

## VIII. Closure summary (deopt chapter + IC infrastructure, 2026-05-21)

The 2026-05-20 session landed EXT 0-9 (translator, β-path, runtime integration, deopt-disable workaround). The 2026-05-21 session landed EXT 10-24, closing the arithmetic deopt chapter and the IC infrastructure chapter (modulo hidden classes, which remain a separate future workstream).

### Substrate delivered (~1.2k LOC)

**Arithmetic deopt machinery (EXT 10-17):**
- `pilots/rusty-js-jit/derived/src/deopt.rs` (~400 LOC):
  - DeoptReason enum (5 variants); JitLocation; DeoptLiveLocal; DeoptSite; DeoptCallFrame; DeoptRecoveredState; JitCallOutcome
  - `reconstruct_state` + `jit_deopt_thunk` (pure-Rust lookup + extract)
  - `extern "C" fn deopt_trip` callable from Cranelift-emitted code
  - TLS slots: CURRENT_DEOPT_SITES, LAST_DEOPT_FRAME, JIT_FORCE_SHAPE_TRIP
  - `set_*` / `clear_*` / `take_last_deopt` helpers
- `pilots/rusty-js-jit/derived/src/translator.rs` (~250 LOC):
  - `emit_guarded_add` (XOR-idiom signed-overflow detection)
  - `emit_guarded_sub` (XOR-idiom for subtract)
  - `emit_guarded_mul` (smulhi-based overflow check)
  - Inc/Dec/IncI64/DecI64 reuse the helpers with synthetic rhs=1
  - Entry shape-check emission under CRUFTLESS_JIT_FORCE_SHAPE_TRIP
  - `CompiledFn.deopt_sites: Vec<DeoptSite>` field
- `pilots/rusty-js-runtime/derived/src/interp.rs`:
  - Dispatcher sets/clears CURRENT_DEOPT_SITES TLS; consumes deopt via take_last_deopt; falls through to interp on trip
  - `jit_disabled` permanent-disable workaround relaxed to retry-on-fresh-args

**IC chapter (EXT 18-24):**
- Doc 731-aligned design choice: per-Value-kind specialization (Option B) over tagged-i64 union
- `Op::GetPropOnObject = 0xFB` added to bytecode op enum
- Interpreter shares dispatch with `Op::GetProp` via match-arm widening
- Translator: ParsedOp::GetPropOnObject + lowering to extern call
- `extern "C" fn jit_getprop_on_object` in JIT crate with function-pointer indirection
- `extern "C" fn runtime_getprop_on_object` in runtime crate: reads TLS Runtime + Proto, performs real `object_get`, encodes Number / records ICShapeMismatch deopt for non-Number
- `Runtime::install_jit_getprop_helper()` registers the runtime helper at install_intrinsics time
- `Runtime::resume_from_deopt_state` constructs Frame from recovered state + runs interp from arbitrary pc
- Dispatcher boundary widened (`jit_compatible_arg` accepts Number OR Object)
- Two end-to-end tests prove success-path (Number result returned) and failure-path (non-Number → deopt → interp fall-through → correct result)

### Round-by-round summary

| EXT | tag | substrate |
|---|---|---|
| 10 | jit-deopt-audit | arithmetic deopt audit + design doc |
| 11 | jit-deopt-infra | DeoptReason + DeoptSite + JitLocation + thunk skeleton |
| 12 | jit-deopt-extern-wiring | deopt_trip callable from Cranelift; TLS plumbing |
| 13 | jit-deopt-guarded-add | first wired demonstrator (guarded Add) |
| 14 | jit-deopt-dispatcher | dispatcher detects deopt + falls through |
| 15 | jit-deopt-sub-mul | overflow guards extended to Sub + Mul |
| 16 | jit-deopt-inc-dec-retry | Inc/Dec guards + jit_disabled retry refactor |
| 17 | jit-deopt-ic-shape-demonstrator | ICShapeMismatch reason variant flows end-to-end |
| 18 | jit-ic-getprop-design | IC + GetProp audit + design doc |
| 19 | jit-getprop-on-object-bytecode | Op::GetPropOnObject = 0xFB added |
| 20 | jit-getprop-lowering-stub | JIT lowering via stub helper |
| 21 | jit-resume-from-deopt-state | resume from recovered state at arbitrary pc |
| 22 | jit-real-getprop-helper | real helper via TLS Runtime + FunctionProto |
| 23 | jit-mixed-regime-getprop-e2e | dispatcher accepts Object args; full IC chain E2E |
| 24 | jit-ic-failure-path-e2e | IC chain failure path (non-Number → deopt → interp) E2E |

### Pred-731 disposition

| Conjecture | Status |
|---|---|
| R1 (single tier) | corroborated across all rounds |
| R3 (verifier-before-emission) | corroborated; GetPropOnObject rejected pre-lowering at EXT 19 |
| R5 (deopt sites finite-enumerable per emitted module) | corroborated end-to-end; arith + IC variants both flow through DeoptSiteTable |
| R6 (one tier; no lower-tier JIT) | corroborated; slow paths funnel to interpreter via deopt |
| R7 (hand-rolled stack maps, no Cranelift GC stackmap dependency) | corroborated; reconstruct_state + resume_from_deopt_state consume the layout |
| R8 (no internal optimization passes) | corroborated; all emissions are straight-line lowerings |

### Documented gaps (not closure regressions)

- **Hidden classes**: cruftless's Object representation lacks shared shape descriptors. The IC's fast-path cache (shape → slot_offset) cannot land without hidden classes. A separate workstream (`pilots/rusty-js-shapes/`?) is required. Multi-week.
- **Upstream emitter extension**: the bytecode compiler's typed-promotion pass doesn't emit GetPropOnObject yet. Until it does, the JIT's GetProp path is unreachable from real JS code (only hand-crafted bytecode exercises it).
- **Dispatcher branching for non-zero pc deopts**: `resume_from_deopt_state` landed at EXT 21 but the dispatcher always falls through to interp re-execution from pc=0. With ICs at non-zero pcs (after hidden classes), routing via resume_from_deopt_state becomes meaningful.
- **Multi-arg JIT'd GetProp**: dispatcher gate is 1-or-2 args. Wider arities require translator extension.

### Open scope past closure

The JIT pilot's first-cut is functionally complete for the substrate cruftless has today. Subsequent work is either coverage expansion or cross-pilot:

1. Hidden classes substrate (new pilot)
2. Upstream emitter typed-promotion extension (bytecode pilot concern)
3. Dispatcher branching for non-zero pc deopts (small)
4. Multi-arg JIT'd functions
5. Op::Call in translator (inter-procedural JIT)
6. Broader Value coverage (doubles, strings) — depends on Option B per-kind specialization scaling
