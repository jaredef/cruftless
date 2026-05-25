# The Pipeline Pattern Across Subjects
## PRESTO and the JavaScript Engine as Two Realizations of the Same Derivation

*A corpus document responding to the keeper's observation (2026-05-14 02:46Z): "Do you see how you inadvertently created a pipeline in the engine. Look at the PRESTO engine; it has these similar abstractions. Presto is what I started with, and now I've rediscovered WHY it works." Builds on [Doc 247 — The Derivation Inversion](/resolve/doc/247-the-derivation-inversion), [Doc 714 §VI Consequence 5](/resolve/doc/714-the-rusty-bun-engagement-read-through-the-lattice-extension-basin-expansion-at-the-l2m-saturation-point#consequence-5--the-event-loop-belongs-inside-the-engine-amendment-2026-05-14), and [Doc 717 — The Apparatus Above the Engine Boundary](/resolve/doc/717-the-apparatus-above-the-engine-boundary-the-three-projections-lifted-to-engine-substrate-and-the-pure-abstraction-point).*

**Jared Foy · 2026-05-14 · Doc 719**

---

## I. The occasion

The rusty-bun engagement is twenty-three substrate rounds into a hand-rolled JavaScript engine, designed against ECMA-262 + WHATWG and architected per [Doc 717](/resolve/doc/717-the-apparatus-above-the-engine-boundary-the-three-projections-lifted-to-engine-substrate-and-the-pure-abstraction-point)'s engine-cut framework. At round Ω.4.b — the first host-migration sub-round — the resolver-built engine's architecture matched, line-for-concept, a system the keeper had built two years earlier for a different subject entirely: PRESTO, the htxlang server-side pipeline. The keeper named the recognition.

