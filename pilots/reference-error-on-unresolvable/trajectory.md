# reference-error-on-unresolvable — Trajectory

## REOU-EXT 0 — workstream founding (2026-05-25)

**Trigger**: keeper directive "A" at PPA-EXT 1 close. T262C matrix post-PPA-EXT-1 cluster #2 (39 tests, for-of dstr ReferenceError on unresolvable in const/var/let-pattern + default-init) traces to a fundamental runtime-tier violation: cruft's Op::LoadGlobal silently returns Undefined on miss, where ECMA-262 §6.2.4.5 GetValue + §9.1.1.4.4 GetBindingValue require ReferenceError throw.

**Blast-radius pre-fix recon**: affects every identifier read (`var y = undefRef;` should throw, currently silent). typeof and delete have spec-special silent paths (§13.5.3, §13.5.1.2) that must be preserved.

**Doc 740 multi-tier identification**: R = {opcode-split, runtime-handler, compiler-typeof, compiler-delete}. Per FODAS lesson + PPA-EXT 1 cascade pattern, all four close together as one commit.

**Pre-spawn Pred-reou.* (combined with EXT 1 close)**:
- Pred-reou.1: bare-ident read throws ReferenceError
- Pred-reou.2: `typeof undefRef` returns "undefined" (no throw)
- Pred-reou.3: `delete undefRef` returns true (no throw)
- Pred-reou.4: dstr default-init unresolvable throws ReferenceError
- Pred-reou.5: test262-sample net positive with zero PASS→FAIL regressions
- Pred-reou.6 (Rule 13 discipline): closes in 1 implementation round

## REOU-EXT 1 — multi-tier closure (2026-05-25)

**Edits** (~40 LOC total):
1. `op.rs`: add `Op::LoadGlobalOrUndef = 0xFF` + operand-size + dispatch table entries.
2. `interp.rs`: Op::LoadGlobal handler now returns `Err(ReferenceError("{name} is not defined"))` on miss. New Op::LoadGlobalOrUndef handler mirrors the prior silent-undef behavior.
3. `compiler.rs`: at `Unary { op: Typeof|Delete, argument: Identifier }` site, if argument doesn't resolve to local/upvalue, emit `Op::LoadGlobalOrUndef` + the typeof/delete op directly (bypassing the default Identifier compilation path which would emit Op::LoadGlobal).

**Minimal repros** (all GREEN):
- `var y = undefRef` → ReferenceError ✓
- `typeof undefRef` → `"undefined"` ✓
- `delete undefRef` → `true` ✓
- `for (const [x = undefRef] of [[]]) {}` → ReferenceError ✓

**test262-sample** (results booked at chapter close).

**test262-sample** (vs pre-REOU baseline 5719):
- PASS: 5719 → **5787** (+68)
- FAIL: 1573 → 1506 (−67)
- FAIL→PASS transitions: **69**
- PASS→FAIL regressions: **2** (`language/statements/for-in/S12.6.4_A{1,2}.js`)
- Runnable pass rate: 78.4% → **79.4%**

**Regressions diagnosed**: both regressed tests share the pattern `try { for(__key in undefined){ var key=__key; } } catch(){}` followed by a bare-ident read `key`. Minimal repro confirms: `var key` inside a try-block does NOT get hoisted to script/function scope in cruft. Pre-REOU the silent-undef masked it; REOU's spec-correct throw surfaces the real substrate bug. Per Doc 740 §IV.2 this is a substrate-introduction signature: REOU is correct for its target; the surfaced gap is a sibling-locale candidate (`var-hoisting-through-try-block`).

**Five Pred-reou.* dispositions**:
| Predicate | Disposition |
|---|---|
| Pred-reou.1 (bare-ident read throws) | ✅ HELD |
| Pred-reou.2 (typeof returns "undefined") | ✅ HELD |
| Pred-reou.3 (delete returns true) | ✅ HELD |
| Pred-reou.4 (dstr default unresolvable throws) | ✅ HELD |
| Pred-reou.5 (net positive, zero regressions) | ⚠ +68 net but 2 regressions (substrate-introduction signature; pre-existing bug surfaced) |
| Pred-reou.6 (Rule 13 ≤1 round) | ✅ HELD (1 round) |

### Findings

**Finding REOU.1**: Doc 740 multi-tier closure across {opcode-split, runtime-handler, compiler-typeof, compiler-delete} delivered as predicted. +68 PASS net at full cascade scope; pattern continues PPA-EXT 1 result of "stricter substrate exposes pre-existing latent bugs."

**Finding REOU.2**: var-hoisting-through-try-block bug was masked by silent-undef for >1 year of substrate evolution. REOU surfaces it via 2 well-defined test failures — the corpus IS the regression instrument (Finding IX.1 corroborated yet again). Sibling locale `var-hoisting-through-try-block` queued.

**Status**: REOU-EXT 1 CLOSED.
