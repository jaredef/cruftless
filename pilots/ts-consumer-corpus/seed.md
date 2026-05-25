# ts-consumer-corpus — Resume Vector / Seed

**Locale tag**: `L.ts-consumer-corpus` (top-level per Doc 737 §IV)

**Status as of 2026-05-24**: **CHAPTER CLOSED at TCC-EXT 1 (1 implementation round; Pred-tcc.5 HELD)**. All five Pred-tcc.* HELD plus the discipline falsifier. Baseline parse-success = **37.7%** (141/374 files from rxjs + ajv + pino). Failure-frequency table operational; top-10 categories cover ~52% of corpus and set the sub-locale priority order. Substrate-bug priority finding (Finding TCC.3): TSR's strip step corrupts source inside string/template literals on ~11 files — highest-priority next sub-locale.

**Historical status (founding)**: WORKSTREAM FOUNDED (TCC-EXT 0). Spawned per keeper directive (option B: consumer-corpus-driven scaling) over the alternative (spec-driven checklist chase). This locale is the **empirical measurement instrument** that drives the downstream sub-locale arc (`ts-resolve-enums/`, `ts-resolve-classes/`, `ts-resolve-generics-calls/`, `ts-resolve-decorators/`, etc.) — failures in the corpus surface what to build next, in priority order set by frequency.

