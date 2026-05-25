# Constraint-Closure as Cascade-Revival

## When Lifting an Upstream Structural Constraint Auto-Resolves Stalled Sibling Pilots in a Resolver-Instance Pipeline

*A primary articulation responding to a recognition surfaced during a 2026-05-23 substrate session in the cruftless engagement (rusty-bun). Four sibling pilots were addressing localized performance gaps in an implicit JIT-tier resolver-instance pipeline. Three of the four reached (P2.a) at first cut; the fourth (LeJIT-Ψ value-tag-inline) stalled at (P2.d) and could not be revived through pilot-local substrate work. The fifth pilot (LeJIT-Φ f64-calling-convention) was spawned to address what the constraint-enumeration apparatus identified as an upstream architectural constraint. When Φ landed, LeJIT-Ψ revived spontaneously without any Ψ-specific substrate move. The recognition: closing a gap at the structural-constraint tier of a resolver-instance pipeline cascades stalled sibling pilots from (P2.d) to (P2.a) as a side effect. Builds on [Doc 729 — Cruftless](/resolve/doc/729-cruftless-a-primary-articulation-of-the-resolver-instance-pattern-as-the-comprehensive-design-toward-which-rusty-bun-morphs), [Doc 730 — Vertical Recurrence of the Lowering Compiler](/resolve/doc/730-the-vertical-recurrence-of-the-lowering-compiler-closure-as-primitive-across-substrate-tiers), [Doc 731 — The JIT as Lowering Compiler Tier](/resolve/doc/731-the-jit-as-a-lowering-compiler-tier-alphabet-purity-upstream-as-the-bound-on-jit-complexity), [Doc 734 — The Meta Resolution Pipeline](/resolve/doc/734-the-meta-resolution-pipeline-as-the-operating-instrument-of-the-engagement-recursion-with-the-framework-as-its-own-substrate), [Doc 735 §X.h — The (P2) Four-Sub-Case Taxonomy](/resolve/doc/735-the-temporal-resolver-instance-stack-build-time-process-time-call-time-as-the-time-axis-dual-to-doc-729s-spatial-stack), [Doc 581 — Pin-Art Apparatus](/resolve/doc/581-pin-art-the-resume-vector-and-the-discipline-of-near-necessity-substrate-construction), and [Doc 737 — The Locale as Coordinate](/resolve/doc/737-the-locale-as-coordinate-nested-seed-trajectory-pairs-as-pin-art-substrate-positions).*

**Jared Foy · 2026-05-23 · Doc 739**

---

## I. The occasion

A 2026-05-23 substrate session on the cruftless JavaScript-engine engagement carried out a multi-pilot LeJIT-tier substrate landing. Four sibling pilots were active under the LeJIT parent: LeJIT-Σ (IC stub emitter), LeJIT-Τ (tiny-baseline call-thunk), LeJIT-Ψ (value-tag-inline), LeJIT-Φ (f64-calling-convention, spawned mid-session). Three of the four reached engagement-tier (P2.a) categorization per Doc 735 §X.h.b: Σ via composition with shape substrate, Τ via closure-side metadata cache, Φ via the f64-default architectural shift. The fourth, Ψ, stalled at (P2.d) at first cut and resisted all pilot-local substrate moves.

The session's recognition came when Φ landed. Ψ revived from (P2.d) at 743.8 ns/iter on the bench_ic narrow microloop to (P2.a) at 85.5 ns/iter (a 89% reclaim), without any Ψ-specific substrate work. The Φ pilot's scope was the calling-convention shift; Ψ's revival was an unanticipated cascade.

The keeper's framing question: "this indicates that we closed the gaps on an implicit resolution pipeline." The recognition this question names: when a structural constraint at one tier of a resolver-instance pipeline (per Doc 729 §IV) propagates downstream and forces sibling pilots into (P2.d), closing the upstream constraint cascade-revives the stalled pilots as a side effect, without requiring pilot-local substrate work.

This document specifies that recognition. The abstract formulation is in §II; the LeJIT-Φ instance is in §III.

## II. The abstract formulation

