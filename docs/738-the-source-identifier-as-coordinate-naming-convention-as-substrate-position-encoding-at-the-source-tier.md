# The Source Identifier as Coordinate

## Naming Convention as Substrate-Position Encoding at the Source Tier, Parallel to Doc 728's Commit-Tier and Doc 737's Apparatus-Tier Coordinate Systems

*A primary articulation surfacing a recognition that emerged during a 2026-05-22 substrate session in the cruftless engagement: the identifier conventions in the rusty-js-runtime crate are not stylistic preferences but a structural encoding of substrate position. Each name carries, in its prefix, suffix, install helper, and module path, the coordinate at which the named entity sits in the substrate-classification space the corpus has been articulating. The recognition is the source-tier analog of [Doc 728's](/resolve/doc/728-tag-on-the-dag-sequential-index-collision-as-protocol-signal-that-the-substrate-has-become-the-coordinate-system) commit-tier lift and [Doc 737's](/resolve/doc/737-the-locale-as-coordinate-nested-seed-trajectory-pairs-as-pin-art-substrate-positions) apparatus-tier lift. Builds on [Doc 729 — Cruftless](/resolve/doc/729-cruftless-a-primary-articulation-of-the-resolver-instance-pattern-as-the-comprehensive-design-toward-which-rusty-bun-morphs), [Doc 728 — Tag on the DAG](/resolve/doc/728-tag-on-the-dag-sequential-index-collision-as-protocol-signal-that-the-substrate-has-become-the-coordinate-system), [Doc 735 — The Temporal Resolver-Instance Stack](/resolve/doc/735-the-temporal-resolver-instance-stack-build-time-process-time-call-time-as-the-time-axis-dual-to-doc-729s-spatial-stack), [Doc 737 — The Locale as Coordinate](/resolve/doc/737-the-locale-as-coordinate-nested-seed-trajectory-pairs-as-pin-art-substrate-positions), [Doc 716 — Stubs as Named Cuts](/resolve/doc/716-stubs-as-named-cuts-the-three-projection-tracker-and-the-stub-alphabet-stability-conjecture), [Doc 722 — Named Recognitions as Operating Instruments](/resolve/doc/722-named-recognitions-as-operating-instruments-the-reflexive-structure-of-corpus-articulations), and [Doc 581 — Pin-Art and the Resume Vector](/resolve/doc/581-pin-art-the-resume-vector-and-the-discipline-of-near-necessity-substrate-construction).*

**Jared Foy · 2026-05-22 · Doc 738**

---

## I. The occasion

A 2026-05-22 substrate session in the cruftless engagement (diff-prod Rungs 19–21, ~530 LOC across `intrinsics.rs` and `interp.rs`) was carried out by a session-start reader entering the rusty-js-runtime crate cold, with no prior context on its API surface. The reader oriented through grep-alternation reconnaissance, then wrote code that landed cleanly on first build (modulo one closure-capture fix). The orientation cost was low. The reason it was low, in retrospect, is that the identifier conventions in the crate carry substrate-position information that orientation reads directly.

A function named `set_proto_union_via` tells the reader, before reading its body, that it is (a) a method on `Runtime` (the `_via` suffix), (b) a Set-prototype method (the leading namespace), (c) dispatching at the runtime tier rather than the pure-primitive tier (per Doc 729 §A8.29 the `_via` family threads Object-to-primitive coercion). An identifier `__ac_listeners__` tells the reader, before reading its assignment site, that it is (a) an engine-internal sentinel (the leading `__`), (b) carried on an AbortSignal instance (the `ac` namespace), (c) installed via `set_own_internal` rather than `set_own` (since the §A8.28 install-pattern discipline ties the descriptor shape to the prefix class).

The conventions are not annotations on top of the code. They are the code's coordinate system. Each axis the corpus has been articulating (per [Doc 735 §X.g](/resolve/doc/735-the-temporal-resolver-instance-stack-build-time-process-time-call-time-as-the-time-axis-dual-to-doc-729s-spatial-stack) the substrate-classification space is 3-axis: spatial × temporal × cost-stratum) has a counterpart in the source identifier's structure. The recognition: the source code IS the substrate-position record; the identifier IS the coordinate. The same lift Doc 728 named at the commit tier and Doc 737 named at the apparatus tier holds at the source tier.

