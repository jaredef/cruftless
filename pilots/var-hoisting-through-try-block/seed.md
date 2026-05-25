# var-hoisting-through-try-block — Resume Vector / Seed

**Locale tag**: `L.var-hoisting-through-try-block` (top-level)

**Status as of 2026-05-25**: **CLOSED at VHTB-EXT 1** (1 implementation round).

**Workstream**: spec-correct §15.2.10 VarScopedDeclarations — `var` declarations hoist to the enclosing function/script scope across every non-function syntactic boundary (try/catch/finally, if, for, for-in, for-of, while, do-while, switch, block, labelled).

**Trigger**: surfaced by REOU-EXT 1 via 2 regressed for-in tests (`S12.6.4_A{1,2}.js`). Pre-REOU the silent-undef path masked the bug; REOU's spec-correct ReferenceError throw made it visible. Corpus IS the regression instrument (Finding IX.1 again).

**Author**: 2026-05-25 session.
**Parent**: none (top-level).
**Composes with**:
- ECMA-262 §15.2.10 VarScopedDeclarations
- [REOU trajectory](../reference-error-on-unresolvable/trajectory.md) Finding REOU.2 — the surfacing event

## I. Telos

`var` declarations nested inside try/if/for/while/switch/etc. must be pre-allocated at the enclosing function/script scope so reads after the construct correctly resolve to the hoisted binding (Undefined if never assigned).

## II. Apparatus + Methodology

Edit: `pilots/rusty-js-bytecode/derived/src/compiler.rs`. Add `collect_hoisted_var_names()` recursive walker that descends every non-function statement. Call it during Phase A.6 to pre-allocate locals for all nested `var` declarators across the module body.

## III. Carve-outs

- `let`/`const` are block-scoped per §13.3.1.1 LexicallyScopedDeclarations — NOT hoisted.
- FunctionDecl and ClassDecl bodies start fresh hoisting scopes — walker does not recurse into them.
- For-binding head: only `for (var x ...)` hoists `x`; `for (let/const x ...)` is block-scoped.

## IV. Verification

Minimal repro:
```js
try { for(__key in undefined){ var key=__key; } } catch(e){}
console.log(typeof key);  // expected: "undefined" (hoisted, never assigned)
```
Pre-fix: ReferenceError. Post-fix: "undefined" ✓.

test262 fixture `S12.6.4_A1.js` PASS post-fix.

## V. Resume protocol

Read seed. The fix is one recursive walker in compiler.rs at the module-level hoisting phase. Per-statement recurrence is straightforward; FunctionDecl/ClassDecl are the carve-outs.
