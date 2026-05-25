# string-trim-whitespace-spec — Trajectory

## SPTW-EXT 0+1 — founding + closure (2026-05-25)

**Trigger**: keeper "Now continue with trajectory" after Doc 736 Appendix A landed. Matrix probe surfaced trim cluster (#5) as a coherent 22-test root cause.

**Edits** (~35 LOC) — see seed §II.

**Pre-implementation gotcha**: initial slow-path-only fix appeared to do nothing. Probe revealed the IC fast path `fast_string_trim` was returning Some(self) for non-ASCII strings whose first byte (UTF-8 leading byte for NBSP/BOM) failed the narrow ASCII WS check; the early-return "no trim needed" fired without ever consulting the slow path. Fix moved the `!s.is_ascii() → bail` check BEFORE the trim-byte-scan.

**Verification**:
- Probes (NBSP, BOM, regular ASCII): all GREEN
- Exemplar (22 trim fixtures): PASS 0 → **21** (+21; the 1 remaining is unrelated brand-toString concern on Arguments receiver)
- Regression on String/prototype previously-passing (494): 494/494 preserved

### Findings

**Finding SPTW.1**: cluster homogeneity check confirmed. The trim 22-test cluster had a single substrate root cause (whitespace-set under-coverage); 21/22 close in one round; only 1 unrelated test remained. Per Finding T262C.4 / EPSUA.6, this is shared-upstream at the sub-cluster level — the discriminator works.

**Finding SPTW.2 (fast-path / slow-path interaction)**: substrate fixes that change slow-path behavior must verify the corresponding IC fast path doesn't shadow them. The fast path's "no trim needed" early-return was a Doc 740-style substrate-introduction-prefix issue: optimizing for ASCII strings while accidentally short-circuiting non-ASCII strings. The carve-back (bail-to-slow-on-non-ASCII) is the deeper-layer closure.

**Status**: CLOSED at SPTW-EXT 1.
