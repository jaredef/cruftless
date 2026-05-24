# 707 — Pin-Art at the Behavioral Surface: Bidirectional Probes

*Author: Jared Foy. 2026-05-10.*

[Doc 706](/resolve/doc/706-three-pilot-evidence-chain-derivation-from-constraints) closed the formalization-then-derivation loop across three pilots and recorded the apparatus' value claim in terms of LOC-ratio reduction. The framing through that doc treated the apparatus as a one-directional pipe: constraints in, derivation out, verifier confirms.

This document records a deeper reading. The constraints the apparatus extracts are not one-directional inputs to derivation. They are **pins on a behavioral surface, carrying information in both directions**. Each pin constrains what a derivation must do; the same pin reveals an invariant of the original implementation that was otherwise implicit. Pin-Art (Docs [270](/resolve/doc/270-pin-art), [619](/resolve/doc/619-pin-art-canonical-formalization)) named this bidirectionality in the architectural-surface case. This document records its operational expression on the *behavioral-surface* case, anchored to a measured pilot run.

## The reframe

The keeper's directive that surfaced this reading: *"the goal should be plug-and-play interoperability with no regressions"* and *"these pins have bidirectional information flow."* The first sentence sharpened the apparatus' target away from "100% behavior parity" (which would lock in implementation accidents and contingent inefficiencies). The second sentence named what the apparatus is actually doing structurally: producing pins whose information flow is two-way.

[Doc 705 §10.1](/resolve/doc/705-pin-art-operationalized-for-intra-architectural-seam-detection) anchored Pin-Art to intra-architectural seam detection: probes on code-structural surfaces, six signal classes, manual GitLab anchor + tooled Bun anchor. The same apparatus operates one tier up, on behavioral contracts rather than code structure. The pins are different (spec invariants, test reps, consumer expectations, conformance-suite entries) but the bidirectional probe-and-surface dynamic is the same.

## The five pin classes

The rusty-bun apparatus' constraint corpus pulls from five distinct sources. Each is a class of pin with its own probe semantics:

| Pin class | Source | Probe semantics |
|---|---|---|
| Spec invariant | WHATWG / W3C / HTML / RFC | Normative authority. The standard says this MUST hold. |
| Test rep | Implementation's own test corpus | Observational. The implementation passes this; therefore (within the test suite) it has this property. |
| Consumer expectation | Production downstream code | Dependency-survey. A real consumer's source relies on this; therefore it is an invariant the original implementation is implicitly committed to. |
| WPT entry | Web Platform Tests | Conformance-suite. The browser-vendor consortium maintains this as the canonical operational test of spec compliance. |
| Implementation-source probe | Static analysis of impl source | Architectural-witness. Patterns in the source (sync/async, unsafe density, FFI calls) reveal behavioral commitments not at the test boundary. |

Each pin class carries a different *kind* of information. Together they cover the behavioral surface from multiple directions: what the spec demands (above), what the implementation observes (within), what consumers depend on (below), what conformance suites measure (the side), and what the impl source reveals (under).

## Bidirectional information flow

The forward direction of each pin is operationally familiar. A pin constrains a derivation: the derivation must satisfy what the pin asserts. This is what [Doc 706](/resolve/doc/706-three-pilot-evidence-chain-derivation-from-constraints) measured across three pilots and what the verifier closes.

The backward direction is the under-articulated half. Each pin also reveals an invariant of the *original* implementation that would otherwise have remained implicit on its surface. This is the half [Doc 270](/resolve/doc/270-pin-art) identified for architectural surfaces: probes carry information from the surface back to the prober, surfacing structure the surface alone does not display.

For the URLSearchParams pilot, run on 2026-05-10, the bidirectional readings of the eleven consumer-regression pins are:

| Pin (cited consumer) | Forward: derivation must | Backward: original is committed to |
|---|---|---|
| AWS SDK SigV4 canonical-query sort | UTF-16 code-unit sort, case-sensitive | This exact sort semantics; case-sensitive ASCII order |
| Stripe SDK metadata key encoding | Percent-encode `[` and `]` | Not literal-bracket passthrough |
| Express `?debug` empty-value parse | Parse no-equals as empty string, not absent | This exact empty-vs-absent distinction |
| undici body wire encoding | Space → `+` in form-urlencoded codec | This codec, not RFC 3986 |
| node-fetch duplicate-key preservation | Repeated entries round-trip through wire | Wire format preserves order through string serialization |
| OAuth 1.0a normalization (RFC 5849) | Stable sort by name, value-order preserved within | Stable sort within equal names |
| ky / wretch / ofetch wrapper pairs | Pairs feed directly to URLSearchParams | No transformation between wrapper and codec |
| WPT URL test data simple query | `?a=b&c=d` round-trips identically | Standard form-urlencoded for canonical input |
| WPT URL test data empty value | `?a` round-trips as `a=` | Equals is added at serialization time |
| WPT URL test data unicode query | UTF-8 percent-encoding for non-ASCII | This byte-mapping, not UTF-16 |
| Express optional `?` prefix | Parse with or without leading `?` identically | No semantic difference |

