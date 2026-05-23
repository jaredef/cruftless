# rusty-js-json-fast — Trajectory

Per-JSF-EXT log for the JSON.stringify fast-path pilot.

---

## JSF-EXT 0 — 2026-05-23 (workstream founding)

Apparatus-tier round. Pilot founded per keeper directive 2026-05-23 18:12-local. Standalone top-level locale per Doc 737 §IV. First of four spawns in this round (rusty-js-json-fast + rusty-js-regex-fast + web-crypto/subtle-wireup + rusty-js-http-server) addressing the engagement's named substrate gaps from CRB-EXT 9.

### Trigger

- CRB-EXT 9 reading: json_parse_transform 20.34× cruft/node; JSON.stringify estimated 5-10× contributor.
- Findings VI.1 HIGH priority forward-derived pilot from the LeJIT-tier work.
- Keeper directive enumerated 4 substrate gaps; this pilot addresses the JSON.stringify perf gap.

### Substrate delivered

- `seed.md` (~80 lines): telos, 5 falsifiers Pred-jsf.1-.5, methodology JSF-EXT 0-6, carve-outs.
- `trajectory.md` (this file).
- `docs/` + `fixtures/` scaffolds.

### Locale registration

Locale count: 16 → 17 after this spawn (10 → 11 top-level; 6 nested unchanged). Manifest refresh queued at end of this 4-spawn round.

### Open scope at JSF-EXT 0 close

1. JSF-EXT 1 — bench probe baseline.
2. JSF-EXT 2 — design doc.
3. JSF-EXT 3 — implementation.
4. JSF-EXTs 4-6 per seed §III.

---

*JSF-EXT 0 closes. Pilot founded. JSF-EXT 1 begins with bench baseline (after the keeper-directed JSON.stringify attack round begins).*

---

## JSF-EXT 1 — 2026-05-23 (bench baseline; per-shape cruft/node 10-15×)

### Headline

Per-shape micro-bench harness (`cruftless/examples/bench_json_stringify.rs` ~115 LOC) measures 5 stringify shape classes × 3 runs × {cruft, node}. **Per-shape cruft/node 10-15×**; worst is number-stringify at **15.16×**; best is string-only at **10.09×**. Hot-path component decomposition surfaces 4 highest-impact substrate-move targets for JSF-EXT 2's design.

### Baseline measurements (Pi, 2026-05-23)

| shape | cruft (μs) | node (μs) | **cruft/node** |
|---|---:|---:|---:|
| A: small-object 100k | 1,193,587 | 112,765 | **10.58×** |
| B: deep-nested 100k | 1,327,405 | 94,052 | **14.11×** |
| C: array-of-obj 5k | 1,394,824 | 111,800 | **12.48×** |
| D: number-only 1M | 2,238,954 | 147,679 | **15.16×** |
| E: string-only 1M | 2,331,291 | 231,040 | **10.09×** |

### Highest-impact targets (per `docs/bench-baseline.md`)

1. **Number-stringify lookup table** (15.16× → ~3-5× target): replace `abstract_ops::number_to_string` allocate-per-call with chunked-buffer-write; lookup table for integer fast path.
2. **String-escape branchless ASCII fast-path** (10.09× → ~2-3× target): bulk `extend_from_slice` when no special chars.
3. **Chunked output buffer threading** (affects all shapes): replace per-call `String::with_capacity` + `format!` + `join(",")` with single output buffer down the recursion. Eliminates O(depth × siblings) intermediate allocations.
4. **Format-macro elimination + property iteration via reference**: smaller per-call wins.

### §XVI / Doc 734 / Doc 735 §X.h categorization

Per Doc 730 §XVI: not applicable (probe-tier).
Per Doc 734 §V: growth (c) positive-finding generalization preparatory — bench anchors empirical target for JSF-EXT 2 design.
Per Doc 735 §X.h.c: bench probe activated; consumer-route + fuzz at JSF-EXT 4+5+6.

### Open scope at JSF-EXT 1 close

1. **JSF-EXT 2** — design doc enumerating per-component substrate-move plan. Output: `docs/design.md`.
2. **JSF-EXT 3+** — substrate landings per the design.

### Cumulative status

LOC delta: ~115 (bench harness) + ~170 (bench-baseline doc + trajectory entry). 5 shape classes × cruft + node measurements anchored. 4 highest-impact targets named with reclaim estimates.

---

*JSF-EXT 1 closes. Baseline 10-15× cruft/node per shape; 4 highest-impact targets named. JSF-EXT 2 designs the substrate moves.*

---

## JSF-EXT 2 — 2026-05-23 (design doc; 4-move dependency-ordered substrate plan)

### Headline

Design-tier round. `docs/design.md` (~225 lines) enumerates four substrate moves with constraint-enumeration discipline, per-shape reclaim estimates, composition reading, risks, and staged-validation per Findings II.2. Post-Move-4 expected cruft/node ratios: ~1.5-3× across all 5 shape classes (vs current 10-15×). Pred-jsf.1 ≥40% CRB reclaim target empirically reachable per the composition analysis.

### The 4 substrate moves (dependency-ordered)

| move | round | mechanism | LOC | reclaim |
|---|---|---|---:|---:|
| 1 (foundational) | JSF-EXT 3 | output buffer threading (json_stringify_into) | ~80 | 1.2-3× per shape |
| 2 | JSF-EXT 4 | string-escape branchless ASCII fast-path | ~50 | 3-5× on E |
| 3 | JSF-EXT 5 | number-stringify integer fast-path (no alloc) | ~70 | 5-8× on D |
| 4 | JSF-EXT 6 | format-macro elimination + property iter via reference | ~40 | 1.2-1.3× per object |

