# spec-builtins-wrong-result — Diff-Prod Analysis

| Fixture | Status | Relevance |
|---|---|---|
| object-statics | PASS | Object built-in methods are part of the 389-fail wrong-result cluster |
| math-statics | PASS | Math namespace methods are intrinsic-object wrong-result candidates |
| date-ops | PASS | Date built-in operations fall under spec-builtins wrong-result |
| promise-statics | PASS | Promise built-in combinators are intrinsic-object surfaces |
| coercion-pipeline | FAIL | Mechanism gap #1 (ToPrimitive hint dispatch) directly causes wrong-result in built-in methods |

This locale targets 389 test262 failures where spec built-in constructors and prototypes produce wrong values across 122 surface families. Most relevant PASS fixtures confirm individual built-in surfaces work correctly. The coercion-pipeline FAIL is significant: mechanism gap #1 (ToPrimitive hint dispatch) is a shared upstream cause of wrong results across multiple built-in families.
