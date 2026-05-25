# array-length-setter-truncation — Seed

## Telos

Route `arr.length = N` (assignment via OrdinarySet) through ECMA-262 §10.4.2.1 ArraySetLength so that decreasing length truncates the backing storage (deletes index entries ≥ N). Currently only `Object.defineProperty(arr, "length", {value:N})` routes through array_set_length; the assignment path silently stores the new value without truncating.

## Apparatus

- `pilots/rusty-js-runtime/derived/src/interp.rs` — `Runtime::object_set_pk` (length-write entry for assignment).
- `pilots/rusty-js-runtime/derived/src/generated.rs` — `array_set_length` (the §10.4.2.1 algorithm, already present and tested under defineProperty path).
- `scripts/test262-sample/results/test262-sample-2026-05-25/` — failure cluster ranks 1 + 8-12 (Array.prototype.{forEach,filter,map,reduce,indexOf} with no-feature-tag; tests of shape `15.4.4.X-7-b-NN` which mutate `arr.length` inside a getter callback and assert the post-truncation index is not visited).
- diff-prod gate: 42/42 must hold.

## Methodology

1. Minimum probe: `var a=[0,1,2,"last"]; a.length=3; a.hasOwnProperty("3")` — currently `true`, should be `false`.
2. In `object_set_pk`, when key=="length" and `internal_kind == Array`, construct a transient `{value: N}` descriptor and dispatch `array_set_length`. Silently ignore errors at this entry (matches the existing object_set_pk semantics of silent failure on non-writable; throwing-on-set is a deferred concern).
3. Verify probe passes; verify exemplar (forEach 15.4.4.18-7-b-14, -7-b-15, etc., plus filter / map / reduce sibling tests).
4. Run random-300 adjacent regression (Array.prototype.* surfaces, for-of, defineProperty).

## Carve-outs

- Strict-mode throwing on `arr.length = badValue` (out-of-range / non-uint32) — currently object_set_pk has no throw channel; this would require widening the signature, deferred to ALST-EXT 2 candidate.
- Stuck-non-configurable-element truncation TypeError propagation — same widening dependency, deferred.
- Array length descriptor `writable: false` enforcement on assignment — array_set_length already throws; once routed through, the silent-ignore at object_set_pk swallows it. Sufficient for parity (silent in sloppy mode is the spec behavior for OrdinarySet-with-non-writable).

## Composes-with

- Doc 581 (Pin-Art); Doc 729 (resolver-instance pattern, §10.4.2.1 already at the runtime resolver); Doc 730 (vertical recurrence — same ArraySetLength routine, two entry points).
- Predictive ruleset rule 13 (revert-then-deeper-layer): array_set_length is the deeper-layer closure; the assignment path was the shallow path that needed to be promoted.

## Resume protocol

Read `trajectory.md` tail.
