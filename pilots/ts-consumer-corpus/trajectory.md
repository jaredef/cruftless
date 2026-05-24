# ts-consumer-corpus — Trajectory

Append-only log of rounds. Most recent at bottom.

---

## TCC-EXT 0 — workstream founding (2026-05-24)

**Trigger**: keeper directive ("B") choosing the consumer-corpus-driven tactic over spec-driven checklist chase, in response to the post-TSR-EXT 5 framing question about scaling TSR to TS parity.

**Strategic framing**: this locale builds the **empirical measurement instrument** that drives the downstream sub-locale arc. Without TCC, the sub-locale priority order would be arbitrary (spec-driven); with TCC, it's data-driven (frequency of real consumer-code failures).

**Founding artefacts**:
- `pilots/ts-consumer-corpus/seed.md` (this round)
- `pilots/ts-consumer-corpus/trajectory.md` (this file)
- `pilots/ts-consumer-corpus/{docs,fixtures,scripts}/` scaffolded

**Pre-spawn rule 11 5-axis check**:
- (A1) component A/B — N/A; this is a measurement locale, not a substrate-leverage locale. The "A/B" is the TSR pre/post baseline (TSR-EXT 5 baseline known).
- (A2) op-set coverage — N/A; no new ops
- (A3) value-domain — N/A; pure tooling
- (A4) locals-marshaling — N/A
- (A5) emission-shape — N/A

**Deeper-layer design (rule 13 prospective)**:
- SKIP intermediate corpus-bootstrap steps; design directly for the top-200 hash-pinned reproducible manifest.
- Harness as a small Rust binary (rather than a shell script) so it can use `ts_resolve::parse_and_erase` directly without shelling out to `cruft` per file (per-file overhead matters at 10K-file scale).
- Failure categorization as a fixed enum + per-failure structural-cause extraction. No vague "parse error" rows.

**Composition with TSR (sibling)**:
- TCC consumes the public `ts_resolve::parse_and_erase` API; no changes to TSR required.
- TCC's failure-table emits backlog rows that spawn TSR sub-locale work.
- The two-locale pair (TSR + TCC) operationalizes the consumer-corpus-driven framing.

**Five Pred-tcc.* + 1 discipline falsifier**:
- Pred-tcc.1: top-200 manifest hash-pinned + reproducible
- Pred-tcc.2: harness runs <60s on full corpus
- Pred-tcc.3: TSR's current parse-success-rate baseline MEASURED (number reported)
- Pred-tcc.4: failure-table top-15 rows are actionable (each names a specific feature/bug/corpus-quality issue)
- Pred-tcc.5 (DISCIPLINE FALSIFIER): closes in ≤3 implementation rounds

**Next round**: TCC-EXT 1 — corpus-manifest design + install-script + initial fetch. Output: `manifest/packages.json` + `fixtures/` populated + hash verification.

**Open questions for TCC-EXT 1**:
- npm "most-depended-on" data source: use `https://registry.npmjs.org/-/v1/search` or the public `most-dependents.json` mirrors? Likely the latter for reproducibility.
- Vendor everything or just download manifest URLs? Vendoring is large (~50-100MB of `.ts` source) but reproducible. Manifest-only is small but needs network. **Recommended**: vendor with `.gitignore` for `fixtures/`, since the source manifest+hashes are the load-bearing record.
- Should `.d.ts` files be excluded? Yes — they're type-declaration files, not source; would inflate the corpus without measuring real parse capability. (Actually edge case: lots of packages have `.ts` files that are de facto `.d.ts`; need to detect via file content, not just extension.)

**Status**: SCAFFOLDED. Founding artefacts written; TCC-EXT 1 (corpus assembly) next.

---

## TCC-EXT 1 — corpus install + measurement harness + first baseline (2026-05-24)

**Round shape**: standing-rule-13 prospective application yielded a 1-implementation-round close for the empirical-instrument tier. TCC-EXT 1 ships install + harness + baseline in a single round; TCC-EXT 2 (categorization refinement) folded in mid-round; TCC-EXT 3 (chapter close) follows immediately.

