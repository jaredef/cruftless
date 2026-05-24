# ts-resolve-type-only-imports — Resume Vector / Seed

**Locale tag**: `L.ts-resolve-type-only-imports` (top-level)

**Status as of 2026-05-24**: **CHAPTER CLOSED at TROI-EXT 1 (1 implementation round; standing rule 13 twelfth corroboration; LARGEST-YIELD substrate fix of the parity arc)**. Execute-parity **60.7% → 69.0%** (+8.3 pp). CRUFT_FAIL count: 39 → 8 (31 files unblocked by ONE 110-LOC post-strip pass). Pred-troi.3 + Pred-troi.4 HELD STRONGLY — failure-table top tag shifted from `prototype-undefined` (30 files, ESM cycle) to small unrelated singletons. **Finding TROI.2**: the rxjs ESM-cycle issue was diagnosed as a runtime-substrate concern but the actual fix was at the resolver tier (TSR elision). Substantive refinement of Doc 729 — the resolver-instance pattern is even more powerful than originally framed.

**Historical status (founding)**: WORKSTREAM FOUNDED (TROI-EXT 0). Third downstream sub-locale from TXC's failure-table. Spawned per keeper directive "A" at TRE close. Addresses the 30-file ESM-cycle cluster (77% of remaining CRUFT_FAILs after TRMLE + TRE).

**Workstream**: detect imports whose imported bindings are used ONLY in stripped-out type positions, and strip those imports. Mirrors `tsc --verbatimModuleSyntax` behavior: type-only imports don't materialize at runtime, eliminating ESM cycles whose load chain is purely-typological.

**Key insight (pivot from initially-proposed runtime-ESM-cycle locale)**: the 30-file failure cluster's ROOT CAUSE is rxjs's `types.ts` importing `Observable` and `Subscription` for type-position use only. tsc's emit elides these imports entirely. If TSR detects this and elides them too, the runtime ESM cycle never forms — no substrate-runtime change needed.

**Author**: 2026-05-24 session.
**Parent**: none (top-level).
**Siblings**: TSR, TCC, TXC, TRMLE, TRE.
**Composes with**:
- [TXC TRE-close failure-table](../ts-execute-corpus/results/2026-05-24/divergence-table.md) — row 1 (30 prototype-undefined) traces here
- [Finding IX.5 + TXC.2](../rusty-js-jit/findings.md) — Doc 729 refinement: TSR's responsibility extends to elision of runtime side-effects from purely-type constructs
- [TSR strip.rs](../ts-resolve/derived/src/strip.rs) — substrate to modify
- [Standing rule 14 + 15](../rusty-js-jit/findings.md) — conservative-strip + chapter-close-inspect disciplines

## I. Telos

**Empirical answer to**: detecting unused (type-only) imports in TSR's post-strip pass eliminates the ESM cycle in rxjs's types.ts, lifting execute-parity ≥6 pp (≥22 files) toward the prototype-undefined cluster's 30-file scope.

### I.1 First-cut scope

Post-strip pass approach:
1. Run the existing strip-and-erase pipeline to produce stripped source.
2. For each `import { ... } from '...'` declaration in the source, examine the IMPORTED-NAMES list.
3. For each imported name, count occurrences in the STRIPPED OUTPUT (the source bytes that survived the strip).
4. If NO imported name from a single import statement appears in the stripped output, strip the entire import statement.
5. Partial-strip case (some names used, some not): out of MVP scope; full statement remains.

This is identical in spirit to `tsc --verbatimModuleSyntax`'s "elide imports never used as values" rule.

### I.2 Constraints

```
C1. Existing 51/51 ts-resolve unit tests pass.
C2. TCC parse-parity 98.1% does not regress.
C3. TXC execute-parity 60.7% does not regress.
C4. diff-prod 42/42 PASS.
C5. Conservative-strip per rule 14: if usage detection is uncertain
    (e.g., dynamic imports, computed property access via name string),
    BAIL on the elision — keep the import.
C6. Post-strip pass runs AFTER all strip rules; reads the stripped
    output to count name occurrences; emits ADDITIONAL strip ranges
    for the import statements; re-runs the byte-space-fill.
```

### I.3 Falsifiers

**Pred-troi.1**: ≤80 LOC for the post-strip pass.
**Pred-troi.2**: TSR + TCC + diff-prod + cruft suites all pass.
**Pred-troi.3 (LOAD-BEARING)**: TXC execute-parity ≥66.7% (+6 pp lift; covers the rxjs ESM cycle which is ~22 of the 30 prototype-undefined files).
**Pred-troi.4 (DOC 729 EXTENSION)**: post-fix, the failure-table top tag SHIFTS away from prototype-undefined. If it stays, the cycle has additional causes beyond type-only imports.
**Pred-troi.5 (DISCIPLINE)**: closes in ≤3 implementation rounds. Twelfth prospective application of standing rule 13.

## II. Apparatus + Methodology

- Edit `pilots/ts-resolve/derived/src/strip.rs::Scanner::run` to add a post-strip pass after merge-strips.
- Re-measure TCC + TXC.

1. **TROI-EXT 0** — workstream founding.
2. **TROI-EXT 1** — implementation + re-measure + chapter close. Single-round target.

## III. Carve-outs

- Partial-strip (mix of used + unused names) deferred. MVP only strips when ALL names from one import are unused.
- Dynamic `import()` not analyzed.
- Side-effect-only imports (`import 'side-effect'`) preserved unconditionally.

## IV. Standing artefacts

- `pilots/ts-resolve-type-only-imports/seed.md`, `trajectory.md`
- Edit at `pilots/ts-resolve/derived/src/strip.rs`
- Tests at `pilots/ts-resolve/derived/tests/strip.rs`

## V. Resume protocol

Read seed + trajectory tail. The post-strip pass is mechanical (count name occurrences in stripped output → add strip ranges for unused-import statements). Re-measure TXC; expect ≥6 pp lift toward closing the ESM-cycle cluster.
