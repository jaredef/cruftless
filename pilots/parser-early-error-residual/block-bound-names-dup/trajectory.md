# block-bound-names-dup — Trajectory

## BBND-EXT 1 — block-scope LexicallyDeclaredNames duplicate + LDN∩VDN check (2026-05-25)

**Trigger**: PEER baseline run surfaced 3/3 inspected fails as `language/block-scope/syntax/redeclaration/*` (Finding PEER.1). Heuristics §V row-coherence satisfied (shared mechanism: §13.2.1 LDN duplicate / LDN-VDN-overlap). Per keeper directive "spawn locale" + Doc 737 §II promotion (multi-rung-shape sub-workstream warrants nested locale).

**Edits** (~60 LOC in `stmt.rs`):

1. `parse_block_statement` — call `self.check_block_bound_names(&body)?` after the body parse, before constructing `Stmt::Block`.
2. New `Parser::check_block_bound_names` helper — walks top-level body statements, buckets each into LDN or VDN per §13.2.6 + Annex B B.3.2, then enforces dup-in-LDN and LDN-intersect-VDN. Splits AsyncFunction / Generator / AsyncGenerator from plain FunctionDeclaration so the Annex B carve-out applies only to the latter in non-strict.

**Verification**:

| Probe | Result |
|---|---|
| `{ let f; function* f() {} }` | SyntaxError ✓ |
| `{ class f {} async function* f() {} }` | SyntaxError ✓ |
| `{ async function* f() {} async function f() {} }` | SyntaxError ✓ (B.3.2 narrow: carve-out is plain-function-only) |
| `{ let f; class f {} }` | SyntaxError ✓ |
| `{ const x = 1; let x = 2; }` | SyntaxError ✓ |
| `{ var x; let x; }` (LDN∩VDN) | SyntaxError ✓ |
| `{ let x; var x; }` (same, reversed) | SyntaxError ✓ |
| `{ let a; let b; }` | parses ✓ |
| `{ var x; var x; }` | parses ✓ |
| `{ function f() {} function f() {} }` (sloppy, B.3.2) | parses ✓ |
| `{ let x; { let x; } }` (nested shadowing) | parses ✓ |
| `{ var x; function x() {} }` (sloppy, both VDN) | parses ✓ |

**Yield**:

| Surface | Before | After |
|---|---|---|
| PEER 100-exemplar suite | 0/100 | **4/100** (+4) |
| `block-scope/syntax/redeclaration/` full dir (95 tests) | (untested before) | **76/95 (80.0%)** |

**Gates**:
- diff-prod: **42/42 PASS, 0 FAIL**
- Random 300 prev-PASS: **300/300, 0 regressions**

**Findings**

**Finding BBND.1 (Annex B B.3.2 carve-out is grammar-specific, not semantically-broad)**: the spec text "duplicate entries are exclusively due to FunctionDeclarations" refers to the GRAMMAR production `FunctionDeclaration` (the plain `function` form), NOT to all hoistable function-like declarations. AsyncFunctionDeclaration / GeneratorDeclaration / AsyncGeneratorDeclaration are sibling productions under HoistableDeclaration; they do NOT get the carve-out. The dup-detection helper must discriminate by `(is_async, is_generator)`, not by "is it a FunctionDecl". Standing recommendation: when an Annex B carve-out names a specific production, take the grammar production literally; sibling productions in the same syntactic class do not inherit the carve-out.

**Finding BBND.2 (one-rung 76/95 yield via single shared §13.2.1 helper)**: the entire 95-test redeclaration directory is gated by ONE early-error rule applied at ONE production site. Single-rung yield: 76 tests. The 19 remaining fails are likely the strict-mode-specific or var-var-with-Annex-B-edge shapes; small follow-on. Standing recommendation: when a test262 sub-directory shares its name with a single spec production (here `redeclaration`), the yield-per-LOC ratio is typically extreme; check the spec rule first, the test bodies second.

**Status**: BBND-EXT 1 CLOSED. PEER 100-exemplar +4 (4/100); cluster `block-scope/syntax/redeclaration/` 76/95.

---

## BBND-EXT 2 — nested-block var hoisting + Annex B dual-role function-decl (2026-05-25)

