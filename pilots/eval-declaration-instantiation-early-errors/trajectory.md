# eval-declaration-instantiation-early-errors — Trajectory

## EDIEE-EXT 0 — FOUNDING (2026-05-27)

Spawned from LPA-EXT 11 after the direct-eval baseline demoted broad
`direct-eval-lexical-capture` into four substrate child arcs.

Baseline source:

```text
/Users/jaredfoy/Developer/cruftless-sidecar/results/direct-eval-baseline-20260527-040212/
```

Parent baseline:

```text
PASS=131
FAIL=212
SKIP=4
NOJSON=0
TOTAL=347
```

Owned bucket:

```text
missing SyntaxError / eval declaration-conflict early errors: 138 rows
```

Focused suite:

- 16 representative missing-SyntaxError rows under
  `language/eval-code/direct/`.
- Coverage spans arrow functions, ordinary function declarations and
  expressions, methods, generators, async functions, and async
  generators.

**Finding EDIEE.1 (dominant direct-eval failure is early-error, not
lexical capture)**: the largest direct-eval bucket is missing
SyntaxError before execution. This locale should start by reproducing
the declaration-conflict predicate, not by changing eval caller
environment capture.

**Status**: EDIEE-EXT 0 FOUNDED locally. No substrate edits in this rung.

Focused baseline:

```text
$ T262_ROOT=/Users/jaredfoy/Developer/cruftless-sidecar/test262 \
  CRUFT_BIN=/Users/jaredfoy/Developer/cruftless/target/debug/cruft \
  pilots/eval-declaration-instantiation-early-errors/exemplars/run-exemplars.sh

EDIEE exemplars: PASS=2 FAIL=14 SKIP=0 NOJSON=0 / 16 (12.5%)
```

Initial pass rows:

- `async-func-decl-a-following-parameter-is-named-arguments-declare-arguments.js`
- `async-func-decl-a-preceding-parameter-is-named-arguments-declare-arguments.js`

Initial failing rows cover arrow, ordinary function, method, generator,
generator method, and async-generator forms that should reject direct-eval
declaration conflicts before execution.

## EDIEE-EXT 1 — DIRECT-EVAL `arguments` CONFLICT (2026-05-27)

Implemented the first declaration-instantiation predicate at the runtime
direct-eval entry point:

- detect caller frames where a formal parameter named `arguments`
  coexists with the engine's `arguments` slot;
- parse eval source and detect top-level var-scoped declarations of
  `arguments` (`var`, function declaration, and var-scoped loop heads);
- reject before executing the eval source with `SyntaxError`.

The move stays at runtime eval rather than parser-only because the
predicate depends on caller-frame bindings. Parser-only rejection would
lack the function parameter environment needed for this row family.

Generator calls had a second boundary issue: eager generator execution
converted `SyntaxError` into a pending generator error object. For
parameter-default eval, the error is call-time and must throw before a
generator object is returned, so generator call handling now propagates
`SyntaxError` immediately.

Focused result:

```text
$ cargo build --bin cruft -p cruftless
$ T262_ROOT=/Users/jaredfoy/Developer/cruftless-sidecar/test262 \
  CRUFT_BIN=/Users/jaredfoy/Developer/cruftless/target/debug/cruft \
  pilots/eval-declaration-instantiation-early-errors/exemplars/run-exemplars.sh

EDIEE exemplars: PASS=16 FAIL=0 SKIP=0 NOJSON=0 / 16 (100.0%)
```

Gate:

```text
$ scripts/diff-prod/run-all.sh
PASS=42 FAIL=0
```

**Finding EDIEE.2 (caller-frame predicate owns the first cluster)**:
the initial 14 failing rows were not parser syntax rows. They needed an
eval declaration-instantiation predicate with access to caller locals.

**Finding EDIEE.3 (generator error storage masked call-time eval
errors)**: the final 6 failing rows were generator and async-generator
forms. Their direct-eval `SyntaxError` was produced but stored as a
pending generator error instead of thrown at call time.

**Status**: EDIEE-EXT 1 closes the 16-row focused suite. Next expansion
should sample additional missing-SyntaxError rows from the 138-row
bucket, especially lexical/function-name conflicts beyond `arguments`.

## EDIEE-EXT 2 — DIRECT-EVAL BYTECODE AND FULL BUCKET EXPANSION (2026-05-27)

Expanded from the 16-row focused suite to the full 138-row
missing-SyntaxError bucket from the direct/indirect eval baseline.

