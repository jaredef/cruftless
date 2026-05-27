# parser-permissiveness-audit-extensions — Diff-Prod Analysis

| Fixture | Status | Relevance |
|---|---|---|
| for-in-for-of-lowering | FAIL | Escaped `of` in for-of head and head-vs-body name conflicts are direct targets |
| arrow-functions | PASS | Duplicate arrow param names are an extension target; valid arrows must not regress |
| arrow-edge-cases | FAIL | Arrow duplicate-param early error overlaps with edge-case arrow scenarios |

This locale extends PPA with three targeted parser sub-sites: escaped contextual keyword `of` in for-of, duplicate arrow params, and for-in/of head-bound-name conflicts. The for-in-for-of-lowering FAIL is directly relevant since the head-vs-body BoundNames conflict check fires at for-in/of compilation. The locale is CLOSED at PPAE-EXT 1 with +7 exemplar gains and zero regressions across for-of, for-in, and arrow-function surfaces.
