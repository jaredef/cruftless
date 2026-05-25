# set-like-validation — Seed

## Telos

ECMA-262 §24.2.1.2 GetSetRecord: Set.prototype.{union, intersection, difference, symmetricDifference, isSubsetOf, isSupersetOf, isDisjointFrom} require their argument to be a Set-like Object — an Object with `size` (Number), `has` (callable), and `keys` (callable). Plain Arrays must throw TypeError (Array is iterable via Symbol.iterator but lacks size/has/keys).

cruft accepts any iterable (uses Symbol.iterator via `collect_iterable`), passing Arrays silently. test262 cluster: `Set/prototype/{op}/array-throws.js` + `called-with-object.js` (14 tests).

Identified via top-failure-reason audit (Expected a TypeError to be thrown but no exception was thrown at all).

## Apparatus

- `pilots/rusty-js-runtime/derived/src/interp.rs::set_proto_{union,intersection,difference,symmetric_difference,is_subset_of,is_superset_of,is_disjoint_from}_via` (lines 1350-1474, 7 sites).

## Methodology

Add helper `Self::validate_set_like(self, other) -> Result<(), RuntimeError>` that throws TypeError if:
- other is not Object, OR
- other has no `size` property (or size is NaN), OR  
- other has no callable `has`, OR
- other has no callable `keys`.

Call at the top of each Set op, before `collect_iterable`. (collect_iterable still works for real Sets — they have Symbol.iterator. Custom set-like with non-iterable keys() is a separate gap.)

## Carve-outs

- Custom set-like with keys() that returns a non-Symbol.iterator iterator: deferred — would require calling other.keys() and iterating the result, not the default Symbol.iterator path.
- GetSetRecord's size coercion side-effect ordering: defer to future rung.

## Resume protocol

Read `trajectory.md` tail.
