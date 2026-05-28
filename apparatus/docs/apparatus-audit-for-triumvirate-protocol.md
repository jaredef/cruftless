# Apparatus Audit for the Triumvirate Protocol

The audit prerequisite from `triumvirate-protocol-keeper-helmsman-arbiter.md` §V. This document inventories the Cruftless apparatus's current affordances against the read/write requirements the three triumvirate roles will impose, surfaces the gaps the operational protocol must close, and provides the input the keeper needs to authorize the protocol's specification.

The audit is staged as a prospective doc, sibling to the triumvirate articulation, and is not committed pending keeper review. Once the keeper signs off on the audit's findings, the operational protocol can be specified against this concrete inventory.

## I. Method

Two parallel Explore-tier resolvers were dispatched against the repository on 2026-05-28, each scoped to a non-overlapping audit dimension:

- **Resolver A (observation + action surfaces).** Inventoried what a resolver can currently read at Rung 1 (observation surfaces) and write at Rung 1 (action surfaces). Output is an enumerated table of every apparatus-tier surface, file paths, line counts, and per-surface consumption classification (helmsman-suited / arbiter-suited / both / gap).
- **Resolver B (coordination machinery).** Assessed three coordination dimensions the triumvirate will require — veto machinery, escalation channels, resolver-state separation — and classified each by current-state, closest-existing-analogue, and gap-severity.

The two resolvers were dispatched in parallel with no shared context; their outputs were synthesized by the principal context into the sections below. The synthesis preserves both resolvers' findings without re-deriving them; cross-references between resolvers' outputs are noted inline.

## II. Findings: observation surfaces (Rung 1 read)

The apparatus is **well-populated for observation**. Resolver A enumerated 215 seed+trajectory pairs (per Doc 737 fractal recurrence), the apparatus-tier docs (`apparatus/docs/` — repository-apparatus, predictive-ruleset, standing-rule-13, agent-feedback-schema, arc-as-coordinate, orphan-disposition-protocol, the two ledgers, the orphan-disposition instance record), the locale manifest, the arc registry, the corpus mirror, and the measurement-instrument output trees.

### II.1 Surfaces well-suited to helmsman consumption

- Per-locale `seed.md` + `trajectory.md` pairs (215 each) and per-locale `analysis.md` where present. Helmsman reads the seed for telos + apparatus, the trajectory tail for in-flight state, and the analysis for empirical anchoring against diff-prod and test262.
- `apparatus/docs/predictive-ruleset.md` (15 consolidated rules) and `pilots/rusty-js-jit/findings.md` (26-rule canonical ledger). Helmsman internalizes these on every loop entry.
- `apparatus/docs/standing-rule-13-prospective-application.md`. Helmsman consults during Phase 4 negative-rung recovery.
- `apparatus/locales/CANDIDATES.md`. Helmsman consults pre-spawn for new locales.
- `apparatus/docs/deferrals-ledger.md`. Helmsman consults at Phase 6 emission and at un-defer-condition detection.
- Measurement-instrument outputs (`scripts/diff-prod/results/`, `pilots/apparatus/test262-categorize/full-suite/results/`, `pilots/apparatus/cross-runtime-bench/results/YYYY-MM-DD*/`). Helmsman consults to validate gates per substrate move.

### II.2 Surfaces well-suited to arbiter consumption

