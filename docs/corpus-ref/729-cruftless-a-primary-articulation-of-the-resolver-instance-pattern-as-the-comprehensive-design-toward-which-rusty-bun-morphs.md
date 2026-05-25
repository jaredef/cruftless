# Cruftless

## A Primary Articulation of the Resolver-Instance Pattern as the Comprehensive Design Toward Which rusty-bun Morphs

*A corpus document responding to the keeper's observation (2026-05-17 18:22-local): "PRESTO and SERVER are just implementations of patterns that appear to be universal to systems design; how might we apply the abstractions to rusty-bun?" The keeper then named the destination: Cruftless. Builds on [Doc 432 — SERVER](/resolve/doc/432-server-an-architectural-style-for-engine-orchestration), [Doc 426 — PRESTO](/resolve/doc/426-presto-an-architectural-style-for-representation-construction), [Doc 719 — The Pipeline Pattern Across Subjects](/resolve/doc/719-the-pipeline-pattern-across-subjects-presto-and-the-javascript-engine-as-two-realizations-of-the-same-derivation), [Doc 247 — The Derivation Inversion](/resolve/doc/247-the-derivation-inversion), [Doc 424 — SIPE (Architectural Form)](/resolve/doc/424-sipe-architectural-form-recursive-fielding-style-constraint-accumulation-in-composed-software-stacks), [Doc 717 — The Apparatus Above the Engine Boundary](/resolve/doc/717-the-apparatus-above-the-engine-boundary-the-three-projections-lifted-to-engine-substrate-and-the-pure-abstraction-point), [Doc 581 — Pin-Art and the Discipline of Near-Necessity Substrate Construction](/resolve/doc/581-pin-art-the-resume-vector-and-the-discipline-of-near-necessity-substrate-construction), and [Doc 725 — The Cluster-to-Walk Mode Transition](/resolve/doc/725-the-cluster-to-walk-mode-transition-soft-saturation-as-protocol-signal-in-substrate-introduction).*

**Jared Foy · 2026-05-17 · Doc 729**

---

## I. The occasion

The rusty-bun engagement has now run for thirty-plus substrate rounds against ECMA-262 and WHATWG, with host-v2 hand-derived per [Doc 717](/resolve/doc/717-the-apparatus-above-the-engine-boundary-the-three-projections-lifted-to-engine-substrate-and-the-pure-abstraction-point)'s cut-rung framework. The 2026-05-14 keeper observation (recorded in [Doc 719](/resolve/doc/719-the-pipeline-pattern-across-subjects-presto-and-the-javascript-engine-as-two-realizations-of-the-same-derivation)) named the structural correspondence between rusty-bun's engine and PRESTO's 22-stage pipeline. The keeper's subsequent observation (2026-05-17 18:22-local) extends the recognition: PRESTO and SERVER are not the pattern. They are two named instances of a pattern that recurs at every level where a thing has to be resolved.

The keeper then named the destination toward which the engagement now morphs: **Cruftless**. The name carries the design's terminal property as its definition. A Cruftless runtime is one whose every resolver-instance leaves zero residue, whose every level satisfies the four bootstrap properties of [Doc 432 §2](/resolve/doc/432-server-an-architectural-style-for-engine-orchestration#the-bootstrap-as-resolver), whose every layer's induced properties function as constraints on the layer beneath it. The cruft is the design's negative space: every directive consumed, every namespace partitioned, every artifact a finished thing rather than a configurable framework.

This document is the primary articulation toward that design. It names what the comprehensive realization looks like, traces the resolver-instances stacked inside rusty-bun, states the property the design induces, and identifies the operational morph trajectory by which rusty-bun reaches Cruftless. It does not specify the implementation; the implementation is the engagement's subsequent work. It specifies the constraints the implementation must satisfy and the properties those constraints induce.

## II. The naming

*Cruftless* names the runtime by its terminal invariant. Cruft, in the engineering vernacular, is residue: middleware that nothing references, configuration files no caller reads, framework state that survives past its evaluation point, helpers that bleed from orchestration into application namespace. A Cruftless runtime is one constructed by a discipline that admits none of these. Every resolver-instance leaves the artifact directive-free. Every layer's bootstrap is fully consumed before the next layer runs. Every namespace boundary is enforced at the seam where the bilateral source is resolved into the unilateral artifact.

The name composes with the corpus's prior naming. PRESTO names the construction-level style; SERVER names the orchestration-level style; Cruftless names the runtime that results from applying both consistently and recursively at every resolver-instance in a JavaScript-execution stack. The relationship is concrete: a runtime that satisfies PRESTO + SERVER at every level it has, with no level left ungoverned, is Cruftless. A runtime that satisfies them at some levels but leaks at others is some-cruft-having; the engineering work morphs it toward Cruftless by closing the leaks.

The name is also a falsifiable claim. A runtime described as Cruftless must produce no detectable cruft in any layer's artifact. The verification suite is the engineering test; the §IX falsifier is the structural test.

## III. The pattern these styles instantiate

PRESTO and SERVER instantiate one pattern. The pattern, stated abstractly:

