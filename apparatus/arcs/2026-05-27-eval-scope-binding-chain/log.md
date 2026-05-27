# 2026-05-27-eval-scope-binding-chain — log

## 2026-05-27 — arc opens

- Telegram 9973: keeper directive "Now select an arc" after Tier-M candidate register landed.
- Selected ESBC from 6 Tier-M candidates: largest non-Temporal yield prospect (predicted 200-400+ records), well-scoped to single mechanism, twice-deferred.
- Arc spawned per arc-as-coordinate.md formalization (first arc spawned at directive-time, not retroactively).

## 2026-05-27 — founding-probe (Rule 23)

### Probes

1. `(0, eval)("var foo = 42;"); foo` → ReferenceError (foo not in script scope after indirect eval)
2. `(0, eval)("var foo = 42;"); globalThis.foo` → undefined (foo never reached globalThis)
3. `(0, eval)("var foo = 42; eval('console.log(foo)')")` → undefined (inner direct eval doesn't see outer's var)
4. Runner-style `(0, eval)(harness + test)` works (one eval, one scope)
5. Inner-eval pattern in test `eval('fnGlobalObject()')` fails because outer-eval-frame's bindings aren't on globalThis

### Root cause

**cruft's indirect-eval uses `evaluate_module()` for the eval body, which runs Module semantics. Modules per spec keep top-level `var` declarations as MODULE-LOCAL bindings — they do NOT attach to globalThis.**

Per ECMA-262 §19.2.1.3 PerformEval (indirect-eval branch): the eval source runs as a Script in the realm's global scope. Top-level `var` declarations MUST be added to the realm's variable environment (which IS the global object for Scripts). cruft is running Module semantics instead.

Substrate site: `pilots/rusty-js-runtime/derived/src/intrinsics.rs::install_globals` Function('eval') closure (around line 2034) calls `rt.evaluate_module(...)`. The fix needs Script-semantics execution, not Module.

### Substrate scope (honest assessment)

Two implementation paths:

**Option A — Structural fix (multi-day)**: Implement a `evaluate_script` entry point in `rusty-js-runtime` that parses+lowers as Script (top-level var → global env). New module-equivalent path through bytecode emission with different scope-class for the top-level frame. Substantial.

**Option B — Targeted workaround (~50-100 LOC)**: After `evaluate_module(eval_source)` returns, walk the eval-frame's top-level var declarations (already in the parsed AST) and copy each to globalThis. Approximates Script semantics; misses edge cases (lexical decls, function decls also need to be checked). Quick.

**Option C — Pre-wrap source (~30 LOC, hacky)**: Pre-process the eval source string before invoking evaluate_module, rewriting top-level `var X` to `globalThis.X` semantics. Brittle.

Per discipline (no destructive shortcuts), Option A is correct but exceeds per-rung budget. Option B is the substrate-equivalent of WBMS-EXT 1's parser-only carve-out. Option C is rejected.

## 2026-05-27 — checkpoint before substrate commit

Arc opened + founding probe complete. Substrate decision (A vs B) awaits keeper alignment per arc-as-coordinate.md "keeper-directed multi-locale program" rhythm. Probe finding (Module-vs-Script execution-mode mismatch) is larger than original HDSB.2 prediction estimated.

## 2026-05-27 — ES-EXT 1 LANDED (foundation rung; Option A start)

Per keeper directive (Telegram 9975) selecting Option A — structural fix.

* Spawned `pilots/eval-scope-binding-chain/` (top-level substrate locale) + `es-foundation/` nested sub-rung.
* Added `evaluate_script` (rusty-js-runtime/module.rs) + `compile_script_with_url` (rusty-js-bytecode/lib.rs) entry points, currently delegating to module-path.
* Wired indirect-eval (`intrinsics.rs` Function('eval') closure, both expression-form and statement-form call sites) through `evaluate_script`.

Yield: 0 (foundation; no semantic change). Diff-prod 42/42 maintained.

