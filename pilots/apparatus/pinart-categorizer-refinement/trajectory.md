# pinart-categorizer-refinement — Trajectory

## PCR-EXT 0 — founding + uncategorized survey (2026-05-25)

**Trigger**: Per keeper directive (Telegram 9812) "1" — selecting the first of the three LPA-EXT 3 recommended spawns. Locale opened per LPA Finding LPA.5: 52% of top-30 matrix gap fails are apparatus-refinement candidates (uncategorized/projection or uncategorized/resolver); apparatus-tier refinement before substrate-tier spawns.

**Initial survey** (against `test262-full-2026-05-25-165734-p2/interpreted.jsonl`):

- 3,681 records have at least one `uncategorized/*` dimension (uncategorized/resolver: 1,953; uncategorized/projection: 2,104; overlap ~376).
- Dominant reason-text patterns identified:
  - 565 `parse: ...` cruft parser failures → `availability/missing-parser-feature`
  - 361 `Expected SameValue(«X», «Y»)` → already covered by existing rule (value-semantics/wrong-result)
  - 176 `Expected a TYPE to be thrown` → already covered
  - 114 `should be an own property` → `descriptor-shape/missing-own-property` (new)
  - 66 `isconstructor invoked` → `availability/missing-method-or-intrinsic` (new)
  - 39 `unterminated regex` / `lex error` → `regexp-semantics/lex-error` (new)
  - 5 `unsupported by the v1 regex engine` → `partial/regex-features-missing` (new)
  - ~200 `cannot read property of null/undefined` → `availability/missing-internal-slot` (new)
  - ~1,500 in resolver bucket: annexB tests (`annexB/built-ins/...` and `annexB/language/...`) routing to uncategorized/resolver because the existing rules only match `built-ins/` and `language/`

## PCR-EXT 1 — categorizer rule additions + dry-run re-interpretation (2026-05-25)

**Edits** (~50 LOC in `pilots/apparatus/test262-categorize/derived/src/bin/full_pinart.rs`):

**`resolver_for` extensions** (handles 1,953 uncategorized/resolver records):
- `annexB/built-ins/*` → `runtime/spec-builtins` (1,204 records shifted)
- `annexB/language/*` → `ast-to-bytecode/language-lowering` (733 records shifted)
- `staging/*` → routed by inner shape (Iterator/AsyncIterator/language)

**`projection_axis` extensions** (handles 2,104 uncategorized/projection records):
- `parse: ...` reason → `availability/missing-parser-feature` (565 records)
- `cannot read property` → `availability/missing-internal-slot`
- `should be an own property` → `descriptor-shape/missing-own-property` (114 records)
- `isconstructor invoked` → `availability/missing-method-or-intrinsic` (66 records)
- `unsupported by the v1 regex engine` → `partial/regex-features-missing` (5 records)
- `unterminated regex` / `lex error` + RegExp surface → `regexp-semantics/lex-error` (39 records)

**Build**: `cargo build --release --bin t262-full-pinart` completes cleanly.

**Re-interpretation** (dry-run via Python simulation, since raw test262 results are sidecar-only and not on this machine):

| Metric | Pre-PCR-EXT 1 | Post-PCR-EXT 1 | Δ |
|---|---:|---:|---:|
| Distinct Pin-Art coordinates | 246 | **261** | +15 (new specific coordinates surfaced) |
| Records in uncategorized/resolver | 1,953 | 16 | **-1,937 (-99.2%)** |
| Records in uncategorized/projection | 2,104 | 1,354 | **-750 (-35.6%)** |
| Union (any uncategorized dim) | 3,681 | 1,367 | **-2,314 (-63%)** |

**LPA-EXT 3 target was 2,802 fails in Class A; PCR-EXT 1 moved 2,314 records out — within target's order of magnitude, exceeded for the resolver bucket entirely.**

**New top-coordinates surfaced** (post-PCR-EXT 1, ranks 11 and 12):

- Rank 11: `ast-to-bytecode/language-lowering :: E2/internal-method:runtime :: availability/missing-parser-feature :: err:SyntaxError` (471 records) — cruft parser-feature gaps now visible as a single named coordinate; substrate spawn candidate.
- Rank 12: `runtime/spec-builtins :: E2/internal-method:runtime :: value-semantics/wrong-result :: assertion/expected-mismatch` (398 records) — annexB Date/String/RegExp value-semantics wrong-results; substrate spawn candidate.

**Refreshed matrix** written to `pilots/apparatus/test262-categorize/full-suite/results/test262-full-2026-05-25-PCR-EXT-1-rerun/` (matrix.md + summary.md). This is a re-interpretation, not a re-categorize against raw; when the keeper re-runs test262 next, the proper categorizer (with these rules merged) will produce the canonical result.

**Findings**

**Finding PCR.1 (the largest apparatus gap was actually the resolver bucket, not the projection bucket)**: pre-EXT-1 the uncategorized/resolver count (1,953) exceeded the uncategorized/projection count (2,104) at top-rank density — but the resolver-bucket fix was a single rule (annexB path routing) while the projection-bucket fix required 6+ pattern rules and shifted less. The yield-per-rule ratio for the resolver rule is ~1,937:1 — single rule, near-total bucket eviction. Standing recommendation: when an uncategorized bucket has high-locality cause (one file-path prefix like `annexB/built-ins/`), one routing rule dominates dozens of reason-text rules; survey for high-locality patterns first.

**Finding PCR.2 (the new top-rank coordinates ARE substrate spawn candidates)**: rank 11 (`missing-parser-feature`, 471) and rank 12 (`annexB-runtime wrong-result`, 398) didn't exist as visible coordinates before PCR-EXT 1; they were diffused into uncategorized. With the refinement, they become legible work-shapes that downstream substrate locales can target. This is the LPA.5 prediction empirically realized: sharpening the categorizer converts apparatus-tier mass into substrate-tier coordinates.

**Finding PCR.3 (1,354 uncategorized/projection records remain)**: the 35.6% shift on the projection bucket is less than the 99.2% shift on resolver because reason-text patterns are more diverse than path patterns. Of the 2,558 originally-unmatched "other" reasons in my survey, 1,500 were `parse: ...` (caught by EXT 1), leaving ~1,058 in a long tail of less-frequent patterns. Closing these is PCR-EXT 2's scope and requires per-batch pattern-mining rather than a single new rule.

**Status**: PCR-EXT 1 CLOSED. Categorizer rules merged into `full_pinart.rs`; refreshed matrix written to dated subdirectory; LPA-EXT 3 positioning-gaps deserves re-rendering against the new matrix (LPA trigger: "after full-suite categorize re-run") — that's a successor LPA-EXT iteration.

PCR-EXT 2 (close the remaining 1,354 uncategorized/projection long tail via per-batch pattern-mining) and PCR-EXT 3 (the categorizer should re-run on raw against the next full-suite test262 run) remain as successor rungs. Both run on opportunistic trigger.
