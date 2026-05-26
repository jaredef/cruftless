# ts-execute-corpus — Resume Vector / Seed

**Locale tag**: `L.ts-execute-corpus` (top-level per Doc 737 §IV)

**Status as of 2026-05-24**: **CHAPTER CLOSED at TXC-EXT 1 (1 implementation round; standing rule 13 ninth corroboration)**. Execute-parity baseline = **5.1%** vs parse-parity baseline = 95.2% (**90.1 pp gap**). 64% of the gap traces to ONE substrate gap: cruft's module loader is TS-unaware. Load-bearing next sub-locale: `ts-resolve-module-loader-extension/`.

**Historical status (founding)**: WORKSTREAM FOUNDED (TXC-EXT 0). Spawned per keeper directive at the close of the parse-parity arc (TCC reached 95.2%) with the explicit framing of the **full-parity research arc**. This locale is the second instrument-tier locale, complementing TCC: TCC measures **parse-parity (P)**; TXC measures **execute-parity (E)**, the achievable proxy for **behavioral-erasure parity (B)** which the Doc 729 resolver-instance pattern implicates as the load-bearing definition of "full parity" in cruftless's TS resolver tier.

**Workstream**: build an execute-parity measurement harness that runs each `.ts` corpus file under BOTH `bun` (oracle) and `cruft` (system-under-test) via a synthetic driver, then diffs the observable output. The synthetic-driver MVP: for each file, attempt `import * as M from FILE; console.log(Object.keys(M).sort().join("\\n"))`. Per-file outcome categorized into a fixed taxonomy (MATCH / DIVERGE / BUN_FAIL / CRUFT_FAIL / SETUP_FAIL). The per-package corpus-aggregate baseline becomes the execute-parity research instrument that drives the runtime-bearing-construct sub-locale arc.