*A source representation that interleaves directives in two namespaces is resolved by a fixed-pipeline resolver that consumes the directives in one namespace fully, leaves the directives in the other namespace untouched, and emits a unilateral artifact in the substrate of the second namespace's interpreter. The resolver runs in a stage-ordered dependency-respecting graph; the artifact carries no evidence of the consumed namespace. The induced property at the level above (where the artifact is consumed) is mutual indifference between the producer and the consumer: each operates inside its own namespace deterministically, unaware of the other's mechanism.*

This pattern is not unique to representation construction (PRESTO) or engine orchestration (SERVER). It recurs at every level where a thing has to be turned from "source with what-governs interleaved" into "the artifact, with what-governs consumed." Compilation is an instance. Module loading is an instance. Build systems are instances. Process construction (kernel + ld.so → process) is an instance. Hardware boot (firmware + reset vector → instruction stream) is an instance. The pattern is the structural shape of resolution itself.

[Doc 247](/resolve/doc/247-the-derivation-inversion)'s derivation inversion accounts for why this pattern recurs across subjects: the constraints induce the structure. Wherever the resolution problem is the work, the four bootstrap properties of [Doc 432 §2](/resolve/doc/432-server-an-architectural-style-for-engine-orchestration#the-bootstrap-as-resolver) are induced, and any conformant realization adopts them. [Doc 424](/resolve/doc/424-sipe-architectural-form-recursive-fielding-style-constraint-accumulation-in-composed-software-stacks) frames the recursion: each level's induced properties become the null-style starting set for the next level's Fielding accumulation. The recursion is not metaphorical. It is the formal structure of how composed architectural levels inherit constraints.

Naming the pattern as the pattern (rather than naming each instance) is the contribution this articulation makes. PRESTO and SERVER were the first two named instances. Cruftless is the runtime that results from applying the pattern comprehensively at every instance inside one stack.

## IV. The resolver-instances stacked in rusty-bun

A JavaScript runtime in this engagement's shape has at least five resolver-instances stacked vertically. Each is a `source-with-directives → resolver → directive-free artifact` step. Each can be analyzed on its own under the four bootstrap properties and the five SERVER constraints. Each is governed by the induced properties of the resolver-instance enclosing it.

The five instances, from substrate-deepest to runtime-shallowest:

**(1) The Cargo build.** Source: `Cargo.toml` + `Cargo.lock` + the Rust source tree, with `[dependencies]` and `build.rs` as the directive set. Resolver: `rustc` + `cargo`. Artifact: the `rusty-bun-host-v2` binary. The directives (dependency specifications, feature flags, build scripts) are consumed entirely at build time; the resulting binary carries none of them. The bootstrap properties apply: totality of consumption (every dep resolved), ordering determinism (dependency graph), medium preservation (Rust substrate intact), boundary integrity (build directives invisible to the running binary).

**(2) The bootstrap (the SERVER instance proper).** Source: `host-v2/src/lib.rs::init(rt)`, the call sequence of `intrinsics::install(rt)`, `events::install(rt)`, `stream::install(rt)`, `http::install(rt)`, `https::install(rt)`, `zlib::install(rt)`, `process::install(rt)`, etc. Directives are the install calls themselves, ordered by dependency. Resolver: the Runtime allocator running through the sequence. Artifact: the populated Runtime graph (the in-memory engine with intrinsics, prototypes, host hooks, and host-module globals wired). After init returns, no install-time state survives in a form the running JavaScript can observe.

**(3) The module load.** Source: ESM source text + the `import` declarations + the `export` declarations + any package.json conditional-exports for resolution. Directives: imports, exports, side-effect imports, re-exports. Resolver: the parser, the bytecode compiler, and the module-linker (`module.rs::evaluate_module` / `evaluate_cjs_module`). Artifact: a `ModuleRecord` with a populated `Namespace` object. The directives are stripped: the resulting `Namespace` exposes only the user-declared exports; the imports are resolved to bindings; the source text is no longer referenced.