Substrate moves:

- added explicit `DirectEval <u8>` bytecode for syntactic `eval(...)`;
- removed runtime inference of direct eval from loaded callee value, which
  had incorrectly treated `(0, eval)(...)` as direct eval;
- threaded `Frame::is_arrow` and `Frame::param_count` so
  `var arguments` rejection distinguishes arrow parameters named
  `arguments` from arrow body bindings named `arguments`;
- constrained eval var/lexical collision rejection to non-strict,
  non-arrow direct eval sources for this rung.

Measurement:

```text
$ cargo build --bin cruft -p cruftless

EDIEE missing-SyntaxError bucket:
PASS=137 FAIL=1 SKIP=0 NOJSON=0 / 138

EDIEE direct+indirect eval sweep:
PASS=278 FAIL=65 SKIP=4 NOJSON=0 / 347

Baseline comparison against
direct-eval-baseline-20260527-040212:
FAIL -> PASS: 159
PASS -> FAIL: 12
SKIP -> SKIP: 4
```

Gate:

```text
$ scripts/diff-prod/run-all.sh
Expanded upstream suite: PASS=59 FAIL=53 / 112
```

Residual:

```text
language/eval-code/direct/strict-caller-function-context.js
```

**Finding EDIEE.4 (direct eval needs syntactic lowering, not value
recognition)**: callee-value recognition collapses direct and indirect
eval. The bytecode tier must preserve the syntactic reference shape, so
`eval(...)` lowers to `DirectEval` while `(0, eval)(...)` remains
ordinary `Call`.

**Finding EDIEE.5 (arrow parameter defaults need parameter/body split)**:
arrow parameter default eval can reject when another parameter is named
`arguments`, but body lexical/var bindings named `arguments` are lower
scope and must not participate in the parameter-default eval conflict.
`Frame::param_count` gives the runtime enough coordinate data to avoid
conflating those layers.

**Post-pull integration note (2026-05-28)**: after rebasing onto
`origin/main` at IR-EXT 39, upstream already carried the syntactic
`DirectEval` opcode and global eval substrate. Reapplying the EDIEE
declaration-conflict guard restores the focused suite to 16/16 and keeps
the missing-SyntaxError bucket at 137/138. The wider direct/indirect eval
sweep now shows 159 fail-to-pass rows plus 12 old-baseline pass-to-fail
rows introduced by the newer upstream eval-global path. Those regressions
are outside this locale's missing-SyntaxError predicate and should be
handled by the eval environment/global binding follow-on locales.

**Status**: EDIEE-EXT 2 closes the declaration-conflict bucket except
for the strict-caller reserved-word row. The remaining
missing-SyntaxError row should be split as the next predicate rather than
folded into the non-strict declaration-conflict check.

## EDIEE-EXT 3 — STRICT-EVAL RESERVED BINDING CLOSURE (2026-05-28)

Closed the residual strict-caller row from EDIEE-EXT 2 by separating the
strict-eval predicate from the non-strict declaration-conflict predicate.

Substrate moves:

- compute `strict_eval` once for direct eval from caller strictness or an
  eval-source directive prologue;
- reject direct-eval var-scoped declaration names that are invalid strict
  binding identifiers (`eval`, `arguments`, `yield`, and strict-mode
  future reserved words);
- keep the existing caller lexical/var conflict guard scoped to
  non-strict direct eval, now using the same `strict_eval` predicate.

Measurement:

```text
$ cargo build --bin cruft -p cruftless

Focused residual:
language/eval-code/direct/strict-caller-function-context.js PASS

EDIEE exemplars:
PASS=16 FAIL=0 SKIP=0 NOJSON=0 / 16

EDIEE missing-SyntaxError bucket:
PASS=138 FAIL=0 SKIP=0 NOJSON=0 / 138

EDIEE direct+indirect eval sweep:
PASS=280 FAIL=63 SKIP=4 NOJSON=0 / 347
```

**Finding EDIEE.6 (strict caller is a direct-eval syntax context)**:
`PerformEval` inherits strictness from a strict caller before evaluating
the eval source. A sloppy eval string inside a strict caller therefore
uses strict binding-identifier restrictions, so `var public = 1` must
throw `SyntaxError` even without a directive prologue in the eval text.

**Status**: the EDIEE missing-SyntaxError bucket is closed at 138/138.
Remaining failures in the wider eval sweep are outside this bucket and
belong to follow-on eval environment/global binding partitions.
