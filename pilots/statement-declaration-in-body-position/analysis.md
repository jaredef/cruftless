# statement-declaration-in-body-position — Diff-Prod Analysis

| Fixture | Status | Relevance |
|---|---|---|
| hoisting-semantics | PASS | var/function hoisting interacts with the Statement-vs-Declaration grammar distinction |
| labeled-control-flow | PASS | Labelled statement body positions are one of the parse_substatement call sites |
| for-in-for-of-lowering | FAIL | for-in/for-of body positions require Statement-only parse; this locale's direct trigger cluster |

This locale (CLOSED) added parse_substatement to reject Declaration tokens in control-flow body positions per ECMA-262 section 13.1. The for-in-for-of-lowering FAIL is directly relevant: the 14 triggering test262 fixtures were for-in/for-of declaration-in-body tests. That fixture's continued FAIL likely reflects broader for-in/for-of lowering issues beyond the parse-tier fix this locale delivered.
