# rusty-js-jit/top-level — Trajectory

Per-TL-EXT log for the top-level JIT extension pilot.

---

## TL-EXT 0 — 2026-05-23 (workstream founding)

Apparatus-tier round. Pilot founded per keeper directive 2026-05-23 20:52-local as the (b-narrow) instantiation of Doc 740's multi-tier reading. Nested under LeJIT per Doc 737 §IV.

### Trigger

- JSF-EXT 10 CRB measurement: cumulative -12% across JSF M1-M4 + CharCode-EXT 1+2; cruft/node 17.93×; residual checksum loop 1480 ms.
- Doc 740 §III.4 multi-tier closure analysis: substrate + dispatch closed; remaining cost lives at interp loop dispatch per iter.
- Recon (2026-05-23 ~20:48-local): identified 3 JIT alphabet gaps (PushConst, GetProp, CallMethod) preventing the inner for-loop body from JIT-eligibility; no OSR mechanism; module bytecode never enters JIT.

### Substrate delivered

- `seed.md` (~135 lines): telos, 8 constraints C1-C8, 5 falsifiers Pred-tl.1-.5, methodology TL-EXT 0-7, carve-outs.
- `trajectory.md` (this file).
- `docs/` + `fixtures/` scaffolds.

### Locale registration

Locale count: 20 → 21 after this spawn (12 top-level unchanged; 7 → 8 nested). Manifest refresh queued at end of TL-EXT 0.

### Open scope at TL-EXT 0 close

1. **TL-EXT 1** — design doc enumerating per-move substrate plan.
2. **TL-EXT 2-5** — implementation per the design.
3. **TL-EXT 6** — composition probe + CRB final disposition.

### Cumulative status

LOC delta: 0 (apparatus round only).

---

*TL-EXT 0 closes. Pilot founded as (b-narrow) first cut. TL-EXT 1 designs the per-move substrate plan.*

---

## TL-EXT 1 — 2026-05-23 (design doc; 5-round dependency-ordered substrate plan)

### Headline

Design-tier round. `docs/design.md` (~225 lines) enumerates five substrate rounds dependency-ordered per Doc 740 §II.2 multi-tier reading: M1 PushConst-Number → M2 module-body wrap → M3 GetProp+length-IC → M4 CallMethod+charCodeAt-IC → M5 composition probe + CRB final disposition. Per-move mechanism + LOC + reclaim + falsifier specified. 6 named risks with mitigations.

### The 5 rounds

| round | move | tier | LOC | reclaim |
|---|---|---|---:|---:|
| TL-EXT 2 | M1 PushConst-Number | alphabet | ~30 | ~0% (substrate-intro signature) |
| TL-EXT 3 | M2 module-body wrap | entry-mechanism | ~80 | ~0% (entry-introduction) |
| TL-EXT 4 | M3 GetProp+length-IC | alphabet+IC | ~80 | ~5-15% |
| TL-EXT 5 | M4 CallMethod+charCodeAt-IC | alphabet+IC | ~100 | **~40-60% (pipeline-connection)** |
| TL-EXT 6 | M5 composition probe | measurement | 0 | (gate) |

Per Doc 740 §II.2 (P4): cumulative reclaim materializes at the final-tier-closure round (TL-EXT 5). M1-M3 are each substrate-introduction at their respective tier; per Finding II.2-bis, near-zero standalone reclaim is the signature.

### Pred-tl.1 gating analysis

Current state: CRB json_parse_transform 2188 ms; checksum loop 1480 ms at 0.592 μs/charCodeAt-call. Target: ≤1500 ms.

Required reclaim: ~688 ms. If JIT body matches typical Cranelift tight-loop output (~50 ns/iter), checksum loop drops from 1480 ms to ~125 ms — releases ~1355 ms. Pred-jsf.1 + Pred-tl.1 both met at projected ceiling.

### 6 named risks

R1 String encoding bit-layout discovery (at TL-EXT 4 impl time);
R2 Module-body GC roots preservation;
R3 Compile-time bail predictability for alphabet check;
R4 TB metadata cache vs module-once entry semantics;
R5 Top-level scope LoadGlobal/StoreGlobal — empirical at TL-EXT 3; may expand scope if json_parse_transform's globals bail the wrapper;
R6 Per-iter JIT body floor (interp-floor finding from JSF chain may recur).

