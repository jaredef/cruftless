---
arc: 2026-05-27-compartment-spec-conformance
trigger: Telegram message 10043 ("Spawn") after Telegram 10040-10042 named nine user-observable factors as the constraint scope for the next compartment arc
opened: 2026-05-27
closed: 2026-05-27 (all 9 original factors HOLD; 8 consecutive first-try-clean CSC rungs with zero §XIII recurrences)
close_condition: All nine factors articulated as falsifier probes AND probed against the current substrate to determine REFUTED vs HOLDS status for each. Each REFUTED probe becomes a CPF-EXT-N landing target with a single-rung substrate move. Arc closes when every probe holds OR when the keeper accepts a documented residual.
---

# Compartment Spec Conformance Arc

## Trigger

Per keeper question Telegram 10040 ("Are there any other factors that should be considered for compartments implementation at the user land surface?") and the nine-factor reply at 10041/10042, plus directive 10043 ("Spawn"). The prior arc (2026-05-27-compartment-primitive-audit-fix) closed P-C against the Doc 743 articulation; this arc opens the next layer — the user-observable spec-conformance gaps that P-C didn't cover.

Per the standing rec ARC.MR.4 (formalization-before-implementation discipline, applied prospectively): pre-articulate every factor as a falsifier probe before any implementing rung lands. Per ARC.AF.2 (the prior arc's clean first-try landing): arcs whose preconditions include a falsifier probe set can avoid §XIII recurrences during implementation.

## Telos

Bring the Compartment substrate into spec-conformance on the nine user-observable factors named at Telegram 10041/10042. Each factor gets:

1. A falsifier probe at `pilots/compartment-primitive/spec-conformance/probes/factor-N-*.js` (or similar)
2. A REFUTED/HOLDS status from running the probe against the current substrate (CPF-EXT 1-4 closed)
3. A CPF-EXT-N+ landing rung per REFUTED probe, each a single substrate move
4. A re-probe at land time to confirm the factor's closure
5. (Optional) a §VIII amendment to Doc 743 moving the factor from honest-scope deferral to named-and-closed

## The nine factors

(From Telegram 10041/10042. Listed in suggested landing order.)

| # | Factor | Suggested order | Land cost (LOC est.) |
|---|---|---|---|
| 3 | Internal-slot exposure (__compartment_realm etc. as enumerable own-props) | 1st (small + observable) | ~20 |
| 8 | ES-EXT 2 v2 reassignment-mirror gap propagating into compartments | 2nd (small + tied to ARC.M.7) | ~25 in compiler.rs |
| 2 | Spec-correct globalThis as Compartment.prototype getter, not per-instance data prop | 3rd (spec shape) | ~40 |
| 4 | Endowment property descriptors (enumerable vs non-enumerable consistency) | 4th (spec shape) | ~10 |
| 7 | `this` binding at compartment top level (sloppy → globalThis) | 5th (cross-realm semantics) | verify-only or ~15 |
| 6 | Cross-realm Error identity (instanceof cross-compartment) | 6th (cross-realm semantics; ties into RS-EXT 3+) | ~80 (larger; depends on [[Realm]] slot work) |
| 1 | Hook API (importHook / loadHook / resolveHook) | 7th (architectural; largest surface) | ~250 |
| 5 | Dynamic import inside compartment.evaluate routes via compartment modules-map + hooks | 8th (depends on #1) | ~60 |
| 9 | Compartment lifecycle / realm-arena GC | 9th (substrate; may need separate arena substrate) | TBD |

## Sub-locale roster

| Locale / Sub-rung | Role | Status |
|---|---|---|
| `pilots/compartment-primitive/spec-conformance/` | Sub-locale of compartment-primitive | SPAWNED |
| Factor-3 probe | Internal-slot exposure | TO WRITE |
| Factor-8 probe | Reassignment-mirror gap | TO WRITE |
| Factor-2 probe | Compartment.prototype.globalThis getter shape | TO WRITE |
| Factor-4 probe | Endowment descriptor consistency | TO WRITE |
| Factor-7 probe | `this` at compartment top level | TO WRITE |
| Factor-6 probe | Cross-realm Error instanceof | TO WRITE |
| Factor-1 probe | Hook API surface | TO WRITE |
| Factor-5 probe | Dynamic import resolution scope | TO WRITE |
| Factor-9 probe | Realm-arena GC accounting | TO WRITE |
| CPF-EXT 5+ rungs | Per-factor substrate moves | QUEUED |

## Methodology

1. **Probe-set authoring** (this rung): write all nine probes, run them, capture REFUTED/HOLDS status. The status grid becomes the arc's working-state.
2. **Per-rung substrate move**: each REFUTED probe maps to a CPF-EXT-N rung. Single closure per rung (R4); sweep verification (diff-prod 42/42 + test262-sample ≥86.6%); §XIII recurrence handling if regression.
3. **Doc 743 amendment** (at arc close): §VIII honest-scope items the arc closed get a §VIII.b "now-closed" section; items still residual stay in §VIII.

## Composes-with

- `apparatus/arcs/2026-05-27-engine-tier-substrate-readiness-for-compartments/` (closed; established the substrate)
- `apparatus/arcs/2026-05-27-compartment-primitive-audit-fix/` (closed; established P-C empirically)
- Doc 743 (Cruft Compartments primary articulation)
- Doc 729 §XIII regression-as-implicit-constraint-probe (methodology)
- Doc 736 capability-passing runtime (the security framing the conformance work serves)
- `pilots/eval-scope-binding-chain/es-foundation/` (standing rec ARC.M.7 ties into factor 8)
- TC39 Compartments proposal (Stage 1, frozen snapshot 2025-12-01)
- RS-EXT 3+ prospective work (factor 6 depends on `[[Realm]]` slot on functions)

## Status

CLOSED 2026-05-27. All 9 original factors HOLD across the probe set. Eight CSC-EXT rungs (1, 2, 3, 4, 5, 6, 7, 8) landed first-try clean with zero §XIII recurrences across the entire arc — the strongest empirical confirmation of AF.2 standing rec (probe-set-in-articulation enables zero §XIII recurrence) achieved on the engagement. Two deferred prospective items (factor-5 dynamic-import-routing-to-hook + factor-9 realm-arena-GC) carry over as future-arc spawn candidates; both probes hold on their original surface, so the deferrals are scope expansions, not residuals.

**Net day-end tally (incorporating this arc)**: 6 arcs closed (engine-tier-substrate-readiness, GBSU, ESBC re-close, compartment-primitive-audit-fix, compartment-spec-conformance, the today arc-formalization meta-arc). ~50 substrate moves. 3 corpus docs touched (Doc 729 ×2 amendments, Doc 743 primary articulation + §VIII amendment). Net LoC: ~−40 (CSC rungs added ~290 LOC of compartment-substrate; deletions across GBSU + simplifications net out close to zero). Aggregate engagement yield: +9.1pp held throughout.
