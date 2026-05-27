# line-terminator-conformance — Diff-Prod Analysis

| Fixture | Status | Relevance |
|---|---|---|
| asi-rules | PASS | ASI depends on line terminator recognition; PASS suggests LF/CR cases work |
| string-escapes | FAIL | String escape sequences interact with line terminator handling in literals |
| unicode-identifiers | PASS | Unicode handling at the lex tier is adjacent |

The PASS on asi-rules confirms LF and CR line terminators are correctly recognized for ASI purposes. The locale's gap is specifically U+2028 (LS) and U+2029 (PS) recognition at every lex site that gates on line termination. The string-escapes FAIL may partially intersect: the post-2019 JSON-superset amendment permits LS/PS literally inside string bodies, which requires the lexer to distinguish "LS/PS as line terminator" from "LS/PS as string content." Diff-prod fixtures using ASCII-only source would not expose the LS/PS gap.
