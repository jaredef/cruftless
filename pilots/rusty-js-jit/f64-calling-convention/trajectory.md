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