Read down the forward column: each row is a constraint on a future derivation. Read down the backward column: each row is a behavioral invariant Bun (and any other URLSearchParams implementation) is implicitly committed to, with the consumer who depends on it cited.

The backward column is information that was otherwise unavailable. Bun's source code does not carry the annotation "AWS SigV4 depends on this sort being case-sensitive." Bun's test corpus does not say "Stripe will break if `[` and `]` are not percent-encoded." Only the consumer corpus, with cited sources, surfaces this dependency map. **The apparatus is producing structural information about the implementation that exists nowhere else.**

## Two outputs, not one

The apparatus' value claim, restated:

**Output 1 — the derivation.** [Doc 706](/resolve/doc/706-three-pilot-evidence-chain-derivation-from-constraints) measured this across three pilots (since extended to six on disk). Aggregate LOC ratio approximately 3% of upstream targets. Forward direction.

**Output 2 — the dependency-surface map of the original.** First explicit articulation in this document. The consumer corpus, with cited source per pin, is a survey of the implementation's behavioral commitments to its downstream ecosystem. Backward direction.

Output 2 has standalone value, even if no derivation ever ships. A Bun maintainer asking "if I change URLSearchParams.sort to be case-insensitive, what breaks?" can read the consumer corpus and find the answer (AWS SDK SigV4) with cite. The same maintainer asking "if I stop percent-encoding brackets, what breaks?" can find the answer (Stripe SDK) with cite. The map exists outside Bun's source, outside its tests, outside the spec, and yet it constrains every change Bun makes.

This is what Pin-Art [Doc 619](/resolve/doc/619-pin-art-canonical-formalization) calls a **probed surface**: the surface itself does not display its topology, but a sufficiently dense set of probes makes the topology visible. Density matters. Eleven cited consumer expectations across seven categories on URLSearchParams produce a map dense enough to navigate; eleven hundred would produce one detailed enough to constrain every plausible change.

## The plug-and-play criterion as forward operational consequence

The keeper's plug-and-play target operationalizes the forward direction concretely:

```
For each behavioral surface S:
  let consumer corpus C(S) = { cited consumer expectations probing S }
  let derivation D(S) = the rusty-bun derivation of S
  D(S) is plug-and-play with respect to C(S) iff every pin in C(S)
    that passes against the original implementation also passes
    against D(S).
```

Zero regressions is the criterion. The pilot run closing 11/11 consumer-regression on URLSearchParams is the first operational instance.

The criterion is not "100% parity" (which would lock in implementation accidents) and not "100% spec conformance" (which would diverge from how production code actually uses the surface). It is exactly **what the consumer corpus says works**, with cites. Spec conformance is necessary but insufficient; ecosystem matching against arbitrary Bun behavior is overcommitting; *consumer-corpus-witnessed-behavior conformance* is the operational sweet spot.

## The dependency-surface map as backward operational consequence

The same consumer corpus, read backward, constitutes a structural artifact about the original implementation. This artifact has uses independent of any derivation:

**Pre-change impact analysis.** Before changing a behavioral surface, query the consumer corpus to enumerate which downstream consumers depend on the current behavior. Decide whether to break, deprecate, or preserve based on cited dependency.

**Implicit invariant audit.** Surface invariants the original implementation never explicitly stated. Some will be deliberate; some will be accidental commitments the maintainers were unaware of. Both are now visible.

**Specification-vs-practice gap detection.** Each pin's forward column matches a derivation against spec; the backward column reveals where the original *diverges* from spec because consumers came to depend on the divergence. Stripe-bracket-encoding is consistent with spec. Bun's earlier `encode(undefined) → 0` in TextEncoder (per Doc 706) was *not* consistent with spec but matched V8/WPT. Each such divergence is a place where the spec and the ecosystem have negotiated and the spec lost.

**Maintenance handoff artifact.** When a project changes maintainers, the consumer corpus transmits "what downstream actually relies on" in a form the new maintainer can read in an afternoon. Implicit knowledge passes through the corpus rather than through the predecessor.

These uses are downstream of Pin-Art's bidirectional reading. They were latent in the apparatus from the start. Naming them explicitly converts them from accident to design.

## Implications for the apparatus

1. The apparatus' aggregate output is a **multi-source-witnessed behavioral map**, not a derivation alone. Future pilots should commit run-notes that record both the forward (verifier closure) and backward (dependency-surface) readings of each pin class.