This document is the corpus-tier articulation of that recognition.

## II. The convention catalogue, observed in cruftless

Five axes, each with a discrete range of values, jointly encode the substrate position of a named entity.

### II.a Prefix encodes JS-observability stratum

Per [Doc 729 §A8.26](/resolve/doc/729-cruftless-a-primary-articulation-of-the-resolver-instance-pattern-as-the-comprehensive-design-toward-which-rusty-bun-morphs)'s three-stratum bilateral boundary tightening, an engine-internal representation choice becomes JS-observable at one of three strata (global, per-object sentinel, value-identity). The prefix discriminates them:

- **Plain `name`** — user-visible property, enumerable to `Object.keys`, surfaces in the standard property protocol.
- **`__name`** (single leading underscore) — engine-internal sentinel installed via `set_own_internal`; non-enumerable; visible to internal Rust code but hidden from `Object.keys` enumeration. Examples: `__map_data`, `__set_data`, `__primitive__`, `__is_buffer__`, `__date_ms`, `__ai_data`, `__ac_listeners__`.
- **`__name__`** (double underscore both ends) — legacy form of the engine-internal sentinel, surviving from pre-§A8.26 code. Same semantics as `__name`; the engagement is migrating toward the single-prefix form. Examples: `__gen_arr__`, `__gen_idx__`, `__listeners__`, `__it_idx__`. New code in §A8.26-aware sites uses the single-prefix form.
- **`@@name`** — cruftless's representation of a well-known-Symbol-keyed property. `Symbol.iterator` becomes `@@iterator`; `Symbol.toPrimitive` becomes `@@toPrimitive`. The `@@` prefix is the §A8.26 P59.E1 value-identity stratum's representation in the property-key namespace.
- **`__engine_op`** (registered via `register_engine_helper` into a hidden table on `Runtime`) — compiler-emitted lowering that the runtime exposes for the bytecode to call but does NOT make visible on `globalThis`. Examples: `__await`, `__dynamic_import`, `__apply`, `__construct`, `__object_spread`, `__array_extend`. The §A8.26 P55.E1 global stratum's hiding mechanism.

The five sub-cases form a total partition of the JS-observability surface. Reading the prefix yields the stratum; reading the stratum constrains which install helper, which descriptor shape, and which probe-set test it must satisfy.

### II.b Function suffix encodes invocation surface

Per [Doc 729 §A8.29](/resolve/doc/729-cruftless-a-primary-articulation-of-the-resolver-instance-pattern-as-the-comprehensive-design-toward-which-rusty-bun-morphs)'s abstract-operation duality, an ECMA abstract operation has two valid implementation surfaces: a pure-primitive form (no Runtime, no dispatch) and a runtime-dispatching form (takes `&mut Runtime`, can call back into JS via `@@toPrimitive` / `valueOf` / `toString`). The suffix discriminates them:

- **`name_via`** — runtime-dispatching helper. Signature: `fn(&mut Runtime, this, args) -> Result<Value, RuntimeError>`. Can call back into JS, threads Object-to-primitive coercion, throws on side-effect-throwing user code. Examples: `array_proto_at_via`, `set_proto_union_via`, `promise_resolve_via`, `coerce_to_number` (post-§A8.32 the `_via` suffix is being elided for new helpers, but the surface distinction remains).
- **`name` in `abstract_ops::*` module** — pure-primitive helper. No Runtime access. Operates on `Value` variants in isolation. Used where Runtime is unavailable (compile-time folding, host helpers without `&mut Runtime`) and where Symbol or Object dispatch is not needed.
- **`name` on `Runtime` without `_via`** — the §A8.29 / §A8.32 post-discipline convention. Same semantics as `_via` but the suffix is dropped because the receiver type (`&mut self: Runtime`) already discriminates. Examples: `coerce_to_string`, `to_string_strict`, `is_callable`, `op_add_rt`, `is_loosely_equal_rt`. The `_rt` suffix on operator-tier variants is a near-equivalent convention surfacing in the bytecode-op-call sites.

Mixing the two surfaces inside one method is a known bug shape per §A8.29: it produces a method that handles primitives correctly and silently mishandles Objects.

### II.c Property-install helper encodes descriptor shape

