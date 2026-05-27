# regexp-conformance — Diff-Prod Analysis

| Fixture | Status | Relevance |
|---|---|---|
| regexp-ops | PASS | Core RegExp prototype methods are the primary conformance surface |
| regexp-advanced | PASS | Named groups, unicode, flags are engine-level conformance targets |
| regexp-symbol-protocols | FAIL | @@split, @@match, @@replace, @@search protocol conformance is part of the cluster |
| string-ops | PASS | String methods dispatching to regexp exercise the integration surface |

This locale is the parent coordinate for the 491-failure regexp conformance cluster in the full-suite matrix. The regexp-symbol-protocols FAIL is directly relevant since the cluster includes RegExp.prototype[@@split] captures, @@match coercion, and related well-known Symbol protocol gaps. The regexp-ops and regexp-advanced PASSes confirm the core engine paths work. The locale's yield is measured via full-suite matrix re-categorization after each rung.
