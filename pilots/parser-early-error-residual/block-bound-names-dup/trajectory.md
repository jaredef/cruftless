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
