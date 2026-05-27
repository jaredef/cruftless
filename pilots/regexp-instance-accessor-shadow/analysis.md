# regexp-instance-accessor-shadow — Diff-Prod Analysis

| Fixture | Status | Relevance |
|---|---|---|
| regexp-ops | PASS | RegExp instance property access (source, flags, lastIndex) is exercised |
| regexp-advanced | PASS | Advanced regexp usage depends on correct accessor vs data property semantics |
| property-key-order | FAIL | Own property enumeration order changes when shadow data props are removed from instances |

This locale refactors RegExp instances so source/flags/global/ignoreCase/etc. are prototype accessor getters (per spec section 22.2.6) rather than own data properties, and lastIndex gets the correct descriptor {w:t,e:f,c:f}. The property-key-order FAIL (mechanism gap #6: OrdinaryOwnPropertyKeys ordering) is tangentially relevant since removing 9 own data properties from RegExp instances changes their enumeration surface. The locale's direct measurement is via test262 property-descriptor inspection tests.
