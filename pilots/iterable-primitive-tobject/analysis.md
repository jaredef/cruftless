# iterable-primitive-tobject — Diff-Prod Analysis

| Fixture | Status | Relevance |
|---|---|---|
| iteration-protocol | PASS | Basic iteration protocol works; gap is specifically primitive-to-iterable path |
| destructuring-iterators | PASS | Iterator-based destructuring works for object sources |
| string-ops | PASS | String operations work, but spread on string primitives uses collect_iterable |
| structured-clone | PASS | Uses collect_iterable internally for Set/Map reconstruction |

The PASS on iteration-protocol and string-ops may seem to contradict the locale's bug, but the gap is narrow: only the `collect_iterable` path short-circuits on non-Object values. `for-of` on strings and `Array.from("abc")` use separate ToObject-wrapping paths that work correctly. The bug surfaces specifically on spread into array (`[..."abc"]`), `new Set([..."xyz"])`, and similar collect_iterable callers. The diff-prod fixtures likely exercise the working paths rather than the broken collect_iterable route.
