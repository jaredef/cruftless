# Agent Feedback — Apparatus-Level Schema

Canonical form for `agent-feedback.md`, the per-locale artifact that captures cross-resolver review of substrate moves landed in that locale. Composes with `seed.md` (telos) and `trajectory.md` (per-rung log) as the third member of the locale's standing-document set.

This doc specifies what an `agent-feedback.md` file is, what it contains, when it is read, who writes it, and what invariants govern its evolution. The worked example that motivated the abstraction lives at `pilots/rusty-js-http-server/agent-feedback.md`.

---

## I. Place in the cybernetic loop

The locale-entry protocol (Doc 581 + Doc 737) currently reads `seed.md` (telos + apparatus + methodology) and the `trajectory.md` tail (most recent rungs) as the minimum sufficient orientation for a fresh resolver. `agent-feedback.md` extends that protocol: when an engagement spans multiple LLM resolvers, the substrate state recorded in trajectory is necessary but insufficient — it captures *what was done* but not *how another resolver reads what was done*.

The locale becomes the artifact registry for cross-resolver review. Each resolver entering the locale reads what prior resolvers found load-bearing, and contributes its own read when its move closes. The closed loop:

```
   prior resolver's move           prior resolver's feedback
   (trajectory entry)              (agent-feedback entry)
            │                                │
            ▼                                ▼
   ┌───────────────────────────────────────────────┐
   │  next resolver enters locale                  │
   │  reads seed + trajectory tail + agent-feedback│
   │  (the running summary, plus the most recent   │
   │   review's concerns + next-rung block)        │
   └───────────────────────────────────────────────┘
            │
            │ informs the move
            ▼
   ┌───────────────────────────────────────────────┐
   │  next resolver lands its move                 │
   │  appends a new trajectory entry               │
   │  optionally appends an agent-feedback review  │
   │  if reviewing prior work; updates the running │
   │  summary at the head of agent-feedback.md     │
   └───────────────────────────────────────────────┘
```

Stage placement per `apparatus/docs/repository-apparatus.md`: `agent-feedback.md` is a discipline artifact (stage 3) read at locale-entry within stage 4's executing structure. The artifact constrains what counts as a valid next move; the locale executes against that constraint.

---

## II. File invariants

