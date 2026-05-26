# annexB-runtime-quirks — Trajectory

## ABRQ-EXT 0 — FOUNDING (2026-05-26)

Spawned to avoid collision with the already-active `hoistable-declaration-as-statement-body` locale. Selected from `apparatus/locales/CANDIDATES.md` Tier E entry `(x)` because it is still open, matrix-anchored, and runtime/built-ins scoped rather than parser-tier.

### Scope boundary

Included:

- Date legacy methods
- String HTML methods
- global escape/unescape
- RegExp.prototype.compile
- RegExp legacy constructor accessors as later rung candidates

Excluded:

- Annex B language grammar
- HDSB binding semantics
- `with` runtime semantics
- regexp grammar/engine surfaces unless delegated by `regexp-conformance`

### Founding apparatus

- `seed.md` created.
- `exemplars/exemplars.txt` created as a stratified first-pass suite.
- No runtime substrate edits landed.

### Next move

ABRQ-EXT 0 baseline inspection:

1. Run the exemplar suite.
2. Partition by surface.
3. Record exact pass/fail counts and dominant reason shapes.
4. Select ABRQ-EXT 1 by smallest additive surface with highest coherent yield.

### Baseline inspection

Corrected the first-pass exemplar list for current Test262 filenames
(`escape/argument_types.js`, `RegExp.prototype.compile/pattern-string.js`,
etc.; several names in the initial hand-picked list were stale relative to
this checkout).

Command:

```
pilots/annexB-runtime-quirks/exemplars/run-exemplars.sh
```

Result:

```
ABRQ exemplars: PASS=5 FAIL=36 / 41  (12.2%)
```

Failing surface partition:

| Surface | Fails | Read |
|---|---:|---|
| String HTML methods | 14 | pure missing-method surface on `String.prototype` |
| Date legacy methods | 8 | partial existing methods, wrong Date math / brand / alias behavior |
| escape/unescape globals | 6 | pure missing-global surface plus conversion tests |
| RegExp.prototype.compile | 3 | existing or partial method with missing error/compile semantics |
| RegExp legacy accessors | 2 | missing descriptor/accessor surface |
| Other | 3 | runner/path or mixed residual from the stratified set |

### First-rung selection

ABRQ-EXT 1 should target **String HTML methods**. Reason:

- largest coherent exemplar slice (14 fails),
- pure missing-method shape,
- additive registration on `String.prototype`,
- low collision with Date internals and RegExp-conformance,
- spec behavior is mechanical: coerce receiver to string, stringify argument where present, return an HTML wrapper string.

`escape` / `unescape` is also additive and likely small, but has fewer exemplars in this stratified set and more conversion/error-order details. It is the natural ABRQ-EXT 2 candidate if EXT 1 lands cleanly.

### Status

ABRQ-EXT 0 CLOSED. ABRQ-EXT 1 selected: String HTML methods.