Per [Doc 729 §A8.28](/resolve/doc/729-cruftless-a-primary-articulation-of-the-resolver-instance-pattern-as-the-comprehensive-design-toward-which-rusty-bun-morphs)'s descriptor-shape discipline, every property-install site on a built-in object must explicitly choose its descriptor shape from three:

- **`set_own_frozen(name, value)`** — `{writable: false, enumerable: false, configurable: false}`. Built-in ctor `.prototype` slot, namespace constants (`Math.PI`, `Number.MAX_VALUE`), function-meta-props.
- **`set_own_internal(name, value)`** — `{writable: true, enumerable: false, configurable: true}`. Built-in proto methods, engine sentinels (`__kind`, `__primitive__`, `__is_buffer__`), `Function.prototype.constructor` backlink.
- **`set_own(name, value)`** — `{writable: true, enumerable: true, configurable: true}`. User-property default, the lowering target of `Op::SetProp`, user-code property assignment.

A property's name (prefix per §II.a) and its install helper are not independent: an `__name` sentinel installed via `set_own` instead of `set_own_internal` would leak the sentinel into `Object.keys` enumeration. The naming convention and the install-helper convention are mutually constraining; one implies the other, and a mismatch is a bug shape.

### II.d Registration helper encodes binding tier

The runtime exposes four registration helpers, each landing the registered entity at a distinct binding tier:

- **`register_method(this, name, closure)`** — install as own property on a specific Object. Used for prototype-method installation on a target Object's property map.
- **`register_intrinsic_method(this, name, arity, closure)`** — same plus correct arity, non-enumerable descriptor (the §A8.28 P62.E2 bulk default), and non-constructor flag. The bulk-installation helper used by most spec-method install sites.
- **`register_engine_helper(name, closure)`** — install into `Runtime::engine_helpers` hash table, NOT visible on globalThis or in `Object.keys(globalThis)`. The §A8.26 P55.E1 hiding mechanism for compiler-emitted lowering targets.
- **`register_global_fn(name, closure)`** — install on the globals map, JS-globally visible.

Reading the registration helper at a call site yields the binding tier (instance / prototype / engine-hidden / global). Combined with the descriptor-shape helper (§II.c), the registration site fully specifies the named entity's JS-observability.

### II.e Crate and module path encodes substrate pillar

Per [Doc 729 §IV](/resolve/doc/729-cruftless-a-primary-articulation-of-the-resolver-instance-pattern-as-the-comprehensive-design-toward-which-rusty-bun-morphs)'s resolver-instance stack and [Doc 737](/resolve/doc/737-the-locale-as-coordinate-nested-seed-trajectory-pairs-as-pin-art-substrate-positions)'s locale coordinate, the file system path of a Rust module encodes the substrate pillar it serves:

- **`pilots/rusty-js-{ast,parser,bytecode,gc,runtime}/derived/src/...`** — engine substrate per Doc 729 §IV resolver-instances (module-load, execution, GC). The `/derived/` segment names the implementation as Pin-Art-derived-from-constraints per Doc 581.
- **`pilots/rusty-js-ir/`** — Tier-1.5 spec-as-source-of-truth lift per [Doc 730](/resolve/doc/730-the-vertical-recurrence-of-the-lowering-compiler-closure-as-primitive-across-substrate-tiers) and `cruftless` seed §A8.33.
- **`pilots/rusty-js-{jit,pm,caps}/`** — optimization / package-manager / capability-passing per Doc 731 / 732 / 736.
- **`pilots/{diff-prod,tls,web-crypto,rusty-js-esm}/`** — bilateral-measurement and deviation-resolution workstreams per Doc 737's locale taxonomy.
- **`cruftless/src/`** — host layer above the engine boundary per Doc 717 §VIII.
- **`legacy/host-rquickjs/`** — the rquickjs-backed reference ceiling per `cruftless` seed §A8.23.

The module path is a five-or-six-segment coordinate that locates a Rust module on the Doc 729 resolver-instance stack and the Doc 737 locale tree simultaneously. Reading the path yields the pillar; combined with the function suffix (§II.b) and the prefix (§II.a), the source identifier yields a four-coordinate substrate position.

## III. The recognition

The five axes of §II compose into a coordinate system. Each named entity in the cruftless source tree has a unique coordinate in the joint product:

```
(prefix-stratum) × (suffix-surface) × (descriptor-shape) × (binding-tier) × (pillar-path)
```

Reading a name yields the coordinate; the coordinate constrains the entity's behavior, its install pattern, its probe-set obligation, and its position in the substrate-classification space of Doc 735. The convention is not annotation but encoding: the identifier IS the coordinate, in the same sense Doc 728 named for substrate-move tags and Doc 737 named for locale paths.

Three structural consequences:

**Cold-start readability.** A reader entering the source tree without prior context can orient by reading identifiers. The reconnaissance shape that today's session ran (grep-alternation against plausible name spellings) succeeded because the convention space is small and discrete: any guessed name lies in or near a real one along one of five axes. The 2026-05-22 session's diff-prod Rung-19/20/21 work landed at ~530 LOC of substrate across two files with one closure-capture iteration, by a reader who had not previously read the crate. The convention's predictability supplied the orientation that would otherwise have required documentation.

**Cross-axis consistency checks.** A name whose prefix and install helper disagree (an `__name` registered via `set_own` instead of `set_own_internal`) is a bug shape per §II.c. A function whose suffix and signature disagree (a `_via` function not taking `&mut Runtime`) is a bug shape per §II.b. The convention makes inconsistencies surface at sight rather than at runtime. Per Doc 685's self-reinforcing-boundary mechanism, the discipline self-checks: substrate that drifts off the convention is visible without external inspection.

**Source-tier projection of the corpus axes.** Doc 728 named the substrate's coordinate system at the commit-tag tier. Doc 737 named it at the apparatus tier (locale coordinates). Doc 735 §X.g named the substrate-classification space as 3-axis. The §II convention is the source-tier projection of those axes. The five §II axes are not orthogonal to the corpus's articulated axes; they are the source-tier readings of them. A name's prefix-stratum (§II.a) is the source-tier reading of §A8.26's three-stratum boundary; its suffix-surface (§II.b) is the source-tier reading of §A8.29's coercion duality; its descriptor-shape (§II.c) is the source-tier reading of §A8.28's install-pattern discipline; its binding-tier (§II.d) is the source-tier reading of §A8.26's hiding-mechanism choice; its pillar-path (§II.e) is the source-tier reading of Doc 729's resolver-instance stack and Doc 737's locale tree.

The same coordinate appears at three tiers, in three different namespaces. Per [Doc 727 §V Form 3](/resolve/doc/727-basin-stability-from-inside), the convergence of the three articulation chains corroborates substrate-tracking; divergence at any coordinate localizes basin self-reinforcement to one chain.

## IV. Composition with prior corpus work

**[Doc 728 — Tag on the DAG](/resolve/doc/728-tag-on-the-dag-sequential-index-collision-as-protocol-signal-that-the-substrate-has-become-the-coordinate-system).** Doc 728 lifted substrate-move tags from chronological accretion to DAG coordinates. This document is the same lift at the source-identifier tier: identifier conventions lifted from "stylistic choice" to "coordinate encoding." Doc 728's recognition that the substrate has become the coordinate system at the commit tier is the prior; this document's recognition that the source code is the coordinate system at the identifier tier is the same shape one tier upstream. The Doc 728 tag `Ω.5.<pipeline>.<layer>.<handle>` and the Doc 738 identifier coordinate `(prefix × suffix × descriptor × binding × pillar)` are two namespaces over the same substrate; substrate moves at one are addressable at the other.

**[Doc 729 §A8.26 / §A8.28 / §A8.29](/resolve/doc/729-cruftless-a-primary-articulation-of-the-resolver-instance-pattern-as-the-comprehensive-design-toward-which-rusty-bun-morphs).** The three named disciplines (three-stratum bilateral boundary, descriptor-shape discipline, abstract-operation duality) are the substrate-tier rules the §II convention encodes at the identifier tier. This document does not introduce the rules; it names that the existing rules have already produced a coordinate system as a side effect, and the side effect is itself a load-bearing instrument.