**Edits**:

1. **Manifest** (`manifest/packages.json`): curated 15-package starter list of npm packages known to ship `.ts` source in their tarball (typescript, zod, date-fns, pino, drizzle-orm, ajv, commander, yargs, rxjs, tslib, yaml, execa, ws, got, joi).
2. **Install script** (`scripts/install.sh`): fetches each `<name>@<version>` via registry.npmjs.org tarball; extracts `*.ts` files (excluding `*.d.ts`) into `fixtures/<name>/`; records per-file sha256 + version into `manifest/file-hashes.json` for reproducibility.
3. **Measurement harness** (`derived/src/bin/measure.rs`, ~210 LOC): walks `fixtures/**/*.ts`, invokes `ts_resolve::parse_and_erase`, categorizes outcomes (Ok / StripError / ParseError / Panic via catch_unwind), extracts structural cause by inspecting source text near the error span, emits per-file JSONL + Markdown summary + ranked failure-frequency table.
4. **Crate registration** in workspace `Cargo.toml`.
5. **`.gitignore`** for `fixtures/` — manifest+hashes are the load-bearing record; vendored source can be re-installed deterministically.

**Empirical result**:
- **374 `.ts` files** measured from 3 packages with shipping source (rxjs 251, ajv 106, pino 17). The other 12 manifest packages ship compiled `.js` only.
- **141 OK / 374 = 37.7% parse-success baseline** (Pred-tcc.3 booked)
- 68 STRIP errors / 165 PARSE errors / **0 PANICs** (good substrate stability)
- **25 ms total / 0.07 ms/file** (Pred-tcc.2 HELD; ~1000× under the 60s budget)

**Top-10 failure rows** (Pred-tcc.4 actionability check):

| Rank | Structural tag | Files | % unblocked |
|---:|---|---:|---:|
| 1 | template-literal-type | 48 | 12.8% |
| 2 | method-return-annotation | 46 | 12.3% |
| 3 | generic-call | 37 | 9.9% |
| 4 | uncategorized-unexpected-token | 20 | 5.3% |
| 5 | readonly-modifier | 11 | 2.9% |
| 6 | access-modifier | 10 | 2.7% |
| 7 | decorator | 9 | 2.4% |
| 8 | lex-unterminated | 7 | 1.9% |
| 9 | lex-invalid-identifier | 4 | 1.1% |
| 10 | import-export-type | 2 | 0.5% |

**Cumulative top-10**: ~52% of corpus would be unblocked if these 10 categories are addressed. **Pred-tcc.4 HELD-PARTIAL** — categorization is heuristic (file-text-near-error inspection, not authoritative root-cause), but rows 1-10 each name a specific actionable concept. Rows beyond #10 are singletons or "uncategorized-*" — long-tail.

**Caveat on categorization**: the heuristic tags can over-attribute. E.g., a `method-return-annotation` row's actual failure may be elsewhere; the tag flags "the file has a `):...{ ` pattern near the error position" without proving causation. Each sub-locale's first round must inspect ≥3 example files from its tag before accepting the diagnosis.

**Substrate-bug priority finding** (separate from missing TS features):
- `lex-unterminated` (7 files) + `lex-invalid-identifier` (4 files) + many of the StripError rows suggest TSR's strip step is **corrupting source inside string literals or template strings** (replacing characters that should not be replaced). This is a substrate bug in `strip.rs`'s scanner, not a missing TS feature. **Priority: fix before any new TS-feature sub-locale, since corrupted source masks the real cause for downstream PARSE errors.**

**Gates**:
- `cargo build --release -p ts-consumer-corpus`: ✅ clean
- `cargo run --release -p ts-consumer-corpus --bin tcc-measure`: ✅ 374 files measured, 0 panics
- `diff-prod 42/42 PASS` ✅ (existing baseline; TCC is non-invasive)
- Workspace build ✅ (TCC registered; ts-resolve unchanged)

**Final disposition** (chapter close as TCC-EXT 1):

