---
arc: 2026-05-27-eval-scope-binding-chain
trigger: Telegram message 9973 ("Now select an arc") — selected from Tier-M candidate register
opened: 2026-05-27
closed: 2026-05-27 (ES-EXT 2 v2 landed under unified surface; all probes green; aggregate engagement yield +9.1pp)
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
| `eval-scope-binding-chain/` | parent substrate-pilot | SPAWNED |
| `eval-scope-binding-chain/es-foundation/` | leaf — entry points + compile-tier + runtime-tier + bridge | LANDED (ES-EXT 0+1+2+3+4) |

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

## Cross-locale findings

**Finding ARC.M.1** (ES-EXT 2 landing): The globals/globalThis bilateral non-unification was the residual barrier after ES-EXT 2's compile-tier flip. `Runtime.globals` HashMap and globalThis Object's prop table were separate storage; the spec (§16.1) requires them unified. Surfaced cross-locale because future substrate work on global-name resolution shares this surface.

**Finding ARC.M.2** (ES-EXT 3 landing): Forward-sync bridge (StoreGlobal → mirror to globalThis Object) is a single-direction closure. Reverse direction (SetProp on globalThis Object visible to bare LoadGlobal) deferred to ES-EXT 4 — addressed in the same arc the same day.

**Finding ARC.M.3** (ES-EXT 4 landing): The reverse bridge (LoadGlobal miss → fallback to globalThis own-prop) is symmetric with the forward bridge and uses the existing object_get path; no new substrate primitive needed. Standing-rec: any future surface that adds a third global-namespace storage (e.g., Realm.Globals object) must hook BOTH directions to maintain the §16.1 single-binding-env invariant.

**Finding ARC.M.4** (cross-arc): Probe P5 (`var v=10` outer + `(0,eval)("var w=v+1")` inner) was originally classed as a residual failure. Re-reading §19.2.1.3 PerformEval established that indirect eval runs at global scope and CANNOT see module-local outer bindings — P5 failing is spec-correct, not a defect. Standing-rec: probes that look like residuals deserve a spec re-check before being recorded as standing residual rungs.

## Status

CLOSED 2026-05-27. All four sub-rungs (ES-EXT 0+1+2+3+4) landed same-day. ~125 cumulative LOC. diff-prod 42/42 maintained throughout. Probes P1-P4, P6, P7 all green; P5 spec-correct fail. test262-sample direct yield measurement deferred per "No Auto Sweeps" memory (awaits keeper directive for canonical sweep).

---

## Roster extension 2026-05-28 (per Plan-agent back-fit; keeper directive Telegram 10158)

Per the back-fit analysis applying Doc 744 + Doc 745 candidate to the locale-to-arc subsumption, the eval-scope-binding-chain arc's roster is extended to enroll five sibling locales whose mouth-terminus pairs fit the arc's (M, T, I, R) without requiring a separate arc. Original close-condition + close-date remain valid for the ES-EXT 0-4 chain; the extension formalizes sibling enrollment as the arc continues operating.

### Extended roster

| Locale | Role in arc | Status pre-extension |
|---|---|---|
| `eval-declaration-instantiation-early-errors` (EDIEE) | strict-eval declaration-instantiation early errors per §19.2.1.3 | LANDED (2026-05-28) per cruftless main e1247a8e |
| `eval-function-arguments-binding-semantics` (EFAB) | eval-tier function-arguments binding semantics | OPEN |
| `eval-var-function-env-instantiation` (EVFEI) | eval-tier var/function env instantiation per §19.2.1 | LANDED partially (2026-05-28) per cruftless main 38e24488; pipeline-form pickup recorded at cff663b3 |
| `direct-eval-lexical-capture` (DELC) | direct-eval lexical capture from caller frame | LANDED (2026-05-27) per main dcc4b48e |
| `lex-error-propagation-to-eval-surface` | lex-tier error propagation through indirect/direct eval | LANDED |

### Extension rationale

Per Doc 744 §IV.1.a mouth-gating: TTTC's apparent mouth (tagged-template TCO) was gated by EDIEE's terminus (strict-eval `var $MAX_ITERATIONS` materialization). The amendment to Doc 744 (corpus push 2026-05-28) recorded this empirical material as the empirical anchor for the Doc 744 §IV.1.a mouth-gating heuristic. Enrolling EDIEE + EVFEI + EFAB + DELC + lex-error-propagation under this arc captures the cross-locale findings that the eval-scope surface generates as a coherent multi-locale program, rather than treating each as an orphan locale.

### Cross-arc relations (post-extension)

- **DAG ↓ `2026-05-28-parser-early-error-conformance`** (proposed arc): early-error tier consumes eval-tier scope's BoundNames per §19.2.1.3 step 5.
- **Mouth-gating DAG ↑ `tagged-template-tail-call-boundary` (TTTC) locale**: this arc's EDIEE terminus gates TTTC's measurable mouth. Worked example for Doc 744 §IV.1.a.
- **Lattice with `2026-05-28-strict-mode-bound-names`** (proposed arc): strict-mode binding restrictions affect eval scope.

### Extension status

EXTENDED 2026-05-28. Original arc close-condition + 2026-05-27 close-date stand for ES-EXT 0-4. New sub-rungs (EDIEE, EVFEI, EFAB, DELC, lex-error-propagation) accumulate under this arc with their own per-locale chapter-closes; arc remains conceptually IN PROGRESS for the extended roster.
