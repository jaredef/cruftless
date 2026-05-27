# string-proto-method-length-and-split-limit — Diff-Prod Analysis

| Fixture | Status | Relevance |
|---|---|---|
| string-ops | PASS | String.prototype.split is a core string-ops surface; limit semantics exercised here |
| regexp-ops | PASS | split with RegExp separator is the installation path where length=0 was hardcoded |
| coercion-pipeline | FAIL | ToUint32(limit) is a coercion step; mechanism gap #1 (ToPrimitive) may affect limit argument |

This locale fixes two coupled bugs: String.prototype.{split,replace,replaceAll}.length was 0 instead of 2, and split's limit parameter used NaN-as-no-limit instead of ToUint32 semantics. The string-ops PASS suggests the core split behavior works for common cases. The coercion-pipeline FAIL is relevant because ToUint32 on the limit argument is a coercion step that composes with ToPrimitive dispatch (mechanism gap #1) when the limit is an Object.
