# json-parse-reviver-semantics — Seed

**Locale tag**: `L.json-parse-reviver-semantics`

**Status**: FOUNDED 2026-05-28 from the JSON carve-out search after applying
the pipeline-form discovery heuristic.

## Telos

Close `JSON.parse(text, reviver)` semantics, distinct from raw JSON proposal
availability, JSON.stringify traversal, Date.prototype.toJSON, and Temporal
toJSON surfaces.

Current matrix search against
`pilots/apparatus/test262-categorize/full-suite/results/test262-full-2026-05-27-161641/interpreted.jsonl`
finds **20 JSON.parse failures**:

```text
10 abrupt-completion/throw-missing
 9 value-semantics/wrong-result
 1 abrupt-completion/wrong-throw-type
```

The dominant cluster is reviver semantics: bottom-up walk order, holder/root
wrapper, delete-on-undefined, property get/delete/define abrupt propagation,
prototype visibility, Proxy interaction, and text coercion before parse.

## Pipeline Form

Per `docs/engagement/prospective/pipeline-form-discovery-as-predictive-heuristic.md`:

- **Mouth (M)**: `JSON.parse(text, reviver)` call, including text argument
  coercion and optional callable reviver.
- **Terminus (T)**: a parsed JS value tree after spec `InternalizeJSONProperty`
  has walked the holder/root structure, called reviver with `(key, value)`,
  deleted properties when the reviver returns `undefined`, and propagated
  abrupt completions.
- **Interior (I)**:
  1. text coercion via strict `ToString`;
  2. JSON grammar parse into cruft object/array/value substrate;
  3. synthetic root holder object with empty-string property;
  4. post-order object/array traversal;
  5. per-key `Get`, reviver `Call`, `Delete`/`Set` observable effects.
- **Relations (R)**:
  - lattice with Proxy/internal-method semantics at `Get`, `Delete`, own-key,
    and define-property points;
  - lattice with Array length/prototype visibility for array reviver rows;
  - disjoint from current eval/global declaration work and from
    `ihi-array-entries` performance work.

## Apparatus

- Exemplar list: `exemplars/exemplars.txt` (the 20 current JSON.parse rows).
- Runner: `exemplars/run-exemplars.sh`.
- Runtime entry point:
  `pilots/rusty-js-runtime/derived/src/interp.rs::json_parse_via`.
- Parser/materializer:
  `pilots/rusty-js-runtime/derived/src/intrinsics.rs::json_parse`.
- Existing current behavior: `json_parse_via` ignores the reviver argument and
  uses static `abstract_ops::to_string`, so object text coercion abrupts are
  currently misclassified as parse `SyntaxError`.

## First Rungs

1. **JPRS-EXT 0**: founding and focused exemplar suite.
2. **JPRS-EXT 1**: baseline inspection. Run all 20 rows and split exact
   families: text coercion, root holder, call order, delete-on-undefined,
   abrupt propagation, Proxy/prototype visibility.
3. **JPRS-EXT 2**: text coercion + callable reviver gate. Use runtime
   `ToString` so `text-object*.js` rows stop misrouting object coercion into
   JSON grammar syntax errors.
4. **JPRS-EXT 3**: `InternalizeJSONProperty` root-holder walk, delete-on-
   undefined, and post-order call sequence.
5. **JPRS-EXT 4**: Proxy/prototype/abrupt edge rows if not closed by the
   generic internalize walk.

## Carve-Outs

- `JSON.stringify` replacer/space/property traversal remains separate.
- `JSON.rawJSON` / `JSON.isRawJSON` are proposal-method availability, not this
  locale.
- `Date.prototype.toJSON` and Temporal `toJSON` rows are separate intrinsic or
  Temporal class work.
- Cross-realm `$262` rows remain harness/realm substrate unless a local row
  proves otherwise.

## Resume Protocol

Read this seed, then `trajectory.md`. Run:

```sh
T262_ROOT=/Users/jaredfoy/Developer/cruftless-sidecar/test262 \
CRUFT_BIN=/Users/jaredfoy/Developer/cruftless/target/debug/cruft \
pilots/json-parse-reviver-semantics/exemplars/run-exemplars.sh
```

Before substrate edits, check `git status --short` and avoid colliding with
eval/global declaration or bytecode `DefineGlobal` work.
