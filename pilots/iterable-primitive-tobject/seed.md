# iterable-primitive-tobject — Seed

## Telos

ECMA-262 §7.3.20 GetIterator(obj, hint) treats the input via property access semantics: `obj[Symbol.iterator]` performs ToObject(obj) implicitly when `obj` is a primitive. Strings, numeric wrappers, etc. that have @@iterator on their prototype iterate correctly via `for-of`, `Array.from`, and spread per spec.

cruft's `collect_iterable(rt, src)` short-circuits to `Ok(Vec::new())` whenever `src` is not `Value::Object`. Result: `[..."abc"]` returns `[]`, `new Set([..."xyz"])` is empty, any Set op called with a string argument silently returns empty. Probe surfaced via ESNE-EXT 3's verification round.

`for-of` on a string + `Array.from("abc")` both work because they use different paths that ToObject-wrap the receiver first; only the `collect_iterable` path is broken.

## Apparatus

- `pilots/rusty-js-runtime/derived/src/intrinsics.rs::collect_iterable` (~line 5680). Single source of the bug.
- 17 callers across `interp.rs` + `intrinsics.rs` — all benefit transparently.
- `Runtime::to_object` already implements the spec ToObject for String/Number/Boolean/BigInt primitives.
- `new String("abc")[Symbol.iterator]()` works correctly (verified probe).

## Methodology

Replace the non-Object short-circuit with a `to_object` call. Undefined/Null still throw per spec. The result Object's `@@iterator` lookup + dispatch is the existing code path.

## Carve-outs

- BigInt: spec mandates ToObject(bigint) → BigInt wrapper which has no @@iterator (per §22.3 — BigInt is non-iterable). Behavior: to_object returns a BigInt wrapper; method lookup finds no @@iterator; existing code already errors on "iterator is not an object" when method call returns non-Object. Spec wants TypeError; either way TypeError thrown. Acceptable.
- Number: same shape — Number wrappers have no @@iterator. for-of on numbers errors per spec. Same TypeError path.
- Boolean: same.
- Symbol: ToObject(symbol) → Symbol wrapper, no @@iterator. Same.

## Composes-with

- ESNE-EXT 3 verification round (the trigger; surfaced the gap during iterator leak testing).
- Standing Rule 13 (revert-then-deeper-layer-closure): the surface callers don't need 17 fixes; a single source-layer change closes all.

## Resume protocol

Read `trajectory.md` tail.
