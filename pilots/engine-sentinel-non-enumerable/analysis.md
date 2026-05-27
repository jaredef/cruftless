# engine-sentinel-non-enumerable — Diff-Prod Analysis

| Fixture | Status | Relevance |
|---|---|---|
| map-set-ops | PASS | Map/Set operations pass; but __map_data/__set_data sentinels leak through Object.keys/for-in |
| weakmap-weakset | PASS | WeakMap/WeakSet pass; but __is_weakmap/__is_weakset sentinels are enumerable |
| date-ops | PASS | Date operations pass; but __date_ms sentinel leaks through enumeration |
| property-key-order | FAIL | Property enumeration order may expose sentinel keys that should be hidden (mechanism gap #6) |

The PASS on map-set-ops, weakmap-weakset, and date-ops indicates that these fixtures do not probe Object.keys or for-in enumeration on Map/Set/WeakMap/WeakSet/Date instances -- if they did, the leaked __X sentinels would produce extra keys that differ from Bun's output. The FAIL on property-key-order may partially overlap: OrdinaryOwnPropertyKeys ordering + sentinel visibility could cause enumeration-order diffs. The locale's fix (installing sentinels via a non-enumerable descriptor) would also prevent JSON.stringify and structured-clone from picking up engine-internal state.