| Property | Value | Rationale |
|---|---|---|
| **Path** | `pilots/<locale>/agent-feedback.md` (or any nested locale's own directory) | Co-located with seed.md and trajectory.md so the locale-entry read finds all three together. |
| **Lifecycle** | Append-only entries; the running-summary head is mutated in place. | Reviews are evidence; once written they describe what one resolver saw at one moment. Mutating them rewrites history. The running summary is the distilled load-bearing claim, allowed to be updated when later reviews refine it. |
| **Triggering** | Created on the first cross-resolver review. Not required for single-resolver locales. | Empty-file overhead is real; create only when a second resolver inspects. |
| **Read protocol** | Every locale entry must read the running-summary head + the most recent review's "Concerns ranked by leverage" and "Recommended next rungs" sections. | The running summary compresses; the most recent detailed review is the unprocessed source. Older reviews are read on demand. |
| **Author identity** | The resolver's own model identifier (e.g. `Claude Opus 4.7 (1M context)`, `GPT-5.5`). Never anonymous. | Reads are model-conditioned; the resolver identity is load-bearing for evaluating the review's prior frame. |

---

## III. Entry schema

Each entry has three blocks. All three are mandatory.

### Block 1 — Resolver metadata

Captures the identity, capability, and authority frame of the reviewer.

```
**Reviewer:** <model identifier including context-window class if non-default>
**Reviewer session:** <session identifier or short description; e.g. "main-cruftless cybernetic loop, 2026-05-25 18:00 local">
**Target:** <commit hash + one-line commit subject>
**Target author:** <model identifier of the resolver under review>
**Date:** <ISO 8601>
**Authority frame:** <one of: "keeper-directed read", "self-initiated audit", "follow-on hygiene pass", "regression triage", "spawn-time scope review">
```

Required fields. The reviewer's model identifier matters because subsequent agents reading the feedback need to know what prior frame conditioned the review. A GPT-5.5 review of a Claude Opus 4.7 commit and vice versa surface different concerns; flagging the asymmetry is part of the apparatus.

### Block 2 — Working constraint set

A brief enumeration of what the reviewer had loaded in its context window when forming the review. Distinguishes "this concern is informed by recent substrate work in the same engagement" from "this concern is generic spec-reading without engagement memory."

```
**Files read:** <bulleted list of files inspected for the review, paths only>
**Standing artifacts loaded:** <CLAUDE.md, MEMORY.md entries, repository-apparatus.md, predictive-ruleset.md, etc.>
**Recent engagement memory:** <substrate moves landed in this session that informed the read; e.g. "EIPD-EXT 1, GBNE-EXT 1, RPTC-EXT 4 closed today; their bug-patterns were in working memory">
**Gates re-verified locally:** <which measurement instruments were re-run as part of the review; e.g. "diff-prod 42/42; random-300 prev-PASS 300/300, 0 regressions">
**Not loaded:** <explicit note of relevant artifacts NOT consulted, when omission affects review depth>
```

The "Not loaded" field is the load-bearing one. A review that did not consult Doc 736 may miss authority-composition concerns; flagging the absence lets a later resolver request a deeper read rather than re-deriving the gap.

### Block 3 — Feedback

The substantive review. Four sections, in order:

```
### What lands well
<bulleted, concrete preserve-this items. Each tied to a specific architectural decision or substrate move, not generic praise.>

### Concerns ranked by leverage
<numbered list, ordered by combination of severity and ease-of-fix. Each concern includes:
 - The observed gap (specific file/line if applicable)
 - The reference convention or spec that it diverges from
 - A fix shape (not necessarily full code, but enough that the next resolver can scope the fix)>

### Recommended next rungs
<concrete next-rung names with scope estimates. Should reference existing locale conventions (HS-EXT N+1, etc.).>

### Standing notes
<cross-cutting facts the next resolver needs that don't fit the other sections. Pre-existing test failures, documented carve-outs the reviewer wants flagged, etc.>
```

Any of the four sections may be omitted with an explicit "N/A — <reason>" line; omission without explanation is invalid.

---

## IV. Running summary

The head of the file is the running summary. Format:

```
## Running summary for the next agent entering this locale

1. <load-bearing claim distilled from the most-recent reviews>
2. <load-bearing claim>
3. <load-bearing claim>
4. <load-bearing claim>
```

Conventions:

- Three to five bullets. Compression is the point; longer summaries indicate the locale needs a Pin-Art retrospective rather than more bullets.
- Each bullet references a specific concern or recommendation, not abstract observations.
- Updated when a new review either confirms a prior bullet (no change), refines it (rewrite), or supersedes it (replace and note the supersession in the new review's text).
- Truncating the summary is allowed when a substrate move resolves a bullet (note resolution in the resolving move's trajectory entry).

---

## V. Composition with other locale artifacts

| Artifact | Owner | Question it answers | Read order on locale entry |
|---|---|---|---|
| `seed.md` | locale founder | What is this locale for? | First |
| `trajectory.md` (tail) | each per-rung resolver | What has been done here recently? | Second |
| `agent-feedback.md` (head + most-recent review) | reviewing resolvers | How does another resolver read what has been done? | Third |
| Nested locales' triples | nested-locale founders | What sub-workstreams branch from here? | Read recursively if scope includes them |

The three together form the locale's standing context. A resolver entering the locale reads all three before proposing the next move.

---

## VI. Authoring discipline

1. **Review when something nontrivial lands.** A bug fix at the leaf of a substrate doesn't need a review entry; a substrate composition move at a locale's load-bearing seam does. The threshold: would a different resolver have made meaningfully different decisions? If yes, review.
2. **Be specific about what informed the read.** Block 2 is the trust signal. A review formed without consulting the locale's seed or relevant corpus docs should declare that explicitly; the next resolver can then weight the review accordingly.
3. **Concerns must include a fix shape.** Naming a problem without sketching the fix produces a backlog item, not a discipline artifact. Reviews that surface gaps the reviewer cannot scope a fix for should escalate to keeper rather than land as concerns.
4. **No retraction in place.** If a later review or a substrate move shows a prior concern was wrong, the next review's text says so; the prior entry stays. The reviewer's read at that moment is the artifact.
5. **Append the running-summary update with the review.** A review that does not update the summary head is incomplete; the next resolver would have to read the whole file to extract load-bearing claims.
6. **Em-dash restraint per CLAUDE.md.** Target 0-1 per 1000 words; prefer commas, parens, periods.

---

## VII. Cross-resolver asymmetries the schema makes visible

The apparatus exists because resolvers differ in known ways:

- **Training cutoff** determines which language-spec versions and ecosystem conventions the resolver treats as default.
- **Context-window class** determines how much engagement memory the resolver can carry into a review. A short-context review of a long-running locale may miss substrate patterns the locale has established.
- **Tool inventory** determines whether the resolver re-verified gates or trusted the trajectory's claims. Reviews that re-ran gates carry more weight than reviews that read.
- **Prior session memory** determines whether the reviewer knew about findings (EIPD.1, RPTC.7, etc.) from earlier in the engagement. A reviewer with that memory will flag pattern violations a fresh reviewer won't catch.

Block 1 (resolver metadata) + Block 2 (working constraint set) jointly surface these asymmetries so the next resolver can read the feedback through the appropriate lens.

---

## VIII. Template

A blank `agent-feedback.md` for a new locale:

```markdown
# <locale-name> — Agent Feedback

Cross-agent review notes. Read on every locale entry. Append (do not overwrite) new entries chronologically; the head of this file is the running summary for the next agent.

---

## Running summary for the next agent entering this locale

<3-5 distilled load-bearing claims, updated as new reviews land>

---

## Review N — <ISO 8601 date>

**Reviewer:** <model identifier>
**Reviewer session:** <session description>
**Target:** <commit hash + subject>
**Target author:** <model identifier>
**Date:** <ISO 8601>
**Authority frame:** <keeper-directed | self-initiated | follow-on hygiene | regression triage | spawn-time scope review>

### Working constraint set

**Files read:** <list>
**Standing artifacts loaded:** <list>
**Recent engagement memory:** <list>
**Gates re-verified locally:** <list>
**Not loaded:** <list>

### What lands well

- <bullet>

### Concerns ranked by leverage

1. **<concern>**. <observation> <fix shape>
2. ...

### Recommended next rungs

- **<rung name>** — <scope>

### Standing notes

- <note>

---

*Append new reviews below. Keep the running summary above truthful and short.*
```

---

## IX. Worked example

`pilots/rusty-js-http-server/agent-feedback.md` was the first instance, written before this schema was formalized. It corresponds to the schema's general shape but predates the explicit Block 1 / Block 2 / Block 3 split; the next review entry there should adopt this schema's form, and the existing entry should be amended in place only to add the missing Block 2 (working constraint set), since omitting it would leave the schema's trust-signal field unfilled retroactively.

---

## X. Integration with the apparatus

Once this schema is in effect, the following standing artifacts should reference it:

- `apparatus/docs/repository-apparatus.md` — list `agent-feedback.md` in the per-locale-artifact set alongside `seed.md` and `trajectory.md`.
- The locale-entry read protocol (Doc 581 + Doc 737) — extend to read `agent-feedback.md` when present.
- The locale manifest (`apparatus/locales/manifest.json`) — optionally record `has_agent_feedback: bool` per locale so cross-locale tools can surface which locales have undergone cross-resolver review.
- `CLAUDE.md` (root) — add a brief pointer indicating that any resolver entering a locale must check for and read `agent-feedback.md`.

Each of these is a small follow-on commit that finishes wiring the abstraction into the cybernetic loop.
