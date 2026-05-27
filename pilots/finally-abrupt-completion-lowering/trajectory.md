# finally-abrupt-completion-lowering — Trajectory

## FACL-EXT 0 — founding (2026-05-27)

**Trigger**: LPA-EXT 8/9 compiler gap analysis identified finally-on-abrupt-loop-exit as mechanism gap #5. The compiler's TryEnter/TryExit does not account for break/continue/return crossing a try boundary.

**Status**: FOUNDED. FACL-EXT 1 (try-block stack + break/continue TryExit emission) is the first substantive rung.
