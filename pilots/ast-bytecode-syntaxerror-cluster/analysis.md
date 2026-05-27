# ast-bytecode-syntaxerror-cluster — Diff-Prod Analysis

| Fixture | Status | Relevance |
|---|---|---|
| asi-rules | PASS | ASI restriction early errors are one sub-surface of the 1296-fixture SyntaxError cluster |
| directive-prologues | FAIL | Directive prologue strict-mode early errors (e.g., "use strict" + duplicate params) are lowering-time SyntaxErrors |
| temporal-dead-zone | FAIL | TDZ enforcement for let/const is a lowering-time early-error concern overlapping this cluster |
| with-scoping | FAIL | `with` in strict mode should be a SyntaxError; this is a lowering-phase early-error rule |

The 1296-fixture cluster targets early-error rules enforced at AST-to-bytecode lowering time (post-parse). The FAIL on directive-prologues, temporal-dead-zone, and with-scoping confirms three sub-surfaces of this cluster are broken: strict-mode early errors, let/const redeclaration/TDZ scoping, and with-statement strict-mode rejection. The PASS on asi-rules shows that some early-error-adjacent rules work at the parser level, but the lowering-phase rules that this locale owns remain incomplete.
