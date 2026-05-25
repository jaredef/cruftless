# reference-error-on-unresolvable — Resume Vector / Seed

**Locale tag**: `L.reference-error-on-unresolvable` (top-level)

**Status as of 2026-05-25**: **CLOSED at REOU-EXT 1** (1 implementation round; Doc 740 multi-tier closure across R = {opcode-split, runtime-handler, compiler-typeof, compiler-delete}).

**Workstream**: ECMA-262 §6.2.4.5 GetValue + §9.1.1.4.4 GetBindingValue compliance — unresolvable identifier reads must throw ReferenceError. Cruft's `Op::LoadGlobal` silently returned Undefined on miss, violating the spec across every identifier read path. typeof/delete must take the silent path per §13.5.3 / §13.5.1.2 special-cases.

**Author**: 2026-05-25 session.
**Parent**: none (top-level).
**Siblings**: TCC, TXC, T262C, FODAS, PPA.
**Composes with**:
- ECMA-262 §6.2.4.5 GetValue + §9.1.1.4.4 GetBindingValue (read throws on unresolvable)
- §13.5.3 step 3.b.iii (typeof special-case: returns "undefined", does not throw)
- §13.5.1.2 (delete of unresolvable: returns true in sloppy mode)
- [Doc 740](../../docs/corpus-ref/740-multi-tier-cascade-revival-when-the-hot-path-traverses-multiple-tiers-closing-one-tier-alone-is-insufficient.md) (P4) multi-tier closure
- [Doc 742](../../docs/corpus-ref/742-the-resolver-instance-pattern-at-full-strength-downstream-dispatch-and-upstream-elision-as-doc-729s-empirical-refinements-from-a-typescript-parity-research-arc.md) §V upstream-elision at runtime-tier
- [FODAS trajectory](../for-of-destructuring-assignment-semantics/trajectory.md) Finding FODAS.2 (combined multi-tier closure pattern)
- [PPA-EXT 1 trajectory](../parser-permissiveness-audit/trajectory.md) Finding PPA.1 (eval CompileError→SyntaxError cascade exemplar)

## I. Telos

**Empirical answer to**: does splitting Op::LoadGlobal (throws on miss) from Op::LoadGlobalOrUndef (silent fallback, used only at typeof/delete sites) close the §6.2.4.5 / §9.1.1.4.4 violation cluster without regressing tests that depend on the special-case typeof/delete semantics?

## II. Apparatus + Methodology

R = {opcode-split, runtime-handler, compiler-typeof-site, compiler-delete-site}.

Edits:
1. `pilots/rusty-js-bytecode/derived/src/op.rs`: add Op::LoadGlobalOrUndef = 0xFF.
2. `pilots/rusty-js-runtime/derived/src/interp.rs`: Op::LoadGlobal throws ReferenceError on miss; Op::LoadGlobalOrUndef mirrors prior silent-undef behavior.
3. `pilots/rusty-js-bytecode/derived/src/compiler.rs`: at `Unary { op: Typeof|Delete, argument: Identifier }` site, emit Op::LoadGlobalOrUndef + the typeof/delete op directly. Identifier compilation otherwise unchanged (emits Op::LoadGlobal which now throws).

Verification: minimal repros + test262-sample re-measurement.

## III. Carve-outs

- Local + upvalue identifier reads UNCHANGED.
- StoreGlobal's strict-mode reference-error path already correct (interp.rs:6907).
- typeof on member-access (`typeof obj.prop`) unchanged — only bare-identifier typeof routes through LoadGlobalOrUndef.
- delete on member-access (`delete obj.prop`) unchanged.

## IV. Standing artefacts

- `pilots/reference-error-on-unresolvable/seed.md`, `trajectory.md`
- Edits at op.rs / interp.rs / compiler.rs (~30 LOC combined)

## V. Resume protocol

Read seed + trajectory tail. The fix is one opcode + 4 sites. Verify via the four minimal repros: bare ident read throws / typeof returns "undefined" / delete returns true / dstr default unresolvable throws.