- `apparatus/docs/repository-apparatus.md` (§III–VII). Foundational apparatus articulation; the arbiter loads this as the discipline reference against which helmsman moves are evaluated.
- `apparatus/locales/manifest.json` (~250 lines, 214 active locales). Stable summary surface; the arbiter reads the manifest as the cross-locale state-at-a-glance.
- `apparatus/arcs/` (15+ arcs, each with `arc.md` + `log.md`). Per-arc summaries are the natural cross-rung consolidation tier.
- `apparatus/docs/deletions-ledger.md`. Methodology-coherence audit surface (the arbiter's archetypal use).
- `apparatus/docs/coverage-gap-orphan-disposition-2026-05-28.md` and the protocol it instantiates. Apparatus-meta artifact; arbiter consults for arc-coherence evaluation.

### II.3 Identified gaps in observation surfaces

1. **No rule-drift audit surface.** The arbiter's archetypal question — "are the standing rules still load-bearing, or has methodology drifted?" — has no aggregate read. Arbiter must currently infer from trajectory-tail sampling, which conflicts with the cleaner-context premise.
2. **No cross-instrument measurement dashboard.** Each instrument (diff-prod, test262-sample, test262-full, CRB, TCC, TXC) writes to its own output tree; assembling cross-instrument state-at-a-glance requires running each separately and synthesizing manually. Arbiter is forced into the helmsman's per-instrument-script-invocation workflow.
3. **No per-arc apparatus-meta summary.** Per-arc `arc.md` records the arc's scope and per-rung log, but does not record arc-tier methodology consequences (which standing rules the arc validated, which rules it stressed, what new candidates the arc surfaced for ledger entry). Arbiter must assemble this from the deferrals-ledger + deletions-ledger + per-locale trajectory tails.

These gaps are non-blocking — the arbiter can do the work — but they push the arbiter back into helmsman-shaped consumption, undermining the cleaner-context premise. The operational protocol should specify whether the apparatus extends to fill these gaps (new aggregator surfaces) or whether the arbiter accepts the workload as part of its role.

## III. Findings: action surfaces (Rung 1 write)

Resolver A's action enumeration is reproduced in compressed form. Reversibility classes: **L** = locally-reversible, **R** = repo-history-amendable via git, **P** = pushed-irreversible at remote, **X** = external-irreversible.

| Act class | Class | Current discipline status |
|---|---|---|
| Substrate edits to `pilots/*/derived/src/` | R | Free; subject to standing rules 1–26 |
| Apparatus edits (ledger appends, manifest refresh, arc spawning) | R | Free per protocols; ledgers append-only |
| Trajectory + seed authoring | R | Free; required per substrate move |
| Local commits | R | Keeper-authorized per request (CLAUDE.md) |
| Push to origin/main | P | Keeper-authorized per request; no force-push |
| Measurement runs | L | Free (helmsman-initiated for gate validation) |
| Build invocations | L | Free |
| Refresh scripts (`apparatus/locales/discover.sh`) | R | Required after new locale spawning |
| Corpus-pipeline scripts (stages 1–3) | X at stage 3 | Not-explicitly-governed by triumvirate yet; keeper-sole-authority on corpus per Doc 707 |
| Telegram escalation | X | Free; no taxonomy or severity discipline |
| GitHub operations via `gh` CLI | X | Free; no governance |
| Subagent dispatch (Agent tool) | L (draft) → R (landed) | Free; helmsman-coordinated; no arbiter-veto exists |

### III.1 Identified gaps in action surfaces

The action surface is **operationally mature for substrate work** and **partially codified for apparatus work**. The triumvirate-specific gaps overlap entirely with the coordination-machinery gaps in §IV and are not separately enumerated here.

## IV. Findings: coordination machinery (the triumvirate's load-bearing gaps)

Resolver B's three-dimension assessment is the audit's substantive finding. Each dimension surfaces a gap requiring net-new construction; the apparatus's existing artifacts provide only conceptual scaffolding the protocol will build on.

### IV.1 Veto machinery — CRITICAL gap

**Current state**: zero. `.git/hooks/` contains only `.sample` placeholders; no `.githooks/`; no Husky / lefthook / pre-commit framework; no `.github/workflows/` (no CI pipeline); no branch protection rules discoverable locally; no apparatus-tier doc on veto mechanics, propose/veto coordination, or arbitration surfaces.

**Closest existing analogue**: `apparatus/docs/agent-feedback-schema.md`. The schema establishes cross-resolver review with structured fields (`agent_id`, `rung#`, `observation`, `recommendation`, `confidence`, `composes_with`) instantiated at `pilots/<locale>/agent-feedback.md`. The schema is the apparatus's most veto-shaped artifact, but its operational mode is **post-hoc consultation, not pre-landing brake**. The reviewing resolver flags concerns that the next resolver internalizes; there is no machinery that interrupts the helmsman before a substrate move lands.

**What the operational protocol must build**: a propose/veto coordination surface that lets the arbiter brake the helmsman before pushed-irreversible action. Three candidate architectures the protocol should consider:

1. **Pre-commit-hook arbiter-call**: a hook in `.githooks/` that requires arbiter sign-off before any commit lands. Strong gate; fragile if the arbiter is unreachable or its session is between instantiations.
2. **PR-required workflow**: convert main-branch pushes to PR-required, with the arbiter as a required reviewer. Standard pattern; requires CI infrastructure not currently present in the repo.
3. **Local-discipline propose/veto channel**: a dedicated apparatus surface (`apparatus/proposals/` or `apparatus/veto-queue/`) where the helmsman posts proposed moves before landing and the arbiter responds. Light-weight; depends on the helmsman honoring the discipline (which is exactly the failure mode an external veto would prevent).

The protocol's choice among these will depend on how seriously the engagement treats the merge-incident class of failure that motivated the triumvirate.

### IV.2 Escalation channels — HIGH gap

**Current state**: a single Telegram channel via the MCP relay (`mcp__plugin_telegram_telegram__reply`). Text replies only. No structured-message types in current use. No per-resolver tagging in the relay protocol; the keeper reads a flat stream and must distinguish resolver-identity manually from message content.

**Closest analogue**: the agent-feedback schema includes resolver metadata fields, but these live in intra-artifact records, not in Telegram message metadata. Trajectory-tier references to keeper directives use the `Telegram NNNN` message-id convention as the canonical citation anchor; this is sufficient for after-the-fact attribution but provides no real-time disambiguation.

**What the operational protocol must specify**: at minimum, a routing discipline that lets the keeper distinguish substrate-flow updates (helmsman) from apparatus-meta consultation (arbiter). Two candidate architectures:

1. **Single-channel prefix convention**: all messages prefix `**[HELMSMAN]**` or `**[ARBITER]**`. Zero new machinery; depends on resolver self-tagging discipline.
2. **Separate Telegram channels per role**: the keeper runs two chat windows, one per resolver-role. Stronger separation; requires keeper-tier configuration of the Telegram plugin to support multiple channels.

A secondary concern the protocol should address: severity levels. Currently every helmsman message is a status update; the arbiter's messages will sometimes be a veto event the keeper must adjudicate (which the keeper would want pushed to attention) and sometimes a methodology-drift observation the keeper can defer (which the keeper would not want interrupting work). A severity convention — at minimum `INFO / CONSULTATION / VETO-PENDING` — would let the keeper triage.

### IV.3 Resolver-state separation — HIGH gap

**Current state**: no machinery. `.claude/worktrees/` exists at the repo root but is empty; the Agent tool's `isolation: "worktree"` parameter is not invoked by any apparatus protocol. Repo-level `.claude/` has no `settings.json`; settings live globally at `~/.claude/`. No skill definitions for an arbiter-shaped context; both resolvers would load identical CLAUDE.md + apparatus/docs/ context by default.

**Closest analogue**: the CLAUDE.md tier separation (apparatus/ rung-1, docs/ rung-2) is articulated discipline but not mechanically enforced. Agents currently rely on explicit keeper directives ("read Doc 736") to curate their loaded context; there is no template or hook that constructs an arbiter-shaped read on session entry.

**What the operational protocol must specify**:

1. **The arbiter-shaped context.** A precise specification of which apparatus surfaces the arbiter loads on session entry. Candidate inclusion set: `apparatus/docs/*` + `apparatus/locales/manifest.json` + per-arc `arc.md` summaries + the two ledgers + the orphan-disposition records. Candidate exclusion set: per-locale `trajectory.md` tails (load only on arbiter-initiated inspection), `pilots/*/derived/src/` source files (the arbiter is not a substrate-editor), `docs/engagement/` keeper-tier sidecar (read only on keeper directive).
2. **Instantiation machinery.** Whether the arbiter session is spun up via the Agent tool's `isolation: "worktree"` mode, via a dedicated Claude Code session the keeper opens separately, or via a skill that loads the arbiter-shaped context. The first preserves the strongest separation; the third is the lightest-weight.
3. **Handover protocol.** When the arbiter's context fills, how is the role re-instantiated without losing the apparatus-meta state the prior instance accumulated. A summary-tier record the arbiter writes before context-end (analogous to the trajectory-tail discipline for the helmsman) is the obvious candidate.

The arbiter's epistemic value depends entirely on the separation being real. If the operational protocol specifies an arbiter that shares context with the helmsman in practice, the arbiter is structurally indistinguishable from a second helmsman opinion, which is not what the triumvirate ontology calls for.

## V. Consolidated gap matrix

| Gap | Category | Severity | Existing scaffolding |
|---|---|---|---|
| Rule-drift audit surface | Observation (arbiter-suited) | LOW | Trajectory-tails (helmsman-shaped) |
| Cross-instrument measurement dashboard | Observation (arbiter-suited) | LOW–MEDIUM | Per-instrument result trees |
| Per-arc apparatus-meta summary | Observation (arbiter-suited) | LOW–MEDIUM | `arc.md` per-rung logs |
| Arbiter veto mechanism | Coordination | **CRITICAL** | Agent-feedback schema (post-hoc only) |
| Escalation channel taxonomy + routing | Coordination | HIGH | Telegram MCP relay (single channel, no tagging) |
| Resolver-state separation machinery | Coordination | HIGH | Worktrees exist but unused; tier-separation discipline articulated |
| Arbiter handover protocol | Coordination | MEDIUM | None |
| Corpus-pipeline triumvirate-tier governance | Action | DEFERRED per triumvirate §VI | Manual 3-stage discipline |

The three CRITICAL/HIGH coordination gaps are the load-bearing work the operational protocol must specify. The LOW–MEDIUM observation gaps are quality-of-life improvements that affect how naturally the arbiter inhabits its role; they can be deferred to a follow-up audit-and-fill pass if the protocol-derivation timeline is tight.

## VI. Implications for the operational protocol

The audit surfaces three concrete deliverables the operational protocol must produce:

1. **A veto mechanism specification** — one of the three candidates in §IV.1 chosen and operationalized, with the propose/veto coordination surface either built (pre-commit hook / PR workflow / proposals directory) or explicitly committed to as a discipline-only constraint.
2. **A Telegram escalation discipline** — at minimum the role-prefix convention from §IV.2 and a severity taxonomy that lets the keeper triage. Optionally separate channels per role.
3. **An arbiter session-instantiation specification** — the inclusion/exclusion set for arbiter-shaped context from §IV.3, the instantiation mechanism (worktree-isolated Agent / dedicated Claude Code session / skill-curated context), and the handover-summary discipline.

Three secondary deliverables are surfaced by the LOW-severity gaps and may be deferred:

4. **A rule-drift aggregator** for arbiter consumption (the standing rules with recent-rung citations of each rule's invocation or stress).
5. **A cross-instrument measurement summary surface** the arbiter loads instead of per-instrument result trees.
6. **An arc-tier apparatus-meta summary** that consolidates per-arc methodology-consequences.

The audit makes no recommendation among the candidates within each deliverable; that is the operational protocol's design work, which the keeper will direct after reading this audit.

## VII. Carve-outs and non-claims

- The audit does not evaluate whether the triumvirate ontology is correct or sufficient; the triumvirate articulation already stands as the keeper's directive, and the audit assumes that ontology as input.
- The audit does not specify any resolver model selection or per-role prompt-engineering; those are operational-protocol concerns.
- The audit does not enumerate the helmsman's accumulated context as it currently stands in the principal session; the audit's lens is apparatus-tier affordances, not in-flight conversation state.
- The audit does not validate the gaps via reproduction (no attempts to install a pre-commit hook to confirm none exists; the absence is read from filesystem inspection only). If the keeper wants stronger evidence on any specific gap, a follow-up probe can be commissioned.

---

**Status**: PROSPECTIVE — primary articulation per keeper directive Telegram 10191, sibling to `triumvirate-protocol-keeper-helmsman-arbiter.md` (Telegram 10189). Pending: (1) keeper review of audit findings; (2) keeper directive on which gaps the operational protocol should close (CRITICAL/HIGH at minimum; LOW-severity at keeper's discretion); (3) operational protocol drafted against this audit's gap matrix.

**Promotion**: CANONICAL at apparatus tier 2026-05-28 per keeper directive Telegram 10214. The Stage 1 promotion bundle (9 docs: triumvirate ontology + audit + operational protocol + 5 engagement docs + service-tier-and-statefulness protocol) landed as one coordinated commit. Stage 2 mechanical-veto tier, Stage 3 observation-gap fills, and Stage 4 service-tier activation remain pending keeper appointment of arbiter / watcher / deputy sessions per the operational protocol §VII.
