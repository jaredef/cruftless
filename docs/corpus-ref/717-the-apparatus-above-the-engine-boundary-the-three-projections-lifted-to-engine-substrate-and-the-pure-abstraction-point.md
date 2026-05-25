# The Apparatus Above the Engine Boundary
## The Three Projections Lifted to Engine-Substrate and the Pure Abstraction Point

*A corpus document responding to the keeper's conjecture (2026-05-13 20:06Z): "We can use the DAG / lattice / alphabet to follow the substrate / surface up to the pure abstraction point of the entire engine." Builds on [Doc 714 sub-§4.b (the layer-floor cut-location framework)](/resolve/doc/714-the-rusty-bun-engagement-read-through-the-lattice-extension-basin-expansion-at-the-l2m-saturation-point), [Doc 715 §X–§XI (the consumer-substrate DAG as load-bearing object)](/resolve/doc/715-the-consumer-substrate-dependency-graph-as-the-load-bearing-object-beneath-the-joint-mi-lattice), and [Doc 716 (stubs as named cuts; the three-projection tracker)](/resolve/doc/716-stubs-as-named-cuts-the-three-projection-tracker-and-the-stub-alphabet-stability-conjecture). Folded into the engagement's telos via the same session's resume-vector update.*

**Jared Foy · 2026-05-13 · Doc 717**

---

## I. The occasion

The 2026-05-13 night close established the engagement's first scalar parity metric: 88.2% (105 of 119) byte-identical to Bun on the curated top-N package list. Five substrate moves landed in the bridge layer that day, returning +24 packages from 68.0% to 88.2%; the first of those — a one-line substitution of `Object.getOwnPropertyNames` for `Object.keys` in the CJS→ESM bridge — returned +16 packages by itself, the highest substrate-fanout return recorded in the engagement.

The cluster-by-cluster diagnosis of the remaining 14 failures produced a single coherent finding. Every residual decomposes into Bun-specific import-binding synthesis behaviors that the embedded engine (rquickjs over QuickJS) cannot reproduce above the engine boundary and that QuickJS's own parser cannot accept. Five distinct synthesis behaviors observed: (i) ESM-without-`export default` → default = namespace; (ii) ESM-with-only-`export default X` → expose X's own properties as named exports; (iii) `export default function NAME` → also expose NAME as named export; (iv) string-literal export aliases (`export { x as 'm-search' }`); (v) modern parser features beyond QuickJS's grammar acceptance (class-field arrow-fn variants, minified-ESM patterns that trigger SIGSEGV).

For the first time in the engagement, the productive surface lies *below* the embedded engine boundary. Above-engine substrate is empirically exhausted at the curated-119 scale; the K1/K2/K3 stub stratum from [Doc 716](/resolve/doc/716-stubs-as-named-cuts-the-three-projection-tracker-and-the-stub-alphabet-stability-conjecture) cannot reach this work. Per the keeper directive landing the same evening (19:53Z), the QuickJS hand-roll folds into telos as Tier-Ω.

The keeper's subsequent conjecture (20:06Z) named the structural move that makes Tier-Ω legible: the three projections of [Doc 715](/resolve/doc/715-the-consumer-substrate-dependency-graph-as-the-load-bearing-object-beneath-the-joint-mi-lattice) and [Doc 716](/resolve/doc/716-stubs-as-named-cuts-the-three-projection-tracker-and-the-stub-alphabet-stability-conjecture) — DAG, lattice, alphabet — are not specific to host-substrate. They generalize. Lifted across the engine boundary, they name the engine's own structure all the way up to a *pure abstraction point* where the three projections collapse into a single formal-semantics object: the ECMA-262 + WHATWG specifications taken together as a limit.

This document develops the lift. The thesis: the apparatus that Doc 715 and Doc 716 developed at host-substrate scale generalizes one level up to engine-substrate scale, with the same three projections producing analogous operational outputs, and with a stable alphabet at cardinality four. The lift is not new methodology; it is the existing methodology applied to a substrate layer the engagement had previously treated as opaque.

## II. The boundary, and what lies on each side

The rusty-bun-host engagement, up through 2026-05-13 evening, treated the embedded JS engine as a black-box substrate node. The apparatus measured its own substrate — Rust pilots + JS-side polyfills + module loader + bridge transforms — and reasoned about the engine only through the surfaces it exposed to consumer code (`Object.getOwnPropertyNames`, `Module::declare`, the reduced ES2020 parser, the frozen ESM namespace object).