**Trigger**: 19 residual fails in `block-scope/syntax/redeclaration/` after BBND-EXT 1. Inspection partitioned them into two sub-shapes:

- **Nested-block var hoisting** (~10 tests): `{ { var f; } let f; }` and inverses. The inner block's `var` hoists to the enclosing block-level VDN; the enclosing block's check must see it.
- **Plain function ↔ var in non-strict** (~9 tests): `{ var f; function f() {} }`. Per Annex B B.3.2 + B.3.3, a plain function-decl in non-strict contributes to BOTH LDN and VDN with the same decl-identity. A var with the same name is a DIFFERENT decl → LDN∩VDN crosses on distinct decl-ids → error. The Annex B carve-out is *only* about suppressing the dup-LDN check when entries are exclusively plain functions; it does NOT eliminate the function from LDN.

**Edits** (~80 LOC refactor of `check_block_bound_names`):

1. Switched from flat lex/var lists to per-entry `(name, span, decl_id, is_lex, is_var, plain_func_nonstrict)` records; each declaration gets a unique `decl_id`.
2. Plain function in non-strict now emits a single record with both `is_lex=true` and `is_var=true` (Annex B B.3.2/B.3.3 dual role).
3. New `collect_block_entries(body, nested=bool, ...)` walks Stmt::Block / If / For / ForIn / ForOf / While / DoWhile / Switch / Try / Labelled recursively at `nested=true`. At nested depth, only var-side contributions hoist (vars hoist; lex declarations and pure-LDN functions don't).
4. Dup-LDN check: distinct lex decl_ids ≥ 2, suppressed iff all are plain-function-non-strict.
5. LDN∩VDN check: exists (li, vi) with different decl_ids, where NOT both are plain-function-non-strict.

**Verification**:

| Probe | Result |
|---|---|
| `{ { var f; } let f; }` | SyntaxError ✓ |
| `{ let f; { var f; } }` | SyntaxError ✓ |
| `{ function f() {} var f; }` | SyntaxError ✓ |
| `{ var f; function f() {} }` | SyntaxError ✓ |
| `{ function f() {} function f() {} }` (sloppy B.3.2) | parses ✓ |
| `{ { var x; } var x; }` (nested var-var) | parses ✓ |
| `{ let x; { let x; } }` (lex shadowing) | parses ✓ |

**Yield**:

| Surface | Before | After |
|---|---|---|
| `block-scope/syntax/redeclaration/` full dir (95 tests) | 76/95 | **95/95 (100%)** |
| PEER 100-exemplar suite | 4/100 | 4/100 (same — slice only had 4 redeclaration samples; both rungs closed all 4) |

**Gates**:
- diff-prod: **42/42 PASS, 0 FAIL**
- Random 300 prev-PASS: **300/300, 0 regressions**

**Findings**

**Finding BBND.3 (Annex B is dual-role, not demote-only)**: B.3.2/B.3.3 for plain function in non-strict block does NOT remove the function from LexicallyDeclaredNames — it adds the function to VarDeclaredNames as well. The B.3.2 carve-out then suppresses *only* the dup-LDN early-error when all duplicate entries are exclusively plain functions; it does not affect the LDN∩VDN intersection rule, because the function's LDN entry and its own VDN entry share a decl-id and so do not cross. A separate var-decl IS a different decl-id, so its VDN entry crosses with the function's LDN entry → error. Standing recommendation: when modeling spec carve-outs that say "X acts as both Y and Z", track decl-identity through both contributions; the carve-out reads against decl-identity, not against the name.

**Finding BBND.4 (var-hoisting demands recursive collection)**: §13.2.1's VDN includes vars from contained Statements (Block/If/For/while/try/labelled). Naive single-pass body-walk misses inner-block vars and under-reports the conflict. Recursive collection at nested depth must allow var harvesting while pruning lex declarations (block-scoped, do not hoist) and pruning non-plain-non-strict functions. Standing recommendation: any VarDeclaredNames or VarScopedDeclarations algorithm involving block scope MUST traverse into the contained Statement productions; the spec's algorithm is structurally recursive, the implementation must follow.

**Status**: BBND-EXT 2 CLOSED. `block-scope/syntax/redeclaration/` cluster fully closed (95/95). Two-rung total for this nested locale.
