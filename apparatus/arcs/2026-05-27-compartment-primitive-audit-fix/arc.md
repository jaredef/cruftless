---
arc: 2026-05-27-compartment-primitive-audit-fix
trigger: Telegram message 10038 ("Continue") after Telegram 10037 reported arc 2026-05-27-engine-tier-substrate-readiness-for-compartments closed with three implicit constraints named (IC.CP1, IC.CP2, IC.CP3) by the P-C probe set
opened: 2026-05-27
closed: 2026-05-27 (single-rung landing; all P-C probes hold; gates maintained)
close_condition: P-C probe set re-runs with all 8 probes holding (no REFUTED, no inconclusive that materially affects the property); CP-EXT 1-4 substrate aligned with the post-GBSU + post-ESBC unified-surface architecture; test262-sample baseline maintained (≥86.6%) with the Compartment-tagged test fixtures additionally passing.
---

# Compartment Primitive Audit-Fix Arc

## Trigger

Per Doc 743 §V (P-C property statement) + ARC.MR.4 standing rec (formalization-before-implementation discipline): the falsifier probe set for P-C executed today refuted the property in 4 of 8 probes, surfacing three implicit constraints in the existing CP-EXT 1-4 substrate. Per Doc 729 §XIII applied prospectively, those constraints must be closed before any further compartment work (hooks, cap-handle wrapping, cross-realm instanceof) extends the implementation. This arc is that audit-fix.

## Telos

Bring the CP-EXT 1-4 substrate into alignment with the post-GBSU + post-ESBC engine substrate so P-C as articulated in Doc 743 §V actually holds. Specifically close:

- **IC.CP1**: `Compartment.prototype.globalThis` accessor today is registered via `register_intrinsic_method` which makes it a callable METHOD; reading `c.globalThis` returns the function object itself (with `length: 0`, `name: "globalThis"`), not the realm's globalThis Object. Fix: make `globalThis` a per-instance data property pointing to the compartment's globalThis ObjectRef, set at ctor time.

- **IC.CP2**: endowments dictionary is iterated and `rt.object_set(gt, k, v)` is called on the per-compartment `gt`, but the compartment realm's actual `self.global_object` during `enter_realm` is NOT swapped to that `gt` — enter_realm filters the PRIMORDIAL global_object via retain_global_string_props, never swapping. So writes to `gt` at ctor time don't appear inside `compartment.evaluate("X")` because the realm's binding-resolution path consults a different Object. Fix: route `enter_realm` for compartment realms to swap `self.global_object` to the compartment's gt; ensure gt is pre-populated with the intrinsic allowlist + endowments at ctor time.

- **IC.CP3**: `compartment.evaluate` routes through `rt.evaluate_module`, which uses Module semantics — top-level `var` declarations become module-local bindings that don't persist on the realm's globalThis. Per ECMA §16.1 + §19.2.1.3, Script semantics is required (`var X` attaches to globalThis). Fix: route compartment.evaluate through `rt.evaluate_script` (the ESBC v2 entry point) so the compile-tier flip emits the StoreLocal + StoreGlobal pair at script-top-var declaration. With IC.CP2 fixed (gt is the realm's global_object), the StoreGlobal writes persist on the compartment's globalThis across evaluate calls.

## Sub-locale roster

| Locale / Sub-rung | Role | Status |
|---|---|---|
| `pilots/compartment-primitive/` | Top-level substrate locale spawned by this arc | SPAWNED |
| CPF-EXT 1 (globalThis accessor fix) | IC.CP1 closure | PLANNED |
| CPF-EXT 2 (realm-globalThis swap) | IC.CP2 closure (structural) | PLANNED |
| CPF-EXT 3 (gt population at ctor) | IC.CP2 closure (init) | PLANNED |
| CPF-EXT 4 (evaluate_script routing) | IC.CP3 closure | PLANNED |
| P-C probe re-run | Verification rung | PLANNED |

## Methodology

Per arc-as-coordinate.md + Pin-Art Rule 23 founding-probe discipline:

1. **Founding probe** (already executed in prior arc): the P-C probe set IS this arc's founding measurement. Three IC named.
2. **Per-rung substrate move**: each CPF-EXT rung addresses one IC. Single closure per rung (R4).
3. **Per-rung verification**: after each rung, re-run the affected P-C probe + diff-prod + test262-sample. Hold the 86.6%+ baseline.
4. **Arc close**: when all 8 P-C probes pass + sweep parity, the arc closes; Doc 743 §V P-C is empirically grounded.
5. **§XIII discipline applied throughout**: any regression names an implicit constraint the prior plan missed; the plan revises accordingly.

## Cumulative yield (populates as rungs land)

| Checkpoint | P-C probes passing | test262-sample | Notes |
|---|---|---|---|
| Pre-arc (current state) | 4 of 8 (1 hold + 3 inconclusive treated as not-passing) | 86.6% (6296/978/397) | Substrate ready, implementation behind |

## Cross-locale findings (populate as rungs land)

(empty — arc just opened)

## Composes-with

- `pilots/global-binding-surface-unification/` (LANDED) — the precondition that lets enter_realm be a globalThis swap rather than a property-table filter.
- `pilots/eval-scope-binding-chain/es-foundation/` (LANDED) — the precondition that lets compartment.evaluate use evaluate_script with the StoreLocal+StoreGlobal-at-decl mirror.
- Doc 729 §VII.B engine-helpers bilateral — preserved invariant; the compartment's globalThis must NOT expose engine helpers.
- Doc 736 capability-passing — the endowments dictionary IS the capability-handle passing mechanism this arc unblocks.
- Doc 743 P-C — the property this arc validates by empirical probe.

## Status

CLOSED 2026-05-27. All three implicit constraints (IC.CP1 + IC.CP2 + IC.CP3) closed in a single substrate move (CPF-EXT 1+2+3+4 collapsed to one rung per ARC.AF.1 — the ICs share a structural invariant; staged landing would have produced intermediate states with no observable property). Sweep parity at 86.6%; diff-prod 42/42; all concrete P-C probes hold. ~80 LOC added + ~20 LOC deleted. Zero §XIII recurrences during land — the falsifier probe set in the prior arc pre-articulated every constraint the audit needed to close (ARC.AF.2 standing rec).
