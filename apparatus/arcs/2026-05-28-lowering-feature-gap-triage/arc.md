---
arc: 2026-05-28-lowering-feature-gap-triage
trigger: keeper directive 2026-05-28 ("which arc do you want to choose?" -> "do it")
opened: 2026-05-28
closed: IN PROGRESS
close_condition: All current `availability/missing-lowering-feature` reason clusters are triaged into landed substrate, nested locale ownership, existing-locale absorption, or deliberate redirect/defer.
---

# Lowering Feature Gap Triage Arc

## Trigger

After surveying the top-level arcs and candidate queue, the keeper directed the
chosen lowering-feature candidate to proceed under the apparatus methodology.
The selected coordinate is candidate `(w) cruft-lowering-feature-gaps`, rooted
in PCR-EXT 2's `availability/missing-lowering-feature` projection.

## Telos

Partition and close the current bytecode compiler "not yet supported" residue
without founding one broad language-lowering bucket. The arc treats
`compile: ...` reason shapes as evidence of AST-to-bytecode resolver residue
and routes each cluster to the smallest coherent substrate locale.

## Sub-locale roster

| Locale | Role | Status | LOC | Direct yield |
|---|---|---|---:|---:|
| `cruft-lowering-feature-gaps/` | parent baseline + partition locale | FOUNDED 2026-05-28 | pending | pending |
| `cruft-lowering-feature-gaps/super-reference-lowering/` | first child, `super` compile diagnostics | FOUNDED 2026-05-28 | pending | pending |

## Cumulative yield

| Checkpoint | Matrix run | Pool | Closed | Notes |
|---|---|---:|---:|---|
| Arc open | `test262-full-2026-05-27-161641` | 113 | 0 | 7 malformed JSONL rows ignored by tolerant parser; selected rows use `projection == availability/missing-lowering-feature` or `reason` prefix `compile:`. |
| CLFG-EXT 1 | focused exemplar suite | 32 | 0 | Rule-23 baseline: 32/32 still fail with compiler diagnostics; first child should target `super` residue. |
| SRL-EXT 1 | child exemplar suite | 22 | 0 | Child baseline: 22/22 still fail with `super` compiler diagnostics; internal split recorded in child trajectory. |
| SRL-EXT 2 | child exemplar suite | 22 | 3 | Object-literal HomeObject bridge flips the three computed-property object method/accessor `super` rows. |
| SRL-EXT 3 | child exemplar suite | 22 | 5 | Super PutValue base/key ordering flips the two object-method compound/update rows. |
| SRL-EXT 4 | child exemplar suite | 22 | 9 | No-extends SuperProperty fallback flips the four base-class/no-extends rows. |
| SRL-EXT 5 | child exemplar suite | 22 | 13 | Delete SuperReference routing flips the four `delete super` rows. |

## Cross-locale findings

**Finding LFGT.1 (super dominates but does not exhaust the coordinate)**: the
founding matrix pass finds 96/113 rows in `super`-related compile diagnostics:
`super reference outside of a class`, bare `super`, no-extends class use, and
`super(...) outside of a class`. The remaining 17 rows split across for-in
destructuring heads, invalid update targets, and one complex-assignment target.
This requires triage before substrate editing so the `super` chapter does not
silently absorb unrelated lowering tails.

**Finding LFGT.2 (Rule-23 baseline confirms compile-time ownership)**: the
32-path stratified exemplar suite produced `PASS=0 FAIL=32 SKIP=0 NOJSON=0`.
Every row failed with the expected `compile:` diagnostic after stale path
cleanup. The first substrate child should therefore target bytecode compiler
context/lowering rather than parser acceptance or runner policy.

**Finding LFGT.3 (existing-locale collision check routes to child, not sibling)**:
`for-head-this-super-target/` owns a parser-only invalid-LHS carve-out, while
`object-literal-computed-property-semantics` and
`class-lowering-residual-repartition` are broader deferred/audit-first
candidates. The explicit `compile: super...` cluster is narrow enough to found
as a child of CLFG.

**Finding LFGT.4 (super cluster has an internal low-collision entry point)**:
SRL-EXT 1 splits the 22 exemplars into object-literal HomeObject, direct-eval
context capture, delete/bare-super early-error routing, super property
assignment/update, and no-extends runtime behavior. The low-collision first
move is object-literal HomeObject or super property assignment/update; direct
eval should wait behind the active eval-environment arc.

**Finding LFGT.5 (HomeObject bridge is isolated and yield-positive)**:
SRL-EXT 2 closes the object-literal HomeObject subcluster without touching
direct-eval context propagation or broad class residuals. The helper reads the
literal object's live prototype at call time, preserving `Object.setPrototypeOf`
visibility.

**Finding LFGT.6 (PutValue ordering needs base capture before key coercion)**:
SRL-EXT 3 closes the two `super[key]` compound/update rows by capturing the
HomeObject super base before evaluating the computed key. The runtime helper
then performs side-effectful `ToPropertyKey`, preserving the spec order where a
key object's `toString()` can mutate the HomeObject prototype without changing
the already-captured super base.

**Finding LFGT.7 (no-extends SuperProperty is runtime fallback, not syntax
rejection)**: SRL-EXT 4 closes the four base-class/no-extends rows by treating
missing super-base bindings as `Object.prototype` for instance/base-constructor
reads and `Function.prototype` for static reads. The computed-key rows also
confirm that key coercion must remain side-effectful even when the class has no
explicit `extends`.

**Finding LFGT.8 (delete-super is a reference-evaluation branch)**: SRL-EXT 5
closes the four `delete super` rows by lowering delete of SuperProperty as
reference evaluation followed by a required `ReferenceError`, not as a parser
or compiler ban. The uninitialized-this fixture fixes the order constraint:
`PushThis` precedes computed-key evaluation.

## Status

IN PROGRESS. Parent locale founded; Rule-23 baseline completed; first child
`super-reference-lowering/` founded. SRL has closed the object-literal
HomeObject, object-method PutValue ordering, no-extends SuperProperty, and
delete SuperReference subclusters. Pending next move: defer to the
eval-environment arc for direct-eval `super` capture or explicitly join that
arc's current settlement.
