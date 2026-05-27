# typed-array-missing-method — Diff-Prod Analysis

| Fixture | Status | Relevance |
|---|---|---|
| typed-arrays | PASS | TypedArray prototype methods are the direct surface; missing methods are the gap |
| buffer-encode | PASS | Buffer encoding shares substrate with ArrayBuffer/TypedArray backing stores |
| dataview-methods | FAIL | DataView.prototype methods are part of the 469-fail missing-method coordinate |
| typedarray-methods | FAIL | TypedArray method wrong-result failures overlap the missing-method surface |

This locale targets 469 test262 failures where TypedArray/DataView/ArrayBuffer prototype methods are entirely missing. The typed-arrays diff-prod fixture PASSes for the methods that exist, while dataview-methods and typedarray-methods FAIL, confirming gaps in both DataView and TypedArray surfaces. The missing-method coordinate means whole prototype entries are absent, distinct from wrong-result (which is the sibling typed-array-wrong-result locale).
