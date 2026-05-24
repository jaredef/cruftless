# Repository Apparatus for the Cybernetic Discipline

The cruftless engagement runs as a closed-loop cybernetic system: substrate work modifies the codebase, measurement instruments observe its behavior against external oracles, discipline artifacts constrain what counts as a valid move, and the locale structure both executes the work and accumulates the record of what was attempted. This doc enumerates every piece of the apparatus, organized by role in the feedback loop, with explicit pointers to where each piece lives in the repo + which corpus doc articulates its theory.

The apparatus is comprehensive — every standing artifact this engagement uses is listed here. New artifacts should be added to this doc as they are created.

---

## I. The cybernetic loop

```
         ┌──────────────────────────┐
         │  (1) substrate state     │
         │   (the cruftless code)   │
         └────────┬─────────────────┘
                  │
                  │ observed by
                  ▼
         ┌──────────────────────────┐
         │  (2) measurement         │
         │      instruments         │
         └────────┬─────────────────┘
                  │
                  │ produces signals interpreted via
                  ▼
         ┌──────────────────────────┐
         │  (3) discipline          │
         │      artifacts           │
         └────────┬─────────────────┘
                  │
                  │ constrains what next move counts as
                  ▼
         ┌──────────────────────────┐
         │  (4) locale structure    │
         │   (executes + records)   │
         └────────┬─────────────────┘
                  │
                  │ commits modifications to
                  ▼
              back to (1)
```

The loop sustains across sessions. Each cycle adds to (3) by surfacing new findings + standing rules; each cycle adds to (4) by accumulating seed/trajectory pairs at new locales. The apparatus is what makes the loop self-improving rather than drift-prone.

---

## II. (2) Measurement instruments — what observes

Every substrate move is observed by at least one instrument before it lands. Instruments are organized by what they observe + what oracle they triangulate against.

### Cross-runtime parity instruments

| Instrument | Path | Observes | Oracle |
|---|---|---|---|
| **CRB (cross-runtime bench)** | `pilots/cross-runtime-bench/scripts/run-bench.sh` | Wall-clock per-fixture vs node + bun; median of N runs | node v22.22.0 + bun 1.3.11 |
| **diff-prod** | `scripts/diff-prod/run-all.sh` | Byte-identical stdout per fixture under cruft vs bun | bun 1.3.11; 42-fixture suite |
| **test262-sample** | `scripts/test262-sample/run-sample.sh` | ECMA-262 conformance via the official test suite | TC39 test262; 7,589-test curated representative sample |
| **canonical fuzz** | `pilots/rusty-js-shapes/consumer-migration/fixtures/fuzz-canonical.mjs` | Deterministic accumulator over 2000 randomized invocations | node acc=−932188103 (version `cmig-ext-17-2026-05-23`) |

### Locale-tier corpus instruments

| Instrument | Path | Observes | Categorization |
|---|---|---|---|
| **TCC (TypeScript consumer corpus, parse-parity)** | `pilots/ts-consumer-corpus/derived/src/bin/measure.rs` | `ts_resolve::parse_and_erase` outcome per real npm `.ts` file | OK / STRIP / PARSE / PANIC; structural-tag heuristic |
| **TXC (TypeScript execute corpus, execute-parity)** | `pilots/ts-execute-corpus/derived/src/bin/measure.rs` | Exit-status comparison under bun + cruft per fixture | MATCH / DIVERGE / BUN_FAIL / CRUFT_FAIL / SETUP_FAIL / TIMEOUT |
| **TCC manifest** | `pilots/ts-consumer-corpus/manifest/packages.json` + `file-hashes.json` | Hash-pinned corpus of 374 `.ts` files from rxjs + ajv + pino | reproducibility instrument |

### Component-decomposition probes

| Instrument | Path | Observes | Used by |
|---|---|---|---|
| **JSF A/B probe** | `pilots/rusty-js-json-fast/fixtures/component-ab-probe.mjs` | 5 additive variants × 50-iter warmup × 500-iter measurement | Rule 11 axis A1 |
| **string_url_sweep A/B probe** | `pilots/cross-runtime-bench/fixtures/string_url_sweep/component-ab-probe.mjs` | Header-loop dominator decomposition | IHI/GPI/IPBR locales |

### Self-validation instruments

