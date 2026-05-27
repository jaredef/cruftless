# ast-bytecode-missing-method — Diff-Prod Analysis

| Fixture | Status | Relevance |
|---|---|---|
| class-inheritance | PASS | Basic class statements/expressions work; missing methods are in class-element specifics (private fields, accessors) |
| private-field-encapsulation | FAIL | Private field/method encapsulation is a major sub-surface of the 1088-fixture missing-method cluster |
| super-new-target | FAIL | super/new.target semantics overlap with class element internal method availability |

The 1088-fixture cluster this locale targets spans class elements, private fields, and static intrinsics that cruft does not expose. The FAIL on private-field-encapsulation directly confirms that private class elements -- a major sub-surface (42 of 100 exemplars in class expressions/statements) -- are broken. The FAIL on super-new-target confirms adjacent class-element internal method gaps. The PASS on class-inheritance shows basic class structure works; the missing pieces are the advanced element-level internal methods and accessor semantics.
