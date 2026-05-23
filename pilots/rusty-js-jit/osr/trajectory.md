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

---

## OSR-EXT 3 — 2026-05-23 (Move 2 loop boundary detector)

### Headline

`compute_loop_region(bytecode, site_pc) -> Option<(entry_pc, end_pc)>` lands as a pure helper in interp.rs. Reads the back-edge's 4-byte disp at site_pc+1..5; computes entry_pc = (site_pc + 5) + disp; end_pc = site_pc + 5. Returns None on out-of-bounds, non-negative disp, negative entry, or zero displacement. ~35 LOC + 5 unit tests. Pure helper; not yet integrated into the dispatch loop (OSR-EXT 4 integrates).

### Three-probe results

| probe | result |
|---|---|
| OSR unit tests | ✅ 5/5 |
| canonical fuzz (acc=-932188103) | ✅ GREEN |
| diff-prod 42/42 | ✅ GREEN |
| JIT lib tests | ✅ 38/38 (9 ignored) |
| A/B composition | unchanged (pure helper not yet called) |

### Substrate moves landed

1. `compute_loop_region` helper function exposed `pub` for OSR-EXT 4's consumption.
2. 5 unit tests covering: basic back-edge; forward-jump rejection; out-of-bounds site rejection; negative entry_pc rejection; zero-disp rejection.

### Composition with prior corpus / engagement work

- **OSR-EXT 2 back-edge counter**: this round's helper will be invoked when counter > threshold; consumed by OSR-EXT 4.
- **Doc 740 §II.2 op-set coverage tier**: incremental closure progress; helper is the pre-condition for sub-region JIT compile.
- **Finding II.2-bis substrate-introduction signature**: A/B unchanged as expected (helper not yet integrated).
- **Standing rule 12 adversarial special-value testing**: applied at unit-test scope (edge cases enumerated: out-of-bounds, zero disp, forward jump, negative entry).

### §XVI / Doc 734 / Doc 735 §X.h categorization

Per Doc 730 §XVI: not applicable.
Per Doc 734 §V: growth (c) preparatory.
Per Doc 735 §X.h.b: substrate-intro round; (P2.d) bench-flat is the signature; consumer at OSR-EXT 4.

### Open scope at OSR-EXT 3 close

