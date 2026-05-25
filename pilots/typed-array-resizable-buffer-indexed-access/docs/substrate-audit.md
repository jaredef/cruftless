# RAB / TypedArray Substrate Audit

## Summary

The first witness is `Array.prototype.at`, but the highest shared layer is the
TypedArray / ResizableArrayBuffer substrate. The current runtime cannot close
this by patching `.at`, because there is no backing buffer/view relation for
`.at` to ask.

## Current Runtime Shape

TypedArray and ArrayBuffer constructors are installed together in
`install_typed_array_stubs`.

Each constructed instance is an ordinary object with:

- `length`
- `byteLength`
- numeric own properties
- non-enumerable `__kind`
- prototype = shared TypedArray prototype

Construction cases:

- `new Uint8Array(N)` stores zero values directly as numeric properties on the
  view object.
- `new Uint8Array(arrayLike)` copies numeric properties from the source object
  to the view object.
- `new ArrayBuffer(N, { maxByteLength })` currently flows through the same
  stub path, so it is not a first-class backing store.

There is no representation for:

- backing store identity
- current backing byte length
- maximum backing byte length
- view byte offset
- fixed-length vs length-tracking view
- out-of-bounds view state
- resize propagation

## Why `.at` Is Not The Fix Site

`Array.prototype.at` does the expected generic-array algorithm:

1. convert receiver to object
2. read `length`
3. coerce index
4. read indexed property

For a true TypedArray view, steps 2 and 4 should observe the view's current
typed-array semantics. In the current runtime, they only observe ordinary
object properties. After a backing buffer shrinks, the fixed-length view's
stored `"length"` and indexed properties remain stale, so `.at(-1)` can still
return `3` when the spec expects `undefined`.

Adding a special case to `.at` would only patch one leaf. The adjacent gated
population already includes `filter`, `find`, `forEach`, `includes`, `indexOf`,
`join`, `map`, `reduce`, `slice`, `sort`, `Object.defineProperty`,
`Object.freeze`, `for-in`, and `for-of`. Each needs the same view/buffer truth.

## Minimal Substrate Shape

An EXT 1 implementation should introduce a shared backing object or internal
record with at least:

```text
ArrayBuffer record:
  current_byte_length
  max_byte_length
  bytes/vector of Value-or-byte storage
  resizable flag

TypedArray view record:
  viewed_buffer
  element_kind
  bytes_per_element
  byte_offset
  fixed_length: Option<usize>
```

Derived operations:

- `view_current_length(view)`:
  - fixed-length view: if `byte_offset + fixed_length * bpe >
    buffer.current_byte_length`, the view is out-of-bounds; length is 0 for
    generic array-like reads.
  - length-tracking view: length is
    `(buffer.current_byte_length - byte_offset) / bpe` when in-bounds.
- `view_get_index(view, i)`:
  - out-of-bounds or `i >= view_current_length` -> `undefined`
  - otherwise read backing storage at `byte_offset + i * bpe`
- `ArrayBuffer.prototype.resize(newByteLength)`:
  - reject non-resizable buffers
  - reject `newByteLength > max_byte_length`
  - shrink/grow current length
  - grown memory is zero-filled

## Insertion Points

Likely runtime insertion points:

- `install_typed_array_stubs`: split `ArrayBuffer` from TypedArray
  constructors and allocate records.
- `try_array_length` or a new `length_of_array_like` branch: recognize
  TypedArray views and compute dynamic length.
- `read_property` / integer-indexed read path: recognize TypedArray views and
  compute dynamic element access.
- TypedArray prototype methods and generic Array methods benefit only if they
  route through these shared reads.

## Rejected Shortcut

Do not add:

```text
if receiver is TypedArray and method is Array.prototype.at { ... }
```

That violates the Doc 721 chain-walk result: `.at` is the first symptom leaf,
not the alphabet top. It would leave the adjacent gated population untouched
and would likely require repeating the same patch in every Array method.

## EXT 1 Gate

Proceed to code only if the change can establish the buffer/view record and
route both length and indexed reads through it. Otherwise spawn a broader
ArrayBuffer/TypedArray representation locale and leave this locale as the
first-witness coordinate.
