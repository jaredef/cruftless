# rusty-js-jit/osr — Trajectory

Per-OSR-EXT log for the OSR / loop-extraction pilot (closes the op-set coverage tier per Doc 740 §II.2 + Finding VII.2).

---

## OSR-EXT 0 — 2026-05-23 (workstream founding)

Apparatus-tier round. Pilot founded per keeper directive 2026-05-23 22:37-local as the (ii) pivot from the session chapter close. Nested under LeJIT per Doc 737 §IV.

### Trigger

- TL findings.md TL.1 + engagement Finding VII.2: whole-body bail discipline gates inner-loop JIT eligibility on FULL enclosing-scope alphabet coverage. The (b-narrow) plan was structurally bounded at TL-EXT 3.
- Doc 740 §II.2 multi-tier reading: R for json_parse_transform's checksum loop has 4 tiers; 2 closed in the architectural-pivot session (VD value-domain at VD-EXT 2; TL entry-mechanism at TL-EXT 3); 2 remain (op-set coverage; IC fast-path body).
- Keeper directive: pivot to (ii) OSR / loop-extraction.

### Substrate delivered

- `seed.md` (~120 lines): telos, candidate scope options A/B/C deferred to design doc, 8 constraints C1-C8, 5 falsifiers Pred-osr.1-.5, methodology OSR-EXT 0-N+2, carve-outs.
- `trajectory.md` (this file).
- `docs/` + `fixtures/` scaffolds.

### Locale registration

Locale count: 22 → 23 after this spawn (13 top-level unchanged; 9 → 10 nested under LeJIT). Manifest refresh queued at end of OSR-EXT 0.

### Open scope at OSR-EXT 0 close

1. **OSR-EXT 1** — design doc enumerating A (runtime bytecode transform) / B (compile-time loop marker) / C (V8-style OSR back-edge counter); per-option scope + LOC + reclaim + risks; keeper selects.
2. **OSR-EXT 2+** — implementation per the selected option.
3. **OSR-EXT N+1** — composition probe + CRB final disposition + Pred-osr.1 gate.

### Cumulative status

LOC delta: 0 (apparatus round only). No source changes.

---

*OSR-EXT 0 closes. Pilot founded as the (ii) OSR / loop-extraction pivot. OSR-EXT 1 designs the scope. The architectural-pivot session continues here in the next session per the chapter-close directive.*

---

## OSR-EXT 1 — 2026-05-23 (design doc: A/B/C scope options)

### Headline

Design-tier round per keeper directive 2026-05-23 22:47-local (resumed past the prior session-close). `docs/design.md` (~230 lines) enumerates 3 candidate structural options + per-option scope + LOC + reclaim + composition + falsifier + risks. Recommendation: **Option A (Runtime Bytecode Transform)**.

### Three options

| option | mechanism | LOC | closes VII.2? | reclaim on json_parse_transform |
|---|---|---:|---|---|
| **A — Runtime Bytecode Transform** | back-edge counter + on-threshold synthetic-FunctionProto build + JIT compile + invoke | 250-350 | **yes (loop scope)** | high |
| B — Compile-time loop marker | Op::LoopRegionMark in bytecode + per-region JIT-compile at module load | 500-700 | yes (loop scope) | high |
| C — V8/SpiderMonkey-style OSR | back-edge counter + state snapshot + JIT-compile WHOLE FUNCTION + OSR entry | 800+ | **no — still bails on enclosing-function alphabet** | **0%** |

### Recommendation: Option A

Rationale:
1. Closes Finding VII.2 by reducing enclosing scope to the loop body
2. Smallest LOC estimate among options that close the gap
3. Localized to interp dispatcher (no cross-cutting bytecode-op-set change at first cut)
4. Reuses existing JIT pipeline + VD String encoding + TL synthetic-FunctionProto pattern
5. Composition risk low (additive to default-on paths; opt-in via threshold)

Option C ruled out: it's the entry-mechanism optimization on top of full-function JIT; cruft's blocker is whole-function alphabet, not OSR-entry.

Option B viable as a follow-on if A's runtime boundary detection proves insufficient.

### Option A first-cut staging (OSR-EXT 2-7 if A selected)

