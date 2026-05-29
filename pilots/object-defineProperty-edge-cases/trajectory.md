# object-defineProperty-edge-cases - Trajectory

## ODP-EXT 0 - spawn + Phase-2 baseline inspect (2026-05-29)

**Status**: FOUNDED / PROBE COMPLETE. No substrate source edits.

**Trigger**: Helmsman CAACP `object-defineProperty-phase-0-phase-2-r4` directed substrate resolver R4 (`instance_id=codex-substrate-resolver-4`) to spawn `pilots/object-defineProperty-edge-cases`, refresh the locale manifest, inspect the Object.defineProperty matrix cluster, apply C4 reason-coherence, and propose the Phase-3 substrate move shape.

### Phase 1 - Spawn

- **M**: test262 Object.defineProperty rows exercise descriptor conversion and target object define-own-property semantics.
- **T**: Cruft matches ECMA-262 `Object.defineProperty` edge cases across data/accessor descriptors, array and arguments exotics, symbol keys, non-extensible/non-configurable invariants, and inherited descriptor interactions.
- **I**: runtime object-definition substrate, currently reached through `intrinsics.rs::install_object_stubs`, generated `object_define_property`, and `interp.rs::object_define_property_via`.
- **R**: EPSUA child locale over ECMA-262 built-in object semantics.
- **Observability**: interpreted test262 full-suite matrix plus focused R4 rerun of Object.defineProperty rows.
- **Mouth-gating prerequisite**: helmsman ack before any Phase-3 substrate landing.

Rule-11 5-axis pre-spawn check:

- A1 component A/B: component-A built-in call enters component-B object internal methods.
- A2 op-set: `ToPropertyKey`, `ToPropertyDescriptor`, `DefineOwnProperty`, `ValidateAndApplyPropertyDescriptor`, ArraySetLength, arguments exotic mapping.
- A3 value-domain: ordinary objects, arrays, arguments objects, typed arrays, symbol keys, data descriptors, accessor descriptors, generic descriptors, non-extensible/non-configurable targets.
- A4 locals-marshaling: no function-local slot marshaling; descriptor objects are runtime heap objects read via ordinary property access.
- A5 emission-shape: runtime helper extraction/refactor plus focused semantic fixes in `interp.rs`; Object static registration stays as entry routing.
- A6 spec-section enumeration: ECMA-262 §20.1.2.4 and §10.1.6.3 are the load-bearing spec coordinates.

### Phase 2 - Baseline Inspect

Input requested by helmsman: 2026-05-29 post-EPSUA matrix, Object.defineProperty 21-cell cluster.

Artifact status: no 2026-05-29 matrix artifact with the cited 21-cell cluster was present in the R4 checkout or configured sidecar during this probe. ODP-EXT 0 therefore used the latest available full-suite interpreted matrix and a current focused R4 rerun as the baseline evidence:

`pilots/apparatus/test262-categorize/full-suite/results/test262-full-2026-05-28-123833-p2/interpreted.jsonl`

Selector: `surface == "Object.defineProperty"`.

Matrix result:

```
total=54
descriptor-shape/property-semantics       43
abrupt-completion/throw-missing            4
availability/missing-global-or-binding     4
availability/missing-method-or-intrinsic   3
```

Failure-mode result:

```
runner/no-output              14
failure/other                 14
err:TypeError                 12
assertion/expected-mismatch   10
err:ReferenceError-like        4
```

Current R4 focused rerun over the 54 selected files:

```
PASS=4
FAIL=37
NO_JSON_OUTPUT_OR_PANIC=13
```

The rerun surfaced runtime panics in `pilots/rusty-js-runtime/derived/src/interp.rs::object_define_property_via`, including unwraps around the descriptor-body paths. This confirms that the `runner/no-output` bucket is not only categorizer residue; it still maps to a real current execution hazard.

### Sample Failure Partition

Representative rows sampled from the current rerun and source inspection:

