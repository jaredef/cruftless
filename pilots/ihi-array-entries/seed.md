# ihi-array-entries — Seed

**Locale tag**: `L.ihi-array-entries`

**Status**: FOUNDED 2026-05-28. Spawned from
`apparatus/locales/CANDIDATES.md` Tier A entry `(c)` after a current
`json_parse_transform` probe preserved the candidate's performance
surface.

## Telos

Extend the closed `interp-hot-intrinsics` substrate from String receiver
entries to Array receiver entries, targeting hot Array prototype method
calls in realistic mixed workloads.

The first anchor is `json_parse_transform`, whose body exercises:

```text
JSON.parse -> Array.prototype.filter -> Array.prototype.map -> JSON.stringify
```

Current N=5 local probe on 2026-05-28:

```text
equality: EQUAL
node median: 100 ms
bun median: 69 ms
cruft median: 1691 ms
cruft/node: 16.91x
cruft/bun: 24.51x
```

## Apparatus

- Parent substrate: `pilots/interp-hot-intrinsics/`.
- Runtime table: `pilots/rusty-js-runtime/derived/src/interp_ic_table.rs`.
- Dispatch site: `pilots/rusty-js-runtime/derived/src/interp.rs`
  `Op::CallMethod` / `Op::CallMethodIcCached`.
- Benchmark anchor:
  `pilots/apparatus/cross-runtime-bench/fixtures/json_parse_transform/main.mjs`.
- Correctness surfaces: per-method Test262 rows for
  `built-ins/Array/prototype/{filter,map,indexOf,includes,slice,push,pop}` plus
  diff-prod and existing Array method fixtures.

## Rule-11 Pre-Spawn Check

- **A1 component A/B**: current single-fixture probe confirms the workload
  remains high gap and uses Array method calls in the transform body. A
  more exact component A/B is the first baseline rung.
- **A2 op-set**: holds. The planned surface is `Op::GetProp` followed by
  `Op::CallMethod`, the same call shape consumed by IHI.
- **A3 value-domain**: holds for real Array objects. The existing
  `IhiReceiverKind::Array` currently maps every `Value::Object`, so the
  first substrate rung must distinguish Array objects from generic objects
  before entries can be correct.
- **A4 locals marshaling**: holds. The IHI entry ABI receives receiver plus
  argument vector; callback-bearing methods (`filter`, `map`) can either
  remain out of first implementation or deliberately re-enter
  `Runtime::call_function` through a widened entry ABI if measurements
  justify it.
- **A5 emission shape**: holds with caveat. `CallMethodIcCached` already
  burns entry indices into bytecode; the companion `GetPropSkipForMethod`
  rewrite is currently String-only and should remain so unless a safe
  Array bail path is implemented.

## First Rungs

1. **IAE-EXT 0**: founding, current `json_parse_transform` probe, and
   substrate inspection.
2. **IAE-EXT 1**: exact baseline inspection. Add a local component probe
   that isolates `filter`, `map`, `push`, and `indexOf/includes` call
   surfaces under cruft. Decide first entry set from measured hotness.
3. **IAE-EXT 2**: array receiver/prototype cache substrate. Add
   Array-specific intrinsic ObjectId cache fields and make IHI's
   override-safety gate resolve through `array_prototype` for Array
   entries, not `string_prototype`.
4. **IAE-EXT 3**: first Array entries. Prefer non-callback entries
   (`push`, `pop`, `indexOf`, `includes`) unless IAE-EXT 1 proves
   callback dispatch dominates enough to justify widening the entry ABI
   for `filter`/`map`.

## Carve-Outs

- Do not merge eval/global declaration work into this locale.
- Do not alter parser, bytecode lowering semantics, or current
  `DefineGlobal` work.
- Do not add broad Array method rewrites before the receiver/prototype
  cache substrate is correct.
- `filter` and `map` are the benchmark-visible methods, but their callback
  shape may require a separate ABI rung rather than a first-entry shortcut.

## Resume Protocol

Read this seed, then `trajectory.md`. Read the tail of
`pilots/interp-hot-intrinsics/trajectory.md` for the closed IHI bytecode
rewrite mechanism. Before implementation, inspect current dirty work with
`git status --short` and avoid colliding with eval/global declaration
instantiation changes.
