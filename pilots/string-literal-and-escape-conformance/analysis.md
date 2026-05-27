# string-literal-and-escape-conformance — Diff-Prod Analysis

| Fixture | Status | Relevance |
|---|---|---|
| string-escapes | FAIL | String escape sequences are the direct target surface of this locale |
| unicode-identifiers | PASS | Unicode escape handling in the lexer shares substrate with string escapes |
| directive-prologues | FAIL | Strict-mode legacy-octal rejection depends on directive-prologue detection |

This locale targets section 12.9 StringLiteral escape conformance: empty \u{} braces, 3-digit octal cap, and strict-mode legacy-octal/NonOctalDecimalEscape rejection. The string-escapes FAIL is the direct surface: malformed escape handling is exactly what this locale fixes. The directive-prologues FAIL connects to the SLEC-EXT 2 deferred work (retroactive strict-mode rejection of strings lexed before "use strict" is detected). Unicode-identifiers PASSes, confirming the shared \u{} escape infrastructure works for identifiers.