| Row | Mechanism | Current reading |
|---|---|---|
| `15.2.3.6-4-217.js` | ValidateAndApplyPropertyDescriptor | Same non-configurable non-writable data redefinition throws when it should be accepted. |
| `15.2.3.6-4-218.js` | ValidateAndApplyPropertyDescriptor | Changing the value of non-configurable non-writable data property fails to throw. |
| `15.2.3.6-4-254.js` | Accessor descriptor discriminator | Same accessor descriptor with explicit `set: undefined` throws when it should be accepted. |
| `15.2.3.6-4-257.js` | Accessor descriptor discriminator | Same accessor descriptor with explicit `get: undefined` throws when it should be accepted. |
| `15.2.3.6-4-184.js` | Array exotic length/index | Key `4294967295` is treated as length-affecting; expected length remains `0`. |
| `15.2.3.6-4-188.js` | Array exotic non-writable length | Defining index equal to non-writable length should throw TypeError; current result misses the throw. |
| `15.2.3.6-4-193.js` | Array exotic index update | Existing array index update preserves the old value instead of applying the descriptor value. |
| `15.2.3.6-4-289-1.js` | Arguments exotic mapping | Defining a mapped arguments index does not disconnect/update parameter mapping as expected. |
| `15.2.3.6-4-410.js` | Prototype-chain descriptor lookup | Own defineProperty over inherited non-writable prototype data property does not shadow correctly. |
| `symbol-data-property-writable.js` | Symbol-key vs string-key | Symbol-key descriptor is not reflected by getOwnPropertyDescriptor; path uses string-key lookups. |
| `typedarray-backed-by-resizable-buffer.js` | TypedArray/resizable adjacent | Resizable-buffer out-of-bounds semantics are present but probably belong to typed-array substrate. |

Mechanism segmentation:

| Mechanism bucket | Rows / evidence | Reading |
|---|---:|---|
| Core descriptor-shape / property semantics | 43/54 matrix rows | Dominant bucket; includes ValidateAndApplyPropertyDescriptor, data/accessor discrimination, and descriptor field preservation. |
| Abrupt-completion / throw-missing | 4/54 matrix rows | Subset of the same semantic area where illegal descriptor transitions should reject. |
| Availability residue | 7/54 matrix rows | Older matrix rows with missing binding/method signatures; not the substrate center. |
| Array exotic length/index | 8 current exemplar failures | Large enough for a follow-up rung, but still under define-own-property semantics. |
| Symbol-key vs string-key | 4 current exemplar failures | Strong evidence that current ODP code mixes `PropertyKey` and string-only lookup/storage. |
| Arguments exotic mapping | 2 current exemplar failures | Requires mapped-parameter defineProperty behavior after the core descriptor path is stable. |
| Prototype-chain descriptor lookup/shadow | 3 to 4 current exemplar failures | Own-property creation must not be blocked by inherited non-writable descriptors. |
| TypedArray/resizable | 1 current exemplar failure | Adjacent; defer unless shared defineProperty helper naturally covers it. |

### C4 Reason-Coherence

PASS.

The dominant bucket is `descriptor-shape/property-semantics`: 43/54 rows, 79.6%. This exceeds the C4 40% threshold. The cluster is coherent enough to proceed to a Phase-3 substrate design without pivoting.

If helmsman intended the cited 21-cell post-EPSUA matrix specifically, that artifact was unavailable in this worktree/sidecar. The available evidence still supports the same move shape: the Object.defineProperty failures converge on property-key-aware descriptor validation and define-own-property semantics.

### Current Implementation Cross-Reference

- `pilots/rusty-js-runtime/derived/src/intrinsics.rs::install_object_stubs` registers `Object.defineProperty` and performs Proxy defineProperty trap prelude checks before dispatch.
- `pilots/rusty-js-runtime/derived/src/generated.rs::object_define_property` routes ECMA-262 §20.1.2.4 to runtime execution.
- `pilots/rusty-js-runtime/derived/src/interp.rs::object_define_property_via` contains the current descriptor conversion, array handling, typed-array handling, and ValidateAndApply-like checks.

Observed implementation risk:

- Several paths derive `key_pk` but then query/update through a string-only `key`, which explains the symbol-key failures and risks ordinary object descriptor mismatches.
- Non-configurable validation appears asymmetric: some legal same-descriptor updates throw, while some illegal value/writable changes fail to throw.
- Array length/index handling needs a dedicated follow-up around canonical array-index boundaries and non-writable length.
- Prototype inherited non-writable data properties appear to interfere with defineProperty own-property creation.

### Proposed Phase-3 Substrate Move Shape

Start with the shared semantic center rather than per-row patches:

1. **ODP-EXT 1**: Introduce or consolidate property-key-aware own descriptor lookup/storage and a ValidateAndApplyPropertyDescriptor helper used by `object_define_property_via`. Fix SameValue checks, accessor/data field-presence checks, and throw/no-throw outcomes for non-configurable descriptors. This should also close the symbol-key reflection failures.
2. **ODP-EXT 2**: Repair Array exotic define-own-property behavior for length/index boundaries, including `4294967295`, shrink/grow, non-writable length, and value application to existing indices.
3. **ODP-EXT 3**: Repair arguments exotic mapped-parameter defineProperty behavior and prototype-chain own shadow creation.
4. **ODP-EXT 4**: Re-evaluate remaining TypedArray/resizable and Reflect.defineProperty rows; land locally if only shared helper fallout remains, otherwise transfer/defer to the typed-array or Reflect locale.

