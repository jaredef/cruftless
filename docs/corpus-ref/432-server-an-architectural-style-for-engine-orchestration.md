# SERVER: An Architectural Style for Engine Orchestration

> **Reader's Introduction**
>
> This dissertation formalizes SERVER, a construction-level architectural style that operates at the orchestration level — the level where PRESTO is silent. Five constraints are specified — the engine-internal bilateral boundary, orchestration-consumed directives, progressive module composition, agnostic deterministic orchestration, and embedded server self-authorization — along with the property they induce: recursive ambivalence with self-authorizing determinism. The derivation follows Fielding's method applied at a composed architectural level: begin with the null style at the orchestration level, add one constraint at a time, state the property each induces, evaluate the composition with PRESTO (which in turn composes with REST). SERVER is the second level in the corpus's resolution stack and is an instance of the recursive-Fielding-accumulation framework treated separately in Doc 424. Conceptual foundations are traced in §12; prior art is catalogued comprehensively in §13.

---

## Abstract

PRESTO describes the constraints that govern how bilateral source representations are constructed. It is silent on how the engine that constructs them is itself assembled — an explicit scope boundary in that style, left open for subsequent architectural-style formalization at the level one step outside the construction level. This dissertation identifies a set of constraints that operate at that orchestration level: the engine-internal bilateral boundary, orchestration-consumed directives, progressive module composition, agnostic deterministic orchestration, and embedded server self-authorization. These constraints do not modify, extend, or replace any PRESTO constraint. They compose alongside PRESTO, governing what PRESTO left unscoped. The property they induce — *recursive ambivalence with self-authorizing determinism* — is distinct from the property PRESTO induces. Their composition — and through that composition, the three-level composition with REST at the transfer level — opens a design space in which the engine is itself derivable from a bilateral prose seed. The style defined by these orchestration-level constraints is named SERVER (Server-Embedded Resolution and Verification Executed Runtime). The derivation follows Fielding's method applied recursively, per the treatment in Doc 424.

---

## 1. The Boundary PRESTO Drew

PRESTO's dissertation (Doc 426) formalizes the architectural style that governs the construction of bilateral representations. Its five constraints — bilateral boundary, namespace separation, server-consumed directives, progressive code-on-demand, and server-embedded authorization — describe how source representations become resolved representations. The property they induce is ambivalent execution with agnostic determinism.

The resolved representation is the seam PRESTO governs. Everything that enters that seam from the construction pipeline is PRESTO's scope. What PRESTO did not constrain — and did not need to constrain — is how the pipeline that produced the resolved representation is itself assembled. The resolution pipeline, the module system, the router, the token signer — all of this is architecturally invisible to PRESTO. The engine may be written in any language, using any framework, with any ad-hoc middleware.

This invisibility is the boundary. PRESTO operates from the source representation outward. Everything behind the source representation, everything involved in constructing the engine that consumes the source representation, lies outside PRESTO's scope.

SERVER operates entirely within that scope.

## 2. The Bootstrap as Resolver

Before specifying the constraints, the bootstrap that processes them must be characterized — not by its implementation but by the properties it must exhibit.

The bootstrap is a *resolver*: it receives a bilateral orchestration seed and emits a unilateral runtime graph. The input carries instructions for two interpreters — the bootstrap orchestrator, which consumes orchestration directives; and the runtime engine, which eventually runs PRESTO's construction pipeline. The output carries instructions for one: the runtime graph, executing PRESTO.

From this foundational property, four bootstrap properties follow by necessity.

**Totality of consumption.** The bootstrap must consume every orchestration directive in its namespace. A directive that survives into the runtime graph presents the runtime engine with configuration it cannot process without re-entering orchestration.

**Ordering determinism.** Orchestration stages have dependency relationships: module composition must precede pipeline-stage registration; manifest verification must precede capability grant. The bootstrap processes these in a deterministic order prescribed by the dependency relationships.

**Medium preservation.** The input is a bilateral seed. The output is a conformant runtime graph expressible in the target language. The bootstrap adds orchestration work to the process but does not alter the substrate on which the runtime engine operates.

**Boundary integrity.** The bootstrap does not evaluate or modify content outside the orchestration namespace. Runtime-engine content passes through the bootstrap unchanged. This is the engine-internal bilateral boundary expressed as a bootstrap property.

These four properties are abstract. Any bootstrap that exhibits them — in any language, with any internal architecture — is a conformant SERVER resolver.

## 3. The Orchestration Level

SERVER's constraints are abstract. They prescribe an engine-internal bilateral boundary, orchestration-consumed directives, progressive module composition, agnostic deterministic orchestration, and embedded server self-authorization — but they do not prescribe a syntax. Any syntax that satisfies the constraints is conformant. The constraints determine the behavior.