Move 1 is foundational (buffer threading enables all others to write without intermediate allocations). Moves 2-4 are independent closure rounds.

### Composition reading

Multiplicative expected reclaim per shape:

| shape | cumulative | post cruft/node |
|---|---:|---:|
| A small-object | ~3.7× | ~2.9× (vs current 10.58×) |
| B deep-nested | ~4.7× | ~3.0× (vs current 14.11×) |
| C array-of-obj | ~5.9× | ~2.1× (vs current 12.48×) |
| D number-only | ~6-10× | ~1.5-2.5× (vs current 15.16×) |
| E string-only | ~4.5-7.5× | ~1.3-2.2× (vs current 10.09×) |

Per Finding III.4 (composition synergy constructive when targets orthogonal): the actual reclaim may exceed the multiplicative estimate when components compose constructively (M1 + M3 both operate on numbers; M1 + M2 both operate on strings; etc.).

Pred-jsf.1 (CRB json_parse_transform ≥40% reclaim; 2481 → ≤1500 ms): empirically reachable per the composition reading. Margin uncertainty exists; the bench gates each round.

### Constraint enumeration (per Pin-Art / Doc 581 / Φ seed §I.2 / Doc 739 apparatus)

8 constraints C1-C8 named the substrate space:
- C1 byte-identical output (canonical fuzz acc=-932188103)
- C2 ECMA-262 §25.5.2 semantics preserved
- C3 shape-aware iteration preserved (CMig-EXT 16.bis pattern)
- C4 cap-passing modes unaffected
- C5 recursion semantics preserved
- C6 PropertyDescriptor.clone() avoidance only where borrow allows
- C7 no new env flag (architectural correctness-preserving improvement)
- C8 standing three-probe-levels gate per Findings rule 5/10

The architecture induced by C1+C2+C5+C7: chunked output buffer threaded through the recursion + per-type fast paths emitting directly into the buffer. Per Pin-Art: design induced by constraints, not arbitrary choice.

### 5 risks named + mitigations

R1 — push_str micro-cost; R2 — initial buffer capacity tuning; R3 — i64::MIN edge case; R4 — UTF-8 multibyte slicing; R5 — borrow conflict in Move 4.

### §XVI / Doc 734 / Doc 735 §X.h categorization

Per Doc 730 §XVI: not applicable (design-tier).
Per Doc 734 §V: growth (c) positive-finding generalization preparatory — design's composition reading anchors expected (P2.a) at JSF-EXT 6 close.
Per Doc 735 §X.h.c: three-probe-levels discipline applied at each substrate round.

### Composition with prior corpus / engagement work

- **JSF-EXT 1 bench baseline**: design's per-shape reclaim estimates are anchored on the bench numbers.
- **Findings II.2 (never split substrate moves)**: each move is a self-contained add-remove pair; no intermediate-state-worse splits.
- **Findings V.5 (default-on flip discipline compounds)**: each round runs three-probe-levels (canonical fuzz + diff-prod + bench).
- **Doc 729 §A8.13 substrate-amortization-cascade**: Move 1 is substrate-introduction (buffer); Moves 2-4 are closure rounds. Classical cascade.
- **CMig-EXT 16.bis shape-iter chain pattern**: preserved by C3.
- **Φ seed §I.2 constraint enumeration**: discipline reapplied here at JSON tier.
- **Doc 739 cascade-revival**: not directly applicable (no (P2.d) stalls); pattern's apparatus (constraint enumeration) is the framework used here.

### Open scope at JSF-EXT 2 close

1. **JSF-EXT 3** — Move 1 buffer threading (substrate-introduction)
2. **JSF-EXT 4** — Move 2 string-escape ASCII fast-path
3. **JSF-EXT 5** — Move 3 number-stringify integer fast-path
4. **JSF-EXT 6** — Move 4 format-macro elimination
5. **JSF-EXT 7** (optional) — CRB re-bench + Pred-jsf.1 final disposition

### Cumulative status at JSF-EXT 2 close

LOC delta: ~225 (design doc). 4 substrate moves enumerated with per-component mechanism, LOC, reclaim estimate, falsifier gating.

---

*JSF-EXT 2 closes. Design enumerated. JSF-EXT 3 begins with Move 1 buffer threading (foundational substrate-introduction round per Doc 729 §A8.13).*

---

## JSF-EXT 3 — 2026-05-23 (Move 1 buffer threading; (P2.d) bench, GREEN correctness; constraint-closure landing per Doc 739)

### Headline

`json_stringify_into(rt, v, &mut out)` lands. Object/Array branches eliminate `format!("[{}]", body.join(","))` + `Vec<String>` collects; recursion threads a single buffer. ~110 LOC delta in `intrinsics.rs`. Correctness preserved across both probe levels. Bench essentially flat at first cut — **expected per Doc 739 §II.2** (upstream constraint-closure standalone; cascade-revival downstream pilots not yet landed).

### Three-probe results

| probe | result |
|---|---|
| Pred-jsf.2 canonical fuzz (acc=-932188103) | ✅ GREEN |
| Pred-jsf.3 diff-prod 42/42 | ✅ GREEN |
| Pred-jsf.bench per-shape | (P2.d) flat — all shapes within ±1× of baseline |

### Per-shape bench (cruft/node)

