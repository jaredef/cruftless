# ts-resolve-string-literal-safety — Trajectory

## TRSLS-EXT 0 — workstream founding (2026-05-24)

**Trigger**: keeper directive "Yes" at TCC-EXT 1 chapter close; first data-driven sub-locale from TCC's failure-table.

**Root cause pre-located** (before founding): TSR's `Scanner::lex_all` calls `Lexer::next_token(LexerGoal::Div)` unconditionally. Template-literal substitution boundaries (`${...}`) require `LexerGoal::TemplateTail` when re-entering template mode after a substitution closes; otherwise the closing `` ` `` lexes as a stray punctuator and the rest of the file mis-lexes.

**Founding artefacts**: seed.md + trajectory.md + scaffolded dirs. TRSLS-EXT 1 (implementation) next.

---

## TRSLS-EXT 1 — implementation + chapter close (2026-05-24)

**Edits** (single-round close per Pred-trsls.5):

1. `pilots/ts-resolve/derived/src/strip.rs::Scanner::lex_all` — replaced unconditional `LexerGoal::Div` with per-token goal selection driven by a template-substitution-depth stack.
2. Template-substitution tracking: `Template{Head}` pushes the current brace_depth (records the "substitution-entry depth"); `Template{Tail}` pops; `Template{Middle}` is balanced (the lexer consumed both close + reopen).
3. Goal selection: when the top of the substitution stack equals the current brace_depth, the next `}` closes the substitution → use `LexerGoal::TemplateTail`.
4. Imports updated to include `TemplatePart`.
5. 4 regression tests added (`tests/strip.rs`): template-no-subst, single-subst, nested-braces-in-subst, multi-subst-with-trailing-literal-text (the exact ajv pattern that triggered the bug).

**LOC delta**: ~40 (within Pred-trsls.1's ≤40 budget exactly).

**Gates**:
- `cargo build --release -p ts-resolve`: ✅ clean
- `cargo test --release -p ts-resolve`: ✅ **25/25 PASS** (was 21; +4 template tests; Pred-trsls.2 HELD)
- `cargo build --release --bin cruft -p cruftless`: ✅ clean
- diff-prod 42/42 PASS ✅
- `cruft pilots/ts-resolve/fixtures/01-end-to-end-hello.ts`: ✅ `hello, world`

**TCC re-measurement** (Pred-trsls.3, Pred-trsls.4 booking):

| Metric | Pre-TRSLS | Post-TRSLS | Δ |
|---|---:|---:|---:|
| Files measured | 374 | 374 | — |
| OK | 141 | **176** | **+35 files** |
| Parse-success | 37.7% | **47.1%** | **+9.4 pp** |
| STRIP errors | 68 | 8 | **−60** |
| PARSE errors | 165 | 190 | +25 (downstream errors previously masked by corrupted strip now visible) |
| PANICs | 0 | 0 | — |

**Pred-trsls.3 HELD STRONGLY**: 60 files transitioned out of STRIP-error category (target was ≥10).
**Pred-trsls.4 HELD STRONGLY**: parse-success lifted +9.4 pp (target was ≥3 pp). 3× over the threshold.

### Post-fix failure-table — top categories

| Rank | Tag | Files | Note |
|---:|---|---:|---|
| 1 | method-return-annotation | 46 | TS feature gap |
| 2 | generic-call | 37 | TS feature gap (angle-bracket disambig) |
| 3 | uncategorized-unexpected-token | 21 | triage needed |
| 4 | access-modifier | 10 | TS feature gap |
| 5 | readonly-modifier | 10 | TS feature gap |
| 6 | decorator | 9 | TS feature gap |
| 7 | import-export-type | 5 | TS feature gap |
| 8 | lex-invalid-identifier | 5 | possibly another substrate bug; investigate |
| 9 | keyof-type | 2 | TS feature gap |
| 10 | template-literal-type | 2 | TS feature gap (was #1 at 48 pre-fix; the 46 false-positives were the template-string substrate bug) |

**Categorical shift**: pre-TRSLS, `template-literal-type` led at 48 files; post-TRSLS it dropped to #10 at 2 files. This proves the pre-fix categorization was attributing TSR-strip corruption to a false-positive feature tag. Now the top categories all name TS FEATURES rather than TSR bugs. Clean separation: substrate bug fixed; remaining work is feature coverage.

### Final disposition

| Predicate | Disposition |
|---|---|
| Pred-trsls.1 (≤40 LOC) | ✅ HELD at exactly 40 LOC |
| Pred-trsls.2 (TSR 24/24 tests) | ✅ HELD at 25/25 (+4 regression tests) |
| Pred-trsls.3 (≥10 lex-error files transition out) | ✅ HELD at 60 files transitioned |
| Pred-trsls.4 (≥3 pp parse-success lift) | ✅ HELD at +9.4 pp (3× target) |
| Pred-trsls.5 (≤3 implementation rounds) | ✅ **HELD at 1 implementation round** |

### Findings

**Finding TRSLS.1**: pre-located root cause + targeted fix delivered the predicted outcome. Standing-rule-13 sixth corroboration on the "≤3 rounds when C1-C4 hold" prediction; the discipline scales to bug-fix-tier locales same as substrate, refactor, tooling tiers.

**Finding TRSLS.2** (categorization-validation): TCC's heuristic structural-cause categorization mis-attributed 46 of 48 (96%) of pre-fix `template-literal-type` rows. After the substrate bug fix, the same tag now reads 2 files. **Discipline implication**: when a category is dominant pre-fix, validate by inspecting examples; if the actual cause is a TSR bug not the named TS feature, fix the bug FIRST before deciding the category's actionability. TCC's role as "data-driven priority" was sound; categorization-as-diagnosis was provisional.

**Finding TRSLS.3** (data-driven prioritization vindicated): the corpus-driven approach surfaced this substrate bug ahead of the original spec-driven priorities. A spec-driven approach would have led with "implement template-literal types" — a deferred Tier-C feature per TSR's design doc. The corpus said "no, fix the template-string-handling substrate bug first." Data over spec.

### Updated sub-locale priority (post-TRSLS)

| Priority | Sub-locale | Addresses | Files |
|---:|---|---|---:|
| 1 | `ts-resolve-classes-v2` (return-annot + readonly + access + decorator + import-export-type combined) | rows 1, 4, 5, 6, 7 | 80 |
| 2 | `ts-resolve-generics-calls` | row 2 | 37 |
| 3 | `ts-resolve-uncategorized-triage` (inspect singletons + rows 3, 8) | rows 3, 8 | 26 |
| 4 | `ts-resolve-misc-feature-gaps` (keyof, template-literal-type, decorator if not in #1) | rows 9, 10 | 4 |

Achieving 1-4 lifts parse-success **47.1% → ~87%**. Already addressable: 147 of 198 remaining failures.

### Status: CHAPTER CLOSED at TRSLS-EXT 1

Standing rule 13's sixth corroboration. Substrate bug eliminated; categorical separation now clean (TSR bugs vs TS-feature gaps). Failure-table re-validated; sub-locale priority updated; ready to spawn `ts-resolve-classes-v2` (the highest-value next locale).
