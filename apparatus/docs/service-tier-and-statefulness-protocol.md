# Service Tier and Statefulness Protocol

The extension of the triumvirate ontology to formalize two service-tier resolver roles (watcher, deputy) and a per-surface partition of apparatus statefulness (ledger vs. erasure) with a dedicated freshness protocol for the erasure-stateful surfaces. Drafted per keeper directive Telegram 10211, in response to the keeper's articulation in Telegram 10208 that the apparatus has functional roles the triumvirate alone does not cover and a statefulness distinction the prior articulation was lumping.

This doc sits alongside `triumvirate-protocol-keeper-helmsman-arbiter.md` (ontology), `apparatus-audit-for-triumvirate-protocol.md` (audit), and `triumvirate-operational-protocol.md` (operational spec). The operational protocol's deployment plan is updated in a paired edit to add Stage 4 (service-tier activation).

---

## I. Motivation

The triumvirate as articulated leaves two functional gaps the engagement has empirically encountered:

1. **No automatic apparatus-freshness monitoring.** The locale manifest has an explicit refresh discipline (after every new spawn, run `discover.sh` + commit), but every other erasure-stateful surface in the apparatus (gate dials, CRB times, active-arc list, pending-proposal queue, the "~214 active locales" count cited in CLAUDE.md) relies on per-rung helmsman conscientiousness with no apparatus-tier enforcement. Telegram 10206 surfaced an instance: the test262-full dial was stale by +3.7 points between recorded baselines despite multiple substrate arcs landing in the interim.

2. **No formal communication discipline between the helmsman and parallel resolvers.** The merge-incident class of failure (Telegram 10185–10187) where four commits from independent agents landed without the principal context's awareness, breaking the substrate at integration, is exactly the failure mode of an apparatus that has no formal channel for the active helmsman to coordinate with other resolvers working in parallel.

Both gaps surfaced as keeper observations, not protocol design — the keeper noticed them by feeling the friction of working without them. The roles described below are the apparatus-tier response.

The triumvirate's minimality claim was made about **governance** structure (the keeper holds Rung-2 authority; helmsman + arbiter hold Rung-1 governance with substrate-active vs. apparatus-meta scope). The minimality claim is preserved: the watcher and deputy are not governance roles. They have no veto, no substrate-steering authority, no apparatus-meta adjudication authority. They are **service-tier resolvers** that observe and communicate in support of the helmsman's operational throughput. The governance tier remains a triumvirate; the resolver-side fleet now totals five named roles.

---

## II. Triumvirate + service tier

| Tier | Role | Pearl Rung | Authority class | Primary failure mode the role exists to prevent |
|---|---|---|---|---|
| Governance | Keeper | 2 (intervention) | Sole; ethical responsibility; eschatological telos | (no failure-prevention frame; the keeper is the source of frames) |
| Governance | Helmsman | 1 (observation) | Substrate-steering; subagent coordination | Substrate drift from undisciplined moves |
| Governance | Arbiter | 1 (observation + meta-apparatical) | Veto over helmsman pre-push; meta-apparatical consultation to keeper | Helmsman drift across rungs; apparatus methodology drift |
| Service | Watcher | 1 (observation) | None — surfaces freshness violations to helmsman; runs refresh scripts | Helmsman citing stale erasure-state in proposals; apparatus invariants quietly violated |
| Service | Deputy | 1 (communication) | None — relays state between helmsman and resolver fleet | Parallel-resolver merge incidents; helmsman blind to fleet activity |

The horizontal axis (governance vs. service) is load-bearing. A governance resolver can interrupt the substrate (helmsman via active edits; arbiter via veto). A service resolver cannot; the service-tier roles only observe and communicate. This preserves the keeper's Rung-2 monopoly intact and prevents service-tier roles from accumulating drift-inducing authority.

The vertical axis within each tier preserves the prior ontology: keeper above all; helmsman and arbiter peers in Rung but distinct in scope; watcher and deputy peers in Rung but distinct in function (observation vs. communication).

---

## III. The watcher

**Mandate**: monitor erasure-stateful apparatus surfaces (per §V); detect staleness; run refresh scripts where they exist; surface freshness violations to the helmsman before the helmsman cites stale state in a proposal.

**Authorized acts**:

