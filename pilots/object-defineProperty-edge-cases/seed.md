# object-defineProperty-edge-cases - Seed

**Locale tag**: `L.object-defineProperty-edge-cases` (top-level EPSUA child candidate; Phase 0/2 founded by substrate resolver R4).

**Status**: Founded for read-only Phase 0/2 probe. No substrate source edits have landed in this locale yet.

**Workstream**: ECMA-262 `Object.defineProperty` edge cases, especially `ToPropertyDescriptor`, `ToPropertyKey`, `OrdinaryDefineOwnProperty`, and `ValidateAndApplyPropertyDescriptor` behavior as exercised by the test262 Object.defineProperty cluster.

**Trigger**: Helmsman CAACP `object-defineProperty-phase-0-phase-2-r4` directed R4 to spawn the locale, refresh the manifest, baseline-inspect the current matrix cluster, run C4 reason-coherence, and propose a Phase-3 move shape without landing substrate.

**Composes with**:
- [EPSUA](../../apparatus/arcs/2026-05-25-ecmascript-parity-shared-upstream/arc.md) - parent ECMAScript parity arc.
- [T262C](../apparatus/test262-categorize/seed.md) - full-suite categorizer and interpreted-matrix source.
- ECMA-262 §20.1.2.4 `Object.defineProperty`.
- ECMA-262 §10.1.6.3 `ValidateAndApplyPropertyDescriptor`.
- ECMA-262 `ToPropertyDescriptor`, `ToPropertyKey`, `OrdinaryDefineOwnProperty`, Array exotic `[[DefineOwnProperty]]`, and Arguments exotic mapped-parameter behavior.

## I. Telos

Close the Object.defineProperty edge-case cluster by replacing the current ad hoc body-level handling with property-key-aware descriptor validation and define-own-property behavior that matches ECMA-262 invariants.

The target is not just availability of `Object.defineProperty`; it is correct behavior across descriptor coercion, accessor/data discrimination, non-configurable descriptor updates, non-extensible targets, symbol keys, array length/index transitions, prototype-chain shadowing, and arguments-object mapped parameters.

## II. Apparatus + Methodology

ODP-EXT 0 is a Phase 0/2 probe:

1. Select the current Object.defineProperty rows from the interpreted test262 full-suite matrix.
2. Re-run the selected rows against current R4 Cruft where practical.
3. Sample representative failures and bucket them by mechanism.
4. Apply C4 reason-coherence: if one bucket is at least 40%, propose a Phase-3 substrate move shape; otherwise yield with a pivot.

Candidate substrate after approval:

1. `pilots/rusty-js-runtime/derived/src/intrinsics.rs::install_object_stubs` for the public `Object.defineProperty` entry and Proxy prelude.
2. `pilots/rusty-js-runtime/derived/src/generated.rs::object_define_property` for the generated abstract-operation call boundary.
3. `pilots/rusty-js-runtime/derived/src/interp.rs::object_define_property_via` and adjacent object/array/arguments helpers for the actual `DefineOwnProperty` and `ValidateAndApplyPropertyDescriptor` behavior.

## III. Carve-outs

- Proxy defineProperty trap invariants are adjacent unless the Object.defineProperty rows directly require them.
- TypedArray resizable-buffer out-of-bounds behavior remains adjacent to the typed-array / resizable-buffer substrate unless a later probe shows it is blocked only by shared descriptor validation.
- Full Reflect.defineProperty boolean-return semantics are adjacent unless repaired by extracting a shared define-own-property helper.
- Broad Object static method parity outside defineProperty and getOwnPropertyDescriptor is out of scope.

## IV. Baseline Snapshot

The requested 2026-05-29 post-EPSUA 21-cell artifact was not present in the R4 checkout or configured sidecar when ODP-EXT 0 ran. The latest available full-suite interpreted matrix was:

`pilots/apparatus/test262-categorize/full-suite/results/test262-full-2026-05-28-123833-p2/interpreted.jsonl`

Rows selected by `surface == "Object.defineProperty"`: 54.

Current R4 focused rerun over those 54 files emitted 41 JSON results: 4 PASS, 37 FAIL, and 13 runner/no-output rows. The no-output rows hit runtime panics in `object_define_property_via` paths, including `interp.rs` unwraps around the descriptor body.

## V. Proposed Phase-3 Shape

If approved, start with a shared property-key-aware define-own-property/ValidateAndApplyPropertyDescriptor closure rather than per-test patches:

1. ODP-EXT 1: make descriptor lookup/storage consistently `PropertyKey`-aware, extract or consolidate ValidateAndApplyPropertyDescriptor checks, and fix non-configurable same-descriptor and illegal-change outcomes.
2. ODP-EXT 2: repair Array exotic define-own-property length/index boundaries (`4294967295`, length non-writable, shrink/grow).
3. ODP-EXT 3: repair arguments mapped-parameter defineProperty behavior and prototype-chain shadow creation.
4. ODP-EXT 4: decide whether remaining TypedArray/resizable rows belong here or transfer to the typed-array locale; then align Reflect.defineProperty if a shared helper exists.

Estimated rung count: 3 to 4 substrate rungs after this probe.