| Instrument | Path | Observes |
|---|---|---|
| **derive-constraints apparatus** | `derive-constraints/` | Extract → cluster → invert pipeline for spec-constraint discovery |
| **locale manifest discoverer** | `scripts/locales/discover.sh` → `scripts/locales/manifest.json` | Filesystem walk of all `seed.md` files; produces coordinate manifest |

---

## III. (3) Discipline artifacts — what constrains

The discipline is recorded in artifacts that future substrate work must consult. These are the conditioning corpus + discipline set per the SIPE-T mapping (see Doc 541 Appendix E).

### Canonical findings + rules

| Artifact | Path | Role |
|---|---|---|
| **findings.md** | `pilots/rusty-js-jit/findings.md` | Canonical append-only ledger; 26 findings + 15 standing rules across 10 addenda |
| **predictive-ruleset.md** | `docs/predictive-ruleset.md` | Consolidated derived view of the 15 standing rules as falsifiable predictions |
| **standing-rule-13-prospective-application.md** | `docs/standing-rule-13-prospective-application.md` | The revert-then-deeper-layer-closure thesis with 12 prospective corroborations |

### Locale-tier discipline

| Artifact | Path | Role |
|---|---|---|
| **CANDIDATES.md** | `scripts/locales/CANDIDATES.md` | Tiered queue of prospective locales; spawn-protocol + status legend |
| **locale manifest** | `scripts/locales/manifest.json` | Authoritative inventory; load-bearing record of the coordinate space per Doc 737 |

### Orientation + policy

| Artifact | Path | Role |
|---|---|---|
| **AGENTS.md / CLAUDE.md** | `AGENTS.md`, `CLAUDE.md` (identical) | Project identity + workspace layout + commit-and-authorization discipline + standing corpus references |
| **THIS DOC (repository-apparatus.md)** | `docs/repository-apparatus.md` | Comprehensive enumeration of the apparatus + cybernetic-loop framing |

### Per-locale discipline (recurrent)

Every locale carries its own discipline artifacts per Doc 581 + Doc 733:

| Artifact | Path pattern | Role |
|---|---|---|
| **seed.md** | `pilots/<locale>/seed.md` | Telos + constraints (Cn) + falsifiers (Pred-n) + methodology + carve-outs + composes-with |
| **trajectory.md** | `pilots/<locale>/trajectory.md` | Append-only round-by-round record; each round describes a substrate move with empirical outcome |
| **docs/design.md** | `pilots/<locale>/docs/design.md` | Per-locale apparatus + LOC budget + open risks (where used) |

---

## IV. (4) Locale structure — what executes + records

The locale is the fundamental unit of substrate work in this engagement (per Doc 737). Every workstream is a locale; every locale has its own seed/trajectory pair; nested locales spawn at deeper coordinates per Doc 737 §II promotion threshold.

### Locale workspace layout

```
pilots/
  <locale-name>/
    seed.md              ← apparatus + telos + falsifiers
    trajectory.md        ← append-only rounds
    docs/                ← per-locale design docs (optional)
    fixtures/            ← per-locale test fixtures (optional)
    derived/             ← Pin-Art-derived Rust crate (where applicable)
      Cargo.toml
      src/
```

### Current locale count

36 locales as of 2026-05-24 (25 top-level, 11 nested) — see `scripts/locales/manifest.json` for the authoritative list.

### Locale spawn protocol (per Doc 737 + standing rule 11)

1. Confirm the workstream's multi-rung shape (Doc 737 §II promotion threshold).
2. Run rule 11's 5-axis pre-spawn coverage check (A1 component A/B is load-bearing).
3. Create `pilots/<name>/seed.md` with telos / constraints / falsifiers / methodology / carve-outs / composes-with.
4. Run `scripts/locales/discover.sh`; commit the refreshed manifest in the same change as the seed.
5. Begin substrate work at round EXT 0 (founding) → EXT 1 (first substrate move) → EXT N (close).

### Locale resume protocol

To pick up work on any locale:
1. Read `seed.md` first (telos + apparatus + methodology).
2. Read `trajectory.md` tail (most recent rounds).
3. The pair is sufficient for a fresh reader to become operational on that workstream in one read (Doc 581 resume-vector discipline).

---

## V. (1) Substrate state — what is observed/modified