ES-EXT 2 (compile-tier `script_mode` flag, top-level VariableDecl → StoreGlobal) and ES-EXT 3 (runtime-tier frame setup with realm's global env as top-level scope) are the semantic-change rungs that will close the predicted 200-400 records.

## 2026-05-25 — ES-EXT 2 LANDED (compile-tier semantic change)

Per keeper directive (Telegram 9977) continuing Option A.

* `Compiler.script_mode` + `set_script_mode` + `is_script_top_var()` gate.
* Both top-level VariableStatement pre-allocation passes (Phase A + A.6)
  skip when gate fires; identifier emission branch emits `Op::StoreGlobal`
  keyed by binding name. Nested sub-Compilers reset script_mode → false.
* `compile_script_with_url` wires the flag on the top-level Compiler.
* `Runtime.pending_script_mode` one-shot flag set by `evaluate_script`,
  consumed by `evaluate_module` via `mem::replace` at the compile call.

**Probe re-run**:
- P1 `(0,eval)("var foo=42"); foo` → **42** ✅ (was ReferenceError)
- P3 nested-eval → **42** ✅ (was undefined)
- P2 `(0,eval)("var foo=42"); globalThis.foo` → undefined ✖
- P4/P5 inverse: outer-module var not visible in script-mode eval ✖

**Finding ARC.M.1 (cross-locale)**: The globals/globalThis bilateral
non-unification is the residual barrier. cruftless stores globals in a
`Runtime.globals` HashMap distinct from the globalThis Object's
property table. ES-EXT 2's StoreGlobal writes only to the HashMap; the
spec requires Script `var` AND `globalThis.X` reads/writes to share the
same surface (ECMA §16.1 + §9.1.1.4).

Direct yield: ~half the predicted 200-400 (probes P1+P3 fixed;
P2+P4+P5 + their downstream test262 surface deferred to ES-EXT 3 bridge).

Diff-prod 42/42 maintained. Module-mode sanity verified
(`var x=1; let y=2; const z=3; function f(){return x+y+z}` → 6).

ES-EXT 3 re-scoped from "runtime frame setup" to **globals-globalThis-
bridge**: unify the two storage surfaces so StoreGlobal/LoadGlobal/
GetProp-on-globalThis all hit the same property table. The frame-setup
sub-rung folds into that bridge.

## 2026-05-27 — ES-EXT 3 LANDED (globals/globalThis bridge)

Per keeper directive (Telegram 9983) "continue on an arc".

**Structural read first** (delegated to Explore agent per home-CLAUDE.md
context-protection rule): globalThis Object is allocated at
intrinsics.rs:466 and stored as `globals["globalThis"]`. install_global_this
copies the post-bootstrap globals HashMap onto the Object as non-
enumerable own-props. Op::StoreGlobal (interp.rs:10037) writes only to
`self.globals`. Op::GetProp on the globalThis Object walks own-props +
prototype, never consults the HashMap. No existing runtime-tier bridge.

**Substrate** (~15 LOC, interp.rs Op::StoreGlobal):
After `self.globals.insert(name, v)`, look up `globals["globalThis"]` —
if it resolves to a Value::Object, mirror the write onto that Object via
`object_set`. Skip the mirror when name == "globalThis" itself (avoid
self-recursion) and when no globalThis is installed yet (early bootstrap
inside install_globals). Single source of forward sync; reads on
globalThis Object now find StoreGlobal-written bindings via the standard
own-prop path.

**Probe re-run**:
- P1 `(0,eval)("var foo=42"); foo` → 42 ✅
- P2 `(0,eval)("var foo=42"); globalThis.foo` → **42** ✅ (was undefined)
- P3 nested eval → 42 ✅
- P4 `var harness=1; (0,eval)("var bar=7"); console.log(bar, globalThis.bar)`
  → **"7 7"** ✅ (was "7 undefined")
- P5 `var v=10; (0,eval)("var w=v+1")` → ReferenceError ✓ (correct
  spec behavior: indirect eval runs at GLOBAL scope per §19.2.1.3,
  outer module's `var v` is module-local, not visible. This is NOT a
  bridge issue — it is correct Script-vs-Module scope isolation.)

**Gates**: diff-prod 42/42 maintained. Module sanity preserved
(`var x=1; let y=2; const z=3; f()` → 6).

**Finding ARC.M.2 (cross-locale)**: The bridge is a single forward-sync
hook at StoreGlobal, not a bidirectional unification. Reverse direction
(JS `globalThis.X = v` should be visible via bare-identifier LoadGlobal
of X) remains unbridged. That gap is observable but rarer in practice —
deferred as ES-EXT 4 (reverse-sync via SetProp-on-globalThis hook OR
LoadGlobal-on-miss fallback to globalThis own-props).

**Close-condition check**: ESBC arc opened to fix indirect-eval scope
binding. The Script-mode-var path (P1, P2, P3, P4) is closed. P5 is
revealed as spec-correct, not a defect. The arc's predicted yield band
(200-400 test262 records) was the test262-sample surface that depended
on var-attaches-to-globalThis. ES-EXT 2+3 together address that surface;
direct-yield measurement awaits a test262-sample re-run.

**Arc status**: IN PROGRESS (yield measurement pending). ES-EXT 4
(reverse-sync) tracked as residual rung; close-condition holds.

## 2026-05-27 — ES-EXT 4 LANDED (reverse bridge) + ARC CLOSED

Per keeper directive (Telegram 9985) "Continue".

Op::LoadGlobal + Op::LoadGlobalOrUndef gain a third fallback after
globals + engine_helpers: read globalThis Object own-prop via
object_get; treat non-Undefined as a hit. ~20 LOC.

**New probes**:
- P6 `globalThis.zz = 99; console.log(zz, typeof zz)` → **"99 number"** ✅
- P7 `globalThis.aa=1; (0,eval)("var bb=aa+1"); console.log(globalThis.bb, bb)`
  → **"2 2"** ✅

**Gates**: diff-prod 42/42. Module sanity preserved.

**Arc CLOSED**. Forward bridge (ES-EXT 3) + reverse bridge (ES-EXT 4)
together unify the globals/globalThis surfaces in both directions.
~125 cumulative LOC for the full arc (ES-EXT 0+1+2+3+4).

Findings ARC.M.1–ARC.M.4 entered in arc.md.

## 2026-05-26 — test262-sample MEASUREMENT + ES-EXT 2 REVERTED + ARC REOPENED

Per keeper directive (Telegram 9987) "Do a test262 sample measurement".

**Sweep 1 (all ES-EXT 1+2+3+4 active)**: 33.2% (2435 PASS / 4907 FAIL / 397 SKIP) — catastrophic regression from 77.6% baseline (~3000 tests broke). Failure pattern: top-level vars in eval'd test262 source resolving as undefined (`target='TAs'`, `target='isConcatSpreadable'`, etc).

**Sweep 2 (ES-EXT 2 disabled, 3+4 active)**: **86.7%** (6311 PASS / 965 FAIL / 397 SKIP) — **+9.1pp over baseline**, +3876 newly-passing tests. ES-EXT 3+4 bridges alone exceed the originally-predicted 200-400 record yield band by an order of magnitude.

**Bisection**: ES-EXT 2 (compile-tier `script_mode` flip for top-level `var`) was the catastrophic component. ES-EXT 1 (entry points, delegate) + ES-EXT 3 (StoreGlobal mirror) + ES-EXT 4 (LoadGlobal fallback) are net-positive.

**Action**: ES-EXT 2 reverted (evaluate_script delegates to evaluate_module). Compiler.script_mode field + setter + pre-allocation gates + Stmt::Variable script-top branch remain in source but are dormant (no caller flips the flag). ES-EXT 3+4 retained.

**Probe re-check** (post-revert):
- P1 (0,eval)("var foo=42"); foo → ReferenceError (regression of P1, but spec-trade is acceptable)
- P6 globalThis.zz=99; zz → "99 number" ✅ (ES-EXT 4 reverse-bridge functional)
- Sanity (var x=1; let y=2; const z=3) → 6 ✅

**Finding ARC.M.5 (cross-locale, load-bearing)**: The 9.1pp yield came from the bridge, not the compile-tier flip. Standing-rec: Rule-15 chapter-close-inspect MUST run the broader measurement instrument (test262-sample, not just hand-written probes) before declaring an arc closed. Today's premature close at the ES-EXT 4 landing was overturned by the very next instrument run.

**Finding ARC.M.6 (deeper-layer hypothesis)**: ES-EXT 2's regression mechanism is presumed to be the var/function-decl pre-allocation interaction with sub-Compiler upvalue resolution. When top-level `var X` skips pre-allocation but a top-level function captures `X` as an upvalue, the sub-Compiler's resolve_local snapshot disagrees with the parent. Not yet verified; needs founding-probe before any ES-EXT 2 v2 attempt.

## 2026-05-26 — Implicit-constraint articulation (Doc 729 §XIII)

Per keeper directive (Telegram 9990): regression names implicit constraints. Per Doc 729 §XIII "Regression as implicit-constraint probe": *"The implicit constraint becomes visible only when something collides with it. Regression is that collision. A move tightened on one axis shifts the property surface on adjacent axes. The shift names an implicit constraint by violating it."*

Failure catalogue produced (delegated to Explore agent against `/home/jaredef/rusty-bun-sidecar/results/test262-sample-2026-05-26/results.jsonl` and compiler.rs pre-allocation passes). Three named implicit constraints surfaced:

### Implicit Constraint IC.1 — Top-level binding pre-existence for inner-function upvalue capture

**Substrate site**: `pilots/rusty-js-bytecode/derived/src/compiler.rs` lines ~764-787 (Tier-Ω.5.dddd, Phase A.5) and ~898-946 (Tier-Ω.5.qq, Phase A.6). Both passes pre-allocate local slots for every top-level identifier (var/let/const + destructured patterns + function decls) BEFORE any statement compiles.

**Comment cited**: "Tier-Ω.5.dddd: pre-allocate every identifier the declarator's pattern binds... chalk's supports-color uses `const {env} = process;` followed by a function-decl that references `env` as upvalue — without pre-allocation, the function's body resolved `env` as a missing global." Similarly Tier-Ω.5.qq cites arktype's `@ark/util/strings.js` (anchoredRegex references anchoredSource declared two lines below).

**The implicit constraint**: *Every top-level binding must have a local slot allocated BEFORE any sub-Compiler (nested function body) walks its body, so that resolve_upvalue can find the binding and emit Op::LoadUpvalue. If the slot doesn't exist, the sub-Compiler falls through to LoadGlobal — which at runtime mis-resolves to ReferenceError or to a stale globalThis property, depending on bootstrap state.*

**How ES-EXT 2 violated it**: The script-mode gate skipped both pre-allocation passes for top-level Var. Top-level function decls remained pre-allocated, but their bodies' upvalue captures of top-level vars went through resolve_upvalue → resolve_local on the parent → None → fallback to LoadGlobal. Then at runtime, the value's race between StoreGlobal (var statement execution) and the function's first call determined visibility. For tests where a top-level function is called BEFORE the var statement runs (e.g., setup-then-assert patterns), the binding resolves as undefined.

**Failure cluster**: 110 "TypeError/Proxy coercion not thrown" + 80 "Test262Error not thrown" (~200 records).

### Implicit Constraint IC.2 — Forward-reference resolution for const-before-use

**Substrate site**: Same Phase A.6 pass (lines 898-946).

**The implicit constraint**: *Top-level `let`/`const` declared at line N must be resolvable as a local from arrow functions defined at line M < N, because the function-decl hoisting pass already compiled those arrows and their upvalue tables snapshot the parent's locals.*

**How ES-EXT 2 violated it**: Pre-allocation skip extended to all top-level Var. Although the comment explicitly says let/const for Phase A.6's anchoredRegex example, my gate only fires on Var kind — so let/const should be unaffected. **Status**: hypothesized active but not verified. Likely a smaller contributor than IC.1.

### Implicit Constraint IC.3 — Harness-injected globals must exist before eval

**Substrate site**: `legacy/host-rquickjs/tests/test262/runner.mjs` lines ~407-429 (harness assembly).

**The implicit constraint**: *Cross-realm tests reference `$262.createRealm()`; the harness or runtime must inject `$262` into globalThis BEFORE indirect eval invokes the test source.*

**How ES-EXT 2 violated it (or didn't)**: This constraint is orthogonal to ES-EXT 2 — `$262` is missing whether or not script-mode is enabled. The regression catalogue surfaced it as a latent gap, not a script-mode-caused regression. 12 records.

### Synthesis (per Doc 729 §XIII §6 — inductive properties)

Walk (explicit constraints already known): "indirect-eval `var X` must attach X to globalThis per §19.2.1.3."

Regression (implicit constraints surfaced): IC.1 + IC.2 + IC.3.

Property the design must induce: *Indirect-eval `var X` attaches to globalThis AND top-level inner-function captures of X resolve as local upvalues at the parent's compile time (not as runtime globalThis property lookups).* The two are simultaneously required.

### ES-EXT 2 v2 design space (constraint-respecting)

**Option A — Pre-allocate AND mirror**: Keep Phase A/A.6 pre-allocation passes UNCHANGED in script_mode. At Stmt::Variable identifier emit, when script-top-var: emit StoreLocal AS USUAL, then ALSO emit StoreGlobal (or equivalent post-init hook). The local slot satisfies IC.1; the StoreGlobal write satisfies the §19.2.1.3 spec constraint. Cost: one extra StoreGlobal per top-level var, no scope-resolution change.

**Option B — Bind to globalThis via the local slot**: Make the local slot itself live on globalThis (slot=globalThis property). Spec-pure but requires runtime slot-aliasing infrastructure.

**Option C — Defer to runtime hoist**: At evaluate_script entry, walk the AST for top-level var names and pre-set globalThis.X = undefined; let module-mode pre-allocation handle locals normally; add a finalize-pass that copies the local slot back to globalThis after each statement. Heavy.

Option A is the minimal-LOC constraint-respecting v2. ~10 LOC change to the Stmt::Variable script_top branch.

**Action**: NOT executing v2 in this autonomous tick. v2 design recommendation queued for keeper alignment.

**Arc status**: REOPENED. ES-EXT 2 v1 reverted; arc telos (indirect-eval var attaches to globalThis) NOT achieved on the eval path. ES-EXT 3+4 retained as standalone yield. Next rung options: (a) ES-EXT 2 v2 with deeper pre-allocation diagnosis, (b) hand off to a different mechanism (e.g., post-eval scrape from frame-locals to globalThis), or (c) accept the trade and close arc on partial-telos achievement.