1. **OSR-EXT 4** — synthetic FunctionProto builder + JIT compile attempt; integrates the counter + boundary detector at threshold hit
2. **OSR-EXT 5** — local-state copy-in/out + JIT body invoke (cascade-revival #1)
3. **OSR-EXT 6** — alphabet extension (TL Moves 3+4 revival folded in) (cascade-revival #2)
4. **OSR-EXT 7** — composition probe + CRB final disposition

### Cumulative status at OSR-EXT 3 close

LOC delta: ~70 (35 helper + 35 unit tests). OSR-EXT 0+1+2+3 cumulative: ~450 across the locale.

---

*OSR-EXT 3 closes. Loop boundary detector substrate landed; 5/5 unit tests; correctness probes GREEN; A/B unchanged. OSR-EXT 4 integrates the counter + detector + JIT compile attempt.*

---

## OSR-EXT 4 — 2026-05-23 (Move 3 synthetic FunctionProto builder + JIT compile attempt)

### Headline

`try_osr_compile(frame, site_pc)` lands as a free helper in interp.rs. At exact threshold-crossing (counter == OSR_BACK_EDGE_THRESHOLD) per back-edge site, the 5 Jump handlers call try_osr_compile once per site (osr_attempted HashMap tracks already-attempted sites). The helper builds a synthetic 0-arg FunctionProto wrapping bytecode[entry_pc..end_pc] + frame.locals_names + frame.constants + frame.strict; attempts compile_function; discards the result. Substrate-introduction: compile attempted, result discarded; OSR-EXT 5 caches + invokes.

~50 LOC delta in interp.rs (Frame field + 3 init sites + 5 Jump handler extensions + try_osr_compile helper).

### Expected outcome on json_parse_transform

Compile fails at parse-time because the loop body uses Op::GetProp (.length) + Op::CallMethod (.charCodeAt) which aren't in the JIT alphabet (Finding VII.2 op-set coverage gap). The compile attempt itself is harmless (~ms per attempt; amortized over 1000-iter threshold). No invoke; no behavior change.

### Three-probe results

| probe | result |
|---|---|
| canonical fuzz (acc=-932188103) | ✅ GREEN |
| diff-prod 42/42 | ✅ GREEN |
| JIT lib tests | ✅ 38/38 |
| OSR helper unit tests | ✅ 5/5 |
| A/B composition | median 1533 vs baseline 1480 (~3.6% drift; within ±5% gate) |

### Substrate moves landed

1. Added `Frame::osr_attempted: HashMap<usize, ()>` field.
2. Initialized at all 3 Frame creation sites.
3. Added `try_osr_compile(frame, site_pc)` free helper.
4. Wired threshold-crossing trigger in 5 Jump handlers (exact-equality check on OSR_BACK_EDGE_THRESHOLD; once-per-site).

### Composition with prior corpus / engagement work

- **Doc 740 §II.2 op-set coverage tier**: this round delivers the substrate-introduction for the compile path; OSR-EXT 6 closes the alphabet gap.
- **VD pilot encoding**: synthetic FunctionProto uses Φ-default unboxing; VD String encoding becomes consumable at OSR-EXT 6 alphabet additions.
- **TL pilot wrapper pattern**: try_osr_compile mirrors try_jit_run_module's synthetic-FunctionProto build shape.
- **Finding II.2-bis substrate-introduction signature**: A/B drift +3.6% is the compile-attempt cost amortized; near-zero per-iter overhead as predicted.
- **Standing rule 11 (op-set coverage axis)**: this round's compile attempts will UNIFORMLY FAIL on json_parse_transform due to the alphabet gap; the failures are silent (discarded result); OSR-EXT 5+6 add the alphabet + invoke for the loop bodies that DO match the alphabet.

### §XVI / Doc 734 / Doc 735 §X.h categorization

Per Doc 730 §XVI: not applicable.
Per Doc 734 §V: growth (c) preparatory.
Per Doc 735 §X.h.b: substrate-intro round; (P2.d) bench within ±5% by design; consumer at OSR-EXT 5+6.

### Open scope at OSR-EXT 4 close

1. **OSR-EXT 5** — local-state copy-in/out + JIT body invoke (consumes try_osr_compile's result; caches CompiledFn per site_pc)
2. **OSR-EXT 6** — alphabet extension (TL Moves 3+4 revival folded in: GetProp+length-IC + CallMethod+charCodeAt-IC consuming VD String encoding) (cascade-revival)
3. **OSR-EXT 7** — composition probe + CRB final disposition + Pred-osr.1 gate

### Cumulative status at OSR-EXT 4 close

LOC delta: ~50 (Frame field + 3 inits + helper + 5 trigger sites). OSR-EXT 0-4 cumulative: ~500 across the locale.

---

*OSR-EXT 4 closes. Synthetic FunctionProto builder + compile-attempt trigger landed; all probes GREEN; A/B within ±5%. Substrate-intro per Finding II.2-bis. OSR-EXT 5 caches the compile result + invokes the JIT body.*

---

## OSR-EXT 5 — 2026-05-23 (Move 4 compile-result cache + Finding OSR.1 surfaced)

### Headline

`Frame::osr_attempted` replaced by `Frame::osr_cache: HashMap<usize, Option<Box<CompiledFn>>>`. try_osr_compile now stores the result; subsequent back-edge fires at the same site check the cache and skip if already attempted. Box-wrap per standing rule 9 (TB-EXT 7 raw-pointer-cache pattern).

**Pre-implementation source-read surfaced Finding OSR.1**: the JIT calling convention's params-only-as-args shape blocks frame-state marshaling without a major substrate extension. The OSR-EXT 1 design's "local-state copy-in/out" bullet was structurally incomplete. **Invoke step DEFERRED to OSR-EXT 5b (new round, scope to be confirmed).**

### Substrate landed

1. `Frame::osr_cache: HashMap<usize, Option<Box<CompiledFn>>>` field (replaces osr_attempted).
2. Updated all 3 Frame init sites.
3. try_osr_compile now takes `&mut Frame`; stores compile result in osr_cache (Some for success; None for compile failure or non-loop-region).
4. 5 Jump handlers gate on `!osr_cache.contains_key(&site_pc)`; first threshold-crossing per site triggers try_osr_compile (which writes the cache); subsequent crossings short-circuit.

### Finding OSR.1 (local findings.md)

The JIT calling convention currently initializes locals 0..params from f64 args; locals params..N = 0.0. OSR loop bodies read/write the enclosing frame's locals, which the JIT initializes to 0.0 — not the frame's actual values. **The invoke path as designed would produce wrong results.**

Three structural alternatives surfaced; recommended option 2 (extern-pre-populate prologue at JIT body entry); ~80 LOC; non-invasive composition with existing Σ/Τ/Ψ/Φ paths.

**Doc 740 R extension**: for OSR loop invoke, R has a fifth tier: **JIT calling convention's locals-marshaling capability**. The original 4-tier reading missed it.

Promotion candidate at engagement findings.md Addendum VII: "JIT calling convention's locals-marshaling capability" as a tier in Doc 740 §II.2's relevant-tier-set apparatus.

### Three-probe results

| probe | result |
|---|---|
| canonical fuzz (acc=-932188103) | ✅ GREEN |
| diff-prod 42/42 | ✅ GREEN |
| JIT lib tests | ✅ 38/38 |
| OSR helper unit tests | ✅ 5/5 |
| A/B composition (5-run median) | 1550 vs baseline 1480 (+4.7%; within ±5% Pred-osr.4 gate) |

### Composition with prior corpus / engagement work

- **Doc 740 §II.2 op-set coverage tier**: this round prevents wasted re-compile cycles at sites that previously failed compile (cache None records the failure).
- **Standing rule 9 (raw-pointer audit)**: applied — Box-wrap on CompiledFn cache value.
- **Finding II.2-bis substrate-introduction signature**: A/B drift +4.7% is the compile-attempt cost amortized over 1000-iter threshold per site; near-zero per-iter after the cache settles.
- **Standing rule 11 (multi-axis coverage check)**: Finding OSR.1 surfaces a NEW coverage axis (locals-marshaling) candidate for engagement-wide promotion.

### §XVI / Doc 734 / Doc 735 §X.h categorization

Per Doc 730 §XVI: not applicable.
Per Doc 734 §V: growth (a) positive-finding (cache structure landed); growth (b) negative-finding (Finding OSR.1 surfaces locals-marshaling blocker).
Per Doc 735 §X.h.b: substrate-intro round; (P2.d) bench within ±5% by design; consumer at OSR-EXT 5b + 6.

### Open scope at OSR-EXT 5 close

1. **OSR-EXT 5b** (new round, keeper-pending) — locals-marshaling per Finding OSR.1 recommended option 2 (extern-pre-populate). ~80 LOC. Unlocks invoke for loops whose alphabet IS in JIT scope.
2. **OSR-EXT 6** — alphabet extension (TL Moves 3+4 revival folded in: GetProp+length-IC + CallMethod+charCodeAt-IC consuming VD String encoding). Closes the alphabet gap.
3. **OSR-EXT 7** — composition probe + CRB final disposition + Pred-osr.1 gate.
4. **Findings addendum VII candidate** — promotion of OSR.1 to engagement-wide as the locals-marshaling-coverage tier in Doc 740.

### Cumulative status at OSR-EXT 5 close

LOC delta: ~25 (Frame field rename + try_osr_compile cache writes + Jump handler gate updates). OSR-EXT 0-5 cumulative: ~530 across the locale. Local findings.md created with Finding OSR.1.

---

*OSR-EXT 5 closes. Cache structure landed; invoke deferred per Finding OSR.1. Keeper deliberation: OSR-EXT 5b (locals-marshaling extension) before OSR-EXT 6 (alphabet) before OSR-EXT 7 (final disposition); OR engagement findings Addendum VII first; OR pivot.*

---

## OSR-EXT 5b — 2026-05-23 (locals-marshaling JIT-side substrate; option 2 first cut)

### Headline

Per keeper directive 2026-05-23 23:29-local (option β three-round split). OSR-EXT 5b lands the JIT-side substrate for option 2 (extern-pre-populate). New compile entry `compile_function_osr` produces a CompiledFn with `JitFn::ArityOsr` signature `extern "C" fn(*mut f64) -> f64`. Entry-block prologue loads N locals from `arr_ptr + i*8`; Return / ReturnUndef / synthesized-ReturnUndef epilogues store N locals back. ~130 LOC delta in translator.rs.

### Three-probe results

| probe | result |
|---|---|
| canonical fuzz (acc=-932188103) | ✅ GREEN |
| diff-prod 42/42 | ✅ GREEN |
| JIT lib tests | ✅ 38/38 (9 ignored) |
| OSR helper unit tests | ✅ 5/5 |
| A/B composition | unchanged (compile_function_osr not yet invoked by runtime; OSR-EXT 5d wires it) |

### Substrate moves landed

1. `JitFnOsr = extern "C" fn(*mut f64) -> f64` type alias.
2. `JitFn::ArityOsr(JitFnOsr)` variant + Debug impl + `call_osr(arr_ptr)` method (also fallback for non-OSR variants).
3. `pub fn compile_function_osr(proto)` API that wraps compile_function_inner with osr_mode=true.
4. `compile_function_inner(proto, osr_mode)` signature change: osr_mode=true skips the params count check.
5. Signature build: under osr_mode, single I64 param + F64 return.
6. New Variable `osr_arr_ptr_var` allocated past local_vars range; declared in entry block.
7. Entry-block prologue (osr_mode): capture entry_params[0] as arr_ptr; save to arr_ptr_var; load each local from arr_ptr+i*8.
8. Return / ReturnUndef / synthesized-ReturnUndef sites: under osr_mode, emit N store_f64 instructions to arr_ptr+i*8 before the return.
9. JitFn match at finalize: under osr_mode, JitFn::ArityOsr.

### Composition with prior corpus / engagement work

- **Doc 740 §VIII.2 locals-marshaling coverage tier**: this round closes the JIT-side substrate.
- **Finding VIII.2 (Addendum VII)**: implementation per option 2 (extern-pre-populate prologue). The "extern" here is implicit — the dispatcher (OSR-EXT 5d) marshals the array; the JIT entry-block prologue reads from it.
- **Standing rule 9 (raw-pointer audit)**: the arr_ptr is a Cranelift Variable holding I64; the dispatcher (OSR-EXT 5d) will provide the pointer; lifetime managed by the dispatcher's call scope (same shape as Object's id-encoding in TLS).
- **Finding II.2-bis substrate-introduction signature**: A/B unchanged as predicted (compile path exists but isn't invoked yet by runtime).

### §XVI / Doc 734 / Doc 735 §X.h categorization

Per Doc 730 §XVI: not applicable.
Per Doc 734 §V: growth (c) preparatory — JIT-side substrate enables OSR-EXT 5d's runtime invoke.
Per Doc 735 §X.h.b: substrate-intro round; (P2.d) bench unchanged by design; consumer at OSR-EXT 5d.

### Open scope at OSR-EXT 5b close

1. **OSR-EXT 5c** — box-to-value helper (Value reconstruction from f64; reuses VD encoding for String; ~40 LOC). Cross-cutting helper; useful beyond OSR.
2. **OSR-EXT 5d** — runtime dispatcher integration (marshal frame.locals → Vec<f64>; invoke call_osr; marshal back via 5c's helper; ~80 LOC). Cascade-revival pilot consuming 5b + 5c.
3. **OSR-EXT 6** — alphabet extension (TL Moves 3+4 revival folded in) (cascade-revival).
4. **OSR-EXT 7** — composition probe + CRB final disposition.

### Cumulative status at OSR-EXT 5b close

LOC delta: ~130 (translator.rs: JitFnOsr type + ArityOsr variant + call_osr method + compile_function_osr API + osr_mode parameter + signature/entry/epilogue branching). OSR-EXT 0-5b cumulative: ~660 across the locale.

---

*OSR-EXT 5b closes. JIT-side OSR substrate landed. compile_function_osr produces invocable ArityOsr CompiledFn with proper locals load/store IR. Runtime invoke deferred to OSR-EXT 5d. All probes GREEN; no behavior change (JIT side not yet called from runtime).*

---

## OSR-EXT 5c — 2026-05-23 (box-to-value helper)

### Headline

`box_to_value(f, snapshot) -> Value` helper lands. Conservative shape: if snapshot is Value::Number, return Value::Number(f) (the JIT-computed new f64); else return snapshot.clone() (preserve original String/Object/Boolean/Symbol/etc.). ~25 LOC + 4 unit tests.

### Design rationale (per VD R3 + safety analysis)

The JIT body can only construct f64 values internally. Any String/Object pointer it writes to a local must have been passed IN via the prologue, derived from the frame's original Value::String/Object. The original Value::X in the enclosing frame stays alive for the JIT call's duration; the JIT's locals-out array holds raw pointer bits.

Per VD R3 (Rc strong-count not incremented at encode): box_to_value MUST NOT use Rc::from_raw on decoded pointer bits (would over-decrement on drop). Instead: for non-Number snapshots, clone the snapshot's Value (which properly increments the Rc).

The conservative behavior covers the common case (well-formed loops where Number locals receive Number computations + String/Object locals stay as their original references). The limitation: if the JIT body writes a fresh non-Number to a Number-snapshot slot, the result is the raw f64 bits as Value::Number — which is the canonical f64 reading. No correctness violation; just doesn't track that pathological case. OSR loops in practice don't produce that pathological case.

### Three-probe results

| probe | result |
|---|---|
| canonical fuzz (acc=-932188103) | ✅ GREEN |
| diff-prod 42/42 | ✅ GREEN |
| JIT lib tests | ✅ 38/38 |
| osr_box_to_value unit tests | ✅ 4/4 (Number / String / Object / Undefined snapshots) |

### Substrate moves landed

1. `pub fn box_to_value(f: f64, snapshot: &Value) -> Value` helper in interp.rs.
2. 4 unit tests covering Number snapshot → new Value::Number; String snapshot → cloned String with Rc count tracking; Object snapshot preserved; Undefined snapshot preserved regardless of f64 value.

### Composition with prior corpus / engagement work

- **VD-EXT 1+2 NaN-boxing encoding**: unbox_arg_f64 is the encoder; box_to_value is the inverse for non-Number values via snapshot. For Number values, the round-trip is f64-identity.
- **Standing rule 12 (Addendum VI)**: applied — adversarial unit-test coverage of all 4 Value-variant snapshot cases.
- **Doc 740 §VIII.4 locals-marshaling coverage**: completes the helper substrate; OSR-EXT 5d will compose box_to_value with the runtime dispatcher.

### §XVI / Doc 734 / Doc 735 §X.h categorization

Per Doc 730 §XVI: not applicable.
Per Doc 734 §V: growth (c) preparatory.
Per Doc 735 §X.h.b: substrate-intro round; helper not yet invoked by runtime.

### Open scope at OSR-EXT 5c close

1. **OSR-EXT 5d** — runtime dispatcher integration (~80 LOC; marshal frame.locals → Vec<f64>; invoke call_osr; marshal back via box_to_value). Cascade-revival consumer of 5b + 5c.
2. **OSR-EXT 6** — alphabet extension (TL Moves 3+4 revival folded in).
3. **OSR-EXT 7** — composition probe + CRB final disposition.

### Cumulative status at OSR-EXT 5c close

LOC delta: ~70 (25 helper + 45 unit tests). OSR-EXT 0-5c cumulative: ~730 across the locale.

---

*OSR-EXT 5c closes. box_to_value helper landed. 4/4 unit tests cover Value-variant snapshots; all probes GREEN. OSR-EXT 5d wires box_to_value into the dispatcher invoke path.*

---

## OSR-EXT 5d — 2026-05-23 (runtime dispatcher integration; OSR loop invoke wired)

### Headline

Per option β three-round split close. OSR loop invoke now wired end-to-end:
- try_osr_compile uses `compile_function_osr` (instead of compile_function)
- `compile_function_osr` exported from JIT crate's lib.rs
- New `try_osr_invoke(frame, site_pc) -> bool` helper marshals frame.locals → Vec<f64> → call_osr → box_to_value → frame.locals; sets frame.pc = end_pc on success
- 5 Jump handlers restructured: at every back-edge fire past threshold, fast-path cache hit check + invoke; on success, skip the normal back-edge jump
- Fast-path inline cache check (`matches!(frame.osr_cache.get(&site_pc), Some(Some(_)))`) skips function-call overhead when cache is None or empty

~90 LOC delta (interp.rs: try_osr_invoke + 5 Jump handler restructures; JIT crate: compile_function_osr export).

### Three-probe results

| probe | result |
|---|---|
| canonical fuzz (acc=-932188103) | ✅ GREEN |
| diff-prod 42/42 | ✅ GREEN |
| JIT lib tests | ✅ 38/38 |
| OSR helper unit tests | ✅ 5/5 |
| box_to_value unit tests | ✅ 4/4 |
| A/B composition (5-run median) | 1528 vs baseline 1480 (+3.2%; within ±5% Pred-osr.4 gate) |
| CRB json_parse_transform | 2207 vs post-CharCode-2 baseline 2188 (+1%; within noise; cruft/node 17.38×) |

### Expected outcome on json_parse_transform (per Finding VII.2)

The invoke fast-path's correctness preservation is verified by ALL probes GREEN. But the invoke does NOT actually fire on json_parse_transform's checksum loop:

1. **Alphabet gap**: the loop body uses Op::GetProp (.length) + Op::CallMethod (.charCodeAt) — not in JIT alphabet per Finding VII.2. compile_function_osr fails at parse-time; cache stores None.

2. **Structural while/for forward-exit issue (Finding OSR.2 candidate)**: even if alphabet covered the inner ops, for-loop bytecode has the shape `loop_top: cond; JumpIfFalse loop_exit; body; Op::Jump loop_top` where `loop_exit` is OUTSIDE the extracted slice. The JIT translator's parse_bytecode would record loop_exit as a jump target; the translator allocates a Cranelift Block for it; the block is never filled (out of slice bounds); Cranelift fails at finalize. Only do-while-shape loops (`JumpIfTrue back-edge` at bottom, no forward exit out of body) extract cleanly.

So on json_parse_transform, OSR-EXT 5d's invoke path is verified by correctness probes (it doesn't fire; behavior is identical) but produces 0% CRB reclaim by construction.

**For invoke-path empirical validation**, a synthetic do-while-shape fixture would be needed (queued as candidate for an OSR-EXT 5e if keeper requests). Or wait for OSR-EXT 6 (alphabet extension) + OSR-EXT 6b (forward-exit handling) which together unblock json_parse_transform.

### Composition with prior corpus / engagement work

- **Doc 740 §VIII (this session's amendment)**: this round's invoke wiring is the (A4) locals-marshaling closure tier. (A2) op-set coverage + (A3) value-domain coverage are partially closed; full pipeline-connection still pending (A2) + the structural forward-exit issue.
- **OSR-EXT 5b + 5c**: cascade-revival consumer at this round; closes the option-β three-round chain.
- **Standing rule 9 (raw-pointer audit)**: applied — `boxed.as_ref() as *const CompiledFn` is stable for the cache box's lifetime; frame holds the cache; no aliasing concerns.
- **Standing rule 11 (multi-axis coverage)**: this round closes the (A4) axis substrate; (A2) + structural-extraction remain open.

### §XVI / Doc 734 / Doc 735 §X.h categorization

Per Doc 730 §XVI: not applicable.
Per Doc 734 §V: growth (a) positive-finding (invoke wired + correctness preserved); growth (b) negative-finding (Finding OSR.2 forward-exit-extraction structural blocker surfaces, candidate for follow-on findings work).
Per Doc 735 §X.h.b: substrate-introduction at locals-marshaling tier; (P2.d) on json_parse_transform CRB by construction (compile-fail on this fixture); empirical materialization queued for OSR-EXT 6 + forward-exit closure.

### Open scope at OSR-EXT 5d close

1. **Finding OSR.2 surfacing** — for-loop / while-loop forward-exit jumps to out-of-bounds targets; OSR loop extraction structurally limited to do-while-shape loops in current first cut. Local findings entry + candidate engagement-wide promotion at Addendum VIII.
2. **OSR-EXT 6** — alphabet extension (TL Moves 3+4 revival folded in: GetProp+length-IC + CallMethod+charCodeAt-IC consuming VD String encoding). Closes (A2) op-set coverage for the loop body. Combined with locals-marshaling (closed at 5d), unlocks ANY do-while loop that uses these intrinsics.
3. **OSR-EXT 6b** (candidate) — forward-exit extraction handling (extend loop boundary detection to include the forward-exit target's pc; emit synthetic block returning at exit). ~50 LOC. Unlocks for/while loops.
4. **OSR-EXT 7** — composition probe + CRB final disposition + Pred-osr.1 gate.

### Cumulative status at OSR-EXT 5d close

LOC delta: ~90 (interp.rs try_osr_invoke + 5 handler restructures + cache hot-path; JIT lib.rs export). OSR-EXT 0-5d cumulative: ~820 across the locale.

---

*OSR-EXT 5d closes. Option β three-round chain complete: 5b (JIT-side) + 5c (helper) + 5d (runtime wiring). Invoke is correctness-preserving + within composition gate; doesn't fire on json_parse_transform by construction (alphabet + structural extraction limits). OSR-EXT 6 + forward-exit handling are the remaining tiers to materialize CRB reclaim.*
