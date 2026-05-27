# ast-bytecode-wrong-result — Diff-Prod Analysis

| Fixture | Status | Relevance |
|---|---|---|
| arguments-object | FAIL | Arguments object semantics (mapped/non-strict accessor coupling) is the top surface family in the 1244-fixture cluster |
| reference-semantics | FAIL | Reference resolution wrong-result shapes overlap with the lowered-code-produces-wrong-values pattern |
| hoisting-semantics | PASS | Hoisting produces correct values; the wrong-result gap is in deeper argument/parameter binding semantics |
| closures-scopes | PASS | Closure capture works; the wrong-result cluster targets parameter-binding shadowing, not closure capture |

The FAIL on arguments-object is a direct confirmation of the top sub-surface in this 1244-fixture cluster: mapped arguments in non-strict mode (accessor coupling between named parameters and arguments[i]) produces wrong values. Mechanism gap #8 (Arguments object shape -- is Array instead of exotic) is the load-bearing upstream cause. The FAIL on reference-semantics confirms adjacent wrong-result shapes. The PASS on hoisting-semantics and closures-scopes shows that variable resolution works in the general case; the gap is specifically in the arguments-object exotic behavior and parameter-binding edge cases.
