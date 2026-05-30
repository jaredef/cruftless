# array-strict-throw-discipline — Seed

## Telos

Materialize the engine-DAG coordinate

```
runtime/array-prototype :: E3/intrinsic-object:ecma-262 :: mutating-method/throw-discipline :: TypeError-on-frozen-or-non-writable
```

Array.prototype mutating methods (pop, push, shift, unshift, splice, sort, ...) must throw TypeError when the receiver array is frozen or has non-writable length, per ECMA-262 §10.4.2.1 ArraySetLength step "If newLenDesc.[[Writable]] is false, return false" composed with §23.1.3 Set(O, "length", newLen, Throw=true).

The locale's scope is the Result-thread propagation from the Array-prototype intrinsic methods through `object_set_checked` to `array_set_length_define_property_via` (which is already spec-correct on the throw side). The substrate move is the dispatcher; the abstract op was already done at a prior rung.

## Origin

Founded 2026-05-30 per keeper APPROVED of helmsman proposal
`apparatus/proposals/pending/2026-05-30T200000Z-asta-ext-0-array-frozen-throw/proposal.md`
(Telegram 10614). Driven by findings-ledger Entry 016 (SAMPLE.1) Doc 721 chain-bundle analysis identifying the Array sub-bundle (15 cells) as the largest substrate-shaped sub-axis of the cross-family missing-TypeError-throw pattern.

## Work shape

**Heuristics §IV classification**: D (Runtime Intrinsic Semantics) — Array-prototype throw discipline.

The Doc 721 chain-walk surfaced the swallowing site at `interp.rs:11543` (`let _ = self.array_set_length_define_property_via(...)`) as the highest shared layer. Closure at that layer requires either (a) lifting `object_set_pk` to Result (wide blast radius), or (b) introducing `object_set_checked` (narrow dispatcher per Rule 27 + Rule 21 + Doc 739 cascade-revival pattern; same shape as TAECSF-EXT 0's `typed_array_set_index_checked`). Option (b) selected.

## Apparatus

- **Direct probe**: `/tmp/probe-asta-0.js` (7-cell assertion suite). 7/7 PASS post-rung.
- **Cluster regression instruments**: TAMM ≥87, TAWR ≥71, diff-prod ≥64/48.
- **Canonical sample**: test262-sample post-rung 88.8% (6817-6818 / 7680-7681; n=2 variance ±1 observed — falsifier event for Rule 29 at this instrument; see trajectory.md for the falsifier-driven Rule 2 reactivation).

## Methodology

Per Doc 744 four-tuple + Doc 721 + Rule 17 segmentation: this locale scopes against the Array sub-bundle of SAMPLE.1, NOT the full cross-family 86-cell residual. Sibling sub-bundles (Map/Set frozen-receiver, Promise dispatcher receiver-validation, Object.assign throw-propagation, for-of iterator-protocol + put-const) are separate substrate moves per Rule 17.

## Carve-outs

- **Element-set strict-throw**: not addressed at founding. If a test cell surfaces `Object.freeze(arr); arr[0] = x` expecting TypeError, the substrate move is at the bytecode SetIndex handler (similar to ASTA-EXT 0's length-set; separate rung).
- **Map/Set/WeakMap/WeakSet sibling sub-bundles**: deferred to separate locales per Rule 17.
- **Iterator-protocol TypeError throws** (for-of cluster of 5 cells): deferred to a parser/IR locale.
- **`put-const` destructuring** (for-of cluster of 4 cells): deferred to a parser locale.
- **Promise dispatcher receiver-validation** (Promise cluster of ~14 cells): deferred to a Promise locale.

## Composes-with

- `apparatus/docs/predictive-ruleset.md` Rules 4, 11, 13, 17 (pre-scoping segmentation; this locale scopes against the sub-bundle), 18 (brand-check at registration wrapper), 20 (cross-module reason-shape coherence), 21 (probe-first via existing helper), 22 (axis-discriminator; this locale is the Array sub-axis), 27 (substrate-spec-correctness; the swallowing site is the architectural conflict), 28 (rectification at dispatch tier), 29 (post-land measurement protocol).
- Corpus Doc 721 (chain-walk applied to SAMPLE.1's Array sub-bundle).
- `apparatus/docs/findings-ledger.md` Entry 016 (SAMPLE.1 — this locale is the first substrate-spawn from the SAMPLE.1 chain-bundle decomposition).
- `pilots/typed-array-byte-storage-conformance/` — sibling locale; both use the narrow-dispatcher cascade-revival pattern (TAECSF-EXT 0's `typed_array_set_index_checked` is the template; ASTA's `object_set_checked` mirrors it at the Array-prototype layer).
- `apparatus/arcs/2026-05-28-array-exotic-substrate/` — enrolling arc.

## Resume protocol

Read `trajectory.md` tail. Founding rung is ASTA-EXT 0 (length-set throw discipline); subsequent rungs address element-set strict-throw on frozen Arrays + the sibling Doc-721 sub-bundles.

**Status**: FOUNDED 2026-05-30 by ASTA-EXT 0 (helmsman session, keeper directive Telegram 10614). Founding rung LANDED with probe 7/7 PASS and cluster gates preserved. test262-sample marginal +0.1 PP (88.7% → 88.8% with ±1 PASS variance across n=2; Rule 29 falsifier event recorded). Sibling sub-bundles deferred per seed §Carve-outs.