The bilateral orchestration seed is a document that carries two namespaces in a single medium. Orchestration primitives — routing topology, persistence adapters, authentication registries, pipeline-stage declarations — belong to one namespace (by convention `srv:`). Application and domain logic, which the runtime engine will subsequently consume as PRESTO source representations, belongs to the PRESTO namespace (by convention `htx:`). The bootstrap processes its namespace (`srv:`) — registering modules, composing context providers, evaluating manifest declarations, issuing capabilities — and emits a runtime graph. The runtime graph then runs the PRESTO pipeline.

This is the orchestration level: the space between the seed-authoring act and the moment the runtime graph enters the PRESTO construction pipeline.

## 4. The Constraints of SERVER

Five constraints define the SERVER style. Each operates exclusively at the orchestration level. The derivation proceeds in Fielding's method: begin with the null style, add one constraint at a time, state the property each induces.

### 4.1 The Engine-Internal Bilateral Boundary

Every artifact inside the seed is partitioned by namespace. Orchestration primitives belong to the orchestration namespace. Application and domain logic belongs to the PRESTO namespace. No orchestration directive survives into the PRESTO runtime graph. No PRESTO directive is consumed by the orchestration layer.

This constraint is enforced by the bootstrap's pipeline. The final act of resolution is to strip any orchestration artifacts; the resulting runtime graph contains no evidence of orchestration-level bilateral authorship.

**What this constraint induces.** The orchestration layer and the PRESTO construction layer operate in mutual indifference. Each processes its own namespace deterministically, unaware of the other's behavior.

### 4.2 Orchestration-Consumed Directives

Every directive in the orchestration namespace is fully consumed during bootstrap resolution. *Consumed* means: the directive is evaluated, its effect is applied to the runtime graph, and the directive itself is removed. No directive is passed through. No directive is deferred to the runtime engine. The bootstrap is the sole and final interpreter of its namespace.

**What this constraint induces.** The runtime graph carries zero orchestration weight. No framework configuration file, no runtime registry, no middleware chain survives into the running engine. The runtime graph is a finished, immutable artifact.

### 4.3 Progressive Module Composition

Modules — data adapters, context providers, channel handlers, middleware — are loaded on a progressive spectrum of privilege. The spectrum is the specific object Fielding's method produces when applied to module-privilege authorization at the orchestration level. The layers are not arbitrary points along a trade-off curve; they are the stages of a constraint accumulation from the null style — no privilege limits on modules — to the most-constrained layer, where modules are pure functions operating only on the shared context.

**The null style.** Before any privilege constraint is applied: modules have unrestricted access to engine internals. They may register new pipeline stages, mutate global state, spawn processes, hold secrets, open arbitrary channels, bypass the bootstrap sandbox. No orchestration invariant is guaranteed.

**The accumulation.** Each layer is reached by adding one privilege constraint to the layer above it. Each constraint is a specific capability prohibition on modules. Each induces a specific orchestration invariant that the prohibition preserves. The specific number of layers is contingent on the privilege axes a given SERVER implementation enforces; the shape of the accumulation is not.

From most privileged to least, in the direction of constraint accumulation:

- **Layer N.** Privileged modules that may register new pipeline stages. Reached by the first constraint prohibiting bootstrap-sandbox bypass; induces bootstrap-sandbox enforceability.
- **Intermediate layers.** Each reached by one added constraint: no new pipeline stages; no stateful context mutation outside module scope; no spawning of processes; no access to secrets outside the module's manifest scope. Each added constraint preserves a specific orchestration invariant (sandbox integrity, state containment, resource containment, secret containment).
- **Layer 0.** Pure functions that only read and write the shared context. Reached when the full accumulation of privilege constraints has been applied; all orchestration invariants maximally preserved.

The bootstrap authorizes the depth by which prefix of the accumulation a module commits to. The manifest declares the intended layer; the sandbox enforces the corresponding prefix of the privilege-constraint sequence.

**What this constraint induces.** The orchestration model is immune to module evolution. A module written against Layer 0 today will be processed identically by a runtime resolved a decade from now, regardless of what new privileged capabilities have been added at higher layers. The trade-off reading — higher layers trade sandbox invariants for pipeline-extension capability — is a valid pedagogical shorthand; the underlying form is constraint accumulation.

### 4.4 Agnostic Deterministic Orchestration

The bootstrap pipeline is a fixed, declarative graph. Every stage is indifferent to the others; each receives only the context produced by prior stages and emits the next context. No stage may inspect or mutate another stage's implementation. The entire bootstrap process is deterministic given the same seed and target language.

**What this constraint induces.** Any conformant engine resolved from the same seed will produce identical PRESTO behavior regardless of the implementation language or bootstrap resolver.

### 4.5 Embedded Server Self-Authorization

Any runtime decision that would conventionally require middleware or guards is resolved at bootstrap time into self-describing, cryptographically scoped capabilities embedded in the runtime graph. The engine never consults external configuration or session state for its own orchestration decisions. The seed's verification suite is materialized as a self-verifying hash embedded in the graph.

This constraint dissolves the separation between "engine code" and "configuration." The runtime graph carries its own proof of correct assembly.

