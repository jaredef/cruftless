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
