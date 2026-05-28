# eval-var-function-env-instantiation — Trajectory

## EVFEI-EXT 0 — founding baseline (2026-05-28)

**Trigger**: After EDIEE closed the direct-eval declaration-conflict
early-error bucket, the remaining LPA-EXT 11 eval partition still had a
coherent var/function environment-instantiation cluster. The candidate
registry had `eval-var-function-env-instantiation` marked
baseline-first.

**Collision check**:

- `eval-declaration-instantiation-early-errors/` owns missing
  SyntaxError declaration conflicts and is already active.
- `eval-scope-binding-chain/` owns indirect eval script/global scope
  selection and is already active.
- `direct-eval-lexical-capture/` exists but is too broad for this
  mechanism.
- No `eval-var-function-env-instantiation/` locale existed in the
  manifest before this spawn.

**Baseline artifacts**:

```text
/Users/jaredfoy/Developer/cruftless-sidecar/results/eval-var-function-env-baseline-20260528-072957/
/Users/jaredfoy/Developer/cruftless-sidecar/results/eval-var-function-env-full-20260528-073029/
```

Focused 16-row sample:

```text
PASS=6
FAIL=10
SKIP=0
NOJSON=0
TOTAL=16
```

Full 61-row pool:

```text
PASS=29
FAIL=32
SKIP=0
NOJSON=0
TOTAL=61
```

**Finding EVFEI.1 (materialization, not early error)**: the failing rows
do not ask for missing SyntaxError. They ask for eval declarations to
materialize as bindings and object descriptors in the selected variable
environment. This makes the locale distinct from EDIEE.

**Finding EVFEI.2 (global descriptor shape is part of the same mechanism)**:
rows such as `var-env-var-init-global-exstng.js` and
`var-env-func-init-global-update-configurable.js` fail on descriptor
value/enumerable/writable/configurable assertions, not on parse or
runtime completion. The binding materialization helper must own both the
binding and descriptor shape.

**Status**: EVFEI-EXT 0 CLOSED. Locale founded; next move should start
with direct eval local `var`/function declaration materialization before
global descriptor update cases.

## EVFEI-EXT 1 — pipeline-form pickup before materialization (2026-05-28)

**Trigger**: Keeper directive after reading the newly-committed
`docs/engagement/prospective/pipeline-form-discovery-as-predictive-heuristic.md`:
"Spawn the locale or if the trajectory is already open, pick it up."
Registry check found `eval-var-function-env-instantiation/` already
spawned and load-bearing for this surface, so this rung resumes the
existing trajectory rather than founding a sibling.

**Baseline rerun**:

```text
T262_ROOT=/Users/jaredfoy/Developer/cruftless-sidecar/test262 \
CRUFTLESS_SIDECAR=/Users/jaredfoy/Developer/cruftless-sidecar \
TEST_ARTIFACTS_DIR=/Users/jaredfoy/Developer/cruftless-sidecar/results \
CRUFT_BIN=/Users/jaredfoy/Developer/cruftless/target/debug/cruft \
pilots/eval-var-function-env-instantiation/exemplars/run-exemplars.sh
```

Result:

```text
PASS=6
FAIL=10
SKIP=0
NOJSON=0
TOTAL=16
```

This matches EVFEI-EXT 0 and confirms the trajectory is still open.

**Pipeline four-tuple (Doc 745 candidate heuristic)**:

- **Mouth (M)**: direct or indirect `eval(source)` where `source` is
  Script-shaped text containing `var` declarations or hoistable function
  declarations.
- **Terminus (T)**: `EvalDeclarationInstantiation` has created or updated
  the selected variable environment before eval body execution, with the
  correct observable value and global descriptor shape.
- **Interior (I)**: eval call-site lowering (`Op::DirectEval` versus
  ordinary global eval call) → parser Script source ingestion →
  var/function declaration collection → declaration-instantiation
  materializer → eval execution frame/global object → test262 observable
  value/descriptor assertions.
- **Relations (R)**: DAG relation from parser/bytecode call-site lowering
  into runtime eval execution; lattice relation with
  `eval-declaration-instantiation-early-errors/` (same declaration list,
  different terminus) and `eval-scope-binding-chain/` (same Script/global
  environment substrate, different terminus); alphabet-exchange relation
  at the runtime global-binding helpers shared with GBSU/script-mode
  evaluation.

**Finding EVFEI.3 (materialization cannot be the first edit until the
mouth is proven)**: A direct local materialization edit would be a
mis-stated-pipeline move if the failing direct rows are not demonstrably
entering the `Op::DirectEval` path. EDIEE's prior work introduced the
opcode, but EVFEI's failures are still compatible with an interior
misalignment at the call-site/callee-check point: syntactic `eval(...)`
may lower correctly while runtime falls back to the registered global
eval native if the callee identity test fails, or the eval source may be
rerouted through the globalish expression-wrapper path before declaration
instantiation can operate.

