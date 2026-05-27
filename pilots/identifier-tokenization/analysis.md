# identifier-tokenization — Diff-Prod Analysis

| Fixture | Status | Relevance |
|---|---|---|
| unicode-identifiers | PASS | Unicode identifier handling works; gap is specifically ReservedWord rejection |

The PASS on unicode-identifiers confirms that identifier tokenization for Unicode characters works correctly. This locale's gap is orthogonal: cruft accepts ReservedWords (e.g., `var break = 1`) as BindingIdentifiers where the spec requires SyntaxError. No diff-prod fixture directly exercises ReservedWord-as-binding-identifier rejection, as diff-prod fixtures test runtime behavior of valid programs rather than parser rejection of invalid ones.
