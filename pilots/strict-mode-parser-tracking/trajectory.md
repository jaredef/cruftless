# strict-mode-parser-tracking — Trajectory

## SMPT-EXT 0+1 — founding + closure (2026-05-25)

**Trigger**: EPSUA-EXT 3; keeper "2" (constraint #2 — strict-mode parser tracking). Pre-scoping per EPSUA C4 narrowed scope from prospective ~80 to in-scope ~12 (top-level yield-ident-valid only); strict-mode + generator-context axes deferred.

**Edits** (~20 LOC) — see seed §II.

**Verification**:
- Minimal probe: `var yield = 4; for ([x = yield] of [[]]) print(x)` → 4 ✓
- Exemplar (yield-ident-valid, 12 tests): PASS 0 → 8 (+8, 67%)
- Regression: for-of (495), for-in (79), arrow-function (264) → unchanged (0)

### Findings

**Finding SMPT.1**: simple parser-state addition (function_body_depth counter, 20 LOC) closed 8/12 of the top-level yield-as-identifier sub-cluster cleanly. The remaining 4 require deeper tracking (generator-vs-non-generator OR strict-vs-sloppy within function-bodies) — deferred to SMPT-EXT 2+ candidates.

**Finding SMPT.2 (EPSUA arc-tier)**: fourth constraint sub-locale; cumulative EPSUA = 21 actual / ~163 projected = **13% of prospective amortization** (essentially flat). Pred-epsua.4 (≥2 within projection) appears falsified across constraints #4, #5, #2. The prospective doc's projections were uniformly inflated by the matrix-aggregation pattern.

**Status**: CLOSED at SMPT-EXT 1.

## SMPT-EXT 2 — strict_mode parser state + arrow-param strict reserveds (2026-05-25)

**Trigger**: keeper "Let's check out full strict mode tracking" after PPAE-EXT 4 split is_reserved_word into unconditional + broad, and the arrow-param check needed the broad set in strict contexts.

**Edits** (~50 LOC):
- `parser.rs::Parser`: add `strict_mode: bool` field (init false).
- `parser.rs::peek_use_strict_directive`: source-byte peek for `"use strict"` / `'use strict'` at current lookahead (no token consumption).
- `parser.rs::parse_module`: detect "use strict" directive prologue at entry; set strict_mode.
- `stmt.rs::parse_function_body`: detect "use strict" directive at body entry; save prior strict_mode; restore on body exit. Inner functions inherit parent strict.
- `expr.rs` yield branch: extended condition `(function_body_depth > 0 || strict_mode)` — strict-mode yield is unconditionally YieldExpression.
- `expr.rs::parse_arrow_function` reserved-word check: mode-gated — uses `is_reserved_word` (broad, incl. strict-only) + eval/arguments in strict; `is_unconditional_reserved_word` (Keyword only) in sloppy.

**Verification**:
- `"use strict"; var af = arguments => 1;` → SyntaxError ✓
- `"use strict"; var af = (yield) => 1;` → SyntaxError ✓
- `var af = (yield) => 1; af(1)` → 1 (sloppy valid) ✓
- `var af = arguments => arguments` → works (sloppy) ✓
- Random 300 language adjacent: 300/300, 0 regressions

**Exemplar** (24 yield-ident-invalid + bindingidentifier-no-* + identifier-strict-futurereservedword fixtures):
- PASS: 0 → **3** (the arrow-param-strict-arguments/eval/yield cases)
- 21 remaining use `yield` inside function-body or top-level-strict where compile-time YieldExpression-in-non-generator-strict throw is needed — requires generator-context tracking (SMPT-EXT 3 candidate).

### Findings

**Finding SMPT.3**: full strict-mode tracking is a structural unlock. Mode-gated predicates (is_reserved_word vs is_unconditional_reserved_word; eval/arguments as ident) now have a source-of-truth (`self.strict_mode`); per-site predicate selection becomes mechanical. SMPT-EXT 3 candidate: generator-context tracking — when active, yield in function body is YieldExpression; when inactive + strict, yield is reserved-word SyntaxError; when inactive + sloppy, yield is IdentifierReference.

**Finding SMPT.4 (sub-cluster decomposition)**: yield-ident-invalid (~12 onlyStrict tests in cluster) split into:
- arrow-param-strict-yield (3 of 12): closed by SMPT-EXT 2
- function-body-yield-as-ident-strict (~5): needs SMPT-EXT 3 (generator tracking + YieldExpression-not-in-generator-throws)
- for-of head-yield-init-strict (~4): same as above

**Status**: SMPT-EXT 2 CLOSED.

## SMPT-EXT 3 — generator-context tracking (2026-05-25)

**Trigger**: keeper "let's do smpt ext 3" after Addendum XIII Finding SMPT.4 predicted ~9 of remaining 21 yield-ident-invalid tests close on the generator-context axis.

**Edits** (~45 LOC):
- `parser.rs::Parser`: add `in_generator: bool` field (init false). Carries the §15.5 YieldExpression-valid-only-inside-generator predicate.
- `stmt.rs::parse_function_body`: split into thin wrapper + `parse_function_body_g(is_generator: Option<bool>)`. `Some(g)` introduces a generator boundary (save/set/restore `in_generator`); `None` preserves enclosing (used by static-block, which is not a generator boundary).
- Eight function-defining call sites updated to pass `Some(is_generator)`:
  parser.rs (default export), stmt.rs (FunctionDecl + class method), expr.rs (generator-method shorthand, async-method shorthand, getter/setter, plain method, FunctionExpression).
- `expr.rs` arrow body: pass `Some(false)` for arrow block body and save/set/restore `in_generator=false` around arrow expression body. Per §15.3, ConciseBody is not [Yield]-parameterized — arrows never inherit generator-context.
- `expr.rs` yield-branch: condition replaced with `(self.in_generator || self.strict_mode)`. Strict-mode + !in_generator now `return Err(ParseError)` with the §13.2 reserved-word message.

**Replaces** SMPT-EXT 1's `function_body_depth > 0` heuristic with the spec-correct `in_generator` predicate. SMPT-EXT 1's depth-bump remains for any future call site that needs body-vs-top-level discrimination (currently unused by the yield-branch).

**Verification**:
- `"use strict"; function f() { yield 1; }` → SyntaxError ✓
- `function f() { var yield = 4; console.log(yield); }` → 4 ✓ (sloppy non-generator: yield is identifier; previously WRONG with EXT 1's depth heuristic, now correct)
- `var yield = 4; for ([x = yield] of [[]]) console.log(x)` → 4 ✓ (SMPT-EXT 1 regression check)
- Random 300 prev-PASS: **300/300, 0 regressions**
- diff-prod: **42/42 PASS**

**Exemplar** (19 prev-failing yield-ident-invalid + identifier-strict-futurereservedword + bindingidentifier-no fixtures):
- PASS: 0 → **14**

### Findings

**Finding SMPT.5**: SMPT.4's axis-decomposition prediction held — 14/19 closure on the generator-axis residual matches "≈9 expected" within order-of-magnitude (under-projected by 1.5×; the axis-share estimate was conservative). Standing Rule 22 instantiation confirms.

**Finding SMPT.6 (predicate-correctness over heuristic)**: replacing `function_body_depth > 0` with `in_generator` not only fixed the strict-mode case but corrected the sloppy non-generator function-body yield-as-identifier case (SMPT-EXT 1 was an over-rejecting heuristic). The principled predicate beat the proxy predicate on both axes.

**Status**: SMPT-EXT 3 CLOSED.

## SMPT-EXT 4 — strict/generator yield residual closure (2026-05-29)

**Trigger**: Helmsman EPSUA parallel-R4 adjudication approved the read-only residual segmentation for four latest full-suite parser-owned `yield` rows. Scope remained strictly SMPT: strict-mode + generator-context predicate residuals, no non-strict PPAE for-head changes.

**Edits** (~40 LOC):
- `expr.rs::parse_object_literal`: reject bare shorthand `{ yield }` when `strict_mode || in_generator`, before the shorthand expression is emitted. This covers both object-literal shorthand and the strict assignment-pattern shorthand conversion path.
- `stmt.rs::parse_class_body`: parse static blocks as strict code under `[~Yield]` by temporarily setting `strict_mode=true` and `in_generator=false`, preventing generator-function context from leaking into `static { ... }`.
- `stmt.rs::parse_variable_statement`: after a declarator without initializer, reject a same-line non-separator token. This catches the `let\n yield 0;` lexical-declaration residual where `yield 0` must not be accepted as a following expression on the same declaration line.

**Verification**:
- Target test262 exemplars: **4/4 PASS** (all expected SyntaxError):
  - `language/expressions/assignment/dstr/obj-id-identifier-yield-ident-invalid.js`
  - `language/expressions/object/identifier-shorthand-yield-invalid-strict-mode.js`
  - `language/statements/class/static-init-invalid-yield.js`
  - `language/statements/let/syntax/let-newline-yield-in-normal-function.js`
- Protective probes:
  - `"use strict"; function f() { yield 1; }` -> SyntaxError
  - `function f() { var yield = 4; console.log(yield); } f();` -> 4
  - `var yield = 4; for ([x = yield] of [[]]) console.log(x);` -> 4
- `cargo build --release --bin cruft -p cruftless` -> PASS.
- `cargo test --release -p rusty-js-parser` -> blocked by pre-existing unrelated lexer unit `tests/spec_golden.rs::legacy_octal_rejected` accepting `07` as a number token; parser tests reached before that failure passed.

### Findings

**Finding SMPT.7 (residual segmentation correctness)**: the four latest parser-owned yield residuals were not a single missing "yield reserved" switch. They split across three concrete grammar coordinates: shorthand IdentifierReference, ClassStaticBlockStatementList context, and lexical declaration continuation. Applying the exact predicate at each coordinate closed all four target rows without regressing sloppy yield-as-identifier probes.

**Finding SMPT.8 (static-block context boundary)**: static blocks are a strict non-generator body even when syntactically nested inside a generator function. Treating them as `parse_function_body_gs(Some(false), None, true)` is the correct parser-state boundary; preserving outer `in_generator` was the failure shape.

**Status**: SMPT-EXT 4 CLOSED.
