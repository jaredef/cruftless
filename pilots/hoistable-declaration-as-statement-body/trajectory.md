# hoistable-declaration-as-statement-body — Trajectory

## HDSB-EXT 0 — FOUNDING (2026-05-26)

Spawned per keeper directive (Telegram 9853) from TECR-EXT 2 lift surface. Cluster A of Tier K (5 missing-syntax-feature concentrations). Largest single reason-shape in missing-syntax-feature (475 of 1017).

Pool: 475 fixtures from the 2026-05-25-full matrix, all emitting `parse: HoistableDeclaration is not allowed as Statement body`. Distribution:
- 190 annexB/language/eval-code/direct/
- 100 annexB/language/eval-code/indirect/
- 185 annexB/language/function-code/*

Baseline: 0/475 PASS. Verified direct probe: `if (true) function f(){ return 42; } f();` rejects at parse-time.

## HDSB-EXT 1 — LANDED (2026-05-26)

### Substrate edits

1. `pilots/rusty-js-parser/derived/src/parser.rs`: added `pub(crate) allow_annex_b_function_in_substatement: bool` field to `Parser`, defaulted false in `new`.
2. `pilots/rusty-js-parser/derived/src/stmt.rs::parse_substatement`: when `self.is_ident("function")`, check the flag + `!self.strict_mode` + lookahead-not-`*` (generator carve-out). If all hold, fall through to `parse_statement()` instead of erroring. Otherwise emit the same rejection.
3. `pilots/rusty-js-parser/derived/src/stmt.rs::parse_if_statement`: save flag → set true → parse consequent + alternate → restore flag.

Net LOC: ~25 across 2 files.

### Probes (Rule 23 verification at landing)

- `if (true) function f(){ return 42; } f();` → 42 ✓
- `"use strict"; if (true) function f(){}` → REJECTS ✓
- `if (true) function* g(){}` → REJECTS ✓ (generator carve-out)
- `for (var i=0;i<1;i++) function g(){}` → REJECTS ✓ (for-body not in carve-out)

All four expected outcomes confirmed.

### Yield

- HDSB exemplar pool: **0 → 150/475 PASS (+150, 31.6%)**.
- Diff-prod: 42/42 maintained.
- Cross-locale regression sweep:
  - numeric-literal-conformance: 147 (unchanged)
  - identifier-tokenization: 261 (unchanged)
  - string-literal-and-escape-conformance: 59 (unchanged)
  - line-terminator-conformance: 31 (unchanged)

No regression. HDSB-EXT 1 is isolated to the if-body parse site by construction (flag is only set in `parse_if_statement`).

### Residual decomposition (325 fails)

| Shape | Count | Tier | Locale destination |
|---|---:|---|---|
| `assert is not defined` (eval frame) | 90 | runtime | eval-scope-binding-chain (HDSB-EXT 3 spec) |
| `fnGlobalObject is not defined` | 30 | runtime | same |
| `Expected SameValue undefined vs function` | 50 | lowering | HDSB-EXT 2 (binding semantics) |
| `value is not updated following evaluation` | 20 | lowering | same |
| `f is not defined` (binding scope) | 25 | lowering | same |
| `Identifier f cannot be redeclared (lexical/var conflict)` | 15 | lowering | same (cruft lexical binding too strict) |
| `An initialized binding is not created` | 10 | lowering | same |
| other / drill-pending | ~85 | mixed | per-shape inspection |

### Findings

**Finding HDSB.1 (parser-extension opens the door; lowering closes the rest)**: A 25-LOC parser carve-out at a single grammar-production site closes 31.6% of the cluster directly. The remaining residuals are not parser-tier — they are lowering-tier (Annex B B.3.4 binding semantics: lexical + var binding creation with conditional evaluation paths) and runtime-tier (cruft's eval scope chain doesn't surface harness bindings). Substrate moves at different tiers must be in different locales per the post-TECR apparatus discipline. Standing recommendation: when a parser-only fix opens a previously-rejected language construct, expect 30-50% direct yield with the remainder requiring lowering/runtime substrate at separate locales — don't attempt to chase those residuals in the parser-tier locale.

**Finding HDSB.2 (the eval-scope harness gap is a cluster of its own)**: 120 of the 325 residuals are `assert / fnGlobalObject is not defined` in direct/indirect eval. The pattern matches across HDSB, but is independent of the HoistableDeclaration mechanism — these tests use eval to exercise the Annex B B.3.4 path because the spec language carves out eval-code specifically. A separate locale (eval-scope-binding-chain) should target the cruft eval-frame's failure to surface enclosing-scope bindings. Estimated pool: likely 200-400+ records engagement-wide if the pattern generalizes beyond HDSB.

### Status

HDSB-EXT 1 CLOSED. HDSB-EXT 2 + HDSB-EXT 3 deferred as separate substrate scopes (lowering + runtime tiers respectively).
