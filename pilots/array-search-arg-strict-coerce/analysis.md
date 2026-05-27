# array-search-arg-strict-coerce — Diff-Prod Analysis

| Fixture | Status | Relevance |
|---|---|---|
| array-methods | PASS | Core array search methods (indexOf, includes) pass for normal numeric/string args |
| symbol-toprimitive | FAIL | Symbol-to-number coercion should throw TypeError; this mechanism gap (#1 ToPrimitive hint dispatch) is the same shape as the locale's bug |
| error-throws | PASS | General TypeError throwing works; the gap is specifically in to_number's Symbol path |

The PASS on array-methods confirms that at/indexOf/lastIndexOf/includes work for typical usage with numeric indices. The locale's bug only manifests when Symbol values are passed as index arguments (should throw TypeError per ToNumber(Symbol), currently returns NaN silently). The FAIL on symbol-toprimitive confirms mechanism gap #1 (ToPrimitive hint dispatch) is active, and this locale's fix (switching from static `abstract_ops::to_number` to dispatching `coerce_to_number`) addresses the same underlying pattern. The missing fromIndex support in includes is an additional gap not probed by diff-prod.
