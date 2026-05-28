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
