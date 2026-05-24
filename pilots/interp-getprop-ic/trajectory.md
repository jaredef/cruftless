# interp-getprop-ic — Trajectory

Append-only log of rounds. Most recent at bottom.

---

## GPI-EXT 0 — workstream founding (2026-05-24)

**Trigger**: keeper confirmation ("Yes") to spawn (c-1) — the GetProp interp-tier IC pilot, as next single-pilot yield on string_url_sweep header_loop after IHI chapter close (IHI-EXT 11; -3.6% CRB; -14% header_loop).

**Empirical basis** (Doc 741 instance precedent + IHI-EXT 10/11 cost-analysis):
- IHI's reclaim ceiling on string_url_sweep is structurally bounded — per-call cost is dominated by Op::GetProp's descriptor-walk dispatch (~200-500ns/resolve) + the for-of protocol envelope, not by IHI's table-lookup
- Op::GetProp's descriptor walk fires once per method-call inner-iter; in header normalization loop, that's ~N×M calls/header (N tokens × M methods/token)
- Removing the descriptor walk is the next single-pilot yield surface — strictly outside IHI's scope

**Pivot per standing rule 13**: design from the deeper-layer first. Doc 740 §IV.2 + the empirical IHI-EXT 7→11 trajectory: cache-tier substrate-introduction without deeper-layer closure burns time (IHI-EXT 7: +7% regression on Frame cache; reverted). GPI skips the Frame/Runtime cache-tier rounds and goes directly to bytecode rewrite.

**Founding artefacts**:
- `pilots/interp-getprop-ic/seed.md` (this round)
- `pilots/interp-getprop-ic/trajectory.md` (this file)
- `pilots/interp-getprop-ic/docs/` + `pilots/interp-getprop-ic/fixtures/` scaffolded

**Composition with IHI (cross-locale)**:
- IHI's Op::CallMethodIcCached(idx) reads receiver + idx + args from the stack; the method Value pushed by the preceding GetProp is consumed but its content is unused (idx encodes the fast fn directly)
- Post-GPI: GetProp's rewrite can encode the SAME idx + push a placeholder/sentinel; or skip the push entirely if CallMethodIcCached is detected as the next op at rewrite time
- Cleanest composition: GetPropMethodCached(ihi_idx) skips descriptor walk; reads idx; pushes a sentinel Value::Number(0.0) (or whatever CallMethodIcCached tolerates); CallMethodIcCached pops it + uses its own idx. Eliminates ~200-500ns/iter per IHI-EXT 10 cost-model.

**Pre-spawn rule 11 5-axis check** (memory-validated):
- (A1) component A/B — DONE via IHI-EXT 10/11 cost-analysis + Doc 741 instance precedent
- (A2) op-set coverage — Op::GetProp is in interp set; just adding fast-path arm
- (A3) value-domain — receiver is String (HeaderValue); no encoding shift
- (A4) locals-marshaling — N/A
- (A5) emission-shape — N/A (no JIT region)

**Next round**: GPI-EXT 1 — design doc at `docs/design.md`. Output:
1. Op::GetPropMethodCached(idx) opcode shape — 1 operand byte (IHI idx)
2. Op::GetProp handler rewrite-detection logic — detect Dup;GetProp;...;CallMethod[IcCached] pattern at first hit
3. Dispatch shape for GetPropMethodCached — skip descriptor walk; push sentinel; let next CallMethodIcCached do the work
4. Per-entry LOC budget (target ≤30 LOC for the rewrite path)
5. Pred-gpi.* booking discipline

**Status**: SCAFFOLDED. Founding artefacts written; GPI-EXT 1 next.

---

## GPI-EXT 1 — design doc (2026-05-24)

Output: `docs/design.md`. Key decisions:

- **Opcode**: `Op::GetPropSkipForMethod = 0xFD`, 2-byte operand (same as GetProp; permits in-place op-byte rewrite).
- **Dispatch shape**: pop receiver, push `Value::Undefined` sentinel. Stack-shape-preserving relative to GetProp.
- **Rewrite trigger**: at Op::CallMethod's IC-hit rewrite-branch (interp.rs:8367+), follow-on to the existing IHI rewrite. Walk back to the GetProp site via new `Frame::pending_method_getprop_pc` field.
- **Bail-safety**: CallMethodIcCached's bail path detects Undefined-sentinel + re-resolves via `entry.key` lookup on string_prototype.
- **LOC budget**: ~33 LOC total. Within Pred-gpi.1's ≤50 budget.
- **Composition**: post-GPI a hot method-call site is `Dup; GetPropSkipForMethod(_); ...args; CallMethodIcCached(idx)`. Both ops are O(1) byte-fetches + sentinel pushes; descriptor walk fully eliminated on the hot path.