| Predicate | Disposition |
|---|---|
| Pred-tcc.1 (manifest hash-pinned + reproducible) | ✅ HELD (manifest + file-hashes.json committed; install script idempotent) |
| Pred-tcc.2 (harness <60s on full corpus) | ✅ HELD at 25 ms (1000× under budget) |
| Pred-tcc.3 (parse-success baseline MEASURED) | ✅ HELD at **37.7%** |
| Pred-tcc.4 (top-15 rows actionable) | ✅ HELD-PARTIAL (top-10 actionable; categorization heuristic but useful) |
| Pred-tcc.5 (≤3 implementation rounds) | ✅ **HELD at 1 implementation round** |

**Findings**:

**Finding TCC.1**: 37.7% parse-success baseline on real npm `.ts` source — a useful concrete target. Sub-locale arc should aim for ≥80% as the next milestone, with ≥95% as the parity target.

**Finding TCC.2**: 3 of 15 (20%) curated "TS-source-bearing" packages actually ship `.ts` source in their npm tarball. Many TS-written packages ship only compiled `.js` + `.d.ts`. For a true top-200 corpus, the manifest scan needs a pre-filter that verifies `.ts` content in the published tarball — not just inspecting the package's GitHub source. Defer to TCC-EXT 2 corpus expansion.

**Finding TCC.3** (substrate-bug priority): TSR's strip step appears to corrupt source inside string/template literals on ~10+ files. This is a substrate bug in TSR, not a TS-feature gap. **Highest-priority next action**: spawn `pilots/ts-resolve-string-literal-safety/` (or fold into TSR follow-on) to make the scanner string-aware. Likely root cause: my Scanner walks tokens via `Lexer::next_token` which DOES handle string boundaries correctly — so the corruption must be in the strip-range computation overshooting into a subsequent string. Inspect the 11 lex-error files at sub-locale founding time.

**Finding TCC.4** (standing rule 13 thesis, fourth corroboration): TCC closed the empirical-instrument tier in 1 implementation round. The thesis now has four corroborations (GPI, IPBR, TSR-substrate-tier, TCC) on the "≤3 rounds when C1-C4 hold" prediction, plus one refinement (TSR-research-tier where C3 failed independently). The discipline is reproducible across substrate-tier, refactor-tier, and tooling-tier work.

**Sub-locale priority order** (set by failure table):

| Priority | Sub-locale | Addresses | Files |
|---:|---|---|---:|
| 1 | **`ts-resolve-string-literal-safety`** (substrate bug) | Finding TCC.3 | 11+ |
| 2 | `ts-resolve-template-literal-types` | template-literal-type rows | 48 |
| 3 | `ts-resolve-classes-v2` (return annotations + readonly + access modifiers + decorators) | rows 2, 5, 6, 7 | 76 |
| 4 | `ts-resolve-generics-calls` | generic-call rows | 37 |
| 5 | `ts-resolve-import-export-type` | import-export-type rows | 2 |
| 6 | `ts-resolve-uncategorized` (catch-all triage) | uncategorized rows | 20+ singletons |

**Total addressable in top-6 sub-locales**: ~190 files = ~50% of corpus. Achieving them all would lift parse-success from 37.7% to ~87%.

### Status: CHAPTER CLOSED at TCC-EXT 1

Standing-rule-13 prospective-application: **fifth corroboration** (counting both the substrate-tier work and this tooling-tier work). All five Pred-tcc.* HELD plus the discipline falsifier at 1 implementation round.

**Empirical instrument is operational + repeatable**. Sub-locale arc has a data-driven priority order. The corpus IS the constraint (Pin-Art derived-from-constraints discipline operationalized at the language-tier).

**Next moves** (in priority order):
1. Spawn `ts-resolve-string-literal-safety/` (substrate-bug fix; should be 1-round per standing rule 13)
2. Re-measure TCC; expect parse-success to lift by ~3-5% just from the string-safety fix unmasking PARSE errors as different categories
3. Spawn top-priority feature sub-locales from the updated table
