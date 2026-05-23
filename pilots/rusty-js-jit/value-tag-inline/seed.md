# LeJIT-Ψ (value-tag-inline) — Resume Vector / Seed

*(Internal name LeJIT-Ψ per the LeJIT seed §I.2 item 4 pre-file. Sibling locale to LeJIT-Σ at `pilots/rusty-js-jit/stub-emitter/`; both children of `pilots/rusty-js-jit/`. On-disk source code lives at `pilots/rusty-js-jit/derived/src/value_tag_inline.rs` within the parent LeJIT crate; the seed + trajectory at this coordinate hold the Pin-Art record.)*

**Locale tag**: `L.rusty-js-jit/value-tag-inline` (nested per Doc 737 §IV)

**Status as of 2026-05-23**: **WORKSTREAM FOUNDED (VTI-EXT 0)**. No code yet. Spawned per LeJIT seed §I.2 item 4 + the keeper's pivot directive after LeJIT-Σ StubE-EXT 5b's bench measurement surfaced that the shape-substrate cascade (Doc 729 §A8.13) had absorbed most of LeJIT-Σ's IC-only contribution (Pred-stub.1 target tightened from ≤90.3 → ≤66.3 ns/iter). The hybrid stance per LeJIT seed §I.2 anticipated this composition; LeJIT-Σ + LeJIT-Ψ + the future dispatcher refactor land multiplicatively, not additively.

**Workstream**: hand-rolled inline tag-check + value-extract emitter for cruftless's NaN-boxed (or tagged) Value encoding. Per LeJIT seed §I.2 item 4: "Value-tag inline emitter for hot Op::GetProp / Op::SetProp / Op::Call paths." Replaces the current per-call extern-helper-based unbox/rebox with inline tag-check + extract via Cranelift IR.

**Author**: 2026-05-23 session.
**Parent**: `pilots/rusty-js-jit/` (LeJIT).
**Sibling**: `pilots/rusty-js-jit/stub-emitter/` (LeJIT-Σ).
**Composes with**:
- [LeJIT seed §I.2 item 4](../seed.md) — names this pilot as one of four hand-rolled regions.
- [LeJIT seed §I.3](../seed.md) — substrate-amortization cascade finding (2026-05-23): LeJIT-Σ alone cannot reach the 3× target post-shape-enrollment; composition with this pilot is the load-bearing path.
- [Doc 731](../../../../corpus-master/corpus/731-the-jit-as-a-lowering-compiler-tier-alphabet-purity-upstream-as-the-bound-on-jit-complexity.md) §VII R1–R8.
- [Doc 735 §X.h](../../../../corpus-master/corpus/735-the-temporal-resolver-instance-stack-build-time-process-time-call-time-as-the-time-axis-dual-to-doc-729s-spatial-stack.md) — three-probe-levels discipline + (P2) four-case categorization.
- [Doc 738 §II](../../../../corpus-master/corpus/738-the-source-identifier-as-coordinate-naming-convention-as-substrate-position-encoding-at-the-source-tier.md) — source-tier convention space.

## I. Telos

Land a hand-rolled inline tag-check + value-extract emitter for cruftless's `Value` encoding such that:

1. **Hot-path arg coercion happens inline** rather than via the current ~30 ns extern helper call. For typed-i64 fast paths (per the β-path of JIT-EXT 5), the JIT preamble can emit `ldr` + `cmp tag, NUMBER_TAG` + `br.ne slow_path` + `extract value bits` directly. Closes the "JIT preamble + arg coercion ~30 ns" cost named in LeJIT-Σ docs/bench-baseline.md §3.

2. **Op::GetProp / Op::SetProp / Op::Call receiver-type discrimination happens inline**. Per Doc 731 §VII, the alphabet's typed-promotion pass (β-path) carries operand-type discrimination at the bytecode tier; the JIT just emits the corresponding inline tag-check at lowering time. No runtime speculation required for cases where the bytecode is already typed.

3. **The inline emission preserves Doc 731 §VII R1** (single tier). Same constraint as LeJIT-Σ: this is a sub-substrate of the same JIT tier, not a second tier.

4. **Composition with LeJIT-Σ + the post-shape-enrollment baseline reaches Pred-stub.1's 3× target multiplicatively.** Per LeJIT seed §I.3's reading: shape (1.36×) + LeJIT-Σ inline IC (~1.3-1.5× on the hot path) + value-tag-inline (~1.2-1.4× on the preamble) ≈ the 3× target from the composite, not from any single pilot alone.

