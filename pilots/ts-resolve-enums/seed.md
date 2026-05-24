# ts-resolve-enums — Resume Vector / Seed

**Locale tag**: `L.ts-resolve-enums` (top-level)

**Status as of 2026-05-24**: **CHAPTER CLOSED at TRE-EXT 1 (1 implementation round; standing rule 13 eleventh corroboration)**. Execute-parity 52.7% → **58.6%** (+5.9 pp). Two fixes landed: enum-MVP-strip + `from`/`as`/`of`/`in`/`instanceof`/`do` removed from `is_overload_blocked_name`. The planned-scope (enum-strip) was 0 pp alone; the load-bearing yield came from the mid-round discovery of the function-overload-keyword-collision bug — seventh reproduction of Finding IX.3 (inspect-then-iterate compound discovery).

**Historical status (founding)**: WORKSTREAM FOUNDED (TRE-EXT 0). Second downstream sub-locale from TXC's post-TRMLE failure-table (row 2: 22 files of `expected LBrace` traced to `enum X { ... }` declarations).

**Workstream**: handle TypeScript `enum` declarations in TSR's strip pipeline. **Two-phase plan**:
- **TRE-EXT 1 (MVP — strip only)**: strip `enum NAME { ... }` declarations entirely (no runtime presence). Parses cleanly; usages of enum members at runtime become `undefined`-property reads (handled by JS as `undefined` access on `undefined` → TypeError at the call site, not at the enum decl). Unblocks parse-tier for files containing enum decls; potentially unblocks execute-tier for files that DECLARE but DON'T USE enums at runtime.
- **TRE-EXT 2 (full lowering — deferred)**: lower `enum X { A, B = 5, C }` to `const X = Object.freeze({ A: 0, B: 5, C: 6, 0: "A", 5: "B", 6: "C" })` per TS reverse-mapping semantics. Restores runtime semantics; bigger LOC + more careful + needs string-vs-number-enum distinction.

This locale's MVP scope is TRE-EXT 1 only. TRE-EXT 2 spawns as a follow-on (or new locale `ts-resolve-enum-lowering/`) after MVP empirical results land.

**Author**: 2026-05-24 session.
**Parent**: none (top-level).
**Siblings**: TSR locales, TXC instrument, TRMLE.
**Composes with**:
- [TXC TRMLE-EXT 1 trajectory](../ts-resolve-module-loader-extension/trajectory.md) — failure-table row 2 surfaces enum
- [Finding IX.5 + IX.6](../rusty-js-jit/findings.md) — multi-definition parity; enum-MVP-strip tests whether **parse parity alone is sufficient for execute parity at files that don't use enum at runtime**

## I. Telos

**Empirical answer to**: how much of the post-TRMLE 47.3% execute-parity gap is attributable to files that contain enum DECLARATIONS but don't USE enums at runtime (where MVP-strip suffices) vs files that USE enums at runtime (where full lowering is needed)?

The bench-anchored target: TXC re-measure shows execute-parity lifts by ≥4 pp (≥15 files) from MVP-strip alone. If the lift is small (<4 pp), most enum-affected files actually use enums at runtime, and TRE-EXT 2 (lowering) is the load-bearing follow-on.

### I.1 First-cut scope (TRE-EXT 1 MVP)

- Add `enum` to the strip-statement rules. Pattern: `enum NAME { ... }` at statement-start; strip from `enum` through the matching closing `}`.
- Handle leading modifiers: `export enum X { ... }`, `export default enum X { ... }`, `declare enum X { ... }`, `const enum X { ... }`, `export const enum X { ... }`. Strip from the leading keyword through the closing brace.

### I.2 Constraints

```
C1. TCC parse-parity does NOT regress (96.5% holds or rises).
C2. TXC execute-parity does NOT regress (52.7% holds or rises).
C3. diff-prod 42/42 PASS.
C4. ts-resolve test suite passes + add regression tests for
    enum / const-enum / export-enum.
C5. Per rule 14 conservative-strip: bail to no-strip if the
    matching close-brace isn't found (degenerate sources).
```

### I.3 Falsifiers

**Pred-tre.1**: ≤40 LOC for the MVP rule.
**Pred-tre.2**: TSR + TCC + diff-prod + cruft test suites all pass.
**Pred-tre.3**: TXC execute-parity ≥56.7% (+4 pp lift). Below means runtime-usage of enums dominates and full lowering is needed.
**Pred-tre.4**: parse-parity ≥96.5% (no regression; may rise).
**Pred-tre.5 (DISCIPLINE)**: closes in ≤3 implementation rounds. Eleventh prospective application of standing rule 13.

## II. Apparatus + Methodology

- Edit: `pilots/ts-resolve/derived/src/strip.rs` — add enum strip in the Ident step rule branch alongside `interface` / `type`.
- Regression: add 4 tests (plain, const, export, declare enum).
- Re-measure: TCC + TXC.

1. **TRE-EXT 0** — workstream founding (this seed + trajectory + manifest refresh).
2. **TRE-EXT 1** — MVP enum-strip + re-measure + chapter close. Single-round target.

## III. Carve-outs

- Full enum lowering (with reverse-mapping object) deferred.
- `const enum` semantics in `tsc` (inlining at use site) deferred.
- TS `namespace { ... }` declarations are a separate construct (also runtime-bearing; deferred to `ts-resolve-namespaces/` if TXC failure-table surfaces them).

## IV. Standing artefacts

- `pilots/ts-resolve-enums/seed.md`, `trajectory.md`
- Edit at `pilots/ts-resolve/derived/src/strip.rs`
- Tests at `pilots/ts-resolve/derived/tests/strip.rs`

## V. Resume protocol

Read this seed + trajectory tail. The MVP is small (≤40 LOC). If TRE-EXT 1's TXC re-measure shows ≥4 pp lift, MVP succeeds + the locale closes. If <4 pp, that's the empirical signal to spawn the lowering follow-on for the runtime-usage case.