**Finding EVFEI.4 (first proof probe is bytecode-mouth shape)**:
Per the pipeline heuristic, the first coherent move was to prove the
mouth-to-interior transition at call-site lowering:

```text
cargo test -p rusty-js-bytecode eval_call -- --nocapture
```

Result:

```text
direct_eval_call_emits_direct_eval ... ok
indirect_eval_call_remains_ordinary_call ... ok
```

Added probe:

- `eval('var x;')` disassembles with `DirectEval 1`.
- `(0, eval)('var x;')` disassembles with ordinary `Call 1` and no
  `DirectEval`.

This proves the EVFEI mouth is correctly marked at the bytecode tier.

**Finding EVFEI.5 (next interior point is runtime eval entry)**:
Since the call-site mouth is proven, the remaining pre-materialization
proof is runtime-side:

1. Instrument or test the runtime branch that checks
   `global_get("eval") == callee`, because a false negative there routes
   the call into ordinary global eval and bypasses `direct_eval_from_frame`.
2. Prove that direct-eval statement-list execution does not pass through
   the globalish expression-wrapper path before declaration
   instantiation.
3. Only after runtime eval entry is proven should EVFEI-EXT 2 add the
   local variable-environment materializer for `var` and function
   declarations.

**Status**: EVFEI-EXT 1 OPEN. Locale resumed; no sibling locale spawned.
Bytecode-mouth proof is complete; next action is the runtime eval-entry
proof probe.

**Runtime-entry proof**:

```text
cargo test -p rusty-js-runtime eval --test run_golden -- --nocapture
```

Result:

```text
direct_eval_reads_caller_variable_environment ... ok
indirect_eval_does_not_read_caller_variable_environment ... ok
```

Added probes:

- `eval("x")` inside a function with `var x = 7` returns `7`.
- `(0, eval)("x")` inside the same caller throws `ReferenceError`.

**Finding EVFEI.6 (runtime entry is proven; materialization is now the
interior blocker)**: The EVFEI mouth-to-runtime-entry path is coherent:
syntactic eval lowers to `Op::DirectEval`, and runtime dispatch enters the
direct eval overlay path. The focused failures therefore sit at the next
interior point: declaration-instantiation materialization before eval body
execution. Direct eval currently overlays existing caller bindings, but it
does not create eval-introduced `var` bindings initialized to `undefined`
or function bindings initialized to instantiated function objects before
running the source.

**Next substrate move**: EVFEI-EXT 2 may now target direct eval local
variable-environment materialization, because the pipeline mouth and
runtime-entry interior point have both been proven.

## EVFEI-EXT 2 — scoped direct-eval overlay materialization (2026-05-28)

**Change**:

- Extended direct eval's temporary global overlay from value-only entries
  to target-aware entries (`Temporary`, caller `Local`, caller `Upvalue`).
- Collected eval `var` and function declaration names before execution.
- Preinstalled eval-introduced `var` names as `undefined` in the overlay
  when the global object remains extensible and the name is not already an
  own global property.
- Preinstalled simple function declarations as function objects for the
  same direct-eval overlay path.
- Wrote post-eval overlay values back into captured caller local/upvalue
  targets before restoring/removing the temporary global properties.

**Verification**:

```text
cargo build -p cruftless --bin cruft
pilots/eval-var-function-env-instantiation/exemplars/run-exemplars.sh
pilots/direct-eval-lexical-capture/exemplars/run-exemplars.sh
cargo test -p rusty-js-bytecode eval_call -- --nocapture
cargo test -p rusty-js-runtime eval --test run_golden -- --nocapture
```

Results:

```text
EVFEI exemplars: PASS=7 FAIL=9 SKIP=0 NOJSON=0 / 16
Direct-eval lexical exemplars: PASS=3 FAIL=0 / 3
bytecode eval_call probes: 2 passed
runtime eval run_golden probes: 6 passed
```

**Finding EVFEI.7 (target-aware overlay closes one row, but module-top
bindings remain a separate interior point)**: The rung moves the focused
sample from 6/16 to 7/16 by letting direct eval update an existing
non-configurable global function binding case. It does not close the
three local `initial`/`postAssignment` rows because those test variables
are top-level module bindings that the outer function does not lexically
capture in its compiled descriptor set. Since direct eval source is
runtime text, the compiler does not know that `initial` or
`postAssignment` must become upvalues of the containing function.

This names the next interior point: direct eval needs either a real
caller environment record chain or a module-top binding mirror that can
serve as a write target for names not statically captured by the caller
frame. Copying more names into the temporary global overlay is no longer
enough; the writeback target itself is missing.

**Status**: EVFEI-EXT 2 PARTIAL. Keep the target-aware overlay because it
is monotonic on the focused probe and preserves DELC. Next rung should
resolve the module-top binding write target before broad global descriptor
work.
