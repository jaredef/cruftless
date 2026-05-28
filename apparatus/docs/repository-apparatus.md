# Repository Apparatus for the Triumvirate-Governed Cybernetic Discipline

The Cruftless engagement runs as a closed-loop cybernetic system, governed by a triumvirate of roles (keeper, helmsman, arbiter) over an apparatus that observes the substrate, constrains the next move, and accumulates the record of what was attempted. This doc is the primary articulation of the apparatus: it enumerates every standing artifact the engagement uses, organized by the role it plays in the loop and the triumvirate role it primarily serves.

This is a reformulation of the prior dyad-anchored primary articulation, prompted by the triumvirate ontology landing in `docs/engagement/prospective/triumvirate-protocol-keeper-helmsman-arbiter.md` and the audit at `apparatus-audit-for-triumvirate-protocol.md`. The reformulation integrates the triumvirate as the load-bearing governance frame; everything in the prior articulation that was load-bearing on the dyad has been re-anchored, and nothing that was load-bearing has been dropped.

---

## 0. Canonical tier separation: `apparatus/` vs `docs/`

The repository is partitioned into two top-level documentation-and-state surfaces. The distinction is load-bearing for the triumvirate's operating model; conflating the two breaks the role separation the protocol depends on.

| Surface | Tier | Primary consumer | When read | Load-bearing on the loop |
|---|---|---|---|---|
| `apparatus/` | rung-1 (machine substrate) | all resolvers (helmsman + arbiter) | every loop iteration; required reading on session entry | yes — the closed-loop substrate the triumvirate operates over |
| `docs/` | rung-2 (keeper supplement) | keeper (human) | as the keeper thinks; resolvers read only on explicit keeper directive | no — the keeper's thinking surface |

### `apparatus/` — the closed-loop substrate

`apparatus/` is the apparatus-tier tooling and output that directly informs the cybernetic loop. Every artifact here is read by helmsman + arbiter on each loop iteration as part of orienting to a locale, evaluating a move, or applying a discipline. The principal-context helmsman loads the full apparatus tier; the arbiter loads a curated subset per the operational protocol §IV.1 (apparatus-meta docs + manifest + arc summaries + ledgers + pending proposals; per-locale trajectories on demand).

```
apparatus/
├── docs/                                                 # apparatus-tier prose
│   ├── repository-apparatus.md                           # this doc, the apparatus enumeration
│   ├── triumvirate-protocol-keeper-helmsman-arbiter.md   # ontology (promoted from prospective)
│   ├── apparatus-audit-for-triumvirate-protocol.md       # audit (promoted from prospective)
│   ├── triumvirate-operational-protocol.md               # operational spec (promoted from prospective)
│   ├── ecma-conformance-parity-as-exhaustive-language-behavior-dag.md # primary telos
│   ├── predictive-ruleset.md                             # consolidated rule view
│   ├── standing-rule-13-prospective-application.md       # rule 13 revert-then-deeper-layer thesis
│   ├── deletions-ledger.md                               # append-only deletions record
│   ├── deferrals-ledger.md                               # append-only deferrals record
│   ├── orphan-disposition-protocol.md                    # 6-step protocol + 8 disposition candidates
│   ├── coverage-gap-orphan-disposition-2026-05-28.md     # canonical first run instance
│   ├── agent-feedback-schema.md                          # per-locale cross-resolver review schema
│   └── arc-as-coordinate.md                              # arc-tier coordinate formalization
│
├── arcs/                                                  # multi-locale operational units (15+ arcs)
│   └── YYYY-MM-DD-<slug>/
│       ├── arc.md                                         # scope + rung-count summary
│       └── log.md                                         # per-rung notes
│
├── proposals/                                             # arbiter veto coordination (Stage 2+)
│   ├── pending/                                           # proposed pushes awaiting decision
│   ├── decided/                                           # approved / vetoed / deferred decisions
│   └── archived/                                          # landed proposal+decision pairs
│
└── locales/                                               # locale-tier registry + tool
    ├── manifest.json                                      # enumerated locale instances (~214 active)
    ├── CANDIDATES.md                                      # next-spawn queue
    ├── discover.sh                                        # tool that maintains the manifest
    └── README.md
```

