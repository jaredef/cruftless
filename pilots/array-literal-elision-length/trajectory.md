# array-literal-elision-length - Trajectory

## ALEL-EXT 1 - route NewArray length hint into array length (2026-05-25)

**Trigger**: avoiding collision with the active parser early-error arc while
continuing the Test262 parity chase. Test262
`built-ins/Array/prototype/includes/sparse.js` failed:

```text
[ , , , ].includes(undefined) Expected SameValue(false, true)
```

Local probe confirmed the deeper shape:

```text
[,,,].length                         // 0, should be 3
0 in [,,,]                           // false, correct hole-ness
[,,,].includes(undefined)            // false, should be true
Array.prototype.includes.call({length:4}, undefined) // true
```

So `Array.prototype.includes` already had the right array-like missing-property
semantics. The bug was array literal construction: pure elisions produced an
array with no indexed properties, but length 0.

**Substrate landed**:

- `pilots/rusty-js-runtime/derived/src/interp.rs`
  - `Op::NewArray` now uses its decoded bytecode length hint and sets
    `length` on the fresh array when the hint is nonzero.
  - This preserves holes: no indexed properties are created for elisions.
- `pilots/rusty-js-runtime/derived/tests/run_golden.rs`
  - Added `array_literal_elision_preserves_length_and_holes`.

**Findings**

**Finding ALEL.1 (elision semantics live at NewArray, not parser)**: the parser
already represented elisions, and the compiler already counted them in the
`NewArray` hint. The runtime was dropping that hint. This is a good example of
the conformance chase picking the lowest resolver seam that already has the
needed fact rather than adding a parser-side workaround.

**Finding ALEL.2 (holes are not explicit undefined)**: the fix sets array
length without installing numeric properties. This keeps `0 in [,,,]` false
while making `includes(undefined)` true, matching the spec difference between
`Get`-based `includes` and presence-checking methods like `indexOf`.

**Verification**

Focused runtime test:

```text
cargo test -p rusty-js-runtime array_literal_elision_preserves_length_and_holes --release -- --nocapture
1 passed
```

Release-binary probe:

```text
[,,,].length              -> 3
0 in [,,,]                -> false
[,,,].includes(undefined) -> true
```

Single Test262 probe:

```text
T262_TEST_PATH=/home/jaredef/test262/test/built-ins/Array/prototype/includes/sparse.js \
T262_HARNESS_DIR=/home/jaredef/test262/harness \
cargo run -p cruftless --bin cruft --release -- legacy/host-rquickjs/tests/test262/runner.mjs
{"path":"/home/jaredef/test262/test/built-ins/Array/prototype/includes/sparse.js","status":"PASS","reason":""}
```

**Next gates**

1. Broader adjacent Array includes/search sample, especially
   `includes/get-prop.js` and `indexOf` sparse cases, to separate Get-vs-Has
   behavior from array-literal length.
2. Audit spread-containing elision literals separately.

**Status**: ALEL-EXT 1 CLOSED.
