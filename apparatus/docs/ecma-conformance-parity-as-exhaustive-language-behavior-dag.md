# ECMA Conformance Parity as Exhaustive Language Behavior DAG

The cruftless apparatus is not merely a workflow for improving a JavaScript engine. It is a closed-loop substrate for extracting the implementation decision basis of ECMAScript.

The apparatus uses conformance work as pressure. Each failing fixture, differential divergence, corpus regression, or benchmark anomaly is treated as evidence that some decision in the engine's DAG has not yet been materialized, named, or placed at the correct tier. A passing result is not the telos by itself. The telos is the explicit coordinate that had to exist for the result to pass.

The primary claim:

```
full parity
  -> closed empirical language-behavior DAG
  -> spec-corresponded decision basis
  -> computable implementation inference
```

Under this articulation, cruftless does not chase ECMAScript conformance as a product polish metric. It uses conformance as the forcing function that enumerates the implementation decisions a JavaScript engine necessarily makes.

## I. Superstructure

The corpus docs give this apparatus a four-layer superstructure.

1. **DAG as load-bearing object** (Doc 715 / Doc 720): consumer-visible behavior is read through a graph of substrate dependencies and engine pipelines. Failures are not isolated bugs; they are residues at nodes, edges, cuts, or joins in that graph.
2. **Three projections** (Doc 716 / Doc 717): the same object is read through DAG, lattice, and alphabet projections. DAG asks what depends on what. Lattice asks at which abstraction rung the cut sits. Alphabet asks which stable decision class the cut belongs to.
3. **Resolver-instance recurrence** (Doc 729 / Doc 730): every tier is a resolver-instance or lowering compiler. Source plus directives becomes directive-free artifact by stage-deterministic resolution. The pattern recurs from ECMA text to IR, IR to Rust, Rust to machine code, package graph to lockfile, and module source to bytecode.
4. **Locale coordinates** (Doc 737): each workstream records extraction at an apparatus coordinate. Seed/trajectory pairs are not project notes; they are the durable coordinate system where the decision basis becomes recoverable by future resolvers.

This document sits at the join of those layers. It states what the apparatus is for at the ECMA conformance frontier.

## II. What the apparatus is

The apparatus is the set of repo-local instruments, disciplines, locales, and substrate tiers that make each decision visible.

- Measurement instruments observe behavior against external oracles: test262, diff-prod, cross-runtime benches, consumer corpora, fuzz fixtures, and per-locale probes.
- Discipline artifacts constrain what counts as a valid next move: standing rules, feedback schemas, spawn protocols, seed/trajectory pairs, and falsifier gates.
- Locales bind a workstream to a coordinate: a seed states the telos and falsifiers; a trajectory records the rounds; optional agent feedback captures cross-resolver readings.
- Substrate tiers receive the work: parser, AST, IR, bytecode, runtime, JIT, shapes, GC, package manager, host APIs, and capability-passing layers.

The loop is cybernetic because each move modifies the substrate, the instruments observe the modified substrate, the discipline interprets the observation, and the locale records the decision for the next resolver.

## III. The language-behavior DAG

Doc 715 names the consumer-substrate dependency graph as the load-bearing object. At ECMA scale, the graph refines into the language-behavior DAG.

Nodes include:

- ECMAScript syntactic forms and early-error rules.
- Abstract operations and internal methods.
- Intrinsic objects, prototypes, constructors, realms, and environment records.
- Parser, AST, bytecode, runtime, JIT, GC, module loader, package manager, and host pipeline stages.
- Authority-bearing host surfaces, including capability handles, import graphs, integrity records, and built-in namespaces.

Edges include:

- Spec dependency edges: one algorithm invokes another, or one internal method depends on another.
- Pipeline data-flow edges: tokens become AST, AST becomes bytecode, bytecode becomes runtime effects.
- Cross-pipeline composition edges: a single behavior requires parser, compiler, runtime, and intrinsic install choices to agree.
- Authority edges: a module can perform an effect only when a capability reference is reachable.
- Test-observation edges: a fixture reaches the behavior through a specific path.

Full conformance parity closes this DAG empirically to the resolution of the suite. It does not merely count green tests; it forces the graph's missing nodes and edges to become explicit.

## IV. DAG, lattice, alphabet

The apparatus reads every residue through the three projections from Docs 716 and 717.

**DAG projection:** where does the observed behavior sit in the dependency graph, and what fanout depends on it?

**Lattice projection:** at what rung does the cut live? For engine semantics this ranges from specification text, algorithmic steps, internal methods, intrinsic objects, execution-context records, and realms. For the implementation stack it ranges across parser, AST, bytecode, runtime, host, package manager, and capability substrate.

**Alphabet projection:** what stable class of decision is this? Examples include parser-form decision, descriptor-shape decision, coercion decision, module-namespace decision, realm-identity decision, host-surface cut, IR primitive, or authority edge.

