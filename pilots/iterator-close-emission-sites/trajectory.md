# iterator-close-emission-sites — Trajectory

## ICES-EXT 0 — founding (2026-05-27)

**Trigger**: LPA-EXT 8/9 compiler gap analysis identified IteratorClose emission as mechanism gap #2. The opcode `Op::IterClose` (0xD2) exists but the compiler emits it only for destructuring partial consumption, not for for-of break/throw/return, yield* delegation, or spread-on-throw.

**Status**: FOUNDED. ICES-EXT 1 (for-of break IterClose emission) is the first substantive rung.
