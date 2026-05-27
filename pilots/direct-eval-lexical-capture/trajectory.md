# direct-eval-lexical-capture trajectory

## DELC-EXT 0 - Spawn (2026-05-27)

Trigger: residual audit from `tagged-template-object-boundary` after ordinary tagged-template object semantics reached `PASS=21 FAIL=6 ABORT=2 / 27`.

Decision: spawn a separate direct-eval lexical capture locale. The tagged-template residuals fail with `tag is not defined` inside eval before template object semantics execute, so the correct owner is eval environment selection.

Initial constraints:

- Preserve indirect eval as global-only.
- Do not merge lex-error propagation; LEP already owns that surface.
- Treat `$262.createRealm` as realm/harness support, not this locale.
- Begin with read-only capture before declaration-instantiation semantics.

Next:

- Run the focused baseline.
- Inspect bytecode lowering of `eval(...)` and runtime call dispatch.
- Determine whether caller frame access exists; if not, document the substrate requirement before attempting a broad implementation.

## DELC-EXT 1 - Focused Baseline and Substrate Audit (2026-05-27)

Command: `./pilots/direct-eval-lexical-capture/exemplars/run-exemplars.sh`

Result: `PASS=0 FAIL=3 / 3 (0.0%)`

Tagged-template residual reasons:

- `cache-differing-expressions-eval.js`: `tag is not defined`
- `cache-eval-inner-function.js`: `tag is not defined`
- `cache-identical-source-eval.js`: `tag is not defined`

Micro-probe:

```js
function tag() { return 41; }
let x = 42;
let out = [];
try { out.push(eval('tag()')); } catch (e) { out.push('tag:' + e.name); }
try { out.push(eval('x')); } catch (e) { out.push('x:' + e.name); }
try { out.push((0, eval)('x')); } catch (e) { out.push('ix:' + e.name); }
console.log(out.join('|'));
```

Output:

```text
tag:ReferenceError|x:ReferenceError|ix:ReferenceError
```

Substrate read:

- Runtime `eval` is installed as a normal global function.
- Its implementation calls `evaluate_module(source, url)` and intentionally evaluates source in a global/module context.
- The implementation comment already marks spec-correct direct eval as deferred because frames currently live on Rust's call stack through recursive `call_function`, not in a runtime frame-stack field.
- `Op::Call` does not distinguish direct eval at the bytecode site, and native function invocation does not receive the caller `Frame`.

Finding DELC.1:

The next implementation move is not a local edit inside `eval`; it requires a call-site capability path from bytecode/runtime dispatch into the eval intrinsic. At minimum, direct eval needs one of:

- a dedicated `Op::DirectEval` emitted only for syntactic `eval(...)`;
- or a native-call ABI extension that can receive a caller-frame environment snapshot;
- plus an eval execution path that resolves identifiers against that captured lexical environment while preserving indirect eval's global-only behavior.

Do not patch this by copying selected globals: that would make the three tagged-template rows pass accidentally while leaving `let`/`const` direct eval wrong and risking indirect-eval regressions.

## DELC-EXT 2 - DirectEval Opcode and Read-Capture Overlay (2026-05-27)

Change:

- Added `Op::DirectEval` and lowered only syntactic `eval(...)` call sites to it.
- Kept the resolved callee on the stack; runtime only enters direct-eval mode when the callee is the current global eval function, otherwise it falls back to ordinary call semantics.
- Factored indirect/global eval execution into `Runtime::eval_source_globalish`.
- Implemented a scoped read-capture overlay for caller locals and upvalues while the eval body runs.
- Recovered logical names for compiler-scoped descriptors such as `<scoped@N>a`, allowing direct eval inside `for (let a...)` to observe `a`.

Verification:

- `cargo check -p rusty-js-runtime`
- `cargo build --release -p cruftless`
- `./pilots/direct-eval-lexical-capture/exemplars/run-exemplars.sh`

Interim result:

- First cut moved the locale from `PASS=0 FAIL=3 / 3 (0.0%)` to `PASS=2 FAIL=1 / 3 (66.7%)`.
- The remaining failure changed from lexical capture (`a is not defined`) to template-site identity across eval invocations.

Read:

This is intentionally not a full PerformEval implementation. It is a call-site-capable substrate rung: direct eval can now see active caller bindings for read-shaped fixtures, and indirect eval remains global. The remaining semantic debt is declaration instantiation, assignment back into caller bindings, and exact lexical-scope liveness instead of descriptor-name recovery.

## DELC-EXT 3 - Eval Instance as Template Site Identity (2026-05-27)

Finding:

Once lexical capture worked, `cache-eval-inner-function.js` failed at the assertion that template objects from separate eval invocations are not SameValue. The parser site key was only `start:end`, so identical eval source strings reused one template object across different eval parses.

Change:

- `__template_object__` now prefixes the parser-emitted source-site key with the active module/eval URL from `current_module_url`.
- Ordinary module/function template sites remain stable under the same URL.
- Each eval invocation already receives a unique `file://<eval:N>` URL, so identical source text in separate eval calls now has distinct template site identity while repeated execution inside one eval invocation still reuses the same key.

Expected closure:

- Direct-eval tagged-template cache rows should now close except realm-owned `$262.createRealm` coverage.

Verification:

- `cargo check -p rusty-js-runtime`
- `cargo build --release -p cruftless`
- `./pilots/direct-eval-lexical-capture/exemplars/run-exemplars.sh`
- `./pilots/tagged-template-object-boundary/exemplars/run-exemplars.sh`

Result:

- Direct-eval lexical exemplars: `PASS=3 FAIL=0 / 3 (100.0%)`.
- Tagged-template exemplars: `PASS=24 FAIL=3 ABORT=2 / 27 (88.9%)`.

Remaining tagged-template residuals:

- `cache-realm.js`: `$262.createRealm` / realm harness ownership.
- `tco-call.js`: no-json / tail-call ownership.
- `tco-member.js`: no-json / tail-call ownership.

Micro-probe:

```js
function tag() { return 41; }
let x = 42;
let out = [];
try { out.push(eval('tag()')); } catch (e) { out.push('tag:' + e.name); }
try { out.push(eval('x')); } catch (e) { out.push('x:' + e.name); }
try { out.push((0, eval)('x')); } catch (e) { out.push('ix:' + e.name); }
console.log(out.join('|'));
```

Output:

```text
41|42|ix:ReferenceError
```

Closure:

The locale's read-shaped direct-eval lexical capture and eval template-site identity objectives are closed. The remaining work is a deeper PerformEval substrate if/when assignment/declaration fixtures become the target.