The success criterion is the conjunction of (1)-(4), measured via the same bench harness as LeJIT-Σ (`cruftless/examples/bench_ic.rs`) but extended to also exercise pure-typed-arithmetic hot loops (where the preamble cost dominates and IC isn't in play).

### I.1 Bounded first-cut telos

The first cut covers:
- **aarch64 only** (the engagement's reference hardware).
- **The Number tag inline-check at JIT-call arg-coercion path**. Other tag classes (String, Boolean, Object, BigInt, Symbol, Null/Undefined) stay extern-helper-based.
- **The arg-coercion path only**. Op::GetProp / Op::SetProp / Op::Call receiver-type discrimination at codegen is queued for VTI-EXT 5+.
- **No box/unbox of f64 inline** in the first cut. The β-path's typed-i64 ops are i64-native; box/unbox to/from i64 is what gets inlined. f64 NaN-box discrimination is harder; deferred.

The first-cut closure criterion: JIT-emitted code for a typed-i64 hot function inlines the Number tag-check at every arg-coercion site; the bench_ic baseline drops by the named ~25-30 ns preamble cost; diff-prod 42/42 holds; the existing JIT bench (`bench_sum`) doesn't regress.

## II. Apparatus

The value-tag-inline emitter is **a hand-rolled aarch64 instruction-emission sub-tier at the JIT call-prologue boundary**, alongside Cranelift. It composes with:

- **Cranelift** owns the function body's lowering; this pilot owns the call-prologue inline tag-check.
- **The β-path typed-i64 alphabet** (per JIT-EXT 5) is the upstream contract: the bytecode tier guarantees the operand-type discrimination via Op::AddI64 etc.; the JIT trusts the typing at codegen.
- **The Value encoding** (per `pilots/rusty-js-runtime/derived/src/value.rs`) is the substrate this pilot emits inline checks against. Specifically the discriminant for `Value::Number(f64)` vs other variants.

Per Doc 730 §XII–§XVI, the bidirectional engine-diff oracle gates each substrate move under diff-prod 42/42 + test262-sample 77.8% under enrollment. The bench probe is `bench_ic` (already at hand) + a new arithmetic-hot-loop bench at VTI-EXT 1.

## III. Methodology

Each VTI-EXT is a substrate move per Doc 581 + Doc 729 §A8.13:

1. **VTI-EXT 0 (this round)** — workstream founding. seed.md + trajectory.md + docs/ scaffold. No code.
2. **VTI-EXT 1** — Pre-emission bench probe. Establish baseline measurement: the per-call arg-coercion cost on a tight typed-i64 hot loop. Output: `docs/bench-baseline.md` + (optionally) extension of `bench_ic.rs`.
3. **VTI-EXT 2** — Inline tag-check emitter design. Decide the discriminant layout reading (Value's variant tag is at offset X; Number's f64 payload at offset Y). Choose inline IR shape (cmp + br.ne for the fast path). Output: `docs/inline-design.md`.
4. **VTI-EXT 3** — `value_tag_inline.rs` module scaffold. `emit_inline_number_check(builder, value_arg, fallback_label) -> Value` Cranelift IR helper. Test-only against synthetic NaN-boxed inputs.
5. **VTI-EXT 4** — Wire into translator's `unbox_int64` / `jit_compatible_int_arg` call-prologue paths. Under `CRUFTLESS_LEJIT_VTI=1` env flag.
6. **VTI-EXT 5** — Op::GetProp / Op::SetProp / Op::Call receiver-type discrimination inline (deferred to a separate sub-round if VTI-EXT 4 exposes scope).
7. **VTI-EXT 6** — Re-measure; (P2) categorize per Doc 735 §X.h.b.
8. **VTI-EXT 7** — Fuzz probe.
9. **VTI-EXT 8** — Default-on flip.

VTI-EXTs 4-8 gate on the bench measurement at EXT 1 + VTI-EXT 2's design landing.

## IV. Carve-outs

- **aarch64 only** (x86_64 in a future sibling round if/when the engagement targets non-Pi hosts).
- **Number tag only** in the first cut. String / Boolean / Object / etc. extension queued.
- **Arg-coercion path only** in VTI-EXTs 1-4. Op-receiver discrimination at VTI-EXT 5+.
- **No NaN-box discrimination** for f64 in first cut.
- **No box/unbox between i64 and f64** inline (this is a separate concern from tag-check).

## V. Standing artefacts

- `pilots/rusty-js-jit/value-tag-inline/seed.md` (this file).
- `pilots/rusty-js-jit/value-tag-inline/trajectory.md` — per-VTI-EXT log.
- `pilots/rusty-js-jit/value-tag-inline/docs/bench-baseline.md` — VTI-EXT 1 output.
- `pilots/rusty-js-jit/value-tag-inline/docs/inline-design.md` — VTI-EXT 2 output.
- `pilots/rusty-js-jit/derived/src/value_tag_inline.rs` — VTI-EXT 3 onward.

## VI. Resume protocol

Read [LeJIT seed §I.2 + §I.3](../seed.md), [LeJIT-Σ docs/bench-baseline.md](../stub-emitter/docs/bench-baseline.md), [Doc 731 §VII](../../../../corpus-master/corpus/731-the-jit-as-a-lowering-compiler-tier-alphabet-purity-upstream-as-the-bound-on-jit-complexity.md), then this seed, then trajectory.md. The next substrate move is VTI-EXT 1 (bench baseline measurement); the design (VTI-EXT 2) can begin in parallel since it's reading + documenting.

Pin-Art tag prefix: `Ω.5.P03.E2.vti-*` per `host/tools/tag-grammar.md` (the inline emitter is compile-tier substrate at the engine-internal codegen sub-tier).

## VII. Composition with parent + sibling

Per LeJIT seed §I.3's substrate-amortization-cascade reading, the three pilots compose multiplicatively:

| pilot | per-iter contribution (estimated) | empirical anchor |
|---|---|---|
| shapes/consumer-migration (parent substrate) | 1.36× | StubE-EXT 5b: 271 → 199 ns |
| LeJIT-Σ stub-emitter (sibling) | 1.3-1.5× (estimated) | partial measurement; full at StubE-EXT 6 |
| **LeJIT-Ψ value-tag-inline (this)** | **~1.2-1.4× (estimated)** | bench_ic ext at VTI-EXT 1; measurement at VTI-EXT 6 |
| dispatcher refactor (future) | 1.5-2× (potential) | unmeasured; deferred to its own pilot |

Combined: ~2.5-3× over the pre-shape baseline of 271 ns from the three landed pilots; the 3× target may need the dispatcher refactor too.

This is the substrate-amortization-as-composition discipline operating per Doc 729 §A8.13: each pilot's contribution is a single-axis substrate move; the product is what reaches the corpus's per-op-cost claim against mainstream JITs.

## VIII. Falsifiers

**Pred-vti.1.** Inline Number-tag check on the arg-coercion path reduces per-call cost by ≥20 ns on a typed-i64 hot loop. Falsifier: measured reduction < 20 ns — re-categorize as (P2.d) per Doc 735 §X.h.b.

**Pred-vti.2.** Diff-prod 42/42 + test262-sample ≥77.6% hold under `CRUFTLESS_LEJIT_VTI=1`. Falsifier: regression at either gate; localize per Pattern 1 (bisect-by-jsonl-diff) of DEBUG-METHODOLOGY.md.

**Pred-vti.3.** Doc 731 §VII R1 single-tier preserves: the inline emission is straight-line Cranelift IR; not a second JIT tier.

**Pred-vti.4.** Per LeJIT seed §I.3's composition claim: bench_ic under (shape enrolled + LEJIT_STUB=1 + LEJIT_VTI=1) reaches ≤120 ns/iter (the joint contribution of all three pilots on the IC hot loop). Falsifier: joint measurement > 120 ns — composition isn't multiplicative as predicted.

**Pred-vti.5.** Doc 738 §II convention conformance: identifiers fit the five-axis source-tier space.

## IX. Hypostatic boundary

Per Doc 372, this seed operates at the functional layer. Inline tag-checking is a published codegen-literature pattern (V8 / JSC / SpiderMonkey all use it); the corpus-original contribution is the composition with the cruftless narrow-alphabet stance: a hand-rolled inline tag-check substrate alongside Cranelift preserves Doc 731 §VII R1's single-tier baseline shape, and its measured multiplicative composition with the shape substrate + LeJIT-Σ corroborates the LeJIT seed §I.2 hybrid-stance claim that mainstream JITs achieve perf through multi-tier hierarchies which cruftless achieves through composed sub-substrates.

---

*Doc 581 standing instrument: this seed is the workstream's stable kernel.*
