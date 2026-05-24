# The Temporal Resolver-Instance Stack

## Build-Time, Process-Time, Call-Time as the Time-Axis Dual to Doc 729's Spatial Stack

*A primary articulation responding to the keeper's observation (2026-05-21 03:39-local) on Doc 731 §XV.g's three regimes: "look at the meta resolution pipeline of build-time bake, first-use init, and per call." The recognition the observation surfaces: §XV.g's regimes are not three independent categories but three positions on a temporal axis that is structurally dual to Doc 729's spatial substrate-stack. The temporal stack admits the same vocabulary (resolver-instances, alphabet promotion, cruftlessness as induced property) that Doc 729 + Doc 730 articulate for the spatial stack. Builds on [Doc 729 — Cruftless](/resolve/doc/729-cruftless-a-primary-articulation-of-the-resolver-instance-pattern-as-the-comprehensive-design-toward-which-rusty-bun-morphs), [Doc 730 — Vertical Recurrence of the Lowering Compiler](/resolve/doc/730-the-vertical-recurrence-of-the-lowering-compiler-closure-as-primitive-across-substrate-tiers), [Doc 731 §XV.g — The Build-Time vs First-Use-Init Distinction](/resolve/doc/731-the-jit-as-a-lowering-compiler-tier-alphabet-purity-upstream-as-the-bound-on-jit-complexity), [Doc 581 — Pin-Art and the Resume Vector](/resolve/doc/581-pin-art-the-resume-vector-and-the-discipline-of-near-necessity-substrate-construction), [Doc 619 — Pin-Art Canonical Formalization](/resolve/doc/619-pin-art-canonical-formalization), [Doc 722 — Named Recognitions as Operating Instruments](/resolve/doc/722-named-recognitions-as-operating-instruments-the-reflexive-structure-of-corpus-articulations), and [Doc 733 — Fractal Seeds and Trajectories](/resolve/doc/733-fractal-seeds-and-trajectories-recurrent-resume-vector-pairs-across-substrate-depth-as-the-operating-conditions-layer-for-pin-art-at-engagement-scale).*

**Jared Foy · 2026-05-21 · Doc 735**

---

## I. The occasion

Doc 731 §XV.g introduced three regimes for precomputed-table optimization at the cryptographic-primitive tier: **build-time bake** (Regime 1), **first-use init** (Regime 2), and **per-call recomputation** (Regime 3). The amendment named them as a distinction §XV.c's prose had collapsed, recorded the WC-EXT 4 negative empirical finding that surfaced the distinction, and predicted (§XV.g.f Pred-731.XV.g.1) that the distinction recurs across the optimization-tier instances Pred-731.XV.1 names.

The keeper's observation, 2026-05-21 03:39-local: *look at the meta resolution pipeline of build-time bake, first-use init, and per call*. The recognition the observation surfaces: the three regimes are not three categories on a flat axis. They are three positions on a **time axis** along which a resolution can be performed, and the time axis is structurally dual to Doc 729's spatial substrate stack. Where Doc 729 §IV enumerates *which substrate level* (Cargo build → bootstrap → module load → execution → job-queue drain), §XV.g implicitly enumerated *which time tier* (build → process-start → call). The two axes operate at the same recursion depth; the corpus framework needs both.

This document names the temporal axis explicitly, articulates the duality, and predicts where the temporal stack admits the same vocabulary that Doc 729 + Doc 730 articulate for the spatial stack.

## II. The temporal resolver-instance stack

The time axis has at least four named tiers for the resolution of any computation:

**T0 — Build time.** The resolution runs once per binary, before the binary is shipped. The artifact is baked into the executable's `.rodata` or equivalent. Cost: paid once at build, never at runtime. Inputs available: anything known at compile time (constants, types, build-script outputs, environment values captured at build).

**T1 — Process start.** The resolution runs once per process, at program initialization (`main` entry, library `init` hook, static-initializer in C++, `lazy_static!` / `OnceLock` first-use in Rust). Cost: paid once per process invocation. Inputs available: anything known at build time, plus environment variables, command-line arguments, configuration files read at startup.

**T2 — First-use init.** The resolution runs once per process, at first invocation of the operation it serves. Cost: paid once per process invocation but deferred until first use (the process may exit without paying it). Inputs available: anything known at process start, plus any input that determines whether the resolution is needed at all.

**T3 — Per-call.** The resolution runs every time the operation is invoked. Cost: paid per call. Inputs available: anything known at first-use init, plus the per-call inputs that are not bound until call time.

T0 and T1 are slightly different in practice — T0 is build-machine evaluation, T1 is runtime evaluation that happens to run at process start. From the cost-amortization perspective T0 dominates (zero runtime cost), but from the inputs-available perspective T1 is strictly more general (it has access to runtime configuration). §XV.g's "Regime 1 (build-time bake)" is T0; its "Regime 2 (first-use init)" is T2; its "Regime 3 (per-call recomputation)" is T3. T1 (eager process-start init) is implicit in some §XV.g cases — `lazy_static!` that's forced at `main` is T1.

