# iterator-close-on-abrupt — Diff-Prod Analysis

| Fixture | Status | Relevance |
|---|---|---|
| iterator-close | FAIL | Directly exercises IteratorClose protocol on abrupt completion (mechanism gap #2) |
| destructuring-iterators | PASS | Destructuring iterators work but may not test non-exhausted close |
| iteration-protocol | PASS | Basic iteration works; gap is specifically the close-on-non-exhaustion path |

The FAIL on iterator-close directly reflects mechanism gap #2 (IteratorClose protocol): after array-destructure, if the iterator was not exhausted, `iterator.return()` must be called per ECMA-262 7.4.9. This locale (CLOSED at ICOA-EXT 1) addressed this by emitting close-if-not-done calls in both emit_destructure and emit_destructure_assign Array paths. The fixture FAIL indicates either residual IteratorClose gaps beyond destructure (e.g., for-of break/return) or that the fixture was recorded before the locale's fix landed.
