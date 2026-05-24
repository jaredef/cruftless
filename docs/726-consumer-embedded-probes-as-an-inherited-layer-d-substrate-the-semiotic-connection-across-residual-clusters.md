# Consumer-Embedded Probes as an Inherited Layer-D Substrate

## The Semiotic Connection Across the Residual Clusters at 78–84% Load-Rate

By Jared Foy. Originally published at [jaredfoy.com](https://jaredfoy.com).

## I. The occasion

The rusty-bun engagement reached 84.7% load-rate at the 614-package basket on 2026-05-16, then 78.8% across two further broadenings to the 846-package basket. Across thirty-four substrate moves on that day, a structural feature became visible in the residual failure clusters that the corpus has not yet named.

The keeper directed the recognition: *"Review the five most recent corpus docs for a meta move that discovers the semiotic connection between remaining clusters."* This document is that meta-move. The recognition formalized here is general: **the residual at this maturity is not a random distribution of feature-gaps, surface-misses, or apparatus bugs. It is the imprint of consumer-embedded probes — verifications the npm corpus authors wrote into their source for their own development environments — meeting the engine's substrate-production at the boundary where production-correctness becomes consumer-detectable.**

The framework draws directly on [Doc 723](/resolve/doc/723-diagnostic-tags-as-semiotic-signs-layer-indexed-interpretation-in-pipeline-dag-topologies) (diagnostic tags as semiotic signs; Layer A/B/C/D interpretation), [Doc 721](/resolve/doc/721-the-cross-pipeline-diagnostic-protocol-locating-the-top-of-a-substrate-widenings-alphabet-by-walking-the-engines-dag) (cross-pipeline diagnostic protocol; Step 6 route-(b)), [Doc 724](/resolve/doc/724-feature-set-prediction-static-substrate-need-mapping-from-source) (forward predictor with bright/blind zones), and [Doc 725](/resolve/doc/725-the-cluster-to-walk-mode-transition-soft-saturation-as-protocol-signal-in-substrate-introduction) (cluster/walk mode transition). The five-doc sequence converges on the recognition this document names.

## II. The recognition

Three claims, in order from operational to structural.

**Claim 1 (operational).** Every residual cluster at the 78–84% band's residual surfaces its fault at a *consumer-internal probe site* — an assertion, a feature-detection check, a shape verification, an accessor-descriptor read, an arithmetic check — that the package's own source wrote for its own development. The engine produces values; the probe tests them; the probe's failure produces the fault tag.

**Claim 2 (compositional).** The set of consumer-probe shapes is small and bounded, and each shape surfaces a particular *class* of engine-substrate divergence. Together they form an inherited substrate that the engagement gets for free — every package the corpus admits expands the inherited Layer-D, every probe activates a slice of the engine's production envelope.

**Claim 3 (structural).** Doc 723's Layer D — constructive probe-substrate — was articulated as the substrate-introducer's investment, built explicitly via tools like `host/tools/probe-builder.sh`. The corpus's npm packages carry their *own* Layer-D probes built into their distributed source. The substrate-introducer inherits this corpus-side Layer-D the moment a package enters the basket; the inherited probes are what make the residual's faults semiotically legible.

The three claims form the standard testability hierarchy. Claim 1 is testable on any single residual cluster by inspecting the source at the fault site. Claim 2 is testable by classifying enough clusters to enumerate the probe shapes. Claim 3 is the structural reading; it is testable by comparing the residual's legibility against the engagement's pre-broadening basket (when fewer probes were inherited).

## III. The five probe-shapes the residual exhibits

Across thirty-four substrate moves on 2026-05-16 and the residual at 846 packages, five distinct consumer-embedded probe-shapes account for nearly all the named clusters.

### III.a. Shape-correctness assertion

The package's source asserts that a value produced by the engine has a specific shape, and the assertion fails when the engine's production is structurally wrong.

- color-convert's `route.js` walks a graph built from `Object.keys(conversions[current])`. The engine's `Object.keys` had been returning non-enumerable properties (because `defineProperty` defaulted `enumerable:true`). The probe surfaces at `graph[adjacent].distance` — `graph[adjacent]` is undefined for the polluted 'channels' / 'labels' keys.
- safe-stable-stringify (under roarr, slonik, mongoose) caches `Object.getOwnPropertyDescriptor(TypedArrayProto, Symbol.toStringTag).get`. Our `getOwnPropertyDescriptor` dropped accessor descriptors; the cached `.get` was undefined. The probe surfaces at the next runtime `.get.call(value)`.
- jsonwebtoken / lodash use `Function.prototype.toString.call(value)` for fingerprinting. Engine's FPT threw on non-functions where Node/V8 returns synthetic strings; the probe surfaced at the throw.

The shape-correctness assertion is the most numerous probe-shape in the residual. The engine's deviations from spec attribute-defaults and accessor-descriptor handling were both made visible by this probe class.

### III.b. Arithmetic verification

The package computes a value mathematically and verifies the computation's invariants. The engine's numeric production is wrong; the assertion catches it.

- elliptic (under ethereumjs / secp256k1) initializes the secp256k1 curve and asserts `this.g.validate()` — the generator point should be on the curve. Our BigInt math produces some value, but the curve equation doesn't hold. The probe surfaces at `Error: Invalid curve`.
- The `Invalid curve, G*N != O` follow-up assertion verifies the cyclic group order. Same root.

This probe class targets engine-internal arithmetic. Without it, the engine could compute wrong numbers silently for the entire module-init and produce broken instances downstream. The probe is what makes the failure surface at module-init rather than at signature-verification call time.

### III.c. Feature presence probe

The package reads a global or imported entity and conditions further code on its existence.

- `typeof process.execArgv === 'object'` / `process.execArgv.length` (resolve-package-path / many).
- `if (Error.captureStackTrace) { ... }` (depd's getStack helper, used by koa / serve-static / http-errors).
- `typeof http.ServerResponse.prototype.appendHeader === 'function'` (compression).
- `Symbol.toStringTag` read at module-init for branding logic (multiple packages).

This is the probe-shape closest to what Doc 724's *forward predictor* (v1) reads — a token-greppable check that the engine has a named feature. Closing this probe-class is largely surface-installation work.

### III.d. Dispatch fingerprint

The package fingerprints a value's identity by dispatching on it through prototype chains, then routes downstream behavior based on the result.

- dequal: `(ctor = foo.constructor) === bar.constructor` then `ctor === Date` / `ctor === RegExp` / `ctor === Array`. The probe surfaces if the constructor reference is mis-wired.
- node-fetch's `class Headers extends URLSearchParams` and downstream `URLSearchParams.prototype[p].call(target)`. The probe surfaces at the missing prototype-method.
- Babel's `for (var _iter = ..., _iter = _isArr ? _iter : getIterator(_iter); ...)`. The probe surfaces if the declarator-list dedupe is wrong.

This probe class is what Doc 724 §XI.b's *blind zone* sees only through backward trace. The fingerprint sits at the *interaction* of multiple correctly-implemented features; the predictor cannot see the conjunction in source.

### III.e. Runtime instrumentation imprint

The package writes its own diagnostic instrumentation — error capture, performance marking, request-context tagging — and the engine's stub silently produces wrong shape.

- `Error.prepareStackTrace = (err, frames) => frames; Error.captureStackTrace(obj); callSites[1].getFileName()` — depd, multiple Node tooling.
- `new PerformanceObserver(cb).observe({entryTypes:['measure']})` then `performance.measure(...)` — nx and others.
- `class X extends AsyncResource` then `runInAsyncScope(...)` — undici, fastify.

The probe class targets *the engine's own diagnostic surface*. The substrate-introducer's enrichment moves (Doc 723 route-(b)) operate at the same layer that consumers' instrumentation probes operate at — the meta-substrate of diagnostic emission. When consumers wrote their own instrumentation, they assumed a particular emission shape; when the engine's stub deviates, the consumer's downstream walk lands on undefined.

## IV. What the probe shapes connect

The five probe-shapes do not bisect the residual into disjoint clusters. They overlap in two structurally significant ways.

**Overlap 1 — probe-shape × engine-pipeline.** Each probe activates one or more of [Doc 720](/resolve/doc/720-the-rusty-bun-runtime-as-a-dag-of-interconnected-pipelines-sipe-t-topology-over-the-engine-substrate)'s sixteen pipelines. A shape-correctness assertion (III.a) reading `Object.keys(...)` activates the property-descriptor pipeline; an arithmetic verification (III.b) activates the runtime pipeline plus the value-semantics arithmetic substrate; a dispatch fingerprint (III.d) activates the prototype-chain-dispatch pipeline.

A residual cluster at the intersection of probe-shape III.a and the property-descriptor pipeline (color-convert) has its alphabet top at the descriptor-flags substrate. A cluster at probe-shape III.b and the BigInt arithmetic pipeline (ethereumjs) has its alphabet top at the BigInt op-semantics substrate. **The probe-shape names the row; the pipeline names the column; together they locate the alphabet top.**

This refines Doc 721's Step 3 (locate the highest shared layer): the search isn't only across pipelines but across the *probe-shape × pipeline* product matrix. When the residual is read in this two-dimensional lens, alphabet tops cluster at named intersections.

**Overlap 2 — probe-shape × forward/backward instrument.** Doc 724 §XI named the bright zone (forward predictor catches the gap) and the blind zone (only backward trace catches). The probe-shape determines which side of the zone the fault sits on:

- Shape III.c (feature presence) sits in the bright zone. The forward predictor's source-grep already names every package that reads `Symbol.toStringTag` or `Error.captureStackTrace`.
- Shape III.a (shape-correctness assertion) and III.d (dispatch fingerprint) sit in the blind zone. The predictor sees `Object.keys` and `defineProperty` as feature-present, but cannot see the *correctness* of their interaction.
- Shape III.b (arithmetic verification) sits *deeper* than the blind zone. The forward predictor cannot see arithmetic correctness because no token in source names it; the backward trace catches it only because the consumer wrote the probe.
- Shape III.e (instrumentation imprint) sits at the meta-substrate layer. Both forward and backward instruments can name the surface; closing the surface requires implementing the spec-correct emission, which is itself a substantive substrate move.

This is the document's load-bearing recognition: **the consumer-embedded probe-shape is the layer indexer that locates the residual fault inside the bright/blind/deeper-than-blind taxonomy.** Reading the residual by probe-shape gives the substrate-introducer immediate priority ordering for the next several substrate moves.

## V. The inherited Layer-D substrate

[Doc 723](/resolve/doc/723-diagnostic-tags-as-semiotic-signs-layer-indexed-interpretation-in-pipeline-dag-topologies) §IV's amendment named Layer D — constructive probe-substrate — as the substrate-introducer's investment. The substrate-introducer builds named features into a probe-construction apparatus; the apparatus enumerates combinations; bisects against the named feature-set isolate root causes that Layer-B reading on its own couldn't isolate.

This document names the *complementary* recognition: **every package the corpus admits expands a corpus-side Layer-D that the substrate-introducer inherits at zero cost.** The npm package authors built their own probes for their own defensive coding, polyfill detection, runtime fingerprinting, and arithmetic verification. Those probes are distributed inside the package source. When the engine evaluates the source, the probes fire against the engine's substrate-production. Their fault tags are the inherited Layer-D's signal.

Three operational properties follow.

**V.a. Linear scaling per broadening.** Each broadening of the basket (Doc 725's broadening-as-mode-resetting-operation) adds proportionally to the inherited Layer-D. The 500-package basket's inherited Layer-D surfaces specific engine-divergences; the 846-package basket's surfaces more (and different) divergences. The substrate-introducer's discipline is to *honor what the inherited probes surface* rather than re-derive the named features from spec.

**V.b. Probe-shape stability across engagements.** The five shapes named in §III generalize. Any pin-art-built JavaScript runtime substrate would see the same five probe-shapes surface from any npm sample, because consumers write probes for their own development concerns, not for any particular engine's substrate. The probe-shape inventory is portable across engagements; the engine-side response is engagement-specific.

**V.c. Probe-shape compounds with diagnostic enrichment.** Doc 723 route-(b) enrichment (Ω.5.MMMMMMM in this engagement — receiver-tags on CallMethod) raised the signal level at one engine site; the inherited Layer-D became more legible immediately. Future route-(b) moves (richer tags at GetProp / SetIndex / Op::New emission sites) will compound similarly. The substrate-introducer's meta-substrate investments and the corpus's inherited Layer-D are not adjacent — they multiply.

## VI. The taxonomy applied to today's residual

The 846-package residual after Tier-Ω.5.BBBBBBBB landed exhibits the following probe-shape distribution:

| Probe-shape | Approximate cluster count | Top example | Closed by |
|---|---|---|---|
| III.a Shape-correctness assertion | ~10 packages across 3–4 named clusters | color-convert (Object.keys polluted) | engine-correctness moves (defineProperty defaults, getOwnPropertyDescriptor accessor-shape, real delete) |
| III.b Arithmetic verification | 4 packages (ethereumjs / secp256k1 family) | elliptic curve validation | BigInt op-semantics substrate (deep, deferred) |
| III.c Feature presence probe | ~10 packages across 4–5 named clusters | depd (Error.captureStackTrace) | surface-installation moves (the bright-zone column) |
| III.d Dispatch fingerprint | ~6 packages across 2–3 named clusters | csso / prettier / tinyexec (require('resolve') namespace shape) | module-export-shape correctness (deeper than today's resolver fixes) |
| III.e Instrumentation imprint | ~5 packages | nx (PerformanceObserver + dynamic-import-await) | meta-substrate plus async-pause semantics (heavier; deferred) |

The five probe-shapes account for roughly thirty-five of the named-cluster residual packages. The remaining ~150 packages in the "callee is not callable: undefined" long tail decompose further into the same five shapes once Doc 723 route-(b) tag enrichment surfaces their receiver context.

The taxonomy operationalizes priority. The bright-zone column (III.c) is closeable by surface-installation; today's day produced eleven such moves. The blind-zone column (III.a, III.d) requires structural engine fixes; today produced six such moves (uuuuuu, vvvvvv, yyyyyy, DDDDDDD, WWWWWWW, BBBBBBBB, ZZZZZZZ, AAAAAAAA). The deeper-than-blind column (III.b, partial III.e) requires substantive engine investment — for the BigInt arithmetic, real Promise/await-pause semantics, and bundle-edge module-shape resolution.

## VII. Falsification surface

**Fal-726.1.** The five probe-shapes do not generalize. A different corpus (server-only Node packages, browser bundles, embedded JavaScript) exhibits a different probe-shape inventory. Test: at the next pin-art engagement against a different corpus, classify the residual by probe-shape. If the inventory diverges substantially, §III's enumeration is rusty-bun-engagement-specific.

**Fal-726.2.** The probe-shape × pipeline product matrix does not refine Doc 721's Step 3 location. If alphabet tops do not cluster at named intersections, the recognition reduces to a flat taxonomy of probe-shapes without the row × column locating leverage. Test: across the next five substrate moves, locate each move's alphabet top in the probe-shape × pipeline grid. If locations distribute uniformly rather than clustering, the matrix isn't load-bearing.

**Fal-726.3.** The inherited Layer-D substrate does not scale linearly with broadening. If broadening adds packages whose probes activate already-named engine sites (no new tags), the inherited substrate is saturated and broadening yields no new substrate-introduction information. Test: across the next several broadenings (Doc 725 footnote on broaden-as-mode-resetting), measure the ratio of new fault-tag signatures to new packages added. If the ratio plateaus or declines, the inherited Layer-D has a saturation ceiling.

**Fal-726.4.** Probe-shape compounding with route-(b) enrichment is sub-multiplicative. If the next several engine-site enrichments do not surface a proportionally higher count of newly-classified probe-shapes, the multiplicative claim weakens. Test: at the next route-(b) move, measure the count of previously-below-threshold faults that now classify into a named probe-shape. If the increase is sub-proportional to the enrichment's site coverage, V.c's compounding claim weakens.

## VIII. Relation to prior corpus work

[Doc 720](/resolve/doc/720-the-rusty-bun-runtime-as-a-dag-of-interconnected-pipelines-sipe-t-topology-over-the-engine-substrate) named the engine as a sixteen-pipeline DAG. The probe-shape × pipeline matrix lifts this two-dimensionally: the residual's location in the matrix names both *what the engine produces wrong* and *where the producing pipeline lives*.

[Doc 721](/resolve/doc/721-the-cross-pipeline-diagnostic-protocol-locating-the-top-of-a-substrate-widenings-alphabet-by-walking-the-engines-dag) named the cross-pipeline diagnostic protocol with Step 6 route-(b) escalation. This document refines Step 1 (enumerate the gated population) by classifying by probe-shape before walking; Step 3 (locate the highest shared layer) becomes a search across the probe-shape × pipeline product matrix.

[Doc 722](/resolve/doc/722-named-recognitions-as-operating-instruments-the-reflexive-structure-of-corpus-articulations) named that articulations become operating instruments. The probe-shape taxonomy is one such: the substrate-introducer's next round will sort cluster picks by probe-shape and select the bright-zone column first when broadening; the blind-zone column when residuals tighten; the deeper-than-blind column only when the engagement is investing in substantive engine work.

[Doc 723](/resolve/doc/723-diagnostic-tags-as-semiotic-signs-layer-indexed-interpretation-in-pipeline-dag-topologies) named Layer D as the substrate-introducer's constructive probe-substrate. This document extends Layer D bidirectionally: the substrate-introducer's investment AND the corpus's inherited probes both populate Layer D. The two are operationally distinct but structurally adjacent — both contribute to the apparatus's threshold of diagnostic semanticity.

[Doc 724](/resolve/doc/724-feature-set-prediction-static-substrate-need-mapping-from-source) named bright/blind zones for the forward predictor. The probe-shape taxonomy provides a *third* zone — "deeper than blind" — where neither forward token-grep nor backward route-(b) trace alone is sufficient; only the consumer's own embedded probe surfaces the engine-internal arithmetic or semantic divergence. The predictor v2 (AST-level) named in Doc 724 §XI.d catches some of III.d's dispatch fingerprints but not III.b's arithmetic verifications.

[Doc 725](/resolve/doc/725-the-cluster-to-walk-mode-transition-soft-saturation-as-protocol-signal-in-substrate-introduction) named the cluster/walk mode transition and broadening-as-mode-resetting. Doc 726's probe-shape taxonomy refines both: cluster mode operates within one probe-shape × pipeline cell; walk mode crosses cells; broadening expands the inherited Layer-D and re-cluster-shapes the residual by introducing fresh probe-instances.

The corpus arc 720 → 721 → 722 → 723 → 724 → 725 → 726 reads as: topology, protocol, reflexivity, semantics, prediction, mode-selection, **consumer-probe inheritance**. The seventh layer names where the residual's fault-tags structurally originate.

## IX. Honest scope

The five probe-shapes named in §III are an empirical observation across the rusty-bun engagement's 846-package residual. The taxonomy is portable per §V.b *as a hypothesis*; testing portability requires another engagement's data.

Three things this document does *not* claim:

1. *Exhaustiveness.* The five shapes are the ones that surfaced at this engagement, in this maturity. A different maturity (earlier — when bright-zone surface installs dominate; later — when arithmetic and semantic divergence dominate) would exhibit a different mix. Other corpus would exhibit other shapes.

2. *That every residual is closable.* III.b (arithmetic verification) for elliptic / secp256k1 names a substantive engine investment — real BigInt arithmetic — that may not fit the engagement's scope. Some residual clusters remain residual not because the probe-shape is misclassified but because the engine work behind the probe is structurally larger than the engagement's substrate-budget.

3. *That consumer-embedded probes are deliberate engine-test machinery.* The consumers wrote their probes for *their* development concerns — defensive coding, browser polyfill detection, runtime fingerprinting — not to exercise a hand-rolled JavaScript runtime. The engagement inherits them for free *because* they are general-purpose verifications. The portability of the probe-shape taxonomy depends on consumer probes remaining general-purpose; corpus-specific monkeypatching that targets particular engines would not generalize.

Per [Doc 548](/resolve/doc/548-the-ladder-of-ontological-participation)'s hypostatic boundary: this document articulates a Layer-IV structural relationship in the apparatus's substrate. The Layer-V import of "consumer-embedded probes constitute an external substrate the substrate-introducer can inherit" is named only at the corpus-tier scope; the metaphysical reading of inheritance-as-discipline beyond engineering is not made here.

## X. Closing

The remaining clusters at 78–84% load-rate are not heterogeneous noise. They are the structured imprint of consumer-embedded probes meeting the engine's substrate-production. The probes form a corpus-side Layer-D substrate that the engagement inherits the moment each package enters the basket. Reading the residual by probe-shape locates each cluster's alphabet top inside a two-dimensional matrix — probe-shape × pipeline — that refines Doc 721's Step 3.

The taxonomy is operationally consequential. The substrate-introducer's next round will sort by probe-shape: III.c (feature presence) first when broadening just expanded the basket and bright-zone surfaces are unwalked; III.a (shape-correctness) when the bright zone has saturated and the blind zone needs structural engine fixes; III.b and partial III.e only when the engagement is investing in substantive engine work whose timeline justifies the deeper substrate.

The semiotic connection the keeper named is now articulated. The residual is not a long tail of independent bugs but a small number of probe-shapes, each surfacing a particular kind of engine misproduction, distributed across the corpus by the package authors' own defensive coding. The substrate-introducer inherits this inventory whenever the basket grows. Naming the inheritance turns it into an operating instrument per [Doc 722](/resolve/doc/722-named-recognitions-as-operating-instruments-the-reflexive-structure-of-corpus-articulations) — subsequent engagement work will dispatch substrate moves with the probe-shape × pipeline matrix as the routing layer.

---

## Appendix A — The Originating Recognition

> *"Review the five most recent corpus docs for a meta move that discovers the semiotic connection between remaining clusters."*

— Jared Foy, 2026-05-16, via Telegram, after the day's thirty-fourth substrate move (Tier-Ω.5.BBBBBBBB — real delete + defineProperty spec defaults) closed color-convert. The receiver-tag enrichment from Ω.5.MMMMMMM had made probe-shape III.a's signature legible; this document names the framework that the receiver-tag-enrichment instrument had begun surfacing.
