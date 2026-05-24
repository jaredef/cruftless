# ts-resolve-module-loader-extension — Resume Vector / Seed

**Locale tag**: `L.ts-resolve-module-loader-extension` (top-level)

**Status as of 2026-05-24**: **CHAPTER CLOSED at TRMLE-EXT 1 (1 implementation round; standing rule 13 tenth corroboration)**. Execute-parity lifted **5.1% → 52.7% (+47.6 pp)** — largest single-fix yield observed all session. Pred-trmle.3 HELD STRONGLY (target ≥40%); Pred-trmle.4 HELD STRONGLY (failure-table shifted from parse-errors to runtime-semantics). **Finding IX.5 Doc 729 refinement EMPIRICALLY CONFIRMED** via three independent data points.

**Historical status (founding)**: WORKSTREAM FOUNDED (TRMLE-EXT 0). First downstream sub-locale from TXC's failure-table. Spawned per keeper directive at TXC chapter close. Addresses Finding TXC.3 (load-bearing single substrate fix unblocks ~64% of the execute-parity gap) AND empirically tests Finding IX.5 (Doc 729 refinement: downstream tiers must dispatch through input resolver).

**Workstream**: extend cruft's runtime module loader (in `pilots/rusty-js-runtime/derived/src/module.rs`) to:
1. **Try `.ts`/`.mts`/`.cts` extensions** during import resolution (before/after the existing `.js`/`.mjs` extension probes; TS-convention order: `.ts` before `.js`)
2. **Apply `ts_resolve::strip_ts`** when loading files with `.ts`/`.mts`/`.cts` extensions (so the strip runs at module-import time, not only at top-level CLI invocation)

Per Finding IX.5, this lifts the resolver-instance dispatch from "top-level only" to "every input boundary in the pipeline" — empirically tests whether the Doc 729 refinement is sufficient to close the parse/execute parity gap.

**Author**: 2026-05-24 session.
**Parent**: none (top-level; downstream sub-locale of TXC).
**Siblings**: `ts-resolve/`, `ts-consumer-corpus/`, `ts-execute-corpus/`, TRSLS/TRCAPS/TRGC.
**Composes with**:
- [Finding TXC.3](../ts-execute-corpus/trajectory.md) — load-bearing single substrate fix identified
- [Finding IX.5](../rusty-js-jit/findings.md) — Doc 729 resolver-instance pattern refinement candidate; this locale's outcome IS the empirical test
- [TXC measurement instrument](../ts-execute-corpus/) — re-measure after the fix; lift in execute-parity quantifies the fix's impact
- [Module loader at `pilots/rusty-js-runtime/derived/src/module.rs`](../rusty-js-runtime/derived/src/module.rs) — substrate to modify (multiple file-read sites; need TS-aware variants)
- [TSR strip API](../ts-resolve/derived/src/lib.rs) — public `strip_ts(&str) -> Result<(String, Vec<TypeWitness>)>`
- [Standing rule 13 + 15](../rusty-js-jit/findings.md) — discipline anchors

## I. Telos

**Empirical answer to**: does extending cruft's runtime module loader with TS-extension awareness + TS-strip dispatch close the 64% load-bearing portion of the parse/execute parity gap?

The bench-anchored target: TXC re-measure shows execute-parity lifts from 5.1% to ≥40% (90.1 pp baseline gap × 64% load-bearing portion = 57.7 pp expected lift → 5.1% + 57.7% ≈ 63%; conservative target is ≥40% to allow for unanticipated downstream issues that may emerge once the upstream block is removed).

### I.1 First-cut scope

Per standing rule 13 + Doc 740 §IV.2: design from the deeper-layer first. The deeper layer here is BOTH (a) extension-aware resolution AND (b) strip-at-load. Skip the "just add extension order" intermediate; do both in one round.