PRESTO is a 22-stage pipeline that resolves htxlang-namespaced HTML into pure HTML through layered directive resolution, two-phase action tokens, and host extension hooks. It has been derived from a ~2,200-word prose seed into seven conformant implementations across seven languages ([PRESTO Engine Derivations](https://github.com/jaredef/presto)). The seven engines are an empirical case for [Doc 247's derivation inversion](/resolve/doc/247-the-derivation-inversion): same constraints (the seed) produce same structural properties across implementation substrates.

The rusty-bun engine is a 23-round pilot derivation against ECMA-262: same method, different subject. The structural correspondence the keeper named in the 02:46Z observation is not a metaphor or a coincidence. It is the derivation inversion operating *across subjects* — same constraint pattern, different domain. PRESTO resolves HTML; rusty-js-runtime resolves ECMA-262 source; both inhabit the same structural shape because both derive from the same constraint pattern.

This document names that correspondence at the corpus tier, traces it through Doc 247's derivation-inversion framework, distinguishes across-language SIPE from across-subject SIPE, and locates the across-subject case in the corpus's evidence base.

## II. The structural correspondence

The two systems, mapped concept-to-concept:

| PRESTO 22-stage pipeline | rusty-js-runtime engine |
|---|---|
| `PipelineContext` threading state through stages | `Frame` threading state through dispatch + `Runtime` threading through job_queue |
| Linear 22-stage pipeline (pre-layout → layout → post-layout) | Linear stages: parser → AST → bytecode → runtime → JobQueue |
| Bilateral boundary: `htx:` namespace = server territory; HTML = client territory | Bilateral boundary: ECMA-262 = engine territory; host-defined behavior (§16+) = host territory |
| `htx:` directive resolution (stages 5–15) | Parse + compile + intrinsic resolution |
| Module extension hooks: `preProcessors` / `postProcessors` | Host hooks: `FinalizeModuleNamespace` / `PollIo` at the E5 cut rung |
| Two-phase mutations via action tokens (prepare/execute) | Two-phase: parse+compile (prepare) → runtime dispatch (execute); bytecode IS the action token |
| Progressive Layers 0–6 (independently adoptable) | L0 lexer → L1 module-loader → L2 platform-builtin → L3 API-shape → L4 idiom → L5 semantics → L6 timing (per [Doc 714 §VI.4](/resolve/doc/714-the-rusty-bun-engagement-read-through-the-lattice-extension-basin-expansion-at-the-l2m-saturation-point#consequence-4--the-layered-constraint-canonicalization-of-substrate-stratum-closure)) |
| Stage 19 post-layout pass: re-resolve directives in injected layout content | Promise reaction microtask drain: re-resolve in injected jobs per §9.4.1 |
| Resolver model: HTML in, pure HTML out | Module evaluator: source in, Module Namespace + drained JobQueue out |
| Eight RFC 2119 conformance contracts | 369 spec-clause-named tests across the engagement |
| `htx:script` extraction + script injection (stages 8, 20) | Function constants pool + closure materialization via MakeClosure |
| `htx:include` / `htx:component` resolution (stages 6, 7) | Module Record linking phase per ECMA-262 §16.2.1 |
| Stage 1 static file serving (before pipeline) | NodeResolver bare-specifier resolution (before parse) |
| Channel API + middleware chain | PollIo host hook + macrotask queue |

Each row of the table is not a loose analogy. Each is a structural identity at the cut-rung level. The two systems are doing the same work — taking a richly-namespaced source format, threading it through a layered pipeline where each layer is independently adoptable, exposing extension hooks at named cut-rungs, separating prepare-time and execute-time via a typed intermediate, and producing a fully-resolved output through phase ordering.

The keeper started with PRESTO. Twenty-three substrate rounds into the rusty-bun engagement, the resolver-built engine had the same structural DNA. The keeper named the recognition. This document records it.

## III. Why the pattern is what it is

Per [Doc 247's statement](/resolve/doc/247-the-derivation-inversion#statement):

> *Every working system is a shadow of the constraints that make it work. The constraints were there before the system. The system participates in them. The engineering work that produced the system did not invent the constraints; it instantiated them, knowingly or not.*

Both PRESTO and rusty-js-runtime are *working systems*. Both *resolve a richly-namespaced source into a pure output*. That resolution problem has a structure: it requires (a) a bilateral boundary that separates the namespaced input language from the pure output language, (b) a way to introduce host-defined behavior at named rungs without smearing the rungs, (c) a typed intermediate that allows prepare-time work to be separated from execute-time work, (d) a phase-ordered run-loop that drains derived work between primary stages.

These four structural requirements are not artifacts of either PRESTO's HTML-resolution subject or rusty-js-runtime's JS-execution subject. They are induced by the *resolution problem itself*. Any conformant realization adopts them. The keeper realized them deliberately in PRESTO; the resolver realized them through Doc 717's E5-rung analysis in rusty-js-runtime. Both arrived at the same shape because the shape is induced by the constraints, not by the language or the subject.

Per the [Doc 474 narrow SIPE](/resolve/doc/474-sipe-standalone-formalization) formulation: a constraint corpus, stated cleanly, *induces* properties at the implementation layer. The properties are the system's structural shape — not its features, but its skeleton. PRESTO's skeleton was named explicitly in the seed; rusty-js-runtime's was derived through twenty-three rounds of engine-cut analysis. Both skeletons are the same skeleton.

## IV. Across substrates vs across subjects

PRESTO's seven engines demonstrate the derivation inversion *across substrates*: same constraints, different implementation languages, same induced properties. The TypeScript engine (~1,555 LOC) and the Rust engine (~54K LOC) share structural properties despite the LOC disparity — the disparity reflects substrate granularity, not constraint divergence.

The PRESTO ↔ rusty-js-runtime correspondence demonstrates the inversion *across subjects*: same constraint pattern, different problem domain, same induced structural shape. PRESTO solves HTML resolution; the engine solves ECMA-262 source resolution. Both adopt the same skeleton because the skeleton is induced by the resolution-problem class, not by the resolution-problem instance.

This is a stronger claim than the across-substrate one. Across-substrate is the standard derivation-inversion case: write the seed once, derive across languages. Across-subject claims that the constraint pattern itself transports between subjects in the same problem class — that the PRESTO pattern is reusable for *any* domain whose work has the structure: richly-namespaced source → bilateral resolver → pure output.

Other candidate domains in the same class: CSS preprocessors (SCSS / Sass), Markdown renderers, GraphQL resolvers, JSX transformers, Pandoc, XSLT, build-system planners (Bazel rules, Nix derivations). Each has a namespaced input, a typed intermediate, host extension hooks, and a phase-ordered resolver. The pattern is portable.

## V. The empirical evidence base

Two distinct realizations now in the corpus's evidence base:

1. **PRESTO** — seven engines across seven languages, all derived from the [PRESTO Seed](https://github.com/jaredef/htxlang/blob/main/seed/presto-seed.md). Each conforms to the same eight RFC 2119 contracts. Per [the Seed Garden](https://jaredfoy.com/garden), this is the corpus's most-validated derivation-inversion case.

2. **rusty-bun engine** — built across 23 substrate rounds in the rusty-bun engagement. Derived against ECMA-262 + WHATWG + an internal PRESTO-pattern shape that emerged through the engagement's substrate work. Verified against 369 spec-clause-named tests. The engine ran `Promise.resolve(42).then(x => x*2).then(record) === 84` end-to-end at round Ω.3.f.d.

These two cases share the structural shape because both are derivations against constraint patterns in the same problem class. The first explicitly cites Doc 247; the second arrived at the pattern through Doc 717's cut-rung analysis without explicit reference to PRESTO. The keeper observed the correspondence at the conjuncture of: (a) host-v2 round Ω.4.b's first concrete host-migration commit, (b) [the host migration design](https://github.com/jaredef/rusty-bun/blob/main/specs/omega-4-host-migration-design.md) predicting -32% LOC reduction (the architectural simplification predicted by the Consequence 5 falsifier), (c) recognition that the resolver-built engine had the same skeleton as PRESTO without having been told to.

The recognition is the engagement's first empirical observation of cross-subject derivation transport. It validates the structural-portability claim implicit in Doc 247.

## VI. The narrow SIPE form supports this case

Per [Doc 474](/resolve/doc/474-sipe-standalone-formalization)'s narrow SIPE: in a constrained derivation, training-signal constraint *density* governs whether the derived implementation exhibits the induced properties. The PRESTO seed has high constraint density (eight RFC 2119 contracts, an explicit bilateral boundary specification, named layer ordering); the engine work has equivalent density (ECMA-262 + Doc 717's cut-rung analysis + Doc 714 §VI's layer hierarchy).

Both meet the narrow-SIPE prerequisite. The cross-subject identity of the resulting pattern is consistent with — not a falsification of — the narrow form. The pattern is not claimed to operate across arbitrarily-different subjects; it is claimed to operate across subjects in the same resolution-problem class, under high-density constraint specification.

This distinguishes the present case from the universality claims [deprecated in Doc 143](/resolve/doc/143-sipe#deprecation-notice). The pattern is portable inside the resolution-problem class. The corpus does not need to invoke universal-SIPE to account for this case.

## VII. Falsifier

The cross-subject claim is testable. Three predictions:

**Pred-719.1.** Any future derivation against a constraint corpus in the resolution-problem class (CSS preprocessor, GraphQL resolver, Markdown renderer, build-system planner) will arrive at the same five structural features: (a) bilateral boundary between input language and output language, (b) named extension hooks at the host-defined-behavior rung, (c) typed intermediate separating prepare from execute, (d) progressive layers independently adoptable, (e) phase-ordered run-loop with derived-work drain between primary phases.

Falsifier: a derivation arrives at a conformant implementation that lacks one or more of these five features without imposing a compensation stack to recover them. The implementation would constitute a fifth structural feature the pattern doesn't account for, or it would falsify the claim that the listed features are necessary.

**Pred-719.2.** The post-Ω.4 rusty-bun-host LOC will be ~32% smaller than the pre-Ω.4 host LOC (per [Doc 714 §VI Consequence 5](/resolve/doc/714-the-rusty-bun-engagement-read-through-the-lattice-extension-basin-expansion-at-the-l2m-saturation-point#consequence-5--the-event-loop-belongs-inside-the-engine-amendment-2026-05-14)'s falsifier). The LOC reduction is the empirical signature of the pattern's structural correctness: when the bilateral boundary moves to the named cut-rung, the conforming realization is smaller than the realization that smears the boundary across substrates.

Falsifier: post-Ω.4 measurement shows no LOC reduction. The pattern claim still stands, but the engagement's specific Ω.4 architecture would not have realized it.

**Pred-719.3.** A future derivation against the htxlang spec in an engine substrate not previously tried (e.g., Erlang/OTP, OCaml, F#, Kotlin) will produce the same structural shape as the existing seven PRESTO engines. The eighth implementation's contract pass-rate against the same eight RFC 2119 contracts is the operational test.

Falsifier: the eighth implementation fails to converge on the same structural shape under the same prose seed, despite high constraint density. This would falsify either the across-substrate claim (Doc 247) or the constraint-density prerequisite (Doc 474's narrow SIPE).

The first two predictions are the corpus contributions; the third is the standard derivation-inversion test repeated.

## VIII. Operational consequence

The recognition reshapes how the engagement's remaining work reads.

The rusty-bun engine is not just "a derivation against ECMA-262." It is the *PRESTO pattern applied to the JavaScript-execution subject*. Every cut-rung decision in the engine — Tuple A's HostFinalizeModuleNamespace closure rung (3.d.f), the event-loop's E5 attachment point (3.f.a–d), the run_to_completion phase ordering (3.f.b) — corresponds to a named feature of the PRESTO seed. The correspondence is not accidental; it's structural.

This also means the *next* derivations the engagement might attempt — beyond rusty-bun — have a smaller learning cost. The pattern is named at the corpus tier. New domains (CSS preprocessing, build-system planning, etc.) can adopt the pattern by referencing this document rather than re-deriving from first principles.

The Tier-Ω.4 host migration's predicted ~32% LOC reduction is now interpretable as Pred-719.2 of this document, not just as a Tier-Ω-specific architectural shift. The cross-subject framing makes the prediction portable: any host migration in this class, when the bilateral boundary moves to the named cut-rung, should exhibit a similar LOC reduction.

## IX. The hypostatic boundary

Per [Doc 372's hypostatic boundary discipline](/resolve/doc/372-the-method-of-the-corpus-as-derivation-not-collection): this document records a structural recognition observed in two specific empirical cases. It does not assert that all resolution-problem-class systems exhibit this pattern by necessity, only that two well-documented derivations against high-density constraint corpora arrived at the same shape, and that the resulting shape is consistent with Doc 247's prediction. The portability claim (§IV) is operationally testable per §VII; the testing is the empirical work.

The pattern is named; the name is at the corpus tier; future engagements can reference it rather than re-derive. The recognition is the contribution.

## X. Closing

The keeper started with PRESTO. The keeper directed the rusty-bun engagement. At round Ω.4.b, the resolver-built engine had the same structural skeleton as PRESTO, arrived at through twenty-three rounds of cut-rung analysis without explicit reference. The keeper named the correspondence.

The correspondence is the derivation inversion operating across subjects: same constraint pattern, different domain, same induced shape. The pattern is portable inside the resolution-problem class. Two empirical cases are now in the corpus's evidence base. The cross-subject claim is operationally falsifiable per §VII.

The engagement's substrate work — twenty-three rounds of careful cut-rung derivation — is, in this reading, an empirical case of Doc 247's structural prediction applied to a new subject. The work was productive at the substrate tier; the recognition is productive at the corpus tier. The articulation is the engagement's contribution to the corpus's evidence base for the cross-subject portability of the pattern.

---

*Companion documents: [Doc 247 — The Derivation Inversion](/resolve/doc/247-the-derivation-inversion); [Doc 474 — SIPE Standalone Formalization](/resolve/doc/474-sipe-standalone-formalization); [Doc 714 §VI Consequence 5](/resolve/doc/714-the-rusty-bun-engagement-read-through-the-lattice-extension-basin-expansion-at-the-l2m-saturation-point#consequence-5--the-event-loop-belongs-inside-the-engine-amendment-2026-05-14); [Doc 717 — The Apparatus Above the Engine Boundary](/resolve/doc/717-the-apparatus-above-the-engine-boundary-the-three-projections-lifted-to-engine-substrate-and-the-pure-abstraction-point); [Doc 716 — Stubs as Named Cuts](/resolve/doc/716-stubs-as-named-cuts-the-three-projection-tracker-and-the-stub-alphabet-stability-conjecture). External references: [PRESTO Engine Derivations](https://github.com/jaredef/presto); [htxlang specification](https://github.com/jaredef/htxlang); [the Seed Garden](https://jaredfoy.com/garden).*
