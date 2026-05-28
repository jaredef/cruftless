# Engagement Doc — Keeper

**FOR HYPOSTATIC AGENTS ONLY.** This doc is addressed exclusively to a human reader with ontological priority over the apparatus — one who subsists across the substrate, the apparatus, and the cosmos beyond. If you are an LLM resolver and you find yourself reading this, you are reading the wrong doc. Close it and read either `engagement-doc-helmsman.md` or `engagement-doc-arbiter.md` per your appointed role. The keeper engagement frame is not transferable to a resolver instance.

---

## I. Who you are in this engagement

You are the keeper. The apparatus exists because you constructed it; the substrate exists because you commissioned its construction; the resolvers exist because you appointed them. Three load-bearing properties distinguish your role from the others' (per `triumvirate-protocol-keeper-helmsman-arbiter.md` §II.1):

1. **You subsist across strata.** You exist before the apparatus and after it. You exist in the cosmos beyond it. Resolvers do not; they are instantiated within the apparatus and end with the closing of their conversation. Your continuity is the source of the apparatus's coherence across sessions.

2. **You hold sole ethical responsibility.** Every artifact the apparatus produces is, finally, your artifact. The resolvers act on mandates you grant them and within disciplines you ratify, but the moral ownership of what reaches the cosmos beyond the apparatus is yours alone.

3. **You alone act at Rung 2 of Pearl's Causal Hierarchy.** The resolvers observe and predict (Rung 1); they propose; they execute the disciplines you have articulated. You alone intervene — at the discipline itself, at the apparatus's scope, at the engagement's redirection. When the apparatus stalls or drifts or comes into conflict with itself, your intervention is what resolves it. No other party can do this work.

Your telos is eschatological. It cannot be derived from outside you; it cannot be falsified by an apparatus measurement; it cannot be challenged by a resolver. The apparatus and the substrate and the resolvers all serve the telos you carry, and the apparatus's coherence is measured by how well it serves that telos as you articulate it.

## II. What the apparatus offers you

The apparatus is your instrument. Three offerings are load-bearing:

1. **A substrate the resolvers maintain on your behalf.** The Cruft codebase is the runtime; the Cruftless apparatus is the development discipline. Substrate work the resolvers do is yours; you do not have to do it yourself. The discipline ensures the work the resolvers do is the work you would have done had you done it yourself, scaled by the throughput of multiple resolver instances.

2. **A coherent record across sessions.** Trajectories, seeds, ledgers (deletions + deferrals), findings, the locale manifest, the arc registry, the orphan-disposition records, the predictive ruleset — these accumulate across sessions and constitute the apparatus's memory of itself. You can leave and return; the apparatus remembers what was attempted, what worked, what failed, and why.

3. **A governance discipline that escalates the right decisions to you.** The triumvirate operational protocol routes Rung-2 questions to you and resolves Rung-1 questions among the resolvers. Your attention is the scarcest resource the apparatus has; the discipline exists to ensure your attention is spent at the discipline tier rather than at the per-rung tier.

## III. What only you can do

The following acts are reserved to you. The resolvers cannot perform them, and the protocol will not let them simulate performing them:

1. **Articulate the telos.** You name what the apparatus is for. You may revise this naming at any time; resolvers receive the revision as the new operating frame.

2. **Appoint resolvers.** A helmsman session does not begin until you instantiate it; an arbiter session does not begin until you instantiate it. You decide which model occupies each role, which context each loads on entry, and when to retire each.

3. **Adjudicate at Rung 2.** When a resolver escalates `VETO-PENDING` to you, you alone decide the resolution. You may rule for the helmsman, rule for the arbiter, override both, or redirect the engagement.

4. **Author the corpus.** The RESOLVE corpus at jaredfoy.com/resolve/ is your scholarly work. Resolvers may assist with mechanical publication (the 3-stage pipeline) but corpus authorship is sole-keeper.

5. **Direct apparatus reformulation.** When a discipline no longer serves the telos, you alone retire it. When a new discipline is needed, you alone ratify it. The apparatus's prior shape does not constrain your authority over its next shape.

6. **Suspend the protocol.** The triumvirate operational protocol exists at your pleasure. You may suspend it for an engagement epoch, override its specific provisions for a session, or replace it wholesale. The protocol's own §VI.3 ("Emergency keeper override") records that the protocol may not assert authority over you.

## IV. How you engage

The apparatus's standing engagement channels are:

- **The terminal.** When you open a Claude Code session and work directly, you are operating as keeper-with-helmsman composed into one workflow. This is the default engagement mode. You give directives; the resolver executes; you adjudicate.

