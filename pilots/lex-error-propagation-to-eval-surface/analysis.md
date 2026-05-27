# lex-error-propagation-to-eval-surface — Diff-Prod Analysis

| Fixture | Status | Relevance |
|---|---|---|
| eval-lexical-capture | FAIL | Eval pipeline is the surface where lex errors must propagate (mechanism gap #4) |
| directive-prologues | FAIL | Strict mode directives in eval interact with lex-error propagation |

The FAIL on eval-lexical-capture (mechanism gap #4: direct eval lexical capture) is adjacent to this locale's scope: lex errors from strict-mode-triggered rejections inside `(0,eval)(...)` must surface as SyntaxError instances at the runner's catch. If lex errors are swallowed in the parse/module pipeline, test262 probes that expect SyntaxError from eval'd invalid source report "expected SyntaxError, got [nothing]." The directive-prologues FAIL intersects because strict-mode activation via "use strict" in eval is a key trigger for the lex-error-class rejections this locale unblocks.
