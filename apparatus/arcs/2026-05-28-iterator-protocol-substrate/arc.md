---
arc: 2026-05-28-iterator-protocol-substrate
trigger: Plan agent's back-fit analysis 2026-05-28 (keeper directive Telegram 10158); empirical: IPBR spawn 2026-05-24 + LPA-EXT 8/9 compiler gap analysis (diff-prod mechanism gaps #2, #3)
opened: 2026-05-28
closed: IN PROGRESS
close_condition: every IteratorClose emission site exists per ECMA §7.4.9; for-of / for-in / destructure / yield* exercising the protocol passes its exemplar suite; abrupt-completion paths route IteratorRecord correctly; AGFA generator-suspension lazy emit per the spec.
---

# Iterator Protocol Substrate Arc

## Trigger

Plan agent's back-fit (2026-05-28, per keeper Telegram 10158) identified nine top-level locales sharing the iterable-protocol mouth-terminus: Iterable receiver + ForIn/Of head AST → IteratorRecord-respecting bytecode + abrupt-IteratorClose. Empirically anchored in IPBR (iter-protocol-bytecode-rewrite, 2026-05-24, EXT 0-2 closure) + LPA-EXT 8/9 compiler gap analysis identifying diff-prod mechanism gaps #2 (iterator-close-on-abrupt) and #3 (iterator-protocol-error-propagation) as a single coherent multi-locale program.

## Telos

Subsume the nine iterator-protocol locales under one arc with explicit (M, T, I, R) per Doc 744. The arc-tier mouth is "Iterable receiver + ForIn/Of head AST + destructure/yield*/Promise.all consumer site"; the arc-tier terminus is "IteratorRecord-respecting bytecode emission at every consuming site plus IteratorClose on abrupt-completion per §7.4.9". The arc-tier interior is the chain GetIterator → IteratorStep → IteratorValue → IteratorClose, plus the abrupt-completion routing across destructure / for-of / yield* / Promise.all. The arc-tier relations: DAG ↓ runtime (iterator intrinsics); lattice with the array-exotic arc (shared LengthOfArrayLike + species); alphabet-exchange ↑ bytecode emitter (the IPEP/ICOA emit-site rewrite).

## Sub-locale roster

| Locale | Role in arc | Status pre-arc |
|---|---|---|
| `iterator-close-emission-sites` | enumeration of mandatory IteratorClose sites | LANDED |
| `iterator-close-on-abrupt` | abrupt-completion routing | LANDED |
| `iterator-protocol-error-propagation` | error propagation through the protocol | LANDED |
| `iter-protocol-bytecode-rewrite` (IPBR) | bytecode emit rewrite for the protocol | LANDED at IPBR-EXT 2 (canonical example) |
| `for-of-destructuring-assignment-semantics` (FODAS) | for-of + destructure consumer site | LANDED |
| `for-in-prototype-chain` | for-in's proto-chain walk + iterator shape | LANDED |
| `generator-coroutine-suspension` | generator suspend / resume primitive | OPEN |
| `async-generator-and-for-await-lowering` (AGFA) | async-iterator continuation | OPEN |
| `iterable-primitive-tobject` | iterable → ArrayLike conversion | LANDED |

Cross-listed (lattice with other arcs):
- `iter-protocol-bytecode-rewrite` (also in `2026-05-28-engine-hot-path-amortization` proposed arc; alphabet-exchange).

## Cumulative yield

Pre-arc baseline: IPBR canonical close at EXT 2 (1 implementation round per rule 13 prospective application; +14 cluster-exemplar PASS). FODAS canonical close with 40-test for-of-dstr iterator-protocol cluster.

Per-arc future measurement: tracked at arc fold per the substrate-shaped-work discipline Phase 5 (Doc 744 §V.2 + Doc 745 candidate §II.5 + §III.2).

## Cross-arc relations

- **Lattice with `2026-05-28-array-exotic-substrate`**: shared LengthOfArrayLike + species at iterator/destructure boundaries. Meet at TypedArray iteration interior.
- **DAG ↓ `rusty-js-runtime`**: iterator intrinsics + IteratorRecord storage.
- **Alphabet-exchange ↑ `2026-05-28-parser-early-error-conformance`**: for-head productions parse into the iterator-protocol mouth shape.
- **Alphabet-exchange with `2026-05-28-engine-hot-path-amortization`**: IPBR locale is a meet point.

## Cross-locale findings

To be promoted as the arc operates. Initial sketch:

**Finding IPS.1 (pending)**: IteratorClose emission discipline applies symmetrically across every for-of / for-in / destructure / yield* / Promise.all consumer site. The §7.4.9 protocol's reach is broader than its spec-section locality; cross-site audit (per Rule 6 surface-completeness) is the standing apparatus for the arc.

## Status

IN PROGRESS — scaffolded 2026-05-28 per keeper directive 10158. Arc is the canonical example of an arc whose roster spans both already-closed locales (IPBR, FODAS, ICOA) and OPEN locales (generator-coroutine-suspension, AGFA). Per the Plan agent's recommendation, this is the second new arc to be back-fit (single protocol, multiple closures already, dense cross-locale findings).