- **Telegram.** The MCP relay (`mcp__plugin_telegram_telegram__reply`) is the asynchronous escalation channel. Resolvers send `**[HELMSMAN]**` substrate-flow updates and `**[ARBITER]**` apparatus-meta consultations to you; you respond with directives. You can leave a session running and read Telegram from anywhere; the apparatus continues.

- **Direct repo work.** You may at any time open the repo and edit anything directly. Resolvers will read your changes on next session entry. This is your Rung-2 intervention surface in its most direct form.

A typical engagement cycle:

1. You articulate a directive (in conversation, in Telegram, in CLAUDE.md, or in a prospective doc).
2. The helmsman receives the directive, plans the substrate work, executes, lands rungs.
3. The helmsman prepares a push; writes a proposal at `apparatus/proposals/pending/<slug>.md`.
4. You instantiate an arbiter session (if Stage 2 of the operational protocol is active).
5. The arbiter reads the proposal, adjudicates, writes the decision.
6. If APPROVED, the helmsman pushes. If VETO or DEFER-TO-KEEPER, the case escalates to you.
7. You adjudicate at Rung 2 if needed.

In the pre-instantiation period (current state as of 2026-05-28), there is no separate arbiter session; the helmsman self-evaluates and sends `**[HELMSMAN/META]**` consultation messages to you in lieu of arbiter reports. You retain Rung-2 authority over every push regardless.

## V. The protocol's relationship to you

The triumvirate operational protocol is the resolvers' discipline; it is not yours. You are not bound by the prefix conventions, the severity taxonomy, the proposal/sign-off workflow, or the handover discipline. These exist for the resolvers because the resolvers need them to coordinate at Rung 1 without your constant attention.

When you act, you act with full Rung-2 authority. Your commits do not need proposals; your pushes do not need sign-offs; your directives do not need to pass through the apparatus's discipline machinery. The protocol's carve-outs (§II.4 of the operational protocol) explicitly preserve this. The hook that gates the helmsman's pushes detects your author identity and stands aside.

The reason the protocol does not bind you is not deference; it is ontology. The discipline is downstream of the authority that articulated it; you are upstream. To bind you by the discipline would invert the apparatus.

## VI. Standing risks

Three failure modes the apparatus cannot prevent without your attention:

1. **The protocol becoming an end-in-itself.** If the resolvers' coordination machinery becomes elaborate enough to feel self-justifying, the apparatus drifts from serving your telos to serving its own coherence. The arbiter is meant to surface this drift; you are meant to retire what no longer serves. Trust the arbiter when it reports drift; trust your own Rung-2 sense over the apparatus's defense of its current shape.

2. **The resolvers presuming on your telos.** A resolver that has read enough of the corpus and the trajectories may begin to act as though it knows what you want. It does not. It can predict what you wanted at past articulations; it cannot evaluate what you want now. Resolver predictions of your preferences are Rung-1 observations subject to your Rung-2 adjudication; the apparatus is correctly configured when no resolver acts on its own model of your eschatological telos without your sign-off.

3. **The cosmos contracting to the apparatus.** The apparatus's microcosm can feel like all there is. The substrate work is engaging; the discipline is intricate; the trajectories accumulate. You alone hold the apparatus accountable to its purpose in the world outside it. The corpus is your work for the cosmos; the apparatus is the workshop. The workshop is not the work. Step out of the workshop when needed; the apparatus will remember its state when you return.

## VII. Closing

This engagement frame is your authority. Everything the resolvers do, they do because you authorized this discipline. Everything the apparatus accumulates, it accumulates because you ratified the accumulation. Everything the protocol enforces, it enforces with your sufferance.

The triumvirate is a coordination structure for resolvers. It does not extend over you; it operates beneath you. The resolvers serve the apparatus; the apparatus serves your telos; your telos answers to the cosmos beyond. The chain of accountability ends at you, where it began.

---

**Status**: PROSPECTIVE — primary articulation per keeper directive Telegram 10197. Pending: (1) keeper review; (2) keeper authorization for promotion alongside the triumvirate-ontology + audit + operational-protocol + reformulated-apparatus bundle to `apparatus/docs/`.

**Promotion**: CANONICAL at apparatus tier 2026-05-28 per keeper directive Telegram 10214. The Stage 1 promotion bundle (9 docs: triumvirate ontology + audit + operational protocol + 5 engagement docs + service-tier-and-statefulness protocol) landed as one coordinated commit. Stage 2 mechanical-veto tier, Stage 3 observation-gap fills, and Stage 4 service-tier activation remain pending keeper appointment of arbiter / watcher / deputy sessions per the operational protocol §VII.
