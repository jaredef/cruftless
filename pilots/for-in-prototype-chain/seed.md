# for-in-prototype-chain — Seed

## Telos

ECMA-262 §14.7.5.6 EnumerateObjectProperties: `for...in` walks the prototype chain, yielding own enumerable string keys at each level, with shadowing (a key first seen as own (enumerable or not) on a closer object is not yielded later from the chain).

cruft's `__for_in_keys` helper currently delegates to `Object.keys` — own enumerable only, no proto walk. Documented as "Spec deviation" at `compiler.rs:1837`. test262 `verifyEnumerable !== true` cluster (6 tests in sample; many more for the "Inherited property is enumerable" pattern in the full corpus) hits this.

Identified by GOPD-EXT 1 probe pass — same `for-in` test also failed in the earlier accessor-shadow audit.

## Apparatus

- `pilots/rusty-js-runtime/derived/src/intrinsics.rs::__for_in_keys` (line 802).
- `pilots/rusty-js-runtime/derived/src/interp.rs::ordinary_own_enumerable_string_keys` (per-level helper available, line 5985).

## Methodology

Replace the delegating body with a proto-chain walk:
1. Collect own enumerable string keys (filtering integer-indexed first per §7.3.22), skipping engine-internal `__X` sentinels.
2. For each prototype, repeat — but skip names already seen (own or non-enumerable inherited that shadowed).
3. Return as Array.

## Carve-outs

- Symbol-keyed properties: excluded (spec).
- Insertion order across proto levels: per spec, ordered chunks per chain level.
- Object.prototype's enumerable surface should be empty (so for-in on a plain `{}` yields nothing) — verify post-fix.
- Engine sentinel `__primitive__` etc. should not leak through; currently filtered at object_keys via the `__` prefix convention; preserve.

## Resume protocol

Read `trajectory.md` tail.
