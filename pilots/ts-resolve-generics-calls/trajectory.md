# ts-resolve-generics-calls — Trajectory

## TRGC-EXT 0 — workstream founding (2026-05-24)

**Trigger**: keeper directive "Continue" at TRCAPS-EXT 1 close. Third data-driven sub-locale from TCC's failure-table (post-TRCAPS row 2, `generic-call` at 40 files).

**Inspection-before-spec**: examined `rxjs/src/internal/Observable.ts` (the row-2 example). Found two patterns under the single tag:
1. Generic arrow `static create: ... = <T>(subscribe?: ...) => { ... }` (line 52)
2. Generic instantiation `new Observable<T>(subscribe)` (line 53) + generic method `lift<R>(operator?: ...) { ... }` (line 67)

Both unambiguous via the `match_angle` close → `(` look-ahead filter.

**Founding artefacts**: seed.md + trajectory.md + scaffolded dirs. TRGC-EXT 1 (implementation + re-measure + close) next.

---

## TRGC-EXT 1 — implementation + chapter close (2026-05-24)

**Three improvements landed** in `strip.rs`:

1. **Unified `<...>(` strip rule** at `Op::Lt`: try `match_angle`; if close found AND next is `(`, strip the angle region. Covers all three generic-call shapes (`f<T>()`, `new X<T>()`, `methodName<T>(...)`) AND generic-arrow `<T>(...) =>`. Look-ahead-to-`(` filter resolves the `a < b` operator ambiguity.

2. **Ternary depth tracking** (mid-round discovery): `?` operators at expression positions push current paren_depth onto a `ternary_stack`; matching `:` pops and skips annotation handling. Fixes Ident-anchored ternary `:` mis-strips like `... ? META_SCHEMA_ID : undefined` that the earlier RParen-anchor disambig missed.

3. **`?` discriminators**: differentiates ternary `?` (push to stack) from optional-property `?:`, optional-chain `?.`, and nullish `??` (lexer-distinct).

**Mid-round discovery (Finding TRGC.1)**: the planned generic rules alone delivered +2.4 pp (40 → 28 generic-call files). The ternary-tracking rule, added after inspecting the next failure category, delivered an additional +7.0 pp by resolving Ident-anchored ternary mis-strips across many `method-return-annotation` false-positives. **The inspect-then-iterate pattern from TRCAPS.1 reproduced: each post-fix inspection surfaced a higher-impact substrate bug than the planned rule.**

**Gates**:
- `cargo build --release -p ts-resolve`: ✅ clean
- `cargo test --release -p ts-resolve`: ✅ **41/41 PASS** (+6 new regression tests: 4 generic patterns + 1 less-than-operator negative + 1 generic-arrow-class-field)
- `cargo build --release --bin cruft -p cruftless`: ✅ clean
- diff-prod 42/42 PASS ✅

**TCC measurement**:

| Stage | OK | Parse-success | Δ |
|---|---:|---:|---:|
| Pre-TRGC (post-TRCAPS) | 224 | 59.9% | — |
| After unified `<...>(` rule | 233 | 62.3% | +2.4 pp |
| After ternary tracking | **259** | **69.3%** | **+7.0 pp** |
| **Cumulative TRGC-EXT 1** | **+35 files** | **+9.4 pp** | over ≥8 pp target |

### Final disposition

| Predicate | Disposition |
|---|---|
| Pred-trgc.1 (≤80 LOC) | ✅ HELD at ~75 LOC |
| Pred-trgc.2 (35/35 tests + 5+ new regressions) | ✅ HELD at 41/41 (+6 tests) |
| Pred-trgc.3 (≥8 pp lift) | ✅ HELD STRONGLY at +9.4 pp |
| Pred-trgc.4 (no regression of 224 OK files) | ✅ HELD |
| Pred-trgc.5 (≤3 rounds) | ✅ **HELD at 1 implementation round** |

### Findings

**Finding TRGC.1** (inspect-then-iterate compound discovery, third reproduction): the seed planned two generic-strip rules; mid-round investigation surfaced a ternary-tracking substrate bug of greater impact than the planned rules (+7.0 pp vs +2.4 pp). The discipline now reproduces at three consecutive locales (TRSLS, TRCAPS, TRGC). **Standing observation worth codifying**: post-fix investigation routinely surfaces higher-impact substrate bugs than the planned scope.

**Finding TRGC.2** (standing rule 13 **eighth corroboration**): TRGC closed in 1 implementation round.

**Finding TRGC.3** (categorization mis-attribution rate): pre-TRGC `method-return-annotation` was 41 files; post-TRGC ternary-tracking fix dropped it to 37 files (only −4), while the SAME fix unblocked 26+ files across other categories. **The TCC heuristic structural-tag is correlative not causal at ~20-30% rate**; the actual root cause distribution per the standing-rule-13 corpus discipline favors substrate-bug-first investigation.

### Updated sub-locale priority

| Priority | Sub-locale | Addresses | Files |
|---:|---|---|---:|
| 1 | Inspect method-return-annotation row (37; another substrate bug likely per TRGC.1 pattern) | row 1 | 37 |
| 2 | Inspect generic-call row (27 remaining post-TRGC; structural variants we didn't catch) | row 2 | 27 |
| 3 | `ts-resolve-readonly-class-field` | row 3 | 9 |
| 4 | `ts-resolve-template-literal-types` (real ones) | row 5 | 5 |
| 5 | `ts-resolve-import-export-type` | row 6 | 5 |

Achieving 1-5 lifts parse-success **69.3% → ~88%**.

### Status: CHAPTER CLOSED at TRGC-EXT 1

Standing rule 13's eighth corroboration. Three substrate improvements landed; +9.4 pp parse-success lift; ternary-tracking discovery (item 2) materialized within the single-round close per the established inspect-then-iterate discipline.
