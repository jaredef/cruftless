# array-literal-elision-length - Seed

## Telos

Array literal elisions must create holes while still advancing the array length.
`[,,,]` has length 3, no own property `"0"`, and
`[,,,].includes(undefined)` returns true because `includes` performs `Get` at
each index and treats absent elements as `undefined`.

This locale deliberately sits away from the active parser early-error arc. The
syntax is already parsed into `ArrayElement::Elision`; the gap is at the
bytecode/runtime construction seam where the compiler's `NewArray` length hint
was ignored.

## Apparatus

- `pilots/rusty-js-parser/derived/src/expr.rs` already emits
  `ArrayElement::Elision` for comma elisions.
- `pilots/rusty-js-bytecode/derived/src/compiler.rs` already emits
  `Op::NewArray` with `elements.len()` for non-spread array literals.
- `pilots/rusty-js-runtime/derived/src/interp.rs::Op::NewArray` decoded the
  hint but ignored it, so elision-only arrays had length 0.
- `pilots/rusty-js-runtime/derived/tests/run_golden.rs` carries the local
  regression probe.

## Methodology

1. Confirm `[,,,].length` currently returns 0 and
   `[,,,].includes(undefined)` returns false.
2. Route `Op::NewArray`'s decoded hint into the freshly allocated array's
   `length` when the hint is nonzero.
3. Preserve hole-ness: do not create indexed properties for elisions.
4. Verify `[,,,].length === 3`, `0 in [,,,] === false`, and
   `[,,,].includes(undefined) === true`.
5. Run the focused runtime test, then the relevant Test262
   `Array/prototype/includes/sparse.js` probe.

## Carve-outs

- Array literals with spread use incremental push/extend helpers; their elision
  semantics may need a separate audit because elisions inside spread-containing
  literals currently push explicit `undefined`.
- `Array.prototype.indexOf` sparse-hole behavior is intentionally different:
  it checks property presence and skips holes.
- Resizable ArrayBuffer / TypedArray includes failures are separate typed-array
  substrate work.

## Resume Protocol

Read `trajectory.md` tail, then inspect `Op::NewArray` and array literal
compilation before changing parser code. Parser elision recognition is not the
first suspect for this locale.
