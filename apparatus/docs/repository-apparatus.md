# Repository Apparatus for the Cybernetic Discipline

The cruftless engagement runs as a closed-loop cybernetic system: substrate work modifies the codebase, measurement instruments observe its behavior against external oracles, discipline artifacts constrain what counts as a valid move, and the locale structure both executes the work and accumulates the record of what was attempted. This doc enumerates every piece of the apparatus, organized by role in the feedback loop, with explicit pointers to where each piece lives in the repo + which corpus doc articulates its theory.

The apparatus is comprehensive — every standing artifact this engagement uses is listed here. New artifacts should be added to this doc as they are created.

---

## 0. Canonical tier separation: `apparatus/` vs `docs/`

The repository is partitioned into two top-level documentation-and-state surfaces, which differ in **who reads them, when, and for what purpose**. The distinction is load-bearing; conflating the two breaks the dyadic operating model.

| Surface | Tier | Consumer | When read | Load-bearing on the loop |
|---|---|---|---|---|
| `apparatus/` | rung-1 (machine) | agent (LLM resolver) | every loop iteration; required reading on entry | yes, the closed-loop substrate |
| `docs/` | rung-2 (keeper supplement) | keeper (human) | as the keeper thinks; agent reads only on explicit request | no, sidecar for development |

### `apparatus/` — the closed-loop substrate

`apparatus/` is the apparatus-tier tooling and output that **directly informs the cybernetic loop**. Every artifact here is read by the agent on each loop iteration as part of orienting to a locale, evaluating a move, or applying a discipline. The agent's operating context must include the relevant `apparatus/` content; absent it, the loop drifts.

Layout:

```
apparatus/
├── docs/                                            # apparatus-tier prose
│   ├── repository-apparatus.md                      # this doc, the apparatus enumeration
│   ├── ecma-conformance-parity-as-exhaustive-language-behavior-dag.md # parity -> decision basis -> computable inference
│   ├── predictive-ruleset.md                        # consolidated 15-rule predictive view
│   ├── standing-rule-13-prospective-application.md  # rule 13 revert-then-deeper-layer thesis
│   ├── deletions-ledger.md                          # append-only record of constraint-induced deletions
│   └── agent-feedback-schema.md                     # per-locale cross-resolver review schema
│
└── locales/                                         # locale-tier registry + tool
    ├── manifest.json                                # enumerated locale instances
    ├── CANDIDATES.md                                # next-spawn queue (Tier-A / hypothetical)
    ├── discover.sh                                  # tool that maintains the manifest
    └── README.md
```

`apparatus/docs/` is not "documentation in the colloquial sense." It is the agent's required pre-reading: the primary articulation that states what the apparatus is for (ecma-conformance-parity-as-exhaustive-language-behavior-dag.md), the schemas the agent must follow when authoring (agent-feedback-schema.md), the rules the agent applies when evaluating moves (predictive-ruleset.md, standing-rule-13), and the enumeration the agent consults to understand the loop it is participating in (this doc).

`apparatus/locales/` is the agent-readable registry of the locale-coordinate space (Doc 737). The manifest is consulted to find existing locales; CANDIDATES.md is consulted before founding a new one; discover.sh is run to refresh the manifest after a spawn.

### Bilateral pilot tier: apparatus-pilots vs substrate-pilots

Per keeper directive 2026-05-25 (Telegram 9804): within `pilots/`, the engagement carries a bilateral boundary between **apparatus-pilots** (cybernetic-loop instruments, measurement apparatus, meta-discipline) and **substrate-pilots** (engine-tier code work). This boundary is made visible in the directory tree:

```
pilots/
├── apparatus/                      # apparatus-pilots: cybernetic-loop instruments + meta-discipline
│   ├── test262-categorize/         # Pin-Art matrix categorizer; produces matrix.md
│   ├── diff-prod/                  # engagement-wide differential-prod methodology + fixtures
│   ├── cross-runtime-bench/        # CRB harness + per-fixture probes
│   ├── ts-consumer-corpus/         # TCC parse-parity measurement instrument
│   ├── ts-execute-corpus/          # TXC execute-parity measurement instrument
│   └── locale-positioning-audit/   # meta-apparatus: locale-graph claim-coherence audit
│
└── <substrate-pilot>/              # substrate-pilots: engine-tier code work (default)
    ├── rusty-js-{ast, parser, bytecode, runtime, gc, jit, shapes, ir, pm, caps}/
    ├── <per-feature-locale>/       # e.g. lexer-goal-symbol-selection/, parser-precedence-in-flag/
    ├── <per-surface-pilot>/        # e.g. tls/, web-crypto/, fetch-api/, ...
    └── <coordinate-spawn>/         # e.g. temporal-availability/, typed-array-wrong-result/
```

Both pilot kinds are rung-1 substrate per the apparatus/-vs-docs/ tier-separation above; the new boundary is INSIDE rung-1, distinguishing **what the pilot produces**:

- **Apparatus-pilots produce instruments + apparatus output** (matrix.md, manifest.json refinements, audit findings, parity measurements). Their telos is the discipline of the cybernetic loop itself.
- **Substrate-pilots produce engine code** (parser rules, bytecode ops, runtime intrinsics, AST shapes). Their telos is conformance/correctness at the language-substrate tier.

A pilot is apparatus iff its primary output is consumed by other pilots' substrate work (test262-categorize's matrix is consumed by every coordinate-shaped locale; diff-prod's gate is consumed at every substrate move's landing). A pilot is substrate iff its primary output is engine code that satisfies conformance probes. The discriminator is the consumer of the pilot's output, not its own internal complexity.

Migration history: this boundary was introduced 2026-05-25 by relocating 6 then-extant apparatus-pilots from `pilots/<name>/` to `pilots/apparatus/<name>/`. Substrate pilots stay at `pilots/<name>/` (the implicit default). New pilots adopt the convention at spawn: apparatus-shaped goes to `pilots/apparatus/<name>/`; substrate-shaped goes to `pilots/<name>/`.

### `docs/` — the dyadic sidecar

`docs/` is the **sidecar for development that the keeper utilizes in the cybernetic dyad**. It is the keeper's thinking surface: live engineering analyses, in-flight phase designs, and the read-only mirror of the published RESOLVE corpus that the keeper composes against. The agent reads from `docs/` only when the keeper explicitly directs it (e.g., "read Doc 736") or when the agent's task requires composing against a specific corpus articulation.

Layout:

```
docs/
├── corpus-ref/   # 81 RESOLVE corpus docs (Doc 123-741), the keeper's published reference
└── engagement/   # this-engagement-specific keeper analyses, designs, and prospective drafts
    ├── prospective/                                  # in-flight corpus-candidate drafts
    ├── arktype-deep-trace.md                         # live trace working doc
    ├── cluster-phase-design.md                       # phase-specific design
    ├── derivation-inversion-on-bun-tests.md          # planning doc
    ├── invert-phase-design.md                        # phase-specific design
    ├── pipeline.md                                   # derive-constraints CLI orchestration
    ├── porting-md-analysis.md                        # live engineering analysis
    ├── seam-detection-design.md                      # specific tool design
    └── xv-audit-bun-dual-class.md                    # §XV audit working doc
```

`docs/corpus-ref/` is a sidecar mirror of the published corpus at jaredfoy.com/resolve/. The corpus is the keeper's prior work; the mirror exists so the keeper can compose against specific docs without leaving the repo, and so the agent can be pointed at a specific Doc N when the keeper deems it necessary.

`docs/engagement/` is the keeper's working surface for this engagement. Designs land here while the keeper is shaping them; promotions to corpus articulation, or to apparatus-tier discipline, happen as separate moves. The agent treats `docs/engagement/` as keeper context, not as agent-loop input.

### Why the separation matters

