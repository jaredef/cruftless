---
name: annexB-runtime-quirks
description: Annex B legacy runtime surfaces: Date legacy methods, String HTML methods, RegExp.compile/legacy accessors, and global escape/unescape.
type: project
---

# annexB-runtime-quirks — Seed

## Substrate-pilot — Tier E, coordinate-driven Annex B runtime surface (ABRQ).

Spawned after HDSB was found to already be owned by another active agent. This locale selects the still-open `annexB-runtime-quirks` candidate from `apparatus/locales/CANDIDATES.md` Tier E. Collision discipline: this locale explicitly excludes Annex B grammar/lowering work already represented by HDSB/WBMS/DIA/regexp-conformance and targets runtime built-in surfaces only.

## Telos

Implement the Annex B legacy runtime surfaces that Test262 treats as normative optional web-compat behavior and that cruft currently exposes as missing binding, missing method, wrong result, or missing throw behavior.

First-cut scope:

- `Date.prototype.getYear`
- `Date.prototype.setYear`
- `Date.prototype.toGMTString`
- String HTML methods: `anchor`, `big`, `blink`, `bold`, `fixed`, `fontcolor`, `fontsize`, `italics`, `link`, `small`, `strike`, `sub`, `sup`
- global `escape`
- global `unescape`
- `RegExp.prototype.compile`

Second-cut / sibling scope:

- RegExp constructor Annex B decimal/identity escape grammar and regexp-engine semantics belong under `regexp-conformance/` unless RC baseline-inspection delegates them here.
- RegExp legacy constructor accessors (`RegExp.input`, `$1`, `lastMatch`, etc.) are runtime, but cross-realm and constructor-brand tests make them a separate rung after the simpler surfaces above.
- Annex B language grammar and binding semantics remain outside this locale.

Coordinate:

```
annexB/built-ins :: E2/internal-method:runtime ::
  cut/legacy-runtime-surface ::
  property/legacy-web-compat-builtins-installed-and-spec-shaped
```

## Apparatus

- `pilots/rusty-js-runtime/derived/src/intrinsics.rs` — global and prototype registration sites.
- `pilots/rusty-js-runtime/derived/src/date.rs` or adjacent Date helpers — Date legacy method behavior.
- `pilots/rusty-js-runtime/derived/src/string.rs` or adjacent String helpers — HTML string method behavior.
- `pilots/rusty-js-runtime/derived/src/regexp.rs` — `RegExp.prototype.compile` if not delegated to `regexp-conformance`.
- **Exemplar suite**: `pilots/annexB-runtime-quirks/exemplars/exemplars.txt` — stratified Annex B built-ins probes.

## Baseline (FOUNDING)

The 2026-05-25 full-suite matrix exposed Annex B built-ins across several high-rank cells:

- missing global/binding: global `escape` / `unescape`
- wrong result: Date legacy methods
- missing method/intrinsic: String HTML methods and RegExp legacy surfaces
- throw missing / wrong throw type: Date brand checks, RegExp compile SyntaxError paths, escape/unescape conversion errors

Candidate queue estimate: ~398 Annex B resolver-routed fails plus ~202 String.prototype HTML-method fails. ABRQ-EXT 0 should refine this into a locale-local pass/fail baseline before any substrate edits.

## Methodology

### ABRQ-EXT 0 — baseline inspection

Run the stratified exemplar suite and partition failures by surface:

1. Date legacy methods
2. String HTML methods
3. global escape/unescape
4. RegExp.prototype.compile
5. RegExp legacy constructor accessors
6. delegated-to-regexp-conformance grammar/engine residuals

Output: trajectory update with exact counts and first-rung selection. Do not implement before this partition is recorded.

### ABRQ-EXT 1 — smallest additive surface

Select the surface with the smallest blast radius and strongest missing-surface signal. Expected candidate: String HTML methods or global escape/unescape.

Gate:

- exemplar subset for selected surface improves materially,
- diff-prod 42/42 maintained,
- no regression in `string-literal-and-escape-conformance`, `regexp-conformance`, or Date tests outside Annex B.

### ABRQ-EXT 2+ — per-surface rungs

Continue surface-by-surface. Each rung should install one coherent legacy surface, not a mixed Annex B grab bag.

## R13 prospective C1-C4

- C1 (sibling): HOLDS — standard intrinsic-registration machinery already exists for globals and prototype methods.
- C2 (shape-compat): HOLDS for String HTML and escape/unescape; PROBED for Date and RegExp.compile because they rely on existing internal slots / regexp engine state.
- C3 (cost-positive): HOLDS if each rung is per-surface and additive.
- C4 (bail-safe): HOLDS only if Annex B language/parser cases remain excluded; mixing grammar and runtime would violate the locale boundary.

## Carve-outs

- No Annex B parser grammar work.
- No HDSB binding semantics.
- No `with` runtime semantics.
- No broad regexp grammar implementation unless delegated by `regexp-conformance`.
- No Intl/Temporal-style implement-chapter work.

## Composes-with

- `apparatus/locales/CANDIDATES.md` Tier E entry `(x)`.
- `pilots/regexp-conformance/` — RegExp grammar and engine sibling.
- `pilots/hoistable-declaration-as-statement-body/` — Annex B grammar sibling owned elsewhere.
- `pilots/string-literal-and-escape-conformance/` — string escape/tokenization sibling, regression guard.
- `apparatus/docs/ecma-conformance-parity-as-exhaustive-language-behavior-dag.md` — Annex B as a named contingent language-behavior surface.

## Status

FOUNDED. ABRQ-EXT 0 baseline inspection is next. No substrate edits have landed in this locale.
