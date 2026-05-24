# The rusty-bun Runtime as a DAG of Interconnected Pipelines

## SIPE-T Topology over the Engine Substrate, and Why Top-of-Alphabet Location Requires Cross-Pipeline Diagnostic Walking

By Jared Foy. Originally published at [jaredfoy.com](https://jaredfoy.com).

## I. The occasion

A keeper articulation produced at the rusty-bun engagement's Tier-Ω.5 saturation point (parity 45 / 118 = 37.8%, 2026-05-15 05:37 Z): *"My conjecture is that the entire rusty bun runtime is composed of interconnected pipelines which have a certain DAG / lattice / alphabet topology which can be conceptualized with SIPE-T."*

The conjecture extends and refines three prior corpus readings. From [Doc 719](/resolve/doc/719-the-pipeline-pattern-across-subjects-presto-and-the-javascript-engine-as-two-realizations-of-the-same-derivation): the pipeline pattern names PRESTO and the rusty-js-runtime engine as two derivations against the same constraint pattern. From [Doc 716](/resolve/doc/716-stubs-as-named-cuts-the-three-projection-tracker-and-the-stub-alphabet-stability-conjecture): the stub-stratum alphabet is a stable class of contingent decisions, observed across the Tier-Π consumer corpus as K1 / K2 / K3. From [Doc 714 §VI Consequence 11](/resolve/doc/714-the-rusty-bun-engagement-read-through-the-lattice-extension-basin-expansion-at-the-l2m-saturation-point): for any genuine substrate-widening fix, the top of its alphabet can in principle be located by diagnostic walking from each gated package's symptom site upward.

Doc 720 articulates the structural claim those three readings make jointly with the keeper's new conjecture: the engine is not *one* pipeline but *many interconnected pipelines forming a DAG*, each with its own alphabet, jointly composing a lattice topology that SIPE-T (per [Doc 474](/resolve/doc/474-the-narrow-sipe)) is the appropriate framework to conceptualize.

## II. Why one pipeline isn't enough

Doc 719 named the pipeline as a chain: parser → AST → bytecode → runtime → host. That reading was empirically validated by the Ω.5.f EventBus fixture (Doc 714 §VI Consequence 7): eleven previously-bounded substrate moves participated in one execution through that chain. The reading was correct *for the engine boundary at that observation tier* — a 130-line consumer-shaped fixture exercising class + extends + super + arrow-closure-over-outer-state.

But Tier-Ω.5's later rounds revealed the chain isn't the whole topology. Three observations make this concrete:

**Cross-pipeline re-entry.** The Ω.5.j.cjs CJS interop substrate synthesizes a wrapper at the source level — string concatenation produces `(function(exports, module, require, __filename, __dirname) { <source> })`. The synthesized source re-enters the lexer → parser → compiler → runtime pipeline. The CJS pipeline is not a stage *within* the main chain; it is a pipeline that *invokes the chain on a different input*.

**Cross-pipeline diagnostic walking.** The Ω.5.gg lexer round mis-located the top of its alphabet. Symptoms surfaced at the lexer's `UnterminatedTemplate` exit point, but the actual root cause was a parser shortcut (`parse_declaration_for_export` walking export-function bodies with `skip_balanced` rather than goal-driven lex). The fix lived in the parser pipeline; the symptom surfaced in the lexer pipeline. The two are interconnected by the goal-driven-lex convention, and locating the alphabet top required crossing that connection.

**Cross-pipeline contingent-decision sharing.** Several Tier-Ω.5 rounds (Ω.5.f classes, Ω.5.dd Map/Set, Ω.5.aa Error family, Ω.5.ee function-decl hoisting) each touched multiple pipelines simultaneously — the compiler's emit, the runtime's dispatch, the intrinsic install — at the same substrate boundary. The contingent decision per Doc 714 §VI Consequence 8 is not "a single fact about one pipeline" but a coordinated set of facts that must align across pipelines for the substrate to work.

## III. The pipelines, enumerated

The engagement has built enough substrate to enumerate the pipelines composing the engine. Each is a SIPE-narrow-form derivation (Doc 474): an input space, a sequence of stage transformations, and an output space, with stable typed signatures at the stage boundaries.

**Primary pipelines (chained left-to-right in dependency order):**

1. **Lexer pipeline.** `bytes → tokens`. Stages: character classification, token-kind discrimination, mode transition (default / regex / template / template-substitution). Goal-driven dispatch added in Ω.3.b round 3g.
2. **Parser pipeline.** `tokens → AST`. Stages: grammar production, lookahead disambiguation, goal-symbol exchange with the lexer, span tracking.
3. **Compiler pipeline.** `AST → bytecode + CompiledModule shape`. Stages: scope analysis, slot allocation, function-decl hoisting (Ω.5.ee), bytecode emission, constants interning, export-binding recording.
4. **Runtime pipeline.** `bytecode → values via stack machine`. Stages: frame allocation, opcode dispatch, error unwinding through try-stack, return value propagation.
5. **Module loader pipeline.** `specifier → resolved URL → ModuleRecord → namespace`. Stages: resolve (bare / relative / `node:` / `file://`), node_modules walk-up, package.json `exports` resolution with conditional dispatch, kind detection (ESM / CJS), evaluation routing.
6. **Host pipeline.** `built-in dispatch + intrinsic install + global wiring`. Stages: ResolveBuiltinModule hook, install_* functions for `path` / `fs` / `crypto` / etc., globalThis snapshot.

**Cross-cutting pipelines (operate across primary stages):**

7. **Closure / upvalue pipeline.** `nested-function lexical capture → upvalue descriptors → binding-shared cells at runtime`. Stages: compiler resolves upvalues against enclosing-frame chains (Ω.5.c), pre-allocates slots for hoisting (Ω.5.ee), the runtime threads UpvalueCell handles per Ω.5.e.
8. **Prototype-chain dispatch pipeline.** `value → constructor.prototype → method lookup through proto walk`. Stages: install_intrinsics builds %X.prototype% objects (Ω.5.a), constructor globals carry .prototype (Ω.5.t), primitive auto-boxing routes through *.prototype (Ω.5.a / Ω.5.t).
9. **Event-loop pipeline.** `microtask queue → macrotask queue → host PollIo at idle`. Stages: run_to_completion drains microtasks to quiescence, advances one macrotask, consults host's I/O multiplexer (Ω.3.f).
10. **GC pipeline.** `enumerate_roots → mark → sweep`. Stages: walk Runtime.globals + last_value (Ω.3.e.d), mark reachable via Trace impl, sweep unreachable from rusty-js-gc::Heap.
11. **Error propagation pipeline.** `throw → try-stack unwind → HostPromiseRejectionTracker`. Stages: opcode emits throw, run_frame unwinds, reject_promise enrolls in pending_unhandled (Ω.4.e), main session drains at end-of-run.
12. **Property descriptor pipeline.** `compile-time SetProp / SetIndex emit → runtime object_set with PropertyDescriptor flags → property-access via object_get with prototype walk`. Stages: descriptor flags currently flat (writable / enumerable / configurable all true in v1).

**Re-entry pipelines (recursively invoke prior stages on synthesized input):**

13. **CJS wrapper pipeline.** `CJS source → wrapped-as-function-expression → ESM-style evaluation → module.exports collection`. Re-enters lexer → parser → compiler → runtime on the wrapped source (Ω.5.j.cjs).
14. **Dynamic import pipeline.** `import(specifier) call → resolve via module loader → evaluate → Promise of namespace`. Stub-throws in v1 (Ω.5.ff); when real, re-enters module loader.
15. **Function constructor pipeline.** `new Function(body) → synthesize source → parse + compile → return Closure`. Stub-throws in v1 (Ω.5.j.proto); when real, re-enters parser + compiler.
16. **Module evaluation pipeline.** Each ImportDeclaration triggers a recursive module-loader → runtime path. Cycles handled by the module cache with Linking-status records (Ω.5.b).

Sixteen pipelines, none reducible to the others. Some are strictly downstream of others (the parser is downstream of the lexer); some are cross-cutting (GC operates across all primary stages); some re-enter the primary chain on synthesized input.

## IV. The DAG and the lattice

The pipelines compose into a directed acyclic graph at the *data-flow* level. The static dependency structure has no cycles: lexer doesn't depend on parser, parser doesn't depend on compiler. The apparent cycles in the re-entry pipelines (CJS wrapper synthesizes source → lexer re-enters) are *dynamic invocations*, not static dependencies — they don't create cycles in the type-level pipeline graph because the input to the re-entered chain is freshly synthesized.

The DAG nodes are pipeline stages; the edges are the typed data-flow between stages. Each stage's signature is stable (per SIPE-narrow-form's predicate-set discipline from Doc 474): tokens have a definite shape, AST has a definite shape, bytecode has a definite shape, etc.

