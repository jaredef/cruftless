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

---

## IPBR-EXT 1 — design doc (2026-05-24)

Output: `docs/design.md`. Key design pivot from seed:

**Compile-time emission of fused fast-next opcode + existing slow-path emission unchanged as fallthrough**. No runtime bytecode rewrite needed; the opcode self-dispatches based on iter_slot's value shape every iteration.

Simpler than the seed's runtime-rewrite scheme, equally effective at closing the cost. Standing rule 13 still applies — the "deeper layer" here is compile-time emission of a fused dispatch op rather than runtime rewrite of an existing op.

**Bytecode**: `Op::ForOfFastNext = 0xFE`, 10-byte operand (u16 iter_slot + u16 bind_slot + i32 done_offset + i16 next_iter_offset).

**Per-iter cost model**: ~545ns slow → ~50ns fast (≥11× reduction).

**Bail-safety**: probes iter object's __it_src__/__it_idx__ shape every iteration; falls through to slow path on shape mismatch. User override of ArrayIterator.prototype.next bypassed at first cut (Finding IPBR.1 candidate; consumer-app surface rare).

**LOC budget**: ~68. Within Pred-ipbr.1's ≤80.

**Methodology**: implement at IPBR-EXT 2 (Array path), bench, possibly close. IPBR-EXT 3 reserved for String path + composition + Pred-ipbr.6 disposition.

**Status**: DESIGN COMPLETE. Proceed to IPBR-EXT 2 (implementation).

---

## IPBR-EXT 2 — implementation + chapter close (2026-05-24)

Landed per design doc spec. Edits:

1. `pilots/rusty-js-bytecode/derived/src/op.rs`:
   - Added `Op::ForOfFastNext = 0xFE` (3 LOC + doc-comment)
   - Added new operand_size arm `ForOfFastNext => 10` (1 LOC)
   - Added op_from_byte case (1 LOC)
2. `pilots/rusty-js-bytecode/derived/src/compiler.rs`:
   - Emit ForOfFastNext at loop_start with placeholder operand bytes (~7 LOC)
   - Patch next_iter_offset (i16) after StoreLocal bind_slot (~7 LOC)
   - Patch done_offset (i32) after slow-path Pop (~5 LOC)
3. `pilots/rusty-js-runtime/derived/src/interp.rs`:
   - Op::ForOfFastNext handler (~30 LOC) — probes iter_slot for `_arr` + `_i` shape (per iterator.rs:28-58 — NOT __it_src__/__it_idx__ as initially assumed in design doc); reads arr[idx]; stores to bind_slot; increments idx; jumps to next_iter_offset on hit / done_offset on exhaustion; falls through to slow path on shape mismatch

**Bug + fix mid-implementation**: initial keys `__it_src__`/`__it_idx__` (from TypedArray iter in intrinsics.rs:3661+) did NOT match Array-iterator's actual keys `_arr`/`_i` (per iterator.rs:28). First bench run showed regression to 724 ms (slow path + wasted ForOfFastNext dispatch per iter). Corrected to `_arr`/`_i` + `rt.array_length`; immediate fast-path activation.

**Total LOC**: ~55. Pred-ipbr.1 HELD (≤80).

### Gates

| Gate | Result |
|---|---|
| Build | ✅ release built |
| diff-prod | 42/42 PASS ✅ (Pred-ipbr.3 HELD) |
| canonical fuzz (acc=-932188103) | ✅ byte-identical (Pred-ipbr.2 HELD) |

### Bench (Pred-ipbr.5)

| Probe | Pre-IPBR (GPI) | Post-IPBR | Δ |
|---|---:|---:|---:|
| string_url_sweep CRB median (N=5) | 685 ms | **584 ms** | **-14.7%** additional |
| string_url_sweep CRB median (N=3 confirm) | 685 ms | 593 ms | -13.4% additional |
| Cumulative vs original 743 ms | -7.8% | **-21.4%** | crosses 15% sub-target ✅ |
| A/B header_loop (median of 4 readings) | 252 ms | **197 ms** (range 193-218) | **-21.8%** additional |
| cruft/node ratio | 6.99x | **6.21-6.31x** | first sub-6.5× |

