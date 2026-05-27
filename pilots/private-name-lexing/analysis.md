# private-name-lexing — Diff-Prod Analysis

| Fixture | Status | Relevance |
|---|---|---|
| private-field-encapsulation | FAIL | Private name lexing is the prerequisite for private field runtime behavior |
| class-inheritance | PASS | Class elements with private names must lex correctly in inheritance chains |

This locale ensures ECMA-262 PrivateIdentifier lexing conformance: `#name` tokenization, ZWNJ/ZWJ rejection as IdentifierStart, `#constructor` reserved name rejection, and same-line class-field terminator enforcement. The private-field-encapsulation FAIL is relevant because private field tests depend on correct private name tokenization upstream. The locale's direct measurement is via the 194-path focused PNL probe (134/194 after PNL-EXT 1). Remaining failures are class-elements static/runtime semantics, not lexing.
