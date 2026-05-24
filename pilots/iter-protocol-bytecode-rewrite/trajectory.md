# iter-protocol-bytecode-rewrite — Trajectory

Append-only log of rounds. Most recent at bottom.

---

## IPBR-EXT 0 — workstream founding (2026-05-24)

**Trigger**: keeper directive ("spawn a locale for iter-protocol-bytecode-rewrite") in response to GPI-EXT 3 chapter-close report identifying for-of envelope as the new per-iter dominator post-IHI/GPI on string_url_sweep header_loop.

**Empirical basis** (per GPI-EXT 3 cost analysis):
- Post-IHI/GPI hot method-call path: ~15ns/iter dispatch
- Per-iter for-of envelope: ~620ns (GetProp"next" + Call + result-alloc + GetProp"done" + JumpIfTrue + GetProp"value" + StoreLocal)
- Iter envelope is now the per-iter dominator by ≥40× over the post-rewrite method dispatch

**Strategic framing** — first prospective falsifier for `docs/standing-rule-13-prospective-application.md`:
- §3 conditions C1-C4 all hold (IHI+GPI sibling-anchor; shape-compatible; cost-positive; bail-safe via stash-original-bytes)
- §5 falsifier: locale must close in ≤3 implementation rounds for the prospective-application thesis to gain second corroboration
- Empirical outcome of this locale will determine whether the thesis is promoted to corpus Doc 742 with strengthened claim or weakened with caveat

**Founding artefacts**:
- `pilots/iter-protocol-bytecode-rewrite/seed.md` (this round)
- `pilots/iter-protocol-bytecode-rewrite/trajectory.md` (this file)
- `pilots/iter-protocol-bytecode-rewrite/docs/` + `pilots/iter-protocol-bytecode-rewrite/fixtures/` scaffolded

**Pre-spawn rule 11 5-axis check**:
- (A1) component A/B — DONE via GPI-EXT 3 cost-analysis + Doc 741 instance precedent
- (A2) op-set coverage — 2 new opcodes (ForOfArrayFastNext 0xFE, ForOfStringFastNext 0xFF); both index-based scans
- (A3) value-domain — Array/String receivers; covered by existing Value variants + an IterState sidecar
- (A4) locals-marshaling — N/A
- (A5) emission-shape — for-of's compiler.rs:1270-1427 emission is the one load-bearing target

**Deeper-layer design (rule 13 prospective application)**:
- SKIP iter-cache substrate-introduction (no IterStateCache HashMap rounds)
- Go directly to multi-op-pattern bytecode rewrite at runtime entry into `__engine_iter_next` on the recognized fast-path iter_slot
- IterState as Value sidecar (or new Value variant); populated by `__engine_get_iterator` when Array.prototype[@@iterator] / String.prototype[@@iterator] are unmodified
- Bail recovery via stash-original-bytes in iter_slot's sidecar; restore on iterator-protocol override or unexpected receiver shape

**Composition with IHI + GPI (cross-locale)**:
- IHI + GPI already rewrote the hot string-method-call dispatch inside the for-of body
- IPBR rewrites the FOR-OF ENVELOPE around those methods
- All three locales compose at the same source-line hot loop; cumulative reclaim materializes at the apparatus-tier per Doc 740 §II.2 P4

**Next round**: IPBR-EXT 1 — design doc at `docs/design.md`. Output:
1. ForOfArrayFastNext / ForOfStringFastNext opcode operand shape (likely u16 iter-state slot + u16 bind slot + i32 done-jump offset = 8 bytes per op)
2. IterState representation: enum variant on Value vs. sidecar table on Runtime
3. Rewrite-pattern detection: how to find the GetProp("next")...GetProp("value") sequence from inside __engine_iter_next
4. Bail-recovery mechanism: stash-original-bytes vs. re-emit
5. Per-feature LOC budget (target ≤80 LOC per Pred-ipbr.1)
6. Override-safety counter shape

**Status**: SCAFFOLDED. Founding artefacts written; IPBR-EXT 1 next.
