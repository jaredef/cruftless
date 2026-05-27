# tagged-template-object-construction — Trajectory

## TTOC-EXT 0 — founding (2026-05-27)

**Trigger**: LPA-EXT 9 lexer substrate gap analysis identified gap L.2 (strings.raw not threaded to runtime). The lexer produces raw data correctly; the compiler and runtime do not consume it. Highest-visibility compiler gap for ecosystem compatibility (String.raw is widely used).

**Status**: FOUNDED. TTOC-EXT 1 (AST raw-quasis threading) is the first substantive rung.

---

## TTOC-EXT 1 — Thread raw quasis through the full pipeline (2026-05-27)

**Trigger**: First substantive rung per the seed's methodology.

**Changes**:

1. **AST** (`pilots/rusty-js-ast/src/lib.rs`): Added `raw_quasis: Vec<Rc<String>>` field to `Expr::TemplateLiteral`. Carries the raw (backslash-preserved) source text alongside the cooked (escape-resolved) text.

2. **Parser** (`pilots/rusty-js-parser/derived/src/expr.rs`):
   - `parse_template_with_substitutions`: captures `raw` from each `TokenKind::Template` token into `raw_quasis` vector alongside `cooked` into `quasis`.
   - `parse_tagged_template`: builds a second `raw_arr` Array expression from `raw_quasis` and passes it as the second argument to `__template_object__(cooked_arr, raw_arr)`.
   - No-substitution tagged template also captures and passes raw.

3. **Runtime** (`pilots/rusty-js-runtime/derived/src/intrinsics.rs`):
   - `__template_object__` now accepts optional second argument (raw array). When present, uses it directly instead of copying cooked-as-raw. Falls back to cooked copy when absent (backward compat for any call site that doesn't pass raw yet).
   - Both cooked and raw arrays are frozen per §13.2.8.3.
   - `.raw` property installed as frozen on the template object.

**Diff-prod results**:

- `tagged-template-raw`: **FAIL → PASS**. `strings.raw` now populated, `String.raw` works, raw preserves backslash sequences, frozen template object, raw array is a frozen Array.
- `template-literals`: PASS (no regression).
- `string-escapes`: still FAIL (unrelated: emoji surrogate pair `.length` returns 1 instead of 2).

**Finding TTOC.1 (the lexer always had the data)**: confirming LPA-EXT 9 Finding LPA.21 — the lexer's `TokenKind::Template { cooked, raw, part }` has carried the raw field from the start. The entire gap was in the parser (discarded `raw` with `..` pattern) and runtime (copied cooked as raw). The fix is 3 files, ~30 net lines.

**Status**: TTOC-EXT 1 CLOSED. The tagged-template-raw fixture now passes. Template object call-site caching (§13.2.8.3 step 3: same template expression returns same object identity) is deferred to TTOC-EXT 2.
