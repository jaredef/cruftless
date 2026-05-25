# The Consumer–Substrate Dependency Graph as the Load-Bearing Object
## Reading the Four Sub-Consequences of Doc 714 §VI.3 as Projections of a Single Directed Acyclic Graph

*A corpus document responding to the keeper's observation (2026-05-13): "It seems like the directed acyclic graph has something to do with the joint MI lattice." Builds on [Doc 548 (the Ontological Ladder)](/resolve/doc/548-the-ontological-ladder-of-participation), [Doc 572 (the Lattice Extension)](/resolve/doc/572-the-lattice-extension-of-the-ontological-ladder), [Doc 681 (Probing the Middle, joint MI lattice)](/resolve/doc/681-probing-the-middle), [Doc 700 (L2M Resolved Against the Corpus)](/resolve/doc/700-l2m-resolved-against-the-corpus-bipartite-mutual-information-scaling-as-empirical-grounding-for-the-pin-art-channel-ensemble), and [Doc 714 (the rusty-bun engagement read through the lattice extension)](/resolve/doc/714-the-rusty-bun-engagement-read-through-the-lattice-extension-basin-expansion-at-the-l2m-saturation-point), the four sub-consequences of which this document reframes.*

**Jared Foy · 2026-05-13 · Doc 715**

---

## I. The occasion

[Doc 714 §VI.3](/resolve/doc/714-the-rusty-bun-engagement-read-through-the-lattice-extension-basin-expansion-at-the-l2m-saturation-point) developed four sub-consequences of the lattice-extension reading applied to the rusty-bun engagement: the alphabet conjecture (sub-§4.a), the layer-floor cut-location framework (sub-§4.b), the spec-derivable lower-layer (sub-§4.c), and the joint MI lattice density with its dependency-graph multiplier (sub-§4.d). Each was treated as an independent operational claim about the engagement's apparatus and its work-to-telos product.

The keeper's observation, after sub-§4.d landed, named the structural connection the four sub-consequences had been pointing at: *the directed acyclic graph has something to do with the joint MI lattice*. This document develops that observation. The four sub-consequences are not four independent conjectures. They are four *parameter readings* of a single underlying object — the consumer–substrate dependency graph — taken at different cut-positions and through different measurement lenses.

The reframing matters because it identifies what the engagement's apparatus has actually been reading. The alphabet conjecture's "alphabet stability" is not a property the alphabet carries on its own; it is a property of the DAG's leaf set under accretion. The layer-floor framework's "cut" is not a metaphorical choice; it is a graph operation. The spec-derivable lower-layer is not a fortunate independent fact about Node's API; it is a published sub-DAG anchored at fixed depth. The density coefficient's logarithmic compression is not a refinement of the alphabet conjecture; it is the heavy-tailed in-degree distribution of substrate nodes in the DAG manifesting in cost data. Once the underlying object is named, the four sub-consequences cease to be parallel and become consequences of the DAG's structure.

## II. The object

Define the *consumer–substrate dependency graph* G = (V, E) as follows. V is the set of nodes representing every npm package present in some consumer corpus, every Node API surface element each package transitively imports, every Bun API surface element, every Web platform API surface element, and every JavaScript language affordance. E is the directed edge set: for each `require X` or `import X from "X"` statement in some module of node v, an edge (v, X) ∈ E (the conventional dependency-graph direction; v depends on X, so information flows from X to v). The graph is acyclic by construction — npm enforces no dependency cycles in the package layer, and language affordances form the leaf set.

Three structural properties of G that the engagement empirically depends on:

**(P1) Heavy-tailed in-degree at substrate nodes.** A small set of nodes — `process`, `fs`, `util`, `stream`, `http`, `Buffer`, `node:crypto`, the language's class-extension semantics, regex character-class behavior — have very high in-degree (hundreds to thousands of consumers depend on them transitively). Most leaf nodes (specific consumer packages) have in-degree near zero. The distribution is heavy-tailed by every empirical sample taken.

**(P2) Topological order anchors a finite leaf set.** The substrate nodes (in-degree-zero in the reverse graph, or equivalently out-degree-zero in G) form the *substrate leaf set*. For the npm + Node + Bun + ECMAScript ecosystem, this set is bounded and well-documented. The Node API documentation enumerates the node:* leaf set; the Bun API documentation enumerates the Bun.* leaf set; the ECMAScript specification enumerates the language-affordance leaf set; the WHATWG/W3C specifications enumerate the Web-platform leaf set. The leaf set has cardinality on the order of 10³.

