# typed-array-resizable-buffer-indexed-access - Seed

## Telos

Close the alphabet-top surfaced by the alphabetically first unresolved
test262 sample prefix after the `FORA-EXT 1` pull: `Array.prototype.at`.

Per the corpus reading of Doc 714 / 715 / 721, `Array.prototype.at` is only the
first witness. The substrate alphabet element is:

```text
TypedArray indexed access over ResizableArrayBuffer-backed views
```

## Apparatus

- Fresh sample:
  `/home/jaredef/Developer/cruftless-sidecar/results/test262-sample-2026-05-25/results.jsonl`
- First-witness fixtures:
  - `/home/jaredef/test262/test/built-ins/Array/prototype/at/coerced-index-resize.js`
  - `/home/jaredef/test262/test/built-ins/Array/prototype/at/typed-array-resizable-buffer.js`
- Shared harness helper:
  `/home/jaredef/test262/harness/resizableArrayBufferUtils.js`
- Current runtime implementation:
  `pilots/rusty-js-runtime/derived/src/intrinsics.rs`
  `pilots/rusty-js-runtime/derived/src/interp.rs`

## Corpus Reading

Doc 714 names per-layer edge-kind alphabets. Doc 715 reframes alphabet
stability as substrate-leaf-set stability at a DAG cut. Doc 721 gives the
operational protocol: enumerate the gated population, walk call chains upward,
locate the highest shared layer, then dispatch only if the move is substrate
shaped.

Applied here:

- Symptom bin: `Array.prototype.at` has 2 failures.
- Highest shared layer: TypedArray/RAB indexed access substrate.
- The `.at` method is not the alphabet top; it is the first observable leaf.

## Current Substrate Finding

The runtime currently represents TypedArrays and ArrayBuffers as ordinary
objects with shape properties:

- `length`
- `byteLength`
- numeric own properties
- non-enumerable `__kind`

The TypedArray constructors copy or zero-initialize numeric properties directly
onto the view object. `ArrayBuffer` is installed through the same stub
constructor loop. There is no current first-class backing buffer object, no
view record carrying `[[ViewedArrayBuffer]]`, `[[ByteOffset]]`, or
fixed-length vs length-tracking state, and no `ArrayBuffer.prototype.resize`
method.

## Methodology

1. Preserve `Array.prototype.at` as the first-witness coordinate.
2. Audit adjacent RAB-backed sample failures before implementing:
   `includes`, `indexOf`, `slice`, `filter`, and TypedArray prototype methods.
3. Decide whether EXT 1 should introduce a small RAB backing-store record or
   defer to a broader ArrayBuffer/TypedArray representation locale.
4. If implementing, land at the shared buffer/view substrate, not inside
   `array_proto_at_via`.

## Predictions

- **Pred-tarbia.1**: adding only `ArrayBuffer.prototype.resize` is insufficient
  to close both `.at` fixtures, because fixed-length views must become
  out-of-bounds after shrink.
- **Pred-tarbia.2**: a shared buffer/view model should flip `.at` and at least
  one adjacent RAB-indexed Array/TypedArray failure.
- **Pred-tarbia.3**: fixture-only `.at` special casing would fail Doc 721 Step
  3, because it patches a leaf symptom rather than the highest shared layer.
- **Pred-tarbia.4**: implementing the shared substrate will expose more honest
  failures in grow/shrink zero-fill, BigInt typed arrays, or detached-buffer
  edges.

## Carve-Outs

- SharedArrayBuffer, Atomics, detach, and growable SharedArrayBuffer are out of
  scope unless the first EXT proves they are unavoidable for these fixtures.
- Full numeric element coercion for every TypedArray kind is not required for
  EXT 1 unless the gated fixtures exercise it.
- Ordinary Array `.at` behavior is out of scope; it is already past the first
  witness and not the shared alphabet top.

## Resume Protocol

Read this seed, then `trajectory.md`. Re-run the two first-witness fixtures.
Then enumerate adjacent RAB failures from the fresh sidecar result set before
choosing an implementation scope.
