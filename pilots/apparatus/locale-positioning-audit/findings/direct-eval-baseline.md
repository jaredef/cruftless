# Direct Eval Baseline Partition

**Locale**: `apparatus/locale-positioning-audit`
**Rung**: LPA-EXT 11
**Date**: 2026-05-27
**Candidate**: `direct-eval-lexical-capture`

## Probe

Baseline artifact:

```text
/Users/jaredfoy/Developer/cruftless-sidecar/results/direct-eval-baseline-20260527-040212/
```

Selector:

- all 286 fixtures under `language/eval-code/direct/`
- all 61 fixtures under `language/eval-code/indirect/`

Total: 347 fixtures.

## Result

```text
PASS=131
FAIL=212
SKIP=4
NOJSON=0
TOTAL=347
```

Failure projection buckets:

| Bucket | Count | Example |
|---|---:|---|
| missing SyntaxError | 138 | `direct/arrow-fn-a-following-parameter-is-named-arguments-arrow-func-declare-arguments-assign-incl-def-param-arrow-arguments.js` |
| wrong result / binding shape | 28 | `direct/arrow-fn-body-cntns-arguments-func-decl-arrow-func-declare-arguments-assign-incl-def-param-arrow-arguments.js` |
| missing ReferenceError / TDZ | 6 | `direct/lex-env-no-init-cls.js` |
| missing TypeError | 3 | `indirect/non-definable-global-function.js` |
| other eval completion / global-env behavior | 37 | `direct/cptn-nrml-empty-block.js` |

## Partition

The baseline falsifies the broad reading of
`direct-eval-lexical-capture` as a single clean spawn. The pool contains
at least four mechanism families:

1. **Eval early-error declaration conflicts**: the largest cluster,
   mostly missing SyntaxError for direct eval declaration instantiation
   around `arguments`, lexical declarations, and strictness.
2. **Eval var/function environment instantiation**: global/local
   `var` and function declaration effects, descriptor shape, and
   non-configurable global property updates.
3. **Eval this/new.target context selection**: direct vs indirect eval
   context and caller strictness.
4. **Eval lexical TDZ / ReferenceError behavior**: uninitialized lexical
   names and class heritage evaluation.

The original Instance 4 x Axis R cue remains valid, but it is not narrow
enough to found as `direct-eval-lexical-capture` yet. The next apparatus
move should split these families into child candidates or choose the
largest early-error cluster as a separate baseline-first locale.

## Recommendation

Do not spawn the broad `direct-eval-lexical-capture` locale. Mark it
audited/split and queue child arcs:

- `eval-declaration-instantiation-early-errors`
- `eval-var-function-env-instantiation`
- `eval-this-newtarget-context`
- `eval-lexical-tdz-reference`

Each child should run a smaller focused baseline before substrate work.
