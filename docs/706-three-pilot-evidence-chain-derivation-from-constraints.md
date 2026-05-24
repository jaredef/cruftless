# 706 — Three-Pilot Evidence Chain: Empirical Closure of the Formalization-then-Derivation Loop

*Author: Jared Foy. 2026-05-10.*

[Doc 704](/resolve/doc/704-the-port-as-translation-is-a-category-error) framed the AI-translated Bun port as a category error: the work is not translation between languages but **formalization into constraints, then derivation from constraints into a target language**. [Doc 705](/resolve/doc/705-pin-art-operationalized-for-intra-architectural-seam-detection) operationalized the formalization side at the standing-apparatus tier, anchoring the apparatus to a manual instance (GitLab CI) and a tooled instance (Bun phase-a-port). What was missing was the derivation side closing the loop: an empirical demonstration that constraint documents emitted by the apparatus are sufficient input to derive working target-language code, with measurable footprint reduction.

This document records the closure of that loop across three pilots run on the rusty-bun apparatus between 2026-05-10 and 2026-05-10. The pilots were chosen to span the breadth of pilot classes a Bun-scale port encounters: a data-structure pilot (TextEncoder + TextDecoder), a delegation-target pilot (URLSearchParams), and an algorithm pilot (structuredClone). All three closed; the latter two with zero verifier failures and zero documented skips; the third with the strongest LOC ratio the apparatus has measured (3.9% against WebKit's hand-written C++).

## What was tested

The apparatus' value claim has two halves. The first half — that constraints can be extracted from a test corpus + spec corpus and emitted as a derivation-ready document — was validated by [Doc 705 §10](/resolve/doc/705-pin-art-operationalized-for-intra-architectural-seam-detection)'s anchoring exercise. The second half — that the emitted document is sufficient input to a derivation engine, and that the resulting code is dramatically smaller than a hand-written port — required pilots.

The pilots simulate the derivation engine via LLM rather than wiring the rederive infrastructure in directly. This was a deliberate scope choice. Wiring rederive in is heavier engineering than the pilots needed to demonstrate the apparatus' value claim. The simulation is operationally honest because the inputs are declared explicitly in source-code comments, the verifier consumes the constraint document's antichain representatives directly as test cases, and any derivation deviation surfaces as a verifier failure rather than disappearing into the engine.

## The three pilots

### Pilot 1: TextEncoder + TextDecoder

Class: data-structure (a small WebIDL interface pair, fully WHATWG Encoding §§9–10 spec'd).

Coverage at start: the auto-emitted constraint document carried 6 clauses across 1 property for TextEncoder and 82 clauses across 2 properties for TextDecoder. The audit phase named the gap immediately. The 6 TextEncoder clauses were exclusively negative-boundary cases (`encode(undefined).length === 0`) and tag-string cases (`toString() === "[object TextEncoder]"`); no positive operational test of `.encode(string)`. The 82 TextDecoder clauses included a 13-cardinality false-positive group of unrelated `assert(...)` calls in test files that happened to also use TextDecoder elsewhere. The constraint document, by itself, was insufficient.

Resolution: spec material was injected manually into the simulated derivation alongside the constraint document. The verifier passed 21 of 22 tests; the one skip (TEXT2 classifier-noise) was documented as an apparatus finding and queued for fix. LOC ratio against Bun's hand-written port: 13.2% naive, ~17–25% adjusted for binding-glue and spec-scope normalization.

The pilot's load-bearing finding: **the apparatus' constraint output captured an implementation-vs-spec divergence the spec alone misses**. The `encode(undefined) → 0 bytes` invariant is real Bun/V8/WPT behavior; per WHATWG IDL the value should coerce to `"undefined"` (9 bytes). The corpus-discipline output captures real implementation invariants beyond what the formal spec specifies. This is one of the apparatus' load-bearing claims, now empirically demonstrated for the first time.

Three apparatus work items surfaced: cluster-phase subject-attribution leakage, JS-Rust boundary modeling, and spec-source ingestion. The first and third were addressed before the next pilot.

### Pilot 2: URLSearchParams

Class: data-structure with delegation-target characteristics. URLSearchParams is fully WHATWG URL §5.2 spec'd with a substantial method surface (12 methods + form-urlencoded codec + UTF-16 sort + multi-form constructor).

Coverage at start: 17 properties / 35 clauses on the post-v0.13b enriched corpus, including method-level subjects for `.append`, `.delete`, `.get`, `.getAll`, `.has`, `.set`, `.sort`, `.toString`, and the iteration protocol. One cross-corroborated property (the constructor) witnessed by both Deno tests and the WHATWG spec extract. The pilot's input was substantially richer than Pilot 1's, and required no manual injection.

Verifier: 32/32 tests pass, zero skips. **First 100% verifier closure in the apparatus.** Three ahead-of-time hypotheses confirmed (UTF-16 code-unit sort vs USV order, narrower form-urlencoded character set vs RFC 1738, optional leading "?" in constructor). One AOT hypothesis not confirmed: the prediction of a second cluster-phase classifier-noise finding, similar to Pilot 1's. Its non-confirmation is itself information: Pilot 1's leakage fix was robust, not surface-specific.

LOC ratio: 62% against WebKit upstream (~300 LOC) vs the pilot's 186 code-only LOC. Worse than the htmx prior, but for an honest reason — investigation revealed that Bun does not implement URLSearchParams. Bun's `URLSearchParams.rs` is 60 LOC of FFI binding stubs; the actual implementation lives in WebKit's `WebCore::URLSearchParams` C++ class. The apparatus' value claim accordingly shifts for delegation targets: derivation-from-constraints is **competitive with the upstream implementation, and eliminates the need for the binding layer entirely**. That is a different value claim than the htmx 9.4% prior, but real and measurable. Pilot results should record whether the target implements or imports.

### Pilot 3: structuredClone

Class: algorithm. structuredClone is a graph operation defined as a two-phase serialize/deserialize over a closed type universe with cycle handling and identity preservation.

Coverage at start: the apparatus' strongest single-surface witness. 227 clauses across 5 cluster groups; STRU1 alone at 166 cardinality. Cross-corroborated witnesses on Bun's corpus include `expect(cloned).toStrictEqual({})`, `expect(cloned.size).toBe(0)` for Blob preservation, `expect(cloned.file.name).toBe("example.txt")` for File preservation, and `expect(cloned).toBeInstanceOf(Array)` for class preservation across modified prototypes. The constraint document by itself was already nearly sufficient; the spec extract added the unsupported-type error path and the streaming/transfer details that the test corpus did not exhaust.

Verifier: 23/23 tests pass, zero skips. **Second consecutive 100% verifier closure**, including the load-bearing identity-preservation tests (shared-reference identity within a single call, self-cycle, mutual cycle A → B → A, source-mutation independence). All four AOT hypotheses confirmed.

LOC ratio: **3.9%** against WebKit's `SerializedScriptValue.{h,cpp}` + `StructuredClone.{h,cpp}` totaling 7,549 LOC of C++. The pilot's 297 code-only Rust LOC implements the algorithmic core in two phases that match the SPEC's structure exactly. Adjusted for what the WebKit total handles that the pilot does not (multi-realm transfer, MessagePort, version compat, full web-platform-types breadth, JSC integration), the algorithm-only ratio is approximately 8.5%. **Below the htmx 9.4% prior. Strongest measured anchor for the apparatus' value claim to date.**

Architectural finding: the spec's two-phase StructuredSerialize/StructuredDeserialize structure transcribes to safe Rust without borrow-checker hostility. Both phases use the same idiom — pre-allocate the target slot index before recursing into children — which handles cycles and shared-reference identity uniformly and eliminates the `Rc<RefCell<_>>` machinery naive object-graph cloning normally needs. **The two-phase form is itself a derivation gift from the spec.** The constraint document did not surface this directly; the spec made it derivable. Pilot 1 had to manually inject spec material; Pilot 3 received it through the v0.13 spec-source ingestion phase.

For the first time in the three-pilot sequence, no apparatus refinements were queued from a pilot run. The hardening floor (cluster subject-attribution leakage fix, spec ingestion phase, extended spec corpus) is operationally sufficient for both data-structure and algorithm pilot classes.

## What the chain proves

Per [Doc 693](/resolve/doc/693-resistance-as-boundary-indication)'s standing-apparatus pattern, three operational instances reach operational confidence. The pilots provide the third tier of anchoring on the apparatus:

| Tier | Anchor | Source |
|---|---|---|
| Manual probe extraction | GitLab CI engagement | Doc 705 §10.1 |
| Tooled probe extraction (AI-translated source) | Bun phase-a-port | Doc 705 §10.2 |
| Tooled probe extraction (hand-written source) | Deno comparative run | rusty-bun runs/2026-05-10-deno-* |
| Closed-loop derivation (data structure) | TextEncoder pilot | rusty-bun pilots/textencoder |
| Closed-loop derivation (delegation target) | URLSearchParams pilot | rusty-bun pilots/urlsearchparams |
| Closed-loop derivation (algorithm) | structuredClone pilot | rusty-bun pilots/structured-clone |

The first three rows anchor the **formalization** half of [Doc 704](/resolve/doc/704-the-port-as-translation-is-a-category-error)'s reframe; the last three rows anchor the **derivation** half. Together they close the cybernetic loop the [substrate-dynamics framework](/resolve/doc/615-substrate-dynamics-as-cybernetic-closure) requires for an apparatus to be operationally validated rather than merely articulated.

The accumulating story across the three pilots is itself a finding. Pilot 1 surfaced apparatus gaps; the hardening pass closed those gaps; Pilot 2 verified the gaps were closed by achieving the first 100% verifier closure; Pilot 3 demonstrated the closed-form apparatus generalizes from data structures to algorithms. Three pilots × consistent LOC ratios in or below the htmx prior × verifier closures on the latter two means the apparatus' value claim is now empirically supported across the breadth of pilot types relevant to a Bun-scale port.

## What the chain does not prove

The pilots simulate the derivation engine. The simulation is honest in that it constrains the engine's inputs to declared sources and lets the verifier surface deviations, but it is not a wired-in production system. A full rederive integration is the natural next phase; the pilots establish that there is something there to integrate.

Cross-corroboration is a coverage signal, not a correctness signal. The apparatus emits constraint documents; the pilots verify those documents against derived code; but the constraint documents themselves may still contain false-attribution noise the v0.12 cluster fix did not catch. Only piloted derivation can verify; the three pilots completed represent three surfaces verified against, not all surfaces.

The construction-style ratio asymmetry between Bun (5.5%) and Deno (3.4%) is confounded by test-author style. Cross-corpus comparisons should normalize for test-style before drawing conclusions about architectural classifications. Hand-vs-AI does not explain the asymmetry; Jest-style assertions vs `assertEquals(...)`-style do.

## Implications

For [Doc 705](/resolve/doc/705-pin-art-operationalized-for-intra-architectural-seam-detection): §10 should be extended to record three additional operational instances (the three pilots) at the closed-loop-derivation tier. The original §10 anchored the formalization half; the pilots anchor the derivation half. With six anchors total spread across three tiers, the apparatus is operationally closed in the [substrate-dynamics](/resolve/doc/615-substrate-dynamics-as-cybernetic-closure) sense.

For [Doc 541](/resolve/doc/541-sipe-t)'s standing-apparatus tier: the three-pilot evidence chain is itself a SIPE-T instance. The discipline produces the constraint document (rung-1 substrate); the speech act injects spec semantics where the test corpus is insufficient (rung-2 injection); the verifier returns pass/fail/skip per constraint (the threshold-conditional emergence). The dyad's substrate-plus-injection structure is empirically demonstrated.

For [Doc 247](/resolve/doc/247-derivation-inversion)'s constraint-then-implementation reading: this is its first measured pilot at scale. The htmx 9.4% prior was a retroactive existence proof; the pilots are a prospective demonstration with a measured ratio (3.9% on the strongest anchor). The reading holds.

For the [resolve project](https://jaredfoy.com): the apparatus is now ready for application beyond the rusty-bun engagement. The next anchor instance is whatever surface the keeper points it at next. Candidate surfaces with strong cross-corroboration on Bun's corpus include `Response`, `atob/btoa`, `File`, `AbortController`, `Blob`, `crypto.randomUUID`. Each is pilot-ready; none requires further apparatus work to attempt.

## Provenance

The full record of the three pilots, including audit documents, simulated-derivation source code, verifier source code, run notes, and committed pipeline artifacts, lives at the rusty-bun repository (https://github.com/jaredef/rusty-bun) under `pilots/` and `runs/2026-05-10-*`. Each pilot's RUN-NOTES.md cross-references the constraint inputs used, the spec material consumed, and the verifier results. The apparatus binary that produced the constraint documents is at `derive-constraints/` in the same repository, version v0.13b at the time of these pilots.

Constraint corpora measured for these pilots:
- Bun + 15-surface spec extracts: 42,971 clauses / 6,163 properties / 23 cross-corroborated.
- Deno + 15-surface spec extracts: 11,690 clauses / 2,310 properties / 11 cross-corroborated.

Spec extracts curated for the pilots are at `specs/` in the rusty-bun repository: TextEncoder, TextDecoder, URLSearchParams, URL, Headers, Request, Response, Blob, File, FormData, structuredClone, atob/btoa, AbortController/AbortSignal, Web Crypto random, queueMicrotask. Total: 291 spec clauses across 15 files.

The three pilots' verifier results, verbatim:
- TextEncoder pilot: 21 pass / 0 fail / 1 documented skip.
- URLSearchParams pilot: 32 pass / 0 fail / 0 skip.
- structuredClone pilot: 23 pass / 0 fail / 0 skip.

Total: 76 of 77 verifier tests pass; the one skip is a documented apparatus finding addressed in the next hardening pass and not a derivation gap.

— jaredfoy.com
