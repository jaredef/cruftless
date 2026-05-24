# ts-resolve-class-and-param-shapes — Trajectory

## TRCAPS-EXT 0 — workstream founding (2026-05-24)

**Trigger**: keeper directive ("No v2 it's all green field") at TRSLS-EXT 1 chapter close. Naming corrected from `ts-resolve-classes-v2` to drop the misleading v2 suffix.

**Per Finding TRSLS.2 discipline**: inspected representative failures before locking down the locale name + scope. Found that the TCC heuristic tags (`method-return-annotation`, `access-modifier`) were correlative not causal — the actual failures cluster around three concrete strip-rule gaps:
1. Destructured-pattern parameter annotations (`{a,b}: T` in param position)
2. Generics on `extends` clause + the `implements` keyword
3. TS-only class-member modifiers (public/private/protected/readonly/abstract/override)

**Founding artefacts**: seed.md + trajectory.md + scaffolded dirs. TRCAPS-EXT 1 (implementation + re-measure + chapter close) next.

---

## TRCAPS-EXT 1 — six substrate improvements + chapter close (2026-05-24)

**Round shape**: started with three planned rules from the seed; investigation surfaced three additional substrate gaps mid-round that became part of the same close. The "inspect before commit" discipline from Finding TRSLS.2 paid off — each new gap was discovered by examining the post-fix failure-table, leading to the next iteration.

**Six landed improvements** (all in `pilots/ts-resolve/derived/src/strip.rs`):

1. **`extends NAME<T,...>` generic stripping** — symmetric to existing `class NAME<T>` rule via `match_angle` helper.
2. **`implements TYPE[, TYPE]* {` clause stripping** — scan forward to next top-level `{` (class body opener).
3. **TS-only class-member modifier stripping** — `public`, `private`, `protected`, `readonly`, `abstract`, `override` when followed by Ident.
4. **Destructured-pattern parameter annotation** — `is_annotation_colon` accepts `RBrace`/`RBracket` anchors when `skip_type` lands on `,`/`)`/`=`.
5. **Ternary `:` vs return-type `:` disambig** — `)`-anchored `:` accepted as annotation only when `skip_type` lands on a function-body opener (`{`, `=>`, `;`, `,`, `=`). Bails on `}` (signal of ternary in return position).
6. **Regex-literal goal selection** — new `expr_or_div_goal` helper consults previous token; returns `LexerGoal::RegExp` at expression-start positions (after `=`, `(`, `,`, keywords like `return`/`typeof`/etc.) and `LexerGoal::Div` after expression-terminator tokens. Fixes regex `/pat/` lexing in real source.
7. **Object-literal-method-shorthand disambig** — refined the obj-lit early-bail in `is_annotation_colon`: only bails when prev-anchor is Ident AND `paren_depth == 0`. A `:` inside `(...)` enclosed by an obj-lit is a method-param annotation, not a key:value separator. Required adding `paren_depth` scalar to Scanner state.

**Discovered mid-round**:
- The 5th and 6th items were not in the seed. The seed had three rules; investigation of post-fix failures (rxjs/Subject.ts → extends/implements; ajv/resolve.ts → regex `/#\/?$/`; ajv/vocabularies/code.ts → ternary `: cond` mis-stripped; ajv/vocabularies/core/ref.ts → method-shorthand return-type) drove the iteration. Each look was ~5 minutes; each fix was ~5-20 LOC.

