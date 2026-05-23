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
