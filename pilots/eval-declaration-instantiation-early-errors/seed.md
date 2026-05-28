# eval-declaration-instantiation-early-errors — Seed

## Substrate Pilot

Spawned from `apparatus/locale-positioning-audit` LPA-EXT 11 after the
direct-eval baseline split `direct-eval-lexical-capture` into narrower
substrate arcs. This is a top-level substrate pilot, not an apparatus
pilot: it owns parser/eval/runtime semantics for direct eval declaration
instantiation early errors.

## Telos

Materialize the coordinate:

```text
parser/runtime-eval ::
  E4/direct-eval-declaration-instantiation ::
  cut/early-error-conflict-detection ::
  property/direct-eval-rejects-conflicting-declaration-bindings
```

The induced property is that direct eval code performs the required
declaration-instantiation conflict checks before execution. The initial
observed failures are missing `SyntaxError` for combinations of function
kind, `arguments` binding, lexical declarations, var/function
declarations, and strictness.

## Apparatus

- LPA baseline artifact:
  `/Users/jaredfoy/Developer/cruftless-sidecar/results/direct-eval-baseline-20260527-040212/`
- LPA partition:
  `pilots/apparatus/locale-positioning-audit/findings/direct-eval-baseline.md`
- Focused exemplar suite:
  `pilots/eval-declaration-instantiation-early-errors/exemplars/exemplars.txt`
- Focused runner:
  `pilots/eval-declaration-instantiation-early-errors/exemplars/run-exemplars.sh`
- Likely substrate sites:
  - `pilots/rusty-js-parser/derived/src/`
  - `pilots/rusty-js-bytecode/derived/src/compiler.rs`
  - `pilots/rusty-js-runtime/derived/src/interp.rs`

## Baseline

LPA-EXT 11 ran all 286 `language/eval-code/direct/` rows plus all 61
`language/eval-code/indirect/` rows:

```text
PASS=131
FAIL=212
SKIP=4
NOJSON=0
TOTAL=347
```

This locale owns the dominant missing-SyntaxError bucket:

```text
missing SyntaxError / eval declaration-conflict early errors: 138 rows
```

The focused exemplar suite starts with 16 representative rows across
arrow functions, ordinary functions, methods, generators, async
functions, and async generators.

## Methodology

- **EDIEE-EXT 0**: founding and focused exemplar extraction.
- **EDIEE-EXT 1**: classify representative rows by conflict shape:
  `arguments` parameter conflict, body lexical conflict, body var
  conflict, and function declaration conflict.
- **EDIEE-EXT 2**: implement the smallest early-error path that rejects
  direct eval declaration conflicts without changing non-eval parsing.
- **EDIEE-EXT 3**: broaden across function-kind variants only after the
  mechanism proves coherent on the focused set.

If inspection shows the dominant owner is parser-only syntax validation,
keep the fix at parser/early-error tier. If it requires runtime
environment records to know caller bindings, move the implementation to
eval declaration-instantiation in runtime and keep parser changes
minimal.

## Carve-outs

- Eval `var`/function binding effects after successful declaration
  instantiation belong to `eval-var-function-env-instantiation`.
- Eval `this` / `new.target` context selection belongs to
  `eval-this-newtarget-context`.
- Eval TDZ / missing ReferenceError rows belong to
  `eval-lexical-tdz-reference`.
- Indirect eval rows are contrast probes only unless the same
  declaration-instantiation early-error mechanism owns them.

## Composes With

- `pilots/apparatus/locale-positioning-audit/trajectory.md` LPA-EXT 11.
- `apparatus/locales/CANDIDATES.md` Tier N child arc.
- `apparatus/docs/predictive-ruleset.md` Rule 23.

## Resume Protocol

Read `trajectory.md` tail, then run:

```text
pilots/eval-declaration-instantiation-early-errors/exemplars/run-exemplars.sh
```

Inspect failures by conflict shape before editing parser/runtime code.

## Status

EDIEE-EXT 0 FOUNDED locally. First implementation rung should classify
the 16-row focused suite before making substrate edits.