**Author**: 2026-05-24 session.
**Parent**: none (top-level).
**Siblings**: `ts-resolve/`, `ts-consumer-corpus/` (parse-parity instrument; this locale's structural twin at the execute tier).
**Composes with**:
- [Doc 729](../../docs/corpus-ref/729-cruftless-a-primary-articulation-of-the-resolver-instance-pattern-as-the-comprehensive-design-toward-which-rusty-bun-morphs.md) — resolver-instance pattern; behavioral-erasure parity (B) is the load-bearing definition for the TS resolver tier
- [apparatus/docs/standing-rule-13-prospective-application.md](../../apparatus/docs/standing-rule-13-prospective-application.md) — sixth prospective application; expected ≤3 implementation rounds
- [TCC seed + trajectory](../ts-consumer-corpus/) — structural precedent; this locale's MVP mirrors TCC's install + harness + table pattern
- [TRGC chapter close (95.2% parse-parity)](../ts-resolve-generics-calls/trajectory.md) — the substrate this locale measures execute-parity over

## I. Telos

**Empirical answer to**: when TSR's strip output is fed to cruft and the resulting program is run, does the observable stdout match what `bun` (which uses its own native TS strip) produces on the same input? Per-file, per-package, per-construct — what does the divergence distribution look like, and which TS features account for which divergences?

The headline number drives the full-parity research arc: a current-state baseline of execute-parity at `?%` (vs parse-parity 95.2%) tells us how much of the "execute-parity gap" is attributable to runtime-bearing TS constructs (enums, ctor-shorthand, decorators, namespaces) vs other erasure-loss patterns.

### I.1 First-cut scope (TXC-EXT 0 founding)

Per standing rule 13 + Doc 740 §IV.2: design from the deeper-layer first. Skip intermediate "just run files and see if they exit 0" steps; design the synthetic-driver pattern + per-file categorization from founding.

- **Oracle**: `bun` (already in repo's bench harness; supports native `.ts` execution via its built-in strip)
- **System-under-test**: `cruft` (our TS resolver pipeline)
- **Driver pattern**: synthetic per-file invocation that exercises the file's exports + side-effects. MVP: `bun -e "import * as M from '<file>'; console.log(Object.keys(M).sort().join('\\n'))"` and equivalent for cruft
- **Per-file outcomes**:
  - `MATCH` — both produce byte-identical stdout
  - `DIVERGE` — both run but stdout differs
  - `BUN_FAIL` — bun couldn't load / run (file might depend on missing deps; not actionable for cruft)
  - `CRUFT_FAIL` — cruft couldn't load / run (the actionable category)
  - `SETUP_FAIL` — driver setup error (filesystem, dependency resolution, etc.)
- **Failure categorization**: similar to TCC's structural-tag heuristic; for DIVERGE rows, examine stderr + first divergent line of stdout

### I.2 Constraints (Pin-Art enumeration)

```
C1. Each per-file invocation runs in isolation; no cross-file state.
C2. Output normalized: strip ANSI codes, normalize whitespace,
    drop volatile content (timestamps, PIDs, random numbers).
C3. Per-file timeout (5s default) to bound runtime cost.
C4. Categorization is structural (matches/diverges/fails) not
    semantic — divergence diagnosis is per-file inspection work.
C5. The harness runs the full corpus in <300s on aarch64 release
    build (5× TCC's budget since each file invokes two interpreters).
C6. Per apparatus/docs/standing-rule-13-prospective-application.md §3:
    sibling-anchor (TCC), shape-compat (same fixtures + manifest),
    cost-positive (instrument LOC ≤300), bail-safe (per-file
    isolation; no cross-contamination).
C7. C5 from TCC: this locale produces NO direct improvement to TSR.
    Its deliverable is the measurement + the backlog. Sub-locale
    improvements happen via the downstream runtime-bearing-construct
    locales it informs.
```

### I.3 Falsifiers

**Pred-txc.1**: harness runs all corpus files in <300s (Pred-tcc.2 scaled by 5× for the bun + cruft invocation pair).

**Pred-txc.2**: execute-parity baseline MEASURED (the number is the chapter-close finding; could be 5% or 90%, both are valuable data).

**Pred-txc.3**: failure-table top rows are actionable — each row's structural tag names either a runtime-bearing TS construct OR a cruft substrate gap OR a SETUP_FAIL categorization issue.

**Pred-txc.4**: comparison of TCC's parse-parity baseline (95.2%) vs TXC's execute-parity baseline answers the **research question of the session**: how much of "full parity" is parse-tier vs execute-tier? A small gap (e.g., 95.2% → 92%) means TSR's stripping ALREADY achieves close-to-full behavioral parity. A large gap (e.g., 95.2% → 30%) means runtime-bearing constructs dominate the parity surface — sub-locales for enums + ctor-shorthand + decorators are high-priority. Either result is high-information.

**Pred-txc.5 (DISCIPLINE FALSIFIER per apparatus/docs/standing-rule-13-prospective-application.md §5)**: closes in ≤3 implementation rounds.

## II. Apparatus

- **Corpus**: same `pilots/ts-consumer-corpus/fixtures/` as TCC. Reuse, don't duplicate.
- **Driver script**: `pilots/ts-execute-corpus/scripts/run-pair.sh` — given a file path, invoke bun + cruft + diff outputs.
- **Harness binary**: `pilots/ts-execute-corpus/derived/src/bin/txc-measure.rs` — walks corpus, invokes driver per file, categorizes, writes results.
- **Output**: `pilots/ts-execute-corpus/results/<date>/{results.jsonl, summary.md, divergence-table.md}`
- **Standing instruments**: TCC's harness + diff-prod remain authoritative for parse-tier; TXC adds execute-tier without disturbing them.

## III. Methodology

1. **TXC-EXT 0** — workstream founding (this seed + trajectory + manifest refresh).
2. **TXC-EXT 1** — driver script + harness binary + first measurement run + per-outcome categorization. Pred-txc.2 + Pred-txc.4 booking. Likely chapter-close round per standing rule 13.
3. **TXC-EXT 2 (if needed)** — refinement of categorization or driver pattern based on TXC-EXT 1's data.

## IV. Carve-outs

- `bun` as the only oracle at first cut. `tsc + node` deferred (would require global `tsc` install + transpile step per file; cost-prohibitive).
- Synthetic-driver MVP only at first cut; package-level entry-point detection deferred.
- 5-second per-file timeout; tight loop. Slow-but-not-failing files categorized as TIMEOUT.
- No actual fixing of divergences in this locale. Downstream sub-locales are where lowering work lands.

## V. Standing artefacts

- `pilots/ts-execute-corpus/seed.md`, `trajectory.md`
- `pilots/ts-execute-corpus/scripts/run-pair.sh`
- `pilots/ts-execute-corpus/derived/src/bin/txc-measure.rs`
- `pilots/ts-execute-corpus/results/<date>/{results.jsonl, summary.md, divergence-table.md}`

## VI. Resume protocol

Read this seed, then trajectory.md tail. Read TCC's structural precedent (`pilots/ts-consumer-corpus/`) since this locale mirrors TCC's pattern at the execute tier. Read the **full-parity research framing** from the keeper directive that spawned this locale: the goal is to **discover what full parity means** by measuring across multiple parity-definition gates, not just to maximize a single number.

## VII. The research arc this instrument anchors

Per the keeper-approved framing at the spawn directive:

```
ts-execute-corpus/          ← THIS LOCALE (execute-parity instrument)
   ↓ failure-table drives ↓
ts-resolve-enums/           ← enum lowering (runtime-bearing)
ts-resolve-ctor-shorthand/  ← parameter-property lowering
ts-resolve-decorators/      ← Stage 3 decorators
ts-resolve-namespaces/      ← IIFE-wrap lowering (legacy)
   ↓ when execute-parity reaches 100% on corpus ↓
[DISCOVERY OBJECT]          ← write up findings as candidate Doc 7XX:
                              "what full parity means in the cruftless
                              resolver-instance pipeline"
```

Four candidate parity definitions (P/E/B/T) per the keeper's framing message; discovery emerges from which definitions HOLD at which substrate tiers under empirical pressure. Both a large gap AND a small gap between TCC's (P) baseline and TXC's (E) baseline are publishable findings — the gap shape IS the research data.