- **Add `ts-resolve` dependency** to `pilots/rusty-js-runtime/derived/Cargo.toml`
- **Extend the import-resolution extension search list** in `module.rs` to include `.ts`, `.mts`, `.cts` (TS-convention: `.ts` BEFORE `.js`)
- **At each file-load site** (per the 5 sites found by grep), detect the `.ts`/`.mts`/`.cts` extension and apply `ts_resolve::strip_ts` to the source string before handing to parse
- **Preserve existing behavior** for `.js`/`.mjs`/`.cjs` (no strip; same as today)

### I.2 Constraints (Pin-Art enumeration)

```
C1. Existing 24/24 ts-resolve tests + TSR sub-suites all pass.
C2. Existing TCC parse-parity 95.2% does NOT regress (the fix is
    additive to import resolution; top-level parse path is
    untouched).
C3. Existing diff-prod 42/42 PASS (the fix is at the module loader,
    not the JS parser; .js fixtures unchanged).
C4. TXC execute-parity lifts to >= 40% (Pred-trmle.3).
C5. No new circular dependency: rusty-js-runtime depending on
    ts-resolve is safe because ts-resolve only depends on rusty-js-
    ast + rusty-js-parser, both of which rusty-js-runtime already
    depends on transitively.
C6. Per Finding IX.5: dispatch is added at every file-load site, not
    just the top-level CLI entry. Per IX.6: this is the empirical
    test of the Doc 729 refinement.
C7. Conservative-strip (rule 14): bail on strip error → fall back
    to raw bytes; cruft's existing parser error is the surface
    diagnostic.
```

### I.3 Falsifiers

**Pred-trmle.1**: total LOC delta ≤80 (the fix is small: ~5 grep'd file-read sites + one extension-array addition).

**Pred-trmle.2**: TSR + TCC + diff-prod + canonical fuzz all unchanged post-fix.

**Pred-trmle.3 (LOAD-BEARING)**: TXC execute-parity lifts from 5.1% to ≥40%. Below 40% means either:
- The substrate fix didn't address what the data showed it should, OR
- Other parity gaps are larger than estimated (runtime-bearing constructs dominate more than expected).

**Pred-trmle.4 (DOC 729 REFINEMENT TEST)**: after the fix, the CRUFT_FAIL row distribution in TXC's failure table SHIFTS — the top categories (parse-error, module-not-found) drop dramatically and runtime-bearing-construct categories (enum, decorator, etc.) become the new top. This directly tests Finding IX.5: if the gap was the integration point, fixing it should reveal the underlying runtime-tier coverage gaps.

**Pred-trmle.5 (DISCIPLINE FALSIFIER per standing rule 13)**: closes in ≤3 implementation rounds. Tenth prospective application of the rule.

## II. Apparatus

- **Substrate edit**: `pilots/rusty-js-runtime/derived/src/module.rs`
- **Cargo dep**: `pilots/rusty-js-runtime/derived/Cargo.toml`
- **Re-measure**: `cargo run --release -p ts-execute-corpus --bin txc-measure`
- **Regression instrument** (rule 14): TCC + diff-prod + cruft test suite all re-run after the fix

## III. Methodology

1. **TRMLE-EXT 0** — workstream founding (this seed + trajectory + manifest refresh + findings note).
2. **TRMLE-EXT 1** — implementation + regression + TXC re-measure + chapter close. Single-round target per standing rule 13.

## IV. Carve-outs

- No new TS feature support at this locale; pure plumbing.
- No CommonJS handling change (`.cjs` only added to extension list; CJS itself is unaltered).
- No package.json `"type": "module"` reading-change; existing behavior preserved.

## V. Standing artefacts

- `pilots/ts-resolve-module-loader-extension/seed.md`, `trajectory.md`
- `pilots/rusty-js-runtime/derived/Cargo.toml` (dep addition)
- `pilots/rusty-js-runtime/derived/src/module.rs` (per-site edits)

## VI. Resume protocol

Read this seed, then trajectory.md tail. Read TXC trajectory's TXC-EXT 1 closing section for the 64% load-bearing finding. Read Addendum X §IX.5 for the Doc 729 refinement candidate this locale empirically tests. The substrate edit is bounded; the empirical outcome is the load-bearing data point for whether the Doc 729 refinement is sufficient to close the parity gap.