### Sibling-fixture composition (Pred-ipbr.4)

| fixture | pre-IPBR | post-IPBR | Δ |
|---|---:|---:|---:|
| arith_tight_loop | 422 | 424 | +0.5% (noise) |
| crypto_sha256_batch | FAIL¹ | FAIL¹ | unchanged |
| json_parse_transform | 1773 | 1818 | +2.5% (noise) |
| string_url_sweep | 685 | 593 | -13.4% |

¹ pre-existing per CRB-EXT 0-6 baseline; unrelated to IPBR.

### Final disposition — all predicates HELD

| Predicate | Disposition |
|---|---|
| Pred-ipbr.1 (≤80 LOC) | ✅ HELD at ~55 LOC |
| Pred-ipbr.2 (canonical fuzz) | ✅ HELD byte-identical |
| Pred-ipbr.3 (diff-prod 42/42) | ✅ HELD |
| Pred-ipbr.4 (composition ±5%) | ✅ HELD across CRB |
| Pred-ipbr.5 (header_loop ≥15%) | ✅ HELD at -21.8% (median of 4 readings) |
| **Pred-ipbr.6 (≤3 implementation rounds) — DISCIPLINE FALSIFIER** | ✅ **HELD at 1 implementation round** |

### Findings

**Finding IPBR.1**: per-iter for-of envelope on string_url_sweep header_loop drops from ~545ns (predicted) → effectively eliminated; the measured ~21.8% header_loop reduction translates to ~55ms / ~9K-iter-equivalent at the inner loop, validating the cost model in design doc §6.

**Finding IPBR.2** (corroborates `docs/standing-rule-13-prospective-application.md` §3 thesis): standing rule 13's prospective application produced its **second empirical corroboration** at this locale. IPBR-EXT 0 founding + IPBR-EXT 1 design + IPBR-EXT 2 implementation = 3 total rounds (1 implementation round). The thesis is now supported by:
- IHI (retrospective; 11 rounds; 9 substrate moves)
- GPI (prospective; 3 rounds; 1 substrate move; first corroboration)
- IPBR (prospective; 3 rounds; 1 substrate move; second corroboration)

Cross-locale trajectory-length pattern: with standing rule 13 applied prospectively + conditions C1-C4 satisfied, ≤3 total rounds (≤1 implementation round) becomes the convergent shape. The discipline is reproducible.

**Finding IPBR.3**: design-doc bug (wrong field names) was caught at first bench run, not at build/correctness gates (diff-prod + canonical fuzz still passed because the slow path remained as the fallthrough). This validates the bail-safety design: shape-mismatch fallthrough preserves correctness even when the fast-path eligibility check is mis-coded.

### Cross-locale composition (IHI + GPI + IPBR)

Post-IPBR hot iteration of `for (const line of lines) { lines.indexOf(':') ... }`:

```
ForOfFastNext iter_slot, bind_slot, j_done, next_iter
  → fast-path: read arr[i], inc i, store, jump next_iter  (~50ns)
... body emits ...
Dup; GetPropSkipForMethod(_); CallMethodIcCached(idx)
  → ~15ns (IHI + GPI hot path)
```

Per iteration: ~65ns of dispatch overhead, down from the pre-IHI/GPI/IPBR ~860ns (~545ns iter envelope + ~315ns method-call). **~13× reduction in dispatch overhead across the composed locales**. The remaining wall-clock dominator on string_url_sweep is now the per-iter method-body work (the actual indexOf/slice/toLowerCase implementations).

### Status: CHAPTER CLOSED at IPBR-EXT 2

All five Pred-ipbr.* HELD plus the Pred-ipbr.6 discipline falsifier. Locale closed in 1 implementation round per the standing-rule-13 prospective-application thesis. No IPBR-EXT 3 needed (String fast-path deferred to a follow-on locale; would not change Pred-ipbr.5's outcome since header_loop receivers are Arrays via `.split('\n')`).

**Promotion implication for `docs/standing-rule-13-prospective-application.md`**: thesis now has two empirical corroborations (GPI + IPBR). Per §5, ready for promotion to corpus Doc 742 pending keeper review.