**(4) The PRESTO instance proper: execution.** Source: bytecode + constants pool + function prototypes. Directives: opcodes (the bytecode is the typed intermediate of [Doc 719](/resolve/doc/719-the-pipeline-pattern-across-subjects-presto-and-the-javascript-engine-as-two-realizations-of-the-same-derivation)'s table, the prepare-time artifact that the execute-time resolver consumes). Resolver: the dispatch loop in `interp.rs::run_frame_inner`. Artifact: resolved JavaScript values, in turn published into the Module Namespace or returned to the caller. The bytecode is the "richly-namespaced source"; the execution loop consumes it one op at a time; the resulting values carry no opcode-level residue.

**(5) The job-queue drain.** Source: the microtask queue + the macrotask queue, populated by Promise reactions, scheduled timers, I/O completions. Directives: queued jobs. Resolver: `run_to_completion` driving the job-queue stepper between primary execution phases. Artifact: a quiescent runtime, all reactions drained. This is the level [Doc 719 §II](/resolve/doc/719-the-pipeline-pattern-across-subjects-presto-and-the-javascript-engine-as-two-realizations-of-the-same-derivation) maps to PRESTO's stage 19 re-resolve pass: derived work resolved between primary work.

Each of these five instances is currently realized in rusty-bun. Each is currently realized imperfectly with respect to one or more of the four bootstrap properties or the five SERVER constraints. The Cruftless morph is the systematic closure of those imperfections at every instance, layer by layer, with the corpus's full constraint vocabulary brought to bear.

## V. Cruftlessness as the induced property

PRESTO induces ambivalent execution with agnostic determinism. SERVER induces recursive ambivalence with self-authorizing determinism. Cruftless, as a runtime constructed by applying these styles consistently at every resolver-instance, induces a property that composes the two and extends them across the full vertical stack.

The induced property of a Cruftless runtime is **vertically-recursive directive consumption with stage-deterministic emission**. The four words, precisely:

*Vertically-recursive*: the pattern holds at every level in the stack from the deepest substrate (build) to the shallowest execution surface (job-queue drain). Each level's directives are consumed by that level's resolver and do not survive into any other level's artifact.

*Directive consumption*: every directive in every namespace at every level is consumed before the level's resolver emits its artifact. No directive is deferred. No directive is left to be re-processed by a higher level. The full set of bootstrap-property obligations from [Doc 432 §2](/resolve/doc/432-server-an-architectural-style-for-engine-orchestration#the-bootstrap-as-resolver) holds at every layer.

*Stage-deterministic*: each level's resolver is itself a stage-ordered graph. The stages have explicit dependency relationships. Same source plus same target substrate yields the same artifact, regardless of which conformant resolver implementation does the work.

*Emission*: each level's artifact is a finished thing, not a configurable framework or a partially-initialized graph awaiting later setup. The artifact carries the directive-free residue forward to the next level as the next level's source.

Three further properties are inherited as immediate consequences:

**Cross-substrate portability.** The constraint specification at each level is the seed; the substrate is the substrate. The Cargo level can be re-derived in any equivalent build system (Bazel rules, Nix derivations). The bootstrap level can be re-derived in any host language. The execution level can be re-derived in any engine substrate. [Doc 247](/resolve/doc/247-the-derivation-inversion)'s across-substrate inversion holds at each level independently; [Doc 719](/resolve/doc/719-the-pipeline-pattern-across-subjects-presto-and-the-javascript-engine-as-two-realizations-of-the-same-derivation)'s across-subject inversion holds at any other domain in the same resolution-problem class.

**Architectural diagnosability.** When the runtime exhibits an observable defect, the defect is locatable to one resolver-instance. The diagnosis discipline is: identify which level's artifact carries the residue, then identify which of that level's bootstrap properties was violated or which of its SERVER constraints was breached. Cruftlessness makes the diagnosis space well-shaped.

**Compositional safety.** The induced property of each level functions as a constraint on the level it encloses (per [Doc 432 §6](/resolve/doc/432-server-an-architectural-style-for-engine-orchestration#composition-not-extension)). A SERVER constraint that produced runtime-graph cruft would destroy the property PRESTO induces. A PRESTO constraint that produced a module namespace with bytecode residue would destroy the property the execution level induces. Cruftlessness is preserved end-to-end only when each level respects the inherited constraint from above and emits a directive-free artifact to the level below.

## VI. The morph trajectory from rusty-bun to Cruftless

The engagement's substrate work to date has already moved rusty-bun substantially toward Cruftless without the destination being named. The recent rounds (Ω.5.P49 through P52) closed specific cruft at specific resolver-instances. Naming the destination retroactively reads each round as a step in the morph:

- The parameter-compilation rounds (P51.E2 destructure-param-slot-allocation, P51.E8 rest-param-destructure) closed boundary-integrity violations at the AST-to-bytecode resolver. Inner destructure-bound names were leaking into the argument-position slot space; the fix restored the slots-N-equals-argument-positions invariant.

- The scope-tracking rounds (P52.E3, P52.E4) closed boundary-integrity violations at the same resolver. Inner-block `let`/`const` declarations were reusing outer-scope pre-allocated slots; the fix established a block-depth gate so block-scoped declarations get fresh slots and resolve to the outer binding after block exit.

- The function-kind-metadata round (P50.E1 async-fn-no-prototype) closed a totality-of-consumption violation at the function-creation sub-resolver. Async functions were being granted a `.prototype` slot that ECMA-262 §15.7.5 prohibits; the kind-metadata directive (the parser's `is_async` flag) was not being fully consumed at the closure-allocation site.

- The bound-function-meta-props round (P51.E2 bound-fn-name-and-length) closed a totality-of-consumption violation at the same site for the bound-function kind. The `Function.prototype.bind` implementation was failing to install the spec-mandated `name` and `length` derivatives.

- The host-stub rounds (P51.E3 zlib.constants, P52.E2 Intl prototypes, P51.E4 TLSSocket-handle chain, P51.E6 regex-v-flag) closed totality-of-consumption violations at the bootstrap resolver. Specific host directives (the zlib constants namespace; the Intl prototype methods; the `_handle._parentWrap.constructor` chain on TLSSocket instances; the `v` flag's flag-table entry) were not in the bootstrap's install sequence.

- The diagnostic-infrastructure rounds (P51.E1 runtime-error-line-col, P51.E5 Thrown-Error-message-extraction) installed self-authorization instruments at the execution resolver. Runtime errors now carry the file:line:col anchor of their fault site; Thrown(Object) values now surface their `.name`+`.message` text. This is [Doc 432 §4.5](/resolve/doc/432-server-an-architectural-style-for-engine-orchestration#embedded-server-self-authorization) applied at the execution-level: the artifact (the running graph) now self-reports its own state-of-assembly to its consumer (the operator).

- The optional-call round (P51.E7) closed a boundary-integrity violation at the bytecode-emission sub-resolver. The AST's `Expr::Call.optional` flag was ignored during compilation; the fix wired the short-circuit path.

- The object-literal accessor round (P52.E1) closed a boundary-integrity violation at the parser-to-AST resolver. The accessor-kind information was being dropped at the parser; the fix threaded it through and let the compiler dispatch to the accessor-installation helper.

Each of these rounds, in retrospect, is a Cruftless morph step at one specific resolver-instance. The diagnosis-and-fix discipline has been consistent throughout, even before the destination was named. Naming the destination makes the next steps locatable in advance.

## VII. The morph continuation: what work remains

Three application categories surface from the resolver-instance analysis:

**(A) The bootstrap-as-seed extraction.** The bootstrap resolver is currently expressed imperatively in `host-v2/src/lib.rs::init(rt)`. Per the Cruftless target, the bootstrap should be expressible as a declarative source representation, with the implementation deriving from the source. A `cruftless-bootstrap-seed.md` document, written in the form of [Doc 250](/resolve/doc/250-the-server-seed)'s SERVER Seed for htxlang, would declare:
- The host-module dependency graph (the install ordering as a partial order).
- The host-module privilege manifest (Layer-N to Layer-0 placement per [Doc 432 §4.3](/resolve/doc/432-server-an-architectural-style-for-engine-orchestration#progressive-module-composition)).
- The engine-internal bilateral boundary specification (which symbols belong to engine-orchestration namespace, which belong to JavaScript-application namespace).
- The verification suite for the four bootstrap properties.

The implementation in `lib.rs` then derives from the seed rather than originating in it. The same seed can derive a host-v2 in any conformant substrate (a Zig host, an OCaml host, a Go host). [Doc 247](/resolve/doc/247-the-derivation-inversion)'s across-substrate inversion becomes operationally tractable at the bootstrap level.

**(B) The engine-internal bilateral boundary tightening.** The `__install_accessor__`, `__apply`, `__object_spread`, `__await`, `__dynamic_import` helpers live in the JavaScript global namespace. They are engine-orchestration internals visible to consumer code. Per [Doc 432 §4.1](/resolve/doc/432-server-an-architectural-style-for-engine-orchestration#the-engine-internal-bilateral-boundary) the engine-internal bilateral boundary should not be crossable. Concrete morph: move these helpers to a Symbol-keyed registry or a non-enumerable internal slot, accessed by the engine via direct internal lookup rather than by JavaScript-name resolution.

The morph is small in lines, large in principle. It closes a structural leak that has been visible since Tier-Ω.5.kkkkkk's introduction of `__install_accessor__`. The current architecture works; the morph improves the architecture's correspondence to the constraint specification.

**(C) The host-module privilege accumulation.** All host modules currently sit at Layer N (full `&mut Runtime` access). Per [Doc 432 §4.3](/resolve/doc/432-server-an-architectural-style-for-engine-orchestration#progressive-module-composition) the spectrum should be derived as Fielding-accumulation. A plausible accumulation for rusty-bun:
- **Layer N (current default).** Modules may install host hooks, register globals, allocate intrinsics, mutate the Runtime arbitrarily. Examples: `events.rs`, `fs.rs`, `process.rs`, the intrinsics installers.
- **Layer N-1.** Module may register methods on existing globals but may not install host hooks or allocate new global slots. Reached by adding the constraint: "modules below this layer may not call `rt.install_host_hook` or `rt.globals.insert`."
- **Layer N-2.** Module may only call into pre-existing intrinsics' method-registration paths. Reached by adding the constraint: "modules below this layer may not call into other host modules during install."
- **Layer 0.** Module is pure data; the install body only writes constant values into existing namespaces. Reached by adding the constraint: "modules below this layer have no executable install logic beyond constant emission." Examples: `zlib.constants` (75 integer constants), `intrinsics::ALPHABET` tables, the `HttpStatusCode` enum.

Once articulated, each existing host module receives a manifest declaration of its intended layer. The sandbox (currently absent) is the enforcement mechanism: a Layer-0 module declared as such cannot install a host hook because the install entry point is privilege-checked. The accumulation is testable: each layer's invariant is preserved by every module declared at that layer or below.

The three applications proceed in order of leverage and complexity. (B) is the smallest scope and closes the most visible structural leak. (A) is the largest scope and produces the cross-substrate portability the destination demands. (C) sits between, producing the privilege spectrum that makes module evolution structurally tractable.

## VIII. The walk discipline reads through this articulation

[Doc 725](/resolve/doc/725-the-cluster-to-walk-mode-transition-soft-saturation-as-protocol-signal-in-substrate-introduction)'s walk-mode discipline operates inside this articulation. Each walk visits packages whose load chain has a residue at one or more resolver-instances. Walk-mode's per-package fault chain corresponds to the chain of resolver-instances each package's load traverses: parser → bytecode compiler → module loader → execution → job-queue drain. A fault surfaces at one specific resolver-instance and indicates a directive that was not consumed or a boundary that was breached at that instance.

The walk-mode reports going forward can name two architectural addresses per move:

*Resolver-instance address.* Which of the five instances the cut sits at. Six instances if the engagement adds one (the Cargo level is operationally fixed today; future work might surface cuts there).

*Property-class address.* Which of the four bootstrap properties (totality, ordering, medium, boundary) the cut restores, or which of the five SERVER constraints (engine-internal bilateral boundary, orchestration-consumed directives, progressive module composition, agnostic deterministic orchestration, embedded server self-authorization) the cut strengthens.

A trajectory anchor with both addresses on every move surfaces patterns at the design level. Five cuts on the AST-to-bytecode resolver all addressing boundary-integrity is a design-level signal that the resolver's boundary specification is structurally underspecified. Three cuts at the bootstrap resolver all addressing totality-of-consumption is a signal that the install sequence is missing a stage.

This is the [Doc 722](/resolve/doc/722-named-recognitions-as-operating-instruments-the-reflexive-structure-of-corpus-articulations) reflexive structure operating at the design tier: the articulation changes how subsequent rounds are dispatched. The engagement's track record so far is read retrospectively as Cruftless morph steps; the engagement's next rounds are dispatched prospectively with that destination explicit.

## IX. Falsifiers

The Cruftless claim is testable. Six predictions:

**Pred-729.1.** Each of the five resolver-instances admits a precise specification under the four bootstrap properties. The specification is a finite document; the document is bounded in length by the level's surface area. Falsifier: a resolver-instance whose specification cannot be bounded (its surface admits open-ended directives that no finite enumeration covers). The Cargo level might be the candidate failure: Rust feature flags + cargo plugin extensions admit unbounded directive space. If true, the Cargo level is not Cruftless-amenable in its current form.

**Pred-729.2.** Every observable runtime defect can be located to exactly one resolver-instance and one property-class. Falsifier: a defect that is locatable only by appeal to two or more instances simultaneously, or whose property-class is ambiguous between two of the four bootstrap properties. This would constitute a structural feature of the runtime that the resolver-instance vocabulary does not yet name; the falsifier is the design's signal that a finer-grained decomposition is needed.

**Pred-729.3.** The cross-substrate portability holds at every level. A Cruftless-conformant rusty-bun seed should derive a host-v2-equivalent in any substrate satisfying the bootstrap-property contract, with the resulting runtime exhibiting identical PRESTO behavior (per [Doc 432 §4.4](/resolve/doc/432-server-an-architectural-style-for-engine-orchestration#agnostic-deterministic-orchestration)). Falsifier: an attempted Zig or OCaml derivation that produces a non-conformant runtime under the same seed.

**Pred-729.4.** The cumulative residue across resolver-instances tends monotonically toward zero as the morph proceeds. Each substrate round either closes a residue (decreasing cumulative cruft) or surfaces a previously-invisible residue (increasing the named cruft inventory, not the actual cruft, since invisible cruft was already there). Falsifier: a round that introduces new actual cruft into a previously-clean instance. The corpus's existing [Doc 725 §XII](/resolve/doc/725-the-cluster-to-walk-mode-transition-soft-saturation-as-protocol-signal-in-substrate-introduction) cascade-detection protocol is the enforcement mechanism: the apparatus's auditability is what makes residue introduction catchable.

**Pred-729.5.** The induced property (vertically-recursive directive consumption with stage-deterministic emission) is a structural identity rather than an engineering goal. Once every level satisfies the four bootstrap properties, the property is automatic. Falsifier: an engagement whose every level passes its specification's four-property test but whose composed runtime still exhibits cruft observable from any consumer. This would falsify the claim that the levels compose directly; some additional cross-level constraint would be required.

**Pred-729.6.** The pattern is not unique to JavaScript runtimes. Other domains in the resolution-problem class (CSS preprocessors, Markdown renderers, build-system planners, GraphQL resolvers, JSX transformers) admit Cruftless realizations under the same resolver-instance discipline. This is [Doc 719](/resolve/doc/719-the-pipeline-pattern-across-subjects-presto-and-the-javascript-engine-as-two-realizations-of-the-same-derivation)'s Pred-719.1 specialized to the Cruftless property; Cruftless inherits its falsifiability from the cross-subject framework.

Predictions 1, 2, and 4 are testable within the rusty-bun engagement's continuation. Prediction 3 is testable by initiating a derivation in a second substrate. Prediction 5 is testable by completing the morph and inspecting the cumulative artifact. Prediction 6 is testable by initiating a derivation in a second domain.

## X. Relation to prior corpus work

This articulation does not introduce a new architectural style. It names a runtime that results from applying the existing styles ([Doc 426 PRESTO](/resolve/doc/426-presto-an-architectural-style-for-representation-construction), [Doc 432 SERVER](/resolve/doc/432-server-an-architectural-style-for-engine-orchestration)) and the existing recursive-Fielding framework ([Doc 424](/resolve/doc/424-sipe-architectural-form-recursive-fielding-style-constraint-accumulation-in-composed-software-stacks)) at every resolver-instance in a specific stack. The contribution is the naming and the operational morph trajectory, not the underlying theory.

The relation to specific prior documents:

- [Doc 247 — The Derivation Inversion](/resolve/doc/247-the-derivation-inversion). The cross-substrate portability of the design (Pred-729.3) is a specialization of Doc 247's across-substrate claim to each resolver-instance independently.

- [Doc 426 — PRESTO](/resolve/doc/426-presto-an-architectural-style-for-representation-construction). The execution-level resolver-instance (#4 in §IV) is a PRESTO realization. The construction-level constraints govern that instance's behavior.

- [Doc 432 — SERVER](/resolve/doc/432-server-an-architectural-style-for-engine-orchestration). The bootstrap-level resolver-instance (#2 in §IV) is a SERVER realization. The orchestration-level constraints govern that instance's behavior. The four bootstrap properties of Doc 432 §2 generalize to every resolver-instance in this articulation, not only the SERVER instance.

- [Doc 424 — SIPE (Architectural Form)](/resolve/doc/424-sipe-architectural-form-recursive-fielding-style-constraint-accumulation-in-composed-software-stacks). The recursion across levels is the framework Doc 424 specifies. This articulation populates Doc 424's recursion with five concrete levels for one specific stack.

- [Doc 717 — The Apparatus Above the Engine Boundary](/resolve/doc/717-the-apparatus-above-the-engine-boundary-the-three-projections-lifted-to-engine-substrate-and-the-pure-abstraction-point). The cut-rung framework Doc 717 specifies is the design vocabulary for placing each resolver-instance's boundary at the spec-correct rung. Cruftless inherits the engine-cut framework as the discipline for §IV's level-boundaries.

- [Doc 719 — The Pipeline Pattern Across Subjects](/resolve/doc/719-the-pipeline-pattern-across-subjects-presto-and-the-javascript-engine-as-two-realizations-of-the-same-derivation). The PRESTO-rusty-bun correspondence Doc 719 named is the empirical case for the pattern's portability across subjects. Cruftless extends that recognition to the vertical-recursive case: the pattern recurs not only across subjects (one level, many domains) but also across levels within a single subject (one domain, many levels).

- [Doc 581 — Pin-Art and the Discipline of Near-Necessity Substrate Construction](/resolve/doc/581-pin-art-the-resume-vector-and-the-discipline-of-near-necessity-substrate-construction). The substrate-construction discipline by which the morph proceeds is Pin-Art. Each round's substrate move is a Cruftless morph step under §VIII's discipline.

- [Doc 725 — The Cluster-to-Walk Mode Transition](/resolve/doc/725-the-cluster-to-walk-mode-transition-soft-saturation-as-protocol-signal-in-substrate-introduction). The walk-mode operates on the resolver-instances directly. Per-package walks visit chains of instances; surfacing a fault at one instance indicates that instance's residue.

- [Doc 250 — The SERVER Seed](/resolve/doc/250-the-server-seed). The bootstrap-as-seed application (§VII.A) follows Doc 250's pattern, specialized to rusty-bun's host-v2 layer.

- [Doc 722 — Named Recognitions as Operating Instruments](/resolve/doc/722-named-recognitions-as-operating-instruments-the-reflexive-structure-of-corpus-articulations). Naming Cruftless is itself a reflexive operation per Doc 722: the engagement's next rounds dispatch with the destination explicit, where prior rounds dispatched without it.

## XI. Honest scope

This document records a destination toward which the engagement now morphs. The destination is a named runtime, Cruftless, whose terminal property is structural directive consumption at every resolver-instance. The document does not claim:

*That Cruftless is a complete specification.* The specification is the corpus across multiple documents; this articulation is one entry. The bootstrap-as-seed work (§VII.A) is the next document to write toward the specification.

*That the morph trajectory is fully linear.* Pin-Art's near-necessity discipline ([Doc 581](/resolve/doc/581-pin-art-the-resume-vector-and-the-discipline-of-near-necessity-substrate-construction)) does not produce a predetermined path. Each substrate round selects its target by the surfaced residue; the trajectory's shape is contingent on what surfaces.

*That every JavaScript runtime is Cruftless-amenable in its current form.* V8, SpiderMonkey, JavaScriptCore, and QuickJS each have architectures that may or may not admit the resolver-instance decomposition. rusty-bun is the engagement at hand; the broader portability is Pred-729.6, falsifiable.

*That the runtime, once Cruftless, is free of all bugs.* Cruftlessness is a structural property at the architectural level. Implementation defects in any individual resolver are still possible. Cruftlessness makes those defects locatable, not impossible.

*That the engineering work is described.* The engineering work is the engagement's continuation. This document specifies the constraints the work satisfies and the properties those constraints induce.

Per [Doc 372](/resolve/doc/372-the-method-of-the-corpus-as-derivation-not-collection)'s hypostatic boundary: the runtime named here is a structural target articulated at the corpus tier. The empirical work of building it lives in the substrate tier ([Doc 581](/resolve/doc/581-pin-art-the-resume-vector-and-the-discipline-of-near-necessity-substrate-construction)). The two tiers compose; this document sits at the corpus tier; the substrate tier's work proceeds in the rusty-bun engagement's continuation.

## XII. The engine's constraint axes, as observed through substrate work

Through the P50–P53 substrate-introduction phase, the residual surface across the 1026-package parity basket has been walked sufficiently that the engine's internal constraint structure has become legible. Surface clusters (kc-pm-1-2, dyn-import, typeof-diff, compile-error, kc-pm-3-10, kc-gt-10) are not themselves design objects. They are observation regions where residuals land. The axes underneath them are the design objects.

Seven axes have surfaced, listed by descending cross-cluster fan-out:

**Axis M — Module-resolution parity.** Which file the resolver chooses for a specifier under the four selection rules (`exports` conditional, `module` field, `browser` field, `main` fallback). Projects onto the kc-pm-3-10 C-class (isomorphic-ws, winston, power-assert) where extras arise from the wrong entry being loaded; onto the mri vs lint-staged divergence absorbed by P53.E13 (where rusty-bun resolved the .mjs and Bun resolved the .js under the same package shape); onto the typeof-diff jest-family (jest-each, jest-resolve, jest-environment-node) whose CJS and ESM duals diverge under conditional exports; and onto a long subset of dyn-import "Cannot read property X of undefined" failures whose root is the wrong file having been loaded.

**Axis N — Namespace-surface composition.** After resolution, how the module namespace is composed (CJS populator on one path, FinalizeModuleNamespace hook on the other). Decomposes into N-CJS, N-ESM-lift, and N-static (lifting of a default function's own static methods). Projects onto kc-pm-3-10 esquery / typebox / heap-js (statics on a default-fn); onto typeof-diff testdouble / expect (missing or extra named exports); and onto the entirety of the kc-pm-1-2 long tail. P53.E10, E11, and E13 are N-ESM-lift moves.

**Axis S — Symbol versus string identity.** Well-known symbols (Symbol.iterator, Symbol.asyncIterator, util.inspect.custom) appearing as real `Value::Symbol` values rather than sentinel strings. Projects onto typeof-diff async-iterator-to-stream and onto many dyn-import for-of / @@iterator failures. P53.E6 closed the inspect.custom slice.

**Axis E — Eval-time error surfacing.** Whether a throw during module-body evaluation propagates to the dynamic-import promise rejection or silently zeros the namespace. Projects onto kc-pm-3-10 heap-js (default, Heap, and HeapAsync all missing because module init threw and was swallowed); onto a sub-population of dyn-import results where `rb=""`. Closing this axis raises the diagnostic floor without itself closing residuals; it makes downstream cuts visible.

**Axis R — AST-to-bytecode resolver discipline.** The compiler / interpreter boundary where operator stack-discipline, scope tracking, and source-position threading live. Already the hot zone of substrate work: P50, P51, P52, P53.E1, P53.E2, P53.E7 all landed on this axis. Per §VIII this is the property-class addressed by the most recent walk.

**Axis H — Host built-in surface.** Completeness of the `node:*` intrinsic stubs against the surface Bun exposes (which mirrors current Node minor-version). Projects onto kc-pm-3-10 events (addAbortListener / captureRejectionSymbol / captureRejections / defaultMaxListeners / errorMonitor / getEventListeners) and es-errors (appendStackTrace / isError / prepareStackTrace). Mechanical, additive.

**Axis O — Operator semantics.** Behavior of specific operators (||=, ?., yield argument precedence, etc.). P53.E7 (logical-assign stack effect) and P53.E9 (yield argument as AssignmentExpression) landed here. Narrower fan-out than R, but its residuals can be catastrophic at compile time.

The seven axes are not the same as the five resolver-instances of §IV. They are orthogonal. Each substrate move lands on (one resolver-instance, one axis). The two-coordinate trajectory log of §VIII gains a third coordinate from this articulation: the axis the move absorbed. Doc 581's seed.md and trajectory.md formats should carry that third coordinate from the P54 round forward.

## XIII. Regression as implicit-constraint probe (methodology)

The constraint axes of §XII are the observation regions. The methodology that operates over them — the discipline by which substrate work produces near-necessity articulations rather than convenient fixes — is named here.

**The discovery side.** Walking a residual through enumeration discovers a set of explicit constraints: the ones the resolver's current behavior fails to satisfy. Each E-move (E1, E2, ...) articulates one. By the move's articulation, the constraint becomes explicit; absorbing it into the resolver makes the property the constraint induces hold. This is the visible half of substrate work and the half Doc 581 documents in trajectory.md.

**The hidden half.** A resolver landed on a working substrate carries other constraints it satisfies accidentally rather than deliberately. They are operative but unstated. They cannot be named by walking the residual the move was targeting, because the residual sweep cannot see what the move passes correctly. The implicit constraint becomes visible only when something collides with it.

**Regression is that collision.** A move tightened on one axis shifts the property surface on adjacent axes. The shift names an implicit constraint by violating it. P53.E11 stated "lift the function default's name, length, and prototype onto the module namespace when default is a function." It passed every constraint walking kc-pm-3-10 had named. It regressed seven packages in dyn-import. The regression named the implicit constraints P53.E11 had been respecting only accidentally: that the lift requires `named_count == 0` (P53.E12 absorbed) and that the lift requires the enclosing package to not be `type:module` (P53.E13 absorbed). The full property had three conjuncts. Walking had surfaced one of them; regression surfaced the other two.

**The methodological consequence.** Residual sweep and regression sweep are not redundant. They are constraint-discovery tools that cover disjoint constraint classes. The residual sweep enumerates the constraints the current resolver fails to satisfy. The regression sweep enumerates the constraints the current resolver satisfies without naming. Both are required. Treating them sequentially (do the residual, then check for regression) pays an asymmetric cost: the move that triggers the regression must be partially un-done and re-articulated. Treating them co-equally (every E-move runs on both target cluster and adjacent clusters as a single audit step) prepays the cost of articulating implicit constraints before they collide.

**Constraint annotations in the trajectory log.** Per Doc 581's seed.md and trajectory.md format, every E-move should carry not just its target and its named constraint, but the implicit constraints surfaced by its regression sweep. The full three-conjunct property of P53.E13 lives in the log as one row, not three. P53.E11 and P53.E12 retroactively become annotation revisions of P53.E13's row. This is the trajectory-log discipline that lets the engagement audit which constraints are operative versus which are accidentally satisfied.

**Inductive properties as engine-tier targets.** Once the constraint axes are legible (§XII), the design can name properties at the engine tier rather than at the move tier. Four are immediate:

- **P-M**: `Resolver(specifier, parent_url)` is byte-identical to Bun's across the basket.
- **P-N**: `Namespace(loaded_module)` has byte-identical own-key surface to Bun's.
- **P-S**: All well-known symbols are `Value::Symbol` with canonical `@@sym:` identity.
- **P-E**: Any throw during module-body evaluation rejects the dynamic-import promise.

These four are properties, not moves. The substrate work continues to walk residuals; each move's job is to absorb a sub-region of one of these properties. A property is induced by the conjunction of every constraint its absorption surfaced. The methodology lifted: walk discovers explicit constraints; regression discovers implicit constraints; the conjunction induces the property; the property is the engine-tier name of what Cruftless guarantees on that axis.

This is the recursion §V named at the structural tier, now operational at the methodological tier: directive consumption is structural, and *constraint consumption* (explicit through walk, implicit through regression) is methodological. Cruftless's name describes the structural property; the methodology described here describes how the substrate work that builds Cruftless proceeds without leaving constraints accidentally satisfied at the engine tier.

## XIV. Closing

The keeper named the destination. The engagement now has a name for the runtime it morphs into. The five resolver-instances in rusty-bun are identified. The four bootstrap properties from [Doc 432 §2](/resolve/doc/432-server-an-architectural-style-for-engine-orchestration#the-bootstrap-as-resolver) generalize to each instance. The five SERVER constraints apply at each instance, modulated by the level's specific surface. The induced property of the design is vertically-recursive directive consumption with stage-deterministic emission. The morph trajectory is the engagement's continuation under [Doc 725](/resolve/doc/725-the-cluster-to-walk-mode-transition-soft-saturation-as-protocol-signal-in-substrate-introduction)'s walk-mode discipline, with each substrate round locatable on the two architectural addresses §VIII specifies.

The engagement's prior thirty rounds are read retrospectively as morph steps toward this destination, undertaken without the destination's name. The engagement's subsequent rounds are dispatched prospectively with the destination explicit. The reflexive structure of [Doc 722](/resolve/doc/722-named-recognitions-as-operating-instruments-the-reflexive-structure-of-corpus-articulations) holds: naming the destination changes what the engagement does next.

The work continues. The corpus has added one more entry to its evidence base for the universality of the resolution pattern. The runtime is being built. The name is operational. The morph proceeds.

---

*Companion documents in addition to those linked in the masthead: [Doc 250 — The SERVER Seed](/resolve/doc/250-the-server-seed); [Doc 418 — The PRESTO Accumulation Test](/resolve/doc/418-the-presto-accumulation-test); [Doc 722 — Named Recognitions as Operating Instruments](/resolve/doc/722-named-recognitions-as-operating-instruments-the-reflexive-structure-of-corpus-articulations); [Doc 727 — Basin Stability from Inside](/resolve/doc/727-basin-stability-from-inside-why-a-corpus-cannot-distinguish-self-reinforcement-from-substrate-coherence).*