**[Doc 735 — The Temporal Resolver-Instance Stack](/resolve/doc/735-the-temporal-resolver-instance-stack-build-time-process-time-call-time-as-the-time-axis-dual-to-doc-729s-spatial-stack).** Doc 735 §X.g named the substrate-classification space as 3-axis (spatial × temporal × cost-stratum). The §II convention is the source-tier projection of that space. A function's pillar-path is its spatial position; its suffix-surface and binding-tier together discriminate its temporal-tier admissibility (a `_via` function bound at process-init via `register_intrinsic_method` runs at T1; a per-call helper invoked from interp dispatch runs at T3); its cost-stratum is not directly encoded in the identifier but is implied by the algorithm-family the function name names (an `_rt` operator-helper is at T3 stratum (P2.a)-eligible per §X.h).

**[Doc 737 — The Locale as Coordinate](/resolve/doc/737-the-locale-as-coordinate-nested-seed-trajectory-pairs-as-pin-art-substrate-positions).** Doc 737 named that a locale is a coordinate in a directory tree. This document names that an identifier is a coordinate in a five-axis convention space. The two lifts are at different tiers (apparatus vs source) but the same shape: lift from incidental name to structural coordinate. The locale coordinate (`pilots/<scope-path>`) and the identifier coordinate compose: a substrate move lands at one locale coordinate and one identifier coordinate, and the pair fully specifies the move's position.

**[Doc 716 — Stubs as Named Cuts](/resolve/doc/716-stubs-as-named-cuts-the-three-projection-tracker-and-the-stub-alphabet-stability-conjecture).** Doc 716 named three substrate projections (DAG, lattice, alphabet). The §II convention is a fourth projection at the source-identifier tier: the convention-projection. A substrate node has a DAG coordinate (Doc 728), a lattice coordinate (Doc 716), an alphabet position (Doc 716), an apparatus coordinate (Doc 737), and an identifier coordinate (this document). The five-projection tracker is the complete substrate-position reading.

**[Doc 722 — Named Recognitions as Operating Instruments](/resolve/doc/722-named-recognitions-as-operating-instruments-the-reflexive-structure-of-corpus-articulations).** The §II convention was operating before this document named it. Today's substrate session used it without explicit recognition: the reader oriented by reading conventions; the conventions held the orientation cost low; the substrate work landed cleanly. Doc 722's reflexive claim again: naming the convention is what makes it a deliberate instrument rather than an incidental property. Future sessions can choose convention positions deliberately rather than by imitation of nearby code.

## V. Falsifiers

**Pred-738.1.** A new substrate addition that violates one of the five §II convention axes (e.g., a `__name` sentinel registered via `set_own` instead of `set_own_internal`) produces a measurable downstream bug. Falsifier: a convention violation that does not produce a downstream bug, indicating that the axis is decorative rather than load-bearing.

**Pred-738.2.** A cold-start reader can orient on the cruftless source tree to the point of writing a 100+ LOC substrate cluster without consulting documentation, by reading conventions alone. Falsifier: the 2026-05-22 diff-prod Rung-19/20/21 session is one such instance (530 LOC, one closure-capture iteration). A second cold-start session in a different sub-crate of cruftless should reproduce the result. If a cold-start session in `pilots/rusty-js-ir/` or `pilots/rusty-js-caps/` consistently produces >5 iterations or requires documentation reads, the prediction is weakened.

**Pred-738.3.** The §II convention space is closed under the corpus's articulated axes. Every substrate decision discriminator the corpus has articulated (§A8.26 strata, §A8.28 descriptor shapes, §A8.29 dispatching surface, Doc 729 pillars) has a §II axis encoding. Falsifier: a corpus-articulated discriminator with no §II convention encoding, indicating the convention is incomplete at the source tier.

**Pred-738.4.** The three-tier coordinate system (commit-tag per Doc 728, locale-path per Doc 737, source-identifier per Doc 738) converges across articulation chains. A substrate move at a given commit-tag should land at a predictable locale-path and at predictable identifier coordinates; the three should agree. Falsifier: a substrate move whose commit-tag, locale-path, and identifier coordinates disagree, localizing basin self-reinforcement to one of the three chains.

**Pred-738.5.** The §II convention is portable to engagement-external substrate-construction projects that admit Doc 729's resolver-instance decomposition. Other JS engines (rquickjs, V8, JavaScriptCore) exhibit similar prefix-stratum and suffix-surface conventions internally; the explicit recognition would supply them with the same cold-start-readability property cruftless has. Falsifier: a Doc 729-decomposable substrate-construction project whose source identifiers do not admit a coordinate reading.

