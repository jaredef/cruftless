# parser-early-error-residual — Diff-Prod Analysis

| Fixture | Status | Relevance |
|---|---|---|
| directive-prologues | FAIL | Parser early errors overlap with directive prologue handling |
| temporal-dead-zone | FAIL | Block-scope early errors (dup LexicallyDeclaredNames) relate to TDZ enforcement |
| hoisting-semantics | PASS | Correct early-error rejection must not break valid hoisting programs |

This locale targets the 809-failure parser-early-error coordinate in the full-suite matrix. Diff-prod fixtures primarily exercise valid programs, so most of the 809 negative-syntax failures have no direct diff-prod analogue. The directive-prologues and temporal-dead-zone failures are relevant because parser early-error enforcement at block scope and directive sites overlaps with those fixtures' tested behaviors. The bulk of this locale's yield is measured via test262, not diff-prod.
