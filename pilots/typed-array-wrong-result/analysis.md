# typed-array-wrong-result — Diff-Prod Analysis

| Fixture | Status | Relevance |
|---|---|---|
| typed-arrays | PASS | TypedArray prototype methods are the direct surface producing wrong results |
| typedarray-methods | FAIL | TypedArray method correctness failures are the direct diff-prod witness |
| dataview-methods | FAIL | DataView methods producing wrong values fall under this coordinate |
| arraybuffer-operations | FAIL | ArrayBuffer operation results compose with TypedArray value semantics |

This locale targets 614 test262 failures where TypedArray/DataView/ArrayBuffer methods return wrong values. The typedarray-methods and dataview-methods FAILs are direct witnesses: existing methods produce incorrect output. The typed-arrays PASS suggests basic TypedArray construction and simple access works, with wrong-result concentrated at abstract-op-level semantics (ValidateTypedArray, IntegerIndexedElementGet, SetValueInBuffer). The arraybuffer-operations FAIL connects because buffer-level value semantics feed into typed array element access.
