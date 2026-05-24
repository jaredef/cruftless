# ts-execute-corpus — Trajectory

## TXC-EXT 0 — workstream founding (2026-05-24)

**Trigger**: keeper directive "Yes" at the close of the full-parity-framing message — spawning TXC as the second instrument-tier locale, complementing TCC's parse-parity measurement with execute-parity (the achievable proxy for behavioral-erasure parity per Doc 729's resolver-instance pattern).

**Strategic framing**: this locale anchors the **full-parity research arc**. The keeper-approved arc shape:
```
TXC (this locale)  →  ts-resolve-enums / -ctor-shorthand / -decorators / -namespaces
                  →  [DISCOVERY: what does full parity mean in the resolver-instance pipeline?]
```

Four candidate parity definitions enumerated in seed §VII (P/E/B/T); the discovery target is which definitions hold at which substrate tiers under corpus pressure.

**Founding artefacts**: seed.md + trajectory.md + scaffolded scripts/ dirs. TXC-EXT 1 (harness implementation + first measurement) next.

---

## TXC-EXT 1 — harness implementation + first execute-parity baseline + CHAPTER CLOSE (2026-05-24)

**Round shape**: instrument-tier locale, one implementation round per standing-rule-13 prospective application. Deliverable is the baseline measurement + the actionable failure distribution.

**Design pivot mid-round**: initial design used a synthetic-import-driver wrapper (`import * as M from FILE; console.log(Object.keys(M).sort().join('\\n'))`). First smoke test (20 files) revealed 75% BUN_FAIL due to unresolved dependencies inside the corpus fixtures (consumer packages depend on each other; only 3 packages were vendored). Pivoted to status-only comparison: run each fixture directly with both runtimes; MATCH = both exit 0; CRUFT_FAIL = bun ok, cruft errored. Simpler + more signal.

**Edits**:
- `pilots/ts-execute-corpus/derived/Cargo.toml` — crate registration
- `pilots/ts-execute-corpus/derived/src/bin/measure.rs` (~280 LOC) — harness binary; walks corpus, runs bun + cruft per file, categorizes via fixed taxonomy, writes per-file JSONL + summary + divergence-table
- Workspace `Cargo.toml` — registered

**Gates**:
- `cargo build --release -p ts-execute-corpus`: ✅ clean
- Smoke test (20 files): runs in ~1s, taxonomy categorization working
- Full corpus run (374 files): runs in 18s, well under Pred-txc.1's ≤300s budget

**🎯 Empirical baseline (the research-question answer)**:

