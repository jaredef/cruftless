# array-create-data-property-discipline — Resume Vector / Seed

**Locale tag**: `L.array-create-data-property-discipline` (top-level)

**Status as of 2026-05-25**: **WORKSTREAM FOUNDED (ACDPD-EXT 0)**. Spawned per keeper directive after ASCD-EXT 1 surfaced this as the sibling sub-locale (Finding ASCD.2 + Finding ASCD.3).

**Workstream**: ECMA-262 §7.3.6 CreateDataPropertyOrThrow + §10.1.6 [[DefineOwnProperty]]. Array.prototype methods that populate a result array (slice, splice, map, filter, concat, etc.) per spec use CreateDataPropertyOrThrow at each element-write — which calls [[DefineOwnProperty]] with `{value, writable:true, enumerable:true, configurable:true}`. This OVERRIDES any pre-existing descriptor (e.g., non-writable property from a custom species constructor).

Cruft's element-writes use `object_set`, which respects the existing descriptor's `writable` flag (silently no-ops if writable=false in sloppy / throws in strict). Tests where a custom species constructor pre-populates the output array with non-writable properties expect the new write to OVERRIDE; cruft preserves the old descriptor, breaking the test.

This is the substrate dual of ASCD: ASCD wired the constructor-selection path; ACDPD fixes the per-element-write path. Together they enable spec-correct slice/splice/map/filter species behavior.

**Pre-scoping probe**: ~14 target-array-non-extensible / non-writable / non-configurable-property tests across map/filter/slice/splice/flat/flatMap (Finding ASCD.3 decomposition).

**Author**: 2026-05-25 session.
**Parent**: none (top-level); sibling to ASCD.
**Composes with**:
- ECMA-262 §7.3.6 CreateDataPropertyOrThrow; §10.1.6 [[DefineOwnProperty]]
- [ASCD trajectory](../array-species-create-discipline/trajectory.md) — Finding ASCD.2 / ASCD.3 (the surfacing event + sub-sub-cluster decomposition)
- [Doc 740](../../docs/740-multi-tier-cascade-revival-when-the-hot-path-traverses-multiple-tiers-closing-one-tier-alone-is-insufficient.md) — multi-tier closure default; ACDPD enables ASCD's deferred slice/splice species-wiring as a follow-on combined move

## I. Telos

Replace plain `object_set` element-writes in Array.prototype.{slice, splice, map, filter, concat, flat, flatMap, fill, copyWithin} with `create_data_property_or_throw` semantics that use [[DefineOwnProperty]] to OVERRIDE existing descriptors. Closes the ~14 target-array-non-writable / non-extensible / non-configurable sub-sub-cluster.

### I.1 First-cut scope (ACDPD-EXT 1)

Two-tier R per Doc 740 default:
- T_1: `create_data_property_or_throw` actually uses [[DefineOwnProperty]] (replace its current `object_set` call with a `define_own_property_overriding` call or inline the descriptor install).
- T_2: All affected Array.prototype methods' per-element write sites call `create_data_property_or_throw` (not `object_set`).

If after T_1+T_2 the slice/splice species-wiring (ASCD carve-out) can be re-enabled cleanly, do so as part of this round.

### I.2 Constraints

```
C1. cargo build + diff-prod 42/42 + canonical fuzz acc=-932188103 hold.
C2. Existing Array-method tests (540+ passing) preserve.
C3. The new define-path correctly throws on non-extensible TARGET
    where the property doesn't already exist AND extensible=false.
C4. Per Rule 14 mirror: the per-element define-write semantics is
    spec-correct (overrides existing descriptors by spec); the
    Rule 14 risk is on tests that depend on the buggy preserve
    behavior (likely none exist among currently-passing).
```

### I.3 Falsifiers

**Pred-acdpd.1**: ~10 of the ~14 target-array-non-writable / non-extensible tests pass post-fix.
**Pred-acdpd.2**: zero PASS→FAIL on previously-passing Array.* tests (540).
**Pred-acdpd.3**: ASCD carve-out for slice/splice species-wiring CAN be lifted cleanly post-ACDPD; if attempted, slice/splice species-create tests close (closing the ~3 remaining IsConstructor variants for slice/splice).
**Pred-acdpd.4 (DISCIPLINE)**: closes in ≤2 implementation rounds.

## II. Apparatus + Methodology

- Edit at `pilots/rusty-js-runtime/derived/src/interp.rs` — `create_data_property_or_throw` + per-method element-write sites.
- Verify via target-array-non-writable / non-extensible / non-configurable fixtures.
- Adjacent regression check on Array.prototype.* previously-passing.

## III. Carve-outs

- TypedArray methods unchanged.
- Map/Set/etc. element-writes unchanged.
- Strict-mode throws-on-failed-define vs sloppy silent — follow spec's CreateDataPropertyOrThrow which throws regardless.

## IV. Standing artefacts

- `pilots/array-create-data-property-discipline/seed.md`, `trajectory.md`
- Edit at runtime substrate (~30-50 LOC)

## V. Resume protocol

Read seed + ASCD trajectory tail (the surfacing event). The fix is one helper update + N call-site replacements. Verify via target-array exemplar suite + adjacent Array regression check. If T_1+T_2 lands cleanly, also re-attempt ASCD's slice/splice species-wiring carve-out (Pred-acdpd.3).
