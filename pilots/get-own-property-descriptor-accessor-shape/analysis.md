# get-own-property-descriptor-accessor-shape — Diff-Prod Analysis

| Fixture | Status | Relevance |
|---|---|---|
| object-define-property | FAIL | defineProperty accessor branch is the source of the descriptor shape bug |
| proxy-basics | PASS | Proxy traps invoke gOPD; pass suggests basic path works for data descriptors |
| proxy-invariants | FAIL | Proxy invariant checks depend on correct accessor vs data descriptor discrimination |

The FAIL on object-define-property directly reflects this locale's scope: when `defineProperty` installs an accessor with `{get: undefined, set: undefined}`, the storage uses `Option<Value>` semantics that collapse `Some(Value::Undefined)` to `None`, losing the accessor-shape signal. The `gOPD` then returns data-shape instead of accessor-shape. The proxy-invariants FAIL may partially stem from the same descriptor-shape confusion, since proxy trap invariant enforcement depends on correct accessor/data discrimination.
