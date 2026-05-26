# Standing Rule 11 (5-Axis Pre-Spawn Check) → Pilot Mis-Targeting Prevention

## Induced property

A pilot spawned under standing rule 11's discipline will not **mis-target** — the substrate work will not be correct on its own terms while irrelevant to the bench's actual cost surface. The induced property is that pilot-funding decisions made BEFORE substrate work begins are pre-validated against five independent coverage axes, each of which is a known historical mis-targeting class.

Anchor: `apparatus/docs/predictive-ruleset.md` Rule 11. Empirical anchor: JSF pilot (Addendum IV / Finding VII.1) where JSON.stringify was projected at 50-70% of cost; component A/B revealed the actual dominator was character-scanning at 77% of wall-clock; JSF reclaim was -1% within noise. Rule 11 was extracted from that surface.

## The accumulation

| # | Constraint | Adds | Induces |
|---|---|---|---|
| 0 | (Null) spawn pilot from intuition about which surface dominates | — | mis-targeting whenever intuition is wrong (the JSF baseline failure mode) |
| 1 | **Axis A1: Component A/B probe required** — empirically identify the actual hot-path component via additive variants (runs in <10 minutes) | empirical dominator identification | property: "pilot-target is the measured dominator, not the projected one" |
| 2 | **Axis A2: Op-set coverage** — confirm the planned substrate work's op-set covers the dominator's op-set | op-set fit check | property: "the substrate move's reach includes the cost-bearing operations" |
| 3 | **Axis A3: Value-domain coverage** — confirm the value-domain matches (e.g., NaN-boxing covers the receiver tag set) | type-domain fit check | property: "the substrate move's discriminations cover the values flowing through the hot path" |
| 4 | **Axis A4: Locals-marshaling coverage** — confirm the entry-mechanism marshals the relevant locals | calling-convention fit check | property: "the substrate move's call-graph interface admits the hot path's actual frames" |
| 5 | **Axis A5: Emission-shape coverage** — confirm the JIT/lowering emission shape matches the dominator's structure | code-gen fit check | property: "the substrate move's emitted shape composes with the hot path's structure" |

The named composition (1+2+3+4+5) is **standing rule 11**. The induced property is pilot mis-targeting prevention across five distinct mis-targeting classes that the engagement's history identifies as load-bearing failure modes. Each axis closes a specific projection-vs-reality mismatch class.

Removing axis A1 admits the JSF-shape mis-target (projection wrong; substrate work on the wrong dominator).
Removing axis A2 admits the substrate-touched-but-op-set-narrow class (substrate work covers the right tier but doesn't fire for the operations actually executed).
Removing axis A3 admits the value-domain-narrow class (substrate work handles the wrong type set).
Removing axis A4 admits the entry-mechanism class (substrate work can't see the relevant frame state).
Removing axis A5 admits the emission-shape class (substrate work's code-gen is wrong shape for downstream composition).

The five axes are operationally independent — each catches a different class of mis-target — but they compose multiplicatively in the sense that satisfying any one without the others is insufficient. A pilot that passes A1 (the right dominator is targeted) but fails A4 (the substrate can't see the relevant locals) still mis-targets.

## Tag on the DAG

This is an **engagement-tier discipline coordinate**:

```
apparatus/spawn-discipline ::
  E0/pre-spawn ::
  cut/multi-axis-coverage-check ::
  property/pilot-mistargeting-prevention
```

Per the predictive ruleset's §Predictive coverage map, rule 11 is the explicit prevention rule for "Cascade over-projection" (jointly with rule 7). Empirically held: every pilot since JSF that ran the 5-axis check landed on the actual dominator; every pilot that skipped axis A1 mis-targeted.

The DAG-projection reading: rule 11 is the FCA structural template applied at the **pilot-spawn-tier substrate-engineering** coordinate. Constraint 1 (A1) is structurally parallel to Fielding's client-server constraint — both are the first move that turns an unconstrained design space into a measurable one. The remaining four axes are structurally parallel to Fielding's downstream constraints (each adds a property the prior configuration didn't have).

## Composes-with

- `apparatus/docs/predictive-ruleset.md` Rule 11 — primary articulation.
- `pilots/rusty-js-jit/findings.md` Addendum IV — empirical extraction.
- Standing rule 13 (revert-then-deeper-layer) — when rule 11's axes hold prospectively, rule 13 can be applied at funding time.
- [`docs/fca-instances/pin-art-resume-vector.md`](pin-art-resume-vector.md) — rule 11 is the spawn-time gate on locale founding.

## Falsification

A pilot that passes the 5-axis check at funding time and yet produces zero or negative empirical yield falsifies the rule's prevention claim for that pilot's class — would indicate a sixth axis not yet named. Empirically held to date; the JSF-shape mis-target has not recurred since rule 11 was operationalized.
