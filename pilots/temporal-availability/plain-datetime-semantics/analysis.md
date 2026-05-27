# temporal-availability/plain-datetime-semantics — Diff-Prod Analysis

| Fixture | Status | Relevance |
|---|---|---|
| date-ops | PASS | Legacy Date operations correct; Temporal PlainDateTime is a separate API surface |

No diff-prod fixtures directly exercise Temporal PlainDateTime semantics. The diff-prod suite covers legacy Date (date-ops, date-constructor-parse) but not the TC39 Temporal proposal surface. This locale's correctness is measured by the parent's exemplar suite at `pilots/temporal-availability/plain-datetime-semantics/exemplars/exemplars.txt`.
