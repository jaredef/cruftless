# parser-permissiveness-audit — Diff-Prod Analysis

| Fixture | Status | Relevance |
|---|---|---|
| arrow-functions | PASS | Arrow param heads are the primary surface for escaped-keyword/dup-param rejection |
| arrow-edge-cases | FAIL | Edge-case arrow parsing overlaps with cover-grammar ReservedWord checks |
| destructuring | PASS | Destructuring patterns in arrow params exercise BindingIdentifier restriction sites |
| for-in-for-of-lowering | FAIL | For-of head sites are audit targets for escaped contextual keyword rejection |
| unicode-identifiers | PASS | Escaped identifiers that happen to be keywords must be rejected at restricted sites |

This locale audits cruft's parser for systematic permissiveness at "no-ReservedWord" sites. The arrow-edge-cases FAIL is relevant because the cover-grammar resolution from CoverParenthesizedExpression to ArrowFormalParameters is exactly where escaped-keyword rejection fires. The for-in-for-of-lowering FAIL overlaps with escaped `of` in for-of head checks. Both PASSing arrow/destructuring fixtures confirm the audit does not over-reject valid programs.