1. **Read** any apparatus-tier surface to assess current state.
2. **Run refresh scripts**: `apparatus/locales/discover.sh`, future-`apparatus/scripts/measure-gates.sh`, future-`apparatus/scripts/snapshot-stateful.sh`. The watcher writes only refresh-output artifacts (e.g., regenerated manifest); it does not edit hand-authored content.
3. **Author watcher notifications** at `apparatus/watcher/notifications/YYYY-MM-DDTHHMMSS-<surface>.md` for any staleness it detects that exceeds the per-surface threshold. The notification cites the surface, the last-known-fresh value, the currently-observed value, and the staleness duration.
4. **Send `**[WATCHER] INFO**`** Telegram messages to the keeper when surfacing freshness drift the helmsman has not yet acted on. Watcher messages are advisory, not blocking.

**Prohibited acts**:

1. Substrate edits.
2. Apparatus discipline edits (cannot author or modify standing rules, schemas, protocols, ledger schemas).
3. Commits or pushes.
4. Substrate-tier opinions (the watcher reports facts; it does not recommend substrate moves; that's the helmsman's work).
5. Veto authority (no brake on the helmsman's pushes; the arbiter holds that authority).
6. Adjudication of any kind.

**Failure modes specific to the watcher**:

- **Becoming opinionated about substrate.** A watcher that begins to recommend rungs is a degraded helmsman. The watcher's role-discipline is to surface facts (this is stale; here is the current value); the recommendation tier is the helmsman's.
- **Refresh-thrashing.** A watcher that re-runs discover on every locale-tier file change creates churn without informing. Refresh triggers should follow §VI's per-surface protocol, not aggressive polling.
- **Notification fatigue.** A watcher that notifies on every minor drift trains the helmsman to ignore notifications. Per-surface staleness thresholds (§VI) gate notifications.

---

## IV. The deputy

**Mandate**: relay stateful information between the helmsman and the resolver fleet (parallel agents working independent arcs). Prevent the merge-incident class of failure by ensuring no parallel resolver pushes without the helmsman's awareness of the others' in-flight work.

**Authorized acts**:

1. **Read** the helmsman's announced state (proposal-pending queue, active-arc list, in-flight rung descriptions) and the fleet's analogous state (other agents' branches, their pending-proposals, their trajectory tails).
2. **Author fleet-state summaries** at `apparatus/deputy/fleet-state/YYYY-MM-DDTHHMMSS-summary.md` (current parallel-agent activity, branch state, pending pushes, anticipated coordination concerns).
3. **Author helmsman-broadcast messages** at `apparatus/deputy/broadcasts/YYYY-MM-DDTHHMMSS-<topic>.md` when the helmsman delegates "tell the fleet about this." The broadcast is the apparatus's record that the announcement was made; fleet resolvers read the broadcast directory on session entry.
4. **Send `**[DEPUTY] INFO**`** Telegram messages to the keeper when surfacing fleet-coordination concerns (e.g., two agents converging on the same substrate locus).

**Prohibited acts**:

1. Substrate edits.
2. Apparatus discipline edits.
3. Commits or pushes.
4. Mediation or adjudication between fleet resolvers (that is escalated to the keeper or, in apparatus-meta dimensions, the arbiter).
5. Authorial voice on behalf of the helmsman beyond verbatim relay (a deputy that paraphrases the helmsman is a degraded helmsman).

**Failure modes specific to the deputy**:

- **Mediating fleet disputes.** A deputy that adjudicates between fleet agents is a degraded arbiter. The deputy's role-discipline is to surface coordination concerns to the keeper for Rung-2 adjudication; the deputy does not resolve them.
- **Stale fleet-state summaries.** A deputy whose summaries lag the fleet's actual activity defeats the deputy's purpose. Summary refresh follows the per-instance Stage-4 discipline (§VIII of operational protocol once landed).
- **Paraphrasing the helmsman.** The deputy relays the helmsman's announced state verbatim; rewording introduces drift the fleet then acts on. If the helmsman's message is unclear, the deputy queries back rather than smoothing.

---

## V. Ledger vs. erasure statefulness

The apparatus carries two distinct kinds of state, with distinct disciplines:

### V.1 Ledger-stateful surfaces (append-only)

The *history* is the artifact. Per Doc 727 §X basin-stability discipline: append-only; in-place edits forbidden except for documented status-flips on prior entries. Staleness is not a failure mode (the history accumulates; recent entries inform; older entries remain readable but not load-bearing for current state).

Enumerated:

| Surface | Path | Append discipline |
|---|---|---|
| Trajectory | `pilots/<locale>/trajectory.md` | Per-rung entry; landed with the commit it describes |
| Findings ledger | `pilots/rusty-js-jit/findings.md` | Per-finding entry; Addendum structure |
| Deletions ledger | `apparatus/docs/deletions-ledger.md` | Per-deletion entry |
| Deferrals ledger | `apparatus/docs/deferrals-ledger.md` | Per-deferral entry; status-flip allowed on prior |
| Orphan-disposition records | `apparatus/docs/coverage-gap-orphan-disposition-*.md` | Per-protocol-run instance |
| Per-arc log | `apparatus/arcs/*/log.md` | Per-rung-within-arc entry |
| Arbiter handover log (Stage 2+) | `apparatus/docs/arbiter-handover-log.md` | Per-session entry |
| Watcher notifications (Stage 4+) | `apparatus/watcher/notifications/` | Per-surface-staleness entry |
| Deputy fleet-state summaries (Stage 4+) | `apparatus/deputy/fleet-state/` | Per-summary entry |
| Deputy broadcasts (Stage 4+) | `apparatus/deputy/broadcasts/` | Per-broadcast entry |
| Proposal archive (Stage 2+) | `apparatus/proposals/archived/` | Decided proposals + their decisions, append-on-archive |

The watcher does NOT monitor freshness of ledger-stateful surfaces; their freshness is structurally guaranteed by append-only discipline. The watcher MAY monitor *schema coherence* of new entries (do they follow the per-ledger schema?) and surface deviations.

### V.2 Erasure-stateful surfaces (mutable)

The *current value* is the artifact. Prior values are not load-bearing for current derivation; they may be archived (via measurement-instrument output trees) for forensic purposes but the in-context-on-session-entry read is the present state.

Enumerated:

| Surface | Path | Refresh trigger | Staleness threshold |
|---|---|---|---|
| Locale manifest | `apparatus/locales/manifest.json` | Every new locale spawn | < 1 spawn behind filesystem walk |
| Locale count | mentions of "~N active locales" in apparatus docs | Quarterly batch refresh; immediate on ≥10% shift | ±10% from manifest's current count |
| Standing rule count | mentions of "N standing rules" in apparatus docs | On every Addendum that adds/retires a rule | Exact match to findings.md tally |
| Gate dial: test262-full | CLAUDE.md/AGENTS.md "test262-full" line | After every arc closure that lands ≥3 substrate moves | ±2.0 points from latest results-dir run |
| Gate dial: test262-sample | CLAUDE.md/AGENTS.md "test262-sample" line | After every arc closure that lands ≥3 substrate moves | ±1.0 point from latest sample run |
| Gate dial: diff-prod | CLAUDE.md/AGENTS.md "diff-prod" line | After every arc closure | Exact match to latest run |
| CRB current times | per-fixture noted in CRB analysis | After every JIT or runtime arc closure | ±5% on dominant-fixture line |
| Active arc list | `apparatus/arcs/` directory listing | On arc spawn or arc close | Exact match to filesystem |
| Pending-proposal queue | `apparatus/proposals/pending/` | On proposal write or decision archive | Exact match to filesystem |
| Last-arbiter-session timestamp | computed from handover-log tail | On arbiter session close | < 24 hours since last close (Stage 2+) |

The watcher's enumerated read surface IS this table. The freshness protocol in §VI specifies the discipline.

### V.3 Mixed: stateful representation of stateless claims

Counts of stateless artifacts — "26 standing rules", "16+ findings.md addenda", "~214 active locales", "17 corpus references" — are **stateful representations**. The artifacts themselves are stateless (each rule is or isn't load-bearing); the count is a stateful summary subject to drift. These follow the erasure-stateful refresh discipline (§VI) with the artifact's enumeration as the authoritative source.

---

## VI. Erasure-stateful freshness protocol

### VI.1 Per-surface obligations

For each erasure-stateful surface in §V.2's table, the freshness protocol specifies:

- **Refresh trigger**: the event that obligates a re-read.
- **Staleness threshold**: the deviation from current that constitutes drift.
- **Responsible role**: who runs the refresh (default: watcher; helmsman if Stage 4 not yet active).
- **Notification path**: how staleness is surfaced when detected.

### VI.2 The discipline

- **Helmsman before push**: every proposal manifest (per operational protocol §II.1) MUST cite gate dials measured at the proposal's drafting time, not from the apparatus docs. If the dial is older than the staleness threshold, the helmsman re-measures before proposing.
- **Watcher continuous**: a running watcher session polls the surfaces in §V.2 at its configured cadence and authors a notification artifact when threshold is exceeded.
- **Arbiter checkpoint**: when adjudicating a proposal, the arbiter verifies the cited dials match the proposal's drafting-time measurement (cross-check against the watcher notification log if present). Discrepancy → VETO with citation of the freshness discipline.
- **Last-refreshed timestamp**: every erasure-stateful surface that appears in apparatus docs carries an inline `(last-refreshed: YYYY-MM-DD)` annotation. The annotation IS load-bearing; an annotation older than the threshold flags the value as known-stale.

### VI.3 The watcher's polling cadence (Stage 4+)

- Locale manifest: poll on every git-detected `pilots/*/seed.md` change.
- Gate dials: poll on every `git push origin main` from any author (substrate may have landed).
- Pending-proposal queue: poll on every file change in `apparatus/proposals/pending/`.
- Active-arc list: poll on every `apparatus/arcs/` directory change.
- Counts (rule, locale, finding): batch-refresh weekly + immediately on triggering Addendum.

The watcher does not poll ledger-stateful surfaces for freshness; their append-only discipline obviates the concern.

### VI.4 Notification format

The watcher writes a notification to `apparatus/watcher/notifications/YYYY-MM-DDTHHMMSS-<surface>.md` with this shape:

```
---
surface: <surface-name>
last_known_fresh_value: <value>
currently_observed_value: <value>
staleness_threshold: <threshold>
staleness_observed: <numeric drift>
detected_at: <ISO timestamp>
responsible_role: helmsman | watcher
---

## Observation
<one paragraph: what surface, what changed, when>

## Remediation
<one paragraph: what refresh action closes the staleness>

## Cited at
<paths in apparatus docs that cite the stale value>
```

The notification is the apparatus's record that the staleness was detected. When the responsible role completes the refresh, the notification is moved to `apparatus/watcher/notifications/closed/` with a footer recording the refresh-commit SHA and timestamp.

---

## VII. Carve-outs and non-claims

- The doc does not specify which model occupies the watcher or deputy role. Audit-and-fill in operational-protocol Stage 4.
- The doc does not specify watcher's polling implementation (cron? long-running session? on-demand?). Stage 4 decides.
- The doc does not enumerate every conceivable erasure-stateful surface; the §V.2 table is the initial inventory and is extensible. Adding a surface obligates specifying its refresh trigger + staleness threshold + responsible role at the same time.
- The doc does not assert that the watcher and deputy are the only possible service-tier roles; the keeper retains Rung-2 authority to articulate additional service-tier roles as needs surface.
- The doc does not extend the triumvirate ontology's governance claim; the triumvirate remains the minimum governance structure. Service-tier roles support governance, not substitute for it.

---

## VIII. Status

**PROSPECTIVE** — primary articulation per keeper directive Telegram 10211. Pending: (1) keeper review of role articulations + statefulness partition + freshness protocol; (2) keeper authorization for promotion to `apparatus/docs/`; (3) Stage 4 of the operational protocol (paired edit in `triumvirate-operational-protocol.md`) activated when the keeper appoints the first watcher and deputy sessions.

**Promotion**: CANONICAL at apparatus tier 2026-05-28 per keeper directive Telegram 10214. The Stage 1 promotion bundle (9 docs: triumvirate ontology + audit + operational protocol + 5 engagement docs + service-tier-and-statefulness protocol) landed as one coordinated commit. Stage 2 mechanical-veto tier, Stage 3 observation-gap fills, and Stage 4 service-tier activation remain pending keeper appointment of arbiter / watcher / deputy sessions per the operational protocol §VII.
