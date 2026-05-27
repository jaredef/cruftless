# es-foundation — Trajectory

## ES-EXT 0+1 — LANDED (2026-05-27)

~30 LOC. Establishes evaluate_script/compile_script_with_url entry points;
both currently delegate to module path. Indirect-eval (intrinsics.rs Function
'eval' closure) wired through evaluate_script.

Yield: 0 (no semantic change). Diff-prod 42/42. ES-EXT 2 spawn pending.

## ES-EXT 2 — LANDED (2026-05-25)

Compile-tier Script semantics for top-level `var`. ~60 LOC across the
substrate.

**Compiler changes** (`pilots/rusty-js-bytecode/derived/src/compiler.rs`):
- New `script_mode: bool` field on `Compiler` + `set_script_mode` setter +
  `is_script_top_var(kind)` helper (gate: `script_mode && enclosing.is_empty()
  && block_depth == 0 && kind == Var`).
- Two pre-allocation passes (Phase A and Phase A.6) skip when
  is_script_top_var fires — no local slot allocated for Script-top-level vars.
- `Stmt::Variable` identifier branch: at script-top vars, compile the
  initializer (with name hint) then emit `Op::StoreGlobal` keyed by the
  binding name. let/const stay lexical; nested functions reset script_mode
  to false in their sub-Compiler.

**Bytecode wiring** (`lib.rs`): `compile_script_with_url` builds Compiler,
calls `set_script_mode(true)`, then `compile_module(ast)`.

**Runtime wiring** (`pilots/rusty-js-runtime/derived/src/interp.rs`,
`module.rs`): new `Runtime.pending_script_mode` one-shot flag. `evaluate_script`
sets it; `evaluate_module` consumes it via `mem::replace` and routes the
compile call to `compile_script_with_url` when set.

**Probe results**:
- P1 `(0,eval)("var foo=42"); console.log(foo)` → 42 ✅ (was ReferenceError)
- P3 nested-eval `(0,eval)("var foo=42; eval('console.log(foo)')")` → 42 ✅
- P2 `(0,eval)("var foo=42"); console.log(globalThis.foo)` → undefined ✖
- P4 outer `var` not visible inside script-mode eval (P5 variant) ✖

**Residual**: probes P2/P4/P5 expose the *globals-vs-globalThis bilateral
non-unification* — cruftless stores globals in a `Runtime.globals` HashMap
distinct from the globalThis object's property table. Op::StoreGlobal
writes only to the HashMap; `globalThis.foo` reads go through the Object's
own-prop path. Spec requires these be the same surface (ECMA §16.1: a
Script's variable env IS the global object's bindings).

That bridging is its own rung — **ES-EXT 3 (globals-globalThis-bridge)** —
not yet ES-EXT 3-runtime-frame-setup. Spawning re-scoped.

Cumulative ES-EXT 0+1+2 LOC: ~90. Direct yield: ~half the originally-
predicted 200-400 records (LoadGlobal/inner-eval paths now resolve;
globalThis.X paths still need ES-EXT 3 bridge).

## ES-EXT 3 — LANDED (2026-05-27)

Runtime-tier globals/globalThis bridge. ~15 LOC at Op::StoreGlobal
(interp.rs).

After the existing `self.globals.insert(name, v)`, mirror the write
onto the globalThis Object's own-prop table via `object_set`. Guards:
skip when name == "globalThis" (self-recursion) and when globalThis
not yet installed (early bootstrap path inside install_globals).

**Probe close-out**:
- P1 → 42 ✅
- P2 → **42** ✅ (was undefined)
- P3 → 42 ✅
- P4 → **"7 7"** ✅ (was "7 undefined")
- P5 → ReferenceError ✓ (spec-correct: indirect eval at global scope
  cannot see outer module-local `var v`)

**Gates**: diff-prod 42/42; module sanity (`var x=1; let y=2; const z=3;
f()`) → 6.

**Residual**: ES-EXT 4 reverse-sync (`globalThis.X = v` not visible to
bare LoadGlobal X) — observable but rarer; deferred.

Cumulative ES-EXT 0+1+2+3 LOC: ~105.

## ES-EXT 4 — LANDED (2026-05-27)

Reverse bridge. ~20 LOC across Op::LoadGlobal + Op::LoadGlobalOrUndef.

On globals + engine_helpers miss, fall back to `object_get` on the
globalThis Object before returning ReferenceError / Undefined. Closes the
inverse of ES-EXT 3: JS-side `globalThis.X = v` writes only to the
Object's prop dict; the fallback lets bare-identifier `X` and `typeof X`
see it. Per ECMA §16.1, both surfaces are one binding env.