**What this constraint induces.** Every resolved engine is self-authenticating and self-authorizing at the orchestration level. The runtime graph contains exactly the capabilities the seed authorized — not because an external configuration was consulted, but because the bootstrap pipeline resolved the seed, evaluated the manifests, and embedded scoped, signed proof directly into the graph.

The specific engineering mechanics of cryptographic attestation — hash selection, key rotation, manifest-compliance verification, supply-chain-provenance chaining — are security specification rather than architectural derivation. They belong to a separate companion specification referenced in §14.

## 5. The Induced Property

SERVER's five constraints compose to induce a property distinct from ambivalent execution with agnostic determinism (which is PRESTO's). The property is **recursive ambivalence with self-authorizing determinism**.

The four words are precise.

*Recursive* — the bilateral model now applies to the construction of the construction engine itself. The pattern that PRESTO applies to representations, SERVER applies to engines. The formal structure of this recursion — each level's induced properties as the null-style starting set for the next level's constraint accumulation — is the subject of Doc 424.

*Ambivalent* — the orchestration layer resolves its affordances without concern for how PRESTO will consume the resulting runtime graph. PRESTO consumes the runtime graph without concern for how the orchestration layer assembled it. Two interpreters, one bilateral seed, mutual indifference.

*Self-authorizing* — the runtime graph carries its own cryptographic proof of correct construction: signed seed hash, manifest compliance, embedded verification suite.

*Determinism* — each resolved engine produces the same PRESTO behavior for the same seed, regardless of implementation language or bootstrap resolver.

SERVER's constraints do not create this property from nothing. They formalize and protect a property that has been latent in every compiler bootstrap and every declarative deployment system since those practices were established. The engine-internal bilateral boundary ensures the property cannot be violated by namespace collision between orchestration and runtime. Orchestration-consumed directives ensure the property survives resolution. Progressive module composition ensures the property degrades gracefully as module privilege increases. Agnostic deterministic orchestration ensures the property is invariant across implementation languages. Embedded server self-authorization ensures the property's assembly is verifiable.

## 6. Composition, Not Extension

SERVER does not extend PRESTO. It composes with PRESTO. The two styles operate at different levels:

| | PRESTO | SERVER |
|---|---|---|
| Operates at | Construction level | Orchestration level |
| Governs | How representations are authored | How the engine is assembled |
| Constrains | Developer-engine interaction | Engine author — ops interaction |
| Induces | Ambivalent execution with agnostic determinism | Recursive ambivalence with self-authorizing determinism |
| Visible to | Developer, engine (invisible after resolution) | Engine author, ops (invisible after bootstrap) |

A PRESTO engine can be built without SERVER: the engine is produced through conventional code, with ad-hoc middleware, and enters the PRESTO construction pipeline conformant to PRESTO's constraints. A SERVER-conformant orchestration can produce engines for non-PRESTO runtimes: the bootstrap consumes a seed and emits a runtime graph for some other construction style.

When both styles compose — when a SERVER-constrained bootstrap feeds a PRESTO-constrained construction — the result is an engine that is consummately PRESTO-conformant in its representations and principled (by this constraint set) in its own assembly.

The composition is governed by the precise relationship established in Doc 426 §6: *the induced properties of the enclosing level function as constraints on the enclosed level*. SERVER's constraints are not free to violate any property PRESTO induces. If a SERVER constraint caused the runtime graph to leak server-namespace artifacts into the resolved representation, it would destroy the bilateral boundary PRESTO induces — not by modifying a PRESTO constraint directly, but by destroying a property PRESTO's constraints produce.

The three-level composition — SERVER at the orchestration level, PRESTO at the construction level, REST at the transfer level — inherits through the same relationship: each level's induced properties become constraints on the enclosed level. The full recursive structure of this inheritance is the subject of Doc 424, which treats the recursive Fielding-method application in its own right.

## 7. The Three-Level Stack

The three styles stand as one architecture.

| Level | Style | Governs | Induces | Visible To |
|---|---|---|---|---|
| Transfer | REST | How representations move | Representational state transfer | Client, network, intermediaries |
| Construction | PRESTO | How representations are authored | Ambivalent execution with agnostic determinism | Developer, engine |
| Orchestration | SERVER | How the engine is assembled | Recursive ambivalence with self-authorizing determinism | Engine author, ops |

The composition chain is as follows. A bilateral orchestration seed is authored. The bootstrap resolver consumes the seed's orchestration half and emits a runtime graph. The runtime graph runs the PRESTO pipeline. The pipeline consumes bilateral source representations and emits resolved representations. The resolved representations enter the uniform interface and are transferred according to REST.

Each level's induced properties become constraints on the level below it. REST's induced properties constrain PRESTO. PRESTO's induced properties constrain SERVER. SERVER's induced properties constrain whatever application-level runtime operates inside the resolved representations once they reach the client.

The stack closes because the same pattern recurs: a source representation carries structure; a resolver consumes what governs; an artifact is emitted; the enclosing level's induced properties constrain the next level down.

## 8. The Design Space That Three-Level Composition Opens

