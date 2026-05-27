# hoistable-declaration-as-statement-body — Diff-Prod Analysis

| Fixture | Status | Relevance |
|---|---|---|
| hoisting-semantics | PASS | Function hoisting works, but this fixture may not cover if-body FunctionDeclaration |
| closures-scopes | PASS | Scope semantics for function declarations in blocks are adjacent |

The PASS on hoisting-semantics suggests basic function declaration hoisting works; the gap is specifically that cruft rejects `if (true) function f() {}` in sloppy mode, which Annex B B.3.4 permits. This is a parser-level acceptance gap (475 test262 failures) rather than a runtime semantics gap, so diff-prod fixtures that test runtime behavior of already-accepted code would not expose it. The fixture would need to contain `if`-body FunctionDeclarations to trigger the rejection.
