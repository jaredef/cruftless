# ts-resolve-type-only-imports — Trajectory

## TROI-EXT 0 — workstream founding (2026-05-24)

**Trigger**: keeper directive "A" at TRE close + TXC failure-table row 1 (30 files prototype-undefined, 77% of remaining CRUFT_FAILs). Pivoted from initially-proposed `runtime-esm-cycle-handling/` locale to this approach: detect type-only imports at TSR tier, mirroring `tsc --verbatimModuleSyntax`. The cycle disappears when the runtime-irrelevant imports are elided upstream.

**Founding artefacts**: seed + trajectory + scaffolded dirs. TROI-EXT 1 (post-strip pass + re-measure) next.

---

## TROI-EXT 1 — post-strip elision pass + chapter close (2026-05-24)

**One substrate edit**: `pilots/ts-resolve/derived/src/strip.rs::Scanner::elide_unused_imports` (~110 LOC). Post-strip pass that:

1. Marks tokens already inside existing strip ranges.
2. Walks for top-level `import ...` statements.
3. Extracts the binding names (handles `import X from`, `import { a, b as c }`, `import X as Y`, skips side-effect-only `import 'x'`).
4. Counts surviving (non-strip) Ident occurrences of each binding name across the rest of the token stream.
5. If ALL bindings have zero non-strip usages, strips the entire import statement.

Mirrors `tsc --verbatimModuleSyntax` elision rule. MVP: all-or-nothing per import (partial-strip when SOME names used + others not is deferred).

**Gates**:
- `cargo test -p ts-resolve`: ✅ 51/51 PASS (existing tests unchanged; the pass is additive)
- `cargo build --bin cruft`: ✅ clean
- diff-prod 42/42 PASS ✅
- **TCC parse-parity 98.1% → 98.4%** (+0.3 pp; minor incidental)
- **TXC execute-parity 60.7% → 69.0% (+8.3 pp)** — Pred-troi.3 HELD STRONGLY (target ≥+6 pp)
- **CRUFT_FAIL count: 39 → 8** — 31 files unblocked by ONE substrate fix
- **Pred-troi.4 HELD STRONGLY**: failure-table top tag SHIFTED from `Cannot read property 'prototype' of undefined` (30 files, ESM cycle) to `SetProp 'code' on non-object` (3 files, unrelated runtime). The ESM-cycle cluster is GONE.

### Findings

**Finding TROI.1** (LARGEST-YIELD substrate fix of the parity arc): the 110-LOC post-strip pass unblocked **31 corpus files** — +8.3 pp execute-parity in one substrate edit. This is the largest single-fix yield observed across the entire TS-parity arc.

**Finding TROI.2** (Doc 729 refinement at the resolver tier, NOT runtime tier): the rxjs ESM-cycle issue (30 files) was originally diagnosed as a runtime-substrate concern requiring cruft's module loader to add ESM live-binding cycle handling. The actual fix was at the **resolver tier** — TSR elides the type-only imports, the cycle never forms, no runtime substrate change needed. This is a substantive refinement of Finding IX.5: **the resolver-instance pattern is even more powerful than originally framed** — by correctly eliding runtime-irrelevant constructs at the resolver tier, downstream tiers don't need defensive handling for problems that should never reach them.

**Finding TROI.3** (TSR-tier saturation reached): with 8 CRUFT_FAILs remaining (all 1-3 files per category), the TSR-tier parity arc is effectively saturated. Remaining issues are genuine substrate gaps (TypeError SetProp, parse-template-tail edge cases, ts-strip error) requiring per-file investigation rather than systematic substrate fixes.

### Status: CHAPTER CLOSED at TROI-EXT 1

Standing rule 13 corroborations: 12.

**Cumulative session execute-parity**: 5.1% → **69.0%** (+63.9 pp across 4 sub-locales: TRMLE +47.6, TRE +5.9, case-label +2.1, TROI +8.3).

**Remaining 8 CRUFT_FAILs**:
- SetProp 'code' on non-object: 3 (runtime substrate, likely small fix)
- Parse expected Colon: 2
- Parse expected RBrace: 1
- Parse expected template: 1
- ts strip error: 1

Single-digit categories; long-tail; per-file investigation.