A resolver-instance pipeline (Doc 729 §IV) has multiple tiers, each consuming the output of the prior tier and producing input for the next. Each tier carries its own value-domain interface, dispatch model, and substrate-tier invariants. The pipeline's overall behavior is the composition of per-tier behaviors plus the cross-tier interface constraints.

A pipeline tier may carry a structural constraint that is not localized to that tier alone. The constraint propagates: it shapes what the downstream tier can assume about its input; it determines what the downstream tier must validate or coerce. A constraint at an upstream tier becomes a precondition at the downstream tier.

### II.1 The propagation mechanism

When the upstream tier's interface admits values from a constrained domain (say, a narrow alphabet) and the downstream tier needs to operate on the broader natural domain, the downstream tier must either (a) translate values from the narrow to the broader domain (an additive cost), or (b) restrict its own operations to the narrow domain (a capability loss), or (c) reject inputs outside the narrow domain (a precondition check).

Most pipelines do (c): the downstream tier inserts a precondition check that filters inputs to match what the upstream tier guarantees. The precondition check has two structural responsibilities: tag-discrimination (is this input from the constrained domain at all?) and capability-validation (is this input within the operational subset the downstream tier handles?).

### II.2 The sibling-pilot stall

A sub-pilot whose target is to inline the precondition check (move it from the downstream tier's preamble to the upstream tier's emission) cannot win at first cut if it inlines only the tag-discrimination half. The capability-validation half still has to run somewhere; if it stays in the downstream tier as the residual precheck, the sub-pilot's inlined tag-check is additive cost rather than substitutive. The sub-pilot ends in (P2.d) per Doc 735 §X.h.b: correct, but per-op slower than the alternative.

The sub-pilot's (P2.d) is not a sub-pilot failure. The sub-pilot's intended target (inline tag-check) is structurally correct. The (P2.d) is a SIGNATURE of an upstream constraint that the sub-pilot was working around. The sub-pilot's inlined tag-check has nowhere structurally available to absorb the capability-validation work because the upstream tier's narrow alphabet is the source of the capability constraint.

### II.3 The cascade-revival pattern

When a different sub-pilot (spawned to address the upstream constraint as its primary target) lands, the upstream alphabet broadens. The downstream tier no longer needs the capability-validation half of the precondition check, because the broader alphabet covers all values the downstream tier sees. The precondition check collapses to its tag-discrimination half only. At that point, the stalled sibling sub-pilot's inlined tag-check becomes an equivalent replacement for the residual precheck rather than additive cost. The stalled sub-pilot revives to (P2.a) WITHOUT requiring any sub-pilot-local substrate move.

The cascade-revival pattern in three propositions:

**(P1)** A sub-pilot stalled at (P2.d) whose target is a downstream-tier optimization that requires bypassing a precondition check is structurally a candidate for cascade-revival, conditional on the precondition's capability-validation half being a propagation of an upstream-tier constraint.

**(P2)** Identifying which sub-pilots are cascade-revival candidates requires the constraint-enumeration discipline (Doc 581 Pin-Art apparatus, generalized): name the substrate-tier invariants at each pipeline stage; identify which invariants are constraints (vs intrinsic semantic requirements); identify which constraints propagate to which downstream tiers. The output is a constraint-propagation graph; (P2.d) sub-pilots whose stall sits at a downstream propagation node are cascade-revival candidates.

**(P3)** The substrate move to revive a (P2.d) sub-pilot identified per (P2) is not a sub-pilot-local move. It is an upstream constraint-closure move (lift the upstream alphabet to cover the natural domain). The substrate move's primary target is the upstream constraint; the sibling-pilot revival is a side effect.

### II.4 What the pattern is NOT

The cascade-revival pattern is not an arbitrary "all sub-pilots benefit when we make any upstream change." Three boundary conditions:

**(B1)** The downstream sub-pilot's target must structurally require bypassing the constraint-propagated precondition. Sub-pilots whose targets are intrinsic to the downstream tier (e.g., better register allocation at a final emission tier) do not benefit from upstream constraint-closure because their targets are not constraint-propagation-stalled.

