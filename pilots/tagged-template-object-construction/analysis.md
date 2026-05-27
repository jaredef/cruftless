# tagged-template-object-construction — Diff-Prod Analysis

| Fixture | Status | Relevance |
|---|---|---|
| tagged-template-raw | FAIL | Mechanism gap #9: strings.raw undefined in tag function calls |
| template-literals | PASS | Untagged template literals work; tagged template call-site object does not |
| string-ops | PASS | String operations correct; gap is template object construction |

This locale targets mechanism gap #9. The tagged-template-raw FAIL fixture is the direct empirical anchor: the compiler lowers TemplateLiteral to an Add chain using only cooked values, never constructing the frozen call-site object with its .raw property. Closing this locale should flip that fixture and fix String.raw.