The substrate is the cruftless codebase itself, organized by Doc 729's resolver-instance pattern (each tier consumes the prior tier's directives; no residue carries downstream).

### Substrate tiers (top-down per Doc 730)

| Tier | Crate(s) | Role |
|---|---|---|
| **Source-language resolvers** | `pilots/rusty-js-parser/derived/`, `pilots/ts-resolve/derived/` | Surface-syntax → AST |
| **AST** | `pilots/rusty-js-ast/` | Common surface representation |
| **IR (spec-as-source-of-truth)** | `pilots/rusty-js-ir/derived/` | Tier-1.5 IR per IR-DESIGN.md |
| **Bytecode** | `pilots/rusty-js-bytecode/derived/` | Compiler + opcode definitions |
| **Runtime** | `pilots/rusty-js-runtime/derived/` | Interpreter + module loader + intrinsics |
| **JIT** | `pilots/rusty-js-jit/derived/` | Cranelift-backed baseline JIT (LeJIT) |
| **Shapes** | `pilots/rusty-js-shapes/derived/` | Hidden classes substrate |
| **GC** | `pilots/rusty-js-gc/derived/` | Garbage collector |
| **Package manager** | `pilots/rusty-js-pm/derived/` | Resolver-instance #0 per Doc 732 |
| **Capability-passing runtime** | `pilots/rusty-js-caps/` | Architecturally-impossible-supply-chain-attack substrate per Doc 736 |
| **Per-surface pilots** | `pilots/{fetch-api, http-codec, sockets, blob, ...}/derived/` | Pin-Art-derived implementations of consumer-facing surfaces |
| **Host binary** | `cruftless/` | The `cruft` CLI |

### Locale instrument tier (operating ABOVE the substrate)

| Locale | Role |
|---|---|
| `ts-consumer-corpus/` (TCC) | Parse-parity measurement |
| `ts-execute-corpus/` (TXC) | Execute-parity measurement |
| `interp-hot-intrinsics/`, `interp-getprop-ic/`, `iter-protocol-bytecode-rewrite/` | Cross-tier IC + dispatch optimization |
| `ts-resolve-*/` family (TRSLS, TRCAPS, TRGC, TRE, TRMLE, TROI, ...) | TSR-tier substrate completion |
| `diff-prod/` | Engagement-wide differential-prod methodology + fixtures |
| `cross-runtime-bench/` | CRB harness + fixtures |

---

## VI. How the loop closes — feedback paths

Each arrow in §I's diagram corresponds to a concrete artifact transition. The loop closes because each transition is bidirectional:

- **(1)→(2)** — substrate modifications trigger re-measurement at every chapter close (canonical workflow: `cargo build --release` → `diff-prod` → `cargo test` → corpus instruments → trajectory entry).
- **(2)→(3)** — measurement results feed into findings.md as new findings + (occasionally) new standing rules. The append-only protocol (Doc 727 §X basin-stability) prevents drift.
- **(3)→(4)** — discipline artifacts inform locale spawning (rule 11) + closure (rule 15) + substrate-bug recovery (rule 13). Locales record discipline application in their trajectory.
- **(4)→(1)** — locale work produces commits to the substrate. The substrate-tier crates evolve with each commit.

The loop's self-improving property:

- **(2) compounds** — each new measurement instrument enables previously-invisible signals (TXC surfacing the 90.1pp parse/execute gap is the canonical instance).
- **(3) compounds** — each new finding + standing rule reduces the search space for future moves (15 rules now prevent at least 11 bug classes; see predictive-ruleset.md §Predictive coverage map).
- **(4) compounds** — locale manifest grows; each locale's seed/trajectory pair becomes available as a precedent for future locales (cross-locale composes-with citations route this).
- **(1) compounds** — substrate-tier crates accumulate Pin-Art-derived correctness with each round.

---

## VII. The apparatus's standing observation patterns

These structural patterns recur across the apparatus's operation. They are codified as standing rules (see `docs/predictive-ruleset.md`) but worth surfacing here as apparatus-level observations:

- **Corpus-as-regression-instrument** (Finding IX.1): instrument-tier locales (TCC, TXC) serve dual roles — priority instrument AND regression instrument. Without the dual role, conservative-strip discipline (rule 14) would not be enforceable.