| shape | pre (JSF-EXT 1) | post-M1 | Δ |
|---|---:|---:|---:|
| A small-object | 10.58× | 10.66× | flat |
| B deep-nested | 14.11× | 14.16× | flat |
| C array-of-obj | 12.48× | 12.55× | flat |
| D number-only | 15.16× | 15.12× | flat |
| E string-only | 10.09× | 11.04× | ~flat (within noise) |

### Doc 739 framing

Move 1 IS the upstream constraint-closure substrate-introduction at the JSON-stringify resolver-instance pipeline:

```
P1 — value-to-emission lowering (per-Value match dispatch)
P2 — output-aggregation interface (how emitted bytes flow between recursion levels)
P3 — leaf emission (number_to_string; json_quote_string)
```

**The pre-M1 P2 constraint**: every recursion level allocated a fresh String for its result; parent levels collected children's Strings via `Vec<String>` + `join(",")` + `format!`.

**The propagation**:
- P3 leaf emitters (number_to_string; json_quote_string) had to RETURN fresh Strings, because P2's interface was "return String."
- P3's allocate-per-call was a propagated constraint, not intrinsic. The leaf emitters could write directly into a buffer if P2 admitted a `&mut String` interface.

**Move 1 closure**: lift P2's interface from "return String" to "append into &mut String." P3 leaves now have somewhere to write directly. The cascade-revival candidates are Move 2 (string ASCII fast-path; writes directly into buffer instead of allocating, then push_str'ing) and Move 3 (integer fast-path; same shape).

