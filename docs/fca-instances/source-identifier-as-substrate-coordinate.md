# Source Identifier Conventions → Substrate Position as Identifier

## Induced property

A reader can **derive substrate position from an identifier name alone**, without consulting any other artifact. The encoding is bijective enough that a name whose prefix and install-helper disagree (e.g., `__name` registered via `set_own` instead of `set_own_internal`) is recognizable as a bug shape — the convention self-checks.

Anchor: [Doc 738](../corpus-ref/738-the-source-identifier-as-coordinate-naming-convention-as-substrate-position-encoding-at-the-source-tier.md). Articulated in `CLAUDE.md` (root) "Source-identifier coordinate conventions" section.

## The accumulation

| # | Constraint | Adds | Induces |
|---|---|---|---|
| 0 | (Null) free-form Rust identifier naming | — | nothing encoded; names are documentation, not coordinates |
| 1 | **Prefix encodes JS-observability stratum** — `plain` (user-visible), `__name` (engine-internal sentinel, non-enumerable), `@@name` (well-known Symbol), `__engine_op` (hidden table) | observability bucket as name | property: "stratum is readable from prefix" |
| 2 | **Function suffix encodes invocation surface** — `_via` (Runtime-side, can call back into JS), `abstract_ops::*` (pure-primitive, no Runtime access) | call-graph capability as name | property: "what a function can reach is readable from suffix and module path" |
| 3 | **Property-install helper encodes descriptor shape** — `set_own_frozen` ({w:f, e:f, c:f}), `set_own_internal` ({w:t, e:f, c:t}), `set_own` ({w:t, e:t, c:t}) | descriptor as helper name | property: "the descriptor of an installed property is readable from the helper called" |
| 4 | **Registration helper encodes binding tier** — `register_method` (own property), `register_intrinsic_method` (with arity + non-enumerable defaults), `register_engine_helper` (hidden), `register_global_fn` (globalThis) | binding-tier as helper | property: "where a function is bound is readable from the registration call" |
| 5 | **Module path encodes substrate pillar** — `rusty-js-{ast, parser, bytecode, runtime, jit, shapes, gc, ir, pm, caps}` etc.; `/derived/` segment marks Pin-Art-derived | substrate-pillar as path | property: "what substrate a piece of code lives in is readable from its module path" |
| 6 | **Convention self-checking** — a name whose prefix and install helper disagree (e.g., `__name` installed via `set_own`) is itself a bug-shape signal | grammar-as-validator | property: "naming-convention violations ARE bugs (the convention is its own static-semantics check)" |

The named composition (1+2+3+4+5+6) is the **source-identifier coordinate system**. The induced property is that the identifier name IS a substrate coordinate — reading the name yields stratum + invocation surface + descriptor + binding tier + substrate pillar, plus a self-check at the prefix/helper agreement axis.

Removing constraint 1 (no stratum prefix) means the stratum is recoverable only by reading code; observability bugs (e.g., __engine sentinels exposed as enumerable) get easier to introduce.
Removing constraint 6 (no self-checking) means the convention degrades to documentation; consistency drifts as substrate evolves.

## Tag on the DAG

This is a **source-encoding tier coordinate** — operates below test262 fixtures but above raw text:

```
source/identifier ::
  E-1/encoding ::
  cut/naming-convention-as-coordinate ::
  property/position-recoverable-from-name
```

The pattern's correctness is observable as the absence of audit findings of prefix/helper disagreements; recent locales (CLIF, ESNE, RES) have surfaced such disagreements and closed them as substrate moves (ESNE-EXT 1: hide __X engine sentinels per the convention). Each finding is the convention catching itself.

## Composes-with

- Doc 738 — primary articulation.
- Doc 729 — resolver-instance pattern; the source identifier is the locator within a tier.
- `CLAUDE.md` (root) "Source-identifier coordinate conventions" — operational rendering.
- [`docs/fca-instances/resolver-instance-directive-free-artifact.md`](resolver-instance-directive-free-artifact.md) — the encoding works because tier boundaries are well-defined.

## Falsification

A substrate site where the naming convention fails to encode the position (the name says one thing, the code does another) without flagging as a bug-shape would falsify constraint 6. Empirically rare; the audit findings in trajectories generally surface convention violations as their initial signal — which is the constraint working as designed.