Estimated rung count: 3 to 4 substrate rungs.

### Gates

Read-only probe plus locale spawn; no runtime substrate gate required. Locale manifest refresh required before commit.

### Tag

`object-defineProperty-baseline-segmentation`

### Finding

**Finding ODP.1**: The Object.defineProperty cluster is not primarily an availability gap. The actionable substrate center is a property-key-aware `ValidateAndApplyPropertyDescriptor` / define-own-property closure. Array exotic length/index, arguments mapping, and prototype shadowing are follow-up specializations of the same internal-method boundary.

## ODP-EXT 1 - property-key-aware ValidateAndApply helper (2026-05-29)

**Status**: LANDED. Phase-3 rung 1, approved by Helmsman CAACP `odp-ext-1-validate-and-apply-r4`.

### Move

`pilots/rusty-js-runtime/derived/src/interp.rs::object_define_property_via` now routes ordinary own-property descriptor lookup and storage through a `PropertyKey`-aware `ValidateAndApplyPropertyDescriptor` helper. The helper implements the §10.1.6.3 center for ordinary data/accessor descriptors:

- absent target property plus non-extensible target returns false to the Object.defineProperty throw path;
- configurable and enumerable changes are rejected for non-configurable properties;
- data/accessor conversion is rejected for non-configurable properties;
- non-configurable non-writable data properties use `SameValue` for value changes and reject writable promotion;
- accessor getter/setter updates preserve field presence, including explicit `undefined`.

The rung also makes `Object.getOwnPropertyDescriptor` use `ToPropertyKey` so Symbol-keyed properties defined by Object.defineProperty reflect through the same `PropertyKey` bucket. Computed strict writes now check Symbol-keyed non-writable data descriptors before routing to `object_set_pk`, closing the strict Symbol exemplar without adding a broader assignment substrate.

Out-of-scope residuals from ODP-EXT 0 remain out of scope here: array exotic length/index, arguments mapped parameters, prototype-shadow own creation, typed-array/resizable, and Reflect.defineProperty boolean-return alignment.

### Gates

Build:

```
cargo build --release --bin cruft -p cruftless
PASS
```

Targeted exemplars:

```
15.2.3.6-4-217.js                                  PASS
15.2.3.6-4-218.js                                  PASS
15.2.3.6-4-254.js                                  PASS
15.2.3.6-4-257.js                                  PASS
symbol-data-property-configurable.js               PASS
symbol-data-property-default-non-strict.js         PASS
symbol-data-property-default-strict.js             PASS
symbol-data-property-writable.js                   PASS
```

43-row descriptor-shape/property-semantics bucket selected from `test262-full-2026-05-28-123833-p2/interpreted.jsonl`:

```
expected=43
outputs=42
PASS=26
FAIL=16
NO_JSON_OUTPUT_OR_PANIC=1
```

The 43 rows were matrix-classified failures at ODP-EXT 0, so this is a +26 PASS gain against the descriptor-shape bucket.

Full 54-row `surface == "Object.defineProperty"` sweep:

```
expected=54
outputs=53
PASS=33
FAIL=20
NO_JSON_OUTPUT_OR_PANIC=1
```

ODP-EXT 0 current rerun baseline for the same 54 surface was `PASS=4 / FAIL=37 / NO_JSON_OUTPUT_OR_PANIC=13`; ODP-EXT 1 therefore raises the focused surface by +29 PASS and removes 12 no-output/panic rows.

### Residuals

Remaining descriptor-bucket failures match the ODP-EXT 0 roadmap:

- array exotic length/index boundary and value application: `4-184`, `4-185`, `4-186`, `4-193`, `4-275`;
- arguments mapped-parameter defineProperty behavior: `4-289-1`, `4-301-1`;
- arguments/index descriptor writability rows: `4-292-1`, `4-293-2`, `4-293-3`, `4-294-1`, `4-295-1`, `4-296-1`;
- prototype-chain own shadow creation: `4-410`, `4-415`, `4-625gs`;
- one remaining no-output row: `4-116`.

Full-surface residuals additionally include ODP-EXT 2 array non-writable length throws (`4-188`, `4-189`, `coerced-P-shrink`) and ODP-EXT 4 typed-array/resizable behavior.

### Tag

`validate-and-apply-property-key`

### Finding

**Finding ODP.2**: Symbol-key correctness required both write-side and reflection-side property-key discipline. Fixing only `Object.defineProperty` storage left `Object.getOwnPropertyDescriptor(obj, sym)` invisible and strict computed writes to non-writable Symbol data properties silent; the coherent substrate boundary is `ToPropertyKey` through define, reflect, and computed strict-write enforcement.
