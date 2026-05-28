# eval-var-function-env-instantiation — Seed

## Substrate Pilot

Spawned from `apparatus/locales/CANDIDATES.md` after the LPA-EXT 11
direct-eval baseline split the broad `direct-eval-lexical-capture` cue
into narrower eval mechanisms. This locale owns eval `var` and function
declaration instantiation effects after EDIEE closed the declaration
conflict early-error bucket.

## Telos

Materialize the coordinate:

```text
runtime-eval ::
  E4/eval-declaration-instantiation ::
  binding-materialization ::
  property/eval-var-and-function-declarations-reify-in-the-right-environment
```

The induced property is that direct and indirect eval create, update, and
describe `var` and function bindings according to the selected eval
variable environment, including global object descriptor shape and
non-configurable global behavior.

## Apparatus

- LPA partition:
  `pilots/apparatus/locale-positioning-audit/findings/direct-eval-baseline.md`
- Candidate registry:
  `apparatus/locales/CANDIDATES.md`, entry `eval-var-function-env-instantiation`.
- Sidecar baseline:
  `/Users/jaredfoy/Developer/cruftless-sidecar/results/eval-var-function-env-full-20260528-073029/`
- Focused exemplar suite:
  `pilots/eval-var-function-env-instantiation/exemplars/exemplars.txt`
- Focused runner:
  `pilots/eval-var-function-env-instantiation/exemplars/run-exemplars.sh`
- Likely substrate sites:
  - `pilots/rusty-js-runtime/derived/src/interp.rs`
  - `pilots/rusty-js-runtime/derived/src/module.rs`
  - `pilots/rusty-js-bytecode/derived/src/compiler.rs`

## Baseline

The full candidate pool is 61 rows selected from
`language/eval-code/{direct,indirect}/`:

```text
PASS=29
FAIL=32
SKIP=0
NOJSON=0
TOTAL=61
```

The 16-row focused sample spans local and global `var`/function
initialization, configurable and non-configurable global updates, global
environment record probes, and direct/indirect contrasts:

```text
PASS=6
FAIL=10
SKIP=0
NOJSON=0
TOTAL=16
```

Failure reasons are coherent:

- function declarations inside eval are not materialized in the expected
  environment;
- `var` declarations inside eval fail to produce the expected
  `undefined` binding or update existing global values;
- global object descriptors for eval-created or eval-updated bindings
  have wrong value/enumerable/writable/configurable shape;
- non-definable global rejection rows mostly already pass and are not the
  dominant missing substrate.

## Methodology

- **EVFEI-EXT 0**: founding and baseline capture.
- **EVFEI-EXT 1**: direct eval local variable-environment materialization
  for `var` and function declarations.
- **EVFEI-EXT 2**: direct eval global object binding creation/update and
  descriptor shape.
- **EVFEI-EXT 3**: indirect eval global binding path, coordinating with
  `eval-scope-binding-chain` where script-mode global scope is the owner.
- **EVFEI-EXT 4**: non-configurable global update/rejection carve-outs if
  residual rows remain after materialization.

If a row is only about scope-chain selection for indirect eval script
mode, route it to `eval-scope-binding-chain`. If a row is about
declaration conflict rejection, route it back to
`eval-declaration-instantiation-early-errors`.

## Composes-with

- `eval-declaration-instantiation-early-errors/` — sibling early-error
  predicate already closed for missing SyntaxError rows.
- `eval-scope-binding-chain/` — sibling script/global scope selection for
  indirect eval.
- `direct-eval-lexical-capture/` — broader direct-eval cue that has been
  split into narrower mechanisms.

## Resume Protocol

Read this seed, then `trajectory.md`. Run the focused exemplar runner
before editing runtime eval or compiler script-mode code.
