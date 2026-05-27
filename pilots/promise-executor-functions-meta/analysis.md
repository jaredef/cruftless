# promise-executor-functions-meta — Diff-Prod Analysis

| Fixture | Status | Relevance |
|---|---|---|
| async-promise | PASS | Promise constructor usage exercises the executor resolve/reject functions |
| promise-statics | PASS | Promise static methods internally create executor functions with the same meta |
| microtask-ordering | PASS | Promise resolution ordering depends on correctly constructed executor functions |

This locale fixes the name and length properties of Promise executor resolve/reject functions: name must be "" (was "<promise-resolve>"/"<promise-reject>") and length must be 1 (was 0). No diff-prod fixture directly inspects `resolve.name` or `resolve.length`, so the three relevant fixtures all pass. The fix is purely observable through reflection and is measured via test262, not diff-prod.
