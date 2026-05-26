# for-in-initializer-annex-b — Trajectory

## FII-EXT 0 — FOUNDING (2026-05-26)

Spawned per keeper directive (Telegram 9865) from the missing-syntax-feature disambiguation map, alongside HLCL. Keeper said "begin work on the first" — first is HLCL; FII spawned but execution deferred.

Pool: 7 fixtures from `annexB/language/statements/for-in/` + sibling dirs, all emitting `parse: expected Semicolon` because cruft's for-head parser treats `var X = init in obj` as a classic for-statement and bails at the missing `;`.

Baseline: 0/7 PASS. Verified probe:
```js
(function() {
  var effects = 0;
  for (var a = ++effects in {});
  assert.sameValue(effects, 1);
})();
```
cruft: `parse: expected Semicolon @byte5994`.

## Status

FII-EXT 0 FOUNDED. FII-EXT 1 (parser carve-out at parse_for_statement) deferred per keeper sequencing. Sibling pattern (HDSB-EXT 1) demonstrates the R13 prospective shape — expect ~15-20 LOC, 1-round closure.
