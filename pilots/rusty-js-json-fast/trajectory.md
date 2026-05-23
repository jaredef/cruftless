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