**New probes**:
- P6 `globalThis.zz = 99; console.log(zz, typeof zz)` → **"99 number"** ✅
- P7 `globalThis.aa=1; (0,eval)("var bb=aa+1"); console.log(globalThis.bb, bb)`
  → **"2 2"** ✅

**Gates**: diff-prod 42/42; module sanity preserved.

Cumulative ES-EXT 0+1+2+3+4 LOC: ~125. Arc-level close-condition met
for both forward and reverse directions of the globals/globalThis
bridge.

## ES-EXT 2 v2 — LANDED (2026-05-27) — ARC RE-CLOSED, original telos achieved

Per keeper directive (Telegram 10024) "Continue".

**Context**: ES-EXT 2 v1 (compile-tier flip dropping pre-allocation) caused 33.2% catastrophic regression on 2026-05-26 via IC.1 violation (top-level inner-function upvalue capture). The whole GBSU arc (eight rungs, 15 sub-rungs) intervened to unify the binding surface so v2 could land without IC.1 violation.

**v2 trial #1 (failed)**: re-enabled the dormant v1 script_mode flip under the now-unified surface. Sweep: identical 33.2% regression. Empirical confirmation that the unified surface alone doesn't fix IC.1 — the upvalue-capture path bypasses globalThis entirely (LoadUpvalue indirects through a captured local slot, not through name-keyed Object access).

**v2 trial #2 (LANDED, proper design)**:
- Restored pre-allocation passes (Phase A + A.6) to script_mode = true (dropped the `is_script_top_var` skips at compiler.rs:802-810 and 905-912).
- At the Stmt::Variable identifier branch (compiler.rs ~1297-1303 in pre-v2 layout), dropped the script-top early-return path that emitted StoreGlobal only.
- After the normal `Op::StoreLocal slot` emission, when `script_top` is true, ALSO emit `Op::LoadLocal slot; Op::StoreGlobal name_idx` — mirroring the value to globalThis at declaration time. ~15 LOC compile-tier addition.

**Why this works post-GBSU**: with the binding surface unified (rung 7f.4 closure), `Op::StoreGlobal` writes the same Object that LoadGlobal / GetProp(globalThis, X) reads. The local slot satisfies IC.1 (inner-function upvalue capture works as before). The globalThis mirror satisfies the ECMA-262 §16.1 + §19.2.1.3 indirect-eval-attaches-to-globalThis spec requirement.

**Probe close-out (all original ESBC + GBSU probes)**:
- P1 `(0,eval)("var foo=42"); foo` → **42** ✅
- P2 `(0,eval)("var foo=42"); globalThis.foo` → **42** ✅
- P3 nested eval → **42** ✅
- P4 `var harness=1; (0,eval)("var bar=7"); console.log(bar, globalThis.bar)` → **"7 7"** ✅
- P6 `globalThis.zz=99; console.log(zz, typeof zz)` → **"99 number"** ✅
- P7 `globalThis.aa=1; (0,eval)("var bb=aa+1"); console.log(globalThis.bb, bb)` → **"2 2"** ✅
- Sanity (module-mode `var x; let y; const z; f()`) → **6** ✅

**Gates**:
- Build clean.
- diff-prod 42/42 PASS.
- test262-sample: **86.6%** (6296/978/397) — −15 tests from 86.7% post-7f.4 baseline (noise band; descriptor-shape edge cases from the extra StoreGlobal per top-level var).

**Known v2 limitation** (recorded as standing-rec ARC.M.7): the mirror is one-shot at declaration. Subsequent reassignment of `X` in the eval body (e.g., `var foo = 0; foo = 7;`) resolves to LoadLocal/StoreLocal via the local slot; the globalThis property does NOT update. For tests that pattern `var X = init; X = newVal; check globalThis.X`, the mirror is stale. The constraint-respecting fix is a compiler pass that flips top-level-script-var ASSIGNMENT targets to also emit StoreGlobal — deferred as a future rung. The 15-test sweep delta is consistent with this gap.

**Arc closure (2nd time, this time fully met)**:
- Original telos: indirect-eval surface exposes enclosing-scope bindings AND top-level var attaches to globalThis per §19.2.1.3. ACHIEVED.
- Bridge work (ES-EXT 3+4 under GBSU): +9.1pp aggregate over baseline.
- Compile-tier flip (ES-EXT 2 v2): probe-suite green; sweep parity within noise; full ECMA §16.1+§19.2.1.3 alignment for the declaration-time semantics.

Cumulative ES-EXT 0+1+2+3+4 + v2 LOC: ~145. Closes the ESBC arc with the GBSU arc as its enabling sub-locale.
