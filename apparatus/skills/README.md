# Apparatus Skills

Claude Code skills tracked at the apparatus tier per keeper directive Telegram 10219. Each skill instantiates a non-helmsman resolver role with that role's curated context-load and operational discipline.

## Canonical location

Skills live at `apparatus/skills/<name>.md`. The repo's `.claude/skills/` is a symlink to this directory so Claude Code's skill discovery finds the apparatus-tracked versions. Edit at the apparatus path; never edit at `.claude/skills/` directly.

After cloning the repo, the symlink is already in place; no per-clone setup is required for skill discovery.

## Roster

| Skill | Role | Triumvirate tier | Engagement doc |
|---|---|---|---|
| `/helmsman-load` | Helmsman | Governance (Rung-1 substrate-steering) | `apparatus/docs/engagement-doc-helmsman.md` |
| `/arbiter-load` | Arbiter | Governance (Rung-1 apparatus-meta + veto) | `apparatus/docs/engagement-doc-arbiter.md` |
| `/watcher-load` | Watcher | Service (Rung-1 observation) | `apparatus/docs/engagement-doc-watcher.md` |
| `/deputy-load` | Deputy | Service (Rung-1 communication) | `apparatus/docs/engagement-doc-deputy.md` |

**The substrate resolver (default role) does not have a load skill.** Substrate resolver is the default for any LLM resolver entering this engagement (per keeper directive Telegram 10225–10226 + CLAUDE.md §"Resolver role discipline"); the standard CLAUDE.md / AGENTS.md / `apparatus/docs/agent-engagement.md` / `apparatus/docs/engagement-doc-substrate-resolver.md` read on session entry is sufficient. Skills exist only for the four appointed roles.

## When to invoke

Each skill is invoked **only when the keeper directly appoints the session to the role** (via Rung-2 intervention, typically in the inbound message that starts the session — e.g., "You are the Helmsman.", "Load Watcher"). A resolver does not invoke `/helmsman-load`, `/arbiter-load`, `/watcher-load`, or `/deputy-load` on its own initiative; the role separation is load-bearing for the apparatus's coordination structure.

When the keeper opens a fresh Claude Code instance and appoints a role, that instance's first action is to invoke the corresponding load skill.

## Discipline

- **Per-skill curated context.** Each skill loads a specified inclusion set and excludes content the role does not need. The inclusion sets are deliberately disjoint where role separation matters (e.g., the arbiter does NOT load per-locale trajectory tails by default; that would conflate the arbiter with the helmsman).
- **Reports session-ready.** Each skill ends by directing the loaded role to send a Telegram `**[ROLE] INFO** — session instantiated, ...` summary so the keeper can confirm orientation before issuing further directives.
- **Handover on close.** Each role has a corresponding handover-log doc (`apparatus/docs/{arbiter,watcher,deputy}-handover-log.md`) that the role appends to before its context budget expires. The next instance reads the handover-log tail to pick up open work.

## Related apparatus

- `apparatus/docs/triumvirate-protocol-keeper-helmsman-arbiter.md` — governance ontology.
- `apparatus/docs/triumvirate-operational-protocol.md` — operational spec with deployment stages.
- `apparatus/docs/service-tier-and-statefulness-protocol.md` — service-tier + statefulness articulation.
- `apparatus/docs/agent-engagement.md` — canonical substrate-disciplined LLM resolver directions.
