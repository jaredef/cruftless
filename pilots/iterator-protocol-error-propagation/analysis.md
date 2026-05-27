# iterator-protocol-error-propagation — Diff-Prod Analysis

| Fixture | Status | Relevance |
|---|---|---|
| iterator-close | FAIL | IteratorClose protocol gaps overlap with iterator error propagation |
| destructuring-iterators | PASS | Destructuring iterators work post-fix via @@iterator-based paths |
| iteration-protocol | PASS | Basic iteration protocol is correct |
| error-throws | PASS | Error propagation works in general; gap was specific to destructure-iterator |

This locale is CLOSED (IPEP-EXT 1). The fix routed Array-destructure paths through `__destr_iter_open/_step/_rest` engine helpers instead of index-based shortcuts, enabling proper error propagation from @@iterator getters and .next() calls. The PASS on destructuring-iterators and iteration-protocol confirms the fix is effective for the covered paths. The iterator-close FAIL reflects the adjacent but distinct IteratorClose gap (mechanism gap #2), which was addressed by the sibling locale iterator-close-on-abrupt.
