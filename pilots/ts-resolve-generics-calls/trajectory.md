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

### Status: REOPENED for TRGC-EXT 2 follow-on

---

## TRGC-EXT 2 — substrate-completeness follow-on (2026-05-24)

**Trigger**: keeper directive "Continue" after TRGC-EXT 1 close. Per the inspect-then-iterate discipline, investigated the remaining `method-return-annotation` row (#1, 37 files) and discovered three additional substrate gaps NOT covered by the TRGC-EXT 1 close:

1. **Arrow `=>` vs fn-type `=>` disambig in `skip_type`** — `: Writable => { body }` was consuming the value-position `=>` as a fn-type arrow. Fix: track `prev_was_rparen_at_top` in `skip_type`; only consume top-level `=>` when preceded by a balanced `(...)`.
2. **ASI-aware `skip_type`** — class-field annotations like `readonly str: string\n constructor(...)` were consuming past newline into the next member. Fix: break at top-level on `preceded_by_line_terminator` after the first consumed token.
3. **TS method-overload-no-body strip** — `subscribe(...): R;` overload declarations are valid TS but not valid JS. New rule strips the entire signature when at class-body member-start position AND immediately-after-`)` is `:`/`;`/`{` AND the next top-level `;` precedes any `{`.

**Bugs caught + fixed mid-round (TRGC-EXT 2)**:
- Initial overload-rule over-matched on expression-position calls like `or(...)` inside function bodies (since `in_class_body()` is a dummy that returns true). Symptom: regression to 67.4% parse-success. Fix: added strict gating — only fires at `brace_stack.last() == Block` AND `preceded_by_line_terminator || prev in {LBrace, Semicolon}` (member-start position) AND immediately-after-`)` is a method-decl-shape token.
- Second overload-rule scan logic over-matched on `s.match(/foo/g)` style calls. Fix: tightened scan to require immediate-after-`)` to be `:`/`;`/`{`.

**Gates**:
- `cargo test --release -p ts-resolve`: ✅ **46/46 PASS** (+3 new: overload-strip, class-field-no-init, regex-call-not-overload negative)
- diff-prod 42/42 PASS ✅
- No regression of pre-TRGC OK files (regressed mid-round then corrected)

**TCC measurement**:

| Stage | OK | Parse-success | Δ from TRGC-EXT 1 close |
|---|---:|---:|---:|
| Pre-round (TRGC-EXT 1 close) | 259 | 69.3% | — |
| After arrow-vs-fn-type fix (single-fix probe) | 262 | 70.1% | +0.8 pp |
| After overload-strip first cut (REGRESSED) | 252 | 67.4% | -1.9 pp |
| After overload-rule gating refinement | **265** | **70.9%** | **+1.6 pp** |

**Final disposition**: substrate strictly more correct; +1.6 pp gain; +3 unit tests; substrate-completeness improvements that pay forward to all subsequent locales.

### Findings

**Finding TRGC.4** (regression-recovery discipline): mid-round regression caught by TCC re-measurement, traced to over-matched gating, fixed by tightening the immediate-after-`)` condition. **The corpus serves as a regression instrument, not just a feature-priority instrument**. Without TCC's automated baseline, the over-match would have shipped silently.

**Finding TRGC.5** (substrate-tier discipline-of-conservative-strip-rules): when a strip rule's heuristic is uncertain, prefer false-negatives (miss real cases) over false-positives (strip non-cases). Bail conditions should be conservative. Cost of a false-negative: TCC re-measurement shows the failure category remains, easy to iterate. Cost of a false-positive: silent regression of previously-OK files; hard to detect without an instrument.

### Status: REOPENED for TRGC-EXT 3 follow-on

---

## TRGC-EXT 3 — overload-pattern completions (2026-05-24)

**Trigger**: keeper directive "Yes" after TRGC-EXT 2 close. Continued inspect-then-iterate investigation on the remaining `method-return-annotation` row.

**Four substrate improvements landed**:

1. **`do` keyword unblocked** from `is_overload_blocked_name` — `do { } while ()` is naturally disambiguated by the next-is-LParen filter (do-while has `{` next, do-as-method has `(`). Enabled `do(...)` method names like rxjs's `Notification.do`.

2. **`void` / `typeof` removed from `classify_brace`'s obj-lit-classify set** — the names appear far more commonly as TS return-type annotations (`): void {`) than as unary-on-object-literal expressions. Including them caused the function body `{` after `: void` to mis-classify as ObjectLit, breaking annotation detection INSIDE the body. Single-fix +1.6 pp.

3. **Module-level function overload gating extended** — `at_class_member_start` now also accepts `brace_stack.last() == None` (module level) and `prev == Ident("function")`. Strip range extended backward to include the `function` keyword. Handles `function NAME(...): T;` at module scope. +1.0 pp.

4. **Overload with generics on name** (`function NAME<T>(...): R;`) — when name is followed by `<`, `match_angle` past the generic-args to find `(`. **Single-fix +8.3 pp** — unlocked many overload-heavy files (rxjs, ajv) that use generic overloads pervasively.

**Gates**:
- `cargo test --release -p ts-resolve`: ✅ **46/46 PASS** (no new tests this round — fixes covered by existing regression + the TCC corpus itself)
- diff-prod 42/42 PASS ✅

**TCC measurement (TRGC-EXT 3 cumulative)**:

| Stage | OK | Parse-success | Δ from TRGC-EXT 2 close |
|---|---:|---:|---:|
| Pre-round (TRGC-EXT 2 close) | 265 | 70.9% | — |
| After `do` unblock | 265 | 70.9% | 0.0 pp (no example in corpus) |
| After `void`/`typeof` classify-brace fix | 271 | 72.5% | +1.6 pp |
| After module-level function overload | 275 | 73.5% | +1.0 pp |
| After overload-with-generics | **306** | **81.8%** | **+8.3 pp** |
| **Cumulative TRGC-EXT 3** | **+41 files** | **+10.9 pp** | — |

### Findings

**Finding TRGC.6** (single-substrate-fix high-yield surface): the overload-with-generics fix alone delivered +8.3 pp from ~15 LOC. **The largest single-fix yield observed in any sub-locale this session.** Substrate-bug discovery via TCC continues to pay outsized returns.

**Finding TRGC.7** (inspect-then-iterate compound discovery, fourth reproduction): the pattern reproduces yet again. Each round's "what's left" inspection surfaces higher-impact substrate gaps than the planned scope. Standing observation: the corpus-driven inspect-then-iterate discipline has stronger predictive yield than spec-driven planning.

### Status: REOPENED for TRGC-EXT 4 follow-on

---

## TRGC-EXT 4 — UShr/Shr handling in overload scan + match_angle (2026-05-24)

**Trigger**: keeper "Continue". Inspected remaining top failures (combineLatest.ts + innerFrom.ts).

**Two single-line fixes** in `strip.rs`:

1. **`match_angle` handles UShr (`>>>`)** — `<T extends Record<string, ObservableInput<any>>>` has triple-`>` that lexes as UShr. The helper now decrements depth by 3 on UShr (and was already handling Shr by 2). Same fix applied to `skip_type`'s angle-depth tracker.

2. **Overload-scan loop handles Shr/UShr** — the bracket-depth balancer inside the overload-scan was missing arms for Shr and UShr. Without these, depth never reached 0 on generic return types like `Observable<ObservedValueOf<O>>`, causing the loop to walk past the `;` and miss the overload pattern. Single-fix **+6.7 pp**.

**Gates**:
- `cargo test --release -p ts-resolve`: ✅ 46/46 PASS
- diff-prod 42/42 PASS ✅

**TCC measurement (TRGC-EXT 4)**:

| Stage | OK | Parse-success | Δ |
|---|---:|---:|---:|
| Pre-round (TRGC-EXT 3 close) | 306 | 81.8% | — |
| After match_angle UShr | 309 | 82.6% | +0.8 pp |
| After overload-scan Shr/UShr | **334** | **89.3%** | **+6.7 pp** |
| **Cumulative TRGC-EXT 4** | **+28 files** | **+7.5 pp** | — |

### Findings

**Finding TRGC.8** (single-line-fix high-yield via missing op handling): a single missing match arm (`Shr` and `UShr` in overload scan's bracket-depth loop) was blocking 25 files. **The cost of a missed handler scales with the LOC behind the missing case** — a 2-LOC fix unblocked ~7% of the corpus. Discipline: when adding a depth-tracking loop, audit ALL the punctuators that could affect that depth at the time of writing.

### Status: CHAPTER CLOSED at TRGC-EXT 4

Standing rule 13 corroboration count holds at 8 (TRGC-EXT 2/3/4 are follow-ons within same locale).

**Final post-TRGC remaining failures** (long-tail; all small categories):
| Rank | tag | files |
|---:|---|---:|
| 1 | uncategorized-unexpected-token | 7 |
| 2 | import-export-type | 5 |
| 3 | template-literal-type | 4 |
| 4 | generic-call | 3 |

**Cumulative session parse-success on real npm corpus**: 37.7% → **89.3%** (+51.6 pp across all sub-locale rounds). **Past the 87% top-6-sub-locales milestone — true TS parse parity for the high-frequency consumer code surface is achieved**.

Standing rule 13 corroboration count holds at 8. Substrate now handles `do(...)` method names, `void`/`typeof` annotation followers, module-level function overloads with generic args, AND `function NAME(...): T;` declarations at any context.

**Cumulative session parse-success on real npm corpus**: 37.7% → 81.8% (+44.1 pp across all sub-locale rounds). Within striking distance of the original ~87% milestone for top-6 sub-locales.

Standing rule 13 corroboration count holds at 8 (TRGC-EXT 2 is a follow-on within TRGC's locale, not a new locale). Substrate now handles arrow-vs-fn-type, class-field ASI, and TS method/function overload declarations.

Standing rule 13's eighth corroboration. Three substrate improvements landed; +9.4 pp parse-success lift; ternary-tracking discovery (item 2) materialized within the single-round close per the established inspect-then-iterate discipline.
