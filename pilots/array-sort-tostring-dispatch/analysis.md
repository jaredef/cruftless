# array-sort-tostring-dispatch — Diff-Prod Analysis

| Fixture | Status | Relevance |
|---|---|---|
| array-methods | PASS | Array.prototype.sort passes for primitive-element arrays; the gap is Object-element toString dispatch |
| symbol-toprimitive | FAIL | ToPrimitive(hint=string) dispatch is exactly the mechanism sort needs for Object elements |

The PASS on array-methods indicates that sort works correctly for arrays of primitives (strings, numbers) where no ToPrimitive dispatch is needed. The locale's bug -- SortCompare using static `abstract_ops::to_string` instead of ToPrimitive(hint=string) -- only manifests when sorting Object elements with custom toString/valueOf. The FAIL on symbol-toprimitive confirms mechanism gap #1 (ToPrimitive hint dispatch) remains active in the broader engine, which is the same dispatch path sort needs. The locale reports CLOSED at ASD-EXT 1 with verification pending.