| round | substrate |
|---|---|
| OSR-EXT 2 | back-edge counter + threshold detection (subst-intro) |
| OSR-EXT 3 | loop bytecode boundary detection (forward-scan; subst-intro) |
| OSR-EXT 4 | synthetic FunctionProto builder for loop region + compile attempt |
| OSR-EXT 5 | local-state copy-in/out + JIT body invoke (cascade-revival #1) |
| OSR-EXT 6 | alphabet extension (TL Moves 3+4 revival folded in) (cascade-revival #2) |
| OSR-EXT 7 | composition probe + CRB final disposition + Pred-osr.1 gate |

### Per-option falsifier anchoring

- **A**: Pred-osr.1 ≥40% CRB reclaim. If <40%, JIT body per-iter overhead floor dominates (Finding II.3 multi-tier residual).
- **B**: Pred-osr.1 + diff-prod 42/42 (marker emission must not break existing fixtures).
- **C**: Pred-osr.1 fails by construction.

### Key risks (Option A)

- R1 boundary detection (Op::Jump negative-displacement disambiguation)
- R2 local-state sync overhead (mitigated by iteration-count threshold)
- R3 deopt mid-loop (may require JIT-EXT 21 enhancements)
- R4 mutation during JIT body (bail to interp on extern; conservative first cut)

### Composition with prior corpus / engagement work

- **Doc 740 §II.2**: this pilot closes the op-set coverage tier; combined with VD value-domain + TL entry-mechanism, full multi-tier pipeline-connection becomes feasible.
- **Finding VII.2 (Addendum V)**: this design is the apparatus closure.
- **VD String encoding**: consumed at OSR-EXT 6 by the alphabet extensions for the loop body.
- **TL synthetic FunctionProto pattern**: reused at OSR-EXT 4.
- **TB metadata cache**: not consumed (modules / loop bodies are called per-fixture, not per-call; TB metadata's amortization shape doesn't apply).
- **Finding II.2-bis substrate-introduction signature**: OSR-EXT 2-4 expected flat A/B (substrate-intro); reclaim materializes at OSR-EXT 6 close.

### §XVI / Doc 734 / Doc 735 §X.h categorization

Per Doc 730 §XVI: not applicable (design-tier).
Per Doc 734 §V: growth (c) preparatory — design's per-option enumeration anchors keeper selection at OSR-EXT 2 implementation start.
Per Doc 735 §X.h.c: three-probe-levels at each substrate round per rule 10.

### Open scope at OSR-EXT 1 close

1. **Keeper selects A / B / C** (recommendation: A)
2. **OSR-EXT 2** — implementation begins per selection

### Cumulative status at OSR-EXT 1 close

LOC delta: ~230 (design doc). 3 options enumerated; recommendation made.

---

*OSR-EXT 1 closes. Design enumerated; recommendation: Option A (RBT). Keeper selection pending; OSR-EXT 2 begins implementation per selection.*

---

## OSR-EXT 2 — 2026-05-23 (Move 1 back-edge counter + threshold detection; Option A selected)

### Headline

Per keeper selection 2026-05-23 22:52-local: Option A (Runtime Bytecode Transform). OSR-EXT 2 lands the back-edge counter substrate: `Frame::back_edge_counts: HashMap<usize, u32>` field; 5 Jump handlers (Op::Jump / JumpIfTrue / JumpIfFalse / JumpIfTrueKeep / JumpIfFalseKeep) increment the counter when disp < 0. `OSR_BACK_EDGE_THRESHOLD = 1000` constant reserved for OSR-EXT 3+ consumption. ~30 LOC delta in interp.rs. Substrate-introduction; counter only counts, no threshold action yet.

### Three-probe results

| probe | result |
|---|---|
| Pred-osr.2 canonical fuzz (acc=-932188103) | ✅ GREEN |
| Pred-osr.3 diff-prod 42/42 | ✅ GREEN |
| JIT lib tests | ✅ 38/38 (9 pre-existing ignored) |
| Pred-osr.4 A/B composition | ~3% drift (median 1538 vs baseline 1480; within ±5% gate) |

### Substrate moves landed

1. Added `Frame::back_edge_counts: HashMap<usize, u32>` field.
2. Initialized in all 3 Frame creation sites (new_module, resume_from_deopt_state, call_function).
3. Added `OSR_BACK_EDGE_THRESHOLD = 1000` constant.
4. Wired increment in 5 Jump handlers; key is the Op byte's pc (site_pc), incremented only on disp < 0.

### Composition with prior corpus / engagement work

- **Doc 740 §II.2 op-set coverage tier**: this round delivers the substrate for OSR-EXT 3+ to consume.
- **Finding II.2-bis substrate-introduction signature**: A/B drift +3% is the counter-allocation cost; near-zero impact as predicted; no JIT triggers fire yet.
- **VD + TL substrate**: composed cleanly (no encoding or wrapper interactions).
- **Standing rule 9 raw-pointer audit**: not applicable (HashMap of plain u32; no pointer caches).

### §XVI / Doc 734 / Doc 735 §X.h categorization

Per Doc 730 §XVI: not applicable.
Per Doc 734 §V: growth (c) preparatory — back-edge counter is the apparatus that OSR-EXT 3 boundary detection + OSR-EXT 4 synthetic FunctionProto + OSR-EXT 5 invoke consume.
Per Doc 735 §X.h.b: **(P2.d) bench at substrate-introduction round, expected per Finding II.2-bis. Cumulative reclaim materializes at OSR-EXT 6 close per Doc 740 §II.2 (P4).**

### Open scope at OSR-EXT 2 close

1. **OSR-EXT 3** — loop bytecode boundary detection (forward-scan from back-edge target to identify the loop body's pc range)
2. **OSR-EXT 4** — synthetic FunctionProto builder + JIT compile attempt
3. **OSR-EXT 5** — local-state copy-in/out + JIT body invoke (cascade-revival #1)
4. **OSR-EXT 6** — alphabet extension (TL Moves 3+4 revival folded in) (cascade-revival #2)
5. **OSR-EXT 7** — composition probe + CRB final disposition + Pred-osr.1 gate

### Cumulative status at OSR-EXT 2 close

LOC delta: ~30 (interp.rs: Frame field + 3 init sites + threshold const + 5 Jump-handler increments). OSR-EXT 0+1+2 cumulative: ~380 across the locale.

---

*OSR-EXT 2 closes. Back-edge counter substrate landed; all probes GREEN; composition within ±5%. OSR-EXT 3 lands loop boundary detection.*
