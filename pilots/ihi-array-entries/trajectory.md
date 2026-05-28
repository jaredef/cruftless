# ihi-array-entries — Trajectory

## IAE-EXT 0 — FOUNDING AND CURRENT PERFORMANCE PROBE (2026-05-28)

Founded as the Array receiver extension of the closed
`interp-hot-intrinsics` substrate.

Current substrate inspection:

- `interp_ic_table.rs` already declares `IhiReceiverKind::Array`, but no
  table entries use it.
- `receiver_kind_of(Value::Object(_))` currently returns `Array`, which is
  too broad for actual Array entries. IAE must refine this through runtime
  object kind/prototype checks before enabling entries.
- `Runtime::ihi_get_cached` / `ihi_set_cached` only carry String intrinsic
  ObjectId fields.
- `Op::CallMethod` override-safety cache population currently resolves
  method ObjectIds via `string_prototype`; Array entries need
  `array_prototype`.
- `GetPropSkipForMethod` companion rewrite is explicitly String-only and
  should remain excluded from first Array rounds unless a receiver-safe
  bail path is designed.

Current benchmark probe:

```text
fixture: pilots/apparatus/cross-runtime-bench/fixtures/json_parse_transform/main.mjs
runs: 5
equality: EQUAL
node median: 100 ms
bun median: 69 ms
cruft median: 1691 ms
cruft/node: 16.91x
cruft/bun: 24.51x
```

The shared CRB script did not run directly on this host because macOS bash
3.2 does not support the script's `declare -A` associative array syntax.
The probe used the same fixture and explicit node/bun/cruft binaries.

**Finding IAE.1 (Array IHI needs prototype-specific cache ownership)**:
adding Array entries is not a table-only move. The IHI dispatcher currently
populates every entry cache through `string_prototype`, which is correct for
the closed String table and wrong for Array entries. The first implementation
rung must separate the cache owner by receiver kind or cached-field family.

**Status**: founded. Next rung is IAE-EXT 1 baseline inspection, with a
component probe that distinguishes callback-bearing Array methods
(`filter`, `map`) from non-callback entries (`push`, `pop`, `indexOf`,
`includes`, `slice`).

## IAE-EXT 1 — LOCAL COMPONENT PROBE INSTALLED (2026-05-28)

Added `fixtures/component-probe.mjs`, a local additive probe over the
`json_parse_transform` body shape:

- baseline `filter(...).map(...)`;
- manual filter + method map;
- method filter + manual map;
- manual filter + manual map.

The probe keeps JSON.parse and JSON.stringify in every variant so the
measured deltas isolate only Array method dispatch/callback contribution
inside the existing CRB fixture envelope.

**Status**: probe installed. Next run it under node/bun/cruft and record the
relative deltas before choosing whether IAE-EXT 2 should target callback
method ABI (`filter`/`map`) or simpler non-callback entries first.

Probe result (`IAE_RUNS=3 IAE_ITER=250`):

```text
node:
baseline_filter_map        median_ms=18.758
manual_filter_map_method   median_ms=18.015
filter_method_manual_map   median_ms=18.167
manual_filter_manual_map   median_ms=18.021

bun:
baseline_filter_map        median_ms=14.137
manual_filter_map_method   median_ms=13.357
filter_method_manual_map   median_ms=13.851
manual_filter_manual_map   median_ms=13.694

cruft:
baseline_filter_map        median_ms=774.196
manual_filter_map_method   median_ms=782.231
filter_method_manual_map   median_ms=786.263
manual_filter_manual_map   median_ms=770.786
```

**Finding IAE.2 (json_parse_transform is not callback-method-dispatch
dominated)**: replacing `filter` and/or `map` with hand-written loops does
not materially improve cruft on the current probe; all cruft variants are
within about 2% at N=3. This falsifies callback-bearing `filter`/`map` as
the first IHI-Array target for this fixture. The first substrate move should
therefore be the lower-risk Array receiver/prototype cache substrate plus
non-callback entries, or a different benchmark anchor whose Array method
dispatch component is actually dominant.

**Status update**: IAE remains viable as an IHI substrate extension, but
`json_parse_transform` no longer justifies widening the IHI entry ABI for
callback-bearing methods as the first implementation move.