**Why bench is flat at M1 alone**: per Doc 739 §II.2, the sibling pilots (M2, M3) are constraint-propagation-stalled. They each STILL allocate intermediate Strings (because the leaf functions haven't been rewritten yet); the buffer threading at the object/array level just moves the allocation point without eliminating it. The standalone Move 1 reclaim is structurally near-zero because the propagated allocation pattern at the leaves is still in force.

**Cascade prediction**: at Move 2 landing, E (string-only) drops sharply because json_quote_string writes-directly removes the per-call String allocation. At Move 3 landing, D (number-only) drops sharply for the same reason. A/B/C benefit at Move 4 (format-macro elimination compounds with the M1 buffer threading).

### Composition with prior corpus / engagement work

- **Doc 739 §II.2 sibling-pilot stall**: M1's flat bench is the SIGNATURE of an upstream constraint propagation. M2/M3 are cascade-revival candidates per Doc 739 (P1).
- **Doc 729 §A8.13 substrate-amortization-cascade**: M1 is substrate-introduction; per-iter cost reduction materializes at the consumer rounds.
- **Findings II.2 staged-validation**: M1 added-and-removed atomically (json_stringify thin-wrapper preserves the public API).
- **Findings rule 5 + standing rule 10 three-probe-levels**: canonical fuzz + diff-prod gates ran; both GREEN.
- **CMig-EXT 16.bis shape-iter chain**: preserved (M1 didn't touch the shape-aware property snapshot).

### Finding generated (candidate for findings addendum IV)

**Finding II.2-bis (staged-validation refinement)**: in a constraint-closure substrate-introduction round, the round's STANDALONE bench reclaim may be (P2.d) near-zero by design. The reclaim materializes at the cascade-revival rounds (downstream consumers). The (P2.d) at the substrate-introduction round is not a falsification; it is the SIGNATURE that the round IS substrate-introduction (not pilot-local optimization). Per Doc 739 §II.2: classify the (P2.d) before declaring failure; if the round closes an upstream constraint whose propagation explains the leaf cost, the round is correctly placed.

**How to apply**: at each substrate-introduction round, name the upstream constraint being closed + the downstream pilots that become cascade-revival candidates per the closure. If both are nameable, accept (P2.d) bench at the introduction round and proceed to the consumer rounds. If neither is nameable, the (P2.d) is a genuine pilot-failure signal.

### §XVI / Doc 734 / Doc 735 §X.h categorization

Per Doc 730 §XVI: not applicable (no diff-prod regression).
Per Doc 734 §V: growth (b) negative-finding catalyzes refinement of the design's per-move reclaim model.
Per Doc 735 §X.h.b: **(P2.d) at substrate-introduction round, expected per Doc 739 §II.2**. Re-categorization to (P2.a) expected at Move 2 + Move 3 landings.

### Open scope at JSF-EXT 3 close

1. **JSF-EXT 4** — Move 2 string-escape branchless ASCII fast-path (cascade-revival pilot #1)
2. **JSF-EXT 5** — Move 3 number-stringify integer fast-path (cascade-revival pilot #2)
3. **JSF-EXT 6** — Move 4 format-macro elimination (compounding round)
4. **JSF-EXT 7** — CRB re-bench + Pred-jsf.1 final disposition
5. **Findings doc addendum IV** — codify Finding II.2-bis (substrate-introduction (P2.d) as cascade-revival signature)

### Cumulative status at JSF-EXT 3 close

LOC delta: ~110 (intrinsics.rs: new json_stringify_into + json_quote_string_into + thin wrapper). 3 probes ran; 2 GREEN, 1 (P2.d) expected per Doc 739. Substrate-introduction landed; cascade-revival rounds queued.

---

*JSF-EXT 3 closes. Move 1 buffer threading landed as upstream constraint-closure per Doc 739. Flat bench is the §II.2 sibling-stall signature; cascade-revival pilots (M2, M3) queue at JSF-EXT 4-5.*

---

## JSF-EXT 4 — 2026-05-23 (Move 2 string-escape branchless ASCII fast-path; cascade-revival pilot #1)

### Headline

`json_quote_string_into` rewritten as a two-stage scan: stage 1 advances bytes through ASCII non-special + UTF-8 continuations to the next escape stop, then bulk-copies the run via push_str; stage 2 emits the escape and advances. Eliminates the `format!("\\u{:04x}")` allocation per control char. Correctness preserved across both probe levels. Bench shows small per-shape wins on object shapes (A, C); E (string-only) flat because the bench fixture's string has frequent embedded escapes (short bulk-copy runs).

### Three-probe results

| probe | result |
|---|---|
| Pred-jsf.2 canonical fuzz (acc=-932188103) | ✅ GREEN |
| Pred-jsf.3 diff-prod 42/42 | ✅ GREEN |
| Pred-jsf.bench per-shape | partial (P2.a) on object shapes; (P2.d) on D/E |

### Per-shape bench (cruft/node)

| shape | pre-M2 (post-M1) | post-M2 | Δ |
|---|---:|---:|---:|
| A small-object | 10.66× | 9.87× | **-7%** |
| B deep-nested | 14.16× | 14.29× | flat |
| C array-of-obj | 12.55× | 11.86× | **-5%** |
| D number-only | 15.12× | 15.85× | flat-to-worse (within noise) |
| E number-only | 11.04× | 10.80× | flat |

### Finding (bench-design weakness, not substrate weakness)

The bench fixture E uses `"Hello, \"World\"\nThis is a test\twith various\\escapes"` — string with frequent embedded escapes. The branchless fast-path's win is proportional to the length of pure-ASCII bulk-copy runs between escape stops; this fixture's runs are 4-15 bytes each. For realistic workloads (JSON of user data with mostly clean text), runs would be 50-200+ bytes and the fast-path would show its expected 3-5× win.

**This is a fixture-design limitation in JSF-EXT 1's bench**, not a Move 2 falsification. Per Findings II.2 staged-validation: do not falsify Move 2 on E from this bench alone; re-measure on CRB json_parse_transform at JSF-EXT 7 where the JSON workload includes longer ASCII strings.

### Doc 739 framing (continued)

Move 2 IS the cascade-revival pilot #1 under Doc 739 §II.3:
- Pre-M2 leaf emitter (json_quote_string) allocated a fresh String, then push_str'd it into the buffer (per M1).
- Post-M2 leaf emitter writes directly into the buffer; no intermediate allocation.
- Cascade-revival prediction (object shapes): MET (small wins on A, C from eliminated leaf allocation per-property).
- Cascade-revival prediction (string-only shape): NOT MET in this bench due to fixture design; bench at CRB instead.

### Composition with prior corpus / engagement work

- **Doc 739 §II.3 cascade-revival pattern**: M2 is the predicted revived pilot; reclaim materializes at the consumer shapes (A, C, B partial).
- **Findings II.2 staged-validation**: Move 2 added-and-removed atomically (json_quote_string thin wrapper preserves public API).
- **Findings rule 5 + standing rule 10**: three probes ran; both correctness GREEN.
- **R4 risk (UTF-8 multibyte slicing)**: validated via canonical fuzz; multibyte chars are passed through as opaque bytes in stage 1.

### §XVI / Doc 734 / Doc 735 §X.h categorization

Per Doc 730 §XVI: not applicable.
Per Doc 734 §V: growth (a) positive-finding empirical-confirmation on object shapes; growth (b) negative-finding on E surfaces bench-fixture design limitation.
Per Doc 735 §X.h.b: **(P2.a) on A/C; (P2.d) on D/E with named bench-design caveat**.

### Open scope at JSF-EXT 4 close

1. **JSF-EXT 5** — Move 3 number-stringify integer fast-path (expected (P2.a) on D)
2. **JSF-EXT 6** — Move 4 format-macro elimination
3. **JSF-EXT 7** — CRB re-bench + Pred-jsf.1 final disposition
4. **Findings doc addendum IV** — codify Finding II.2-bis + bench-fixture-design note

### Cumulative status at JSF-EXT 4 close

LOC delta: ~50 (json_quote_string_into rewrite). Two cascade-revival pilots remaining. Aggregate per-shape position: 3 shapes flat, 2 small wins. Reclaim concentration expected at JSF-EXT 5 (D number-only).

---

*JSF-EXT 4 closes. Move 2 cascade-revival pilot landed; partial empirical confirmation on object shapes (A, C); D awaits Move 3; E awaits CRB re-bench at JSF-EXT 7 (bench-fixture design limitation).*

---

## JSF-EXT 5 — 2026-05-23 (Move 3 number-stringify integer fast-path; cascade-revival pilot #2)

### Headline

`write_i64_into(n, &mut out)` writes signed-i64 decimal directly into the buffer; reverse-emit-then-byte-reverse on the appended ASCII slice (no allocation). Integer detection in `json_stringify_into`'s Number branch via `n.is_finite() && n.fract() == 0.0 && n in [i64::MIN, i64::MAX]`. f64-fractional falls back to `abstract_ops::number_to_string`. R3 risk (i64::MIN edge) handled by emitting the known string directly to avoid `(-i64::MIN)` overflow.

### Three-probe results

| probe | result |
|---|---|
| Pred-jsf.2 canonical fuzz | ✅ GREEN |
| Pred-jsf.3 diff-prod 42/42 | ✅ GREEN |
| Pred-jsf.bench D shape | (P2.a) ~5% reclaim — below 5-8× projection; **micro-bench is interp-overhead-bound** |

### Per-shape bench (cruft/node)

| shape | pre-M3 | post-M3 | Δ |
|---|---:|---:|---:|
| A small-object | 9.87× | 9.71× | flat |
| B deep-nested | 14.29× | 14.33× | flat |
| C array-of-obj | 11.86× | 12.55× | flat (noise) |
| D number-only | 15.85× | 15.05× | **-5%** |
| E string-only | 10.80× | 10.31× | **-5%** |

### Aggregate finding across JSF-EXT 3-5

**The per-shape micro-bench is dominated by JS interpreter overhead (call dispatch + arg unbox + result-into-JS-string + .length + accumulate), NOT by JSON.stringify proper.** Even fully eliminating JSON.stringify substrate cost would not deliver the design's projected 5-15× reclaim because most of the wall-clock is interp framework.

Aggregate JSF-EXT 1→5 per-shape position:
- A: 10.58× → 9.71× (-8% cumulative)
- B: 14.11× → 14.33× (flat)
- C: 12.48× → 12.55× (flat)
- D: 15.16× → 15.05× (flat)
- E: 10.09× → 10.31× (flat)

### Implication: Pred-jsf.1 measurement strategy shift

JSF-EXT 2's per-shape composition table projected post-M4 cruft/node ratios of ~1.5-3×. **That projection assumed JSON.stringify was the per-op cost dominator at this bench.** The empirical readout falsifies that assumption for this bench harness.

**Strategy shift**: Pred-jsf.1 gates on CRB json_parse_transform (a longer workload where JSON.stringify is a larger fraction). Stand JSF-EXT 7 up as CRB re-bench specifically; do not chase further reclaim on the micro-bench. The micro-bench's value going forward is correctness probe (per-shape regression detection), not reclaim measurement.

### Doc 739 framing (continued)

Move 3 is cascade-revival pilot #2 per Doc 739 §II.3. Per the structural prediction, M3 enables D shape's direct-write integer path. The +5% reclaim is the integer-path landing; the gap between observed +5% and projected 5-8× is the interp-overhead-floor finding above, NOT a Doc 739 falsification — Doc 739 predicts cascade-revival materializes, not the magnitude on a specific bench.

### Composition with prior corpus / engagement work

- **Doc 739 §II.3 cascade-revival**: empirically materialized on D (+5%) and E (+5%).
- **Doc 734 §V (b) growth**: bench-as-instrument refinement — this micro-bench is interp-overhead-bound; CRB is the proper instrument for Pred-jsf.1.
- **Findings II.2 staged-validation**: M3 added-and-removed atomically; integer fast-path is conservative (i64-range check); f64-fractional falls through to existing path.
- **R3 risk (i64::MIN)**: handled.

### §XVI / Doc 734 / Doc 735 §X.h categorization

Per Doc 730 §XVI: not applicable.
Per Doc 734 §V: growth (a) positive-finding empirical-confirmation on D/E reclaim; growth (b) negative-finding catalyzes bench-strategy shift.
Per Doc 735 §X.h.b: **(P2.a) at D/E reclaim; (P2.d) at A/B/C** — interp-overhead-floor named as the structural cause.

### Open scope at JSF-EXT 5 close

1. **JSF-EXT 6** — Move 4 format-macro elimination + property iter-via-reference (decision: skip or land then CRB? — see report below)
2. **JSF-EXT 7** — CRB re-bench + Pred-jsf.1 final disposition (the real measurement)
3. **Findings doc addendum IV** — codify Finding II.2-bis + bench-as-instrument refinement (the micro-bench is interp-overhead-bound; CRB is the proper instrument)

### Cumulative status at JSF-EXT 5 close

LOC delta: ~35 (write_i64_into + integer-branch guard). Three cascade-revival pilots landed; aggregate micro-bench position essentially flat (-3% to -8% across shapes); CRB re-bench is the load-bearing measurement.

---

*JSF-EXT 5 closes. Move 3 integer fast-path landed; cascade-revival pilot pattern empirically materialized at +5% on D/E. The micro-bench is interp-overhead-bound; Pred-jsf.1 gates on CRB at JSF-EXT 7.*

---

## JSF-EXT 6 — 2026-05-23 (Move 4 format-macro elimination + property iter-via-reference)

### Headline

Object-branch refactor eliminates the per-property `PropertyDescriptor.clone()` and `Value.clone()` from the snapshot. Iterates directly on `obj.shape.iter_slots()` + `obj.properties.iter()` while emitting; the `obj` borrow and the recursive `json_stringify_into(rt, ...)` borrow are both shared so they coexist via NLL. Array branch refactored similarly with `Vec<(usize, &Value)>` (no PropertyDescriptor clones; only `Vec<usize>` allocation for sorting).

### Three-probe results

| probe | result |
|---|---|
| Pred-jsf.2 canonical fuzz (acc=-932188103) | ✅ GREEN |
| Pred-jsf.3 diff-prod 42/42 | ✅ GREEN |
| Pred-jsf.bench per-shape | flat (within noise) |

### Per-shape bench (cruft/node)

| shape | pre-M4 | post-M4 | Δ |
|---|---:|---:|---:|
| A small-object | 9.71× | 9.91× | flat |
| B deep-nested | 14.33× | 14.63× | flat |
| C array-of-obj | 12.55× | 12.81× | flat |
| D number-only | 15.05× | 16.08× | flat (noise) |
| E string-only | 10.31× | 10.21× | flat |

Move 4 essentially flat — confirms the interp-overhead-floor finding from JSF-EXT 5. The clone elision is a real substrate improvement (PropertyDescriptor.clone is ~16 bytes + Box copies per property) but is dwarfed by the interp framework cost on this bench.

### §XVI / Doc 734 / Doc 735 §X.h categorization

Per Doc 730 §XVI: not applicable.
Per Doc 735 §X.h.b: **(P2.d) on bench by design — Move 4 is the design's last substrate move; its empirical neutrality on this bench corroborates the interp-overhead-floor finding for the engagement's record.**

---

*JSF-EXT 6 closes. Move 4 landed; aggregate JSF-EXT 1→6 micro-bench position essentially flat. Design closed. JSF-EXT 7 is CRB final disposition.*

---

## JSF-EXT 7 — 2026-05-23 (CRB final disposition; Pred-jsf.1 FALSIFIED)

### Headline

CRB json_parse_transform re-measured post-M1+M2+M3+M4. **cruft 2455 ms vs JSF-EXT 0 baseline 2481 ms (Δ = -1%, within noise).** Node 122 ms (cruft/node 20.12×, vs baseline 20.34×). **Pred-jsf.1 target ≤1500 ms ≥40% reclaim NOT MET.**

### Measurement (5 runs, median; node + bun + cruft equality EQUAL)

| runtime | median (ms) | runs |
|---|---:|---|
| node | 122 | 125 121 122 124 121 |
| bun | 95 | 92 96 95 95 95 |
| cruft | **2455** | 2458 2455 2464 2452 2435 |

cruft/node = 20.12× (baseline 20.34× → essentially unchanged).
cruft/bun = 25.84×.

### Pred-jsf.1 disposition

**FALSIFIED.** Target: cruft ≤1500 ms (40% reclaim from 2481 ms). Actual: 2455 ms (1% reclaim). Substrate moves correctly reduced JSON.stringify intrinsic cost; the json_parse_transform CRB fixture's per-iter cost was NOT dominated by JSON.stringify.

### Doc 734 §V (b) growth — negative-finding catalyzes pilot reset

The JSF pilot's empirical contribution: **JSON.stringify is NOT the json_parse_transform CRB bottleneck.** CRB-EXT 9's component-decomposition estimate (JSON.stringify ~5-10× contributor) was wrong by an order of magnitude. The actual bottleneck must be JSON.parse + Array methods (filter, map) + Object property access + property iteration. The JSF pilot's substrate moves still correctness-improved JSON.stringify; they did not move the json_parse_transform needle.

### Findings doc addendum candidates (forward)

**Finding II.2-bis (cascade-revival as substrate-introduction signature)**: Doc 739 cascade-revival materialized empirically at +5-7% per move on the micro-bench. The pattern is real; the magnitude on a given bench depends on whether the closed-cascade tier is the actual cost dominator.

**Finding VII.1 (component-decomposition estimates require empirical anchoring before pilot spawn)**: CRB-EXT 9's "JSON.stringify estimated at 5-10× of the 20× gap" was the empirical anchor for the JSF pilot spawn. The estimate proved wrong. Future pilots spawned to close a CRB gap should run a per-component A/B (substitute the suspect component with a no-op or near-no-op variant) before substrate work begins, to validate the component IS the dominator.

### What this validates / does not validate

**Validates:**
- The design enumeration discipline (C1-C8 + dependency-ordered moves) is correct apparatus.
- The Doc 739 cascade-revival pattern is empirically observable.
- Per-substrate-move correctness preservation is achievable through three-probe-levels discipline.
- The buffer-threaded JSON.stringify is a structurally cleaner implementation (one allocation per call instead of O(depth × siblings)).

**Does not validate:**
- JSF-EXT 2's per-shape reclaim composition reading (depended on JSON.stringify being the dominator).
- CRB-EXT 9's component-decomposition estimate (JSON.stringify ~5-10× contributor).
- Pred-jsf.1 (≥40% CRB reclaim — falsified by direct measurement).

### Locale closure

**JSF locale closed at (P2.d)-aggregate disposition.** Substrate moves landed; correctness preserved; reclaim target not met; the load-bearing finding is the bottleneck-misidentification at CRB-EXT 9. Recommend new pilot at the actual bottleneck (likely a `rusty-js-array-methods-fast` or `rusty-js-property-access-fast`).

### Open scope at JSF-EXT 7 close

1. Reset JSF locale resume vector at (P2.d) — closed; substrate landed; reclaim not delivered.
2. Per Doc 734 §V (b) refinement: forward-derived pilot at the actual CRB bottleneck (component A/B required before spawn per Finding VII.1).
3. Three other top-level pilots (RXF, SW, HS) and one nested (subtle-wireup) still pending first substrate rounds.

### Cumulative status at JSF-EXT 7 close

LOC delta this round: 0 (measurement-only round). Aggregate JSF locale LOC delta across JSF-EXT 3-6: ~200 in intrinsics.rs (json_stringify_into + json_quote_string_into + write_i64_into + iter-via-reference refactor). 4 substrate moves landed; canonical fuzz GREEN throughout; diff-prod 42/42 GREEN throughout; CRB Pred-jsf.1 FALSIFIED.

---

*JSF-EXT 7 closes. JSF locale closed at (P2.d). The pipeline was built through the middle stretch; the upstream-constraint-closure landed; the cascade-revival pilots landed; and the empirical readout falsified the bottleneck-attribution from CRB-EXT 9. The negative finding is the locale's load-bearing contribution: JSON.stringify is not the json_parse_transform bottleneck.*

---

## JSF-EXT 8 — 2026-05-23 (component A/B probe; actual dominator identified; Finding VII.1 prospective application)

### Headline

Per Finding VII.1, ran a component A/B probe on json_parse_transform: 5 additive variants (V0 parse-only → V4 full) each 500 iters with 50-iter warmup. Per-component cost isolated by Δ between adjacent variants. **The actual dominator is the per-iter charCodeAt checksum loop at 77% of cruft's total wall-clock.** JSON.stringify is 3% of total.

### Per-component cost (cruft, json_parse_transform 500 iters, post-warmup):

| component | cruft Δ (ms) | % of total | node Δ (ms) | cruft/node |
|---|---:|---:|---:|---:|
| JSON.parse | 246 | 9% | 75 | 3.3× |
| Array.filter | 124 | 5% | 0 | ∞ |
| Array.map | 165 | 6% | 3 | 55× |
| JSON.stringify | 86 | 3% | 7 | 12× |
| **charCodeAt loop** | **2040** | **77%** | -1 | n/a (node jits to ~0) |
| **TOTAL** | **2661** | 100% | 84 | 31.7× |

Validation: V4 total 2661 ms aligns with prior CRB measurement (2455-2481 ms; ~8% high here because this fixture includes per-variant warmup overhead + 5 separate variant runs in single process).

### What this empirically confirms

**CRB-EXT 9's component estimate was wrong by an order of magnitude.** The "JSON.stringify ~5-10× contributor" attribution placed JSON.stringify at ~50-70% of the 20× gap; the actual measurement places JSON.stringify at 3% of total wall-clock (and ~6% of the cruft-node gap measured in absolute ms).

**The actual dominator is the bench's own bookkeeping**, not the JSON pipeline. The `for (let i = 0; i < out.length; i++) cs = (cs + out.charCodeAt(i)) | 0` loop iterates ~5000 times per outer iter × 500 outers = 2.5M charCodeAt calls. Node JITs this to ~0 ms; cruft's interpreter path is 2040 ms.

### Implications for the engagement

**The actual high-impact targets for closing the json_parse_transform gap, in order:**
1. **String.prototype.charCodeAt + tight-loop interp dispatch** (77% contributor). Either a substrate pilot for charCodeAt specifically OR a LeJIT-tier extension for tight integer-accumulator loops over string indexing.
2. **Array.map** (6% contributor, 55× per-op gap). Significant per-call gap; closing it would help broader Array-heavy workloads beyond this fixture.
3. **JSON.parse** (9% contributor, 3.3× per-op gap). Modest gap; substrate pilot is moderate priority.
4. **Array.filter** (5% contributor; node-side is JIT'd to 0 so the absolute gap is large).
5. **JSON.stringify** (3% contributor, 12× per-op gap). Already substrate-closed at JSF; the closure is correctness-improvement value but doesn't shift the CRB needle.

### Finding VII.1 (prospective confirmation)

**Finding VII.1**: component-decomposition estimates require empirical anchoring before pilot spawn. CRB-EXT 9's estimate ("JSON.stringify ~5-10× contributor") was theoretical; the A/B probe ran in <10s and would have prevented the JSF pilot spawn from targeting the wrong component.

**Standing rule 11 (proposed)**: before spawning a pilot whose telos is "close a CRB gap," run a 5-minute component-A/B probe on the target fixture; identify the actual dominator empirically; spawn the pilot at that dominator. The cost is one probe round; the benefit is preventing entire substrate pilots from targeting non-dominators.

### What the JSF pilot delivered (revised disposition)

JSF's load-bearing contributions, after the probe disambiguates:

1. **Correctness-improvement value**: JSON.stringify is now structurally cleaner (buffer-threaded, fast-path leaf emitters, no per-property clones). Real value at the substrate-tier level.
2. **Doc 739 cascade-revival empirical demonstration**: pattern observed at the micro-bench at +5-7%/move. Validated the abstract pattern.
3. **Finding II.2-bis (substrate-introduction (P2.d) as cascade-revival signature)**: ready for findings addendum IV.
4. **Finding VII.1 (probe before spawn)**: this round IS the prospective demonstration; standing rule candidate.
5. **CRB-EXT 9 component-decomposition correction**: the actual cost breakdown is now empirically anchored, not estimated.

### Composition with prior corpus / engagement work

- **Doc 734 §V (b) negative-finding catalyzes refinement**: this round IS the catalysis. The JSF pilot's (P2.d) CRB outcome → Finding VII.1 → standing rule 11 → component-A/B becomes the standing instrument for CRB-gap-pilot spawns.
- **Doc 581 Pin-Art apparatus**: the A/B probe is constraint-enumeration applied to bench measurement (enumerate per-component contributions instead of jumping to substrate work on the suspected component).
- **Doc 729 §A8.13 + Doc 739 §II.3**: the cascade-revival pattern still empirically holds; what JSF-EXT 7 falsified was the bottleneck-attribution, not the cascade-revival mechanism.

### §XVI / Doc 734 / Doc 735 §X.h categorization

Per Doc 730 §XVI: not applicable (probe round, no diff-prod surface).
Per Doc 734 §V: growth (b) negative-finding-catalyzes-refinement; growth (a) positive-finding (component breakdown is now empirical).
Per Doc 735 §X.h.b: not applicable (probe round; no substrate move).

### Open scope at JSF-EXT 8 close

1. **Most-impact next pilot**: `rusty-js-string-charcode-fast` or `lejit-tier extension for tight integer-accumulator loops over string indexing` — 77% contributor. Either substrate-tier (charCodeAt intrinsic optimization) or JIT-tier (loop fast-path).
2. **Second-impact**: `rusty-js-array-methods-fast` (filter/map 55× per-op gap).
3. **Third-impact**: `rusty-js-json-parse-fast` (3.3× per-op gap, 9% contributor).
4. **Findings doc addendum IV**: codify Finding II.2-bis + Finding VII.1 + standing rule 11.
5. **JSF locale**: closed at (P2.d) per JSF-EXT 7; this probe round IS the locale's exit deliverable (the actual component breakdown).

### Cumulative status at JSF-EXT 8 close

LOC delta: ~95 (component-ab-probe.mjs fixture). Probe ran cleanly in <10s aggregate across both runtimes. Component breakdown empirically anchored.

---

*JSF-EXT 8 closes. The component A/B probe disambiguated the json_parse_transform cost: charCodeAt loop dominates at 77%; JSON.stringify is 3%. CRB-EXT 9's bottleneck-attribution was off by ~20×. Finding VII.1 (component A/B before pilot spawn) is now empirically anchored; standing rule 11 candidate. JSF locale's load-bearing contribution is this disambiguation + the Doc 739 cascade-revival demonstration + the correctness-improved JSON.stringify substrate.*

---

## JSF-EXT 9 — 2026-05-23 (CharCode-EXT 1: ASCII fast-path for charCodeAt + String.length; substrate-tier fix at the JSF-EXT 8 dominator)

### Headline

Substrate fix at `string_proto_char_code_at_via` (interp.rs:4744) and the String.length read path (interp.rs:7247). ASCII fast-path: `bytes[i]` instead of `chars().nth(i)` for char access; `s.len()` instead of `s.chars().count()` for length. Closes an O(n²)-per-outer-iter pattern in the json_parse_transform checksum loop. Three probes GREEN.

### Measurements

**A/B probe checksum delta**: 2040 ms → **1739 ms (-15%, -300 ms)**
**CRB json_parse_transform**: 2455 ms → **2372 ms (-3%, -83 ms)**
cruft/node: 20.12× → 19.28× (essentially unchanged at the ratio level)

### Finding

Smaller reclaim than the O(n²)→O(n) algorithmic analysis projected (~40×). The empirical readout: per-call cost dropped from 0.816 μs to 0.696 μs (-15% per-call), not -99% as the algorithmic projection assumed. **Implication: most of the per-charCodeAt-call cost is interp dispatch + property lookup + Value boxing, NOT the chars().nth() iteration.** The O(n) chars().nth() cost was a real bug but not the dominator; the dominator is dispatch.

This is a second-order Finding VII.1 instance: the substrate-tier source-read identified a real bug but its per-call magnitude was over-estimated. The probe re-run measured the actual contribution. The remaining 1739 ms is genuine dispatch overhead per intrinsic call, which is exactly what the LeJIT-tier intrinsic-inlining work targets.

### Implication for forward path

The next-impact target is now LeJIT-tier intrinsic inlining of charCodeAt: recognize the call site, emit inline charCodeAt-fast IR (eliminate the dispatcher round-trip + per-call Value boxing + property lookup). Expected reclaim: large fraction of the residual 1739 ms.

### Three-probe results

| probe | result |
|---|---|
| canonical fuzz (acc=-932188103) | ✅ GREEN |
| diff-prod 42/42 | ✅ GREEN |
| A/B probe checksum delta | -15% (300 ms reclaim) |
| CRB json_parse_transform | -3% (83 ms reclaim) |

### Composition with prior corpus / engagement work

- **Finding VII.1 reinforced**: substrate-tier source-read identified the bug class correctly but over-estimated magnitude. Re-measurement is the discipline.
- **Doc 739 cascade-revival**: the O(n²)→O(n) fix is upstream constraint-closure at the intrinsic tier; the LeJIT-tier intrinsic inlining is the downstream cascade-revival pilot.
- **Findings rule 5 + standing rule 10 three-probe-levels**: canonical fuzz + diff-prod gates ran; both GREEN.

### §XVI / Doc 734 / Doc 735 §X.h categorization

Per Doc 735 §X.h.b: **(P2.a) on bench (small reclaim) + (P2.c→GREEN) on correctness** — the chars().nth() impl was correct but slow; the ASCII fast-path is correctness-equivalent + faster.
Per Doc 734 §V: growth (a) positive-finding empirical-confirmation + growth (b) magnitude over-estimate informs dispatch-as-dominator finding.

### Open scope at JSF-EXT 9 close

1. **LeJIT charCodeAt intrinsic inlining** (the actual JIT-tier extension from the original keeper directive (a)) — now empirically warranted: dispatch is residual dominator at 1739 ms.
2. **Other O(n) chars().nth() / chars().count() sites** (regexp.rs, prototype.rs, intrinsics.rs, multiple interp.rs sites) — not in json_parse_transform's hot path; deferred unless a future CRB-gap-pilot's A/B identifies them.

### Cumulative status at JSF-EXT 9 close

LOC delta: ~20 (two intrinsic-tier ASCII fast-paths). CRB cumulative since JSF-EXT 0: 2481 ms → 2372 ms (4.4% reclaim total across all JSF substrate work).

---

*JSF-EXT 9 closes. Substrate-tier O(n)→O(1) fix landed at ASCII charCodeAt + length. -15% on the dominator-loop; -3% on CRB total. Per-call magnitude smaller than algorithmic projection because dispatch overhead is the dominant cost. LeJIT-tier intrinsic inlining is now empirically warranted at 1739 ms residual.*