- **Inspect-then-iterate compound discovery** (Finding IX.3 / rule 15): planned substrate fixes routinely surface unplanned higher-impact substrate gaps when the post-fix failure-table top row is inspected. Reproduced 9+ times across the 2026-05-24 TS-parity arc; identified as a SIPE-T instance in Doc 541 Appendix E.

- **Multi-tier cascade-revival** (Doc 740): individual tier closures may produce near-zero or negative measurements alone; cumulative reclaim materializes at the deeper-layer closure. Rule 13's prospective application avoids paying the intermediate-tier substrate-introduction tax.

- **Resolver-instance boundary contract** (Doc 742): O1 (top-level dispatch) + O2 (downstream dispatch) + O3 (upstream elision); each independently measurable; the apparatus enables the measurement.

- **SIPE-T scale-invariance** (Doc 541 Appendix E): the apparatus's choice-points + conditioning + discipline + prompt structure recurs self-similarly across head and tail of the failure-table distribution.

---

## VIII. Standing corpus references

The apparatus is theoretically grounded at jaredfoy.com/resolve/. Load-bearing corpus docs that the apparatus implements:

| Doc | Articulates |
|---|---|
| **Doc 581** | Pin-Art apparatus + resume-vector discipline (seed/trajectory pair) |
| **Doc 727 §X** | Basin-stability append-only update protocol for findings.md |
| **Doc 729** | Resolver-instance pattern (the substrate's architectural target) |
| **Doc 730** | Vertical recurrence of the lowering-compiler closure across substrate tiers |
| **Doc 731** | The JIT as a lowering-compiler tier; alphabet-purity bound on JIT complexity |
| **Doc 732** | The package manager as resolver-instance #0 |
| **Doc 733** | Fractal seeds-and-trajectories: pair recurrence across substrate depth |
| **Doc 734** | The meta-resolution pipeline as operating instrument of the engagement recursion |
| **Doc 736** | Capability-passing runtime: architecturally-impossible supply chain attacks |
| **Doc 737** | The locale as coordinate: nested seed/trajectory pairs as substrate positions |
| **Doc 738** | The source identifier as coordinate: naming-convention encoding at the source tier |
| **Doc 540** | Pin-Art apparatus formalization |
| **Doc 541 Appendix E** | SIPE-T scale-invariance via cruftless instance |
| **Doc 739 / 740 / 741** | Cascade-revival series; rule 13's theoretical anchor + empirical materialization |
| **Doc 742** | Resolver-instance boundary contract (O1/O2/O3) — TSR-parity arc consolidation |

---

## IX. Update protocol

This doc is a comprehensive enumeration; it must stay in sync with the apparatus. Update when:

1. **A new instrument is built** — add to §II with path + observes + oracle.
2. **A new discipline artifact lands** — add to §III with path + role.
3. **A new locale is founded** — increment §IV's locale count (`scripts/locales/discover.sh` reports the new count).
4. **A new substrate tier crate is added** — add to §V's tier table.
5. **A new corpus doc lands that the apparatus implements** — add to §VIII.

Update protocol per Doc 727 §X: this doc is consolidated-view (not append-only). It tracks the CURRENT state of the apparatus; the canonical history lives in findings.md + corpus + commit log.

---

## X. Quick-reference index

For navigating the apparatus by role:

- **Want to spawn a new locale?** → `scripts/locales/CANDIDATES.md` (queue) + Doc 737 (theory) + §IV.spawn-protocol above.
- **Want to land a substrate move?** → `docs/predictive-ruleset.md` (rules) + AGENTS.md commit discipline + locale's seed §Pred-* falsifiers.
- **Want to add an instrument?** → §II above + Doc 581 (Pin-Art apparatus framing).
- **Want to add a standing rule?** → `pilots/rusty-js-jit/findings.md` next Addendum + this doc's §III update + `docs/predictive-ruleset.md` consolidated update.
- **Want to publish corpus?** → use the corpus publish pipeline (write to `~/corpus-master/corpus/` → mirror to `~/resolve/corpus/` + push → `bun run seed` from `~/jaredfoy/`).
- **Want to recover from a substrate-bug regression?** → Rule 13 + Rule 14 + Rule 15; see `docs/standing-rule-13-prospective-application.md`.

---

*Last updated 2026-05-24 (15 standing rules, 36 locales, 10 findings.md addenda, 14 corpus references). Update when any new apparatus piece lands per §IX.*
