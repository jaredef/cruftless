# eval-function-arguments-binding-semantics — Seed

## Substrate-pilot — compiler+runtime locale for eval, function declarations, and the arguments object.

Per keeper directive 2026-05-27, promoted from CANDIDATES.md Tier M entry (abe). Mechanism gaps #4 (direct eval lexical capture) and #8 (arguments object shape) from the LPA-EXT 8 diff-prod empirical cross-check. Covers 582 test262 rows (LPA-EXT 5 Arc F, 0% diff-prod pass rate).

## Telos

Close three binding-instantiation mechanisms at the compiler+runtime tier:

1. **Direct eval lexical capture**: `eval("x")` where `x` is an outer `const`/`let` must resolve against the enclosing declarative environment record, not just globals. Currently crashes (exit 70).
2. **Arguments exotic object**: the `arguments` binding in non-arrow functions must be an exotic Arguments object (§10.4.4), not a plain Array. `Array.isArray(arguments)` must return `false`. The exotic object has integer-indexed property access, a `.length` property, and (in sloppy mode) parameter aliasing.
3. **Sloppy-mode parameter aliasing**: in sloppy mode, `arguments[0]` and the first named parameter are aliased — writing to one updates the other. In strict mode, no aliasing.

## Current state

- **Eval**: the compiler's `eval` path compiles the eval source as a fresh module with no access to the caller's lexical scope (`compiler.rs` does not thread the enclosing scope's locals/upvalues into the eval compilation context). Result: `eval("x")` with outer `const x = 42` throws ReferenceError or crashes.
- **Arguments**: `compiler.rs:60–66` allocates `arguments_slot` for non-arrow functions. The runtime fills it with a plain `Value::Object` of `InternalKind::Array` containing the actual args. This means `Array.isArray(arguments)` returns `true`, `arguments` has Array prototype methods, and the exotic behavior (non-enumerable `.callee`, non-configurable `.length` in strict mode) is absent.
- **Aliasing**: not implemented. `arguments[0] = 99` does not update the named parameter in sloppy mode.

## Constraints

- `compiler.rs:60–66` — arguments_slot allocation
- Runtime `call_function` — populates arguments_slot with Array
- Eval compilation path — must thread enclosing scope
- `strict-binding-eval-arguments/` — parser-tier early error rejection (adjacent, not overlapping)
- `non-simple-params-strict-body/` — parser-tier (adjacent)

## Methodology

1. **Rung 1**: Replace the arguments Array with an exotic Arguments object. Create `InternalKind::Arguments` with integer-indexed access, `.length`, `.callee` (sloppy only), and `Symbol.iterator`. `Array.isArray` must return `false`.
2. **Rung 2**: Sloppy-mode parameter aliasing. In the runtime's `call_function`, when the function is non-strict and has simple parameters, bind `arguments[i]` and `params[i]` to shared cells.
3. **Rung 3**: Direct eval lexical capture. Thread the enclosing scope's local/upvalue descriptors into the eval compilation context so `resolve_local` finds outer `let`/`const`/`var` bindings.
4. **Rung 4**: Strict-mode eval scoping. In strict mode, `eval("var x = 1")` must not leak `x` into the enclosing scope.

## Composes-with

- `strict-binding-eval-arguments/` — parser-tier; this locale handles runtime-tier
- `non-simple-params-strict-body/` — parser-tier early error for `"use strict"` + non-simple params
- `directive-prologues` fixture — tests sloppy-mode eval leak and arguments aliasing

## Carve-outs

- `arguments.callee` in strict mode: specified to throw TypeError; deferred to a strict-mode hardening rung
- Annex B arguments object `__defineSetter__` / `__defineGetter__`: deferred to `annexB-runtime-quirks/`

## Resume protocol

Read this seed, then `trajectory.md` tail.

## Diff-prod anchors

| Fixture | Status | Connection |
|---|---|---|
| `eval-lexical-capture` | FAIL (exit 70) | Direct eval cannot resolve outer const/let — crashes |
| `arguments-object` | FAIL | Array.isArray(arguments)=true; arrow inherits 0 instead of outer length; no aliasing |
| `directive-prologues` | FAIL | Sloppy eval var leak and arguments aliasing both fail |
| `comma-grouping-eval` | FAIL | eval completion values partially broken |
