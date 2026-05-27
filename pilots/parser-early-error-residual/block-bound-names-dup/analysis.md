# block-bound-names-dup — Diff-Prod Analysis

| Fixture | Status | Relevance |
|---|---|---|
| temporal-dead-zone | FAIL | TDZ enforcement depends on correct block-scope LexicallyDeclaredNames duplicate rejection |
| closures-scopes | PASS | Block-scope let/const declarations must not allow duplicates that break scope semantics |
| hoisting-semantics | PASS | Annex B var-in-block vs lexical-in-block interaction is the VDN/LDN overlap check |

This nested locale enforces ECMA-262 section 13.2.1 early errors: duplicate LexicallyDeclaredNames and LDN/VDN overlap at block scope. The temporal-dead-zone FAIL is directly relevant since TDZ tests exercise block-scope redeclaration scenarios. The closures-scopes and hoisting-semantics PASSes confirm that valid block-scoped programs are not over-rejected, which is critical given Rule 14's false-positive caution for restriction-adding moves.
