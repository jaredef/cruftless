# var-hoisting-through-try-block — Diff-Prod Analysis

| Fixture | Status | Relevance |
|---|---|---|
| hoisting-semantics | PASS | var hoisting across syntactic boundaries is the direct surface |
| closures-scopes | PASS | Scope chain correctness composes with hoisted var visibility |
| error-throws | PASS | try/catch error handling is the syntactic context where the hoisting bug manifested |

This locale (CLOSED) added collect_hoisted_var_names() to pre-allocate var declarations nested inside try/if/for/while/switch blocks at the enclosing function/script scope. All relevant fixtures PASS, consistent with the closure. The fix was a bytecode compiler change (Phase A.6 hoisting walker), not a runtime protocol issue. No numbered mechanism gap applies; the bug was incomplete VarScopedDeclarations traversal.