**Workstream**: build a parse-success + execute-success harness over the top-N most-depended-on `.ts` packages on npm. Each measurement run produces:
1. A success-rate baseline (TSR's current capability surface)
2. A failure categorization table (lexical / parser / runtime / not-implemented) ranked by file-frequency
3. Per-failure pointers (which TS feature unblocks which N% of corpus)

The corpus IS the constraint per Pin-Art discipline (Doc 738 source-identifier coordinates + Doc 581 derived-from-constraints) — what real consumer code uses, NOT what the TS spec enumerates. TSR-EXT 5's null result already vindicated this framing at the substrate-leverage layer; we apply the same principle to feature-priority selection.

**Author**: 2026-05-24 session.
**Parent**: none (top-level).
**Siblings**: `ts-resolve/` (the TSR locale this corpus measures; closed at TSR-EXT 5).
**Composes with**:
- [Doc 581](../../docs/corpus-ref/581-the-resume-vector.md) — Pin-Art derived-from-constraints; corpus IS the constraint
- [Doc 723 §IV.b](../../docs/corpus-ref/723-diagnostic-tags-as-semiotic-signs-layer-indexed-interpretation-in-pipeline-dag-topologies.md) — finding-density via clean null results; each corpus failure is a finding
- [Doc 729](../../docs/corpus-ref/729-cruftless-a-primary-articulation-of-the-resolver-instance-pattern-as-the-comprehensive-design-toward-which-rusty-bun-morphs.md) — resolver-instance pattern; TSR is one such instance, this corpus measures its coverage
- [apparatus/docs/standing-rule-13-prospective-application.md](../../apparatus/docs/standing-rule-13-prospective-application.md) — TCC is the fourth prospective application of the thesis; expected ≤3 implementation rounds for instrument tier
- [TSR chapter close](../ts-resolve/trajectory.md) — empirical anchor; TSR-EXT 5's Finding TSR.1 informs the corpus locale's scope (skip the substrate-leverage probe; focus on parse/execute coverage)

## I. Telos

**Empirical answer to**: when TSR is pointed at the top-200 most-depended-on `.ts` packages on npm, what fraction of files parse without TSR error, and what fraction of files that parse also execute without runtime divergence vs Node's tsc-pipeline behavior?

The headline numbers drive the downstream arc: a parse-success-rate of ≥95% is the parity target; the gap (5% of files) becomes the prioritized backlog of sub-locales.

### I.1 First-cut scope

Per standing rule 13 + Doc 740 §IV.2: design from the deeper-layer first. SKIP intermediate steps (e.g., "build a partial corpus first, then expand"). Go directly to:
- The 200-package manifest (selection criterion: npm download count over last 30 days, filtered to packages with `.ts` source files in their public `src/` or `lib/` paths — NOT just typings)
- Hash-pinned reproducible install (vendoring the `.ts` source files to `fixtures/` with a manifest of source paths + content hashes)
- A measurement harness that invokes `ts_resolve::parse_and_erase` per file + categorizes outcomes
- A failure-frequency table emitted as `failure-table.md`

Out of scope (deferred to follow-on sub-locales):
- Actually FIXING the failures — this locale just measures
- Execution-correctness diffing against Node — TCC-EXT 3 or a follow-on
- Auto-rebuilding the corpus periodically — TCC-EXT 3 if useful

### I.2 Constraints (Pin-Art enumeration)

```
C1. Corpus selection is reproducible: a manifest of (package@version,
    source-file-path, sha256-hash) committed to the repo. Re-running
    the harness against the manifest yields identical input bytes.
C2. Per-file outcomes categorized into a fixed taxonomy:
      OK — parse succeeded; erased AST returned cleanly
      LEX — rusty_js_parser::Lexer rejected (likely a JS-level issue
            that's not TSR's responsibility — flag for investigation)
      PARSE — rusty_js_parser::parse_module rejected the stripped
              output (TSR's strip produced invalid JS; bug or missing
              feature)
      STRIP — TSR's strip step itself errored
      PANIC — Rust panic anywhere in the pipeline (highest priority bug)
C3. Failure-table rows ranked by file count; the top-15 are the
    actionable backlog. A row that names a specific TS feature
    (e.g., 'enum' or 'decorator') points at a specific sub-locale.
C4. Harness runs in <60s on the full corpus (200 packages × ~50
    .ts files each = ~10K files); per-file ~6ms budget. Per-file
    work is just strip+parse; no execution at this round.
C5. The corpus locale produces NO direct improvement to TSR — its
    deliverable is the measurement + the backlog. Improvement
    happens via the downstream sub-locales it informs.
C6. Per apparatus/docs/standing-rule-13-prospective-application.md §3:
    (C1.sibling-anchor) `ts-resolve` itself is the consumer; TCC is
                        the measurement of its current state
    (C2.shape-compat)  TSR's API (`parse_and_erase`) is stable;
                        compatible
    (C3.cost-positive) instrument LOC ≤ 200 (small); informational
                        payoff is large (prioritized backlog)
    (C4.bail-safe)     a harness failure doesn't affect TSR or
                        cruftless; isolated
```

### I.3 Falsifiers

**Pred-tcc.1**: top-200 manifest assembled + hash-pinned, with reproducible content fetch (script + checksums). Falsifier: any non-determinism in the corpus contents.

**Pred-tcc.2**: harness runs all corpus files in <60s on Aarch64 release-build cruft. Falsifier: per-file overhead exceeds budget; would block CI integration.

**Pred-tcc.3**: TSR's current parse-success-rate baseline is MEASURED (not estimated). The actual number could be anywhere from 30% to 90%; the falsifier is failing to measure it cleanly. Result becomes the chapter-close report.

**Pred-tcc.4**: the failure-table's top-15 rows are actionable — each names either (a) a specific TS feature missing from TSR, (b) a runtime-substrate bug in cruftless, or (c) a corpus-quality issue (e.g., a `.ts` file is actually `.d.ts` mislabeled). Vague "parse error at line N" rows are NOT actionable; the categorization must surface the structural cause.

**Pred-tcc.5 (DISCIPLINE FALSIFIER per apparatus/docs/standing-rule-13-prospective-application.md §5)**: locale closes in ≤3 implementation rounds. Per the fourth prospective application of the rule; if exceeded, diagnose which C-condition failed (C3 cost-positive is the load-bearing one per TSR-EXT 5's refinement).

## II. Apparatus

- **Corpus manifest**: `pilots/ts-consumer-corpus/manifest/packages.json` — list of `{name, version, npm-url, expected-files, sha256}` records.
- **Corpus source**: `pilots/ts-consumer-corpus/fixtures/` — vendored `.ts` files under `<package>/<rel-path>.ts`. Vendored for reproducibility; refreshed on demand via the install script.
- **Install script**: `pilots/ts-consumer-corpus/scripts/install.sh` — pulls each package via npm registry tarball; extracts `*.ts` files (excluding `*.d.ts`) under `src/`/`lib/`; verifies hashes; writes to `fixtures/`. Idempotent.
- **Measurement harness**: `pilots/ts-consumer-corpus/scripts/measure.sh` (or a small Rust binary `pilots/ts-consumer-corpus/derived/src/bin/measure.rs`) — walks `fixtures/`, invokes `ts_resolve::parse_and_erase`, categorizes outcomes, writes per-file results to JSONL + an aggregate failure-table.
- **Failure table**: `pilots/ts-consumer-corpus/results/<date>/failure-table.md` — Markdown table; rows = failure-kind × representative-file-snippet × frequency.
- **Bench instruments**: harness wall-clock on the full corpus (for Pred-tcc.2).
- **Correctness instruments**: existing TSR test suite + diff-prod (untouched).

## III. Methodology

1. **TCC-EXT 0** — workstream founding (this seed + trajectory + manifest refresh + CANDIDATES update).
2. **TCC-EXT 1** — corpus-manifest design + install-script implementation + initial corpus fetch (~200 packages, ~10K files). Output: `manifest/packages.json` + `fixtures/` populated + hash verification.
3. **TCC-EXT 2** — measurement harness implementation + first measurement run + failure-table emission. Pred-tcc.3 booking (the baseline number). Pred-tcc.4 booking (actionability check).
4. **TCC-EXT 3 (if needed)** — refinement: categorization improvements, edge-case handling. If TCC-EXT 2 already produces actionable output, chapter closes there in 2 implementation rounds.

(**Discipline target**: ≤3 implementation rounds per Pred-tcc.5.)

## IV. Carve-outs and bounded scope

- Parse-success only at first cut; execution-correctness diffing deferred to a follow-on (would need Node as oracle + per-file fixture setup).
- Top-200 packages; broader corpus deferred.
- `*.ts` files only; `.tsx` deferred (separate locale via `ts-resolve-jsx/` when ready).
- Snapshot manifest at one date; auto-refresh deferred.
- No fixes — this locale measures; sibling sub-locales fix.

## V. Standing artefacts

- `pilots/ts-consumer-corpus/seed.md`, `trajectory.md`
- `pilots/ts-consumer-corpus/docs/design.md` (TCC-EXT 1)
- `pilots/ts-consumer-corpus/manifest/packages.json` (TCC-EXT 1)
- `pilots/ts-consumer-corpus/fixtures/` (TCC-EXT 1)
- `pilots/ts-consumer-corpus/scripts/install.sh` (TCC-EXT 1)
- `pilots/ts-consumer-corpus/scripts/measure.sh` OR `pilots/ts-consumer-corpus/derived/src/bin/measure.rs` (TCC-EXT 2)
- `pilots/ts-consumer-corpus/results/<date>/failure-table.md` + `results.jsonl` (TCC-EXT 2+)

## VI. Resume protocol

Read this seed, then trajectory.md tail. Read `pilots/ts-resolve/seed.md` + `trajectory.md` (TSR-EXT 5 chapter close) for the substrate this corpus measures. Read `apparatus/docs/standing-rule-13-prospective-application.md` §3a (C-condition independence) — this locale is the fourth prospective application of the rule; expected ≤3 rounds. Read `apparatus/locales/CANDIDATES.md` Tier-D (j-k) for the strategic-arc context — TCC is the empirical instrument that informs which Tier-D follow-on sub-locales to spawn next, in what priority.

## VII. Downstream arc this locale informs

Per the keeper-approved (B) tactic, the sub-locale arc to spawn (in priority order set by TCC-EXT 2's failure table):

```
ts-consumer-corpus/         ← THIS LOCALE (measurement instrument)
  │
  ├ failure analysis ranks →
  │
ts-resolve-enums/           ← high-frequency runtime-bearing
ts-resolve-classes/         ← ctor-param shorthand, abstract, accessor
ts-resolve-generics-calls/  ← f<T>() angle-bracket disambig
ts-resolve-decorators/      ← Stage 3 decorators
ts-resolve-namespaces/      ← legacy but persistent in tooling
ts-resolve-conditional/     ← cond + mapped + template-literal types
ts-resolve-jsx/             ← separate locale; JSX/TSX
```

Each sub-locale targets ≤3 implementation rounds per the standing-rule-13 thesis. TCC's failure table provides the priority signal — features that unblock the most corpus files spawn first.

## VIII. Strategic context

TSR-EXT 5's Finding TSR.1 (annotation-driven IPBR shape skip returned NULL) materially refined what TSR is FOR: not substrate-leverage at the IPBR consumer, but native `.ts` execution as a coverage capability + (potentially, still to probe) JIT IC specialization + VD tag preservation at other consumers. TCC operationalizes the coverage capability claim — without it, TSR's value proposition is "it handles the surface I happened to test"; with it, the value proposition is "it handles ≥95% of real `.ts` source on npm."

The corpus is the constraint. Build the instrument; let the data drive the backlog.
