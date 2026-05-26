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
3. **Source-to-tokens:** user JavaScript source becomes a token stream under the lexical grammar of §11/§12, with the lexical goal symbol selected by the syntactic context (the lexer↔parser feedback edge per §XI.1). The lexer's contingent decisions include token-form selection per goal, had-escape preservation, NoLineTerminator-here tracking, ASI candidate identification, and string/template/numeric/regex literal lexing.
4. **Tokens-to-AST:** the token stream becomes AST plus import/export records under the syntactic grammar of §13–§15, with the parser owning goal-symbol selection at each lex call.
5. **AST-to-bytecode:** AST becomes opcode streams, constants, scope records, and module records.
6. **Bytecode-to-values:** the runtime dispatch loop consumes opcodes into values and side effects.
7. **Package-to-lockfile:** package metadata and tarballs become a closed artifact registry.
8. **Import graph-to-authority graph:** module reachability and capability passing determine which effects can occur.

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

The parser and bytecode pipeline handle user source text. The first stage of this pipeline — tokenization under a goal-symbol regime — is upstream of every downstream tier on this side and is itself a resolver-instance with its own contingent-decision surface; see §XI's lexical-grammar / tokenization coordinate class and §XI.1's articulation of the lexer↔parser feedback edge.

```
source text
  -> tokens (under InputElementDiv / InputElementRegExp / InputElementTemplateTail goal symbols, selected by the parser's syntactic context)
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

- Lexical grammar / tokenization: goal-symbol selection (InputElementDiv vs InputElementRegExp vs InputElementTemplateTail), token-form decisions (template literal, numeric literal incl. separators and BigInt suffix, string-escape decoding, line continuation), had-escape preservation on identifier tokens (the `break` vs `break` distinction required for §11.6.2 reserved-word early errors), ASI insertion points, NoLineTerminator-here tracking, regex literal lexing.
- Parser form: strictness propagation, module-mode parsing, early errors, destructuring heads, reserved-word gates, source-position threading.
- AST-to-bytecode lowering: operator semantics, lexical binding, super/new.target placement, control-flow emission, scope materialization.
- Runtime abstract operation: ToPrimitive, ToObject, ToLength, SameValue, IsCallable, property access, descriptor definition, iterator close.
- IR alphabet primitive: a spec operation or control form required to express ECMA algorithms without ad hoc Rust.
- Host and harness surface: `$262`, timers, filesystem, process, module loading, network APIs, and capability-bearing host hooks.
- Realm and identity substrate: intrinsics, prototypes, constructors, module namespace objects, species, symbols, and cross-realm behavior.
- Capability substrate: authority passing, closed import graphs, load-time integrity, compartments, and removal of ambient authority.

A parity fix is apparatus-complete only when it identifies which coordinate class it closed, or explicitly records that the coordinate remains unnamed.

### XI.1 The lexer↔parser feedback edge

The lexical-grammar coordinate class has one structural feature that no other class has: a back-edge to the syntactic grammar. ECMA-262 §12 specifies that the lexical goal symbol applied at each input position is **chosen by the syntactic context**. The canonical instance is the divide/regex disambiguation (`/` opens a RegularExpressionLiteral when the prior syntactic context expects a primary expression; it opens a DivPunctuator after a Member/Call/Identifier in expression position). Template-tail re-entry after `}` inside a TemplateMiddle is the second instance. The closing `}` of a substitution must be lexed under InputElementTemplateTail, which the parser must signal.

This relation is genuinely cyclic at the calling-convention level: tokens flow forward (lexer → parser), goal symbols flow backward (parser → lexer). Every other edge in the DAG is forward data-flow. The lexer↔parser pair is the one place where the language-behavior DAG is not strictly acyclic.

The cycle is resolved by the resolver-instance discipline of [Doc 729](../../docs/corpus-ref/729-cruftless-a-primary-articulation-of-the-resolver-instance-pattern-as-the-comprehensive-design-toward-which-rusty-bun-morphs.md) §IV in a specific shape: the lexer is a resolver-instance whose input includes a **goal-symbol directive** parameter from the parser; each individual lex call is acyclic (goal in, token out, stage-deterministic). The feedback loop exists at the calling-convention level (parser owns the lex loop, drives the goal selection from its own syntactic state) but does not exist within any single lex call. Goal-symbol selection is therefore the parser's contingent decision, not the lexer's; the lexer's contingent decisions are the token-form decisions per goal symbol.

The cruft lexer carries this discipline as `LexerGoal::{InputElementDiv, InputElementRegExp, InputElementTemplateTail}` at `pilots/rusty-js-parser/derived/src/lexer.rs:16`. The parser passes the goal at every call site; the lexer does not reach into parser state. This is the legitimate resolution of the one non-acyclic relation, and naming it explicitly is the apparatus tax this section pays so the DAG framing holds without quiet exception.

A coordinate at the lexical-grammar tier is apparatus-complete when it identifies (a) which goal-symbol is in scope for the production, (b) which token-form decision the substrate must make, (c) whether the parser-side goal-symbol-selection rule is correctly conditioned on the relevant syntactic context. Today's session has touched two such coordinates: ALTA-EXT 1 (NoLineTerminator before `=>` — token-tier flag consumption at the parser tier) and prior arc work on had-escape preservation (`pilots/parser-permissiveness/` — A3 axis was the explicit token-tier obligation). Both should be re-read against this coordinate class now that it has a name.

#### XI.1.a Empirical close — the LGSS locale

The first locale spawned under the new coordinate class is `pilots/lexer-goal-symbol-selection/` (LGSS), closed 2026-05-25 in three rungs. Its empirical evidence refines this section.

**What LGSS established.** The back-edge resolution discipline articulated above (goal-as-directive-parameter, lexer never reaches into parser state, per-call stage-determinism) is sound but the implicit constraint it names — *goal-symbol selection is a function of the prior token's expression-completion status (plus template-substitution context)* — was distributed across the cruft parser as scattered ad-hoc carriers before LGSS landed:

- 1 inline `if token_completes_expression(...) { Div } else { RegExp }` derivation in `bump_regexp`.
- 2 explicit `LexerGoal` parameters at parser-tier method signatures (`rewind_lexer_to(pos, goal)`, `refetch_lookahead_with_goal(goal)`).
- 2 `LexerGoal::X` literal mentions at downstream-tier call sites (`stmt.rs:1251`, `expr.rs:1583`).

After LGSS-EXT 1+2 the same constraint is consolidated to:

- 1 named predicate (`derive_lex_goal_after`) — the canonical site for the Div/RegExp decision; §12.9.5 made executable.
- 1 named parser-state field (`Parser::current_lex_goal`) carrying the per-bump invariant.
- 1 named per-bump hook (inline tail of `bump_regexp`) maintaining the invariant.
- 2 intent-named methods (`enter_template_tail`, `rewind_lexer_to`) for the irreducible carriers (see §XI.1.b).
- 0 external (non-parser-crate) `LexerGoal` literal mentions.

**LoC accounting.** Net executable-code delta across the three rungs: **+4 LOC**. The grow is the cost of naming the implicit constraint (the new predicate, field, and hook each cost their bytes); it is offset against the 2 deleted call-site mentions, the 2 deleted method-signature parameters, and the 1 deleted inline derivation. The full doc-and-comment delta is +46 lines (doc-comments on the new field, the new predicate, and the two intent-named methods explaining the §XI.1 carrier discipline).

What was *eliminated from the DAG* is not raw LoC; it is **surface contamination**. Before LGSS, the source-to-tokens → tokens-to-AST tier boundary leaked `LexerGoal` into stmt.rs and expr.rs (downstream tiers); the goal-symbol-selection discipline existed at the wrong tier. After LGSS, the goal-symbol-selection lives entirely inside the parser crate, with the lexer holding the directive-parameter contract and the parser owning every consumer. Two tiers of the DAG (tokens-to-AST and downstream) became goal-symbol-agnostic. The lexer↔parser feedback edge is now confined to the resolver-instance boundary the §XI.1 articulation says it should live at.

#### XI.1.b Irreducible carriers within tokenization-coordinate scope

LGSS-EXT 3 documented two carriers that cannot be reduced further within the lexical-grammar coordinate class:

1. **`enter_template_tail`** — the template-substitution-close re-lex. cruft's lexer emits Template{Middle/Tail} *starting at the `}` byte itself*; the `}` is the leading delimiter of the next template part, not its predecessor. The only correct sequence is **re-lex at the same byte position** under TemplateTail goal. This is structurally distinct from bump's forward fetch and cannot be folded into a per-bump hook without restructuring lexer byte-boundaries (deep lexer change affecting raw/cooked-string semantics) or introducing a pre-bump hook (parser-machinery change). Both are outside tokenization-coordinate scope.

2. **`rewind_lexer_to`** — the for-head bare-identifier fast-path bail. With cruft's single-token lookahead, the parser must commit to bumping an identifier before it can see whether `in`/`of` follows; if not, the rewind is the recovery. Eliminating the rewind requires either two-token lookahead (architectural) or threading the spec's `[+In]` grammar parameter through the precedence climber (§13.7.5 ForStatement; eliminates the need for the fast-path entirely because `parse_expression` under `[+In]` would refuse `id in obj` as a RelationalExpression in for-head LHS position). The `[+In]` move is the spec-aligned alternative and is a candidate locale of its own.

Both carriers are intent-named (the method name expresses the parser-tier purpose) and parser-internal (no downstream tier sees `LexerGoal`). They are the minimum the lexer↔parser back-edge reduces to under cruft's current parser architecture. The apparatus marks them as **bounded structural exceptions**, distinct from the unbounded ad-hoc goal-passing the original §XI.1 articulation guarded against. Future architectural work (lexer byte-boundary restructure; `[+In]` threading) could collapse one or both; until then, naming them explicitly preserves the §XIII discipline (no closure without a named coordinate).

**Spinoff locale candidate**: `parser-precedence-in-flag` — implement `[+In]`/`[-In]` threading through the precedence climber. Would eliminate the `rewind_lexer_to` carrier; lives outside LGSS scope. CANDIDATES queue.

**Standing finding (LGSS.5)**: locale boundaries are themselves coordinates. When remaining carriers are blocked by orthogonal substrate constraints, the honest closure documents the obstructions and surfaces the spinoff locale candidates that would address them. The rule-11 pre-spawn check should include *"what carries the locale's scope, what borders sit on other locales' scopes"* alongside the existing 5 axes.

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

