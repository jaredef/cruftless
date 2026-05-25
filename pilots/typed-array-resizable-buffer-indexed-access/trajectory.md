# typed-array-resizable-buffer-indexed-access - Trajectory

## TARBIA-EXT 0 - corpus-informed founding (2026-05-25)

**Trigger**: keeper correction to read the docs/corpus around "alphabet" before
founding the next locale.

**Pull state**:

```text
b5af5ea5 FORA-EXT 1: for-of RHS parses AssignmentExpression, not Expression
```

**Fresh sample baseline**:

```text
PASS: 6264
FAIL: 1060
SKIP: 384
Runnable pass rate: 85.5% (6264 / 7324)
```

**Corpus read**:

- Doc 714: edge-kind alphabets are per `(stratum, layer)`; substrate work
  should lift the layer whose constraint question is actually failing.
- Doc 715: alphabet stability is substrate leaf-set stability at a DAG cut.
- Doc 721: the top of a substrate-widening alphabet is found by walking the
  gated population to the highest shared layer.
- Doc 731: downstream complexity grows when upstream alphabets collapse needed
  discriminations.

**Corrected interpretation**:

The alphabetically first unresolved prefix is:

```text
Array.prototype.at
```

But both leaves are ResizableArrayBuffer / TypedArray fixtures:

```text
built-ins/Array/prototype/at/coerced-index-resize.js
built-ins/Array/prototype/at/typed-array-resizable-buffer.js
```

So `.at` is the first witness, not the alphabet top.

**Pre-scoping findings**:

1. `coerced-index-resize.js` fails first because `rab.resize` is undefined.
2. `typed-array-resizable-buffer.js` fails because a fixed-length TypedArray
   view still returns `3` after its backing buffer shrinks below the view's
   required byte range.
3. Runtime TypedArray stubs are ordinary objects with `length`, `byteLength`,
   numeric own properties, and `__kind`; the constructor loop also installs
   `ArrayBuffer` through this shape-stub path.
4. There is no shared backing store, view offset, fixed-length vs
   length-tracking state, or resize propagation.

**Adjacent gated population**:

Scanning the fresh sidecar `results.jsonl` for RAB / resize / ArrayBuffer /
TypedArray signatures surfaces ~40 leaves, including:

- `Array.prototype.{at,filter,find,forEach,includes,indexOf,join,map,reduce,slice,sort}`
- `Object.defineProperty` over TypedArray views
- `Object.freeze` over TypedArray views
- `language/statements/for-in/resizable-buffer.js`
- `language/statements/for-of/typedarray-backed-by-resizable-buffer*.js`

This confirms the Doc 721 Step 3 reading: the 2-test `.at` prefix is a first
witness, while the highest shared layer is the RAB/TypedArray indexed-access
substrate.

**Rule 11 axis check**:

- (A1) component A/B: not local Array `.at`; shared RAB/TypedArray substrate.
- (A2) op-set coverage: `ArrayBuffer.prototype.resize`, `LengthOfArrayLike`
  on TypedArray views, integer-indexed property reads.
- (A3) value-domain: byte length, max byte length, bytes-per-element,
  byteOffset, fixed-length view, length-tracking view, out-of-bounds view,
  grown zero-fill.
- (A4) locals-marshaling: N/A.
- (A5) emission-shape: N/A.
- (A6) spec sections: `Array.prototype.at`, ResizableArrayBuffer,
  TypedArray constructors, integer-indexed exotic object access.

**Status**: TARBIA-EXT 0 founded. EXT 1 should begin with adjacent RAB failure
enumeration and only then decide whether to implement a small backing-store
record or spawn a broader ArrayBuffer/TypedArray substrate locale.

## TARBIA-EXT 0.5 - substrate audit (2026-05-25)

**Audit artefact**: `docs/substrate-audit.md`.

**Finding TARBIA.1**: the current runtime has no central RAB/TypedArray truth
to patch. TypedArrays and ArrayBuffers are ordinary object stubs with `length`,
`byteLength`, indexed own properties, and `__kind`; `ArrayBuffer` itself is
installed through the same constructor loop. There is no backing store, view
record, byte offset, fixed-length / length-tracking distinction, or resize
propagation.

**Finding TARBIA.2**: `Array.prototype.at` is a correct first witness but an
incorrect fix site. The method reads `length` and indexed properties generically.
The stale answer comes from the receiver's object-stub state after resize, not
from the `.at` algorithm.

**Decision**: EXT 1 must either introduce a shared buffer/view record and route
length + integer-indexed reads through it, or defer to a broader
ArrayBuffer/TypedArray representation locale. Leaf-patching `.at` is rejected.

**First-witness baseline probes**:

```text
coerced-index-resize.js
FAIL: callee is not callable: undefined (method='resize')

typed-array-resizable-buffer.js
FAIL: Expected SameValue(undefined, 3)
```

## TARBIA-EXT 1 - shared RAB/view record and integer-indexed access (2026-05-25)

