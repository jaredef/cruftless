# regexp-proto-test-coercion — Diff-Prod Analysis

| Fixture | Status | Relevance |
|---|---|---|
| regexp-ops | PASS | RegExp.prototype.test is the direct target of this locale's coercion fix |
| symbol-toprimitive | FAIL | ToPrimitive/ToString coercion path (@@toPrimitive -> toString -> valueOf) is mechanism gap #1 |
| coercion-pipeline | FAIL | The full coercion pipeline is what test() must use instead of static to_string |

This locale aligns RegExp.prototype.test with spec section 22.2.5.5: ToString-coerce the argument via the full ToPrimitive path and route through RegExpExec. The symbol-toprimitive and coercion-pipeline FAILs are relevant because the fix depends on correct ToPrimitive hint dispatch (mechanism gap #1). The regexp-ops PASS confirms the basic test() path works for string arguments. The locale's test262 gains depend on the coercion pipeline working end-to-end.
