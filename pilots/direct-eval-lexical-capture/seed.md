# direct-eval-lexical-capture seed

Status: spawned 2026-05-27

## Telos

Close direct eval lexical environment capture: a call syntactically shaped as `eval(source)` must evaluate `source` against the caller's lexical and variable environments, while indirect eval `(0, eval)(source)` remains global-only.

This locale is distinct from `lex-error-propagation-to-eval-surface/`. LEP closed lex/parse errors reaching eval's catch surface. DELC is about environment selection for successful eval execution.

## Coordinate

- Resolver: `ast-to-bytecode/language-lowering`
- Rung: `E2/internal-method:execution-semantics`
- Axis: `R/ast-to-bytecode`
- Surface: `language/eval-code/direct`, tagged-template eval residuals
- Primary projection: direct eval cannot see caller lexical/function bindings

## Apparatus Basis

`apparatus/locales/CANDIDATES.md` marks `direct-eval-lexical-capture` as baseline-first. LPA names it as a fresh boundary candidate after excluding lex-error propagation and tagged-template object construction.

The immediate trigger is the `tagged-template-object-boundary` residual audit:

- `cache-differing-expressions-eval.js`: `tag is not defined`
- `cache-eval-inner-function.js`: `tag is not defined`
- `cache-identical-source-eval.js`: `tag is not defined`

Those failures occur before `GetTemplateObject`; eval cannot see the containing `tag` binding.

## Mechanism Hypothesis

The current runtime installs `eval` as a normal global function. Its implementation calls `evaluate_module(source, url)`, which compiles and runs source in a fresh module frame with global lookup. The implementation comment explicitly defers spec-correct direct eval because frames live on the Rust call stack and there is no runtime frame-stack field for caller lexical capture.

Likely implementation boundaries:

- Parser/bytecode must distinguish direct eval call sites from ordinary calls to a value named `eval`.
- Runtime must provide an eval entry that can execute against the caller frame's lexical environment.
- Indirect eval must continue to use global evaluation.
- Function, `var`, `let`, and `const` declaration instantiation inside eval must be handled deliberately; this first locale should begin with read-only capture and shadowing probes before declaration-instantiation expansion.

## Baseline Probes

- Direct eval reads outer function binding: `function tag(){}; eval("tag")`.
- Direct eval reads outer lexical binding: `let x = 1; eval("x")`.
- Direct eval shadows locally: `let x = 1; eval("let x = 2; x")`.
- Indirect eval contrast: `let x = 1; (0, eval)("x")` must not see local `x`.
- Tagged-template residuals under `language/expressions/tagged-template/cache-*-eval.js`.

## Falsifiers

- If direct eval already receives caller bindings and only declaration instantiation fails, split to `eval-function-arguments-binding-semantics`.
- If failures are parse/lex errors, redirect to `lex-error-propagation-to-eval-surface/`.
- If only `$262.createRealm` remains, redirect to realm/harness support.
- If implementing direct eval requires broad frame-stack surgery, stop after baseline and document the frame substrate requirement before editing runtime dispatch.

## Trajectory

1. Build the focused baseline and classify direct vs indirect eval behavior.
2. Locate the call lowering site for `eval(...)`.
3. Decide whether a small opcode/helper can pass caller-frame bindings safely, or whether a runtime frame-stack substrate is required.
4. Close read-only direct eval capture before declaration instantiation.

## Resume Rule

On resume, read this seed, `trajectory.md`, and `exemplars/run-exemplars.sh` before modifying eval runtime code.
