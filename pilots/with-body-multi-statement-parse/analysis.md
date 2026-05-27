# with-body-multi-statement-parse — Diff-Prod Analysis

| Fixture | Status | Relevance |
|---|---|---|
| with-scoping | FAIL | `with` statement scoping is the direct runtime surface this locale implements |
| closures-scopes | PASS | Scope chain behavior composes with with-statement object environment records |
| proxy-basics | PASS | Proxy has-trap dispatch is a WBMS-EXT 2 residual surface for with-environment lookup |

This locale moved `with` from a parser stub to typed AST + bytecode + runtime support (WBMS-EXT 1 parse fix, WBMS-EXT 2 runtime semantics). The with-scoping FAIL is the direct witness: with-statement scope behavior still has residuals (73/264 PASS at WBMS-EXT 2). Remaining failures are dominated by @@unscopables, Proxy has integration, global-this aliasing, and eval environment records, which are tracked by sibling locales like with-unscopables-proxy-has.