**LOC delta**: ~140 across strip.rs (slightly over Pred-trcaps.1's ≤120 budget — accepted because the six fixes deliver +12.8 pp on a +10 pp prediction).

**Gates**:
- `cargo build --release -p ts-resolve`: ✅ clean
- `cargo test --release -p ts-resolve`: ✅ **35/35 PASS** (+10 new tests: 7 class+param shape + 3 regex/division regression)
- `cargo build --release --bin cruft -p cruftless`: ✅ clean
- diff-prod 42/42 PASS ✅ (zero regression on .js paths)

**TCC measurement** (Pred-trcaps.3 + Pred-trcaps.4 booking):

| Stage | OK count | Parse-success | Δ |
|---|---:|---:|---:|
| Pre-TRCAPS baseline (post-TRSLS) | 176 | 47.1% | — |
| After items 1-3 (seed scope) | 190 | 50.8% | +3.7 pp |
| After item 6 (regex goal) | 194 | 51.9% | +1.1 pp |
| After item 5 (ternary disambig) | 198 | 52.9% | +1.0 pp |
| After item 7 (method-shorthand) | **224** | **59.9%** | **+7.0 pp** |
| **Cumulative TRCAPS-EXT 1** | **+48 files** | **+12.8 pp** | over ≥10 pp target |

**Pred-trcaps.4 (no regression)**: HELD — diff-prod 42/42; the 176 pre-TRCAPS-OK files all remain OK (parse-success monotonically increased).

### Final disposition

| Predicate | Disposition |
|---|---|
| Pred-trcaps.1 (≤120 LOC) | ⚠️ PARTIAL at ~140 LOC (6 fixes vs 3 planned; accepted for +12.8 pp) |
| Pred-trcaps.2 (35/35 tests + 7 new regressions) | ✅ HELD at 35/35 |
| Pred-trcaps.3 (≥10 pp parse-success lift) | ✅ HELD STRONGLY at +12.8 pp (28% over target) |
| Pred-trcaps.4 (no regression of 176 OK files) | ✅ HELD |
| Pred-trcaps.5 (≤3 implementation rounds) | ✅ **HELD at 1 implementation round** |

### Findings

**Finding TRCAPS.1** (substrate-bug compound discovery): the seed planned three TS-feature strip rules; investigation surfaced three additional substrate bugs (regex goal, ternary disambig, method-shorthand) of greater impact than the planned rules. The 7-pp lift from item 7 alone (paren-depth gating on object-literal bail) exceeded the seed's three-rule combined impact. **Discipline implication**: when investigating post-fix failures, look for substrate bugs in TSR's existing rules before assuming the cause is a missing TS feature. Each inspection costs ~5 min; the compound payoff is high.

**Finding TRCAPS.2** (standing rule 13 seventh corroboration): TRCAPS closed in 1 implementation round under Pred-trcaps.5's ≤3 budget. Seven concrete substrate changes shipped together as one focused implementation round. Discipline scales to "multiple-fix" implementation rounds, not just single-fix.

**Finding TRCAPS.3** (Pin-Art derived-from-constraints vindicated at sub-locale tier): the constraint here was TCC's failure-table + the inspected example files. Each fix was derived from an empirical failure, not a spec checklist. The corpus drove the priority + the scope. TS spec was consulted only to verify the strip rule's correctness, not to prioritize.

### Updated post-TRCAPS sub-locale priority

| Priority | Sub-locale | Addresses | Files |
|---:|---|---|---:|
| 1 | `ts-resolve-generics-calls` (generic-call: `f<T>()` disambig) | row 2 | 40 |
| 2 | TRSLS-EXT 2 follow-on (method-return-annotation 41 still dominant — likely yet another substrate bug; inspect first) | row 1 | 41 |
| 3 | `ts-resolve-readonly-class-field` | row 3 | 9 |
| 4 | `ts-resolve-template-literal-types` (real ones now; small count) | row 4 | 7 |
| 5 | `ts-resolve-import-export-type` | row 6 | 5 |

Achieving rows 1-5 lifts parse-success **59.9% → ~87%**.

### Status: CHAPTER CLOSED at TRCAPS-EXT 1

Standing rule 13's seventh corroboration. Substrate clean; six concrete improvements landed; parse-success up +12.8 pp (28% over Pred-trcaps.3 target). The "inspect then iterate" discipline from Finding TRSLS.2 compounded: each post-fix failure surface led to the next substrate improvement within the same round.
