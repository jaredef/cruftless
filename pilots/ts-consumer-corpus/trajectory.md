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
