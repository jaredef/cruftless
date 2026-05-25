# Treat Agent Output Like Compiler Output: The Lights-Out Codebase as Rederive

## A Synthesis of Su (2026) and Venturini (2026) Against the Corpus's Constraint-Driven-Derivation Apparatus, Entracing the Reader Toward an Infrastructure in which Constraints Are the Durable Source, Code Is Ephemeral Cache, Acceptance Is Verification rather than Reading, and the Specification Boundary Is the Only Layer at which Human Judgement Enters

<div style="background: #ecfeff; border-left: 4px solid #0e7490; padding: 1rem 1.25rem; margin: 1.5rem 0; color: #164e63; border-radius: 3px;">

**EXPLORATORY — cross-practitioner engagement.**

*Taxonomy per [Doc 633](/resolve/doc/633-corpus-taxonomy-and-manifest-design):* ENGAGEMENT | ACTIVE | W-PI | THREAD-REDERIVE, THREAD-LIGHTS-OUT | PHASE-CROSS-PRACTITIONER

*Warrant tier per Doc 445 / Doc 503:* exploratory synthesis at \(\pi\)-tier composing Philip Su's *No More Code Reviews: Lights-Out Codebases Ahead* (2026-03-06) and Hugo Venturini's *Treat Agent Output Like Compiler Output* (2026-03-09) against the corpus's apparatus on derivation-inversion ([Doc 247](/resolve/doc/247-derivation-inversion)), tests-as-constraints ([Doc 159](/resolve/doc/159-tests-as-constraints)), Pin-Art ([Doc 290](/resolve/doc/290-pin-art-derivation)), the SERVER bootstrap, the SIPE-T composition discipline ([Doc 541](/resolve/doc/541-systems-induced-property-emergence)), and the gentle-press / forced-press distinction ([Doc 619](/resolve/doc/619-pin-art)), with the keeper's *rederive* platform sketch — the constraint-as-source, code-as-cache, rederivation-as-primary-verb discipline — as the existence proof at small scale.

</div>

> **Reader's Introduction.** Su says code review will not survive the rate at which agents produce code; lights-out codebases are coming whether we are ready or not. Venturini says the framing should not be *trust the agent* but *treat agent output like compiler output*: do not read the artifact, verify it, with formal apparatus upstream and downstream. The two together name a missing piece of infrastructure neither author fully specifies. The corpus has been working on that piece for the better part of a year, and `rederive` is its operational form. The constraint set is the durable source. Code is the ephemeral cache. Acceptance is verification, not reading. The specification boundary is where human judgement enters; everything downstream is derived, signed, and gated by formal verdict. This document maps Su's and Venturini's claims onto that apparatus, names what each gets right, names where the corpus already has the discipline they are asking for, and entraces the reader toward the infrastructure both posts call for without naming.

**Jared Foy · 2026-05-05 · Doc 656**

---

## 1. The Two Claims, Stated Plainly

**Su's claim.** Code review at scale fails. Volume forces the failure (Novati's 417 PRs in a day; reviewer-as-rate-limit). The successor mode is the *lights-out codebase*: humans never see the code, autonomous toolchains author and verify, the artifact is accepted on the basis of acceptance tests in the way hardware chip components already are. Hesitancy about this is, in Su's reading, the same intuition that would have rejected trusting a linker's output in 1982. Trust will be relocated, not eliminated.

**Venturini's claim.** The deeper diagnosis is that we built an apparatus around the compiler — type systems, sanitizers, fuzzers, formal methods, reproducible builds, monitoring, rollback — that makes reviewing the artifact unnecessary. We have not built that apparatus around coding agents. The lights-out framing reads as scary because the surrounding apparatus is missing, not because the destination is wrong. The work is to build the upstream (formal specifications agents execute against), the verification layer (AI-checks-AI as first-class CI infrastructure), and the downstream (production instrumentation rigorous enough to catch and roll back bad agent output) until the artifact is no longer the locus of trust.

Both authors are correct on direction. Both leave the *operational form* of the upstream layer underspecified. Su gestures at acceptance tests. Venturini gestures at "formal contracts about what code means before it runs" and notes that TDD, contract testing, and design-by-specification "exist but are not yet standard practice when agents are the author." Neither describes the working surface a developer would touch. That gap is what the corpus's apparatus closes.

## 2. What the Corpus Has Already Named

