# strict-binding-eval-arguments — Diff-Prod Analysis

| Fixture | Status | Relevance |
|---|---|---|
| directive-prologues | FAIL | "use strict" directive recognition is the precondition for strict-mode binding rejection |
| arguments-object | FAIL | `arguments` as a binding identifier in strict mode is a direct target of this locale |
| destructuring | PASS | Destructuring binding sites are where eval/arguments as BindingIdentifier can appear |

This locale adds SyntaxError rejection when `eval` or `arguments` appear as BindingIdentifier in strict mode. The directive-prologues FAIL is relevant because correct strict-mode detection is the precondition (the seed notes Parser::strict_mode must be true). The arguments-object FAIL (mechanism gap #8) is directly connected: the arguments object shape and its binding-identifier restrictions are entangled surfaces. Destructuring PASSes, suggesting binding-id checks in destructuring contexts already work for non-reserved names.