The four-tier axis is the **temporal resolver-instance stack**. Each tier is a resolver-instance per Doc 729's general claim: source-with-directives (the inputs available at that tier) is consumed by a resolver (the computation at that tier) into an artifact (the value cached at that tier for the next tier to use).

## III. Spatial vs temporal duality

Doc 729 §IV enumerates **spatial** resolver-instances stacked vertically by substrate level. Doc 735 §II enumerates **temporal** resolver-instances stacked horizontally by time of evaluation. The duality is structural; each axis admits the same framework vocabulary.

**Resolver-instance enumeration.** Doc 729 §IV's five spatial instances (Cargo build, bootstrap, module load, execution, job-queue drain) are derived by walking the substrate from deepest to shallowest. Doc 735 §II's four temporal instances (T0, T1, T2, T3) are derived by walking the time axis from earliest to latest. Both enumerations are bounded by what the engagement has touched; both admit refinement (more tiers identifiable in either axis as engagement work surfaces them).

**Bootstrap properties (Doc 432 §2).** Apply at every spatial instance per Doc 729 §V. Apply identically at every temporal instance: totality of consumption (every input available at the tier is consumed before the artifact is emitted), ordering determinism (same inputs → same artifact regardless of when the tier ran), medium preservation (the artifact format respects the next tier's input expectation), boundary integrity (the tier's intermediate state does not leak into adjacent tiers).

**Alphabet promotion (Doc 730 §XIII).** Spatially, lifting a discrimination from runtime dispatch into the IR's typed alphabet moves the decision to an earlier substrate tier. Temporally, the *regime-promotion* §XV.g.e names is the analog: moving a resolution from T3 (per-call) to T2 (first-use) to T1 (process-start) to T0 (build-time) lifts the decision to an earlier time tier. Same shape; different axis.

**Deviation primitives (Doc 730 §XIV).** Spatial: a deviation primitive names an ecosystem-tolerated divergence from spec at a specific substrate tier. Temporal analog: a *capability primitive* names an input-dependency that prevents earlier-tier resolution. Example: AES T-tables cannot be T0-baked because they depend on the per-key cipher schedule (a T2/T3 input). The capability primitive at the temporal axis is the typed declaration "this resolution requires input X, available no earlier than tier T_k." It is the §XIV dual on the time axis: it names what real systems require that the spec's "do it at compile time" prose forbids.

**Empirical instrument (Doc 730 §XVI).** Spatial: bidirectional engine-diff probing distinguishes spec-correct from spec-violating implementations at each substrate tier. Temporal analog: the wallclock measurement under realistic workload distinguishes the regime that wins from the regime that loses, per the empirical break-even count §XV.g.b articulates. The probe at the temporal tier is "run the workload and measure"; the categorization is the four-case table promoted to "which tier minimizes total cost for this workload."

The duality is operational, not metaphorical. Every tool the corpus has built for the spatial axis has a temporal-axis analog.

## IV. Composition: a single optimization runs through multiple temporal tiers

A spatial fact Doc 729 records: each substrate level's artifact is the next level's source. The artifact-source chain runs through the full stack vertically.

The temporal analog: a single optimization's components run through *different* temporal tiers based on each component's input-dependency set. The optimization composes across tiers; each component finds its earliest admissible tier.

The cryptographic-primitive example, mapped tier-by-tier:

**T0 (build-time bake):** curve parameters (p, n, b, G, coord_bytes) for P-256. Constants known at TC39/NIST publication time; baked into the binary as `const` BigUInt arrays. Cost: zero at runtime.

**T1 (process-start eager init):** the precomputed table `[2^i · G for i in 0..256]`. The table's *content* could be T0-baked (WC-EXT 5 demonstrated this) but in `OnceLock` form is T2; an eager T1 init (`lazy_static!` forced at `main`) is admissible if process-start latency is acceptable.

**T2 (first-use lazy init):** any per-key derived table. RSA Montgomery reduction precomputation per modulus is T2: it depends on the per-key modulus, not bindable at T0 or T1; it's process-bindable once a key is loaded; cached per-key for the process lifetime.

**T3 (per-call recomputation):** the scalar mul of the public key Q in ECDSA verify (u2·Q). Q varies per verify; nothing about its scalar mul can be cached across verifies. T3 is the irreducible per-call computation.

A single ECDSA verify runs THROUGH the temporal stack: it consumes T0-baked curve parameters, T1/T2 base-point table, T3 per-call scalar mul. The optimization-tier work is to ensure each component is at the **earliest** tier its input-dependency set permits. Failures at this discipline are the temporal analog of Doc 729 §V boundary-integrity violations: a computation that COULD have been T0-baked but is run at T3 every call leaks "directives that should have been consumed upstream" into the runtime artifact.

