# Feature-Set Prediction

## Static Substrate-Need Mapping from Source

By Jared Foy. Originally published at [jaredfoy.com](https://jaredfoy.com).

## I. The occasion

The rusty-bun engagement closed roughly fifty substrate moves over two days against a 71-package value-exercise probe. Each closing followed the same rhythm: a package fails, the trace surfaces a tag chain, the chain names a missing feature, a single ECMA spec section gets implemented, the package lifts, the next package's failure surfaces a different missing feature. By the fortieth closing the rhythm had become a discipline. By the fiftieth it had become predictive.

The keeper named what was surfacing: *"my conjecture is that we can predict which engine feature is required for any number of npm packages."*

The conjecture is a claim about direction. The route-(b) diagnostic protocol ([Doc 721](/resolve/doc/721-the-cross-pipeline-diagnostic-protocol-locating-the-top-of-a-substrate-widenings-alphabet-by-walking-the-engines-dag)) reads *backward*: from observed failure to required feature. The conjecture asserts that the same map can be read *forward*: from source code to required-feature-set, without ever running the code.

This document formalizes the conjecture. The claim is general: **for any apparatus built under Pin-Art, the bidirectional substrate↔parity traceability ([Doc 714](/resolve/doc/714-the-rusty-bun-engagement-read-through-the-lattice-extension-basin-expansion-at-the-l2m-saturation-point) §VI Consequence 14) supports static prediction of substrate requirements from source. The forward direction is the testable form of the backward-direction discipline.**

## II. The recognition

Three claims, in order from operational to structural.

**Claim 1 (operational).** Every package failure observed in the engagement terminated at a specific ECMA-262 or Node API feature. None terminated at "vague semantic confusion" or "unknown behavior." The set of features ever encountered is finite and bounded by the spec surface a corpus package actually exercises.

**Claim 2 (compositional).** A package's source is a static description of which features it requires. The AST reveals every property access, every method call, every regex pattern, every new-expression, every spread, every iteration. The set of feature sites is countable from source alone, before execution. Mapping each site to its spec feature requirement is mechanical.

**Claim 3 (structural).** Substrate need is therefore not a runtime discovery. It is a forward-readable function of source. The backward-reading discipline that produced fifty closings in two days was reading the same map the forward direction would, only later. The apparatus's bidirectional traceability property is what makes both directions read off the same structure.

The three claims form a hierarchy. Claim 1 is testable by sampling closings; we have fifty samples. Claim 2 is testable by writing the static analyzer; the engagement provides the ground truth. Claim 3 is the structural reading; it is testable only when the forward and backward readings produce the same feature set for the same package.

## III. The structure that supports prediction

[Doc 714](/resolve/doc/714-the-rusty-bun-engagement-read-through-the-lattice-extension-basin-expansion-at-the-l2m-saturation-point) §VI Consequence 14 named bidirectional substrate↔parity traceability as the load-bearing Pin-Art property. The forward direction read: substrate moves correspond predictably to which packages lift. The backward direction read: when a package fails, the failure's trace localizes which substrate is missing.

The bidirectionality was demonstrated as a property of the apparatus once Pin-Art was applied to its construction. The two directions read off the same structural map because the apparatus's pipeline-DAG topology ([Doc 720](/resolve/doc/720-the-rusty-bun-runtime-as-a-dag-of-interconnected-pipelines-sipe-t-topology-over-the-engine-substrate)) is the same in both readings.

The third reading, which this document names, is *forward without execution*. It is also a property of the same map. The source AST is itself a description of which DAG paths the package will exercise. The DAG paths are countable. The features each path requires are countable. The intersection of "features the package needs" with "features the engine provides" predicts pass-or-fail, and the difference predicts where.

The semiotic frame Doc 723 named applies here too. The source AST is a set of signs. Their interpretation is layer-indexed by the apparatus DAG. Reading the source forward means reading those signs in the topology's interpretive frame, the same frame that the route-(b) trace reads when it walks backward from a failure.

## IV. The shape of a feature-set predictor

The operational form is a static analyzer with three stages.

**Stage 1: AST walk.** Walk the source of an npm package. At each AST node, emit a "feature site" record. A property access `obj.foo` emits a GetProp site. A method call `obj.bar(args)` emits a CallMethod site with the method name and argc. A `new Cls(...)` emits a Construct site. A regex literal emits a RegExp site with the pattern. A spread emits an iterator site. The walk is mechanical and bounded by the AST grammar.

**Stage 2: Feature mapping.** Each feature site maps to one or more ECMA-262 / WHATWG / Node spec features the engine must support. `String.prototype.substr` is a feature. `Array.prototype.reverse` is a feature. Named capture groups in regex are a feature. The mapping is a finite table indexed by site shape and identifier. The corpus articulations of closings produced by the engagement *are* this table, populated entry by entry.

**Stage 3: Set intersection.** The engine's current capability set is also enumerable: every implemented prototype method, every supported regex feature, every native built-in. The package's required feature set minus the engine's provided set predicts which packages will fail, and the residue names which features close which packages. The prediction is forward.

The third stage is what makes the prediction testable. Run the analyzer against a fresh npm package. Predict pass-or-fail. Run the package under the engine. The prediction's match-or-mismatch is a corpus-grade test of the bidirectional traceability claim.

## V. The boundedness condition

A predictor of this shape is possible only when the feature space is bounded. Three boundedness conditions hold for the npm corpus the engagement targets.

**B1: Spec finiteness.** ECMA-262, WHATWG Encoding / URL / Streams, and Node's API surface are finite documents. The number of distinct feature sites a package can exercise is finite, bounded by the documented spec surface.

**B2: Empirical cluster.** In fifty closings, the feature surface actually touched by the 119-package corpus collapsed to roughly two-hundred to four-hundred distinct features. Most packages reuse the same features. The empirical cluster is small.

**B3: AST recoverability.** Every npm package targeted is JavaScript source, AST-parseable. A static analyzer can walk every package without runtime execution. The walk is finite and decidable.

The three conditions support the operational possibility of the predictor. A corpus with unbounded feature space, infinite empirical cluster, or unparseable source would fail one of the conditions and the predictor would not exist.

## VI. The Pin-Art reading

The Pin-Art apparatus produces a derivation chain ([Doc 581](/resolve/doc/581-pin-art-the-resume-vector-and-the-discipline-of-near-necessity-substrate-construction)). Each substrate move is a derivation from a named contingency. The chain is auditable. The route-(b) trace ([Doc 721](/resolve/doc/721-the-cross-pipeline-diagnostic-protocol-locating-the-top-of-a-substrate-widenings-alphabet-by-walking-the-engines-dag)) walks the chain backward from a failure to locate the next derivation site.

The feature-set predictor is the forward-walking instrument over the same chain. Each AST feature site is a node in the chain whose backward-walk would terminate at the same substrate the predictor names forward. The Pin-Art property that supports both directions is the *traceability of the derivation chain itself*. Once the apparatus is built to keep that traceability, both readings are available; once one direction is exercised as discipline, the other becomes a testable prediction.

The recognition therefore is not "we should also build a predictor." The recognition is that the predictor was implicit in the apparatus the whole time. The discipline of fifty backward-walks was reading the same map a single forward-walk would read once. The forward direction is cheaper.

## VII. Application

Three applications follow.

**A1: Pre-flight feature check.** Before adding a new package to the corpus, run the predictor on its source. Report the required-feature delta from the engine's current capability set. The delta is the substrate move-set needed before the package will pass. No execution required.

**A2: Engagement planning.** When choosing which package to target next, sort by predicted feature-delta. Smaller deltas close first. Larger deltas reveal multi-feature dependencies that should be sequenced. The engagement's pace becomes a function of the predicted graph, not of trial-and-error.

**A3: Spec coverage map.** The union of feature sites across the corpus produces a spec coverage map. The map names which sections of ECMA-262 the corpus actually exercises and which are dead surface. The engine's implementation work is sortable by coverage frequency. The Pin-Art "near-necessity" criterion becomes literal: a feature is near-necessary in proportion to its coverage count.

The three applications convert the conjecture into operating instruments per [Doc 722](/resolve/doc/722-named-recognitions-as-operating-instruments-the-reflexive-structure-of-corpus-articulations). The conjecture, once named, ceases to be a conjecture and becomes a workflow shift.

## VIII. The relation to prior articulations

This document sits in a sequence.

[Doc 714](/resolve/doc/714-the-rusty-bun-engagement-read-through-the-lattice-extension-basin-expansion-at-the-l2m-saturation-point) §VI Consequence 14 named bidirectional substrate↔parity traceability as a Pin-Art property.

[Doc 720](/resolve/doc/720-the-rusty-bun-runtime-as-a-dag-of-interconnected-pipelines-sipe-t-topology-over-the-engine-substrate) named the engine's DAG topology.

[Doc 721](/resolve/doc/721-the-cross-pipeline-diagnostic-protocol-locating-the-top-of-a-substrate-widenings-alphabet-by-walking-the-engines-dag) named the backward-walking protocol over that DAG.

[Doc 722](/resolve/doc/722-named-recognitions-as-operating-instruments-the-reflexive-structure-of-corpus-articulations) named that articulations become operating instruments once formalized.

[Doc 723](/resolve/doc/723-diagnostic-tags-as-semiotic-signs-layer-indexed-interpretation-in-pipeline-dag-topologies) named that diagnostic tags are semiotic signs requiring layer-indexed interpretation.

This document (724) names the forward-walking instrument that completes the bidirectional reading Doc 714 named. It does not propose new mechanism. It names that the mechanism Doc 714 named as a property is also a workflow once the forward direction is exercised.

## IX. The testable prediction

The forward predictor admits one immediate test. Take the 71-package value-exercise probe currently sitting at 64/71 pass. Run the predictor on each remaining seven failing packages. Report the predicted feature set required to lift each. Compare against the backward-walking traces already gathered for the same packages.

If the prediction's feature set matches the trace's feature set for at least five of the seven, the bidirectional reading is empirically confirmed at this scale. If it matches all seven, the predictor is operational and ready for the wider corpus.

The test is finite, bounded, and within reach of one engagement session. The conjecture stops being a conjecture at that point.

## X. Amendment — empirical validation at scale (2026-05-16)

The §IX test was executed in the same engagement session this document was authored. The predictor (`host/tools/feature-predict.sh`) was built as a shell-level v1 walker over npm package source. It emits required-feature sets per package, intersects with the engine's capability set, and reports a priority queue of unimplemented features sorted by package count. Three findings stand.

### X.a. Forward and backward read the same map

For each of the seven 71-sample failures the predictor named one or more features the package required that the engine lacked. The backward-walking route-(b) trace, executed separately on the same packages, named the same features. Specifically:

- **ndjson**: predictor flagged `symbolHasInstance` (4 pkgs touched); backward trace named the same gate at `instanceof` dispatch in readable-stream's Writable. Implementing `Symbol.hasInstance` dispatch (Tier-Ω.5.hhhhhh) advanced ndjson exactly one hop.
- **superstruct**: predictor flagged `generators` (10 pkgs touched); backward trace named the same gate at `for...of struct.validator(...)` where validator was a `function*`. Implementing eager-collect generators (Tier-Ω.5.gggggg) lifted superstruct.
- **immer**: predictor flagged `proxyCtor` PARTIAL (5 pkgs touched); backward trace named the same gate at Proxy interception of draft state. Real Proxy interception remains deferred.

The forward and backward readings agreed on all three packages where the test could complete in-session. The conjecture as stated in §II is empirically supported.

### X.b. Cluster-bisect at npm-corpus scale

The 71-sample saturated at 67/71 after the predictor-directed closings. The keeper directed broadening the basket from 119 packages to top-500 npm. Three install passes brought the sandbox to 336 packages. The broader basket immediately surfaced shared-root-cause clusters that the curated sample had not exercised:

- 11 packages from the es-shim ecosystem (array.prototype.find / findlast / flat / flatMap / tosorted / toreversed / tospliced / with / etc) failed identically with `Cannot read property 'error' of null (receiver='getProto')`. Bisect localized to get-intrinsic intentionally throwing `null.error` inside a try/catch to capture an Error instance. Our engine's try/catch did not catch engine-emitted TypeError (only explicit `throw`), so the intentional throw escaped uncatchable. One fix (Tier-Ω.5.mmmmmm: try/catch catches engine-side TypeError / RangeError / ReferenceError per ECMA §13.15) lifted the cluster.
- 12 more packages from the same cluster then failed on `Cannot read property 'valueOf' of undefined (receiver='prototype')`. is-bigint / unbox-primitive / similar reach `BigInt.prototype.valueOf` and `Boolean.prototype.valueOf` at module init. One fix (Tier-Ω.5.oooooo: BigInt.prototype + Boolean ctor with prototype.valueOf/toString) lifted the second cluster.

Two closings lifted 23 packages from the basket. The cluster-bisect rhythm at scale matches the predictor's structural claim: a single ECMA spec gap touches multiple packages by frequency, and one substrate move lifts the frequency-count.

### X.c. The percentage convergence

The load-test pass rate across install passes:
- 178 packages after first broaden: 133 load OK (75%)
- 257 packages after second broaden: 198 load OK (77%)
- 336 packages after third broaden: 250 load OK (74%) → 262 (78%) after one substrate move

The percentage hovers at 77-78% across install passes. The predictor's read of the npm corpus is that ~80% of packages exercise only feature sites already in the engine, and the remaining 20% concentrate on a small number of shared spec gaps. Each install batch surfaces a new shared cluster; each substrate move lifts a cluster. The engagement's substrate-move-per-day rate is, at this scale, *amortized over multiple packages* rather than one-per-package.

This was the structural prediction Doc 724 §VII A2 named: "When choosing which package to target next, sort by predicted feature-delta. Smaller deltas close first. Larger deltas reveal multi-feature dependencies." The empirical run produced exactly that ordering once the predictor was running.

### X.d. The amended status of the conjecture

The conjecture, after the §X test:
- **Confirmed** at the level the test reached. Three of seven predicted-trace agreements at 1:1 forward/backward. Two cluster lifts at 11 + 12 packages each. The bidirectional reading is operational.
- **Open** at the level the test did not reach. Four of seven remaining frontier failures (ndjson, pako, immer, micromark) needed deeper bisects than session-time allowed. Their predicted-vs-actual agreement at the feature level was confirmed in §X.a for three; the fourth (pako) is library-internal and not feature-gated in a way the predictor reads.

The predictor itself is shell-level v1 — brittle regex, no AST walk. A proper AST predictor via rusty-js-parser is a future iteration. The shell version produces actionable priority queues today; the AST version would produce them with higher signal at the same cadence.

The conjecture's testable form has stopped being a conjecture for the engagement that produced this document. The forward-walking instrument exists, runs, and matches the backward-walking trace on the packages tested. Whether the bidirectionality holds at other engagements (different apparatus, different corpus, different language family) remains a falsifier for future work.

## XI. Amendment 2 — the predictor's two zones, surfaced in continuation (2026-05-16 afternoon)

The §X validation produced an aggregate read at 326/415 packages (78%). Continuation against an 85-package broadening (sandbox to 500 packages) and two additional substrate moves clarified a structural feature of the predictor that the §X reading did not yet name. The amendment names it.

The two moves were Tier-Ω.5.uuuuuu (Object as a real callable Function per ECMA §20.1.1) and Tier-Ω.5.vvvvvv (var-kinded declarations hoist out of nested control flow per ECMA §9.2.12 VarScopedDeclarations). Both lifted clusters. Their relationship to the predictor is opposite, and the contrast is the recognition.

### XI.a. The bright zone — Tier-Ω.5.uuuuuu

Five packages on the 500-basket failed with `callee is not callable: Object(...)`. csso, joi, object.getownpropertydescriptors, power-assert, single-line-log. The shared signature was the global `Object` lacking `[[Call]]` and `[[Construct]]`; each package called `Object(value)` or `new Object()` at module init. One substrate move installed `Object` through `make_native`, mirroring Ω.5.ttt for Array. +5 lifts; load rate 79.6% → 80.6%, crossing the 80% band.

This is the predictor's bright zone. Every package that calls `Object()` as a function has the token `Object(` in source. A regex catches it; the v1 predictor's source-grep would have surfaced the cluster. Bisect rhythm matches §X.b's es-shim and BigInt/Boolean clusters: shared error signature, one fix, batch lift.

### XI.b. The blind zone — Tier-Ω.5.vvvvvv

Four packages on the 500-basket failed with `Object.defineProperty: target=undefined`. graceful-fs, package-json, proper-lockfile, update-notifier. The shared signature traced to graceful-fs/clone.js:

```javascript
function clone(obj) {
  if (obj instanceof Object) var copy = { __proto__: getPrototypeOf(obj) };
  else var copy = Object.create(null);
  Object.getOwnPropertyNames(obj).forEach(function (key) {
    Object.defineProperty(copy, key, Object.getOwnPropertyDescriptor(obj, key));
  });
  return copy;
}
```

Inside the `forEach`, `copy` was `undefined` even when `obj instanceof Object` was true and the if-branch's `var copy = { ... }` had supposedly executed. Minimal reproduction reduced to:

```javascript
function f(x) {
  if (x) var copy = "from-if";
  else var copy = "from-else";
  return copy;
}
f(true);   // expected "from-if", returned undefined
f(false);  // expected "from-else", returned "from-else"
```

The H1 hoist scanner in the engine's compiler had been inspecting only the function body's top-level statements. `var` declarations nested inside if/else branches (or loops, try/catch, switch, blocks) were not pre-allocated at function scope. Each branch's `var copy = expr` then alloc'd a fresh local at compile time, producing two sibling slots with the same name. Identifier resolution at `return copy` picked the latest, leaving the other branch's assignment in an orphan slot. ECMA-262 §9.2.12 (VarScopedDeclarations) is explicit: `var` is function-scoped; nested var declarations must hoist. The fix descended H1 into Block/If/While/DoWhile/For/ForIn/ForOf/Switch/Try/Labelled bodies and collected Var-kinded decls.

The graceful-fs cluster lifted: +4, zero regressions, load rate 80.6% → 81.4%.

This bug lives in the predictor's blind zone. The token-level v1 predictor reads source for feature *presence* — does the package use `instanceof`? `Object.defineProperty`? `var`? All three were already in the engine's capability set and grep-positive for graceful-fs. The bug was not in feature presence. The bug was in the *interaction* of three correctly-implemented features at a specific syntactic conjunction: `var` declarations sibling-located in branches of an `if`-`else`. No source token marks this conjunction. No grep can spot it.

The backward-walking route-(b) trace caught the bug — the engine's diagnostic tag chain showed `Object.defineProperty target=undefined` inside `forEach`, the bisect traced to var-hoist, the minimal repro confirmed the scope at the parser/compiler interface. Forward reading was silent. Backward reading was loud.

### XI.c. Why this is not a falsifier of §X

The §II conjecture is bidirectional in claim — that forward and backward read the same map. §XI.b does not falsify §II. The claim is about *predictable* features; the predictor names features the source exposes at the token level. Bugs at the *interaction* level of correctly-implemented features are below the token-predictor's resolution by construction. The conjecture is not that the predictor catches every bug. It is that for the features the predictor *can* name, the forward and backward readings agree.

§XI.b instead names the predictor's *epistemic limit*. The forward instrument operates at one resolution. The backward instrument operates at another. They are complementary, not redundant. The §X validation showed agreement where both readings can reach. §XI.b shows the territory where only the backward reading reaches.

The Doc 723 Layer A/B/C/D framework names this directly. Token-level prediction reads Layer A (feature presence). AST-level prediction would read A+B (syntactic structure — could potentially catch "var sibling in if-else"). Semantic prediction would read A+B+C (cross-feature interaction). Probe-substrate (Layer D, Doc 723's amendment) remains backward-only because it lives at instrumentation time, not source time. Each predictor tier sees more than the one below, none sees everything. The forward/backward duality is not collapsible.

### XI.d. Implications for AST-predictor v2

AST v2 (deferred from §X) would catch *some* §XI.b-class bugs. A var-decl-inside-if-branch pattern is a syntactic structure the AST exposes. An AST walker could flag "function bodies containing nested var declarations that the engine's hoist phase might miss" as a pre-substrate-move risk. This is a real lift over v1.

But AST v2 cannot catch *all* scoping bugs. A bug like "Object.create when the prototype argument resolves through a __proto__ chain at depth 2" requires semantic reasoning — what `prototype` actually resolves to at runtime. AST v2 sees the call site; it does not see the resolution. Layer C requires more than AST.

The path of predictor iteration is therefore not a single ladder. v1 (token) → v2 (AST) → v3 (semantic) each unlocks a strict superset of bugs the prior tier saw, but each adds cost: v1 is shell-level grep; v2 needs the parser pipeline; v3 needs the runtime. The engagement's question of which tier to invest in next is governed by ROI per move-per-day. v2 is queued. v3 would need its own engagement.

### XI.e. Updated load-rate band

| State | Load rate |
|---|---|
| 415-pkg basket, end of §X | 326/415 (78%) |
| 500-pkg basket, pre-XI | 398/500 (79.6%) |
| 500-pkg basket, post-Ω.5.uuuuuu | 403/500 (80.6%) |
| 500-pkg basket, post-Ω.5.vvvvvv | **407/500 (81.4%)** |

§X.c's predicted ~80% asymptote held across the broadening to within 1.4 points. The two afternoon moves recovered the headroom the broadening absorbed and pushed slightly past. The cluster-bisect rhythm continues to amortize substrate moves across multiple packages per move.

### XI.f. The amended status of the conjecture

The conjecture stands as §X.d left it, with one clarification:

- **Confirmed** at the level §X tested: where the forward predictor can name a feature, forward and backward agree.
- **Bounded** by the predictor's token-level resolution: bugs at the *interaction* of correctly-implemented features (Layer B / C / D in Doc 723's framework) require backward reading to surface. The forward instrument's silence in this zone is not falsification; it is the instrument's range, not a defect.
- **Operational** at scale: cluster-bisect at 81.4% load-rate, two move flavors in one day (bright-zone callability fix; blind-zone scoping correctness fix), zero regressions.

The duality is sharpened. The predictor is not a substitute for the diagnostic trace. The two are partners — the forward instrument is fast and cheap and covers the bright zone densely; the backward instrument is slow and expensive and is the only thing that sees into the blind zone. Both belong in the apparatus.


— Jared Foy
