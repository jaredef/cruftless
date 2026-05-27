# non-ctor-registration-discipline — Diff-Prod Analysis

| Fixture | Status | Relevance |
|---|---|---|
| regexp-ops | PASS | RegExp.prototype methods must not be constructors |
| regexp-advanced | PASS | Advanced RegExp methods share the register_method fix |
| async-promise | PASS | Promise.prototype.{then,catch,finally} must not be constructors |
| class-inheritance | PASS | Built-in function constructor classification affects class extends chains |

This locale fixed two buggy register_method helpers in regexp.rs and promise.rs that defaulted to is_constructor=true, violating ECMA-262 section 21.3. All relevant diff-prod fixtures pass, consistent with the locale's CLOSED status. The fix ensures `new RegExp.prototype.test()` and `new Promise.prototype.then()` correctly throw TypeError.