2. The five pin classes (spec, test, consumer, WPT, implementation-source probe) form a probe taxonomy. The taxonomy should be made explicit in the apparatus' constraint corpus schema, with each clause tagged by its pin class. This sets up per-class verifier reports and per-class dependency-map slices.

3. The plug-and-play criterion replaces the misleading single "behavior parity %" with a falsifiable per-pin-class compliance number. Spec conformance: N% of WPT-equivalent pins pass. Ecosystem compatibility: M% of consumer pins pass. Implementation-internal divergence: K pins where the derivation deliberately differs, each with recorded reason. Three honest numbers, three independent claims.

4. Bun (and any Pin-Art subject) becomes an indirect beneficiary of the apparatus. Even if the rusty-bun derivation never ships, the consumer corpus is a contribution back to Bun's maintainers in the form of an implicit-invariant audit. This is what Pin-Art does: probes carry information in both directions; both directions have value.

## Anchor instance

The URLSearchParams pilot, run on 2026-05-10 against the v0.13b enriched constraint corpus and the consumer corpus introduced in this run, is the first measured instance of bidirectional Pin-Art on a behavioral surface. The numbers:

- Constraint-doc verifier (prescriptive layer): 32 of 32 pass
- Consumer regression suite (descriptive layer): 11 of 11 pass
- Total bidirectional pin closure: 43 of 43

Forward direction: derivation passes every pin. Plug-and-play with respect to the seven consumer categories cited (undici, OAuth 1.0a, Stripe, Express, ky/wretch/ofetch, WPT URL test data, AWS SDK v3).

Backward direction: eleven cited behavioral invariants of Bun's URLSearchParams now exist as a reviewable dependency map. The map cites concrete consumer source (file path, function, URL) for each invariant. Verifiable, falsifiable, contributable upstream.

This is the standing-apparatus tier closure for behavioral-surface Pin-Art. The same pattern repeats for every pilot: produce the derivation; produce the dependency map; verify both halves close.

## What this changes about the corpus story

[Doc 704](/resolve/doc/704-the-port-as-translation-is-a-category-error) named the work as formalization-then-derivation. [Doc 705](/resolve/doc/705-pin-art-operationalized-for-intra-architectural-seam-detection) operationalized the formalization side at the architectural-surface tier. [Doc 706](/resolve/doc/706-three-pilot-evidence-chain-derivation-from-constraints) closed the loop on derivation-from-constraints with measured pilots. This document widens the apparatus' value claim: the pins that close the loop in one direction also map dependencies in the other.

The accumulated chain now reads:

```
Doc 704:  this is formalization-then-derivation, not translation
Doc 705:  formalization at the architectural-surface tier (Pin-Art on code)
Doc 706:  derivation-from-constraints closes the loop (forward direction)
Doc 707:  pins are bidirectional; the dependency map is the second output
```

What the apparatus produces is no longer best described as "a small derivation that matches a constraint doc." A more accurate description: **a bidirectional Pin-Art apparatus on behavioral surfaces, whose two outputs are a derivation and a dependency-surface map of the original**, both produced from the same probe set and both useful to different audiences.

This is the framing the standing-apparatus tier (per Doc 705's structure) should record. Future engagements applying the apparatus to surfaces beyond URLSearchParams (the candidate list at Doc 706 §"Implications" includes Response, atob/btoa, File, AbortController, Blob, crypto.randomUUID; pilots 4 through 6 added Blob, File, AbortController) will produce additional dependency maps as a by-product. Over enough pilots, the aggregate map covers the entire behavioral perimeter of the implementation under study.

## Provenance

- Pilot anchor: rusty-bun `pilots/urlsearchparams/` — `RUN-NOTES.md` (prescriptive verifier, 32/32) and `CONSUMER-REGRESSION.md` (descriptive consumer regression, 11/11), 2026-05-10.
- Apparatus version: `derive-constraints` v0.13b. Six pilot LOC-ratio table at [Doc 706](/resolve/doc/706-three-pilot-evidence-chain-derivation-from-constraints) plus the URLSearchParams pilot's bidirectional extension here.
- Pin-Art source: [Doc 270](/resolve/doc/270-pin-art) introduces the bilateral probe-and-surface formulation; [Doc 619](/resolve/doc/619-pin-art-canonical-formalization) is the canonical formalization. Both ground this document's claim that bidirectionality is the defining property of the probe class.
- Consumer corpus contents: undici / node-fetch / cross-fetch (fetch body encoding); OAuth 1.0a libraries (RFC 5849 §3.4.1.3.2 normalization); Stripe SDK (form-urlencoded request body); Express / Koa / Fastify (request query parsing); ky / wretch / ofetch (fetch-wrapper option dicts); Web Platform Tests URL test data; AWS SDK v3 (SigV4 canonical-query-string). Each pin cited at file-path-and-function granularity in the pilot's `tests/consumer_regression.rs`.

— jaredfoy.com
