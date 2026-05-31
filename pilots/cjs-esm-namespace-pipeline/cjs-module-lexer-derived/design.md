# CMLD Design — derived static CJS export-name scanner (CENP Stage-L centerpiece)

**Status**: design-only (CMLD-EXT 0). No runtime substrate authorized by this document.
**Parent**: `cjs-esm-namespace-pipeline` (CENP) — this is Stage L's name-set producer.
**Composes-with**: Doc 729 (resolver-instance pattern), Doc 581 (Pin-Art derive-from-constraints), the npm `cjs-module-lexer` (behavior reference, not vendored).

## 1. What it produces

A pure function: `cjs_lex(source: &str) -> CjsExportSet`, where

```
CjsExportSet {
  names:     Vec<String>,   // statically-detected named exports
  reexports: Vec<String>,   // require() specifiers whose names are re-exported
  esmodule:  bool,          // source set __esModule (interop marker)
}
```

It does NOT execute the module, does NOT resolve reexport specifiers (the parent threads resolution), and does NOT decide fixed keys (`default`/`module.exports`) — those are Stage L's job. It answers exactly: "what named exports would Node's lexer attribute to this source?"

## 2. Fidelity is the whole game

The parent design (CENP §5) names lexer fidelity as the dominant risk. Two failure directions, both parity-negative vs Node:
- **Under-detection** → cruft exposes fewer names than Node → regresses `debug`/`semver`-class packages.
- **Over-detection** → re-introduces Bun-style noise → the gap we're closing.

Therefore the locale's discipline is: **derive each detection rung against observed Node output, measure the named-export-set diff per package, and only land a rung when it strictly reduces the symmetric set-difference vs Node across the basket.**

## 3. Detection-rung decomposition (each a candidate substrate rung)

Ordered by observed top500 impact (to be confirmed by per-rung measurement):

1. **R1 — direct member assignment.** `exports.NAME =`, `module.exports.NAME =`. (debug-class.)
2. **R2 — object-literal export.** `module.exports = { NAME: …, … }` → member names. (semver-class.)
3. **R3 — defineProperty.** `Object.defineProperty(exports|module.exports, "NAME", …)`; recognize the `__esModule` special-case (sets `esmodule=true`, not a name).
4. **R4 — transpiler prologue.** TS/Babel `exports.NAME = void 0` declaration blocks + `_interopRequireDefault` patterns.
5. **R5 — reexport stars.** `module.exports = require("X")`, `Object.assign(module.exports, require("X"))`, `__exportStar`/`tslib.__exportStar` → emit reexport specifier for parent resolution.
6. **R6 — dynamic guard (negative rung).** Ensure dynamic assignment (`obj[computed] =`, loop-driven `lodash.X =` on the function value not on exports) yields NO false names. (lodash/ms-class → empty.)

## 4. Derivation method (Pin-Art)

For each rung Ri:
1. Identify the package cohort in the Node baseline whose names depend on Ri.
2. Read the cohort's source assignment patterns + the npm cjs-module-lexer behavior for that form.
3. Implement the minimal scanner addition.
4. Measure: per-package symmetric diff of `cjs_lex(source).names` vs Node's actual named-export set.
5. Land only if Ri reduces aggregate symmetric diff without increasing any package's diff (conservative-strip discipline, Rule 14 analogue for a classifier).

## 5. Implementation seam

The scanner is a leaf function with no Runtime access (pure string→struct), so it can live in the runtime crate's module submodule or a small dedicated module, and is unit-testable in isolation against fixture sources. The parent (CENP-EXT 1) calls it from `populate_cjs_namespace_view_at` with the source read from `url`, intersects/augments with the evaluated exports for value binding, and applies fixed keys.

Open seam question for CMLD-EXT 1: lex the raw source, or lex cruft's already-parsed AST for the module? AST-based detection is more robust than text-scanning (no comment/string false positives) and cruft already has the AST. Node uses a bespoke text lexer for speed; cruft may prefer AST-walk for fidelity. To be decided at CMLD-EXT 1 baseline-inspect.

## 6. Risks

- **Reexport resolution depth.** R5 requires the parent to resolve `require("X")` and recursively lex X. Cyclic reexports need a visited-set. Scope R5 carefully; it may warrant its own rung.
- **AST vs text.** If AST-based, must confirm cruft's AST is retained/recoverable at populate time; if text-based, must handle comments/strings/template literals to avoid false positives.
- **Lexer drift from Node.** Node's lexer evolves; we derive against a pinned Node version (v24.11, the baseline's reference) and record it.

## 7. Not claimed

This locale does not implement Stage L itself (that's CENP-EXT 1) and does not touch the ESM finalize path (CENP-EXT 2). It produces and validates the name-set function the parent consumes.
