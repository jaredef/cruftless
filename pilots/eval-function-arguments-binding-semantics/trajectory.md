# eval-function-arguments-binding-semantics — Trajectory

## EFABS-EXT 0 — founding (2026-05-27)

**Trigger**: LPA-EXT 8/9 compiler gap analysis identified mechanism gaps #4 (direct eval lexical capture, crash) and #8 (arguments-as-Array shape violation). Promoted from CANDIDATES.md Tier M entry (abe) after overlap-check confirmed parser-tier locales (`strict-binding-eval-arguments/`, `non-simple-params-strict-body/`) are early-error-only and do not overlap with this runtime-tier scope.

LPA-EXT 5 Arc F: 582 test262 rows, 0% diff-prod pass rate (lowest of all arcs).

**Status**: FOUNDED. EFABS-EXT 1 (Arguments exotic object) is the first substantive rung.