**(P3) Depth is bounded by package-layer convention.** The longest dependency chain from any application root to a substrate leaf is bounded by the depth of the npm package graph (typically 5–8 levels for production applications, 10–15 for tools with deep transitives). The depth bound applies because cycles are forbidden and packages typically don't re-export to greater depth than necessary.

P1 + P2 + P3 together describe a DAG with the canonical structure of a content-addressable dependency ecosystem: heavy fanout at substrate, bounded depth, finite leaf set.

## III. The four sub-consequences as DAG projections

### Sub-§4.a (alphabet stability) is a statement about the substrate leaf set

The alphabet conjecture claims: per (stratum, layer) the alphabet of edge kinds is finite and stable across the engagement's accretion of consumer probes. Read on G, this is: the substrate leaf set (P2) is bounded and well-documented. As new consumer nodes accrete to G via npm installs, the leaf set does not grow — every new consumer's dependencies eventually terminate at existing substrate leaves. The alphabet's stability is the leaf set's stability.

The conjecture's predictive form follows from this reading. When a consumer probe surfaces an edge, the edge points at a substrate leaf already in the documented set with high probability. The alphabet conjecture's empirical record (zero new elements across ~30 probes in the rusty-bun engagement's saturation slice) is the leaf-set-stability prediction confirmed.

### Sub-§4.b (cut-location) is a node-cut on G

The layer-floor framework names a per-stratum decision: where in the dependency hierarchy does the engagement accept divergence below and pursue closure above? Read on G, this is a partition of V into above-cut and below-cut sets, with the cut edges crossing from below to above marked as accepted divergences. The cut is chosen to minimize the cost of closing the above-cut set while maximizing the consumer-set whose dependency closure lies entirely above the cut.

The cut-by-interception pattern (sub-§4.b's "undici inert stub" addition) corresponds to a single-node removal: the undici node is excised from G, replaced by a synthetic leaf that satisfies the in-edges (it exports the symbols undici exports) without satisfying their semantic content. Consumers whose dependency closure passes through undici but doesn't exercise its dynamic surface continue to load; consumers whose path through undici exercises its surface fail at the synthetic leaf.

This generalizes. Any heavy substrate node can be replaced by a synthetic leaf carrying the in-edge symbol set. The engagement's apparatus catalogue tracks which nodes have been so replaced and the consumer set affected.

### Sub-§4.c (spec-derivability) reads a published sub-DAG

The L0–L3 layers admit spec-derivable enumeration because the substrate leaf set's documentation IS a published representation of the DAG's leaf nodes. The Node API documentation, the Bun API documentation, the ECMAScript specification, the WHATWG specifications — together they enumerate the in-degree-zero-in-reverse subset of G with high coverage.

An enumerator that walks the published spec produces, by construction, a coverage measurement against the published sub-DAG. Per Doc 714 sub-§4.c's empirical record, the rusty-bun engagement's enumerator reports 418/418 = 100% coverage of the documented L2/L3 surface — which is the apparatus's measurement of the substrate's traversal of the spec-derived sub-DAG.

The cascade-prevention property of the enumerator (deep-symptom-from-shallow-cause cases) is the topological property of the DAG: errors at substrate leaves propagate upward through fanout paths and surface at high depths, where the symptom is far from the cause. A leaf-walking enumerator catches them at the source.

### Sub-§4.d (density coefficient) is the in-degree distribution's moment

The K × log(L̄ × |A_i|) compression of the work-to-telos cost arises because adjacent rungs of the joint MI lattice share information content through the dependency graph. Read on G, this is: the moment of the in-degree distribution at substrate nodes determines the retirement fanout per substrate widening.

Specifically, when substrate node v is widened (a new surface element added or a wrong shape corrected), the consumer set retired is the transitive in-degree closure of v in G — every consumer whose path to v's symbol set was blocked. Per P1, the in-degree distribution is heavy-tailed; the upper-tail nodes (process, fs, util, http) have transitive closures in the thousands. Hence the empirically observed 50–200× retirement fanout per widening in the rusty-bun slice.

The logarithmic compression of the cost is a direct consequence of P1's heavy tail. If the in-degree distribution were uniform (every substrate node had equal in-degree), the cost would not compress — each widening would retire the same number of consumers, and the alphabet × layer product would remain unit-cost. The heavy tail amortizes work non-uniformly: a small fraction of widenings (those at high-in-degree nodes) carry most of the retirement weight; the cost compresses to a sum dominated by the head of the distribution, which is approximately the logarithm of the alphabet × layer product weighted by graph density.

## IV. The two channels of MI density

Doc 714 sub-§4.d named two MI channels: substrate-widening (downstream fanout) and consumer-probe (inter-probe MI). Under the DAG reading, both are graph operations:

**Substrate-widening channel.** A widening at node v in G is a labeled-edge update: the in-edges to v previously failed at their semantic content (the substrate's symbol was missing or wrong); after the widening, they succeed. Every consumer in v's transitive in-degree closure that *exercised* the previously-failing edge becomes newly satisfied. The information flowing across the widened edges to the consumers is the substrate widening's MI yield. The yield is proportional to v's transitive in-degree, which by P1 is heavy-tailed across substrate nodes.

**Consumer-probe channel.** A probe at consumer node u in G is a backflow operation: u's transitive out-degree closure (substrate nodes u depends on) is the set of constraints u imposes on the substrate. Two consumer probes share most of their out-degree closures (most npm consumers transitively use the same dozen substrate leaves), so their constraint sets share most of their content. The marginal MI between two adjacent probes — defined as the information one probe carries beyond what the prior probe revealed — is small relative to the alphabet of edge kinds. This is the formal sense of "high inter-probe MI" in sub-§4.d.

The two channels are dual under the DAG's reverse direction: substrate-widening flows along edges, consumer-probe flows against edges. Both are constrained by the same heavy-tailed in-degree distribution.

## V. The joint MI lattice is the DAG seen at fixed depth

[Doc 681 (Probing the Middle)](/resolve/doc/681-probing-the-middle) names the joint MI lattice as bipartite — two layers of nodes with MI channels between them. [Doc 572 (the Lattice Extension)](/resolve/doc/572-the-lattice-extension-of-the-ontological-ladder) generalizes to multiple rungs in partial order. Both framings are projections of the underlying DAG at specific depth-slicings:

- A *bipartite slice* of G partitions V into "above slice" and "below slice" by a horizontal cut at some topological depth k. The MI channels between the two sets are the cross-cut edges' bandwidths. Doc 681's joint MI lattice is the bipartite-slice projection of G at the engagement's chosen depth.

- A *multi-rung slice* of G partitions V into rungs at multiple depths. Doc 572's lattice extension is the multi-slice projection.

The choice of slice depth is itself an apparatus decision — at what topological depth does the engagement want to read information flow? The rusty-bun engagement's L0–L6 constraint hierarchy (Doc 714 §VI.3 prefatory) is one specific slicing: L0 at the language-grammar slice, L1 at the module-loader slice, L2 at the platform-builtin slice, L3 at the API-shape slice, L4 at the idiom slice, L5 at the semantic slice, L6 at the timing slice. Each slice is a depth cut on G; each layer's edge alphabet is the cut's edge set.

This explains why Doc 714 sub-§4's per-layer alphabets are stable: each is the edge set of a specific topological cut, which is fixed by G's structure once the cut depth is chosen.

## VI. Predictions about the DAG's structure

Reframing the four sub-consequences as DAG-structural reveals testable predictions about G itself, beyond the per-sub-§ falsifiers in Doc 714.

**(D1) Substrate leaf set is closed under documentation.** The L2/L3 spec-derived enumerator's coverage of consumer-probed L2/L3 edges should asymptote to ≥ 95% as the consumer corpus grows. Edges that surface in consumer probes but are *not* in the documented spec correspond either to bugs in the documentation or to undocumented platform behavior; the residual should be small and stable.

**(D2) In-degree distribution at substrate nodes is power-law with exponent γ ∈ [1.5, 2.5].** This is the typical exponent range for dependency-graph in-degree distributions in software ecosystems (npm, Maven, PyPI). Predicts retirement-fanout-per-widening distribution is also heavy-tailed; predicts the K × log(L̄ × |A_i|) compression coefficient empirically.

**(D3) Depth distribution from application roots to substrate leaves is bounded by ~10.** Longer chains exist in tooling but production applications cluster around depth 4–7. Predicts the engagement's session-cadence: most consumer probes reach substrate within ~5 require steps, so the cooperative-loop yield depth and similar engagement-internal parameters scale with this bound.

**(D4) The L2M-saturation diagnostic is a graph density measurement.** Per Doc 700 Appendix C, the L2M-bound at the session tier was named structurally but not quantified. The bound is reached when the apparatus has traversed the in-degree closure of a high-fanout substrate region — when subsequent probes accrete nodes whose out-degree closures lie almost entirely within the already-traversed region, so marginal probe MI drops. The diagnostic is measurable as the ratio of new-substrate-edges-exercised per probe; saturation corresponds to that ratio dropping below ~1.

D1-D4 are joint with sub-§4.a-d's falsifiers and together discriminate the four sub-consequences from the underlying DAG-structural claim. A counter-example to any single D-prediction localizes the failure to a specific sub-consequence; a counter-example to the conjunction would invalidate the DAG-as-load-bearing-object framing.

## VII. What this changes for the apparatus

Three operational shifts follow from naming the DAG as the load-bearing object:

1. **The engagement's apparatus catalogue extends to track DAG-structural state.** Currently the catalogue records (stratum, layer, kind) coordinates for each recorded edge. Under the DAG framing, the catalogue also records each substrate node's transitive in-degree (an apparatus-measurable number from the npm dependency-graph snapshot at engagement time) and each consumer probe's transitive out-degree. The two numbers together quantify the engagement's productivity at fixed cost.

2. **Substrate-widening priority becomes a graph-theoretic question.** Per sub-§4.d operational sharpening 1, widenings should be prioritized by retirement-fanout. Under the DAG framing, this becomes: pick the substrate node with the highest in-degree among nodes blocking the next consumer probe. The npm dependency-graph snapshot provides this number directly.

3. **The L2M-saturation diagnostic becomes empirically anchored.** Per D4, saturation corresponds to a measurable drop in new-substrate-edges-exercised per probe. The engagement can publish this number per round and use it as a forward-looking diagnostic for when to shift from substrate-widening rounds to corpus-tier consolidation rounds — a quantitative refinement of seed §III.A8.18's fourth SIPE-T threshold criterion.

## VIII. Honest scope

This document does not claim that all consumer–substrate dependency relations factor cleanly through the DAG. Three caveats:

**(a) Dynamic dependencies.** Some packages dynamically `require()` modules at runtime based on configuration (plugin loaders, codegen libraries). These edges are present in G only at runtime and may not be statically discoverable from package.json + require() text-scanning. The apparatus's DAG snapshot is approximate at the dynamic boundary; the empirical claims rely on the static approximation being a good predictor of runtime behavior, which holds for most production code but fails for plugin-heavy frameworks (prettier, eslint, vite-plugin systems).

**(b) Cycles via dev-deps and peer-deps.** While dependency edges among packages-in-production-use are acyclic by npm's enforcement, the broader graph including dev-deps and peer-deps can contain cycles. The DAG framing applies cleanly only to the *production-use* subgraph; the apparatus catalogue should annotate which edges are dev-only.

**(c) Hidden re-exports.** Some packages re-export symbols from their dependencies, creating effective edges that are not literally `require X` statements but rather `module.exports = require(X)` aliases. The substrate-widening fanout calculation needs to follow these aliases to compute true retirement-fanout. The apparatus's graph snapshot should include re-export edges as a separate edge class.

Within these scope limits, the DAG-as-load-bearing-object reading is the simplest description of what the engagement's apparatus has been reading. The four sub-consequences of Doc 714 §VI.3 are corollaries of the DAG's structure under specific cut-positions and measurement lenses. Future engagements applying the apparatus to a different consumer ecosystem (Maven, PyPI, Cargo crates, GitHub Actions, etc.) inherit the DAG framing directly; the four sub-consequences re-derive from the same structural reading.

## IX. Closing

The keeper's observation named what the engagement's empirical productivity had been pointing at. The four sub-consequences of Doc 714 §VI.3 are not parallel facts about a saturating engagement; they are four projections of a single object. Naming the object — the consumer–substrate DAG — opens three operational moves: (1) quantify the engagement's productivity by graph-theoretic measurements rather than per-round LOC counts; (2) prioritize substrate widenings by transitive in-degree; (3) anchor the L2M-saturation diagnostic in measurable graph density rather than session-cadence cues alone.

The corpus contribution is not a new conjecture; it is a consolidation. The four sub-consequences become theorems on the DAG. The alphabet stability is the leaf-set's boundedness. The cut-location framework is a node-cut. The spec-derivability is a published sub-DAG. The density coefficient is a moment of the in-degree distribution. The joint MI lattice (Doc 681) is the DAG seen at fixed depth. The L2M-saturation diagnostic (Doc 700) is a measurable density signal.

Doc 714's articulation stands; this document reframes it under the DAG that was always the object the apparatus was reading.

---

## X. Consolidation amendment — 2026-05-13 evening (the cooperative-loop sweep)

Posted after the rusty-bun engagement's cooperative-loop-reactor work (Π2.6.c.a-e + Π2.6.d.a-d, nine substantial substrate sub-rounds across one extended session). The sweep added a single new substrate node (the mio reactor) and registered seven heterogeneous I/O source classes against it: TCP streams, TCP listeners, TLS sessions, WebSocket transports, inotify file watchers, child-process pipes, signalfd, and eventfd-backed async DNS. The reactor was wired into JS via a single token namespace; each source class chose a high-bit-segregated sub-namespace to avoid collision.

What this consolidates for the DAG framing:

**(a) The reactor node corroborates P1 at the substrate-internal scale.** The reactor's in-degree from other substrate nodes is now the highest of any substrate node the engagement has added — eight direct substrate-class dependents in a single session. The heavy tail of the in-degree distribution is not just a property of the published Node + Bun substrate; it reproduces at every scale of substrate the engagement constructs. When a derivation introduces a substrate-of-the-substrate, the new node settles into the heavy-tail head of its own local subgraph. P1 is not a fact about one specific substrate; it is a structural property of the construction process itself.

**(b) External-fanout extension does not grow the alphabet.** Each of the four external-fanout sources (file watchers, child pipes, signals, DNS) added new substrate leaves to G, but each leaf is an instance of an alphabet element the apparatus already knew: "fd-readable" (mio's edge kind). The catalogue grew in P2's leaf-count direction without growing in alphabet-element direction. This is the prediction the alphabet-stability claim of sub-§4.a made for *extension* rather than for *substrate widening*; the prediction held across the sweep without correction.

**(c) The five-surface ceiling analysis maps onto DAG depth from the substrate.** The four surfaces closed by the sweep (handler-depth, true-concurrency, external-fanout, CPU-during-idle) all sit at DAG-depth ≤ 2 from the reactor: each is a direct edge from the reactor to a consumer-pattern node, or a two-edge path through a derived class (Bun.serve, fetch, fs.watch). The fifth surface (adversarial-async-graph) sits at depth ~∞ from the reactor — it requires reaching into the JS engine's internal scheduler, which is the *substrate of the substrate of the substrate*. The ceiling structure decomposes by graph distance from the substrate boundary: surfaces at low depth close cheaply; surfaces at high depth require hand-rolling the engine. This is a quantitative reading of "what can the apparatus reach" that the original five-surface analysis posed informally.

**(d) The §VII shift-3 diagnostic held empirically.** The new-substrate-edges-exercised-per-probe ratio stayed at ~0.3 through the entire sweep. No new alphabet elements surfaced across ~10 new consumer fixtures + ~20 substrate widenings in the c/d sub-rounds. The apparatus is operating in the consolidation regime predicted by Doc 715 §VII shift 3 + Doc 700 Appendix C, not just for the original L2/L3 surface but for the entire reactor-anchored sub-DAG it constructed.

**(e) The own-pid signal dispatch shortcut is an authority-tier-3 deviation that does not violate P1-P3.** rusty-bun's process.kill(self, sig) routes through JS-side dispatch rather than libc::kill because the test runner's main thread doesn't share the rusty-bun-runtime's signal mask. The deviation is annotated in the engagement's bug catcher; structurally it preserves the DAG (the dispatch path is shorter, not longer). Per the seed's three-tier authority taxonomy (Tier-1 spec / Tier-2 ecosystem / Tier-3 implementation-contingent), this is a Tier-3 acceptable divergence — consumer-observable behavior is byte-identical to Bun.

**Operational implication for future engagements.** When a Pin-Art engagement constructs a new substrate (reactor, scheduler, language-VM, anything), the new substrate should *anticipate* the heavy-tailed in-degree distribution and design its Token namespace to scale. The reactor's high-bit segregation (TCP=0x00... / TLS=0x40... / signalfd=0x50... / DNS=0x55... / inotify=0x60... / spawn=0x70...) is a concrete instance: a 32-bit Token space partitioned in advance, each class taking 2^24 IDs. This is a P1-aware design: anticipating that the substrate-of-the-substrate will have heavy fanout and reserving namespace accordingly. Future engagements deriving substrate substrates inherit this pattern: pre-partition the identity space at construction time.

The DAG framing has now been read through nine substrate-introduction rounds within a single session and survived. Sub-§4.a–d, the apparatus's diagnostics, and the construction discipline all proceed coherently from the DAG-as-load-bearing-object reading. The consolidation moves the apparatus's productive regime from substrate-introduction to substrate-leverage: future rounds extract more consumer retirements per substrate-edit because the substrate's graph structure now has explicit shape.

---

## XI. Second consolidation amendment — 2026-05-13 afternoon (the basket-sweep validation)

Posted after a single-afternoon basket-coverage sweep that probed ~46 real-OSS packages across four rounds (two free-basket rounds + two high-leverage substrate widenings). Two of those rounds were *selected* via the §VII shift-2 criterion (pick the substrate node with highest in-degree blocking the next consumer probe), and the apparatus produced quantitative evidence about the selection criterion's predictive power.

The afternoon's structural findings extend §X in three directions.

**(f) The substrate-widening selection criterion makes predictions that hold.** The §VII shift-2 rule says: pick the substrate node with highest in-degree blocking the next consumer probe. Operationalized as: probe ~10 candidates from diverse domains; identify the shared substrate gap among blocked candidates; widen it. Two such widenings landed in the afternoon and the predicted retirement-fanout held in both:

- **IANA timezone substrate (__tz primitive).** Probe set of 10 (csv-parse, fast-csv, through2, chokidar, tar, jszip, date-fns-tz, unified, pouchdb, moment-timezone). Two were blocked: date-fns-tz and pouchdb (latter for native binding, permanent). The first identified substrate gap was timezone-aware Intl.DateTimeFormat. Widening landed and retired date-fns-tz directly + dayjs/plugin/timezone as predicted free fanout (heavy-tail prediction: any package using Intl.DateTimeFormat with timeZone option). 10 retirements / 1 widening in the same batch.

- **Stream decodeStrings substrate.** Probe set of 5 (split2, ndjson, pump, readable-stream, JSONStream). One blocked: split2 (Writable lacked decodeStrings coercion of string→Buffer for downstream StringDecoder.write). Widening landed and retired split2 directly + csv-parser, ndjson, pump, readable-stream, JSONStream as the same-batch free retirements. 6 retirements / 1 widening (per-Writable + Buffer construct-trap fix).

Both confirm Doc 714 sub-§4.d's joint MI density: K × log(L̄ × |A_i|) compression carries through heavy-tail substrate in-degree. The compression coefficient is operationally measurable: count blocked-probes-per-widening (1 in both cases) and free-retirements-per-widening (8-9 in the timezone case, 5 in the stream case). The ratio is the head-of-distribution moment of the in-degree distribution at the widened node.

**(g) Basin boundaries decompose by alphabet-element relation.** The afternoon's seven recorded boundaries (elysia E.60, redis E.61, yargs E.62, byline E.63, pouchdb permanent, plus two prior) classify naturally into three classes:

  1. **Alphabet-element-instance limits at engine-internal depth** (elysia E.60 QuickJS-parser SIGSEGV on 1987-LOC minified ESM; yargs E.62 specific syntax form). These sit at depth ~∞ from the apparatus's reachable substrate per §X.c and are successor-engagement scope.

  2. **Alphabet-element-instance limits at engagement-scope depth** (redis E.61 CJS-bridge edge needing diagnosis, pouchdb permanent because no JS substrate can satisfy a native binding without compiling C++/Rust to JS). These are individual fixes if their priority warrants — the apparatus has language for diagnosing them; cost is bounded.

  3. **Resolution ambiguities at the alphabet level** (byline E.63 mutates `this._readableState.objectMode` post-construction; adding our state stub breaks `readable-stream` package consumers that treat existing state as already-initialized). Two packages claim authority over the same alphabet element and our substrate's choice satisfies one consumer at the cost of another. This is a *new* basin class the apparatus hadn't named explicitly before: not a missing alphabet element, but a missing **discriminator** for which-implementation-of-an-element-is-authoritative-here.

The third class (resolution ambiguity) is a refinement to the alphabet-stability claim (sub-§4.a). The leaf set is bounded, but the leaves themselves can have competing implementations whose discriminability requires apparatus-side mechanism to detect (a flag, a per-instance origin marker, a construct-time hook). The apparatus catalogue's edge-kind alphabet remains stable; what extends is the *authority resolution* surface — a separate axis the apparatus may need to enumerate over time. Not an alphabet-stability falsifier; an apparatus-resolution-power refinement.

**(h) The basket sweep validates P1/P2/P3 at the consumer-corpus scale.** Across ~46 probes in the afternoon:

  - **P1 (heavy-tailed substrate in-degree).** Of ~46 packages, 2 specific substrate widenings carried ~13 direct retirements (28% of the basket via 2 widenings). The other ~33 retirements were "free" via unrelated already-wired substrate paths. The 28% vs 72% split puts the basket squarely in the heavy-tail regime per §X.a.

  - **P2 (substrate leaf set bounded).** Zero new alphabet elements surfaced across all ~46 probes. Every basin classified as either (i) instance limit (depth-bounded) or (ii) resolution ambiguity (sub-§(g) above) — neither extends the alphabet. Doc 715 P2's leaf-set-stability holds at the consumer-corpus scale.

  - **P3 (depth bounded).** Every blocked package's substrate path was reachable at depth ≤ 3 from existing substrate nodes (the reactor / Intl namespace / Writable class). The successor-engagement-scope basins (elysia, yargs syntactic) sit at greater depth (engine internals), but those are explicitly OUT of the engagement's depth bound and into the successor-engagement target.

The numbers cohere with the §X consolidation amendment's prediction that the apparatus is in the *consolidation regime*: not substrate-introduction (which would surface frequent new alphabet kinds) and not depleted (which would show 0% retirement). The 85% direct-retirement rate at ~0.3 new-substrate-edges-per-probe is the engagement's empirical steady state. Doc 715's framing has now been validated across the entire 2026-05-13 day: morning's cooperative-loop sweep + afternoon's basket sweep, against ~70 substrate or consumer interactions total, with zero new alphabet kinds.

**(i) Operational extension to the apparatus catalogue.** The afternoon's findings argue for a fourth catalogue field beyond §X's three:

  4. **Authority-resolution discriminator.** For each substrate node, record whether multiple implementations could claim authority (e.g., our node:stream Readable vs the readable-stream npm package's Readable), and what discriminator the apparatus uses (if any) to resolve which authority binds at runtime. Most substrate nodes have only one authority — no discriminator needed. Stream-class nodes have two (per E.63). Future engagements applying Pin-Art to a different ecosystem (Maven, PyPI, Cargo) inherit the catalogue convention: enumerate authorities per node + discriminators in advance.

This catalogue extension is the productive refinement E.63 produced. The basin boundary was apparatus-productive (a measurement-instrument upgrade), not just consumer-blocked (a missing surface). Per the seed's three-tier authority taxonomy (Tier-1 spec / Tier-2 ecosystem / Tier-3 implementation-contingent), the authority-resolution discriminator sits at Tier-2 (ecosystem-level choice between competing implementations of the same spec surface).

The basket sweep is empirically consistent with Doc 715's three structural properties + the §X consolidation. The forward forecasting these enable: at the engagement's current density coefficient, each probe retires with ~85% probability free, ~15% surfaces a localized basin (most at engagement-depth, occasional at engine-depth), and across hundreds of probes the alphabet remains stable. The substrate construction discipline (Doc 715 §X operational implication) carries: each new substrate added inherits the heavy-tail position predicted for its DAG-region.