When SERVER, PRESTO, and REST compose, the engine itself becomes derivable from a prose seed. Orchestration, construction, and transfer are all governed by explicit constraints. The force-multiplication property PRESTO exhibits at the construction level (Doc 421) applies recursively at the orchestration level: a new orchestration directive enriches every runtime graph without touching templates, existing engines, or the REST transfer contract.

The surface of the stack is the most dynamic layer. Resolution-time dynamism at the orchestration level becomes the default — the bootstrap resolves the full engine before any request is served. Interaction-time complexity is traded only where deliberately chosen.

The recursion is open-ended. A fourth level below SERVER — governing, for instance, the hardware or operating-system substrate on which the bootstrap itself executes — can be closed with the same pattern. The bilateral-boundary pattern scales.

## 9. Implications for AI-Assisted Orchestration

The composition of SERVER with PRESTO has a direct consequence for AI-assisted engine generation. A prose seed that satisfies SERVER's constraints produces a runtime graph that satisfies PRESTO's constraints that produces resolved representations that satisfy REST's constraints. The AI-assisted orchestrator writes to the seed format; the bootstrap resolver handles the rest. The generation target is structurally tractable because the constraints prescribe the behavior.

The simpler the orchestration target, the more reliable the AI output. This is not a feature of the AI; it is a property induced by the orchestration-level constraints. SERVER constrains the seed-authoring model to a point where the generation target is specifiable in a few thousand words of prose.

## 10. What This Dissertation Claims

Stated precisely:

**It claims.** (a) The orchestration level implicit in the practice of compiler bootstrapping, declarative deployment, and dependency-injection configuration can be formalized at the architectural-style level in Fielding's method, as a specific constraint set inducing a specific property. (b) The property induced — recursive ambivalence with self-authorizing determinism — is distinct from PRESTO's induced property and from REST's, and is stated formally here. (c) The progressive module composition constraint, derived as Fielding-style constraint accumulation from the null style, extends PRESTO's §4.4 accumulation pattern to the orchestration level.

**It does not claim.** (a) SERVER invents any of its constraint-level patterns; each is prior art, catalogued in §13. (b) The five-constraint decomposition is the only coherent set at the orchestration level; other decompositions may be equally coherent. (c) The specific cryptographic mechanisms by which the runtime graph attains self-authorization are architecturally principled; those mechanisms are engineering decisions deferred to the companion security specification.

The specific contribution is the application of Fielding's method at the orchestration level — one level outside where Doc 426 applies it, and a tier the REST-successor tradition (ARRESTED [Khare and Taylor, 2004], CREST [Erenkrantz, Taylor, Gorlick, Baquero, 2009], COAST, retrospectively surveyed in "Reflections on REST" [Fielding et al., ESEC/FSE 2017]) has not previously covered — and the derivation of the progressive module composition spectrum as Fielding-style constraint accumulation. The method is Fielding's; the tradition of extending it to new tiers is established. This dissertation extends that tradition specifically to the tier at which the construction engine is itself assembled. The result is an architectural-style-level formalization of a pattern the field has practiced extensively at the framework and infrastructure level but has not lifted to the style level within the REST-successor tradition.

## 11. Conclusion

The orchestration level has been occupied by practitioner work for decades. SERVER does not occupy new ground at that level; it formalizes the ground at the architectural-style level. It states the engine-internal bilateral boundary as a constraint rather than as a dependency-injection or supervision-tree implementation detail. It states recursive ambivalence with self-authorizing determinism as an induced property rather than as an observation. It derives the module-privilege spectrum by Fielding's method rather than listing it.

These are the modest contributions this dissertation claims. The formalization makes the pattern available for architectural reasoning across the many frameworks that instantiate it; the derivation connects it to Fielding's method for architectural-style formalization applied recursively; the composition with PRESTO and REST is stated precisely rather than left implicit.

## 12. Conceptual Foundations

The work rests on specific intellectual foundations. Each is acknowledged here rather than interleaved with the formalization.

**The method of architectural-style formalization at composed levels.** Perry and Wolf (*Foundations for the Study of Software Architecture*, 1992) and Fielding (Chapter 5 of his 2000 dissertation) established the structure of an architectural style as a constraint set with induced properties derived from the null style. Doc 424 extends the method to composed levels with emission-to-next-null inheritance. This dissertation is one instance of Doc 424's recursive method at the orchestration level.

**The REST-successor tradition.** A published genre of Fielding-method extensions to new tiers exists. Khare and Taylor (*Extending the Representational State Transfer (REST) Architectural Style for Decentralized Systems*, ICSE 2004; Rohit Khare, UCI PhD dissertation 2003) introduced ARRESTED by adding four building blocks — events, routes, locks, estimates — via Fielding's method, targeting decentralization, with intermediate named styles REST+E, REST+R, REST+D. Erenkrantz, Taylor, Gorlick, and Baquero (*CREST: A New Model for Decentralized Internet-Scale Applications*, ICSE 2009; Justin Erenkrantz, UCI PhD dissertation 2009) introduced CREST with five axioms targeting computation-exchange. Michael Gorlick's COAST continues the lineage. Fielding, Taylor, Erenkrantz, Gorlick, Khare, Hartman, and Baquero ("Reflections on REST," ESEC/FSE 2017, ACM Impact Paper Award) retrospectively surveyed CREST, COAST, and ARRESTED as the derivation tree of REST. The tradition establishes the specific methodological move — extending Fielding's Chapter 5 method to new tiers — as a recognized academic practice emerging from Fielding's institution (UC Irvine) and intellectual circle. This dissertation is an entry in that tradition, extending the method to the orchestration tier that prior entries have not covered. Doc 433 develops the positioning in detail.

