# iterator-close-emission-sites — Diff-Prod Analysis

| Fixture | Status | Relevance |
|---|---|---|
| iterator-close | FAIL | Mechanism gap #2: IteratorClose not emitted at for-of break/throw/return |
| iteration-protocol | PASS | Iterator protocol shape correct; close protocol is the gap |
| destructuring-iterators | PASS | Destructuring partial-consumption IterClose already emitted |
| for-in-for-of-lowering | FAIL | Related: for-of compilation missing IterClose on abrupt exit |

This locale targets mechanism gap #2. Op::IterClose (0xD2) exists but the compiler emits it only for destructuring. The iterator-close and for-in-for-of-lowering FAIL fixtures are the direct anchors: for-of break, throw, return, and yield* delegation all lack IterClose emission.
