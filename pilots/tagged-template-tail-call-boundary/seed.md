# tagged-template-tail-call-boundary seed

Status: spawned 2026-05-28

## Telos

Close the tail-call control-flow boundary exposed by the two remaining
tagged-template Test262 rows:

- `language/expressions/tagged-template/tco-call.js`
- `language/expressions/tagged-template/tco-member.js`

These rows were carved out from `tagged-template-object-boundary` after
TemplateStringsArray construction, raw/cooked preservation, freezing,
source-site cache identity, direct-eval context, and cross-realm harness
coverage reached zero ordinary FAIL rows.

## Coordinate

- Resolver: `ast-to-bytecode/control-flow-lowering`
- Rung: `E2/internal-method:execution-semantics`
- Axis: `R/ast-to-bytecode`
- Availability: `available-surface`
- Primary cut kind: tail-position call lowering / frame-control semantics
- Primary projection: tagged-template call/member forms in proper-tail-call
  position abort before the Test262 runner can emit JSON.

## Apparatus Basis

This locale is spawned from TTBO/TTOB terminal carve-out evidence:

- `tagged-template-object-boundary` exemplar set is `25 PASS / 0 FAIL /
  2 NOJSON`.
- The only residual rows are TCO-specific tagged-template call/member forms.
- The residual is no longer a TemplateStringsArray object-boundary symptom.
  It belongs to call-frame/control-flow lowering.

## Baseline Read

Initial two-row focused baseline:

| Fixture | Status |
|---|---|
| `tco-call.js` | NOJSON / abort |
| `tco-member.js` | NOJSON / abort |

Expected closure signal:

- Both fixtures should produce JSON through the Test262 runner.
- Passing is the target, but the first rung may be diagnostic if it converts
  host aborts into ordinary FAIL reasons.

## Invariants

- Tagged-template call lowering must continue to receive the correct
  TemplateStringsArray argument 0 and substitutions after it.
- Member tagged-template forms must preserve receiver / `this` behavior.
- Tail-position handling must not regress ordinary tagged-template rows.
- Any TCO-specific relaxation must be explicit; do not hide aborts by
  skipping the fixtures in the runner.

## Falsifiers

- If the no-JSON rows are caused by runner process handling rather than
  runtime call lowering, redirect to the Test262 runner apparatus before
  changing engine code.
- If the first diagnostic conversion points at general tail-call syntax or
  parser support, split to a broader `proper-tail-call-boundary` locale.
- If tagged-template lowering itself regresses, return the fix to
  `tagged-template-object-boundary`.

## Trajectory

1. Reproduce the two-row no-JSON baseline locally.
2. Convert no-JSON aborts into ordinary Test262 FAIL reasons if possible.
3. Identify whether the failure is parser, bytecode lowering, runtime frame
   unwinding, or runner behavior.
4. Close TCO call/member rows without changing TTBO's ordinary pass surface.

## Resume Rule

On resume, read this seed, `trajectory.md`, and run
`exemplars/run-exemplars.sh` before modifying parser, bytecode, runtime, or
runner code.
