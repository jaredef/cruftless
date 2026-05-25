# strict-mode-parser-tracking — Seed

**Locale tag**: `L.strict-mode-parser-tracking` (top-level; EPSUA-EXT 3; constraint #2).

**Status**: **CLOSED at SMPT-EXT 1** (partial — only the function-body-depth axis; strict-mode + generator-context axes deferred).

**Workstream**: ECMA-262 §13.2 — `yield` is a Keyword only inside generator function bodies + strict-mode code. Outside both, `yield` is IdentifierName and can be used as a regular identifier. Cruft's parser unconditionally treated `yield` as YieldExpression in any context, breaking both:
- sloppy `var yield = 4; ... yield ...` (cluster #4 yield-ident-valid, 12 tests)
- strict `for ([x = yield] of [[]])` (cluster #3 yield-ident-invalid, 12 tests)

SMPT-EXT 1 closes the script/module top-level case (function_body_depth == 0 → `yield` is IdentifierReference). Function-body strict-mode and generator-context tracking deferred.

**Pre-scoping probe** (per EPSUA C4): 12 yield-ident-valid + 12 yield-ident-invalid + 14 yield-expr (gen-feature, separate) = 38 in-cluster, but SMPT-EXT 1 sub-cluster = top-level-yield-ident-valid only (~12).

**Composes with**:
- ECMA-262 §13.2 (Keyword classification); §15.5 (Generator Function definitions)
- [EPSUA](../ecmascript-parity-shared-upstream-arc/) — parent arc
- [PPA-EXT 1](../parser-permissiveness-audit/) — is_reserved_word in object-binding shorthand (mode-blind)

## I. Telos

`yield` at script/module top-level (function_body_depth == 0) parses as IdentifierReference. Inside any function body, current YieldExpression behavior preserved (still naive — generator-vs-non-generator + strict-vs-sloppy not yet disambiguated; deferred to SMPT-EXT 2+ candidate).

## II. Apparatus + Methodology

R = {Parser-state `function_body_depth`, increment at parse_function_body + arrow-expr-body, gated yield branch}. Single-tier from the parser POV; all in one commit.

Edits (~20 LOC):
1. `parser.rs::Parser`: add `function_body_depth: u32` field; init 0.
2. `stmt.rs::parse_function_body`: increment at entry, decrement at exit (saturating).
3. `expr.rs::parse_arrow_function`: bump+decrement around expression-body parse.
4. `expr.rs` yield-branch guard: only enter YieldExpression path if `self.function_body_depth > 0`. Otherwise fall through to the default Ident handler (yield as IdentifierReference).

## III. Carve-outs

- Generator-context vs non-generator-function tracking: NOT in this round. `yield` inside any function body is currently YieldExpression regardless of generator-or-not. Affects ~4 yield-ident-valid tests where the pattern is inside a function.
- Strict-mode tracking: NOT in this round. Per-script and per-function strict-mode propagation deferred. Affects ~12 yield-ident-invalid tests (onlyStrict; spec requires SyntaxError at parse).
- `static`, `implements`, etc. strict-reserved identifiers: not handled.

## IV. Verification

Minimal probe:
- `var yield = 4; for ([x = yield] of [[]]) console.log(x)` → `4` ✓ (was: `undefined`)
- `function* g() { yield 1; }; typeof g` → `"function"` ✓ (unchanged)

Exemplar (yield-ident-valid, 12 tests):
- PASS: 0 → 8 (+8; 67% of in-scope sub-cluster)
- Still-failing 4: yield inside function-body or nested-destructure contexts (require deeper tracking).

Regression (for-of/for-in/arrow-function, 838 previously-passing): 0 regressed.

## V. Status

CLOSED at SMPT-EXT 1.
