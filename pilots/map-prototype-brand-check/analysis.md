# map-prototype-brand-check — Diff-Prod Analysis

| Fixture | Status | Relevance |
|---|---|---|
| map-set-ops | PASS | Map/WeakMap proto method behavior under brand-check enforcement |
| weakmap-weakset | PASS | WeakMap receiver must be rejected by Map.prototype methods and vice versa |
| error-types | PASS | TypeError thrown on cross-proto brand-check failures |

This locale wraps Map.prototype and WeakMap.prototype methods with brand-check closures so cross-proto calls (e.g., Map.prototype.set.call(new WeakMap())) throw TypeError. All three relevant fixtures pass, consistent with the locale's CLOSED status. The brand-check discipline is invisible to well-typed code but critical for spec compliance on cross-proto misuse.