**Move**: implement the Doc 721 alphabet top, not the `.at` leaf. The runtime
now has side-table records for `ArrayBuffer` backing stores and TypedArray
views. `ArrayBuffer` is split out of the old TypedArray constructor loop and
gets `ArrayBuffer.prototype.resize`.

**Runtime substrate**:

- `Runtime::array_buffers`: current byte length, max byte length, zero-filled
  storage.
- `Runtime::typed_array_views`: viewed buffer, byte offset, fixed-length vs
  length-tracking state, bytes-per-element.
- `object_get`: dynamic `length`, `byteLength`, and numeric indexed reads for
  TypedArray views.
- `object_set_pk`: numeric writes on TypedArray views update backing storage.
- `has_property`: integer-indexed presence follows the current view length.
- TypedArray constructors recognize `new Uint8Array(rab, byteOffset, length)`
  and allocate a view instead of copying ordinary object properties.

This is intentionally a minimal substrate model: values are stored at the
element's byte-start offset rather than byte-encoded across all element bytes.
That is enough for the current RAB indexed-access alphabet; byte-accurate
coercion remains a later TypedArray-numeric-semantics axis.

**Verification**:

```text
cargo check -p rusty-js-runtime
PASS

built-ins/Array/prototype/at/coerced-index-resize.js
PASS

built-ins/Array/prototype/at/typed-array-resizable-buffer.js
PASS
```

**Adjacent resizable-buffer slice**:

From the 33 fresh sample failures containing `resizable-buffer` or
`typed-array-resizable-buffer`, the targeted re-run now reports:

```text
PASS: 28
FAIL: 5
```

Newly passing groups:

- `Array.prototype.at`: 2/2
- `Array.prototype.{filter,find,forEach,includes,indexOf,join,map,reduce,slice,sort}`
- `language/statements/for-of/typedarray-backed-by-resizable-buffer` grow and
  ordinary iteration cases

Residuals:

- `Object.defineProperty/typedarray-backed-by-resizable-buffer.js`: descriptor
  writes still bypass the new backing-store write path.
- `Object.freeze/typedarray-backed-by-resizable-buffer.js`: integer-indexed
  exotic invariants for non-empty views are not modeled.
- `language/statements/for-in/resizable-buffer.js`: enumeration does not yet
  surface TypedArray integer indices.
- `for-of/*shrink*`: current iterator shrink behavior produces Test262Error
  where the fixture expects TypeError.

**Prediction booking**:

- Pred-tarbia.1 corroborated: `resize` alone was not the fix; fixed-length
  out-of-bounds behavior was required.
- Pred-tarbia.2 strongly corroborated: the shared model flips both `.at`
  fixtures plus 26 adjacent RAB-indexed leaves.
- Pred-tarbia.3 corroborated by implementation shape: no `.at` special case.
- Pred-tarbia.4 partially corroborated: residuals are now honest descriptor,
  enumeration, and iterator-shrink semantics rather than absent substrate.

**Status**: EXT 1 lands the first substrate rung. EXT 2 should take the
descriptor-write residual first (`Object.defineProperty`), because it is the
same backing-store write path expressed through a different object operation.

## TARBIA-EXT 2 - residual closure over descriptor, freeze, enumeration, iterator shrink (2026-05-25)

**Trigger**: keeper directive to continue with residuals after EXT 1 left five
failures in the RAB-filtered sample slice.

**Move**: close the remaining residual channels without moving back to leaf
patching.

- `Object.defineProperty` on integer-indexed TypedArray views now writes
  through the backing store and throws `TypeError` for out-of-bounds indices or
  accessor descriptors.
- `Object.freeze` now rejects TypedArray views through the exotic-object guard
  expected by the sample fixture.
- `ordinary_own_enumerable_string_keys` surfaces virtual TypedArray integer
  indices, so `for-in` sees `0,1,2` on RAB-backed views.
- TypedArray iterators now throw `TypeError` when a resize makes the source
  view out-of-bounds during iteration, both for `.values()`/`.keys()`/`.entries()`
  and `@@iterator`.

**Verification**:

```text
cargo check -p rusty-js-runtime
PASS

cargo build -p cruftless --bin cruft --release
PASS
```

Residual fixtures:

```text
built-ins/Object/defineProperty/typedarray-backed-by-resizable-buffer.js PASS
built-ins/Object/freeze/typedarray-backed-by-resizable-buffer.js PASS
language/statements/for-in/resizable-buffer.js PASS
language/statements/for-of/typedarray-backed-by-resizable-buffer-shrink-mid-iteration.js PASS
language/statements/for-of/typedarray-backed-by-resizable-buffer-shrink-to-zero-mid-iteration.js PASS
```

Full RAB-filtered sidecar slice:

```text
PASS: 33
FAIL: 0
```

**Status**: EXT 2 closes the current RAB/TypedArray indexed-access alphabet
slice from the 2026-05-25 sample. Further TypedArray work should move to a
fresh failure population rather than continuing this now-green slice.