### §XVI / Doc 734 / Doc 735 §X.h categorization

Per Doc 730 §XVI: not applicable (design-tier).
Per Doc 734 §V: growth (c) positive-finding generalization preparatory — design's reclaim projection anchors Pred-tl.1 gate.
Per Doc 735 §X.h.c: three-probe-levels applied at each substrate round.

### Composition with prior corpus / engagement work

- **Doc 740 multi-tier cascade-revival**: this pilot is the (b-narrow) instantiation; M1-M3 are upstream constraint-closures; M4 is the final tier closure that produces cumulative reclaim.
- **Doc 739 single-tier cascade-revival**: M3+M4 form a cascade-revival pair at the alphabet+IC tier.
- **Doc 731 §XIV.d alphabet purity**: this pilot extends the alphabet narrowly per Pred-tl.4 (3 new ops only).
- **Finding II.2-bis substrate-introduction signature**: M1+M2 each expected to produce ~0% standalone reclaim per the signature shape.
- **Standing rule 11**: satisfied via JSF-EXT 8 A/B probe; the actual dominator is empirically anchored at this pilot's spawn.
- **CharCode-EXT 2 interp-tier IC pattern**: M3+M4 reuse the same intrinsic-ObjectId verification discipline at the JIT tier.

### Open scope at TL-EXT 1 close