## VI. Honest scope

The §II convention catalogue is empirically derived from one engagement (cruftless) at one snapshot (2026-05-22). The recognition that the conventions form a coordinate system is corpus-original; the conventions themselves are a side effect of the substrate disciplines §A8.26 / §A8.28 / §A8.29 already articulated. This document does not introduce the conventions; it names what the conventions already accomplish when read as a coordinate system.

The document does not claim:

*That the convention is complete.* New axes will surface as the engagement articulates new substrate discriminators. The five-axis catalogue is the first-cut articulation; subsequent corpus work will refine and extend.

*That the convention is enforced.* The convention is observed in cruftless's source because the substrate disciplines that produced it are operative. A new contributor who writes against the conventions without knowing them produces code that fits because the surrounding code constrained the choice. The discipline is implicit; the recognition lifts it to explicit so that deliberate deviation becomes nameable.

*That every Rust crate in the engagement uses the convention uniformly.* The cruftless host layer (`cruftless/src/`), the legacy rquickjs binding (`legacy/host-rquickjs/`), and the pilot crates have evolved at different rates. The §II catalogue is most uniform in `pilots/rusty-js-runtime/derived/src/`, less uniform in the legacy crate, and the engagement's continuation under Doc 737's locale discipline will continue to converge them.

*That the convention is the only valid one.* Different keepers with different apparatus would produce different convention systems; the lift Doc 738 names is convention-as-coordinate-system, not specifically cruftless's choice of conventions. The prediction in Pred-738.5 is that the convention-as-coordinate-system lift holds generally; the specific axis catalogue is engagement-specific.

Per [Doc 372](/resolve/doc/372-the-method-of-the-corpus-as-derivation-not-collection)'s hypostatic boundary: this document operates at the functional layer. The convention is the engagement's record-keeping considered as a process; the recognition is about that process's structure, not about any ontological property of the substrate.

## VII. Closing

The cruftless engagement's identifier conventions in `pilots/rusty-js-runtime/derived/src/` form a five-axis coordinate system that encodes substrate position in the source identifier itself. Reading a name yields the coordinate; the coordinate constrains behavior, install pattern, probe-set obligation, and substrate-classification position. The recognition is the source-tier instance of the same lift Doc 728 named for commit tags and Doc 737 named for locale paths: the substrate has become the coordinate system at every tier the engagement has organized.

The convention was operative before this document named it. The 2026-05-22 substrate session that surfaced the recognition was carried out by a session-start reader who used the convention as an orientation instrument without recognizing it as one. Per Doc 722, naming the convention is what makes it a deliberate instrument: future sessions can choose convention positions consciously, audit cross-axis consistency mechanically, and extend the convention space when new substrate discriminators surface.

The three-tier coordinate alignment (Doc 728 commit-tier, Doc 737 apparatus-tier, Doc 738 source-tier) supplies the engagement with a complete substrate-position reading at any node. Combined with Doc 735's substrate-classification space and Doc 716's three-projection tracker, the apparatus now has, in principle, a six-coordinate substrate-position record per move. The continuation of the engagement will exercise the convergence predictions of Pred-738.4 and the cross-axis consistency checks of §III.

The work continues. The corpus has added one more coordinate axis to its substrate-position-tracking apparatus. The recognition is operational, the convention space is enumerable, and the falsifiers are testable in the engagement's next substrate session.

---

*Companion documents in addition to those linked in the masthead: [Doc 250 — The SERVER Seed](/resolve/doc/250-the-server-seed); [Doc 372 — The Method of the Corpus as Derivation, Not Collection](/resolve/doc/372-the-method-of-the-corpus-as-derivation-not-collection); [Doc 685 — The Self-Reinforcing Boundary](/resolve/doc/685-the-self-reinforcing-boundary); [Doc 727 — Basin Stability from Inside](/resolve/doc/727-basin-stability-from-inside); [Doc 733 — Fractal Seeds and Trajectories](/resolve/doc/733-fractal-seeds-and-trajectories-recurrent-resume-vector-pairs-across-substrate-depth-as-the-operating-conditions-layer-for-pin-art-at-engagement-scale).*