The pure abstraction point from Doc 717 is the limit where these projections collapse into the specification's own structure. The DAG becomes the spec's algorithm dependency graph. The lattice becomes the spec's abstraction hierarchy. The alphabet becomes the engine's relation to the spec: conformant, relaxation, extension, or version lag.

The apparatus's job is to move implementation work toward that point without losing the empirical traction of tests and consumer probes.

## V. Resolver instances

Doc 729 frames cruftless as vertically-recursive directive consumption with stage-deterministic emission. The ECMA conformance apparatus inherits that directly.

Relevant resolver-instances:

1. **Spec-to-IR:** ECMA algorithm prose/XML becomes `rusty-js-ir` sections.
2. **IR-to-runtime:** IR lowers to Rust helpers and generated intrinsic code.
3. **Source-to-AST:** user JavaScript source becomes AST plus import/export records.
4. **AST-to-bytecode:** AST becomes opcode streams, constants, scope records, and module records.
5. **Bytecode-to-values:** the runtime dispatch loop consumes opcodes into values and side effects.
6. **Package-to-lockfile:** package metadata and tarballs become a closed artifact registry.
7. **Import graph-to-authority graph:** module reachability and capability passing determine which effects can occur.

Each resolver-instance must consume its directives completely. Residue is the smell: an import still visible as runtime ambiguity, an IR section that still requires ad hoc Rust transcription, a parser flag ignored by bytecode emission, a built-in namespace reachable without an authority edge, or an algorithmic step that has no named coordinate.

## VI. Lowering recurrence

Doc 730 names the lowering compiler as a recurring pattern across substrate tiers. The apparatus depends on this recurrence because ECMA parity work crosses tier boundaries constantly.

Every lowering tier should have:

- Typed primitives.
- Stage-deterministic compilation.
- Verifier-before-emission.
- Implementation freedom only under preserved semantic invariants.

`rusty-js-ir` is the load-bearing example. It is not a user-JS parser. It is the spec-algorithm lowering tier:

```
ECMA algorithm section
  -> IRFunction
  -> linter/verifier
  -> Rust lowering
  -> runtime helper surface
```

The parser and bytecode pipeline handle user source text:

```
source text
  -> AST/import-export records
  -> bytecode/module records
  -> runtime values
```

The two pipelines meet at abstract operations, intrinsics, and helper calls. A conformance failure must therefore be classified before repair: parser form, AST-to-bytecode lowering, runtime abstract op, IR alphabet/lowering, host surface, package graph, or capability authority.

## VII. Parity is empirical closure

Full test262 parity is load-bearing because it closes the decision basis to test262 resolution. It forces the engine to materialize every behavior that the suite can observe.

This is not the same as saying test262 is the mathematical specification of ECMAScript. It is finite and empirical. Passing it proves the engine satisfies the suite, not every possible reading of ECMA-262.

The apparatus therefore treats parity as the first closure condition:

```
test262 parity = empirically closed language-behavior DAG
```

At that point, the visible behavioral residue is gone. The remaining question becomes whether every coordinate that allowed the pass is corresponded to the standard's own semantic structure.

## VIII. Spec correspondence is semantic authority

The second closure condition is correspondence between the decision basis and ECMA-262's algorithmic semantics.

The correspondence target is:

```
spec step or abstract operation
  -> named DAG coordinate
  -> owning resolver-instance
  -> executable substrate implementation
  -> observable conformance behavior
```

Without this correspondence, the apparatus can say "the engine passes the probes." With it, the apparatus can say "this coordinate is the implementation obligation imposed by this semantic step."

The correspondence must be stronger than prose citation. A resolver should be able to ask which spec step, abstract operation, internal method, or host hook owns a coordinate, and which lowering path implements it.

## IX. Authority is part of the DAG

Doc 736 extends the conformance DAG with capability-passing, closed import graphs, and load-time integrity. Authority is not an external security layer. It is an edge relation in the same language-behavior DAG.

Ambient authority creates hidden edges. A module can reach filesystem, network, process, clock, or dynamic import behavior without any explicit path in the graph. Capability passing removes those hidden edges. The reachable effect set becomes computable from the import graph and the capability references passed through it.

That matters for ECMA conformance because host hooks, module loading, dynamic import, realms, compartments, and built-in namespaces all compose with authority. A spec-corresponded DAG that ignores authority would be incomplete. A future Compartments-first JavaScript API needs the same property: closed import graph, explicit capability edges, and load-time integrity so that resolution produces a directive-free, authority-explicit artifact.

Thus capability work is not adjacent to parity. It is the authority projection of the same exhaustive behavior DAG.

## X. Traversal replaces blind discovery

The third closure condition is computable traversal.

An LLM working from ECMA prose performs blind discovery. It reads prose, recalls patterns, guesses a decision structure, writes a patch, and waits for instruments to falsify the guess.

A resolver working against a closed, spec-corresponded DAG should not have to discover the decision structure. It should traverse it:

