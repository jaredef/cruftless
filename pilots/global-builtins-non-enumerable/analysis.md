# global-builtins-non-enumerable — Diff-Prod Analysis

| Fixture | Status | Relevance |
|---|---|---|
| global-constructors | FAIL | Global constructor property descriptors should be {w:t, e:false, c:t} |
| object-statics | PASS | Object static methods work correctly despite descriptor shape |
| prototype-chain | PASS | Prototype chain lookups unaffected by enumerability |

The FAIL on global-constructors is directly explained by this locale: `install_global_this` uses `object_set` which installs all built-in globals as `{w:t, e:t, c:t}` (enumerable), but the spec requires `{w:t, e:false, c:t}` (non-enumerable). Any fixture that checks `Object.getOwnPropertyDescriptor(globalThis, "Map").enumerable === false` will fail. The passing object-statics and prototype-chain fixtures confirm the built-ins themselves function correctly; only their descriptor metadata is wrong.
