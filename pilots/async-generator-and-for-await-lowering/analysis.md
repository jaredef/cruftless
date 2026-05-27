# async-generator-and-for-await-lowering — Diff-Prod Analysis

| Fixture | Status | Relevance |
|---|---|---|
| async-iteration | PASS | Async iteration (for-await-of with async iterables) passes in the diff-prod probe |
| generators | PASS | Sync generator suspension/resume works; async generators extend this with promise wrapping |
| async-promise | PASS | Async/await promise resolution works; async generators combine this with generator suspension |
| generator-suspension | FAIL | Lazy generator suspension gaps (mechanism gap #3) directly affect async generator yield semantics |

The PASS on async-iteration is encouraging but diff-prod likely tests simple happy-path async iteration. The 1492-fixture test262 pool this locale targets includes edge cases in for-await-of destructuring, async generator throw/return protocol, and yield-delegation that go far beyond the diff-prod surface. The FAIL on generator-suspension confirms mechanism gap #3 (lazy generator suspension) is active, and since async generators extend sync generator suspension with promise wrapping, this gap propagates directly into the locale's scope. The healthy PASS on generators and async-promise shows the two halves exist independently; the locale's work is at their composition point.
