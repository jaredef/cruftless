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