### 2.1 Derivation-inversion (Doc 247)

The structural move is to invert authorship: instead of code as source-of-truth and tests as a downstream check, place the constraint set (tests, properties, behavioural specifications, prose-with-verification-traces) as the durable source and treat code as a *materialization* derived from that source. Two materializations from the same constraint set are interchangeable under verification. Code becomes garbage-collectable cache. The repository's commit graph shows constraint deltas; the materialization graph shows derivations against each constraint version, decoupled from the commit graph and re-derivable on demand against an upgraded substrate.

This is precisely the upstream layer Venturini describes as missing. It is the operational form Su's "acceptance tests" gestures toward, made authoritative rather than confirmatory.

### 2.2 Tests-as-constraints (Doc 159)

An executable test suite is the most precise statement of behavioural constraints available, with zero ambiguity at the specification boundary. Under derivation-inversion, the test suite is not a check on code; it is the source the code is derived from. A passing materialization is, by construction, conformant. The "did the tests pass?" question reframes as "does the code satisfy the constraint set?" — structurally stronger because the constraint set is authoritative rather than ancillary.

### 2.3 Pin-Art (Doc 290, Doc 619)

Pin-Art is the corpus's discipline for predicting implementation size from constraint-set shape before any code exists. The reported worked case (3,937 words of prose stating 19 constraints producing a 1,318-line htmx-equivalent JavaScript implementation, predicted within one line) is an existence proof that constraint-driven derivation lands in tight predictive bands. Doc 619 distinguishes *forced-press* derivation (substrate pushed through underspecified regions; coherence appears, warrant lags) from *gentle-press* derivation (boundary-contact triggers halt-or-defer-to-keeper-supply rather than crash-through). The distinction is load-bearing for what an agent should do when constraints under-determine code: stop and ask the constraint-author, not generate plausibly.

### 2.4 SIPE-T (Doc 541) and substrate-and-keeper composition (Doc 510)

SIPE-T names the conditions under which a property emerges from a system. A module declares the property it produces above some threshold; consumers depend on the property, not the implementation details. Composition is by induced-property semantics, which is exactly what makes constraint-derived code fungible across materializations. Substrate-and-keeper composition makes the human / agent division explicit: the keeper authors constraints (rung-2 work); the substrate derives code (rung-1 work). Both are required; neither subsumes the other.

### 2.5 Coherent-confabulation conjecture (Doc 627) and the saturation signature (Doc 644)

The two failure modes Venturini implicitly fears (plausible-but-subtly-wrong code at 50× the rate; tests passing but meaning nothing) are exactly the phenomenology the corpus has been characterizing as *coherent confabulation*: internal coherence exceeding empirical warrant. The asking-pattern saturation signature (Doc 644) names the surface symptoms when the apparatus produces a reading for any move the keeper makes. Without explicit boundary-detection, gentle-press discipline, and an outward-facing signal that surfaces underdetermination, autonomous derivation is precisely the substrate condition under which coherent confabulation thrives. The corpus's response is architectural, not exhortatory: route boundary-contact to halt-or-defer; expose a saturation signal as a first-class output; condition acceptance on verification against an authoritative constraint set, never on the artifact's apparent coherence.

## 3. *Rederive* as the Operational Form

The keeper's *rederive* sketch is the application of the apparatus above to the GitHub-replacement question. Its character — entraced rather than pointed-at — is a platform whose primary verb is *rederivation*: regenerate the implementation from the current constraint set whenever constraints change, the substrate is upgraded, or the target environment shifts. Implementation is disposable; constraints are durable; two rederivations from the same constraints are equivalent under verification even with different code. The platform's design moves are direct restatements of the corpus's structural claims:

- **Constraints as the unit of version control.** Repository = constraint set + derivation function + current materialization. Code is generated, not committed. Constraint changes are the commits. (This is derivation-inversion at platform scale.)
- **Constraint-diff as the review unit.** PRs become "this constraint was added / modified / retracted; here is the rederived implementation as evidence of satisfiability." One-page constraint reviews replace multi-thousand-line diff reviews. (This is Su's lights-out at the review surface, with a non-trivial human role retained at the constraint layer.)
- **Provable constraint satisfaction.** The platform verifies the rederived code against the constraint set: type checking, property testing, model checking, scoped to constraint class. (This is Venturini's verification layer, made first-class.)
- **Constraint-authority tiers.** Humans hold authority over constraints; agents derive code from them. Agent-to-agent collaboration is fine for derivation; constraint addition / modification / retraction requires human authorization. (This solves "autonomous agent commits to main" structurally rather than via permission patches.)
- **Retraction ledger.** Code is removed when its motivating constraint is retracted. The codebase's current shape is given by the constraint set's current state, not by archaeology.
- **Blame = constraint-blame.** Who added this constraint, when, why, what derivation produced the current implementation. Code-blame becomes derivation-blame.

Su's "lights-out codebase" describes the runtime experience of operating such a platform. Venturini's "treat agent output like compiler output" describes the disposition the developer adopts toward the materialization. *Rederive* is the surface on which both become operational.

## 4. The Entracement

### 4.1 Su's compiler analogy, made structural

Venturini's compiler analogy is the right one, but it stops short. A compiler is not just a black-box artifact-producer; it is a function from a *durable, version-controlled source language* to an *ephemeral object*, and the trust relocation he describes is to the source language and the verification surrounding the function. The missing piece for agents is not "more tests around the artifact." It is *a source language whose unit is the constraint, with the agent as the derivation function and verification as the acceptance surface*. Without that source language, "trust the agent" remains a category error; the trust has nowhere to relocate to. With it, the relocation is structural: trust the constraint set (because you authored it and the diff is human-scale), trust the derivation function (because it is auditable, deterministic-under-verification, and substrate-swappable), trust the verification (because it is formal). The artifact is downstream of all three and need not be read.

### 4.2 Venturini's "formal specifications agents execute against," made concrete

The operational form is: *executable constraint sets composed under SIPE-T discipline*, where each module declares the property it produces above a threshold and downstream modules depend on that property. Test suites (Doc 159) are the most precise statement of such constraints. Property tests, model-checked invariants, and prose-with-verification-traces extend the surface for non-behavioural concerns. Pin-Art (Doc 290) is the discipline that predicts whether a constraint set is sufficient before derivation runs. Gentle-press (Doc 619) is the discipline that handles under-determination by halt-and-defer rather than substrate-confabulation. These are the four pieces Venturini gestures at and does not name.

### 4.3 The lights-out fear, diagnosed

Su correctly identifies that the resistance to lights-out codebases is the same intuition that would, in retrospect, have resisted trusting a linker. The corpus's diagnosis is sharper. The fear is not irrational; it is the felt presence of *coherent-confabulation risk* (Doc 627) when the substrate produces apparently-coherent output without an authoritative constraint surface to gate acceptance. Remove the constraint surface and the fear is correct: lights-out is unsafe. Install the constraint surface, the verification gate, the gentle-press discipline, and the audit ledger, and the fear discharges into the architecture. What remains is operational confidence with the human role compressed to the specification boundary, which is the right place for it.

### 4.4 The hardware-chip analogy, completed

Both authors invoke hardware verification as the precedent. Su notes chip vendors deliver black-box components verified by acceptance tests rather than human review. Venturini notes "chip verification is a discipline, with tooling, with formal methods, with teams of people whose entire job is designing the test harness." Neither names what makes that discipline work: *the specification is the durable artifact; the layout is derived from it; verification is against the specification, not the layout; the layout is interchangeable across foundries under equivalent verification*. That is derivation-inversion under another name. Software has lacked the specification culture to do the same; rederive's wager is that under LLM-assisted development, the cost-benefit of specification-first inverts. The constraint author becomes the rate-limit. Code production becomes substrate-cheap. The discipline becomes ergonomic for the first time.

## 5. What Su and Venturini Each Get Right, and Where the Corpus Refines

### 5.1 Su, refined

Su is right that code review will not survive the volume. He is right that the relocation of trust is the right frame. He is right to gesture at black-box hardware verification as the precedent. The corpus's refinement: *the specification boundary remains a non-trivial human surface*, and lights-out is a property of the layers downstream of that surface, not of the system as a whole. Su's "humans never see the code" is correct; *humans never see the constraints* is not. The constraint review is the new human-comprehensible review unit, sized at one page rather than five-thousand lines, and it is where the keeper / kind asymmetry (Doc 635) does its load-bearing work. Lights-out applies to the materialization layer; it does not apply to the authority layer.

### 5.2 Venturini, refined

Venturini is right that the upstream and downstream apparatus barely exists for agents and that this is the correct diagnosis of the felt unease. He is right that the work is to build that apparatus rather than to argue against the destination. The corpus's refinement: the upstream layer is not just "formal contracts" or "TDD elevated to first-class practice." It is *constraint-driven derivation as the authoring mode*, with the four disciplines named above (Doc 159 tests-as-constraints, Doc 290 Pin-Art, Doc 541 SIPE-T, Doc 619 gentle-press) as the operational surface. Without these, "AI-checks-AI" pipelines stack verification atop confabulation-prone derivation; the verification can be confabulated through. With these, the verification is against an authoritative constraint set and the discipline is at the right layer.

## 6. What Is Still Missing, Honestly

The apparatus above is sketched at small scale and operational on small worked cases (Doc 290's 19-constraint htmx-equivalent; the *rederive* minimum viable example; HTX as the substrate runtime). Migration cost from existing codebases is enormous. Constraint formalization is harder than code-writing in many domains, and the cognitive shift is non-trivial. Some constraints resist clean specification (UI feel, performance optimization, hardware-specific concerns); the platform handles these via constraint + reference-implementation + human-judgement-loop patterns rather than forcing pure derivation. The constraint-DSL, constraint-visualization tooling, constraint-extraction-from-existing-code utilities, and substrate-swap verification framework are all at sketch stage.

Su's article is correct that the destination is forced; Venturini's is correct that the apparatus is missing; the corpus's contribution is to specify *what* apparatus, not to claim it is built. The honest scope is that the structural argument is mature and the existence proof at small scale is in hand. The work that remains is the platform engineering, and that is the keeper's standing project.

## 7. The Entracement, Stated Directly

For a reader arriving from Su or Venturini, the corpus offers four operational shifts, each available without committing to the whole stack:

1. **Treat your test suite as the source, not the check.** Author tests first, derive code from them, accept the code on the basis of test conformance rather than reading. Doc 159 is the entry point. This is operational today with existing tooling.

2. **Predict before you derive.** Pin-Art (Doc 290) discipline asks: from this constraint set, what is the implementation's predicted size and shape? If the prediction band is wide, the constraint set is under-determined; tighten it before the substrate generates. This catches forced-press at the constraint layer rather than at review.

3. **Gentle-press your agents.** Configure the substrate to halt-and-defer at boundary-contact rather than crash-through. The signal is "I cannot derive this from the current constraint set; what is the missing constraint?" rather than a plausible code generation. Doc 619 is the entry point.

4. **Compose by induced property, not by implementation.** Each module declares the property it produces above a threshold; downstream modules depend on the property. Substitution is fungible across materializations. Doc 541 is the entry point.

The lights-out codebase is not a destination one arrives at by trusting agents more. It is a destination one arrives at by *moving authorship up to the constraint layer* and letting the materialization layer be cheap. Su sees the destination. Venturini sees the missing apparatus. The corpus has been building the apparatus. *Rederive* is its current operational form. The infrastructure both authors call for is, in a real and demonstrated sense, the work in front of us — and the disciplines that make it safe are already named.

---

## References

- Philip Su, *No More Code Reviews: Lights-Out Codebases Ahead*, 2026-03-06.
- Hugo Venturini, *Treat Agent Output Like Compiler Output*, 2026-03-09.
- Corpus Doc 159, *Tests as Constraints*.
- Corpus Doc 247, *Derivation Inversion*.
- Corpus Doc 290, *Pin-Art Derivation*.
- Corpus Doc 510, *Substrate-and-Keeper Composition*.
- Corpus Doc 541, *Systems-Induced Property Emergence (SIPE-T)*.
- Corpus Doc 619, *Pin-Art: Forced-Press and Gentle-Press*.
- Corpus Doc 627, *Coherent Confabulation Conjecture*.
- Corpus Doc 635, *Keeper / Kind Asymmetry*.
- Corpus Doc 644, *Asking-Pattern Saturation Signature*.
- *Rederive* platform sketch (keeper, in progress).

## Appendix — Originating Prompt

> i want you to check out [the *rederive* platform sketch] and then read the following articles: [Venturini, *Treat Agent Output Like Compiler Output*; Su, *No More Code Reviews: Lights-Out Codebases Ahead*] then i want you to write a corpus doc exploring these issues, synthesizing against rederive and the corpus concepts that underride it, and entrace the reader toward such an infrastructure