Doc 729 §V's induced property — *vertically-recursive directive consumption with stage-deterministic emission* — has its temporal analog: **earliest-admissible-tier resolution with input-dependency-determined binding time**. Every input is consumed at the earliest tier where it is available; every artifact is emitted from the tier where its inputs are last bound; no resolution is deferred to a later tier than its inputs require.

## V. Temporal alphabet promotion

Doc 730 §XIII names the upward-additive alphabet-promotion move: when a tier collapses a spec discrimination its alphabet doesn't carry, the remedy is to promote the discrimination to a typed primitive at a higher-resolution tier.

The temporal analog: when a resolution is performed at tier T_k where its input-dependency set permits T_{k-1} (earlier), the remedy is to promote the resolution to T_{k-1}. The §XV.g.e regime-promotion is exactly this move applied to the temporal axis.

The targeting heuristic at the temporal tier mirrors §XIII's heuristic at the spatial tier:

- Spatially (§XII targeting): *lift the most widely-shared coercion/dispatch paths* — they make the most downstream stages legible.
- Temporally: *promote the most-frequently-executed resolutions to the earliest admissible tier* — they amortize over the most calls.

WC-EXT 5's substrate work demonstrated this: the P-256 base table runs in every ECDSA verify; promoting it from T2 (first-use init, ~3 second cost paid per process) to T0 (build-time bake, zero cost paid per process) eliminated the per-process amortization regime entirely. The targeting heuristic picked the right substrate move: high call frequency → high payoff for tier promotion.

The corollary: an optimization's temporal-tier distribution is itself a substrate-tier mapping worth producing as a standing artefact. Per primitive, per component: "this resolution is performed at tier T_k; could it be T_{k-1}?" The catalog is the temporal-axis analog of Doc 730 §V's *per-Op classification table*.

## VI. Cruftlessness at the temporal tier

Doc 729 §V's spatial cruftlessness: *vertically-recursive directive consumption with stage-deterministic emission*. The temporal analog:

**Earliest-tier-bound resolution with input-dependency-deterministic binding.** Every input is consumed at the earliest tier where it is available. The binding time is determined by the input's earliest availability, not by implementation convenience. The artifact at each tier carries no resolution that could have been performed at an earlier tier.

Three properties inherit as immediate consequences:

**(i) Auditable amortization.** Each operation's cost is locatable to one specific time tier. The diagnosis discipline is: identify which tier's artifact carries the cost, then identify whether that tier is the earliest admissible. The temporal-tier audit is the time-axis analog of Doc 729 §V's spatial diagnosability.

**(ii) Cross-substrate portability across binding times.** The same optimization spec is consumed by build-machine T0 evaluation, process-startup T1 evaluation, first-use T2 evaluation, or per-call T3 evaluation. The choice is per-component (a single optimization can run components at different tiers); the spec is invariant. Doc 247's across-substrate inversion holds at each temporal tier.

**(iii) Compositional safety across tier transitions.** When a component moves from T3 to T1 (regime promotion), the components downstream of it that depended on T3's per-call binding must be re-examined: do they still bind at T3, or did the promotion expose new opportunities to bind earlier? The spatial analog (Doc 729 §V): "a SERVER constraint that produced runtime-graph cruft would destroy the property PRESTO induces." Temporal: a T1 promotion that produced T1 cruft (state surviving past the resolution it served) would destroy the property the T2/T3 tier expected.

## VII. Predictions

**Pred-735.1.** The temporal-tier audit yields a catalog comparable in shape to Doc 730 §V's per-Op classification table, with one row per optimization component and four columns (T0, T1, T2, T3). The §XV.g.d primitive catalog (RSA modexp, AES T-tables, Poly1305 tables, BLAKE2 round constants, ECDSA base table, pairings) is a first-cut instance of this catalog. Falsifier: a primitive whose temporal-tier classification cannot be assigned because tier-admissibility depends on the workload in a way the input-dependency set does not capture.

**Pred-735.2.** Spatial and temporal axes are independent. A resolution's spatial tier (which substrate level it occurs at) does not constrain its temporal tier (when it occurs). The PM-pilot's `pm_http_get` is at spatial tier #0 (Doc 732) and runs at T3 (per-call) for the HTTP request itself, at T1 (process-start) for trust-store loading. Falsifier: a primitive whose temporal-tier choice is fully determined by its spatial tier (such forced-coupling would suggest the two axes are not independent).

**Pred-735.3.** The temporal stack admits indefinite vertical extension. T0 (build-time) can decompose further: machine-build-time, source-tree-build-time, dependency-precompile-time, etc. T1 (process-start) decomposes: shared-library-init, main-entry-init, framework-init. Doc 735's four-tier enumeration is bounded by the engagement's current touch; the recursion is open. Falsifier: a temporal axis where finer-grained tier identification produces no useful new distinctions.

