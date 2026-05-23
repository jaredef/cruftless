# rusty-js-regex-fast — Resume Vector / Seed

**Locale tag**: `L.rusty-js-regex-fast` (top-level per Doc 737 §IV)

**Status as of 2026-05-23**: **WORKSTREAM FOUNDED (RXF-EXT 0)**. No code yet. Spawned per keeper directive 2026-05-23 18:12-local. **Dual focus**: regex performance + memory leak. Per the engagement's pattern, related concerns at the same substrate live in one locale's trajectory with sub-rounds per concern.

**Workstream**: regex engine performance improvements + memory-leak investigation/fix at cruftless's `regex_hand` module. Per keeper triage: both perf and leak are real gaps; the leak investigation is bounded (reproduce + bisect + fix-or-name); perf is multi-week.

**Author**: 2026-05-23 session.
**Parent**: cruftless engagement (`/home/jaredef/rusty-bun`). Standalone top-level pilot.
**Composes with**:
- [CRB-EXT 9 jit-eligible-vs-realistic doc](../cross-runtime-bench/docs/jit-eligible-vs-realistic.md) — empirical anchor: regex contributes to string_url_sweep's 8.31× gap
- [Findings doc IV.4 standing fuzz](../rusty-js-jit/findings.md) — any default-on flip uses canonical fuzz per rule 10
- [Doc 735 §X.h.b](../../../corpus-master/corpus/735-the-temporal-resolver-instance-stack-build-time-process-time-call-time-as-the-time-axis-dual-to-doc-729s-spatial-stack.md) — (P2) categorization for perf moves
- [Doc 736 §IX.6](../../../corpus-master/corpus/736-the-architecturally-impossible-supply-chain-attack-capability-passing-closed-import-graphs-and-load-time-integrity-as-the-design-that-removes-ambient-authority.md) — cap-passing preserved

## I. Telos

**Dual telos**:

1. **Memory leak**: identify + close any leak in `regex_hand`. Per keeper-reported observation; surface unknown. Investigation: reproduce in isolation; bisect; fix root cause OR document as known issue with workaround.

2. **Regex performance**: improve hot-path regex execution. Per CRB-EXT 9 reading: regex is a component of string_url_sweep's 8.31× cruft/node gap. Closing regex's contribution would shift cruft toward bun's speed on realistic string-heavy workloads.

### I.1 First-cut scope per concern

**Leak investigation (RXF-EXT 1-3)**:
- RXF-EXT 1: reproduce in isolation. Build a small fixture that exercises the leak pattern; measure RSS growth via `/proc/self/status` or equivalent.
- RXF-EXT 2: bisect. Per Pattern 1 from DEBUG-METHODOLOGY.md (bisect-by-jsonl-diff if applicable; otherwise binary search on input shape).
- RXF-EXT 3: fix root cause OR document as known issue.

**Performance (RXF-EXT 4+)**:
- RXF-EXT 4: bench probe baseline. Per-op cost on canonical regex patterns (literal, alternation, quantifier, lookahead, anchored).
- RXF-EXT 5: hot-path component decomposition.
- RXF-EXT 6+: substrate moves per identified components.

### I.2 Falsifiers

**Pred-rxf.1** (leak): post-investigation, RSS growth on the repro fixture stabilizes (bounded growth or flat) over a workload that previously grew unboundedly. Falsifier: leak persists post-fix, or no leak actually exists (false-positive report).

**Pred-rxf.2** (perf): post-implementation, string_url_sweep CRB fixture drops by ≥25% wall-clock (target 750 ms → ≤560 ms). Falsifier: <25% reclaim → regex wasn't the bottleneck or implementation didn't deliver.

**Pred-rxf.3**: canonical fuzz remains byte-identical post-implementation. Falsifier: divergence → (P2.c) illegal-speed.

**Pred-rxf.4**: diff-prod 42/42 holds across all 7 regex test classes (literal/star_greedy/lazy/lookahead_pos/lookahead_neg/pathe_pattern/picomatch_pattern).

**Pred-rxf.5**: no new leaks introduced by perf work (leak-tracking probe at RXF-EXT 5+).

## II. Apparatus

Composes with:
- **regex_hand module** (rusty-js-runtime): the substrate being improved
- **CRB pilot** (standing): string_url_sweep bench fixture
- **Canonical fuzz** (standing): correctness gate
- **Process RSS monitoring** for leak investigation

Per Doc 738 §II.e: code lands at `pilots/rusty-js-runtime/derived/src/regex_hand.rs`. Per §II.b: helper functions follow post-§A8.32 receiver-discriminated form.

## III. Methodology

1. **RXF-EXT 0** — workstream founding (this seed + trajectory + scaffold).
2. **RXF-EXT 1** — leak reproduction.
3. **RXF-EXT 2** — leak bisect.
4. **RXF-EXT 3** — leak fix or named-known-issue.
5. **RXF-EXT 4** — perf bench probe baseline.
6. **RXF-EXT 5** — perf hot-path decomposition.
7. **RXF-EXT 6+** — substrate moves per decomposition.
8. **RXF-EXT 7** — composition with canonical fuzz + CRB + diff-prod.
9. **RXF-EXT 8** — default-on flip if applicable.

## IV. Carve-outs and bounded scope

- regex engine only; no new regex features (lookbehind, named groups, etc.) unless required for canonical correctness
- Aarch64 only
- Leak investigation single-session bounded; perf multi-week
- Per Findings rule 5 + standing rule 10: canonical fuzz at default-on flip

## V. Standing artefacts

- `pilots/rusty-js-regex-fast/seed.md`, `trajectory.md`
- `pilots/rusty-js-regex-fast/docs/` for leak-investigation + perf-baseline + design
- `pilots/rusty-js-regex-fast/fixtures/` for regex-specific test cases
- Implementation lands in `pilots/rusty-js-runtime/derived/src/regex_hand.rs`

## VI. Resume protocol

Read this seed, then trajectory.md tail. Read CRB-EXT 9 for empirical context.
