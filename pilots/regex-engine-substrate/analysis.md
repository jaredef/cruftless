# regex-engine-substrate — Diff-Prod Analysis

| Fixture | Status | Relevance |
|---|---|---|
| regexp-ops | PASS | Core regexp exec/test/match paths route through regexp_exec |
| regexp-advanced | PASS | Advanced regexp features (named groups, unicode) are engine-level concerns |
| string-ops | PASS | String.prototype.{match,replace,search,split} delegate to regexp_exec |

This locale treats the regex engine as a first-class substrate, closing engine-level gaps after the spec-filter arc closed the surface side. The 6 residual failures are genuinely engine-level: capture-reset across iterations, u-flag surrogate handling, v-flag set notation, and duplicate named groups. All three relevant diff-prod fixtures pass, indicating the common regexp paths work. The remaining engine gaps are measured via test262, not diff-prod.