The cybernetic loop succeeds when every iteration reads from a stable, minimal, agent-consumable substrate (`apparatus/`). The loop degrades when agent reads expand to include keeper-tier prose, because keeper prose is intentionally exploratory and not yet crystallized into discipline. The dyadic ascent (Doc 711) requires rung-1 and rung-2 stay distinct: the agent operates the rung-1 substrate; the keeper supplements via the rung-2 sidecar; promotions of rung-2 content to rung-1 happen through explicit keeper directives that move artifacts from `docs/` into `apparatus/` (or into a locale's seed.md).

### Promotion path

Three canonical promotions move material from sidecar to substrate:

1. `docs/engagement/<analysis>.md` ➜ `apparatus/docs/<schema-or-rule>.md` when an analysis crystallizes into a reusable schema or standing rule.
2. `docs/engagement/prospective/<draft>.md` ➜ `docs/corpus-ref/NNN-<title>.md` when a draft is promoted to corpus publication (typically via the corpus publish pipeline).
3. `docs/engagement/<phase-design>.md` ➜ inline content of `pilots/<locale>/seed.md` when a phase design crystallizes into a founded locale.

Promotions are explicit keeper moves; the agent does not promote unilaterally.

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
| **CRB (cross-runtime bench)** | `pilots/apparatus/cross-runtime-bench/scripts/run-bench.sh` | Wall-clock per-fixture vs node + bun; median of N runs | node v22.22.0 + bun 1.3.11 |
| **diff-prod** | `scripts/diff-prod/run-all.sh` | Byte-identical stdout per fixture under cruft vs bun | bun 1.3.11; 42-fixture suite |
| **test262-sample** | `scripts/test262-sample/run-sample.sh` | ECMA-262 conformance via the official test suite | TC39 test262; 7,589-test curated representative sample |
| **canonical fuzz** | `pilots/rusty-js-shapes/consumer-migration/fixtures/fuzz-canonical.mjs` | Deterministic accumulator over 2000 randomized invocations | node acc=−932188103 (version `cmig-ext-17-2026-05-23`) |

### Locale-tier corpus instruments

| Instrument | Path | Observes | Categorization |
|---|---|---|---|
| **TCC (TypeScript consumer corpus, parse-parity)** | `pilots/apparatus/ts-consumer-corpus/derived/src/bin/measure.rs` | `ts_resolve::parse_and_erase` outcome per real npm `.ts` file | OK / STRIP / PARSE / PANIC; structural-tag heuristic |
| **TXC (TypeScript execute corpus, execute-parity)** | `pilots/apparatus/ts-execute-corpus/derived/src/bin/measure.rs` | Exit-status comparison under bun + cruft per fixture | MATCH / DIVERGE / BUN_FAIL / CRUFT_FAIL / SETUP_FAIL / TIMEOUT |
| **TCC manifest** | `pilots/apparatus/ts-consumer-corpus/manifest/packages.json` + `file-hashes.json` | Hash-pinned corpus of 374 `.ts` files from rxjs + ajv + pino | reproducibility instrument |

### Component-decomposition probes

| Instrument | Path | Observes | Used by |
|---|---|---|---|
| **JSF A/B probe** | `pilots/rusty-js-json-fast/fixtures/component-ab-probe.mjs` | 5 additive variants × 50-iter warmup × 500-iter measurement | Rule 11 axis A1 |
| **string_url_sweep A/B probe** | `pilots/apparatus/cross-runtime-bench/fixtures/string_url_sweep/component-ab-probe.mjs` | Header-loop dominator decomposition | IHI/GPI/IPBR locales |

### Self-validation instruments

| Instrument | Path | Observes |
|---|---|---|
| **derive-constraints apparatus** | `derive-constraints/` | Extract → cluster → invert pipeline for spec-constraint discovery |
| **locale manifest discoverer** | `apparatus/locales/discover.sh` → `apparatus/locales/manifest.json` | Filesystem walk of all `seed.md` files; produces coordinate manifest |

---

## III. (3) Discipline artifacts — what constrains

The discipline is recorded in artifacts that future substrate work must consult. These are the conditioning corpus + discipline set per the SIPE-T mapping (see Doc 541 Appendix E).

### Canonical findings + rules

| Artifact | Path | Role |
|---|---|---|
| **findings.md** | `pilots/rusty-js-jit/findings.md` | Canonical append-only ledger; 26 findings + 15 standing rules across 10 addenda |
| **predictive-ruleset.md** | `apparatus/docs/predictive-ruleset.md` | Consolidated derived view of the 15 standing rules as falsifiable predictions |
| **standing-rule-13-prospective-application.md** | `apparatus/docs/standing-rule-13-prospective-application.md` | The revert-then-deeper-layer-closure thesis with 12 prospective corroborations |
| **deletions-ledger.md** | `apparatus/docs/deletions-ledger.md` | Append-only record of constraint-induced deletions: what got removed, the named upstream constraint that made deletion safe, the tier/coordinate that became cleaner. Deletions are first-class substrate moves per keeper directive 2026-05-25; the ledger restores trajectory-binding for deleted code that git history alone cannot preserve. |

### Locale-tier discipline

| Artifact | Path | Role |
|---|---|---|
| **CANDIDATES.md** | `apparatus/locales/CANDIDATES.md` | Tiered queue of prospective locales; spawn-protocol + status legend |
| **locale manifest** | `apparatus/locales/manifest.json` | Authoritative inventory; load-bearing record of the coordinate space per Doc 737 |

### Orientation + policy

| Artifact | Path | Role |
|---|---|---|
| **AGENTS.md / CLAUDE.md** | `AGENTS.md`, `CLAUDE.md` (identical) | Project identity + workspace layout + commit-and-authorization discipline + standing corpus references |
| **ecma-conformance-parity-as-exhaustive-language-behavior-dag.md** | `apparatus/docs/ecma-conformance-parity-as-exhaustive-language-behavior-dag.md` | Primary telos: conformance pressure extracts a closed decision basis that can become spec-corresponded computable inference |
| **THIS DOC (repository-apparatus.md)** | `apparatus/docs/repository-apparatus.md` | Comprehensive enumeration of the apparatus + cybernetic-loop framing |

### Per-locale discipline (recurrent)

Every locale carries its own discipline artifacts per Doc 581 + Doc 733:

| Artifact | Path pattern | Role |
|---|---|---|
| **seed.md** | `pilots/<locale>/seed.md` | Telos + constraints (Cn) + falsifiers (Pred-n) + methodology + carve-outs + composes-with |
| **trajectory.md** | `pilots/<locale>/trajectory.md` | Append-only round-by-round record; each round describes a substrate move with empirical outcome |
| **analysis.md** | `pilots/<locale>/analysis.md` | Contingent: diff-prod empirical cross-reference mapping the locale's scope to the 112-fixture PASS/FAIL vector + named mechanism gaps. Present for all non-apparatus locales (125 total). Agents consult this after seed + trajectory to ground substrate decisions in empirical data. |
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
    analysis.md          ← diff-prod empirical cross-reference (contingent)
    docs/                ← per-locale design docs (optional)
    fixtures/            ← per-locale test fixtures (optional)
    derived/             ← Pin-Art-derived Rust crate (where applicable)
      Cargo.toml
      src/
```

### Current locale count

36 locales as of 2026-05-24 (25 top-level, 11 nested) — see `apparatus/locales/manifest.json` for the authoritative list.

### Locale spawn protocol (per Doc 737 + standing rules 11 + 23)

1. Confirm the workstream's multi-rung shape (Doc 737 §II promotion threshold).
2. Run rule 11's 5-axis pre-spawn coverage check (A1 component A/B is load-bearing).
3. Create `pilots/<name>/seed.md` with telos / constraints / falsifiers / methodology / carve-outs / composes-with.
4. Run `apparatus/locales/discover.sh`; commit the refreshed manifest in the same change as the seed.
5. **EXT 0 baseline-inspection rung (standing rule 23, locale-as-probe discipline)**: at founding, before declaring the substrate move-shape, MEASURE the locale's failure-shape against current cruft + INSPECT a sample of failures to verify the substrate move actually lives at the declared coordinate. If baseline-inspection reveals the move-shape is at a DIFFERENT coordinate than the seed initially declared, treat the locale as a probe that surfaced the real target — apply R13 prospective C1-C4 to the surfaced target and land it as the substrate move (the original locale becomes the validating test surface for the surfaced move).
6. Begin substrate work at round EXT 1 (first substrate move; may be at the surfaced coordinate per step 5) → EXT N (close).

### Locale resume protocol

To pick up work on any locale:
1. Read `seed.md` first (telos + apparatus + methodology).
2. Read `trajectory.md` tail (most recent rounds).
3. Read `analysis.md` if present (diff-prod empirical cross-reference: which fixtures exercise the scope, PASS/FAIL state, mechanism gaps).
4. The seed + trajectory pair is sufficient for operational context (Doc 581 resume-vector discipline). The analysis.md adds empirical grounding: it tells the agent whether the engine is correct, divergent, or untested at the locale's surface before any substrate move begins.

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

These structural patterns recur across the apparatus's operation. They are codified as standing rules (see `apparatus/docs/predictive-ruleset.md`) but worth surfacing here as apparatus-level observations:

- **Corpus-as-regression-instrument** (Finding IX.1): instrument-tier locales (TCC, TXC) serve dual roles — priority instrument AND regression instrument. Without the dual role, conservative-strip discipline (rule 14) would not be enforceable.

- **Inspect-then-iterate compound discovery** (Finding IX.3 / rule 15): planned substrate fixes routinely surface unplanned higher-impact substrate gaps when the post-fix failure-table top row is inspected. Reproduced 9+ times across the 2026-05-24 TS-parity arc; identified as a SIPE-T instance in Doc 541 Appendix E.

- **Multi-tier cascade-revival** (Doc 740): individual tier closures may produce near-zero or negative measurements alone; cumulative reclaim materializes at the deeper-layer closure. Rule 13's prospective application avoids paying the intermediate-tier substrate-introduction tax.

- **Resolver-instance boundary contract** (Doc 742): O1 (top-level dispatch) + O2 (downstream dispatch) + O3 (upstream elision); each independently measurable; the apparatus enables the measurement.

- **SIPE-T scale-invariance** (Doc 541 Appendix E): the apparatus's choice-points + conditioning + discipline + prompt structure recurs self-similarly across head and tail of the failure-table distribution.

- **Conformance-as-decision-basis extraction** (`apparatus/docs/ecma-conformance-parity-as-exhaustive-language-behavior-dag.md`): full parity is not only a pass metric; it is the empirical closure condition that forces the ECMAScript implementation decision basis into explicit coordinates. Spec correspondence and computable traversal lift that basis into a guide for future standards implementation.

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
3. **A new locale is founded** — increment §IV's locale count (`apparatus/locales/discover.sh` reports the new count).
4. **A new substrate tier crate is added** — add to §V's tier table.
5. **A new corpus doc lands that the apparatus implements** — add to §VIII.

Update protocol per Doc 727 §X: this doc is consolidated-view (not append-only). It tracks the CURRENT state of the apparatus; the canonical history lives in findings.md + corpus + commit log.

---

## X. Quick-reference index

For navigating the apparatus by role:

- **Want to spawn a new locale?** → `apparatus/locales/CANDIDATES.md` (queue) + Doc 737 (theory) + §IV.spawn-protocol above.
- **Want to land a substrate move?** → `apparatus/docs/predictive-ruleset.md` (rules) + AGENTS.md commit discipline + locale's seed §Pred-* falsifiers.
- **Want to understand the apparatus telos?** → `apparatus/docs/ecma-conformance-parity-as-exhaustive-language-behavior-dag.md` (parity -> decision basis -> spec correspondence -> computable inference).
- **Want to add an instrument?** → §II above + Doc 581 (Pin-Art apparatus framing).
- **Want to add a standing rule?** → `pilots/rusty-js-jit/findings.md` next Addendum + this doc's §III update + `apparatus/docs/predictive-ruleset.md` consolidated update.
- **Want to publish corpus?** → use the corpus publish pipeline (write to `~/corpus-master/corpus/` → mirror to `~/resolve/corpus/` + push → `bun run seed` from `~/jaredfoy/`).
- **Want to recover from a substrate-bug regression?** → Rule 13 + Rule 14 + Rule 15; see `apparatus/docs/standing-rule-13-prospective-application.md`.

---

*Last updated 2026-05-24 (15 standing rules, 36 locales, 10 findings.md addenda, 14 corpus references). Update when any new apparatus piece lands per §IX.*