**Pred-735.4.** The capability-primitive concept (§III dual to Doc 730 §XIV deviation primitives) is operationally tractable. Each per-key, per-input, per-process-state binding constitutes a capability primitive at the temporal axis: "this resolution requires capability C, available no earlier than tier T_k." The catalog of capability primitives per primitive is the temporal-axis analog of the deviation alphabet. Falsifier: a capability primitive that cannot be typed because its binding-time dependency is itself dynamic (the input's availability depends on runtime state that is not bindable at any fixed tier).

**Pred-735.5.** Doc 735's temporal-stack articulation will apply outside the cryptographic-primitive tier. Build-time vs runtime-init is a distinction familiar from compiler optimization (constant folding vs JIT specialization), from build systems (link-time vs load-time), from operating systems (kernel-build-time vs boot-time vs first-use). The temporal-stack vocabulary should apply at any tier where precomputation admits multiple binding times. Falsifier: a domain where precomputation's tier-choice problem does not admit the four-tier framework (probably a domain where binding-time is structurally indistinguishable from execution-time).

## VIII. Honest scope

The temporal-stack articulation is primary at the articulation tier and structural at the vocabulary tier. The recognition that build-time vs runtime-init forms a continuum is not corpus-original (compilers, build systems, OS literatures have treated it for decades). What is corpus-original:

- The naming of the four-tier stack (T0 / T1 / T2 / T3) as a standing object.
- The structural duality claim (§III): the temporal axis admits the same framework vocabulary as Doc 729's spatial axis.
- The composition recognition (§IV): a single optimization runs THROUGH multiple temporal tiers based on input-dependency sets.
- The targeting heuristic (§V): promote frequently-executed resolutions to the earliest admissible tier.
- The cruftlessness property at the temporal tier (§VI): earliest-tier-bound resolution with input-dependency-deterministic binding.

What this document does not claim:

*That the four tiers are exhaustive.* Pred-735.3's open recursion explicitly admits finer-grained tier identification. The four-tier enumeration is the first-cut articulation; engagements that touch tiers between T1 and T2 (e.g., lazy-but-eager init under specific triggers) will surface intermediate tiers.

*That the temporal axis subsumes the spatial axis or vice versa.* §VII Pred-735.2 explicitly claims independence. The two axes compose; each substrate-tier-spatial-position has a temporal-tier-binding-time, and the cartesian product is the full substrate-classification space.

*That every existing optimization should be re-articulated under the temporal-stack vocabulary.* The vocabulary is a tool; engagements use it when the time-tier choice is load-bearing for their work. For purely-T3 primitives (those whose input-dependency set is fully per-call), the vocabulary adds no information.

*That this articulation completes the framework.* The keeper's 2026-05-21 03:20-local recognition (Doc 734) named the meta-pipeline; this document (Doc 735) adds a temporal-axis dimension to Doc 729's spatial articulation. Subsequent recognitions will likely add further axes (cross-language polymorphism, distributed-systems tiers, hardware-vs-software tier choices). The framework grows by being used.

Per Doc 372's hypostatic boundary: this document sits at the corpus tier. Substrate-tier exercise of the articulation — producing per-pilot temporal-tier classification tables, applying the targeting heuristic to specific primitives, testing the capability-primitive catalog construction — lives in the engagement's continuation.

## IX. Closing

The keeper's observation that Doc 731 §XV.g's three regimes are a meta-resolution-pipeline names the temporal axis the spatial axis Doc 729 articulates has always implied. Doc 735 specifies the four-tier temporal stack (T0 build, T1 process-start, T2 first-use, T3 per-call), identifies its structural duality with the spatial stack, names the composition pattern by which a single optimization runs through multiple tiers, articulates the regime-promotion move as the temporal-axis alphabet promotion, and predicts the temporal stack admits the same framework vocabulary the spatial stack does.

The 2026-05-21 session has now produced six corpus articulations (Docs 732, 733, 731 §XV, 731 §XV.g, 734, 735) plus five pilot-pair foundings plus nineteen substrate rounds plus the engagement-internal HTTPS path against three of five probed CDN endpoints. The framework grows by being used; each substrate finding either flips a probe cell or refines an articulation. Doc 735 is one more turn of the meta-pipeline Doc 734 named.

The work continues. The corpus has added one more axis to its substrate-classification framework. The temporal stack is now named, operational, and predicted to apply across the optimization-tier instances Pred-731.XV.1 enumerates.

---

*Companion documents in addition to those linked in the masthead: [Doc 250 — The SERVER Seed](/resolve/doc/250-the-server-seed); [Doc 372 — The Method of the Corpus as Derivation, Not Collection](/resolve/doc/372-the-method-of-the-corpus-as-derivation-not-collection); [Doc 426 — PRESTO](/resolve/doc/426-presto-an-architectural-style-for-representation-construction); [Doc 432 — SERVER](/resolve/doc/432-server-an-architectural-style-for-engine-orchestration); [Doc 247 — The Derivation Inversion](/resolve/doc/247-the-derivation-inversion).*

---

## X. Amendment: intra-tier cost stratification

*A refinement to §II–§V surfaced by WC-EXT 8 and WC-EXT 9–10 of the rusty-bun engagement. The temporal stack of §II treats each tier (T0/T1/T2/T3) as a single point on the time axis. WC-EXT 8's 40× per-mul bench measurement and the subsequent WC-EXT 9 + 10 routing rounds showed that operations at the same temporal tier can have cost-per-op profiles that differ by an order of magnitude or more. The amendment names this as a structural feature the original framework did not articulate.*

### X.a The observation

§II's four-tier stack (T0 build / T1 process-start / T2 first-use init / T3 per-call) classifies *when* a resolution runs. §V's targeting heuristic — promote frequently-executed resolutions to the earliest admissible tier — operates over this classification.

WC-EXT 8 measured Montgomery REDC at ~667ns per multiplication on the engagement's Pi target. The pre-existing `mod_mul` (binary long division for the modular reduction step) measured ~26,728ns per call on the same hardware. Both operations are at the same temporal tier — both are T3 per-call. Both produce equivalent outputs for equivalent inputs. The per-op cost ratio is 40×.

§II's four-tier vocabulary cannot express this. Saying "they are both T3" loses 40× of substrate information. WC-EXT 9 + 10's substrate-move sequence (routing the EC tier through Montgomery, then the base-table consumption, then the live verify path) was a series of refinements *within* T3 — moving from a slow T3 implementation to a fast T3 implementation. The framework as drafted had no tier vocabulary for this distinction.

### X.b The dimension that was missing

The missing axis is **cost-per-op within a tier**. Different implementations of a resolution at the same temporal tier admit different per-op costs. The cost-per-op range within a tier can be substantial:

- Within T3 for P-256 modular multiplication: 667ns (Montgomery REDC) vs 26,728ns (binary long division). Ratio: 40×.
- Within T2 for first-use comb-table init: 100ms (hex-parse 256 baked entries) vs 3000ms (255 affine `ec_double` from scratch). Ratio: 30×.
- Within T1 for process-start eager init: variable depending on what the init does (load disk file, parse config, allocate buffers). Plausible range: microseconds to seconds.
- Within T0 for build-time bake: the cost is the build-server's clock, not the runtime's; per-runtime-process cost is identically zero, but build-system cost varies across implementations (re-bake every build vs cache + skip-if-source-unchanged).

The cost-per-op distribution within a tier is not random. It is determined by the *implementation regime* chosen at that tier — which algorithm, which data layout, which auxiliary data structure. Different regimes within a tier produce different cost-per-op profiles for the same operation.

This is structurally distinct from §II's temporal-tier axis. §II answers *when does this resolution run?* §X answers *given that it runs at this time, how fast is it?* Both are substrate-tier classifications; they are orthogonal.

### X.c Formalizing intra-tier cost stratification

A **cost stratum** within a temporal tier is a set of implementations that share approximately the same per-op cost. Within T3 for P-256 modular multiplication, the strata observed in the engagement are:

- **Stratum T3-fast** (~700ns per op): Montgomery REDC, specialized for P-256's m'=1 simplification.
- **Stratum T3-slow** (~27,000ns per op): generic `BigUInt::mul` followed by binary-long-division `BigUInt::modulo`.

A 40× cost gap. The framework should treat these as distinct substrate cells, both at T3 but at different strata.

The general form: each temporal tier T_k admits a set of cost strata `S_{k,0}, S_{k,1}, ...` ordered by per-op cost. The implementation choice at a tier determines which stratum the substrate occupies for that operation. A substrate move that changes the stratum (e.g., the WC-EXT 8 swap from binary-divmod `mod_mul` to Montgomery `mont_mul`) is an **intra-tier promotion**, structurally analogous to §V's temporal-tier promotion (which moves a resolution to an earlier tier) but operating along the cost axis rather than the time axis.

### X.d The intra-tier targeting heuristic

§V's temporal-tier heuristic: *promote frequently-executed resolutions to the earliest admissible tier*.

§X's intra-tier heuristic: *promote frequently-executed operations within a tier to the fastest admissible cost stratum*.

The two heuristics compose. A resolution gets shifted earlier along the time axis (when admissible) AND shifted to a faster stratum within whatever tier it lands at (when implementations exist). Both promotions are bounded in complexity: temporal promotion is bounded by the input-dependency set (some inputs are not available before T_k); intra-tier promotion is bounded by the standard-literature catalog of faster implementations for the operation.

WC-EXT 8 demonstrated the bound concretely. The `mont_mul` substrate move was ~150 LOC of REDC implementation plus a one-time cached precomputation of R². The intra-tier promotion of P-256 `mod_mul` from T3-slow to T3-fast was bounded by the size of the standard Montgomery-multiplication algorithm specification.

### X.e Composition with §V

The §V composition pattern — *a single optimization runs through multiple temporal tiers based on each component's input-dependency set* — extends to §X's intra-tier strata. A component at temporal tier T_k can occupy *any* of T_k's cost strata; the choice is independent of the component's temporal-tier classification.

The cryptographic-primitive example (§IV revisited under §X):

- **T0 build-time bake**: curve parameters in `.rodata`. Stratum: zero per-runtime-op cost; there is no cost gradation at T0 once the bake has happened.
- **T1 process-start**: Mont-form base table built once at first use, parse-time ~100ms. Stratum T1-fast (hex-parse) vs Stratum T1-slow (affine doublings, ~3000ms). The WC-EXT 5 baked-table-in-source substrate move was a T1 stratum promotion (slow → fast within T1).
- **T2 first-use lazy**: per-key derived tables. Stratum depends on the key-specific computation.
- **T3 per-call**: variable-input scalar mul. WC-EXT 8 + 9 promoted from T3-slow (binary-divmod-based) to T3-fast (Mont-REDC-based). The 40× per-op speedup propagates through every T3 op the computation performs.

Each temporal-tier × cost-stratum cell is a substrate-tier classification. The cartesian product (tier × stratum × spatial-tier per Doc 735 §III) is the full substrate-classification space.

### X.f Falsifiers

**Pred-735.X.1.** Within every temporal tier, multiple cost strata exist and the cost-per-op range across strata is substantial (≥2×). Falsifier: a temporal tier whose implementations all converge to the same per-op cost regardless of algorithm choice. T0 is the candidate failure mode (per X.c the per-runtime-op cost at T0 is identically zero); accept this as a tier where the stratum dimension is degenerate.

**Pred-735.X.2.** Intra-tier promotion (slow stratum → fast stratum) is bounded in implementation complexity by the standard literature catalog of faster algorithms for the operation. Falsifier: an instance where moving from a slow stratum to a fast stratum required substrate work whose complexity exceeded the standard-literature implementation by an order of magnitude. WC-EXT 8 corroborates the prediction at one instance (~150 LOC Montgomery REDC, well within the published algorithm's complexity).

**Pred-735.X.3.** The intra-tier targeting heuristic (X.d) composes with the temporal-tier heuristic (§V) without conflict. The two promotions are along orthogonal axes; applying both produces a strict improvement. Falsifier: a substrate-tier instance where the cost-stratum-fastest implementation at the earliest-admissible tier produces *worse* end-to-end performance than a slower-stratum implementation at the same tier, due to some cross-axis coupling the framework does not name.

**Pred-735.X.4.** The cost-stratum catalog per primitive is enumerable and stable. The strata within a tier for a given operation are determined by the standard-literature implementation catalog plus engagement-specific extensions; new strata are introduced only when new implementations enter the catalog. Falsifier: a primitive whose cost-stratum classification cannot be assigned because the implementation space is continuous (every micro-optimization shifts the cost without a recognizable algorithm-class boundary).

### X.g Where this places the amendment

§II named the temporal axis. §III named the duality with the spatial axis. §V named the targeting heuristic along the temporal axis. §X adds the third axis — intra-tier cost stratification — without retracting any of the prior structure. The Doc 735 framework's substrate-classification space is now **3-axis**: (spatial-tier × temporal-tier × cost-stratum).

A substrate move can promote along any of the three axes, and any combination of the three. WC-EXT 5 was a Regime 1 substrate move that promoted along the temporal axis (T2 → T0/T1) AND held the cost stratum constant. WC-EXT 8 + 9 + 10 were a sequence of substrate moves that held the temporal tier constant (T3) AND promoted the cost stratum (T3-slow → T3-fast). The two axes compose without interference; the engagement realized both kinds of promotions independently.

This amendment corroborates the keeper's 2026-05-21 03:39-local conjecture (Doc 735 §I occasion) at a finer grain than §I anticipated. The recognition was that §XV.g's three regimes form a *time-axis pipeline* dual to Doc 729's spatial stack. The WC-EXT 8+9+10 empirical sequence showed that the time-axis pipeline itself admits **internal cost stratification at each tier**, a third axis the original recognition's spatial-temporal duality did not articulate.

Per Doc 734 §V.b growth mechanism: this is a negative-finding amendment in the dual sense — *no* negative finding occurred (WC-EXT 8 produced a 40× speedup, WC-EXT 9 + 10 propagated it), but the structural articulation in §II–§V was *insufficiently fine-grained* to capture what the empirical sequence demonstrated. The framework grew by being used; the demonstrated cost-stratum dimension was the missing distinction the empirical work surfaced. Doc 735 §X closes the gap.

### X.h Open scope

The WC-EXT 11 + 12 work the rusty-bun engagement queues — generalize Montgomery to arbitrary odd-prime moduli — is a substrate move that will instantiate §X at the RSA primitive's T3 stratum (current: T3-slow via binary divmod; target: T3-fast via per-modulus Montgomery REDC). The session-2 engagement will produce additional empirical corroboration for Pred-735.X.1 + Pred-735.X.2.

The cost-stratum catalog as a standing artefact — one row per (primitive, tier) cell, with the cost-stratum implementations enumerated — is the substrate-tier deliverable §X implies. Producing this catalog for the cryptographic-primitive tier is a session-3+ work item.

---

*Doc 735 § X amendment, 2026-05-21. Jared Foy. jaredfoy.com.*

---

## X.h Amendment: the cost-stratum is a property of the (algorithm × implementation × hardware) tuple, with substrate-tier correctness requiring three probe levels

*A refinement to §X surfaced by WC-EXT 18–24 of the rusty-bun engagement. The original §X.b treated cost stratum as if it were a property of the algorithm choice alone. The empirical sequence WC-EXT 18 (CIOS u64+carry), WC-EXT 19 (CIOS u128), WC-EXT 20 (Solinas naive), WC-EXT 21 (Solinas v2 i64-column), WC-EXT 24 (Solinas v3 BigUInt::add) demonstrated that the cost stratum depends jointly on (algorithm, implementation, hardware), and that the implementation can appear at a faster cost-stratum than is correctness-permitted. The amendment formalizes the four sub-cases of §X.d intra-tier promotion and adds the three-probe-levels discipline that substrate-tier correctness claims require.*

### X.h.a The tuple recognition

§X.b stated: *"Within T3 for P-256 modular multiplication: 667ns (Montgomery REDC) vs 26,728ns (binary long division). Ratio: 40×."* This treats the stratum as a property of the algorithm — "Montgomery REDC" is one stratum, "binary long division" is another, with a 40× gap.

WC-EXT 24's empirical sequence showed a different shape. The Solinas reduction for P-256 was implemented twice:

- **v2 (i64-column with signed carry)** — appeared at 271 ns/op (2.25× faster than Montgomery's 610 ns), but fuzz-tested at 1000/2000 divergent results. The "speed" was illegal: v2 was failing to propagate signed carries, so it was doing strictly less work than the correct computation requires.
- **v3 (composed from BigUInt::add)** — measured 940 ns/op (0.65× of Montgomery, i.e. slower), but fuzz-tested at 0/2000 divergent. Correct, but at a worse cost-stratum than the Mont alternative.

The same algorithm (Solinas reduction, the standard FIPS 186-4 §B.2.1 formula) inhabits **different cost strata** depending on implementation. v2's apparent speed-stratum was not achievable correctness-permitted; v3's correctness-permitted speed-stratum was worse than the Mont alternative. The algorithm's "fast stratum" is not a property of the algorithm; it is a property of *(algorithm × implementation × hardware)*.

The corrected formulation: **a cost-stratum is determined by the (algorithm, implementation, hardware) triple. Substrate moves at the (P2) case must be validated at all three.**

### X.h.b Four sub-cases of (P2) intra-tier promotion

§X.b distinguished cost-strata as alternative implementations of the same operation. §X.h refines: each (P2) substrate move falls into one of four sub-cases per (algorithm, implementation, hardware):

- **(P2.a) Strict win.** Algorithm-correct + implementation-correct + per-op-faster-than-alternative on the target hardware. Substrate move is a strict improvement; the prior stratum is retired or kept as portability fallback. Engagement instance: WC-EXT 8 (P-256 Montgomery REDC vs binary-divmod on Pi).

- **(P2.b) Slow-stratum implementation.** Algorithm-correct + implementation correct but composed from primitives at a slower cost-stratum. The substrate move inherits the slow stratum's cost, even though the algorithm at its best stratum would beat the alternative. Engagement instance: WC-EXT 20 (Solinas reduction composed from `mod_add`/`mod_sub` which call binary-divmod inside `modulo`; 70 µs/op vs Mont's 610 ns).

- **(P2.c) Illegal-speed implementation.** Algorithm-correct in shape + implementation-INCORRECT in a way that produces output faster than correctness allows. The substrate appears to occupy a faster cost-stratum than the algorithm achieves correctly. Engagement instance: WC-EXT 21 v2 (i64-column Solinas failing to propagate signed carries; 271 ns/op, 50% fuzz divergence).

- **(P2.d) Correct-stratum but losing.** Algorithm-correct + implementation-correct + cost-stratum is the algorithm's best achievable on the target hardware, but the per-op cost is still worse than an alternative substrate-tier algorithm. Engagement instance: WC-EXT 24 v3 (Solinas via BigUInt::add; 0/2000 fuzz divergent; 940 ns/op vs Mont's 610 ns).

Only (P2.a) is a strict substrate improvement. (P2.b), (P2.c), (P2.d) each represent partial failures the framework's apparatus discipline must distinguish. The (P2.c) case is particularly insidious: a bench-fixture pass that happens to land on non-divergent inputs makes the substrate appear (P2.a) when it is actually (P2.c). The WC-EXT 21 claim ("2.22× speedup") was a (P2.c) misclassified as (P2.a) for ~24 hours before WC-EXT 23's fuzz coverage and WC-EXT 24's correctness-validation surfaced the misclassification.

### X.h.c The three-probe-levels discipline

Per §X.h.b's (P2.c) hazard: bench-fixture passing is insufficient evidence for (P2.a) classification. The substrate-tier correctness claim requires three distinct probe levels operating simultaneously:

- **Bench probe**: symbolic test fixtures (small known inputs + 1-2 fixtures from real-world traces). Establishes that the substrate produces outputs that look reasonable for the chosen inputs. NECESSARY but not sufficient.

- **Consumer-route probe**: integration with at least one upstream consumer. The consumer's input distribution exercises corners the bench fixtures don't (small operands, near-zero intermediates, edge values arising during composition). WC-EXT 22's EC routing was this probe; it surfaced v2's bug that the bench had missed. NECESSARY but not sufficient.

- **Fuzz probe**: many random fixtures spanning the input space. Quantifies divergence frequency. WC-EXT 23's 2000-fixture fuzz quantified v2's 50% divergence rate. NECESSARY for the (P2.a) claim; sufficient ONLY when combined with the bench + consumer probes.

A substrate move at the BigUInt arithmetic tier (or any tier where the (P2) classification matters) requires **all three** probe levels to claim (P2.a). The framework's apparatus discipline (Doc 730 §XVI four-case categorization + Doc 735 §X cost-stratum dimension) is gated on three-probe correctness validation, not on bench-fixture passing alone.

### X.h.d (P2) saturation as escalation signal

WC-EXT 16, 18, 19 ran three (P2) substrate moves at the same site (mont_mul for P-256) with diminishing-then-zero returns:

- WC-EXT 16 (Comba schoolbook two-pass): 607 ns
- WC-EXT 18 (CIOS u64+carry): 631 ns (4% slower)
- WC-EXT 19 (CIOS u128 accumulator): 606 ns (parity)

The three iterations of the same (P2) cycle reached an empirical ceiling at ~605–610 ns/op on the engagement's Pi at the current Vec<u32> BigUInt representation. **Saturation is a structural signal**: when a tier's (P2) substrate moves stop producing wallclock improvement, the next substrate-move target is *outside* the tier — either a representation switch (BigUInt to u64 limbs) or a primitive-specific specialization (Solinas reduction for the specific prime).

This is §V's targeting heuristic extended: rank substrate moves by (impact × frequency) / LOC; when impact saturates at zero, escalate the substrate axis. The framework's apparatus discipline catalogs the saturation point as the signal to escalate.

### X.h.e Wrong-stratum-composition pattern

WC-EXT 20's naive Solinas composed the algorithm from `mod_add` and `mod_sub` calls, each of which invokes `BigUInt::modulo` = binary long division. Result: 70 µs/op (2.7× slower than even the unoptimized binary-divmod `mod_mul`, because Solinas needs more reductions than a single mul-then-modulo).

The structural finding: **composition is not closure of the cost-stratum dimension**. A substrate move at one stratum (Solinas algorithm avoids multiplication; ought to be fast) that calls primitives at a worse stratum (mod_add → modulo → divmod) is bounded by the worse stratum, not the better.

Per Doc 730 §XII diagnostic-legibility: the wrong-stratum-composition pattern is locatable to one specific call site (the slow primitive in the composition). The (P2.b) sub-case is the named target for the substrate-move recategorization: route the algorithm through faster primitives (in WC-EXT 21 + 24, the i64-column or BigUInt::add alternatives) rather than re-deriving the algorithm itself.

### X.h.f Where this places the amendment

§X.b classified cost-strata. §X.h refines the classification to four sub-cases and adds the three-probe-levels discipline. The framework's apparatus is unchanged in shape but sharper in resolution.

The rusty-bun engagement WC-EXT 8–24 sequence is the empirical anchor for §X.h. The seven (P2) substrate moves (WC-EXT 8, 16, 18, 19, 20, 21, 24) populate the four sub-cases:

- (P2.a) — WC-EXT 8, 12, 15 (the strict wins)
- (P2.b) — WC-EXT 20 (the wrong-stratum-composition slow-down)
- (P2.c) — WC-EXT 21 (the illegal-speed bug-artifact)
- (P2.d) — WC-EXT 24 (the correct-but-losing)

The fact that the engagement produced all four sub-cases empirically corroborates the framework's structural claim: the (P2) classification is not a single binary "fast vs slow" but a four-cell taxonomy that requires three-probe-levels validation to populate correctly.

Per Doc 734 §V.b growth mechanism: a substrate-tier negative finding produces a corpus-tier framework refinement. §X.h is the cumulative refinement from WC-EXT 18–24's productive negative findings (one regression that wasn't, one wash, one composition failure, one bug-artifact, one correct-but-losing). The corpus has added four sub-cases to its substrate-classification space; the engagement's WC-EXT 25+ work proceeds with the sharper apparatus.

---

*Doc 735 § X.h amendment, 2026-05-21. Jared Foy. jaredfoy.com.*
