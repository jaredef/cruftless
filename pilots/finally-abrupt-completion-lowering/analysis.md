# finally-abrupt-completion-lowering — Diff-Prod Analysis

| Fixture | Status | Relevance |
|---|---|---|
| finally-return-override | FAIL | Mechanism gap #5: finally block skipped on break/continue/return from try |
| labeled-control-flow | PASS | Break/continue targets work; gap is their interaction with try-finally |
| error-throws | PASS | Throw-triggered finally works on normal path; abrupt exit path does not |

This locale targets mechanism gap #5. The finally-return-override FAIL fixture is the direct empirical anchor: the compiler emits TryExit only on normal completion, so break/continue/return bypass the finally block entirely. Closing this locale should flip that fixture.
