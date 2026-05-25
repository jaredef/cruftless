# parser-permissiveness-audit-extensions — Seed

**Locale tag**: `L.parser-permissiveness-audit-extensions` (top-level; EPSUA-EXT 2 sub-locale; constraint #5).

**Status**: **CLOSED at PPAE-EXT 1**.

**Workstream**: extend PPA-EXT 1's parser-permissiveness audit with three targeted sub-sites surfaced by post-arc T262C matrix: (a) escaped contextual keyword `of` in for-of head, (b) duplicate parameter names in arrow function, (c) lexical-binding-in-head conflicting with var-declared-in-body in for-in/for-of. Defer for-in destructure-head (separate feature implementation).

**Trigger**: EPSUA-EXT 2 per re-ordered queue after EPSUA-EXT 0.5 ($262 deferral) + EPSUA-EXT 1 (ICOA closure).

**Pre-scoping probe** (per Finding T262C.6 + EPSUA C4 strengthening): sub-cluster sizes
- escaped-of: 1
- params-duplicate: 2
- head-(const|let)-bound: 14
- for-in destr-head: 7 (deferred)

Total in-scope: 17 tests vs ~50 prospective-doc projection (per-reason segmentation correctly narrowed scope).

**Composes with**:
- [PPA-EXT 1](../parser-permissiveness-audit/) — predecessor
- [SDIBP](../statement-declaration-in-body-position/) — sibling parser-tier fix
- ECMA-262 §11.6.2 (contextual keyword unescaped); §15.2.1 (arrow params Early Errors); §14.7.1.2 (for-in/of Early Errors)
- [EPSUA](../ecmascript-parity-shared-upstream-arc/) — parent arc

## I. Telos

Three targeted parser/compile-tier rejections matching ECMA-262 Early Errors. Each is a small per-site addition; rule 14 mirror (adding restriction = false-positive risk) handled by spec-precise check at each site.

## II. Apparatus + Methodology

R = {is_contextual_keyword helper, of-in-for-head replacements, arrow-dup-params check, for-in/of head-vs-body name-conflict check at compile}. All landed combined.

Edits (~70 LOC):
1. `parser.rs`: new `is_contextual_keyword(name)` — requires Ident match AND source span text == name exactly (unescaped).
2. `stmt.rs`: replace `is_ident("of")` → `is_contextual_keyword("of")` at the 4 for-head sites.
3. `expr.rs::parse_arrow_function`: enumerate simple-ident params; reject on duplicate.
4. `compiler.rs::Stmt::ForOf` + `Stmt::ForIn`: §14.7.1.2 Early Error — head's let/const BoundNames must not intersect body's VarDeclaredNames (uses `collect_hoisted_var_names` from VHTB).

## III. Carve-outs

- `as`, `from`, `async`, `let`, `static`, etc. contextual keywords NOT gated (other rounds; only `of` was empirically failing).
- Non-ident destructure params in arrow function — duplicate check covers only `Identifier`; destructure-pattern duplicates (`{a, a}`, `[a, a]`) deferred.
- TDZ + let-in-body + duplicate-decl-in-head variants of head-bound-names: different Early Errors; not in PPAE-EXT 1 scope (deferred to PPAE-EXT 2+ candidate).
- for-in destructure-head feature: not a check addition; substantial compiler work; deferred.

## IV. Verification

Minimal probes (GREEN):
- `for (var x of []) ;` → SyntaxError ✓
- `(a, a) => {}` → SyntaxError ✓
- `for (const x in {}) { var x; }` → SyntaxError ✓

Exemplar (PPAE-targeted, 17 tests):
- PASS: 0 → 7 (+7; escaped-of + params-duplicate + 5 head-bound-names-in-stmt variants)
- Still-failing 10: TDZ / let-in-body / dup-decl variants of head-bound-names — different Early Errors.

Regression check:
- for-of: 495 → 495 (0)
- for-in: 79 → 79 (0)
- arrow-function: 264 → 264 (0)

## V. Status

CLOSED at PPAE-EXT 1.
