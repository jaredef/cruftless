# host-262-shim - Trajectory

## H262S-EXT 0 - spawn + Phase-2 baseline partition (2026-05-29)

**Status**: FOUNDED / PROBE COMPLETE. No substrate source edits.

**Trigger**: Helmsman CAACP `epsua-parallel-4-adjudication-alpha-approved` approved R1 (`instance_id=codex-pop-os-20260529t043820`) for `host-262-shim`: spawn locale, refresh manifest, and partition `$262` failures into named subclusters. Keeper-selected path alpha: cheap additive shim only; `createRealm` separate-track if/when directed.

### Phase 1 - Spawn

- **M**: test262 runner executes tests whose harness expects a `$262` host object.
- **T**: Cruft exposes only the minimal host-hook surface needed by cheap `$262` subclusters; realm construction stays out of scope.
- **I**: additive host-tier installer candidate for `globalThis.$262`; no parser, bytecode, or runtime substrate touched in this probe rung.
- **R**: EPSUA child locale; host-tier shim relation to test262 harness, not a core ECMAScript semantic rewrite.
- **Observability**: T262C interpreted matrix rows plus focused exemplar runs before any EXT 1 substrate.
- **Mouth-gating prerequisite**: helmsman ack of EXT 1 landing plan after this partition.

Rule-11 5-axis pre-spawn check:

- A1 component A/B: host-side runner harness availability, not component-A language semantics.
- A2 op-set: `$262.detachArrayBuffer`, `$262.IsHTMLDDA`, `$262.createRealm`, agent/global/gc hooks as partition labels.
- A3 value-domain: ArrayBuffer/DataView/TypedArray buffers; Annex B undefined-like object; realm/global objects.
- A4 locals-marshaling: no local-slot or closure marshaling expected for cheap host object installation.
- A5 emission-shape: host global property/function installation, probably in `cruftless/src/*`.
- A6 spec-section enumeration: test262 host hooks plus ECMA-262 host-defined realm/buffer operations as exercised by harness tests.

### Phase 2 - Baseline Inspect

Input: `pilots/apparatus/test262-categorize/full-suite/results/test262-full-2026-05-28-123833-p2/interpreted.jsonl`.

Selector: `projection == "runner-harness/$262-or-host-hook"` OR failure reason contains `$262 is not defined`.

Result:

```
total=361
detachArrayBuffer              229
other-$262                      58
isHTMLDDA-emulates-undefined    34
global-hook-or-global-state     24
createRealm-realm               15
gc                               1
```

Sample rows:

- `built-ins/ArrayBuffer/prototype/byteLength/detached-buffer.js` -> `$262 is not defined`.
- `built-ins/DataView/detached-buffer.js` -> `$262 is not defined`.
- `annexB/built-ins/Object/is/emulates-undefined.js` -> `$262 is not defined`.
- `annexB/language/global-code/block-decl-global-existing-global-init.js` -> `$262 is not defined`.
- `built-ins/Proxy/revocable/tco-fn-realm.js` -> `$262 is not defined`.

Reading:

- Cheap-shim subcluster is real. `detachArrayBuffer` alone is 229 rows in this matrix, but it may require existing buffer detach substrate to be present rather than only object availability.
- `isHTMLDDA-emulates-undefined` is smaller (34 rows) and still a cheap-shim candidate, but may need special object truthiness/equality behavior rather than plain object installation.
- `createRealm-realm` is present (15 rows) but no longer dominates the current matrix. It remains out of scope per EPSUA-EXT 0.5 and helmsman adjudication.
- `other-$262` is heterogeneous and should not be included in EXT 1 without exemplar inspection.

### Phase 3 - Pin-Art probe if duplicated

Duplication is present across many fixture directories, but the duplicated error text (`$262 is not defined`) over-aggregates distinct hook requirements. H262S uses the hook/API subcluster as the Pin-Art discriminator before proposing substrate.

### Phase 4 - Land / Revert

No substrate landed in this rung. Locale artifacts only.

### Substrate

None.

### Yield

```
current runner-harness/$262-or-host-hook rows: 361
cheap-candidate partition:
  detachArrayBuffer: 229
  isHTMLDDA-emulates-undefined: 34
deferred:
  createRealm-realm: 15
```

### Gates

Read-only probe; no build or runtime gate required. Manifest refresh required by spawn discipline.

### Tag

`host-hook-partition`

### Finding

**Finding H262S.1**: `$262 is not defined` is an over-aggregated failure reason. The actionable unit is the host-hook subcluster (`detachArrayBuffer`, `IsHTMLDDA`, `createRealm`, etc.), not the matrix projection row as a whole. This matches EPSUA's standing refinement that matrix-cell projections over-count unless segmented by actual upstream cause.

### Phase 6 - Deferral Emission

No ledger entry yet. `createRealm` remains deferred inside EPSUA by prior EPSUA-EXT 0.5 unless keeper requests a separate realm-substrate locale.

### Status

Awaiting helmsman ack for a follow-up EXT 1 landing plan. Proposed next plan: inspect representative `detachArrayBuffer` and `IsHTMLDDA` exemplars directly, then choose one cheap additive host hook for first substrate.
