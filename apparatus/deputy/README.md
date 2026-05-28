# Deputy Working Directory

Working surface for the deputy service-tier resolver per `apparatus/docs/engagement-doc-deputy.md` + `apparatus/docs/service-tier-and-statefulness-protocol.md` §IV. Active when the keeper appoints a deputy session via `/deputy-load`.

## Layout

```
apparatus/deputy/
├── fleet-state/      # per-session fleet-state snapshots (append-only history of fleet activity)
└── broadcasts/       # helmsman-to-fleet relay messages (verbatim attribution)
```

## Fleet-state summaries

At session entry + periodic cadence (Stage 4 protocol setting; default 30 minutes when fleet is active), the deputy authors a fleet-state summary at `apparatus/deputy/fleet-state/YYYY-MM-DDTHHMMSS-<descriptor>.md` with sections:

- **Active branches** — per-branch: author, last commit, scope, divergence from main.
- **Pending proposals** — per-proposal: helmsman session, target branch, summary, risk class.
- **Active arcs** — per-arc: agents touching it, current rung count, last activity.
- **Anticipated coordination concerns** — convergence on same substrate locus, conflicting proposals, branches at risk of conflict.

Prior summaries remain in the directory as historical record; the latest summary represents current state.

## Broadcasts

When the helmsman delegates "tell the fleet that I am about to land X" or analogous, the deputy authors a broadcast at `apparatus/deputy/broadcasts/YYYY-MM-DDTHHMMSS-<topic>.md` with frontmatter:

```yaml
---
helmsman_session: <id>
authored_at: <ISO timestamp>
delivered_by: deputy
topic: <short slug>
fleet_action_requested: rebase | pause | ignore | coordinate
---

## Helmsman's message (verbatim)

<the helmsman's text, exactly as written>

## Context

<one paragraph: what is happening, what the fleet should do in response>
```

Fleet resolvers read the broadcasts directory on session entry; the broadcast is the apparatus's record that the announcement was made.

## Discipline

- **Verbatim relay**: helmsman text in broadcasts is reproduced exactly; smoothing introduces drift the fleet then acts on.
- **No mediation**: when two fleet agents are in tension, the deputy surfaces the tension as a fleet-state summary + Telegram `**[DEPUTY] INFO**` to the keeper; the deputy does not propose resolutions.
- **No git mutation**: the deputy reads `git branch -r` + `git log`, but never commits, merges, rebases, cherry-picks, or creates branches.

## Activation status

This directory is created at Stage 4 of the operational protocol deployment per keeper directive Telegram 10219. The deputy session is not yet appointed; the directory waits.
