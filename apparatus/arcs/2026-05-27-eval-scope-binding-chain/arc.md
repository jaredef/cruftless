---
arc: 2026-05-27-eval-scope-binding-chain
trigger: Telegram message 9973 ("Now select an arc") — selected from Tier-M candidate register
opened: 2026-05-27
closed: IN PROGRESS
close_condition: cruft's indirect-eval surface exposes enclosing-scope bindings (harness `assert`/`fnGlobalObject` AND user `var f` etc.) into the eval body — verified via test262 sample showing the predicted 200-400 record yield band landing or, if narrower, with substrate site documented.
---

# Eval-Scope-Binding-Chain Arc

## Trigger

Per Finding HDSB.2 (2026-05-26) standing-rec: cruft's eval frame does not surface enclosing-scope bindings into the eval body. 120 records of `assert is not defined` / `fnGlobalObject is not defined` failures surfaced inside HDSB exemplars alone; predicted to generalize across the test262 surface to 200-400+ records engagement-wide.

Twice-deferred during prior arcs (Tier-K disambiguation and the Tier-L Temporal arc). Selected per keeper directive (Telegram 9973) from Tier-M candidate register.

## Telos

Substrate fix at cruft's indirect-eval lowering — the scope chain established for the eval body must include the harness's top-level lexical bindings (and any other enclosing bindings the spec mandates).

Specifically:
- `Object.prototype.hasOwnProperty.call(globalThis, "assert")` must hold AFTER the test262 runner has prepended `sta.js + assert.js`, so that `(0, eval)(testSource)` can see `assert`.
- Today: cruft's eval frame appears to start with a fresh global view that excludes the prior eval's top-level bindings.
- Expected: bindings from sta.js + assert.js (and the harness include chain) are visible to the test source eval.

## Sub-locale roster (accumulating)

| Locale | Role | Status |
|---|---|---|
| `eval-scope-binding-chain/` (TBD spawn name) | substrate-pilot | NOT SPAWNED |

Founding-probe + Rule 23 baseline-inspection precede locale spawn.

## Methodology

Per arc-as-coordinate.md formal characteristics + Rule 23 founding discipline:

1. **Verification probe**: reproduce the failure shape via direct probe (assemble harness + test source as the runner does; observe whether assert is visible in indirect-eval).
2. **Substrate-site identification**: trace the eval lowering in cruft (`legacy/host-rquickjs/tests/test262/runner.mjs::(0, eval)(assembled)` invokes cruft's indirect-eval path; the path lives in cruft's runtime — likely `pilots/rusty-js-runtime/derived/src/interp.rs` or `intrinsics.rs::install_globals::eval`).
3. **Predict mechanism**: indirect eval per §19.2 evaluates in the realm's global scope; sequential indirect evals SHOULD see prior bindings. If cruft creates a fresh scope per eval call, the bug is structural.
4. **Spawn locale**: `pilots/eval-scope-binding-chain/` (top-level — substrate, not under Temporal).
5. **Land substrate fix**: per Rule 23 verification-probe at substrate-landing time.
6. **Measure cross-locale yield**: HDSB sub-rungs' eval-residuals are the primary measurement; test262 sample for broader yield estimate.

## Cumulative yield (will populate as rungs land)

| Checkpoint | Cumulative PASS | Notes |
|---|---:|---|

## Cross-locale findings (will populate)

(empty — arc just opened)

## Status

IN PROGRESS as of 2026-05-27. Founding-probe immediately follows.