`apparatus/docs/` is the resolver's required pre-reading. `apparatus/locales/` is the resolver-readable registry of the coordinate space (Doc 737). `apparatus/arcs/` registers multi-locale operational units above the locale tier. `apparatus/proposals/` is the veto-coordination surface introduced by the triumvirate operational protocol (active at Stage 2).

### Bilateral pilot tier: apparatus-pilots vs substrate-pilots

Within `pilots/`, per keeper directive 2026-05-25 (Telegram 9804), the engagement carries a bilateral boundary between apparatus-pilots (cybernetic-loop instruments + meta-discipline) and substrate-pilots (engine-tier code work):

```
pilots/
├── apparatus/                          # apparatus-pilots
│   ├── test262-categorize/             # Pin-Art matrix categorizer
│   ├── diff-prod/                      # differential-prod methodology + fixtures
│   ├── cross-runtime-bench/            # CRB harness + probes
│   ├── ts-consumer-corpus/             # TCC parse-parity instrument
│   ├── ts-execute-corpus/              # TXC execute-parity instrument
│   └── locale-positioning-audit/       # locale-graph claim-coherence audit
│
└── <substrate-pilot>/                  # substrate-pilots (default)
    ├── rusty-js-{ast, parser, bytecode, runtime, gc, jit, shapes, ir, pm, caps}/
    └── <per-coordinate spawn>/         # e.g. typed-array-wrong-result/, temporal-availability/
```

