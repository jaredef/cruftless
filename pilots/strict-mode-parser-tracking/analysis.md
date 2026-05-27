# strict-mode-parser-tracking — Diff-Prod Analysis

| Fixture | Status | Relevance |
|---|---|---|
| generators | PASS | yield-as-keyword vs yield-as-identifier depends on generator context tracking |
| directive-prologues | FAIL | Strict-mode detection feeds the parser tracking this locale implements |
| destructuring-iterators | PASS | for-of destructuring with yield in defaults exercises the yield-branch guard |

This locale (CLOSED at SMPT-EXT 1) added function_body_depth tracking so yield at script/module top-level parses as IdentifierReference instead of YieldExpression. The generators fixture PASSes, confirming yield-as-YieldExpression inside function bodies is preserved. The directive-prologues FAIL is relevant because strict-mode tracking (deferred to SMPT-EXT 2+) depends on correct directive-prologue recognition. The deeper generator-vs-non-generator and strict-vs-sloppy axes remain deferred.