```
feature or failure
  -> observed residue
  -> affected coordinate(s)
  -> projection: DAG / lattice / alphabet
  -> owning resolver-instance
  -> required composing decisions
  -> implementation obligation
  -> falsifier gate
```

This is the category shift the apparatus is designed to make possible. Trust moves from the model's priors to the artifact's correspondence.

The shift only holds if the DAG is machine-usable. If nodes and edges are prose that a resolver must reinterpret, blind discovery has merely moved from the spec to the documentation. The apparatus must therefore prefer structured coordinates, explicit edge relations, reproducible measurements, and verifier/linter-backed lowering over descriptive notes.

## XI. The decision basis

A decision basis coordinate names a contingent implementation obligation. It is not just a bug label.

Examples of coordinate classes:

- Parser form: strictness propagation, module-mode parsing, early errors, destructuring heads, reserved-word gates, source-position threading.
- AST-to-bytecode lowering: operator semantics, lexical binding, super/new.target placement, control-flow emission, scope materialization.
- Runtime abstract operation: ToPrimitive, ToObject, ToLength, SameValue, IsCallable, property access, descriptor definition, iterator close.
- IR alphabet primitive: a spec operation or control form required to express ECMA algorithms without ad hoc Rust.
- Host and harness surface: `$262`, timers, filesystem, process, module loading, network APIs, and capability-bearing host hooks.
- Realm and identity substrate: intrinsics, prototypes, constructors, module namespace objects, species, symbols, and cross-realm behavior.
- Capability substrate: authority passing, closed import graphs, load-time integrity, compartments, and removal of ambient authority.

A parity fix is apparatus-complete only when it identifies which coordinate class it closed, or explicitly records that the coordinate remains unnamed.

## XII. How this guides standards implementation

Once the basis is empirically closed, semantically corresponded, and computably traversable, the DAG becomes a guide for implementing new ECMA standards.

The guide has three tiers:

1. Impact analysis: given a proposal, identify which existing coordinates it perturbs.
2. Derivation: given a spec algorithm, derive the implementation obligations and owning tiers.
3. Authorship feedback: given a proposed semantic change, expose underspecified, contradictory, or unusually expensive decision surfaces.

The first tier can emerge from a closed parity basis. The second and third require spec correspondence strong enough to make the edges decidable.

This is the standards-facing telos: not merely an engine that follows ECMA, but a computable decision basis that can guide future ECMA implementation work.

## XIII. Operating consequences

Every conformance move should leave a coordinate trail.

When closing a failure cluster:

1. Identify the observable residue.
2. Classify the owning tier.
3. Read the residue through DAG, lattice, and alphabet projections.
4. Name the decision coordinate or add a candidate coordinate.
5. Identify the owning resolver-instance and lowering path.
6. Implement the smallest substrate change that closes the coordinate.
7. Re-run the falsifier gate.
8. Record the result in the locale trajectory.

When a move requires a new IR primitive, parser rule, runtime helper, host capability, or authority edge, the apparatus should treat that addition as alphabet growth. Alphabet growth is not bad. Silent alphabet growth is bad.

When a move passes tests but leaves no named coordinate, the result is incomplete from the apparatus perspective.

## XIV. Relation to corpus docs

This articulation is informed by:

- `docs/corpus-ref/715-*.md`: the consumer-substrate DAG as the load-bearing object.
- `docs/corpus-ref/716-*.md`: stubs as named cuts; DAG/lattice/alphabet as operational projections.
- `docs/corpus-ref/717-*.md`: the apparatus lifted across the engine boundary to the pure abstraction point.
- `docs/corpus-ref/720-*.md`: the runtime as interconnected pipelines whose alphabets compose into a DAG/lattice.
- `docs/corpus-ref/729-*.md`: cruftless as vertically-recursive directive consumption with stage-deterministic emission.
- `docs/corpus-ref/730-*.md`: lowering compiler recurrence across substrate tiers.
- `docs/corpus-ref/736-*.md`: capability-passing, closed import graphs, and load-time integrity as authority-edge closure.
- `docs/corpus-ref/737-*.md`: locale-as-coordinate and nested seed/trajectory pairs as substrate positions.

This articulation composes with:

- `apparatus/docs/repository-apparatus.md`, which enumerates the current loop and its artifacts.
- `apparatus/docs/predictive-ruleset.md`, which names standing predictions that constrain substrate moves.
- `apparatus/docs/agent-feedback-schema.md`, which captures cross-resolver readings of locale work.
- `IR-DESIGN.md`, which states the resolver-instance role of `rusty-js-ir`.
- `pilots/rusty-js-ir/seed.md`, which states the current IR telos and alphabet-completeness discipline.

This document is the compact primary statement of what the apparatus is for:

```
conformance pressure
  -> coordinate extraction
  -> decision-basis closure
  -> spec correspondence
  -> computable inference for future implementation
```

