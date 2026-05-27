# lexer-goal-symbol-selection — Diff-Prod Analysis

| Fixture | Status | Relevance |
|---|---|---|
| regex-division-ambiguity | PASS | Directly exercises the regex/div goal-symbol disambiguation |
| regexp-ops | PASS | RegExp operations work correctly post-tokenization |
| template-literals | PASS | Template tail re-entry is a goal-symbol selection site |

All three relevant fixtures PASS, indicating that the current partial goal-symbol selection machinery (bump_regexp deriving goal from token_completes_expression) handles the common cases correctly. The locale's scope is architectural: eliminating the rewind-class failure mode by making goal-symbol selection a single named parser-state predicate at every lex-call boundary. The rewind sites (stmt.rs:1251, expr.rs:1583) are correctness-preserving workarounds that add complexity but do not produce wrong output for the patterns diff-prod exercises.