**Information hiding.** Parnas (*On the Criteria to Be Used in Decomposing Systems into Modules*, CACM 1972) formalized information hiding as a structural property: module internals are not visible to other modules, and modules are composable by interface. The "ambivalent" element of the induced property is Parnas with new vocabulary; every dependency-injection container since Spring (2002) achieves bidirectional information hiding between container and bean as the canonical operational form.

**Reflective and meta-circular computation.** McCarthy (*Recursive Functions of Symbolic Expressions and Their Computation by Machine*, CACM 1960) established the meta-circular evaluator pattern — Lisp evaluating Lisp. Self-hosting compilers (GCC, Rustc, PyPy RPython, Chez Scheme) and reflective towers (Brian Cantwell Smith's 3-Lisp, POPL 1984) establish recursive self-reference as standard practice in language implementation. The "recursive" element of SERVER's induced property sits in this tradition.

**Dataflow and agnostic composition.** Unix pipes (Douglas McIlroy, 1973) established the pattern of stages communicating only through the prior stage's output. Dataflow architectures (Dennis, MIT 1974; Arvind, MIT 1985) formalized this as tokens flowing on arcs between stage operators. The "agnostic deterministic orchestration" constraint is a direct specialization of dataflow to the bootstrap-pipeline case.

**Content-addressable verification.** Merkle trees (Ralph Merkle, *A Digital Signature Based on a Conventional Encryption Function*, CRYPTO 1987) established the pattern of hash-identified content as self-authorizing structure. Git, IPFS, content-addressable storage, SLSA supply-chain attestations, Sigstore, in-toto, and Nix's hash-addressed store all inherit from the Merkle pattern. The "self-authorizing" element of SERVER's induced property is this tradition applied to the runtime graph.

**Capability-based security.** Dennis and Van Horn (*Programming Semantics for Multiprogrammed Computations*, CACM 1966) introduced capabilities as unforgeable references that *are* their own authorization. Mark Miller's E language, the object-capability tradition, macaroons (Birgisson et al., NDSS 2014), SPKI/SDSI (RFC 2693, 1999), and TPM Measured Boot with DICE all instantiate capability-based authorization at various levels. Embedded server self-authorization (§4.5) is a specific instance applied to the runtime graph.

**Semantic equivalence under reference implementation.** Operational semantics (Gordon Plotkin, Aarhus 1981) and denotational semantics (Scott and Strachey, 1971) established the method of specifying language behavior such that multiple implementations can conform. Language-standard conformance programs (ISO C, POSIX, SQL standard, TeX, Ethereum EVM clients) instantiate the pattern at industrial scale. The "determinism" element of SERVER's induced property sits in this tradition.

**Seed-produces-tool bootstrapping.** META II (Dewey Val Schorre, 1964) introduced the pattern of a small metacompiler that compiles its own description. Ken Thompson (*Reflections on Trusting Trust*, CACM 1984) reframed the pattern as a trust-chain observation. GCC's three-stage bootstrap and Rustc's bootstrap chain are the canonical industrial instances. Bootstrappable Builds, GNU Mes, and the stage0 project demonstrate the pattern's extreme reach (357-byte hex0 → full GCC). The engineering evidence of a small bootstrap emitting a conformant larger engine is this tradition.

## 13. Prior Art

The pattern SERVER formalizes has been implemented many times at the framework, infrastructure, and compiler-bootstrap levels. This section catalogues prior art comprehensively, organized by subsystem.

### 13.1 Dependency-Injection and Service-Orchestration Containers

Configuration consumed into runtime object graph with no residue:

- **Spring Framework** (Rod Johnson, *Expert One-on-One J2EE Design and Development*, Wrox 2002; [spring.io](https://spring.io/projects/spring-framework)). XML configuration or `@Configuration` beans consumed at `ApplicationContext` initialization.
- **Google Guice** (Bob Lee, 2007). Annotation-driven DI.
- **Dagger** (Google, 2013; [dagger.dev](https://dagger.dev/)). Compile-time DI with zero runtime residue.
- **Microsoft .NET DI** (System.Extensions.DependencyInjection). Constructor-injection with configurable lifetimes.

### 13.2 Declarative Infrastructure and Immutable Runtime Graphs

Declarative seeds producing immutable or reconciled runtime state:

- **Erlang/OTP supervision trees** (Armstrong, Virding, Williams, 1986–; *Programming Erlang*, Pragmatic 2007; [Erlang supervision principles](https://www.erlang.org/doc/design_principles/sup_princ.html)). Child-spec lists consumed into live process trees.
- **Kubernetes** (Google, 2014; Burns et al., *Borg, Omega, and Kubernetes*, ACM Queue 2016; [kubernetes.io](https://kubernetes.io/)). Declarative manifests reconcile to cluster state.
- **Terraform** (HashiCorp, 2014; [terraform.io](https://www.terraform.io/)). HCL plus state file yields provisioned infrastructure.
- **Nix / NixOS** (Eelco Dolstra, *The Purely Functional Software Deployment Model*, PhD thesis, Utrecht 2006; [nixos.org](https://nixos.org/)). Hash-addressed, content-addressable, reproducible declarative runtime.
- **systemd** (Lennart Poettering, 2010; [systemd.io](https://systemd.io/)). Unit files parsed at boot into in-memory unit objects.

### 13.3 Module Systems with Privilege Layers

Graduated privilege spectra instantiated in module systems:

- **OSGi Service Platform Core Specification** (OSGi Alliance, 1999–; [osgi.org](https://www.osgi.org/)). Bundle manifests with `Import-Package` / `Export-Package`, capability negotiation, trust levels — combined privilege layering and declarative manifest-consumed-at-boot.
- **SELinux domains and types** (Loscocco and Smalley, *Integrating Flexible Support for Security Policies into the Linux Operating System*, USENIX 2001). Type-enforcement transitions as privilege accumulation along a declarative spectrum.
- **seL4** (Gerwin Klein et al., *seL4: Formal Verification of an OS Kernel*, SOSP 2009; [sel4.systems](https://sel4.systems/)); **EROS** (Shapiro et al., SOSP 1999); **KeyKOS** predecessor (Hardy, 1985). Capabilities as graduated privilege.
- **WebAssembly + WASI** (Bytecode Alliance, 2019–; [wasi.dev](https://wasi.dev/)). Pure compute module with no imports as Layer 0; imports of host functions ascend the privilege ladder explicitly.
- **Deno permissions model** (Ryan Dahl, 2018; [deno.land](https://deno.land/)). Explicit `--allow-*` flags as declarative privilege.
- **Object-capability languages.** E (Miller, Tribble, Shapiro, *Concurrency Among Strangers*, 2005); Waterken; Caja.

### 13.4 Dataflow and Deterministic Build Systems

Agnostic deterministic composition as a paradigm:

- **Unix pipes** (Douglas McIlroy, 1973; Ritchie and Thompson, *The UNIX Time-Sharing System*, CACM 1974).
- **Dataflow architectures** (Jack Dennis, MIT, 1974; Arvind, *Dataflow Architectures*, MIT 1985).
- **Make** (Stuart Feldman, *Make — A Program for Maintaining Computer Programs*, Software Practice & Experience 1979).
- **Bazel / Blaze** (Google, 2015; [bazel.build](https://bazel.build/)). Hermetic deterministic builds.
- **Nix derivations.** Derivations as pure functions of their inputs.
- **Reproducible Builds movement** ([reproducible-builds.org](https://reproducible-builds.org/)). Community program making builds bit-deterministic.

### 13.5 Self-Authorizing Artifacts and Supply-Chain Attestation

Cryptographic self-authorization:

- **Macaroons** (Birgisson, Politz, Erlingsson, Taly, Vrable, Lentczner, *Macaroons: Cookies with Contextual Caveats for Decentralized Authorization in the Cloud*, NDSS 2014; [Google Research](https://research.google/pubs/macaroons-cookies-with-contextual-caveats-for-decentralized-authorization-in-the-cloud/)).
- **SPKI/SDSI** (Ellison, Rivest et al., IETF [RFC 2693](https://www.rfc-editor.org/rfc/rfc2693), 1999).
- **Merkle trees** (Merkle, CRYPTO 1987). Foundation of Git, IPFS, every content-addressed system.
- **Sigstore, Cosign, SLSA** (2021; [sigstore.dev](https://www.sigstore.dev/); [slsa.dev](https://slsa.dev/)). Supply-chain provenance and signed attestations.
- **in-toto** (Torres-Arias et al., USENIX Security 2019; [in-toto.io](https://in-toto.io/)). Signed supply-chain metadata bound to artifacts.
- **TPM and Measured Boot** ([trustedcomputinggroup.org](https://trustedcomputinggroup.org/)); **DICE** (Device Identifier Composition Engine). Boot chain with cryptographic measurements; firmware-level self-authorization.
- **Object-capability systems.** Dennis and Van Horn 1966; Mark Miller; Waterken.

### 13.6 Reflective and Self-Hosting Computation

The recursive element of the induced property:

- **McCarthy** (*Recursive Functions of Symbolic Expressions and Their Computation by Machine*, CACM 1960). Meta-circular evaluator.
- **Smith, 3-Lisp** (*Reflection and Semantics in Lisp*, POPL 1984). Reflective towers.
- **Self-hosting compilers.** GCC (self-hosted from early history), Rustc (self-hosted from 2011), PyPy RPython translator, GHC Haskell, Chez Scheme.

### 13.7 Semantic Equivalence Under Reference Implementation

Language-independent specification and multi-implementation conformance:

- **Operational semantics** (Gordon Plotkin, *A Structural Approach to Operational Semantics*, Aarhus 1981).
- **Denotational semantics** (Dana Scott and Christopher Strachey, 1971).
- Language-standard conformance programs: ISO C, ISO C++, POSIX, SQL, Scheme R5RS/R6RS/R7RS, TeX bit-exact output, Ethereum EVM clients (Geth, Parity, Erigon).

### 13.8 Seed-Produces-Tool Bootstrapping

Small bootstrap emitting larger conformant tool:

- **META II** (Dewey Val Schorre, 1964; [ACM DL](https://dl.acm.org/doi/10.1145/800257.808896)). Tiny metacompiler that compiles its own description.
- **Thompson, Reflections on Trusting Trust** (Ken Thompson, Turing Lecture, CACM 1984; [ACM DL](https://dl.acm.org/doi/10.1145/358198.358210)).
- **GCC three-stage bootstrap** ([GCC build docs](https://gcc.gnu.org/install/build.html)).
- **Rustc bootstrap chain** ([Rust Dev Guide](https://rustc-dev-guide.rust-lang.org/building/bootstrapping.html)).
- **Scheme R5RS/R7RS, Chibi-Scheme, Chicken Scheme** — minimal-core bootstrap tradition.
- **Bootstrappable Builds / GNU Mes / stage0** ([bootstrappable.org](https://bootstrappable.org/); [stage0](https://github.com/oriansj/stage0)). 357-byte hex0 → stage0 → M2-Planet → MesCC → TinyCC → GCC. The extreme case.
- **Forth** (Chuck Moore, 1970). Minimal kernel plus self-extending dictionary.

### 13.9 Architectural-Style Composition

The composition of heterogeneous styles within a single system:

- **Perry and Wolf**, *Foundations for the Study of Software Architecture* (1992).
- **Shaw and Garlan**, *Software Architecture: Perspectives on an Emerging Discipline* (1996). Heterogeneous/hybrid style composition: "a component of a system organized in one architectural style may have an internal structure developed in a completely different style."
- **Mehta and Medvidovic**, *Composing Architectural Styles from Architectural Primitives* (ESEC/FSE 2003).
- **Batory**, Feature-Oriented Programming and mixin layers (TOSEM 2002).

### 13.10 Information Hiding and Separation of Concerns

The ambivalence element of the induced property:

- **Parnas**, *On the Criteria to Be Used in Decomposing Systems into Modules* (CACM 1972).
- **Dijkstra**, *On the Role of Scientific Thought* (1974).

### 13.11 Positioning

Across this prior art, the SERVER dissertation's specific contribution is the application of Fielding's method at the orchestration level — one level outside where Doc 426 applies it — and the derivation of the progressive module composition spectrum as Fielding-style constraint accumulation. The pattern at the level of individual constraints is prior art in every case (dependency injection, OTP, Nix, OSGi, SLSA, Merkle, and Unix pipes are the densest sources). The style-level formalization of the pattern as a composed architectural level with inherited constraints from PRESTO is the contribution. Companion deflation documents (Docs 425, 427–431) develop the specific subsumption relationships in detail.

## 14. Referenced Companion Documents

- **Doc 426 — *PRESTO: An Architectural Style for Representation Construction.*** The construction-level style that SERVER composes with at the orchestration level.
- **Doc 424 — *SIPE (Architectural Form): Recursive Fielding-Style Constraint Accumulation in Composed Software Stacks.*** The method SERVER instantiates at the orchestration level. States the recursive-Fielding-accumulation framework of which SERVER is one level.
- **Doc 423 — *Narrowing SIPE: The Architectural Form Against the Literature.*** The literature survey grounding Doc 424's methodological claim.
- **Doc 421 — *Building in PRESTO: A Practitioner's Companion to the Dissertation.*** Developer-facing companion at the construction level; analogous companion at the orchestration level is deferred to Doc 422's eventual fill (currently stubbed).
- **Doc 418 — *The PRESTO Accumulation Test.*** Establishes that §4.3's module-privilege spectrum is formally a Fielding-style constraint accumulation in the same sense as PRESTO's code-on-demand spectrum.
- **Doc 431 — *Pulverizing SERVER.*** Prior-art survey across compiler bootstrapping, DI, capability systems, and supply-chain attestation, developed alongside this dissertation and catalogued in §13.
- **Security specification for runtime-graph attestation (to be written).** The detailed mechanics of hash selection, key rotation, manifest-compliance verification, and supply-chain chaining referenced in §4.5.
- **Doc 422 — *Building in SERVER* (Stub).** Practitioner companion placeholder; conditions for filling the stub named therein.
- **Doc 250 — *SERVER Seed.*** The bootstrap-resolver specification (eight contracts, twelve orchestration directives, fourteen-stage bootstrap pipeline, twelve verification tests) that this dissertation's constraints govern.

## Acknowledgments

This work follows the architectural-style-formalization method of Fielding (2000) extended by Perry and Wolf (1992), applied at a composed architectural level per Doc 424. It sits within the REST-successor tradition established by Khare and Taylor (2003/2004, ARRESTED), Erenkrantz and Taylor-Erenkrantz-Gorlick-Baquero (2009, CREST), Gorlick (COAST), and retrospectively surveyed by Fielding, Taylor, Erenkrantz, Gorlick, Khare, Hartman, and Baquero (2017, "Reflections on REST"). It rests on the dependency-injection, declarative-infrastructure, module-system, and compiler-bootstrap traditions catalogued in §13. The specific insight that enabled the orchestration-level formalization was the observation that the PRESTO seed is itself a bilateral document — orchestration affordances interleaved with resolver affordances — and that the bootstrap resolver stands in the same structural position with respect to the engine that the PRESTO engine stands in with respect to the resolved representation. The pattern that made this observation visible is Ken Thompson's (1984) reflection on the self-reproducing compiler; the specific application to a PRESTO engine bootstrapped from prose extends the REST-successor tradition to a tier the prior genre entries did not cover.

## References

- Armstrong, J., Virding, R., & Williams, M. (1986–). *Erlang/OTP design principles.* [erlang.org](https://www.erlang.org/doc/design_principles/sup_princ.html)
- Batory, D. (2002). *Feature-Oriented Programming and the AHEAD Tool Suite.* TOSEM.
- Birgisson, A., Politz, J. G., Erlingsson, Ú., Taly, A., Vrable, M., & Lentczner, M. (2014). [*Macaroons: Cookies with Contextual Caveats for Decentralized Authorization in the Cloud.*](https://research.google/pubs/pub41892/) NDSS.
- Dennis, J. B., & Van Horn, E. C. (1966). *Programming Semantics for Multiprogrammed Computations.* CACM.
- Ellison, C., et al. (1999). [*SPKI Certificate Theory.*](https://datatracker.ietf.org/doc/html/rfc2693) RFC 2693.
- Erenkrantz, J. R. (2009). *Computational REST: A New Model for Decentralized, Internet-Scale Applications.* UCI PhD dissertation.
- Feldman, S. (1979). *Make — A Program for Maintaining Computer Programs.* Software: Practice and Experience.
- Fielding, R. T. (2000). [*Architectural Styles and the Design of Network-based Software Architectures*, Chapter 5.](https://ics.uci.edu/~fielding/pubs/dissertation/rest_arch_style.htm) UCI doctoral dissertation.
- Fielding, R. T., Taylor, R. N., Erenkrantz, J. R., Gorlick, M. M., Khare, R., Hartman, H., & Baquero, C. (2017). [*Reflections on REST.*](https://dl.acm.org/doi/10.1145/3106237.3121282) ESEC/FSE 2017, Impact Paper Award.
- Khare, R., & Taylor, R. N. (2004). [*Extending the Representational State Transfer (REST) Architectural Style for Decentralized Systems.*](https://www.ics.uci.edu/~rohit/ARRESTED-ICSE.pdf) ICSE 2004.
- Klein, G., et al. (2009). *seL4: Formal Verification of an OS Kernel.* SOSP.
- McCarthy, J. (1960). [*Recursive Functions of Symbolic Expressions and Their Computation by Machine.*](https://www-formal.stanford.edu/jmc/recursive.pdf) CACM.
- McIlroy, M. D. (1973). *Unix pipes.* Bell Labs technical memorandum.
- Merkle, R. C. (1987). *A Digital Signature Based on a Conventional Encryption Function.* CRYPTO.
- Parnas, D. L. (1972). [*On the Criteria to Be Used in Decomposing Systems into Modules.*](https://dl.acm.org/doi/10.1145/361598.361623) CACM.
- Perry, D. E., & Wolf, A. L. (1992). *Foundations for the Study of Software Architecture.* ACM SIGSOFT Software Engineering Notes.
- Plotkin, G. D. (1981). *A Structural Approach to Operational Semantics.* Aarhus University.
- Schorre, D. V. (1964). [*META II: A Syntax-Oriented Compiler Writing Language.*](https://dl.acm.org/doi/10.1145/800257.808896) ACM National Conference.
- Scott, D., & Strachey, C. (1971). *Toward a Mathematical Semantics for Computer Languages.* Oxford PRG.
- Shaw, M., & Garlan, D. (1996). [*Software Architecture: Perspectives on an Emerging Discipline.*](http://sunnyday.mit.edu/16.355/intro_softarch.pdf) Prentice Hall.
- Smith, B. C. (1984). *Reflection and Semantics in Lisp.* POPL.
- Thompson, K. (1984). [*Reflections on Trusting Trust.*](https://dl.acm.org/doi/10.1145/358198.358210) CACM Turing Lecture.
