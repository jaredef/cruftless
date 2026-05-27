# class-elements-static-semantics — Diff-Prod Analysis

| Fixture | Status | Relevance |
|---|---|---|
| class-inheritance | PASS | Basic class parsing and inheritance work; static semantics are post-parse validation |
| private-field-encapsulation | FAIL | Private name early errors (undeclared private names in computed properties) are a direct sub-surface |

The PASS on class-inheritance confirms that class parsing and basic element handling work at the grammar level. The FAIL on private-field-encapsulation overlaps directly with this locale's scope: class-element static semantics for private names require post-parse validation that undeclared private names in computed property positions are rejected as early errors. The locale also targets `arguments` usage inside class field initializers, which no diff-prod fixture probes. This is primarily a parser-phase validation locale; most of its impact will show in test262 fixtures rather than diff-prod.
