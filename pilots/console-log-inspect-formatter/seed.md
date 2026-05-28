# console-log-inspect-formatter — Seed

## Telos

`console.log` (and `.error`, `.warn`) format every non-string argument by routing through static `abstract_ops::to_string`, which returns `"[object Object]"` for any Object (including Arrays). Real Node uses `util.inspect` per arg: Arrays render as `[ 1, 2, 3 ]`, plain objects as `{ key: value }`, primitives as their natural form. The current behavior masks values during probe + makes any test fixture that does `console.log(arr)` essentially silent.

Identified via a misattribution correction: SLV.2 spot-checked `Array.from(s.union(...))` and reported `[object Object]`, attributing it to a Set wrapper-type gap. Re-probe under the corrected lens showed `Array.from` returns the right Array; the formatter is what collapsed. The Set-ops layer is fine; the console-formatter is the gap.

This is a substrate-bridge gap of the same shape as RES-EXT 1 (`.groups`), GBNE-EXT 1 (built-in descriptors), and RPTC.7 (helper-divergence): engine has the data, substrate doesn't surface it in a readable form.

## Apparatus

- `pilots/rusty-js-runtime/derived/src/intrinsics.rs::install_console` (line 5211).
- `pilots/rusty-js-runtime/derived/src/abstract_ops.rs::to_string` (the static helper that returns `"[object Object]"` for Objects).

## Methodology

1. Add `fn inspect_value(rt, v) -> String` in intrinsics.rs (or similar location) that handles, recursively up to a depth cap:
   - Primitives: their natural string form (`null`, `undefined`, numbers, booleans, strings unquoted at top level, quoted inside containers).
   - Array: `[ <element>, <element>, ... ]`. Empty: `[]`.
   - Plain Object: `{ key: <value>, ... }`. Empty: `{}`. Keys unquoted when valid identifiers, quoted otherwise.
   - Function: `[Function: name]` or `[Function (anonymous)]`.
   - Set: `Set(N) { <element>, ... }`.
   - Map: `Map(N) { key => value, ... }`.
   - Date: ISO string.
   - RegExp: `/source/flags`.
   - Error: `<Name>: <message>` plus indented stack if present.
   - Cycle guard via a `HashSet<ObjectRef>` of in-progress containers.
2. Switch console.log/error/warn to call `inspect_value` per arg instead of `abstract_ops::to_string`. Strings at the TOP level remain unquoted (Node behavior); nested strings are quoted.
3. Cap recursion depth (Node default: 2). Beyond depth: `[Array]` / `[Object]`.

## Carve-outs

- Full `util.inspect` option support (`colors`, `breakLength`, `compact`, `getters`, etc.): not in this rung.
- Custom `Symbol.for('nodejs.util.inspect.custom')` hook dispatch: deferred.
- `console.dir`, `console.table`, `console.group`: deferred.
- Color codes: deferred (Node only emits when stdout is a TTY; not needed for probe correctness).
- ANSI-aware width calculation: not needed for v1.

## Composes-with

- Doc 729 (resolver-instance: console is a host-surface resolver above the spec layer).
- Standing Rule 20 (cross-module discipline-drift): `abstract_ops::to_string` use at a user-visible boundary is the bug-pattern; this is the same shape RPTC.7 has been closing across the substrate.
- SLV.2 (the originating misattribution): this locale corrects the spot-check that pointed at the wrong layer.

## Resume protocol

Read `trajectory.md` tail; if `agent-feedback.md` is present, read its head + most recent review per apparatus/docs/agent-feedback-schema.md.

---

## Cross-arc disposition (2026-05-28)

Per coverage-gap-orphan-disposition-2026-05-28.md §II.4: DAG ↑ host-runtime tier. Deferred enrollment in the future `host-node-compat` sub-arc when the `2026-05-28-host-runtime-api-conformance` umbrella is scaffolded with its sub-arc decomposition per Doc 744 §IV.2. Spec source is Node `util.inspect` (not WHATWG), so it sits in the node-compat sub-arc, not web-platform.