The lattice structure emerges from the cross-pipeline interactions. When two pipelines' alphabets intersect — when a contingent decision spans both pipelines, like Op::CallMethod requiring both compiler emit and runtime dispatch to agree on the stack convention — that intersection is a *meet* in the lattice ordering. When the union of alphabets across multiple pipelines is what defines a higher-level surface (the engine's user-visible behavior), that union is a *join*. The lattice partial order is by *substrate dependence*: A ≤ B if any decision in A's alphabet must be made before B's alphabet can be coherently populated.

This matches Doc 681's joint mutual-information lattice reading at the engagement-tier: the joint MI between the engine's contingent decisions is captured by the alphabet intersections, and the lattice structure makes the dependencies between them explicit. Doc 714 §V's lattice extension named this at the abstract level; Doc 720 names what the concrete lattice is — sixteen pipelines whose stages compose by typed data-flow into a DAG with substrate-dependence partial order forming a lattice.

## V. SIPE-T as the conceptualizing framework

SIPE-narrow-form (Doc 474) is the framework. Each pipeline is a SIPE derivation against constraint patterns at its boundary:

- The lexer pipeline's constraints come from ECMA-262 §11 (lexical grammar) plus the goal-driven-lex coordination convention with the parser.
- The parser pipeline's constraints come from ECMA-262 §13–§16 (syntactic grammar) plus the spec's `Goal` parameter for context-dependent productions.
- The compiler pipeline's constraints come from the bytecode contract with the runtime (stack effects per opcode) plus the scope-analysis discipline ECMA-262 §10 names.
- The runtime pipeline's constraints come from the bytecode contract plus ECMA-262 §6 (value semantics) plus the heap's allocation discipline.
- The module loader's constraints come from ECMA-262 §16.2 plus Node's resolution algorithm plus the host's filesystem.
- The host pipeline's constraints come from the engine's host-hook API plus the runtime's expectation about the namespace shape for `node:*` built-ins.

