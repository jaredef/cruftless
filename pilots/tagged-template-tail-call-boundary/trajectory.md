# tagged-template-tail-call-boundary trajectory

## TTTC-EXT 0 - Spawn and carve-out adoption (2026-05-28)

Trigger: keeper directive to spawn the locale for the two remaining
tagged-template carve-out rows.

Source locale:

- `tagged-template-object-boundary` moved the 27-row tagged-template set to
  `25 PASS / 0 FAIL / 2 NOJSON`.
- The two residual rows are `tco-call.js` and `tco-member.js`.

Decision:

- Spawn `tagged-template-tail-call-boundary` as a separate control-flow
  lowering locale.
- Do not keep these rows inside TTBO/TTOB, because the TemplateStringsArray
  object/caching/realm surface is closed for ordinary failures.

Artifacts:

- `seed.md` records the control-flow coordinate, invariants, falsifiers, and
  resume rule.
- `exemplars/exemplars.txt` contains the two TCO fixtures.
- `exemplars/run-exemplars.sh` runs the focused no-JSON probe.

Initial baseline:

- Command: `pilots/tagged-template-tail-call-boundary/exemplars/run-exemplars.sh`
- Result: `0 PASS / 0 FAIL / 2 NOJSON`
- Residuals:
  - `language/expressions/tagged-template/tco-call.js :: NOJSON`
  - `language/expressions/tagged-template/tco-member.js :: NOJSON`

Read:

- The carve-out reproduces as a no-JSON abort in the focused locale.
- First substrate rung should convert aborts to ordinary FAIL reasons before
  changing call semantics.

## TTTC-EXT 1 - Host abort converted to ordinary failure (2026-05-28)

Move:

- Added a JS call-dispatch depth guard in
  `pilots/rusty-js-runtime/derived/src/interp.rs`.
- The guard returns `RangeError("Maximum call stack size exceeded")` instead of
  allowing recursive JS calls to exhaust the Rust host stack.

Verification:

- Command: `cargo build --release --bin cruft -p cruftless`
- Result: build PASS (existing workspace warning set unchanged).
- Command: `pilots/tagged-template-tail-call-boundary/exemplars/run-exemplars.sh`
- Result: `0 PASS / 2 FAIL / 0 NOJSON`

Residuals:

- `language/expressions/tagged-template/tco-call.js`
  - `FAIL :: $MAX_ITERATIONS is not defined @file://<eval:0:stmt>:228:23`
- `language/expressions/tagged-template/tco-member.js`
  - `FAIL :: $MAX_ITERATIONS is not defined @file://<eval:0:stmt>:228:23`

Read:

- The no-JSON crash was a host-stack symptom, now closed at the runtime call
  boundary.
- The first ordinary failure is not yet tail-call semantics. It exposes a
  prerequisite eval declaration-instantiation leak: the test262 include
  `tcoHelper.js` is concatenated, but its top-level
  `var $MAX_ITERATIONS = 100000` is not visible to the later strict indirect
  eval statement.
- Independent repro:
  `(0, eval)('"use strict";\nvar x_eval_probe = 7;\nconsole.log(x_eval_probe);')`
  fails under cruft with `x_eval_probe is not defined`, while Bun prints `7`.

Next:

- Close the strict indirect-eval var visibility leak in the eval
  declaration-instantiation locale, then rerun TTTC to expose the actual
  proper-tail-call boundary.