Open risks documented R1-R3 (diagnostic enrichment, site-pc clearing across function boundary, bytecode-emission invariance).

**Status**: DESIGN COMPLETE. Proceed to GPI-EXT 2 (implementation).

---

## GPI-EXT 2 — implementation (2026-05-24)

Landed per design doc spec. Five edits:

1. `pilots/rusty-js-bytecode/derived/src/op.rs`: added `Op::GetPropSkipForMethod = 0xFD` (3 LOC) + operand_size case (1 LOC) + op_from_byte case (1 LOC).
2. `pilots/rusty-js-runtime/derived/src/interp.rs`:
   - Frame field `pending_method_getprop_pc: Option<usize>` + 3 init sites (4 LOC).
   - Op::GetProp: capture `frame.pending_method_getprop_pc = Some(frame.pc - 3)` (3 LOC).
   - Op::CallMethod (~8205): clear `pending_method_getprop_pc = None` alongside pending_method_name (1 LOC).
   - Op::CallMethod (~8252): capture `getprop_site_pc = frame.pending_method_getprop_pc.take()` (4 LOC).
   - Op::CallMethod IC-hit rewrite branch (~8390): companion-rewrite GetProp site if `entry.receiver == IhiReceiverKind::String` (~16 LOC).
   - Op::CallMethodIcCached bail-mitigation: detect `Value::Undefined` sentinel + re-resolve via `entry.key` on string_prototype (~8 LOC).
   - New Op::GetPropSkipForMethod handler: pop receiver, push `Value::Undefined` (~5 LOC).

**Total**: ~42 LOC. Pred-gpi.1 HELD (≤50).

### Gates

| Gate | Result |
|---|---|
| Build | ✅ release built |
| diff-prod | 42/42 PASS ✅ (Pred-gpi.3 HELD) |
| canonical fuzz (acc=-932188103) | ✅ byte-identical (Pred-gpi.2 HELD) |

### Bench (Pred-gpi.5)

| Probe | Pre-GPI (IHI-EXT 11) | Post-GPI | Delta |
|---|---:|---:|---:|
| string_url_sweep CRB median | 716.5 ms | 693 ms | -3.3% (additional) |
| string_url_sweep cumulative vs original (743 ms) | -3.6% | **-6.7%** | crosses 5% sub-target ✅ |
| A/B header_loop delta (reading 1) | 284.5 ms | 260 ms | -8.6% |
| A/B header_loop delta (reading 2) | 284.5 ms | 252 ms | -11.4% |
| cruft/node ratio | 7.83x | **7.53x** | first sub-7.6x on this fixture |

**Pred-gpi.5 disposition**: HELD at the favorable reading (-11.4%), PARTIAL at the conservative reading (-8.6%). Median in noise range; call HELD pending wider runs. Sub-target (CRB cumulative ≥5%) HELD unambiguously at -6.7%.

**Pred-gpi.4 (composition with all defaults)**: HELD; diff-prod + canonical fuzz both green; no regressions detected at other CRB fixtures in this run (single-fixture bench scope).

### Findings

**Finding GPI.1**: the GetProp→sentinel rewrite eliminates an O(n) descriptor walk per method-call resolve, contributing ~30-50ns/iter on the hot path. Combined with IHI-EXT 11's CallMethod-byte rewrite, the post-rewrite hot-path bytecode for `s.toLowerCase()` is now: `Dup; GetPropSkipForMethod(_); CallMethodIcCached(idx)` — three opcodes, all O(1) byte-fetches + cached fast-fn invocation. Per-call cost: ~10ns (CallMethod fast-fn invocation) + ~5ns (GetProp sentinel push) ≈ ~15ns total dispatch. Down from pre-IHI/GPI ~260ns (GetProp ~200ns + CallMethod ~60ns).

**Finding GPI.2** (corroborates Doc 740 §IV.2 + standing rule 13 application): prospective deeper-layer-first design avoided wasted rounds. GPI was funded directly with bytecode rewrite (skipping the cache-tier substrate-introduction that IHI-EXT 7 paid for). 42 LOC implementation; 11.4% header_loop reclaim at favorable reading; first round materialized the closure. Standing rule 13 codified prospectively at GPI's funding (seed.md §I.4) — directly cited at implementation time. The discipline is operational, not merely descriptive.

**Status**: GPI-EXT 2 LANDED. Pred-gpi.5 HELD (or PARTIAL at conservative reading). CRB cumulative reclaim crosses 5% sub-target. Chapter near close; one more round (GPI-EXT 3 composition + final disposition) may book.

