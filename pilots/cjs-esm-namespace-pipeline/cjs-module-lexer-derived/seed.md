# cjs-module-lexer-derived - Seed

**Locale tag**: `L.cjs-esm-namespace-pipeline.cjs-module-lexer-derived` (CMLD).

**Parent locale**: `cjs-esm-namespace-pipeline` (CENP). This is the nested Stage-L centerpiece per Doc 737 §II (a sub-workstream with multi-rung shape spawns a nested locale at a deeper coordinate).

**Status**: FOUNDED at CMLD-EXT 0 (design + fidelity probe). No runtime substrate lands in this founding round.

## I. Telos

Derive a static CommonJS export-name scanner — functionally equivalent to Node's `cjs-module-lexer` — under Pin-Art discipline, against Node's documented detection behavior and observed output. This is the substrate centerpiece of CENP Stage L: the named-export set for a CJS module imported as ESM must be determined *statically from source text* (before evaluation), matching Node, rather than by runtime enumeration of the evaluated `module.exports` (the current Bun-shaped behavior).

## II. Why this is its own locale, not a single rung

CENP Stage L cannot be done by suppressing runtime enumeration alone: packages like `debug` (9 names) and `semver` (40 names) are CJS whose names Node detects *statically* via `module.exports.X =` / object-literal members. Suppress-without-lexer would regress them to `default`-only. Faithful Node parity therefore *requires* a lexer, and the lexer has multiple independent detection rungs (assignment forms, reexports, defineProperty, edge cases) — multi-rung shape → nested locale.

## III. Baseline-inspect (Phase 2, CMLD-EXT 0)

Probe across installed top500 packages (source pattern vs Node's named-export view, default/module.exports excluded):

| pkg | kind | Node named exports | mechanism |
|---|---|---|---|
| chalk | ESM (`export`) | 12 | real ESM — routes through ESM finalize (CENP-EXT 2 site), NOT this lexer |
| uuid | ESM | 14 | real ESM — same |
| debug | CJS | 9 | `module.exports = fn; module.exports.X = …` — lexer detects |
| semver | CJS | 40 | object-literal / `exports.X =` members — lexer detects |
| lodash | CJS | 0 | dynamic `lodash.X = …` mixin — lexer finds nothing → default only |
| ms | CJS | 0 | `module.exports = function` — default only |

**Finding**: the CJS lexer's job is the `debug`/`semver` class (statically-detectable member assignments) and correctly emitting *nothing* for the `lodash`/`ms` dynamic class. The ESM `export`-syntax packages are out of scope (different Stage-L site).

## IV. Detection surface (the rungs to derive)

Node's cjs-module-lexer recognizes (target detection set):
1. `exports.NAME = …`
2. `module.exports.NAME = …`
3. `module.exports = { NAME: …, … }` (object-literal member names)
4. `Object.defineProperty(exports, "NAME", …)` / `Object.defineProperty(module.exports, …)`
5. Reexport stars: `module.exports = require("X")` and `Object.assign(module.exports, require("X"))` (re-export the lexed names of X)
6. `__esModule` flag (interop marker; passed through as a fixed key by Stage L, not a named export)
7. Common transpiler prologues (TS/Babel `Object.defineProperty(exports, "__esModule", {value:true})` followed by `exports.X = void 0` declarations).

## V. Apparatus

- Node baseline: `legacy/host-rquickjs/tools/parity-results-top500-node-2026-05-31.json`.
- Reference behavior: Node `import(pkg)` named-export view (the `parity-probe.mjs` shape probe under `node`).
- Fidelity oracle: the actual `cjs-module-lexer` npm package's documented detection rules (read for behavior, derived not vendored, per CENP carve-out).
- Consumption site (parent): `pilots/rusty-js-runtime/derived/src/module.rs::populate_cjs_namespace_view_at`.

## VI. Carve-Outs

- Derived, not vendored: the scanner is hand-derived against documented behavior + observed Node output under Pin-Art discipline; the npm `cjs-module-lexer` source is read for *behavior reference* only.
- ESM `export`-syntax packages (chalk/uuid) are out of scope — they route through the ESM finalize path (CENP-EXT 2).
- Deep live-binding correctness for ESM↔CJS cycles is out of scope here; this locale produces the static name *set* only.
- No runtime substrate lands in CMLD-EXT 0.

## VII. Resume Protocol

Read this seed, then `design.md` (detection-rung decomposition + fidelity-measurement plan), then `trajectory.md`. The Node baseline JSON + the parity-probe shape under `node` are the fidelity oracle. Compose against parent CENP `design.md` for the pipeline frame.