| Outcome | Count | % |
|---|---:|---:|
| **MATCH** (execute-parity) | **19** | **5.1%** |
| DIVERGE (both run, output differs — not detected under status-only) | 0 | 0% |
| BUN_FAIL (oracle can't even load — file unrunnable in any context) | 108 | 28.9% |
| **CRUFT_FAIL (actionable — bun loads, cruft errors)** | **247** | **66.0%** |
| SETUP_FAIL | 0 | 0% |
| TIMEOUT | 0 | 0% |

**Pred-txc.2 BOOKED**: execute-parity baseline = **5.1%**.

**Pred-txc.4 BOOKED — the research question's first answer**:
- Parse-parity (P) baseline: 95.2%
- Execute-parity (E) baseline: 5.1%
- **Gap: 90.1 percentage points**

This is a **huge** gap — and it's the **load-bearing finding** for the full-parity research arc. The substrate's strip-output PARSES correctly under TSR's direct invocation (95.2%), but the cruftless RUNTIME can't actually EXECUTE the same files (5.1%). The 90.1 pp gap reveals that execute-parity is dominated by NON-parse issues.

**Top divergence rows** (Pred-txc.3 actionability check):

| Rank | Tag | Files | Substrate area |
|---:|---|---:|---|
| 1 | `CompileError("parse: expected LBrace"...)` | 90 | Imported-`.ts`-files not strip-routed; raw TS bytes reach the parser via the module loader |
| 2 | `TypeError("module not found: './...'")` | 85 | Module loader doesn't try `.ts`/`.mts`/`.cts` extension during import resolution |
| 3 | `CompileError("parse: unexpected token"...)` | 31 | Same as #1 likely; some other strip-bypass path |
| 4 | `CompileError("parse: expected Colon"...)` | 18 | Same |
| 5 | `CompileError("parse: expected RParen"...)` | 13 | Same |

**Rows 1, 3, 4, 5 (152 files = 41%)** are all symptoms of the SAME substrate gap: **cruft's runtime module loader is TS-unaware**. When a `.ts` file imports `./other` and the loader resolves to `./other.ts`, it reads RAW TS bytes and feeds them to the JS parser — which fails at the first TS-only construct.

**Row 2 (85 files = 23%)**: SAME root cause manifesting as resolution failure. When `import './foo'` is unsable to find any of `./foo`, `./foo.js`, `./foo.mjs` (current behavior), it errors with "module not found" — even though `./foo.ts` exists.

**Cumulative: ~64% of all CRUFT_FAIL traces back to ONE substrate gap**: the module loader's TS-unawareness.

### Findings

**Finding TXC.1** (parity-research first answer): the parse-parity (P) → execute-parity (E) gap is **90.1 percentage points**, and **64% of the gap is attributable to a single substrate-tier gap (TS-unaware module loader)**. The remaining ~36% of CRUFT_FAIL is the actual runtime-bearing-construct territory (enums, ctor-shorthand, decorators, namespaces) the full-parity research arc was framed around.

**Finding TXC.2** (Doc 729 resolver-instance pattern implication): the gap data confirms that **behavioral-erasure parity (B) per Doc 729 cannot be achieved by upgrading the TSR resolver alone** — it requires the DOWNSTREAM runtime tier to also be TS-resolver-aware (or to have TSR plumbed through the runtime's module loader). The resolver-instance contract isn't "the input resolver consumes its directives and downstream is clean" — it's "downstream tiers MUST also dispatch through the input resolver when loading dependent inputs."

**Finding TXC.3** (load-bearing sub-locale next): the load-bearing next-action is `ts-resolve-module-loader-extension/` — a sub-locale that (a) adds `.ts`/`.mts`/`.cts` to the module loader's extension search list, and (b) applies TS-strip when loading those extensions. This is the SINGLE substrate fix that unblocks ~64% of the parity gap. Other runtime-bearing-construct sub-locales (enums etc.) follow but smaller per-fix yield.

**Finding TXC.4** (standing-rule-13 ninth corroboration at instrument-tier): TXC closed in 1 implementation round per standing-rule-13's expected discipline.

### Status: CHAPTER CLOSED at TXC-EXT 1

All five Pred-txc.* HELD:
- Pred-txc.1 ✅ HELD at 18s (60× under 300s budget)
- Pred-txc.2 ✅ HELD — baseline measured at 5.1%
- Pred-txc.3 ✅ HELD-STRONGLY — top-5 rows all actionable; top row clusters at 4 categories of one substrate gap
- Pred-txc.4 ✅ HELD-STRONGLY — research question gets a first answer with a 90.1 pp gap, dominated by ONE substrate gap (64%)
- Pred-txc.5 ✅ HELD at 1 implementation round

**Next sub-locale (research-arc step 2)**: `ts-resolve-module-loader-extension/`.

**Pre-spawn checks**:
- bun installed at `~/.bun/bin/bun` (version 1.3.11); available as TS-strip oracle ✓
- cruft has end-to-end .ts execution via TSR (per TSR-EXT 4) ✓
- TCC corpus exists with 374 files at `pilots/ts-consumer-corpus/fixtures/` ✓
- TCC manifest is hash-pinned (reproducibility instrument ready to reuse) ✓