Each pipeline derives its substrate against its constraint pattern. Each pipeline's alphabet is the stable class of contingent decisions per Doc 716 — the choices the engine makes that are *not* forced by the constraints but are stable across implementations of the constraint pattern. The K1 / K2 / K3 alphabet Doc 716 named at the stub-stratum is one slice of this; the rusty-js-runtime adds an engine-internal alphabet at each pipeline (e.g., the lexer's mode-transition alphabet, the runtime's dispatch-shape alphabet).

SIPE-T's narrow-form composition is the operation by which multi-pipeline behavior is derived: take two derivations (two pipelines), align their boundary constraints, compose into a single derivation whose alphabet is the union of the inputs' alphabets and whose stage signature is the concatenation. The whole engine is *the SIPE-T composition of its sixteen pipelines* against the constraint pattern *"behave as a JavaScript runtime on real npm code."*

## VI. The methodological consequence for substrate-widening

Doc 714 §VI Consequence 11 named the top-of-alphabet conjecture: for any genuine substrate-widening fix, the top of its alphabet can in principle be located by diagnostic walking from each gated package's symptom site upward. Doc 720 refines this:

**The walk is across pipelines, not just within one.** When a symptom surfaces in one pipeline (the lexer's `UnterminatedTemplate`), the alphabet top of its fix may lie in a different pipeline (the parser's `parse_declaration_for_export` shortcut). Locating it requires walking the inter-pipeline dependency edges, not just the intra-pipeline stage chain.