The engine boundary, named precisely:

- **Above the boundary** sits the host's contribution: the Rust-side wirings, the JS-side polyfills, the CJS↔ESM bridge transforms, the `globalThis` namespace, the FsLoader, the NodeResolver. The K1/K2/K3 stub stratum operates entirely above this line. Doc 716 catalogues cuts above the line.

- **At the boundary** sits the engine's surface to host code: the embedding API (rquickjs's `Module::declare`, `Function::new`, `Object::set`), the parser interface, the GC's allocation hooks, the runtime's stack-size ceiling, the FFI bridge.

- **Below the boundary** sits the engine's internals: the parser's grammar, the AST, the bytecode compiler, the link-record machinery, the module-namespace-object construction, the execution-context stack, the GC implementation, the JIT (none, in QuickJS's case), the intrinsic-object inventory.

The 2026-05-13 night parity baseline drew the line definitively. Every fix landing in the bridge layer (above the boundary) returns diminishing fanout; the 88.2% ceiling reports that above-boundary substrate has saturated. The remaining 11.8% gap is concentrated below the boundary, in engine-internal behaviors. The K1/K2/K3 alphabet cannot reach there.

## III. Why the existing projections generalize

The structural prerequisites that made the projections operationally useful at host-substrate scale all hold at engine-substrate scale.

**The DAG generalizes.** The consumer-substrate dependency graph G from [Doc 715 §II](/resolve/doc/715-the-consumer-substrate-dependency-graph-as-the-load-bearing-object-beneath-the-joint-mi-lattice) was defined over substrate nodes accessible from consumer code via the embedded engine. The engine itself participates in G as a single coarse-grained node — every consumer behavior reaches the engine before reaching the host's wirings. Decomposing that single coarse-grained node into its internal abstract operations produces a refinement G' of G where the engine boundary disappears and the underlying substrate-edges become visible. The 2026-05-13 night +16-from-one-line result is the empirical signature that this decomposition is meaningful: the consumer-behavior in-degree of `OrdinaryOwnPropertyKeys` (the abstract op behind `Object.getOwnPropertyNames`) is heavy-tailed within ESM-binding-related consumers, exactly mirroring P1 at engine scale.

**The lattice generalizes.** The L0–L6 hierarchy from [Doc 714 sub-§4.b](/resolve/doc/714-the-rusty-bun-engagement-read-through-the-lattice-extension-basin-expansion-at-the-l2m-saturation-point) layers host-substrate by abstraction. Engine internals have their own abstraction stratum, written down explicitly in ECMA-262: spec-prose → algorithmic-step → internal-method → intrinsic-object → execution-context-record → realm. Each engine implementation makes cuts at specific rungs of *this* stratum. rquickjs cuts ESM-namespace at "frozen-on-construction" (an intrinsic-object-rung cut). Bun's behavior implies cuts higher up, at the realm rung, where module-binding synthesis is permitted to consult host-defined behavior (the §16+ hooks of ECMA-262). The lattice projection makes these cut-locations comparable across engines.

**The alphabet generalizes.** [Doc 716 §V](/resolve/doc/716-stubs-as-named-cuts-the-three-projection-tracker-and-the-stub-alphabet-stability-conjecture#v-the-alphabet-projection) conjectured that the host-substrate cut alphabet stabilizes at cardinality three (K1 throw-on-use / K2 no-op return / K3 hardcoded-sentinel). The engine-substrate cut alphabet has a different empirical signature: the engine isn't *stubbing* a surface so much as *realizing* the spec at a particular level of fidelity. The alphabet at engine scale conjecturally stabilizes at cardinality four, developed in §V below.

The lift is therefore not a methodological novelty. It is the same three-projection tracker applied one stratum further down — through the engine boundary that the engagement, up to today, had treated as a single opaque substrate node.

## IV. The DAG projection at engine scale

The engine-scale DAG, G_engine, has as nodes the ECMA-262 abstract operations + WHATWG-defined host operations. Examples: GetValue, PutValue, OrdinaryGetOwnProperty, OrdinaryOwnPropertyKeys, ResolveBinding, GetModuleNamespace, FinishDynamicImport, ParseModule, InitializeBinding, EvaluateImportCall, OrdinaryHasInstance, the various IteratorRecord operations.

Edges in G_engine: consumer-distinguishable behavior B → abstract op AO if B's specified semantics require AO to be invoked. In-degree of AO = number of consumer-distinguishable behaviors that reach it.

**Prediction (mirror of [Doc 715 P1](/resolve/doc/715-the-consumer-substrate-dependency-graph-as-the-load-bearing-object-beneath-the-joint-mi-lattice#vi-prediction-p1-heavy-tailed-in-degree-distribution)).** G_engine's in-degree distribution is heavy-tailed. A small fraction of abstract operations (perhaps the order of 20–30) carry the great majority of consumer behavior, while the long tail of specialty ops (TypedArray-specific iterator records, Atomics atomic-load, etc.) each serve narrow consumer slices.

**Empirical anchor.** The 2026-05-13 +16-from-one-line getOwnPropertyNames result is direct evidence at one node: `OrdinaryOwnPropertyKeys` invoked through GetModuleNamespace's wrapped exotic property keys algorithm carries enough consumer behavior that altering rquickjs's wrapper (above the boundary) — which we did by switching the bridge from `Object.keys` to `Object.getOwnPropertyNames` — flipped 16 of 119 packages. Two further results corroborate: `b49cd8fa` (the __esModule / reserved-key bridge refinements) returned +4 packages by adjusting how the bridge's emitted ESM source surfaces names that ultimately route through the same OrdinaryOwnPropertyKeys node; `b77d7d9f` (the ResolveMessage error-shape) returned +2 by adjusting the error-throwing abstract op in NodeResolver's failure path. Three substrate moves at three nodes in G_engine accounting for 22 of the 24 packages added that slice.

**Operational output (mirror of [Doc 716 §VI output 1](/resolve/doc/716-stubs-as-named-cuts-the-three-projection-tracker-and-the-stub-alphabet-stability-conjecture#vi-the-three-operational-outputs)).** Engine-cut priority list: rank engine-internal cuts (one per (abstract-op × rung) pair) by transitive in-degree of the cut's abstract-op node × cut depth in the engine lattice. The 14 residual parity failures (post 2026-05-13 night) map onto an expected 2–3 distinct (abstract-op × rung) pairs, not 14 separate engine bugs. The prediction is testable directly against the parity-tool's per-package failure trace.

## V. The lattice projection at engine scale

The engine-internal abstraction stratum (the engine-lattice, E0–E5):

- **E0 — Specification text.** ECMA-262 + WHATWG specs as written. Pure prose-and-algorithmic-steps. No implementation choices yet made.
- **E1 — Algorithmic step.** A single numbered step within an abstract operation (e.g., "1. Let O be ? ToObject(V)"). The smallest unit of specified behavior.
- **E2 — Internal method.** An object's [[Get]], [[Set]], [[OwnPropertyKeys]], [[Construct]] etc., as defined per object kind (ordinary object, exotic Module Namespace object, exotic Array object).
- **E3 — Intrinsic object.** %Object%, %Array%, %Promise%, %ModuleNamespace%, etc. The named built-in instances installed in each realm at startup.
- **E4 — Execution-context record.** Module Environment Record, Function Environment Record, Object Environment Record, plus the running execution context stack.
- **E5 — Realm.** The whole realm record — the unit at which host-defined behavior (per ECMA-262 §16+) attaches. Bun's import-binding synthesis lives here.

**Each engine implementation cuts at specific rungs.** A cut at rung Ek means: the engine implements every rung above Ek as the spec specifies, and at Ek it makes an implementation choice that may or may not match the spec exactly. Implementations can cut at multiple rungs independently.

rquickjs's exotic Module Namespace object construction cuts at E2: the [[OwnPropertyKeys]] internal method is implemented at construction time, frozen, and not augmented post-init. That's an E2 cut against ECMA-262's prose, which leaves Module Namespace exotic-object behavior partially host-defined (§16.2.1.5 specifies the abstract algorithm; specific binding-resolution behavior is permitted to vary per host).

Bun's cuts are higher up: at E5, Bun's host-defined behavior installs synthesis on top of the spec's host hooks — when ParseModule encounters an ESM source without `export default`, Bun's host realm augments the resulting Module Namespace with a synthetic `default` binding. The cut sits at E5 (host-defined) rather than at E2 (intrinsic-object-construction-time). The rung difference is exactly what makes Bun's behavior unreproducible via above-engine substrate.

**Prediction (mirror of [Doc 716 §IV's lattice-and-DAG interaction](/resolve/doc/716-stubs-as-named-cuts-the-three-projection-tracker-and-the-stub-alphabet-stability-conjecture#iv-the-lattice-projection-which-rung-the-cut-sits-at)).** Migration cost from engine A to engine B equals the sum, over all (abstract-op × rung) pairs in A's cut record, of the cost to lift each pair to the rung at which B cuts. Concretely: the cost to migrate from rquickjs to a hand-rolled engine equals the cost to lift each rquickjs cut from its current rung to the rung where the hand-roll cuts. If the hand-roll cuts at E5 (matching Bun), every cut in rquickjs that currently sits at E2–E4 must be re-implemented at E5.

**Operational output (mirror of [Doc 716 §VI output 2](/resolve/doc/716-stubs-as-named-cuts-the-three-projection-tracker-and-the-stub-alphabet-stability-conjecture#vi-the-three-operational-outputs)).** Per-rung cut audit: catalogue every observed engine-cut by its rung. The engine whose cuts cluster highest up the lattice (closest to E5) requires the least migration work to reach Bun-parity. The audit is the deliverable input to Tier-Ω.3 (engine selection).

## VI. The alphabet projection at engine scale — the engine-cut stability conjecture

[Doc 716 §V](/resolve/doc/716-stubs-as-named-cuts-the-three-projection-tracker-and-the-stub-alphabet-stability-conjecture#v-the-alphabet-projection) conjectured that the host-substrate cut alphabet stabilizes at cardinality three. The engine-substrate cut alphabet has a different empirical character: the engine is realizing the spec at a particular *fidelity level*, not stubbing out an unimplemented surface. The four-class engine-cut alphabet:

**(E1-class) Spec-conformant.** The engine implements the spec step-for-step at the rung in question. Consumer-distinguishable behavior matches every other spec-conformant engine. Most of any production engine's behavior is E1-class; cuts are the *exceptions* to E1.

**(E2-class) Spec-relaxation.** The engine intentionally implements a simplified version of the spec at this rung, accepting consumer-visible divergence in exchange for implementation simplicity, performance, or embedded-scope appropriateness. rquickjs's frozen Module Namespace is E2-class: the spec permits namespace augmentation under host-defined hooks (§16.2.1.5), but rquickjs's implementation does not provide those hooks, foreclosing the augmentation pathway. Consumer-distinguishable behaviors that depend on namespace-augmentation diverge from spec-conformant engines.

**(E3-class) Spec-extension.** The engine adds behavior beyond the spec at this rung, typically through the spec's permitted host-defined-behavior hooks. Bun's import-binding synthesis is E3-class: ECMA-262 §16.1 permits a host to define behaviors that augment Module Namespace construction, and Bun exercises that permission. Consumer code targeting Bun explicitly may depend on E3-class behavior; consumer code targeting only spec may not.

**(E4-class) Version-lag.** The engine implements an older edition of the spec at this rung; later spec additions are not yet available. QuickJS sits at approximately ES2020 with selective ES2021–2023 patches; certain ES2022+ surfaces (string-literal export aliases at the parser-grammar rung; some class-field forms) are E4-class. The cut is closed by version-update rather than re-implementation.

**Stability conjecture.** Across the engines studied (rquickjs, QuickJS-NG, Boa, V8, JavaScriptCore, SpiderMonkey, Hermes), every observed engine-cut classifies into exactly one of {E1, E2, E3, E4}. No fifth class is required. Note that E1 (spec-conformant) is degenerate as a "cut" — it is the absence of cut — and is included for alphabet completeness; cuts of operational interest are concentrated in E2/E3/E4.

**The stability conjecture is empirically testable.** Pick a fixed corpus of consumer-distinguishable behaviors. Run each engine. For each (engine, behavior) pair where the engine's output diverges from the spec-conformant reference, classify the divergence into the four classes. If any classification requires inventing a fifth class to fit, the conjecture is falsified.

The conjecture is the engine-stratum analogue of [Doc 716 §V's K-stability claim](/resolve/doc/716-stubs-as-named-cuts-the-three-projection-tracker-and-the-stub-alphabet-stability-conjecture#v-the-alphabet-projection). Where Doc 716's conjecture asserts that host-side cuts have three structurally distinguishable kinds, Doc 717's asserts that engine-side cuts have four. Both conjectures rest on the same structural argument: implementation choices factor through a small fixed set of relations to the specification.

**Operational output (mirror of [Doc 716 §VI output 3](/resolve/doc/716-stubs-as-named-cuts-the-three-projection-tracker-and-the-stub-alphabet-stability-conjecture#vi-the-three-operational-outputs)).** Alphabet stability check: when a new engine-cut is recorded, verify its classification falls within {E1, E2, E3, E4}. Each successful classification corroborates the conjecture; the first un-classifiable cut falsifies it and forces alphabet extension.

## VII. The pure abstraction point

The three projections at engine scale collapse, in the limit, into a single formal object: **the ECMA-262 + WHATWG specifications taken together as the formal-semantics limit object**. Every concrete engine is a realization of this limit, distinguished from every other engine by its specific cuts.

The collapse is precise:

- **The DAG-on-engine collapses to the spec's algorithm dependency graph.** Abstract operations are defined in terms of other abstract operations; the dependency relation is read directly off the spec text. G_engine, in the spec-conformant limit, *is* the spec's algorithm graph.

- **The lattice-on-engine collapses to the spec's own abstraction hierarchy.** The E0–E5 stratum is not arbitrary — it is the way ECMA-262 itself is organized: spec-prose at E0; algorithmic steps at E1; internal methods at E2; intrinsic objects at E3; execution-context records at E4; realms (with host-defined hooks) at E5. The lattice projection, lifted to engine scale, *names* the spec's own structure.

- **The alphabet-on-engine collapses to the spec-conformance classification.** E1 (conformant) corresponds to "matches the spec." E2 (relaxation) corresponds to "implements less than the spec at this rung." E3 (extension) corresponds to "implements more than the spec at this rung, through host-defined hooks." E4 (version-lag) corresponds to "implements an older edition of the spec at this rung." The four classes are the four ways an engine can stand in relation to the spec text.

This is the **pure abstraction point** the keeper named. At this point, the apparatus stops being engine-specific and becomes a *reading of the spec itself*. Doc 715's DAG, Doc 716's lattice and alphabet — applied above the engine boundary — read as host-implementation choices against a black-box engine. Applied below the engine boundary, the same projections read as engine-implementation choices against a white-box spec. The convergence at the pure abstraction point unifies the readings: every implementation choice the apparatus tracks, at any layer of the stack, is a cut against the spec-as-limit-object, classifiable by its DAG node, its lattice rung, and its alphabet class.

The convergence is what makes the conjecture load-bearing for Tier-Ω. The engine-selection decision (Ω.3) becomes a measurement against this limit: profile each engine candidate's cuts as a function from (abstract-op × rung) pairs to alphabet-classes. The engine whose cut-profile sits closest to Bun's — measured in the cardinality of pairs requiring lift — is the lowest-migration-cost target.

## VIII. Operational consequences for the engagement

The apparatus is now defined two layers deep — Doc 715/716 above the engine boundary, Doc 717 below it — with a shared structural reading.

**Tier-Ω.3 (engine selection) acquires a concrete deliverable.** The decision artifact is the per-engine cut-profile: for each candidate (QuickJS, QuickJS-NG, Boa, hand-roll from scratch), record (a) the (abstract-op × rung) cuts the engine exhibits, (b) the alphabet class of each cut, (c) the cardinality of cuts requiring lift to reach Bun-parity. The engine minimizing (c) is the selected target. The profiling work is the tracker built per §IX below.

**Tier-Ω.4 (substrate migration) becomes legible at the catalogue level.** Once the selected engine's cut-profile is known, the migration work is the explicit list of cuts requiring lift. Each lift is a focused round; the K1/K2/K3 stub catalogue (Doc 716 §VI) inverts cleanly to the migration checklist — every K1 stub above the boundary that depended on engine-substrate-not-present-in-current-engine-but-present-in-target-engine retires automatically; every host-side polyfill that compensated for an engine cut becomes deletable once the target engine closes that cut natively.

**The K5 placeholder in the night-close trajectory write-up dissolves.** The trajectory.md update from 2026-05-13 night noted "K5 engine-level closures" as a new category. Doc 717's alphabet (E1/E2/E3/E4) supersedes that placeholder. K5 is not a fifth host-stub class; it is the four-class engine-cut alphabet, accessed through a different projection of the same DAG.

**Parity-percentage acquires a structural ceiling.** The 88.2% headline metric is a measurement against rquickjs's current cut-profile. Closing the residual 11.8% is a measurement against the lift required to reach Bun's cut-profile. The two profiles, side by side, predict the parity ceiling achievable on any candidate engine without modification: the residual gap is computable pre-commit from the cut-profile diff. Parity becomes prospectively legible.

## IX. The Doc 717 tracker (operational output)

Mirroring Doc 716 §VI's three-output structure, applied at engine scale. The output is one new tool plus three derived reports.

**Tool: `host/tools/engine-cut-profile.sh`** (to be built under Tier-Ω.3). Walks a fixed corpus of consumer-distinguishable behaviors (the parity-119 + the failed-fixture history from the engagement's recorded basin boundaries E.7–E.63). For each behavior, runs it under each candidate engine + the spec-conformant reference (the spec itself, via reading the algorithm; or a known-conformant engine like V8 for behaviors where V8 matches spec). For each (behavior, engine) where output diverges from reference, records the (abstract-op × rung × alphabet-class) tuple.

**Output 1: per-engine cut profile.** A table per engine showing every observed cut classified by (abstract-op, rung, alphabet-class). The migration cost from rquickjs to candidate X equals the symmetric difference of the two profiles, weighted by abstract-op in-degree.

**Output 2: per-rung audit across engines.** A table showing, for each lattice rung E0–E5, which engines cut at that rung and which behaviors are affected. Identifies rungs where all candidates cut similarly (these are non-discriminating between engine choices) vs. rungs where candidates diverge sharply (these are the decision-critical rungs).

**Output 3: alphabet stability check.** Verifies that every observed engine-cut classifies into E1/E2/E3/E4. The first un-classifiable cut falsifies the conjecture and forces alphabet extension; until then, each successful classification corroborates.

The tracker is the engagement's Ω.3 deliverable, alongside this primary articulation.

## X. The conjecture's predictions

The conjecture makes three operationally testable predictions:

**P1 (in-degree mirror).** G_engine's in-degree distribution is heavy-tailed. ✓ Anchored 2026-05-13: a single abstract-op node (OrdinaryOwnPropertyKeys through Module Namespace) carries +16 packages of consumer behavior. Further measurements pending.

**P2 (migration-cost legibility).** The migration cost from engine A to engine B is computable pre-commitment from the symmetric difference of their cut profiles. Falsifier: a migration whose actual cost diverges substantially from the predicted cost. To be tested when Tier-Ω.3 produces its first engine selection.

**P3 (residual concentration).** The 14 residual parity failures map onto 2–3 distinct (abstract-op × rung) pairs in G_engine, not 14 separate engine bugs. Falsifier: a per-failure classification showing 14 independent (abstract-op × rung) pairs with no shared structure. The classification is computable now against the parity-tool's recorded failure traces.

P3 is the most immediately actionable. Resolving P3 ahead of Tier-Ω.3 sharpens the engine-selection criterion: if the residual is concentrated in 2–3 pairs, the engine-selection question reduces to "which engine cuts highest up the lattice at those specific pairs." If P3 falsifies and the residual is 14 independent failures, the selection question is broader and the migration cost higher.

## XI. The hypostatic boundary

This document does not assert that Bun, V8, JavaScriptCore, or any other engine "is" the spec, nor that the spec "is" what any engine implements. The pure abstraction point is a *reading* — a place from which the apparatus can measure implementations without privileging any one implementation as ground. Per [Doc 372](/resolve/doc/372-the-method-of-the-corpus-as-derivation-not-collection) and the discipline of the resume vector, the apparatus produces a useful reading; it does not collapse onto an ontological claim about what JavaScript "really is."

The spec, the engines, and the host-substrate are all observable through the same three projections. That the projections collapse at the spec is the conjecture; the convergence is what makes the apparatus generalize one stratum further down than where it began.

---

*Companion documents: [Doc 715 — the consumer-substrate DAG](/resolve/doc/715-the-consumer-substrate-dependency-graph-as-the-load-bearing-object-beneath-the-joint-mi-lattice); [Doc 716 — stubs as named cuts](/resolve/doc/716-stubs-as-named-cuts-the-three-projection-tracker-and-the-stub-alphabet-stability-conjecture). Folds into the engagement at trajectory.md §II Tier-Ω as the Ω.3 deliverable.*
