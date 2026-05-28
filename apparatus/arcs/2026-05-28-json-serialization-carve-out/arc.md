---
arc: 2026-05-28-json-serialization-carve-out
trigger: keeper directive 2026-05-28 ("choose an arc" -> JSON serialization carve-out)
opened: 2026-05-28
closed: IN PROGRESS
---

# JSON Serialization Carve-Out Arc

## Telos

Separate remaining JSON failures into substrate coordinates after the parse
surface closed. `JSON.stringify` is not one mechanism: traversal, replacer
PropertyList, replacer-function holder semantics, gap/space, wrapper
substitution, BigInt/toJSON ordering, proxy/realm availability, and string
escaping are distinct rows.

## Active Locale

`pilots/json-stringify-semantics/`

## Yield

| Checkpoint | Result | Notes |
|---|---:|---|
| Baseline | 35 PASS / 24 FAIL / 7 NORESULT | local stringify slice before JSS work |
| JSS-EXT 1 | 40 PASS / 25 FAIL / 1 NORESULT | circular stack detection |
| JSS-EXT 2 | 44 PASS / 21 FAIL / 0 NOJSON | spec `[[Get]]` traversal |
| JSS-EXT 3 | 47 PASS / 18 FAIL / 0 NOJSON | proxy internal-method continuation |
| JSS-EXT 4 | 51 PASS / 14 FAIL / 1 NOJSON | replacer PropertyList construction |
| JSS-EXT 5 | 54 PASS / 11 FAIL / 1 NOJSON | replacer-function holder/root-wrapper and accessor/shape ordering |

## Carve-Outs

- `replacer-array-proxy-revoked-realm.js` fails before JSON semantics because
  `OProxy.revocable` is absent. Coordinate: proxy/realm availability.
- `space-*` rows are gap/indentation and wrapper-space coercion.
- BigInt rows are cross-realm availability plus toJSON/BigInt ordering.
- wrapper substitution rows (`value-number-object.js`, `value-string-object.js`)
  remain separate from replacer wrapper PropertyList construction.
- lone surrogate escaping remains a string-quote coordinate.
