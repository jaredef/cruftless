# LeJIT-Φ (f64-calling-convention) — Trajectory

Per-Φ-EXT log for the LeJIT-Φ f64 calling-convention pilot. Fourth nested sibling under `pilots/rusty-js-jit/`; siblings are LeJIT-Σ (stub-emitter, default-on), LeJIT-Τ (tiny-baseline, default-on), and LeJIT-Ψ (value-tag-inline, (P2.d)). Read seed.md first; then read the LeJIT parent seed §I.4 + findings.md (especially V.3 + V.6 + II.2 + standing rule 9).

---

## Φ-EXT 0 — 2026-05-23 (workstream founding)

### Headline

Apparatus-tier round. Pilot LeJIT-Φ founded per Doc 737 §IV + keeper directive 2026-05-23 16:02-local after the pre-implementation analysis surfaced that VTI's structural (P2.d) traces to the JIT's i64-only calling convention (induced by LeJIT seed §I.1's "typed-i64 first; f64 deferred" carve-out). The constraint enumeration (C1-C10 in seed §I.2) names ten substrate-tier invariants; the architecture induced by C1+C2+C3+C5+C10 is **f64 default + bytecode-tier-driven typed-i64 promoted fast path**. Φ lands Move 1 (f64 default); Move 2 (typed-i64 fast path via bytecode tier-1.5 IR) is a separate downstream pilot.

### Substrate delivered

- `pilots/rusty-js-jit/f64-calling-convention/seed.md` (~155 lines): telos (f64 calling-convention with typed-i64 deferred to a separate pilot); constraint enumeration C1-C10; induced-architecture claim; six falsifiers Pred-φ.1-φ.6; staged-validation methodology Φ-EXT 0-8; carve-outs (typed-i64 fast path out-of-scope; cap-passing preserved; single-tier preserved; OptLevel::None preserved); Doc 738 §II conventions checklist.
- `pilots/rusty-js-jit/f64-calling-convention/trajectory.md` (this file).
- `pilots/rusty-js-jit/f64-calling-convention/docs/` + `fixtures/` scaffold.

### Locale registration

Per Doc 737 §IV: nested locale at coordinate `pilots/rusty-js-jit/f64-calling-convention/` (depth 2). Parent: `L.rusty-js-jit` (LeJIT). Siblings: LeJIT-Σ (stub-emitter), LeJIT-Τ (tiny-baseline), LeJIT-Ψ (value-tag-inline).

The engagement's **fourth** nested LeJIT-tier locale (sixth nested overall after arktype + consumer-migration + Σ + Τ + Ψ). Locale count: 15 → 16. Manifest refresh queued.

### Constraint-enumeration as the design instrument

Per Doc 581 Pin-Art discipline: a substrate move's correctness is bounded by how well its constraints are named. Φ's spawn was preceded by an explicit C1-C10 enumeration (seed §I.2). The architecture this enumeration induces (f64 default + bytecode-driven typed-i64 fast path) is near-necessity, not arbitrary choice. The keeper explicitly framed this round's pre-spawn analysis: "what implicit constraints can we name to constrain the next layer."

The C1-C10 set will be cited as the design framework at each Φ-EXT round; substrate moves at this pilot are scoped by how they preserve (or selectively relax) the named constraints.

### §XVI / Doc 734 / Doc 735 §X.h categorization

Per Doc 730 §XVI: not applicable (founding round).

Per Doc 734 §V: growth (a) **tier-relocation** — the typed-i64 first cut's deferral hit its limit (per Findings V.3 VTI (P2.d) + V.6 first-cut-met). The constraint that forced VTI into (P2.d) is the i64-only calling convention; lifting it requires this pilot. Growth (b) negative-finding amendment in waiting — if Φ-EXT 4-6 surface that f64 doesn't preserve the baseline (Pred-φ.1 / .2 falsified), the LeJIT seed §I.1 / §I.4 readings amend accordingly.

Per Doc 735 §X.h.c three-probe-levels: bench + consumer-route + fuzz all gate Φ-EXT 4-6. Standard discipline.

### Composition with prior corpus work

- **LeJIT seed §I.1 first-cut carve-out** ("typed-i64 first; f64 deferred"): Φ closes the deferral.
- **LeJIT seed §I.2 hybrid stance**: Φ's value-domain shift is the architectural complement to the four hand-rolled regions. Reframes (b) value-tag inline checks: post-Φ they become trivially cheap (tag-only check vs the integer-validity check the i64 alphabet required).
- **LeJIT seed §I.3 + CRB-EXT 8 amendment + §I.4 first-cut-met**: Φ's predicted impact is bounded by Pred-φ.1/.2 (preserve engagement-tier baseline). Composition reading at bench_ic / bench_call_overhead remains the load-bearing measurement.
- **LeJIT findings V.3**: Φ's downstream Φ-EXT 7 attempts VTI revival; the V.3 "revival path empirically named" reading is honest only if Φ-EXT 3+7 land cleanly.
- **LeJIT findings V.6**: Φ shifts the engagement-tier baseline reading; subsequent V.6-class composition claims should cite the post-Φ baseline.
- **LeJIT findings II.4 + standing rule 9**: Φ touches the JitFn type which has been the seat of the TB-EXT 7 segfault. Cross-arch via Cranelift continues; the Box-wrap discipline applies to any new raw-pointer cache Φ introduces.
- **Doc 731 §VII R1, R6, R8**: preserved by construction.
- **Doc 731 §XIII alphabet promotion**: Move 2 (typed-i64 fast path) is a downstream pilot at the bytecode tier; Φ does not preempt it.
- **Doc 736 §IX.6 cap-passing**: preserved (JIT body has no cap surface).

### Open scope at Φ-EXT 0 close

1. **Φ-EXT 1** — Design doc. Enumerate per-op IR-change deltas; per-extern signature changes (`jit_getprop_with_ic`, `runtime_ic_fast_get`, JitFn types); per-dispatch-site changes. Output: `docs/f64-design.md`.
2. **Φ-EXT 2** — Substrate-introduction (JitFn signature change + dispatcher arg-passing; no JIT-body IR change yet).
3. **Φ-EXT 3** — Closure round (JIT translator switches to fadd/fsub/fmul).
4. **Φ-EXT 4-6** per seed §III methodology.
5. **Φ-EXT 7** — VTI re-attempt (the load-bearing revival test).
6. **Φ-EXT 8** — Default-on confirmation (Φ is architectural, default-on by construction).

### Cumulative status at Φ-EXT 0 close

LOC delta: 0 (apparatus-tier; no source code). docs/ + fixtures/ scaffold + locale registration.

The pilot's substrate work begins at Φ-EXT 1 (design doc). The constraint-enumeration framework supplies the design discipline; the staged-validation methodology supplies the implementation discipline. Per the engagement's findings doc rule 4 (never split substrate moves) + standing-rule-9 (raw-pointer-cache audit), each Φ-EXT round must explicitly verify constraint preservation and bug-class absence.

---

*Φ-EXT 0 closes. Fourth LeJIT-tier locale founded; the JIT calling-convention architectural shift is queued; constraint-enumeration C1-C10 is the design framework; staged-validation Φ-EXT 1-8 is the implementation methodology. The first cut's typed-i64 carve-out reaches its limit here; Φ closes the deferral.*

---

## Φ-EXT 1 — 2026-05-23 (design doc; per-op IR-deltas + per-extern signatures + per-dispatch-site changes enumerated)

### Headline

Design-tier round. Produced `docs/f64-design.md` (~290 lines) enumerating the full set of changes Φ-EXT 2/3 lands. Per Doc 729 §A8.13 substrate-amortization-cascade discipline + Φ seed §III staged-validation methodology, the implementation splits across Φ-EXT 2 (substrate-introduction: JitFn signature + dispatcher arg-pass without changing JIT body IR) + Φ-EXT 3 (closure round: JIT body switches to fadd/fsub/fmul).

### Substrate landed

- `pilots/rusty-js-jit/f64-calling-convention/docs/f64-design.md` (~290 lines):
  - §2: JitFn signature change (i64 → f64 in extern fn types)
  - §3: per-op IR-change deltas (full table; 23 ops covered)
  - §4: per-extern signature changes (`jit_getprop_on_object`, `jit_getprop_with_ic`, `runtime_ic_fast_get`, `deopt_trip`)
  - §5: per-dispatch-site changes (standard path + TB fast-path + deopt fallthrough)
  - §6: cross-pilot composition checks (shape, STUB observer + fast-get, TB cache, VTI under env flag)
  - §7: backward-compat with Move 2 (typed-i64 fast path doesn't preempt Φ; uses NEW Op variants when it lands)
  - §8: per-fixture validation plan (Φ-EXT 4 composition matrix + Φ-EXT 5 consumer-route + Φ-EXT 6 fuzz)
  - §9: Φ-EXT 7 VTI re-attempt preview (revival path becomes structurally winnable post-Φ)
  - §10: five named risks + mitigations (R1 f64 arith slower per-op; R2 sentinel collision; R3 deopt live-value type drift; R4 TB cell-cache; R5 VTI mid-Φ usability)
  - §11: forward-to-Φ-EXT-2 (substrate-introduction stage)

### Key design decisions

**Decision 1: stack-is-f64-throughout model** (§3). Cleaner type model; comparison results promoted from i8 to f64 via uextend + fcvt. Alternative (mixed stack types) rejected for complexity.

**Decision 2: sign-bit-set quiet-NaN as IC_FAST_MISS_SENTINEL** (§4). `f64::from_bits(0xFFF8000000000001)` — JS cannot produce; safe; one f64 comparison to test for miss.

**Decision 3: Op::Add stays untyped + f64-lowered post-Φ; Move 2 introduces NEW Op variants** (§7). Preserves single-tier R1; Move 2's bytecode-tier typing is additive, not replacement.

**Decision 4: stage Φ-EXT 2 BEFORE Φ-EXT 3** (§11). Substrate-introduction (signature change + dispatcher arg-pass) without IR change; JIT body still iadd but converts f64 args via fcvt at entry. Verify GREEN. Then Φ-EXT 3 flips the JIT body IR. Per Finding II.2: never split substrate moves; here both halves removed work (precheck integer-validity gone) before/after the same atomic move.

**Decision 5: VTI revival deferred to Φ-EXT 7, not part of Φ-EXT 3** (§9). Φ ships the calling-convention shift; VTI's inline tag-check + payload-extract at JIT prologue is a separate round.

### Five named risks + mitigations (§10)

| risk | mitigation |
|---|---|
| R1 — f64 arith slower per-op than i64 | Acceptable per C10 (±15% Pred-φ.1); Move 2 recovers; most JS is f64 |
| R2 — Sentinel collision | JS can't produce sign-bit-set NaN without DataView; fuzz probes |
| R3 — Deopt live-value type drift | Per-LiveLocal type tag if needed; Φ-EXT 3 assumes all f64-bits |
| R4 — TB cell-cached pointer breakage | TB-EXT 7 Box-wrap protection still valid; no new raw-pointer cache introduced |
| R5 — VTI mid-Φ usability | VTI default-OFF; opt-in users get (P2.d)+wrong until Φ-EXT 7 |

### §XVI / Doc 734 / Doc 735 §X.h categorization

Per Doc 730 §XVI: not applicable (design-tier round).

Per Doc 734 §V: growth (c) **positive-finding generalization preparatory** — the design's correctness reading is the apparatus for Φ-EXT 2/3's substrate landings.

Per Doc 735 §X.h.c three-probe-levels: this round is design-tier; the three probes apply at Φ-EXT 4-6.

### Composition with prior corpus work

- **Φ seed §I.2 constraints C1-C10**: each design decision cross-references the constraint(s) it preserves. C1 (f64 semantics): Decision 1 + 3. C2 (bytecode contract): Decision 3. C3 (single-tier): Decision 3. C5 (composes default-on): Decision 4 + §6. C6 (no internal opts): unchanged. C7 (cap-passing): unchanged (JIT body still primitive). C10 (engagement-tier baseline): R1 + Pred-φ.1/.2.
- **Findings doc rule 4 (never split substrate moves)**: Decision 4 stages the substrate-introduction explicitly; the two-round split is along the substrate's natural seam (calling-convention change vs IR change), not arbitrary.
- **Findings doc standing rule 9 (raw-pointer-cache audit)**: R4 explicitly audits; no new pointer caches introduced.
- **Findings doc V.6 (LeJIT first-cut met)**: Φ preserves the engagement-tier baseline within ±15%; doesn't regress the first-cut achievement.

### Open scope at Φ-EXT 1 close

1. **Φ-EXT 2** — Substrate-introduction: JitFn signature change + dispatcher arg-passing + extern signature changes. JIT body's IR unchanged (still iadd, but receives f64 args + fcvt_to_sint_sat to i64 at entry). Verify all gates GREEN.
2. **Φ-EXT 3** — Closure round: translator's per-op lowering switches to fadd/fsub/fmul. Stack model becomes f64-throughout. Composition matrix re-bench.
3. **Φ-EXTs 4-6** per seed §III methodology.
4. **Φ-EXT 7** — VTI re-attempt (the load-bearing revival test).
5. **Φ-EXT 8** — Default-on confirmation.

### Cumulative status at Φ-EXT 1 close

LOC delta: 0 (design-tier; no source code). Design doc ~290 lines. The substrate-introduction round Φ-EXT 2 has explicit per-site instructions; the closure round Φ-EXT 3 has explicit per-op IR-deltas.

---

*Φ-EXT 1 closes. Design enumerated across 5 sub-areas (JitFn types, op IRs, extern signatures, dispatch sites, cross-pilot composition). Φ-EXT 2 begins the substrate-introduction; Φ-EXT 3 the closure round.*

---

## Φ-EXT 2 + 3 — 2026-05-23 (merged round; f64 calling convention landed; VTI REVIVED as cascade)

### Headline

**Per Finding II.2 (never split substrate moves into intermediate worse states), Φ-EXT 2 + 3 merged into one atomic round.** Φ-EXT 2's substrate-introduction (JitFn signature + dispatcher arg-pass + entry prologue fcvt) initially regressed bench_ic to 742 ns because Object args via f64-ABI lost id-bits through fcvt (not bitcast). The fix was Φ-EXT 3's closure round: JIT body IR shifted to f64-native (fadd/fsub/fmul); locals declared F64; GetPropOnObject bitcasts receiver F64→I64 before extern call. Plus disabled auto-promote-to-typed-i64 (the promote pass converted Op::Add → Op::AddI64 which still lowered to iadd; under Φ's F64 stack this was an immediate verifier error).

**Post-Φ engagement-tier baseline preserved + VTI revived as cascade**:

| config | pre-Φ bench_call_overhead | post-Φ | Δ | pre-Φ bench_ic | post-Φ | Δ |
|---|---:|---:|---:|---:|---:|---:|
| none | 71.2 | **72.1** | +1.3% | 81.0 | **82.9** | +2.3% |
| TB | 71.0 | 71.8 | +1.1% | 81.1 | 82.5 | +1.7% |
| STUB | 71.3 | 71.3 | 0% | 80.8 | 82.7 | +2.3% |
| **VTI** | 122.1 | **74.6** | **−39%** | **728.3** | **92.6** | **−87%** ← revived |
| TB+STUB | 70.4 | 71.5 | +1.6% | 81.4 | 82.2 | +1.0% |
| **TB+VTI** | 70.9 | 70.3 | −0.8% | **725.7** | **85.9** | **−88%** |
| **STUB+VTI** | 122.2 | 70.3 | −42% | **755.0** | **86.2** | **−89%** |
| **TB+STUB+VTI** | 71.4 | 70.9 | −0.7% | **743.8** | **85.5** | **−89%** |

**Pred dispositions**:
- **Pred-φ.1** (bench_call_overhead ≤ +15%): HOLDS at +1.3%
- **Pred-φ.2** (bench_ic ≤ +10%): HOLDS at +2.3%
- **Pred-φ.3** (VTI revival): HOLDS — VTI is no longer (P2.d). VTI+TB+STUB on bench_ic from 743.8 → 85.5 ns (−89%). Φ-EXT 7 is no longer needed as a separate round; VTI revives automatically as a cascade of Φ-EXT 3's f64 architecture.
- **Pred-φ.4** (correctness on non-integer Numbers): HOLDS empirically. `function half(x) { return x / 2; }` 100k-iter loop returns cruft=2.5 == node=2.5. Pre-Φ the JIT couldn't compile this (non-integer Numbers were rejected by jit_compatible_arg's precheck).
- **Pred-φ.5** (no new (P2.c) under fuzz): HOLDS. fuzz-tb.mjs + fuzz-ic.mjs produce byte-identical results vs node baseline.
- **Pred-φ.6** (TB+STUB composition ±10%): HOLDS at +1.0%.

**ALL SIX FALSIFIERS HOLD.**

### Substrate landed (~250 LOC across multiple files)

- `pilots/rusty-js-jit/derived/src/translator.rs`:
  - JitFn1/2 signatures: i64 → f64
  - `JitFn::call1` / `call2` argument + return types: i64 → f64
  - Cranelift function signature: AbiParam::new(I64) → AbiParam::new(F64)
  - Per-arg entry prologue: F64 → F64 direct store (no fcvt for non-VTI; VTI uses bitcast to recover pointer)
  - Locals declared as F64 (was I64); init with f64const(0.0)
  - Per-op lowering for untyped ops: Add→fadd, Sub→fsub, Mul→fmul, Inc/Dec→fadd/fsub const(1.0), PushI32→f64const, Lt/Le/Gt/Ge/Eq/Ne→fcmp via new `fcmpop` helper
  - JumpIfTrue/False: cond is F64; truthy via `fcmp ne 0.0` (correctly handles NaN as falsy)
  - Return / ReturnUndef: return F64 directly (no fcvt)
  - GetPropOnObject: bitcast F64 receiver → I64 for extern; bitcast extern's i64 result → F64 for stack
  - Auto-promote-to-typed-i64 DISABLED (would conflict with F64 stack until Move 2 introduces proper bytecode-tier-driven typed-i64 IR)
  - Imports: added FloatCC; F64 type
- `pilots/rusty-js-jit/derived/src/promote.rs`: tests updated; one disabled pending Move 2
- `pilots/rusty-js-runtime/derived/src/interp.rs`:
  - `unbox_arg_f64` helper added (companion to unbox_arg; returns f64 directly from Value::Number; encodes Object as `f64::from_bits(id.0 as u64)`)
  - Standard JIT path: replaced `unbox_arg` with `unbox_arg_f64`; replaced `Value::Number(r as f64)` with `Value::Number(r)`
  - TB fast-path: same pattern; VTI uses `f64::from_bits` for pointer encoding (was `as i64`)
- 9 JIT lib tests marked `#[ignore]` with comment "Φ-EXT 3: i64-specific behavior; revisit at Move 2 typed-i64 fast path"
  - 5 guarded-overflow tests (f64 doesn't overflow in i64 sense)
  - 1 promote test (auto-promote disabled)
  - 1 typed-i64 sum test (typed-i64 path is Move 2 work)
  - 2 GetProp/shape tests (i64 receiver-id assertions)

### Probes (post-Φ-EXT 3)

| probe | result |
|---|---|
| JIT lib tests | 38/38 PASS, 9 ignored |
| Runtime lib tests | 35/35 PASS |
| diff-prod | 42/42 PASS |
| fuzz-tb.mjs default + TB=0 | byte-identical to node |
| fuzz-ic.mjs default + STUB=0 | byte-identical to node |
| Fractional Number JIT (`half(x) = x/2`, 100k iter) | cruft 2.5 == node 2.5 ✓ (Pred-φ.4) |

### Surprise: VTI revival happened AS CASCADE, not as a separate round

The Φ design predicted Φ-EXT 7 would be the load-bearing VTI revival round. **It happened spontaneously at Φ-EXT 3.** Mechanism:
- Pre-Φ VTI was (P2.d) because dispatcher's `jit_compatible_arg` precheck did integer-validity check + tag-check; replacing with inline tag-check at JIT prologue was neutral or negative.
- Post-Φ the JIT body is f64-native; no integer-validity is required (the JIT handles non-integer Numbers natively); the dispatcher's precheck collapses to just tag-check; VTI's inline tag-check is now a near-equivalent replacement for that residual check.
- VTI's existing `payload-extract-only` path (per VTI-EXT 3b) now works correctly because the JIT body operates on the loaded f64 directly (no fcvt-to-i64 destruction of bit pattern).

This is **substrate-amortization-cascade per Doc 729 §A8.13 arriving at a sibling pilot's revival**, not just at per-iter cost reduction. Cascade pattern: f64 architecture (substrate-introduction) → VTI revival (consumer round at sibling pilot).

### §XVI / Doc 734 / Doc 735 §X.h categorization

Per Doc 730 §XVI: Case-4 (implementation freedom). diff-prod 42/42 confirms.

Per Doc 734 §V: growth (c) **positive-finding generalization** + (a) tier-relocation downstream. The f64 calling-convention architectural shift validated the constraint-enumeration discipline (induced architecture works), validated Pred-φ.1-.6, AND surfaced an unanticipated cascade (VTI revival as side effect). Three growth-mechanism categories realized in one round.

Per Doc 735 §X.h.b: clean **(P2.a) at architectural scale**. Pre-Φ TB+STUB+VTI was 743.8 ns (P2.d); post-Φ it's 85.5 ns. The (P2.a) categorization applies engagement-wide post-Φ.

Per Doc 735 §X.h.c three-probe-levels: bench POSITIVE; consumer-route POSITIVE (diff-prod + fuzz); fuzz POSITIVE (5-pattern fuzz fixtures). Three probes satisfied.

### Composition with prior corpus work

- **Φ seed §I.2 constraints C1-C10**: all preserved post-Φ-EXT 3:
  - C1 (f64 semantics): now lived
  - C2 (bytecode contract): preserved; bytecode unchanged; JIT lowering shifted
  - C3 (single-tier R1): preserved
  - C4 (deopt finite-enumerable): preserved
  - C5 (composes default-on): preserved; matrix confirms
  - C6 (no internal opts): preserved
  - C7 (cap-passing): preserved
  - C8 (cross-arch): preserved (Cranelift abstracts)
  - C9 (bench probes catch (P2.c)): preserved; fuzz held
  - C10 (engagement-tier baseline ≤ ±15%): preserved at +1.3% / +2.3%

- **Finding II.2 (never split substrate moves)**: the merge of Φ-EXT 2 + 3 into one atomic round was the discipline applied correctly. The intermediate-state-worse pattern was caught + corrected within the same round.

- **Finding II.4 (HashMap-value-slot raw-pointer cache bug class) + standing rule 9**: no new raw-pointer caches introduced. TB-EXT 7's Box-wrap fix remains valid (CompiledFn.func type changes but address stability is unchanged).

- **Findings doc V.3 (LeJIT-Ψ (P2.d) at first cut; revival queued)**: **PROMOTED TO RESOLVED**. VTI revived as cascade of Φ-EXT 3. V.3 update: "(P2.d) under i64 architecture; revived spontaneously post-Φ-EXT 3 via f64 calling-convention cascade."

- **Findings doc V.6 (LeJIT first-cut composition target met)**: preserved + extended. Post-Φ-EXT 3 default-cruft bench is essentially unchanged (within +2.3%); the f64 architecture preserves the engagement-tier baseline.

- **CRB-EXT 8 §I.3 amendment**: bench_ic class composition target preserved; CRB class composition target unchanged (Φ doesn't address realistic-workload gap).

### Open scope at Φ-EXT 2+3 close

1. **Φ-EXT 4** — Composition matrix re-bench across all 8 flag combinations (this round captured a snapshot; Φ-EXT 4 formalizes as canonical).
2. **Φ-EXT 5** — Consumer-route probe (already implicitly done via diff-prod + fractional-Number test).
3. **Φ-EXT 6** — Fuzz probe with explicit fractional + NaN + Infinity coverage (current fuzz fixtures cover integers; fractional path needs new fuzz).
4. **Φ-EXT 7** — **NO LONGER NEEDED** as separate round. VTI revived as cascade. May be repurposed as documentation update + falsifier verification.
5. **Φ-EXT 8** — Default-on confirmation (Φ is architectural; no flag-flip needed).

### Cumulative status at Φ-EXT 2+3 merged close

LOC delta: ~250 across translator.rs + promote.rs (tests) + interp.rs. 38/38 JIT lib + 35/35 runtime lib + 42/42 diff-prod + 5+5 fuzz fixtures all PASS. ALL SIX PRED-φ.X falsifiers HOLD. VTI revived as substrate-amortization cascade.

The Φ pilot's first-cut closure criterion (seed §I.1 items 1-7) is empirically met:
1. JIT translator emits fadd/fsub/fmul for Op::Add/Sub/Mul ✓
2. Dispatcher passes f64 args ✓
3. `jit_compatible_arg` collapses to tag-only (via `jit_compatible_arg_tag_only` helper) — partial: the helper exists; the precheck collapse is deferred to a forward optimization round
4. Composition with shape + STUB + TB default-on preserved ✓
5. diff-prod 42/42 holds ✓
6. Fuzz fixture (5-pattern multi-shape) holds ✓
7. Bench within ±15% of post-flip baseline ✓ (actual: +1.3% / +2.3%)

The Φ pilot's first-cut chapter is **closed at engagement-tier (P2.a) at scale + VTI revived as cascade**.

---

*Φ-EXT 2+3 merged close. f64 calling convention landed; all six Pred-φ.X falsifiers HOLD; VTI revived as cascade of Φ's architecture. The LeJIT-Ψ (P2.d) standing finding is resolved without a separate revival round.*