1. **TL-EXT 2** — Move 1 PushConst-Number (alphabet substrate-intro)
2. **TL-EXT 3** — Move 2 module-body wrap (entry-mechanism substrate-intro)
3. **TL-EXT 4** — Move 3 GetProp+length-IC (cascade-revival #1)
4. **TL-EXT 5** — Move 4 CallMethod+charCodeAt-IC (cascade-revival #2; pipeline-connection)
5. **TL-EXT 6** — Composition probe + CRB final disposition

### Cumulative status at TL-EXT 1 close

LOC delta: ~225 (design doc). 5 rounds enumerated; pipeline-connection projected at TL-EXT 5.

---

*TL-EXT 1 closes. Design enumerated. TL-EXT 2 begins implementation with Move 1 PushConst-Number.*

---

## TL-EXT 2 — 2026-05-23 (Move 1 PushConst-Number in JIT alphabet)

### Headline

`ParsedOp::PushConst(f64)` added to JIT alphabet. parse_bytecode signature extended to take `&ConstantsPool`; PushConst's u16 index resolved at parse-time to f64; non-Number constants bail per C8. Translate-pass emits `builder.ins().f64const(n)`. ~30 LOC delta in translator.rs. Substrate-introduction at the alphabet tier; no reclaim expected by design (Finding II.2-bis signature).

### Three-probe results

| probe | result |
|---|---|
| Pred-tl.2 canonical fuzz (acc=-932188103) | ✅ GREEN |
| Pred-tl.3 diff-prod 42/42 | ✅ GREEN |
| Pred-tl.5 JIT lib tests | ✅ 38 pass, 9 ignored (pre-existing Φ-EXT 3 hold) |
| Pred-tl.bench A/B probe checksum | flat (1480 → 1470-1507 median, within noise) |

### Substrate moves landed

1. **Import**: added `use rusty_js_bytecode::constants::{Constant, ConstantsPool};`
2. **ParsedOp enum**: added `PushConst(f64)` variant.
3. **compile_function_inner**: parse_bytecode call now passes `&proto.constants`.
4. **parse_bytecode signature**: now takes `(bc: &[u8], constants: &ConstantsPool)`.
5. **parse-pass Op::PushConst arm**: decodes u16 idx; matches `constants.get(idx)`: `Some(Constant::Number(n))` → `ParsedOp::PushConst(*n)`; other Constant variants return Err per C8 bail discipline.
6. **translate-pass ParsedOp::PushConst arm**: `builder.ins().f64const(*n)` push onto operand stack.

### Composition with prior corpus / engagement work

- **Doc 740 multi-tier reading + Finding II.2-bis**: Move 1 is substrate-introduction at the alphabet tier; near-zero standalone reclaim is the signature, not a falsification.
- **Φ-EXT 3 f64-default calling convention**: PushConst flows on the f64 stack; lossless for any Number constant.
- **Pred-tl.4 scope discipline**: only Number constants accepted; String/BigInt/Regex/Function bail at parse-time. Falsifier met (no other Constant variants supported in this round).
- **Standing rule 9 (raw-pointer audit)**: not applicable (no new pointer caches).

### §XVI / Doc 734 / Doc 735 §X.h categorization

Per Doc 730 §XVI: not applicable.
Per Doc 734 §V: growth (c) preparatory — Move 1 is the alphabet substrate-intro that enables Moves 3+4 cascade-revival pilots.
Per Doc 735 §X.h.b: **(P2.d) bench at substrate-introduction round, expected per Doc 739 §II.2 + Finding II.2-bis. Re-categorization to (P2.a) expected at TL-EXT 5 (M4 CallMethod+charCodeAt-IC) per the cumulative-reclaim materialization point.**

### Open scope at TL-EXT 2 close

1. **TL-EXT 3** — Move 2 module-body JIT entry wrapper (entry-mechanism substrate-intro)
2. **TL-EXT 4** — Move 3 GetProp+length-IC (cascade-revival #1)
3. **TL-EXT 5** — Move 4 CallMethod+charCodeAt-IC (cascade-revival #2; pipeline-connection)

### Cumulative status at TL-EXT 2 close

LOC delta: ~30 (translator.rs alphabet extension). Canonical fuzz + diff-prod GREEN. JIT lib tests 38/38 (9 ignored pre-existing). A/B probe flat as predicted.

---

*TL-EXT 2 closes. Move 1 PushConst-Number landed at the JIT alphabet tier. Flat A/B probe is the substrate-introduction signature; TL-EXT 3 lands Move 2 module-body wrap (the entry-mechanism upstream constraint-closure).*

---

## TL-EXT 3 — 2026-05-23 (Move 2 module-body JIT entry wrapper)

### Headline

Module-body JIT entry wired. JIT crate: `JitFn0` type + `JitFn::Arity0` variant + `call0` method; param check relaxed to accept 0, 1, or 2. Runtime: `Runtime::try_jit_run_module` builds a synthetic FunctionProto from CompiledModule (bytecode + constants + locals + line_starts + source_map + construct_tags + strict; empty upvalues; 0 params); attempts `compile_function`; on success sets TLS + invokes `call0` + clears TLS; discards return (module result is always Undefined per ECMA); on failure falls through to existing interp `run_frame`. ~90 LOC delta.

### Three-probe results

| probe | result |
|---|---|
| Pred-tl.2 canonical fuzz (acc=-932188103) | ✅ GREEN |
| Pred-tl.3 diff-prod 42/42 | ✅ GREEN |
| Pred-tl.5 JIT lib tests | ✅ 38 pass, 9 ignored (pre-existing) |
| Pred-tl.bench A/B probe checksum | flat (1482-1484 median; one 1725 outlier discarded) |

### R5 disposition (LoadGlobal/StoreGlobal scope risk)

Empirical readout: json_parse_transform's top-level bytecode contains MakeClosure (for `makePayload`), Op::Call, Op::CallMethod, presumably LoadGlobal/StoreGlobal for top-level `const payload = ...` and `let checksum = 0`. The JIT compile bails at parse-time on any of those per C8 bail discipline. The wrapper attempts compile, bails cleanly, runtime falls through to interp. **A/B probe flat as predicted — no measurable change. R5 surfaces as "expected bail" on this fixture, not as "regression."**

This is the substrate-introduction signature per Finding II.2-bis: the entry mechanism is now in place; the wrapper exercises the JIT compile path; whether json_parse_transform's body actually JIT-fires is gated on Moves 3+4 (and beyond, since the top-level body has many ops beyond charCodeAt-loop alphabet). The structural infrastructure landed cleanly without regression.

### Composition with prior corpus / engagement work

- **Doc 740 multi-tier reading**: Move 2 closes the entry-mechanism tier; Moves 3+4 are downstream consumer pilots that gate cumulative reclaim.
- **Doc 739 §II.2 substrate-introduction signature**: empirically observed (flat A/B; expected bail on real fixture; no regression).
- **Standing rule 9 (raw-pointer cache audit)**: not applicable (`&synth as *const _` is a stack local lifetime-bound to the function call; passed through TLS only for the duration of the JIT call; cleared post-call).
- **TB metadata cache discipline**: bypassed for module path (modules are called once; TB optimization targets multi-call functions). No tb_metadata wiring at module entry — `compile_function` builds metadata under env flag but the module dispatcher doesn't consult it.

### §XVI / Doc 734 / Doc 735 §X.h categorization

Per Doc 730 §XVI: not applicable.
Per Doc 734 §V: growth (c) preparatory — entry-mechanism substrate-intro that enables Moves 3+4 cascade-revival pilots.
Per Doc 735 §X.h.b: **(P2.d) bench at substrate-introduction round, expected per Doc 740 §II.2 (P4). Re-categorization to (P2.a) expected at TL-EXT 5.**

### Open scope at TL-EXT 3 close

1. **TL-EXT 4** — Move 3 GetProp+length-IC (cascade-revival #1)
2. **TL-EXT 5** — Move 4 CallMethod+charCodeAt-IC (cascade-revival #2; pipeline-connection)
3. **TL-EXT 6** — composition probe + CRB final disposition

### Cumulative status at TL-EXT 3 close

LOC delta: ~90 (translator.rs: JitFn0 + Arity0 variant + call0 + param check relax + match arm; interp.rs: try_jit_run_module + run_module guard). TL-EXT 2+3 cumulative: ~120 LOC (Moves 1+2). Canonical fuzz + diff-prod GREEN throughout. JIT lib tests 38/38.

---

*TL-EXT 3 closes. Move 2 module-body JIT entry wrapper landed. The wrapper attempts JIT compile at module entry; bails cleanly on alphabet gaps (per C8); falls through to interp without regression. A/B probe flat — substrate-introduction signature per Finding II.2-bis. TL-EXT 4 lands Move 3 GetProp+length-IC.*

---

## (b-narrow) chapter close — 2026-05-23

### Summary

(b-narrow) closed structurally at TL-EXT 3, not at TL-EXT 5 as the design projected. Two new findings surfaced during TL-EXT 3 + TL-EXT 4 pre-implementation source-read; both promoted to engagement-wide scope at findings.md Addendum V.

### Rounds landed

| round | substrate | LOC | three-probe | bench |
|---|---|---:|---|---|
| TL-EXT 0 | locale founding + manifest refresh | ~150 | n/a | n/a |
| TL-EXT 1 | design doc | ~225 | n/a | n/a |
| TL-EXT 2 | M1 PushConst-Number (alphabet) | ~30 | GREEN | flat (subst-intro signature) |
| TL-EXT 3 | M2 module-body JIT entry wrapper | ~90 | GREEN | flat (subst-intro signature) |

Total (b-narrow) substrate: ~120 LOC of correctness-preserving JIT-tier infrastructure (JitFn0 + module-body wrapper + PushConst alphabet variant).

### Rounds NOT landed (per Findings VII.2 + VII.3)

| round | originally designed | not landed because |
|---|---|---|
| TL-EXT 4 | M3 GetProp+length-IC | Finding VII.3 — Φ encoding can't carry String receiver identity into JIT body; IC fast-path cannot emit |
| TL-EXT 5 | M4 CallMethod+charCodeAt-IC | Same as TL-EXT 4 |
| TL-EXT 6 | composition probe | Would have measured 0% on json_parse_transform per Finding VII.2 (whole-body bail on top-level body's MakeClosure/LoadGlobal/Op::Call) AND per Finding VII.3 (String identity loss). Both blockers structurally independent of inner-loop alphabet coverage |

### Findings generated (engagement-wide promotion at Addendum V)

- **Finding VII.2 (op-set-coverage)**: whole-body JIT bail discipline gates inner-loop eligibility on FULL enclosing-scope alphabet coverage. (b-narrow)'s inner-loop-only alphabet additions wouldn't have closed json_parse_transform's top-level body. Source-read enumeration must include the FULL bytecode of the enclosing scope at pilot design time.
- **Finding VII.3 (value-domain coverage)**: JIT calling convention encodes only Number+Object Values; all others (String, BigInt, etc.) degrade to 0.0 at unbox_arg_f64. IC fast-paths requiring String receivers cannot emit until Φ encoding is extended. Pre-spawn check: verify calling convention covers required receiver variants.
- **Standing rule 11 extension (two coverage axes)**: rule 11's component A/B probe (Addendum IV) now also gates on op-set coverage + value-domain coverage for JIT-alphabet/IC pilots.

### Locale disposition

**(b-narrow) closed at TL-EXT 3.** Pilot delivered substrate-introduction value at the entry-mechanism tier + alphabet groundwork (PushConst-Number) + two engagement-wide findings (VII.2 + VII.3) + standing rule 11 extension.

**Pred-tl.1 (≥40% CRB reclaim on json_parse_transform): structurally infeasible via (b-narrow) alone.** Both coverage gaps (VII.2 op-set; VII.3 value-domain) must close before any meaningful CRB reclaim materializes. Pivot to (b-architectural) per keeper directive 2026-05-23 21:34-local.

### Composition with prior corpus / engagement work

- **Doc 740 §II.2 (P4) multi-tier reading**: TL pilot empirically validated the pattern — closing one tier (entry-mechanism, TL-EXT 3) without closing the other relevant tiers (op-set coverage; value-domain coverage) produces 0% cumulative reclaim. The pattern's prediction (full pipeline-connection requires ALL relevant tiers closed) held.
- **Finding II.2-bis (substrate-introduction (P2.d) signature)**: TL-EXT 2 + TL-EXT 3's flat A/B is the correct categorization; the rounds DID close their respective upstream tier. The failure mode is on the cumulative consumer side (Moves 3+4) not on the substrate-introduction side.
- **Finding VII.1 (component A/B before pilot spawn)**: rule 11 satisfied at JSF-EXT 8; (b-narrow) plan still mis-projected reclaim because the rule didn't cover op-set / value-domain coverage at the time. Findings VII.2 + VII.3 close that gap going forward.

### Cumulative status at (b-narrow) chapter close

LOC delta (TL-EXT 0-3): ~270 trajectory + ~120 source = ~390 across the locale.
Pred-tl.1: structurally falsified by Findings VII.2 + VII.3.
Pred-tl.2 (canonical fuzz), Pred-tl.3 (diff-prod): GREEN throughout.
Pred-tl.5 (composition with TB/Φ/Σ): GREEN throughout.
JIT lib tests: 38/38 (9 pre-existing ignored).

Findings docs updated:
- TL local findings.md: Findings TL.1 + TL.2
- LeJIT engagement findings.md Addendum V: Findings VII.2 + VII.3 + standing rule 11 multi-axis extension

### Forward to (b-architectural)

Per keeper directive 2026-05-23 21:34-local + 21:45-local: pivot to (b-architectural). Two structurally co-equal pivot targets surfaced by Findings VII.2 + VII.3:

1. **Φ-encoding extension** — close value-domain coverage. NaN-boxing or sentinel-bit pattern for Rc<String> + other non-Number/Object Value variants. Major Φ-tier work; cuts across all JIT pilots.
2. **OSR / loop-extraction** — close op-set coverage by reducing enclosing scope to the loop. Bytecode transform at runtime to extract hot loops into JIT-eligible synthetic functions.

Either or both. Per Doc 740 §II.2 P4: full pipeline-connection on json_parse_transform requires BOTH. The new pilot's scope is a keeper decision.

---

*(b-narrow) chapter closes. Substrate-introduction value at entry-mechanism tier + alphabet groundwork + 2 engagement-wide findings + rule 11 extension delivered. Pred-tl.1 structurally infeasible without architectural pivot. Spawn (b-architectural) pilot per next keeper directive.*
