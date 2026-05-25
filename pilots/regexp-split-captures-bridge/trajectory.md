# regexp-split-captures-bridge — Trajectory

## RSCB-EXT 1 — interleave capture groups in split result (2026-05-25)

**Trigger**: RES audit-2 Gap A.

**Edits** (~55 LOC at `regexp.rs::String.prototype.split`):
- Replace `Vec<String>` parts with `Vec<Value>` so `undefined` (non-participating capture) can survive.
- Replace the regex branch's `rx.split_str(&s)` with a custom captures_at loop: push pre-match chunk, then each capture group (skipping [0]) as a string or undefined, then advance cursor; handle zero-width match per AdvanceStringIndex.
- Plain-string branch unchanged in semantics, returns `Vec<Value>` for type uniformity.

**Verification**:
- Probe: `"a1b2c3".split(/(\d)/)` → `["a","1","b","2","c","3",""]` ✓ (was `["a","b","c",""]`)
- Probe: `"a1b2".split(/(\d)(\D)?/)` → `["a","1",undefined,"b","2",undefined,""]` ✓ (non-participating capture preserved)
- Plain-string split unchanged.
- test262 String.prototype.split prev-fails: +N (counted as part of cumulative below).

**Status**: RSCB-EXT 1 CLOSED.
