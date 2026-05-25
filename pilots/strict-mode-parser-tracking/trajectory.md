# strict-mode-parser-tracking — Trajectory

## SMPT-EXT 0+1 — founding + closure (2026-05-25)

**Trigger**: EPSUA-EXT 3; keeper "2" (constraint #2 — strict-mode parser tracking). Pre-scoping per EPSUA C4 narrowed scope from prospective ~80 to in-scope ~12 (top-level yield-ident-valid only); strict-mode + generator-context axes deferred.

**Edits** (~20 LOC) — see seed §II.

**Verification**:
- Minimal probe: `var yield = 4; for ([x = yield] of [[]]) print(x)` → 4 ✓
- Exemplar (yield-ident-valid, 12 tests): PASS 0 → 8 (+8, 67%)
- Regression: for-of (495), for-in (79), arrow-function (264) → unchanged (0)

### Findings

**Finding SMPT.1**: simple parser-state addition (function_body_depth counter, 20 LOC) closed 8/12 of the top-level yield-as-identifier sub-cluster cleanly. The remaining 4 require deeper tracking (generator-vs-non-generator OR strict-vs-sloppy within function-bodies) — deferred to SMPT-EXT 2+ candidates.

**Finding SMPT.2 (EPSUA arc-tier)**: fourth constraint sub-locale; cumulative EPSUA = 21 actual / ~163 projected = **13% of prospective amortization** (essentially flat). Pred-epsua.4 (≥2 within projection) appears falsified across constraints #4, #5, #2. The prospective doc's projections were uniformly inflated by the matrix-aggregation pattern.

**Status**: CLOSED at SMPT-EXT 1.

## SMPT-EXT 2 — strict_mode parser state + arrow-param strict reserveds (2026-05-25)

**Trigger**: keeper "Let's check out full strict mode tracking" after PPAE-EXT 4 split is_reserved_word into unconditional + broad, and the arrow-param check needed the broad set in strict contexts.

**Edits** (~50 LOC):
- `parser.rs::Parser`: add `strict_mode: bool` field (init false).
- `parser.rs::peek_use_strict_directive`: source-byte peek for `"use strict"` / `'use strict'` at current lookahead (no token consumption).
- `parser.rs::parse_module`: detect "use strict" directive prologue at entry; set strict_mode.
- `stmt.rs::parse_function_body`: detect "use strict" directive at body entry; save prior strict_mode; restore on body exit. Inner functions inherit parent strict.
- `expr.rs` yield branch: extended condition `(function_body_depth > 0 || strict_mode)` — strict-mode yield is unconditionally YieldExpression.
- `expr.rs::parse_arrow_function` reserved-word check: mode-gated — uses `is_reserved_word` (broad, incl. strict-only) + eval/arguments in strict; `is_unconditional_reserved_word` (Keyword only) in sloppy.

**Verification**:
- `"use strict"; var af = arguments => 1;` → SyntaxError ✓
- `"use strict"; var af = (yield) => 1;` → SyntaxError ✓
- `var af = (yield) => 1; af(1)` → 1 (sloppy valid) ✓
- `var af = arguments => arguments` → works (sloppy) ✓
- Random 300 language adjacent: 300/300, 0 regressions

**Exemplar** (24 yield-ident-invalid + bindingidentifier-no-* + identifier-strict-futurereservedword fixtures):
- PASS: 0 → **3** (the arrow-param-strict-arguments/eval/yield cases)
- 21 remaining use `yield` inside function-body or top-level-strict where compile-time YieldExpression-in-non-generator-strict throw is needed — requires generator-context tracking (SMPT-EXT 3 candidate).

### Findings

**Finding SMPT.3**: full strict-mode tracking is a structural unlock. Mode-gated predicates (is_reserved_word vs is_unconditional_reserved_word; eval/arguments as ident) now have a source-of-truth (`self.strict_mode`); per-site predicate selection becomes mechanical. SMPT-EXT 3 candidate: generator-context tracking — when active, yield in function body is YieldExpression; when inactive + strict, yield is reserved-word SyntaxError; when inactive + sloppy, yield is IdentifierReference.

**Finding SMPT.4 (sub-cluster decomposition)**: yield-ident-invalid (~12 onlyStrict tests in cluster) split into:
- arrow-param-strict-yield (3 of 12): closed by SMPT-EXT 2
- function-body-yield-as-ident-strict (~5): needs SMPT-EXT 3 (generator tracking + YieldExpression-not-in-generator-throws)
- for-of head-yield-init-strict (~4): same as above

**Status**: SMPT-EXT 2 CLOSED.
