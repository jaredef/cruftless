# rusty-js-regex-fast — Diff-Prod Analysis

| Fixture | Status | Relevance |
|---|---|---|
| regexp-ops | PASS | Core regex operations correct; this locale targets performance + memory leak |
| regexp-advanced | PASS | Advanced regex features correct; performance is the gap |

No diff-prod fixtures directly exercise this locale's performance scope. Diff-prod measures correctness; this locale targets performance measured by CRB (string_url_sweep's 8.31x gap) and a memory leak investigation. The regexp-ops and regexp-advanced PASS fixtures confirm correctness is not in question.