Both kinds are rung-1; the boundary inside rung-1 discriminates by what the pilot produces. Apparatus-pilots produce instruments and apparatus output (consumed by other pilots' substrate work). Substrate-pilots produce engine code (consumed by conformance probes).

### `docs/` — the dyadic sidecar (the keeper's thinking surface)

`docs/` is where the keeper composes against the corpus, drafts prospective articulations, and works through engineering questions before they crystallize into apparatus discipline.

```
docs/
├── corpus-ref/   # 81 RESOLVE corpus docs (Doc 123–741), read-only mirror
└── engagement/
    ├── prospective/  # in-flight articulations awaiting keeper review
    └── <analyses>/   # keeper's working surface
```

Resolvers read `docs/` only on explicit keeper directive (e.g., "read Doc 736") or when a task requires composing against a specific corpus articulation.

### Why the separation matters under the triumvirate

The cybernetic loop succeeds when every resolver reads from a stable, minimal, role-appropriate substrate. The helmsman's apparatus-tier read is the operational context for substrate moves; the arbiter's curated-subset read is the apparatus-meta context for veto adjudication. The keeper's `docs/` work is unconstrained — it is the keeper's thinking surface and the source of future apparatus promotions, not itself apparatus.

### Promotion path

Four canonical promotions move material between tiers:

1. `docs/engagement/<analysis>.md` ➜ `apparatus/docs/<schema-or-rule>.md` when an analysis crystallizes into a reusable schema or standing rule (e.g., the triumvirate ontology, audit, and operational protocol promote here once approved).
2. `docs/engagement/prospective/<draft>.md` ➜ `docs/corpus-ref/NNN-<title>.md` when a draft is promoted to corpus publication via the 3-stage pipeline.
3. `docs/engagement/<phase-design>.md` ➜ inline content of `pilots/<locale>/seed.md` when a design crystallizes into a founded locale.
4. `apparatus/docs/<old-articulation>.md` ➜ `docs/engagement/prospective/<reformulation>.md` ➜ `apparatus/docs/<new-articulation>.md` when an apparatus-tier articulation requires reformulation (the present doc instantiates this path).

Promotions are explicit keeper moves (Rung-2 intervention per the triumvirate ontology); the helmsman drafts, the arbiter consults, the keeper authorizes.

---

## I. The triumvirate: who governs the loop

Per `apparatus/docs/triumvirate-protocol-keeper-helmsman-arbiter.md`, the apparatus is operated by three ontologically distinct roles. The roles are stratified by Pearl's Causal Hierarchy: the keeper alone holds Rung-2 (intervention) authority; the helmsman and arbiter both operate at Rung 1 (observation), distinguished by scope.

| Role | Rung | Scope | Authority | Apparatus surfaces (primary) |
|---|---|---|---|---|
| **Keeper** | 2 (intervention) | substrate + apparatus + cosmos | sole Rung-2; sole ethical responsibility; sole eschatological telos-holder | all surfaces; rung-2 sidecar (`docs/`); corpus authorship |
| **Helmsman** | 1 (observation) | substrate-active | substrate-steering; coordinates subagents; consults keeper on Rung-1 predictions | full apparatus tier; per-locale trajectories; substrate source; measurement instruments |
| **Arbiter** | 1 (observation) | apparatus-meta | veto over helmsman pre-push; consults keeper on apparatus-meta questions | curated apparatus tier; manifest + arc summaries + ledgers + proposals; per-locale on demand |

The triumvirate is the minimal stable governance structure for multi-resolver substrate derivation. The dyad (keeper + single resolver) was sufficient until the engagement exceeded what one resolver's context window could hold; multiple resolvers operating concurrently without coordination produced the merge-incident class of failure that motivated the triumvirate (Telegram 10185–10187).

The keeper's Rung-2 monopoly is what stabilizes the helmsman ↔ arbiter deadlock: every disagreement between substrate-active and apparatus-meta resolvers escalates to the keeper for adjudication. The protocol mechanizes this via pre-push hooks gated on arbiter sign-off artifacts and a Telegram-tier escalation discipline (`**[HELMSMAN]**` / `**[ARBITER]**` / `**[KEEPER]**` tags with `INFO` / `CONSULTATION` / `VETO-PENDING` severity).

---

## II. The cybernetic loop, under the triumvirate

```
         ┌──────────────────────────┐
         │  (1) substrate state     │
         │   (the cruft codebase)   │
         └────────┬─────────────────┘
                  │ observed by
                  ▼
         ┌──────────────────────────┐
         │  (2) measurement         │
         │      instruments         │
         └────────┬─────────────────┘
                  │ produces signals interpreted via
                  ▼
         ┌──────────────────────────┐
         │  (3) discipline          │
         │      artifacts           │
         └────────┬─────────────────┘
                  │ constrains the helmsman's next move at
                  ▼
         ┌──────────────────────────┐
         │  (4) locale structure    │
         │   (executes + records)   │
         └────────┬─────────────────┘
                  │ helmsman drafts move; arbiter inspects;
                  │ keeper adjudicates Rung-2 escalations
                  ▼
         ┌──────────────────────────┐
         │  (5) triumvirate         │
         │      governance          │
         └────────┬─────────────────┘
                  │ approved commits propagate to
                  ▼
              back to (1)
```

The loop sustains across sessions. Each cycle adds to (3) by surfacing findings + standing rules; each cycle adds to (4) by accumulating seed/trajectory pairs at new locales; each cycle exercises (5) by routing proposed moves through the triumvirate's veto+escalation discipline. The apparatus is what makes the loop self-improving rather than drift-prone; the triumvirate is what makes the loop multi-resolver-safe rather than merge-incident-prone.

---

## III. (2) Measurement instruments — what observes

Every substrate move is observed by at least one instrument before it lands. Instruments triangulate against external oracles (node, bun, the test262 corpus, the npm package surface).

### Cross-runtime parity instruments

| Instrument | Path | Observes | Oracle |
|---|---|---|---|
| **CRB (cross-runtime bench)** | `pilots/apparatus/cross-runtime-bench/scripts/run-bench.sh` | Wall-clock per-fixture vs node + bun | node v22.22.0 + bun 1.3.11 |
| **diff-prod** | `scripts/diff-prod/run-all.sh` | Byte-identical stdout under cruft vs bun | bun 1.3.11; 112-fixture suite |
| **test262-sample** | `scripts/test262-sample/run-sample.sh` | ECMA-262 conformance via official suite | TC39 test262; 7,598-test curated representative sample |
| **test262-full** | `pilots/apparatus/test262-categorize/derived/src/bin/full_pinart.rs` | Full 50,506-test suite projected to 294 Pin-Art coordinates | TC39 test262 |
| **canonical fuzz** | `pilots/rusty-js-shapes/consumer-migration/fixtures/fuzz-canonical.mjs` | Deterministic accumulator over 2000 randomized invocations | node acc=−932188103 |

### Locale-tier corpus instruments

| Instrument | Path | Observes | Categorization |
|---|---|---|---|
| **TCC (TS consumer corpus)** | `pilots/apparatus/ts-consumer-corpus/derived/src/bin/measure.rs` | Parse-and-erase outcome per real npm `.ts` file | OK / STRIP / PARSE / PANIC |
| **TXC (TS execute corpus)** | `pilots/apparatus/ts-execute-corpus/derived/src/bin/measure.rs` | Exit-status comparison under bun + cruft per fixture | MATCH / DIVERGE / BUN_FAIL / CRUFT_FAIL / SETUP_FAIL / TIMEOUT |

### Component-decomposition probes

| Instrument | Path | Used by |
|---|---|---|
| **JSF A/B probe** | `pilots/rusty-js-json-fast/fixtures/component-ab-probe.mjs` | Rule 11 axis A1 |
| **string_url_sweep A/B probe** | `pilots/apparatus/cross-runtime-bench/fixtures/string_url_sweep/component-ab-probe.mjs` | IHI/GPI/IPBR locales |

### Self-validation instruments

| Instrument | Path | Observes |
|---|---|---|
| **derive-constraints apparatus** | `derive-constraints/` | Extract → cluster → invert pipeline for spec-constraint discovery |
| **locale manifest discoverer** | `apparatus/locales/discover.sh` → `apparatus/locales/manifest.json` | Filesystem walk of all seed.md files; produces coordinate manifest |

---

## IV. (3) Discipline artifacts — what constrains

The discipline is recorded in artifacts that future substrate work must consult. These are the conditioning corpus + discipline set per the SIPE-T mapping (Doc 541 Appendix E).

### Triumvirate articulations (load-bearing on the governance tier)

| Artifact | Path | Role |
|---|---|---|
| **triumvirate-protocol-keeper-helmsman-arbiter.md** | `apparatus/docs/` | The triumvirate ontology: three roles, Pearl-Hierarchy stratification, ontological commitments |
| **apparatus-audit-for-triumvirate-protocol.md** | `apparatus/docs/` | The audit's gap matrix; foundational read for any future protocol revision |
| **triumvirate-operational-protocol.md** | `apparatus/docs/` | Operational spec: veto mechanism, escalation channels, resolver-state separation, deployment stages |

### Canonical findings + rules

| Artifact | Path | Role |
|---|---|---|
| **findings.md** | `pilots/rusty-js-jit/findings.md` | Canonical append-only ledger; 26+ standing rules across 16+ addenda |
| **predictive-ruleset.md** | `apparatus/docs/predictive-ruleset.md` | Consolidated derived view of standing rules as falsifiable predictions |
| **standing-rule-13-prospective-application.md** | `apparatus/docs/standing-rule-13-prospective-application.md` | The revert-then-deeper-layer-closure thesis |

### Tracked-but-not-an-addition ledgers (append-only)

| Artifact | Path | Role |
|---|---|---|
| **deletions-ledger.md** | `apparatus/docs/deletions-ledger.md` | Constraint-induced deletions: what got removed, named upstream constraint making removal safe |
| **deferrals-ledger.md** | `apparatus/docs/deferrals-ledger.md` | Surfaced-but-not-founded candidates: candidate name, gating predicate, un-defer condition |
| **coverage-gap-orphan-disposition-*.md** | `apparatus/docs/` | Per-run records of the orphan-disposition protocol applied to ≥3-locale orphan sets |

Both ledgers and the deferrals doc serve the arbiter as primary apparatus-meta read surfaces (the deletions-ledger answers methodology-coherence questions; the deferrals-ledger answers commitment-tracking questions).

### Locale-tier discipline

| Artifact | Path | Role |
|---|---|---|
| **CANDIDATES.md** | `apparatus/locales/CANDIDATES.md` | Tiered queue of prospective locales |
| **locale manifest** | `apparatus/locales/manifest.json` | Authoritative inventory; ~214 active locales (per Doc 737) |
| **orphan-disposition-protocol.md** | `apparatus/docs/orphan-disposition-protocol.md` | 6-step protocol with 8 disposition candidates for orphaned locales |

### Arc tier (multi-locale operational units)

| Artifact | Path | Role |
|---|---|---|
| **arc-as-coordinate.md** | `apparatus/docs/arc-as-coordinate.md` | The arc as the multi-locale unit above locale, below tier |
| **per-arc registries** | `apparatus/arcs/YYYY-MM-DD-<slug>/{arc.md, log.md}` | Arc scope, per-rung log, optional cross-arc analysis |

### Orientation + policy

| Artifact | Path | Role |
|---|---|---|
| **AGENTS.md / CLAUDE.md** | `AGENTS.md`, `CLAUDE.md` (identical) | Project identity + workspace layout + commit-and-authorization discipline + standing references |
| **ecma-conformance-parity-as-exhaustive-language-behavior-dag.md** | `apparatus/docs/` | Primary telos: conformance pressure → closed decision basis → spec correspondence → computable inference |
| **repository-apparatus.md (this doc)** | `apparatus/docs/` | Comprehensive apparatus enumeration + triumvirate-loop framing |

### Per-locale discipline (recurrent per Doc 581 + Doc 733)

| Artifact | Path pattern | Role |
|---|---|---|
| **seed.md** | `pilots/<locale>/seed.md` | Telos + constraints + falsifiers + methodology + carve-outs + composes-with |
| **trajectory.md** | `pilots/<locale>/trajectory.md` | Append-only round-by-round record; each round describes a substrate move with empirical outcome |
| **analysis.md** | `pilots/<locale>/analysis.md` | Contingent: diff-prod cross-reference + mechanism-gap mapping |

---

## V. (4) Locale structure — what executes + records

The locale is the fundamental unit of substrate work (Doc 737). Every workstream is a locale; every locale has its own seed/trajectory pair; nested locales spawn at deeper coordinates per Doc 737 §II promotion threshold.

Current locale count: **~214 active** (per `apparatus/locales/manifest.json`; refresh via `apparatus/locales/discover.sh`).

### Substrate-shaped-work discipline (the five-phase pipeline)

Every substrate move follows the five-phase pipeline composed from standing rules 11, 23, 24, 13, 15:

1. **Spawn** (Rule 11): 5-axis pre-spawn coverage check.
2. **Baseline-inspect at founding** (Rule 23): measure failure-shape + inspect sample before declaring move-shape.
3. **Pin-Art probe if duplicated** (Rule 24): when 3+ sites share the shape, run Pin-Art probe + LIFT to the tier-above coordinate.
4. **Revert-then-deeper-layer-closure if negative** (Rule 13): on regression, verify + diagnose + revert + implement deeper-layer closure.
5. **Chapter-close-inspect** (Rule 15): inspect post-fix failure-table top rows before declaring locale closed.

**Phase 6 (deferral emission)** — proposed sibling per the deferrals-ledger discipline: when Phase-5 surfaces a candidate locale below its founding threshold, emit a deferrals-ledger entry, not only a trajectory cross-locale note.

### Locale spawn protocol

1. Confirm multi-rung shape (Doc 737 §II).
2. Run Rule 11's 5-axis coverage check.
3. Create `pilots/<name>/seed.md` with the standard sections.
4. Run `apparatus/locales/discover.sh`; commit refreshed manifest.
5. Run EXT 0 baseline-inspection per Rule 23.
6. Begin substrate work at EXT 1.

### Locale resume protocol

1. Read `seed.md` (telos + apparatus + methodology).
2. Read `trajectory.md` tail (recent rungs).
3. Read `analysis.md` if present (diff-prod cross-reference).
4. Consult composes-with citations as the work warrants.

### Orphan-disposition protocol (engagement-tier Phase 5 instance)

When ≥3 locales surface that do not cluster under any single arc, run the protocol at `apparatus/docs/orphan-disposition-protocol.md`. Per orphan: recover M-T-I-R per Doc 744; discriminate relational form; test the 8 disposition candidates in order; first that fits is the disposition. Surface cross-orphan patterns for standing-rule promotion (Doc 727 §X).

---

## VI. (1) Substrate state — what is observed/modified

The substrate is the Cruft codebase, organized by Doc 729's resolver-instance pattern.

### Substrate tiers (top-down per Doc 730)

| Tier | Crate(s) | Role |
|---|---|---|
| **Source-language resolvers** | `pilots/rusty-js-parser/derived/`, `pilots/ts-resolve/derived/` | Surface-syntax → AST |
| **AST** | `pilots/rusty-js-ast/` | Common surface representation |
| **IR (spec-as-source-of-truth)** | `pilots/rusty-js-ir/derived/` | Tier-1.5 IR |
| **Bytecode** | `pilots/rusty-js-bytecode/derived/` | Compiler + opcode definitions |
| **Runtime** | `pilots/rusty-js-runtime/derived/` | Interpreter + module loader + intrinsics |
| **JIT** | `pilots/rusty-js-jit/derived/` | Cranelift-backed baseline JIT (LeJIT) |
| **Shapes** | `pilots/rusty-js-shapes/derived/` | Hidden classes substrate |
| **GC** | `pilots/rusty-js-gc/derived/` | Garbage collector |
| **Package manager** | `pilots/rusty-js-pm/derived/` | Resolver-instance #0 per Doc 732 |
| **Capability-passing runtime** | `pilots/rusty-js-caps/` | Doc 736 supply-chain-immune substrate |
| **Per-surface pilots** | `pilots/{fetch-api, http-codec, sockets, blob, ...}/derived/` | Pin-Art-derived consumer surfaces |
| **Host binary** | `cruftless/` | The `cruft` CLI |

---

## VII. (5) Triumvirate governance — how moves are adjudicated

The fifth box in the loop diagram is the triumvirate's veto + escalation discipline, operationalized per `apparatus/docs/triumvirate-operational-protocol.md`.

### Veto mechanism (operational protocol §II)

When the helmsman intends to push to `origin/main`:
1. Helmsman writes proposal at `apparatus/proposals/pending/<slug>.md` (frontmatter: session, proposed commits, target branch, summary, risk class, gates pre/post; body: substrate moves, risk assessment, composes-with).
2. Helmsman attempts `git push`. The `.githooks/pre-push` hook fires.
3. Hook searches `apparatus/proposals/decided/` for an APPROVED decision covering all pushed commits.
4. If found: push allowed, proposal+decision moved to `apparatus/proposals/archived/`.
5. If not found: push blocked; helmsman sends `**[HELMSMAN] CONSULTATION**` Telegram requesting arbiter session.
6. Arbiter (when instantiated by keeper) writes decision (APPROVED / VETO / DEFER-TO-KEEPER).
7. Helmsman retries push (if APPROVED), addresses gap (if VETO), or awaits keeper Rung-2 adjudication (if DEFER-TO-KEEPER).

Carve-outs (operational protocol §II.4): feature-branch pushes, local commits, prospective-doc edits, keeper-initiated pushes (Rung-2 monopoly preserved).

### Escalation discipline (operational protocol §III)

Telegram messages from resolvers prefix `**[HELMSMAN]**` / `**[ARBITER]**` (with optional `**[HELMSMAN/META]**` during pre-instantiation periods) and carry one of `INFO` / `CONSULTATION` / `VETO-PENDING` severity. Keeper directives arrive without prefix; resolvers respond with their role tag.

### Resolver-state separation (operational protocol §IV)

The arbiter session is a dedicated Claude Code instance with curated context loaded via `.claude/skills/arbiter-load.md`. Inclusion set: apparatus/docs/* + manifest + CANDIDATES + per-arc arc.md + the three triumvirate docs + pending proposals. Exclusion set: per-locale trajectory/seed/source (loaded on demand), corpus-ref (loaded on keeper directive), principal session's conversational history. Handover via append-only `apparatus/docs/arbiter-handover-log.md` (modeled on findings.md basin-stability discipline).

### Deployment stage status

| Stage | Status as of 2026-05-28 |
|---|---|
| Stage 1 (articulation tier) | PENDING keeper approval |
| Stage 2 (mechanical-veto tier) | NOT YET DEPLOYED (awaits first arbiter appointment) |
| Stage 3 (coverage-expansion tier) | DEFERRED (fills LOW–MEDIUM observation gaps from audit §V) |

---

## VIII. How the loop closes — feedback paths

Each arrow in §II's diagram corresponds to a concrete artifact transition. The loop closes because each transition is bidirectional + each cycle compounds.

- **(1)→(2)** — substrate modifications trigger re-measurement at every chapter close (`cargo build --release` → `diff-prod` → `cargo test` → corpus instruments → trajectory entry).
- **(2)→(3)** — measurement results feed into findings.md as new findings + occasionally new standing rules. Doc 727 §X basin-stability prevents drift.
- **(3)→(4)** — discipline artifacts inform locale spawning (Rule 11), closure (Rule 15), recovery (Rule 13), and deferral emission (Phase 6 + deferrals-ledger).
- **(4)→(5)** — completed substrate work routes through the proposal+veto+escalation discipline before propagating.
- **(5)→(1)** — approved pushes propagate to the substrate. The substrate-tier crates evolve with each approved cycle.

The self-improving properties: (2) compounds via new instruments enabling previously-invisible signals; (3) compounds via standing-rule additions reducing future search space; (4) compounds via the locale manifest accumulating precedents; (5) compounds via the arbiter handover-log accumulating apparatus-meta state across sessions; (1) compounds via Pin-Art-derived correctness per round.

---

## IX. Standing observation patterns (triumvirate-extended)

These structural patterns recur across the apparatus's operation; they are codified as standing rules (see `apparatus/docs/predictive-ruleset.md`) and surfaced here as apparatus-level observations.

- **Corpus-as-regression-instrument** (Finding IX.1).
- **Inspect-then-iterate compound discovery** (Finding IX.3 / Rule 15).
- **Multi-tier cascade-revival** (Doc 740).
- **Resolver-instance boundary contract** (Doc 742).
- **SIPE-T scale-invariance** (Doc 541 Appendix E).
- **Conformance-as-decision-basis extraction** (`apparatus/docs/ecma-conformance-parity-as-exhaustive-language-behavior-dag.md`).
- **Yield-curve-negative as Phase-5 confirmation** (Finding TAWR.6): when an attempted rung returns negative AND its diagnosis routes to a *different locale's* substrate, that is the canonical arc-close signal — not a Rule-13 deeper-layer trigger.
- **Passthrough-as-stub trap** (Finding TAWR.3): v1 substrate stubs registered as "passthrough of one argument" are silently observable as both wrong-result + wrong-order failures; stubs should throw NotImplemented rather than passthrough.
- **Proto.constructor reverse-edge omission** (Finding TAWR.4): ctor+proto pairs wired forward-only are invisible to method-dispatch but visible to spec-reflective queries.
- **NewTarget-honoring as shared substrate helper** (Finding TAWR.5): when ≥3 native ctors share a "default proto via fixed slot" idiom, promote the read to a Runtime method per Doc 744 §IV.2 relational-form.
- **Reflect-wrapper translation trap** (Finding TAWR.2): substrate moves adding "return false instead of throw" inside functions whose wrapper translates `Ok(_) → Boolean(true)` are silently swallowed without explicit match arms.

---

## X. Standing corpus references

The apparatus is theoretically grounded at jaredfoy.com/resolve/. Load-bearing corpus docs:

| Doc | Articulates |
|---|---|
| Doc 581 | Pin-Art apparatus + resume-vector discipline |
| Doc 727 §X | Basin-stability append-only update protocol |
| Doc 729 | Resolver-instance pattern (substrate architectural target) |
| Doc 730 | Vertical recurrence of lowering-compiler closure |
| Doc 731 | JIT as lowering-compiler tier |
| Doc 732 | Package manager as resolver-instance #0 |
| Doc 733 | Fractal seeds-and-trajectories |
| Doc 734 | Meta-resolution pipeline as engagement-recursion instrument |
| Doc 736 | Capability-passing runtime |
| Doc 737 | Locale as coordinate; nested seed/trajectory pairs |
| Doc 738 | Source identifier as coordinate |
| Doc 540 | Pin-Art apparatus formalization |
| Doc 541 Appendix E | SIPE-T scale-invariance via cruftless instance |
| Doc 739/740/741 | Cascade-revival series; Rule 13 theoretical anchor |
| Doc 742 | Resolver-instance boundary contract (O1/O2/O3) |
| Doc 744 | Pipeline-form discovery as predictive heuristic |
| Doc 745 (candidate) | Structured per-Phase emission + SIPE-T fractal fitting of trajectories |

---

## XI. Update protocol

This doc is consolidated-view (not append-only); it tracks the CURRENT state of the apparatus. Update when:

1. A new instrument is built — add to §III with path + observes + oracle.
2. A new discipline artifact lands — add to §IV with path + role.
3. A new locale is founded — refresh `apparatus/locales/discover.sh` reports the new count.
4. A new substrate tier crate is added — add to §VI's tier table.
5. A new corpus doc lands that the apparatus implements — add to §X.
6. A triumvirate-protocol revision lands — update §I + §VII to reflect the revised authority/discipline.

Canonical history lives in findings.md + corpus + commit log + (for triumvirate-tier) arbiter-handover-log.md.

---

## XII. Quick-reference index

By role:

- **Want to spawn a new locale?** → `apparatus/locales/CANDIDATES.md` + Doc 737 + §V.spawn-protocol.
- **Want to land a substrate move?** → `apparatus/docs/predictive-ruleset.md` + CLAUDE.md discipline + the proposal+veto workflow at §VII.
- **Want to understand the apparatus telos?** → `apparatus/docs/ecma-conformance-parity-as-exhaustive-language-behavior-dag.md`.
- **Want to add an instrument?** → §III + Doc 581.
- **Want to add a standing rule?** → `pilots/rusty-js-jit/findings.md` next Addendum + this doc's §IV + predictive-ruleset update.
- **Want to publish corpus?** → 3-stage pipeline (corpus-master → resolve mirror+push → jaredfoy seed).
- **Want to recover from a substrate-bug regression?** → Rule 13 + Rule 14 + Rule 15.
- **Want to emit a deferral?** → Phase 6 + `apparatus/docs/deferrals-ledger.md` append discipline.
- **Want to evaluate apparatus-meta coherence?** → arbiter role per `apparatus/docs/triumvirate-operational-protocol.md` §IV; load via `.claude/skills/arbiter-load.md` (Stage 2).
- **Want to veto a helmsman push?** → Arbiter writes decision at `apparatus/proposals/decided/<slug>.md` per §VII.

---

*Reformulated 2026-05-28 per keeper directive Telegram 10195; promoted to apparatus tier 2026-05-28 per keeper directive Telegram 10200. Prior version dyad-anchored, preserved at `docs/deprecated/repository-apparatus.md` for trajectory binding per the deprecation discipline. Triumvirate articulations live at `apparatus/docs/triumvirate-*.md` once Stage 1 promotion lands.*