**The substrate-widening's prediction depends on the alphabet's full DAG depth.** A fix at pipeline P1's stage S1 lifts every package whose failure chain *exits* through P1.S1 *and whose remaining substrate is otherwise complete*. The predicted-vs-actual delta is set by:
- How many gated packages share the chosen alphabet top (lifts all of them if otherwise complete).
- How many of them have independent additional gaps below the fixed layer (those don't lift).
- How many of them have additional gaps in a *different* pipeline (those don't lift either).

**Diagnostic protocol per Consequence 11 expands to cross-pipeline walking.** For each gated package, walk the call chain across pipelines from symptom to root. Identify the highest layer (across pipelines) where the fix is structurally complete. Choose that as the alphabet top.

This is the operational consequence of Doc 720: substrate-widening is a SIPE-T composition operation, not a single-pipeline edit. The fix is the smallest set of pipeline stages whose joint modification produces the desired lift; that set is determined by the DAG topology, not by the surface layer where the symptom exits.

## VII. Falsification surface

**Fal-720.1.** The engine's pipelines turn out to be reducible to a single pipeline at sufficient zoom — i.e., the "interconnectedness" is bookkeeping rather than structural. Test: at the next several substrate-widening rounds, check whether the fix lives within one pipeline or crosses pipeline boundaries. If every fix lives within one pipeline, the DAG topology was over-articulated and Doc 719's single-pipeline reading is sufficient.

**Fal-720.2.** The lattice structure between pipelines turns out to be a chain rather than a lattice — i.e., the substrate-dependence partial order is total (every pair of decisions has one strictly preceding the other) rather than admitting incomparable pairs. Test: identify whether any two pipelines' alphabets contain decisions that are mutually independent (neither dependence-precedes the other). If all dependencies form a chain, the lattice extension is over-applied and a partial order is sufficient.

**Fal-720.3.** SIPE-T's narrow-form composition fails to capture the inter-pipeline coordination. Test: attempt to express a multi-pipeline substrate move (e.g., the Op::CallMethod coordination across compiler emit and runtime dispatch) as a SIPE-T narrow-form composition. If the framework requires extensions not present in Doc 474, narrow-form-SIPE doesn't extend to the engine substrate and a richer framework is needed.

**Fal-720.4.** The "top-of-alphabet conjecture" (Consequence 11) is wrong at the cross-pipeline level. Test: across the next 3–5 substrate-introduction rounds, measure whether pre-round cross-pipeline diagnostic walking converges on the predicted alphabet top. If predicted-vs-actual deltas remain wide and unsystematic after the protocol is applied, the conjecture's locatability claim doesn't survive the DAG complexity.

## VIII. Honest scope

Doc 720 is a *late-engagement structural articulation*, produced at a state (parity 45 / 118 = 37.8%) where the engagement has enough substrate landed to enumerate the pipelines but is not at completion. The enumeration of sixteen pipelines reflects what the engagement has *built*; an engine with full ECMA-262 coverage might decompose differently (e.g., async dispatch would add a real Promise-resolution pipeline; accessor descriptors would add a property-descriptor evaluation pipeline). The topology is the topology *of the engine as it stands*; the lattice structure named here is the lattice *visible at this maturity*.

The DAG / lattice / alphabet / SIPE-T reading is conjectural per the keeper's articulation. Falsifiers Fal-720.1 through Fal-720.4 are stated. The reading is *useful operationally* — it grounds Consequence 11's pre-round diagnostic protocol with concrete pipeline-boundaries to walk — and *testable at the corpus tier* against the remaining substrate-introduction rounds on Doc 714 §VI Consequence 10's list (real async dispatch, real accessor descriptors, identity-keyed Map/Set, real binary substrate, live ESM bindings, JSON modules).

## IX. Closing

The rusty-bun runtime, as the engagement has constructed it, is sixteen interconnected pipelines whose stages compose by typed data-flow into a directed acyclic graph, whose cross-pipeline alphabet intersections form a lattice in substrate-dependence partial order, and whose joint composition against the constraint pattern *"behave as a JavaScript runtime on real npm code"* is the SIPE-T derivation the engine instantiates.

The engagement's structural recognition arc — Doc 714 §VI's Consequence 5 (event-loop architecture) → Consequence 6 (sub-agent discipline) → Consequence 7 (pipeline empirical landing) → Consequence 8 (substrate-uncovering inventory) → Consequence 9 (saturation state) → Consequence 10 (tail-shape pivot) → Consequence 11 (top-of-alphabet conjecture) → Doc 720 (DAG / lattice / alphabet topology over the engine substrate) — converges on a reading of *what the engine is*, not just *what it does* or *how it was built*. The engine *is* the SIPE-T composition of its pipelines; what makes it a coherent engine is the lattice structure of its alphabets; what makes it correctable at any specific gap is the locatability of the alphabet top within the DAG.

Forward substrate moves now operate against this reading. Each move walks the cross-pipeline DAG to find its alphabet top; each move's predicted-vs-actual delta tests the locatability claim; each move's residual contributes to the engagement's second deliverable — the inventory of contingent decisions a JavaScript runtime engine necessarily makes. The first deliverable is the engine; the second is the topology under which the engine cohere; Doc 720 names the topology.