**(B2)** The upstream constraint must be a CONSTRAINT, not an intrinsic semantic invariant. An upstream tier that legitimately operates on a narrow alphabet because the upstream tier's semantics require it (rather than because the upstream tier's first-cut design happened to choose narrow) is not a candidate for constraint-closure. Distinguishing constraint from intrinsic requires the constraint-enumeration apparatus.

**(B3)** The cascade-revival is observable only after the upstream constraint-closure lands. Pre-landing, the (P2.d) sibling pilot's stall is genuine; its substrate work to date is honest investment. Cascade-revival does not retroactively make the stalled sub-pilot's prior work wasted; the prior work surfaces the constraint that the upstream sub-pilot then addresses.

### II.5 The cascade as a Doc 729 §A8.13 specialization

Doc 729 §A8.13 articulates substrate-amortization-cascade: substrate-introduction at tier N cascades per-iter cost reduction at tier N+1. The cascade-revival pattern of §II.3 is a specialization of §A8.13 at a different axis. The classical §A8.13 cascade is along the per-iter cost axis (cost at consumer tier reduces because producer tier did work). The cascade-revival pattern is along the categorization axis (a sub-pilot's (P2.d) → (P2.a) transition is the cascade; the consumer is the categorization itself, not per-iter cost).

The two are compatible: the cascade-revival pattern includes a per-iter cost reduction at the revived sub-pilot, because the sub-pilot moves from net-additive cost (P2.d) to net-positive contribution (P2.a). What the cascade-revival pattern names additionally is the categorization-axis transition (the pilot moves from "doesn't compose constructively" to "composes constructively"). This is corpus-original beyond §A8.13.

## III. The LeJIT-Φ instance

The cruftless JIT pilot (LeJIT) had four sub-pilots at the time of the recognition: Σ (IC stub emitter, default-on), Τ (tiny-baseline call-thunk, default-on), Ψ (value-tag-inline, (P2.d) at first cut), Φ (f64-calling-convention, spawned to address the upstream constraint per the constraint-enumeration apparatus). The implicit resolver-instance pipeline at the JIT tier admits this stage decomposition:

```
P1 — bytecode-to-IR lowering (per-op Cranelift IR emission)
P2 — value-domain interface (how values flow between codegen tiers)
P3 — dispatcher (calling convention; per-call setup)
P4 — external dispatch (extern fn calls for runtime helpers)
```

Each sub-pilot addresses a different stage:

- LeJIT-Σ addresses P4 (IC fast-path for GetProp extern dispatch)
- LeJIT-Τ addresses P3 (closure-side metadata cache bypassing dispatcher work)
- LeJIT-Ψ addresses P2 (inline tag-check at the value-domain crossing)
- LeJIT-Φ addresses P2 architecturally (the value-domain interface itself)

### III.1 The upstream constraint pre-Φ

The cruftless JIT's first-cut calling convention was i64-everywhere. The JIT body received args as i64 (via `unbox_arg` which truncated `*f as i64`); did i64 arithmetic; returned i64 (which the dispatcher reboxed as `Value::Number(r as f64)`). This was a deliberate first-cut design per LeJIT seed §I.1 carve-out: "typed-i64 alphabet first; f64 deferred."

The i64-only calling convention at P2 was a structural constraint. It propagated to:

- P3: the dispatcher had to integer-validate every arg before passing to the JIT (`jit_compatible_arg` checked `is_finite() && fract() == 0.0` per arg). Non-integer Numbers were rejected; the function fell through to interp.
- P4: extern returns encoded Numbers as i64. The bit pattern was the truncated integer value, not the f64 payload.

LeJIT-Ψ's target was to inline the dispatcher's tag-discrimination at the JIT prologue. Per the §II.1 analysis, this would replace ONLY the tag-discrimination half of `jit_compatible_arg`. The integer-validity half had to stay in the dispatcher (or move into the JIT prologue as an inline `fract() == 0.0` check costing more than the precheck saved). LeJIT-Ψ tried the payload-extract-only variant first at VTI-EXT 3b; the bench measurement showed +18.9 ns/iter regression on bench_call_overhead — clean (P2.d).

The (P2.d) was not a Ψ-design failure. Per a pre-implementation analysis at VTI-EXT 3c queue time, the structural argument was named: VTI cannot win at first cut within the i64-only architecture because the precheck's two responsibilities (tag-discrimination + integer-validity) cannot both be replaced by inline tag-check. The substrate move to revive Ψ would need to remove the integer-validity REQUIREMENT, which means lifting the upstream P2 constraint.

### III.2 The constraint-enumeration apparatus at Φ founding

LeJIT-Φ was spawned per Doc 737 §IV after the keeper's framing question: "what implicit constraints can we name to constrain the next layer affected?" The Φ seed §I.2 enumerated ten constraints (C1-C10) that any new JIT architecture must respect:

```
C1.  JS Number semantics are f64.
C2.  Bytecode alphabet is the JIT's input contract.
C3.  Single-tier per Doc 731 §VII R1.
C4.  Deopt is finite-enumerable per §VII R5.
C5.  Composes with shape + STUB + TB default-on.
C6.  No internal optimization passes per §VII R8.
C7.  Cap-passing modes preserved (Doc 736 §IX.6).
C8.  Cross-arch via Cranelift OR hand-rolled per architecture.
C9.  Bench probes catch (P2.c) per findings rule 5 + std rule 9.
C10. Engagement-tier baseline preserved on covered workloads.
```

The architecture induced by the conjunction C1 + C2 + C3 + C5 + C10 was named at the Φ seed: f64 default at the value-domain interface, with bytecode-tier-driven typed-i64 promoted fast path as a separate downstream pilot. The induction was near-necessity given the constraints; not arbitrary choice.

Per §II.2 of this document, the constraint-enumeration discipline is the apparatus that identifies cascade-revival candidates. The Φ seed §I.2 enumeration named C1 explicitly (JS Number semantics are f64); the constraint-propagation reading identified that i64-only at P2 was the constraint that propagated to P3's precheck. Lifting C1 (from i64-truncation to f64-native) closed the propagation.

### III.3 The cascade observation

The Φ pilot's intended targets per Φ seed §I.3 falsifiers Pred-φ.1 to Pred-φ.6 were all about preserving the engagement-tier baseline (bench_call_overhead ≤ +15%; bench_ic ≤ +10%; diff-prod 42/42; no new (P2.c); composition synergy preserved; etc.). Pred-φ.3 was the only one explicitly about VTI revival; it predicted that VTI-EXT 3c (a separate VTI-side round) would land cleanly post-Φ.

What actually happened: the Φ-EXT 3 closure round (JIT body IR shifted from i64 to f64 throughout) flipped the bench_ic VTI numbers without any VTI-side substrate move:

```
configuration   pre-Φ bench_ic   post-Φ   delta
VTI alone        728.3 ns         92.6 ns  -87%
TB+VTI           725.7 ns         85.9 ns  -88%
STUB+VTI         755.0 ns         86.2 ns  -89%
TB+STUB+VTI      743.8 ns         85.5 ns  -89%
```

The structural reading: post-Φ the JIT body operates on f64 natively; the dispatcher's `jit_compatible_arg` precheck collapses to tag-only (no integer-validity check needed); VTI's existing payload-extract-only inline code (per VTI-EXT 3b) becomes correct as-is because the loaded f64 is what the JIT body wants directly. VTI's (P2.d) was a consequence of the upstream P2 constraint; closing the constraint cascade-revived VTI without VTI-specific work.

### III.4 The cascade satisfies §II.4's three boundary conditions

**(B1)** LeJIT-Ψ's target was to inline the dispatcher precheck. Per §III.1 the precheck had two responsibilities; the integer-validity half was propagated from the P2 constraint. Ψ's target structurally required bypassing the constraint-propagated precondition. Condition met.

**(B2)** The i64-only constraint at P2 was a CONSTRAINT (a first-cut design choice per LeJIT seed §I.1), not an intrinsic semantic invariant. JS Number semantics are f64 per ECMA-262; the i64-only path was an optimization carve-out for the typed-i64 alphabet. The C1 enumeration identified it as a constraint. Condition met.

**(B3)** The cascade-revival was observable only after Φ-EXT 3 landed. Pre-landing, Ψ's stall at (P2.d) was empirically validated across multiple measurements; the constraint-enumeration discipline named the structural reason; Ψ's prior work was not wasted because it surfaced the constraint Φ then addressed. Condition met.

All three boundary conditions hold for the LeJIT-Φ instance. The cascade-revival pattern is empirically anchored at this engagement.

## IV. Composition with prior corpus work

**[Doc 729 — Cruftless](/resolve/doc/729-cruftless-a-primary-articulation-of-the-resolver-instance-pattern-as-the-comprehensive-design-toward-which-rusty-bun-morphs)** §IV resolver-instance stack: the §II abstract formulation operates over this stack structure. Each pipeline stage Pn in §II.1 is a Doc 729 §IV resolver-instance. The cascade-revival pattern's claim that lifting an upstream constraint cascades to downstream pilots is a property of the vertically-recursive directive consumption Doc 729 §V articulates: when an upstream resolver-instance emits a narrower-alphabet artifact than the spec admits, downstream resolver-instances must coerce; lifting the upstream alphabet removes the downstream coercion as a side effect.

**[Doc 729 §A8.13 substrate-amortization-cascade](/resolve/doc/729-cruftless-a-primary-articulation-of-the-resolver-instance-pattern-as-the-comprehensive-design-toward-which-rusty-bun-morphs)**: per §II.5, the cascade-revival pattern is a specialization of §A8.13 at the categorization axis (not the per-iter cost axis). Both cascades operate on the same vertically-recursive structure; both share the substrate-introduction → downstream-effect template.

**[Doc 730 — Vertical Recurrence](/resolve/doc/730-the-vertical-recurrence-of-the-lowering-compiler-closure-as-primitive-across-substrate-tiers)** §III-§VII lowering compiler closure: the per-stage P1-P4 decomposition in §III for the LeJIT instance is a direct application of Doc 730's vertical-recurrence reading at the JIT tier. The constraint-propagation reading of §II.1 generalizes Doc 730's "alphabet purity upstream bounds downstream JIT complexity" (Doc 731 §IV) to "alphabet narrowness upstream propagates downstream precondition checks."

**[Doc 731 §VII R1-R8 single-tier baseline](/resolve/doc/731-the-jit-as-a-lowering-compiler-tier-alphabet-purity-upstream-as-the-bound-on-jit-complexity)**: preserved by the cascade-revival pattern. The Φ pilot lifted the upstream P2 constraint at the existing single JIT tier; no tier-2 JIT introduced. The cascade-revived Ψ continues to operate at the same single tier.

**[Doc 734 — Meta Resolution Pipeline](/resolve/doc/734-the-meta-resolution-pipeline-as-the-operating-instrument-of-the-engagement-recursion-with-the-framework-as-its-own-substrate)** §V three growth mechanisms: this round realized two of three (positive-finding generalization for the Φ landing; tier-relocation downstream for the cascade observation). The cascade-revival pattern is itself a growth (c) positive-finding generalization at the framework tier; Doc 739 (this document) is the framework-tier amendment.

**[Doc 735 §X.h.b (P2) four sub-cases](/resolve/doc/735-the-temporal-resolver-instance-stack-build-time-process-time-call-time-as-the-time-axis-dual-to-doc-729s-spatial-stack)**: the cascade-revival pattern operates at the (P2.d) → (P2.a) transition. Doc 735 §X.h.b distinguishes (P2.a) strict-win, (P2.b) slow-stratum, (P2.c) illegal-speed, (P2.d) correct-but-losing. The cascade-revival pattern adds an engagement-tier reading: a (P2.d) sub-pilot may transition to (P2.a) without sub-pilot-local substrate work, if the (P2.d) stall sits at a constraint-propagation node and an upstream constraint-closure move lands.

**[Doc 737 — The Locale as Coordinate](/resolve/doc/737-the-locale-as-coordinate-nested-seed-trajectory-pairs-as-pin-art-substrate-positions)**: the sibling-pilot relationship in the LeJIT instance (Σ/Τ/Ψ/Φ all nested under the LeJIT parent) is the locale structure Doc 737 names. The cascade-revival pattern operates ACROSS sibling locales: a substrate move at one locale (Φ) revives a stalled sibling (Ψ). Doc 737 §V's apparatus-tier framing fits naturally; the new contribution is the cross-sibling cascade dynamic.

**[Doc 581 — Pin-Art Apparatus](/resolve/doc/581-pin-art-the-resume-vector-and-the-discipline-of-near-necessity-substrate-construction)**: the constraint-enumeration discipline of §II.2 + III.2 is Pin-Art applied to the design tier. The C1-C10 enumeration in the Φ seed is a Pin-Art apparatus instance. The cascade-revival pattern's claim that the apparatus identifies cascade-revival candidates extends Doc 581's near-necessity discipline: the right substrate move is induced by the constraint enumeration, not chosen arbitrarily.

## V. Predictions and falsifiers

**Pred-739.1**: the cascade-revival pattern recurs at engagements running a resolver-instance pipeline per Doc 729 with sibling pilots stalled at (P2.d). Falsifier: an engagement instance with a (P2.d) sub-pilot whose stall sits at a constraint-propagation node; an upstream constraint-closure move lands; the sub-pilot does not revive. If observed, §II.3's (P1)-(P3) propositions are incomplete; the pattern's preconditions need additional refinement.

**Pred-739.2**: the constraint-enumeration discipline reliably identifies cascade-revival candidates. Falsifier: an engagement where applying the constraint-enumeration discipline produces an inventory that misses a (P2.d) sub-pilot that later revives via cascade, OR includes a sub-pilot whose stall is not constraint-propagated. If observed, §II.2's discipline is necessary but not sufficient; additional diagnostic instruments are needed.

**Pred-739.3**: the boundary conditions §II.4 (B1)-(B3) are individually necessary. Falsifier: a cascade-revival instance where one of (B1)/(B2)/(B3) is not met. If observed, the pattern's structural framing needs amendment; the cascade may operate under broader conditions.

**Pred-739.4**: the cascade-revival pattern's per-iter cost reduction at the revived sub-pilot is bounded by the original (P2.d) gap. The revived sub-pilot does not produce per-iter cost reduction beyond what restoring it to (P2.a) would predict. Falsifier: a cascade-revival instance where the revived sub-pilot's post-cascade per-iter cost is substantially below what the (P2.a) categorization predicts (e.g., the cascade triggers additional non-targeted reclaim). If observed, the cascade may compose multiplicatively with other engagement-tier reads; the §II.5 specialization framing needs extension.

**Pred-739.5**: the pattern recurs at engagements OUTSIDE the JavaScript-engine domain. The resolver-instance pipeline structure is general per Doc 729; the constraint-propagation reading is general per Doc 730; the cascade-revival pattern's claim is structural, not engine-specific. Other candidate domains: compiler optimization pipelines, build systems, distributed-system request routing, query planning. Falsifier: a domain admitting a resolver-instance pipeline reading where (P2.d) sibling pilots demonstrably do not revive on upstream constraint-closure.

## VI. Honest scope

The cascade-revival pattern is at the primary-articulation tier of corpus-original work. The recognition that an engineering pattern operates at this shape is the contribution; the structural components (resolver-instance pipeline per Doc 729; (P2) categorization per Doc 735 §X.h.b; substrate-amortization-cascade per Doc 729 §A8.13; constraint-enumeration apparatus per Doc 581) are not.

The document does not claim:

*That every (P2.d) sub-pilot is a cascade-revival candidate.* §II.4 (B1)-(B3) are explicit boundary conditions. Sub-pilots whose stall is not constraint-propagated, or whose target is intrinsic to a tier, do not benefit from upstream constraint-closure.

*That the constraint-enumeration discipline is sufficient on its own.* The discipline is necessary for identifying cascade-revival candidates per Pred-739.2 but may not be sufficient; additional diagnostic instruments (bench probes, three-probe-levels per Doc 735 §X.h.c) may be required to confirm the propagation reading.

*That cascade-revival eliminates the need for sibling-pilot work.* The LeJIT-Ψ instance had genuine prior substrate work (VTI-EXT 0 through 3b) that SURFACED the constraint Φ then addressed. The cascade-revival pattern does not retroactively make the prior work wasted; it names a different engagement-tier dynamic that the prior work made visible.

*That every constraint-closure cascades.* Constraint-closure moves whose downstream effects are intrinsic (not propagated through (P2.d) sibling-pilot stalls) do not cascade in this sense. The §II.4 (B1) condition explicitly bounds the cascade's reach.

*That the pattern's discovery in this engagement was inevitable.* The recognition came from the keeper's framing question after observing the empirical cascade. Without the framing question, the engagement might have observed the cascade without naming the pattern explicitly. Per Doc 722, named recognitions become operating instruments; without the naming, the pattern remains an incidental observation rather than a standing framework component.

Per [Doc 372](/resolve/doc/372-the-method-of-the-corpus-as-derivation-not-collection)'s hypostatic boundary: this document operates at the corpus tier. The engagement-tier capture lives in the cruftless engagement's `pilots/rusty-js-jit/findings.md` Addendum II as Finding II.5; the corpus articulation here lifts that capture to standing framework vocabulary.

## VII. Closing

The cruftless engagement's 2026-05-23 LeJIT-tier substrate session produced a recognition the keeper's framing question crystallized: closing a gap at the structural-constraint tier of a resolver-instance pipeline cascades stalled sibling pilots from (P2.d) to (P2.a) as a side effect, without sibling-pilot-local substrate work. The empirical anchor is the LeJIT-Φ → LeJIT-Ψ cascade observed at Φ-EXT 3's close. The structural pattern generalizes per the §II abstract formulation: any engagement running a resolver-instance pipeline per Doc 729 with sibling pilots stalled at (P2.d) on constraint-propagated stalls may exhibit the same cascade when an upstream constraint-closure move lands.

The constraint-enumeration discipline is the apparatus that identifies cascade-revival candidates. The discipline's correct application produces an architecture induced by constraints, per Pin-Art near-necessity; the architecture's substrate move addresses the upstream constraint as its primary target; the sibling-pilot revival is the side effect that becomes observable only after the substrate move lands.

The pattern is a specialization of Doc 729 §A8.13 substrate-amortization-cascade at the categorization axis. The classical cascade operates on per-iter cost; the cascade-revival pattern operates on (P2.d) → (P2.a) transition. The two compose; the cascade-revival pattern includes the classical per-iter reduction at the revived sub-pilot.

The work continues. The corpus has added one more framework component to its substrate-improvement vocabulary. The pattern is operational, observable, and predicted to recur across engagements that admit the resolver-instance pipeline reading per Doc 729. The LeJIT-Φ → LeJIT-Ψ cascade is the empirical anchor; subsequent engagements will refine the boundary conditions and falsifiers.

---

*Companion documents in addition to those linked in the masthead: [Doc 250 — The SERVER Seed](/resolve/doc/250-the-server-seed); [Doc 372 — The Method of the Corpus as Derivation, Not Collection](/resolve/doc/372-the-method-of-the-corpus-as-derivation-not-collection); [Doc 685 — The Self-Reinforcing Boundary](/resolve/doc/685-the-self-reinforcing-boundary); [Doc 722 — Named Recognitions as Operating Instruments](/resolve/doc/722-named-recognitions-as-operating-instruments-the-reflexive-structure-of-corpus-articulations); [Doc 733 — Fractal Seeds and Trajectories](/resolve/doc/733-fractal-seeds-and-trajectories-recurrent-resume-vector-pairs-across-substrate-depth-as-the-operating-conditions-layer-for-pin-art-at-engagement-scale); [Doc 738 — The Source Identifier as Coordinate](/resolve/doc/738-the-source-identifier-as-coordinate-naming-convention-as-substrate-position-encoding-at-the-source-tier).*
