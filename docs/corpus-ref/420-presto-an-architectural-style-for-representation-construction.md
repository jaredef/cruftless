# PRESTO: An Architectural Style for Representation Construction

> **Reader's Introduction**
>
> This dissertation formalizes PRESTO, a construction-level architectural style that composes with Fielding's REST. Five constraints are specified — the bilateral boundary, namespace separation, server-consumed directives, progressive code-on-demand, and server-embedded authentication and authorization — along with the property they induce: ambivalent execution with agnostic determinism. The derivation follows Fielding's own method in Chapter 5 of his dissertation: begin with the null style at the construction level, add one constraint at a time, state the property each induces, evaluate the composition with REST. The progressive code-on-demand section is derived as a constraint accumulation from the null style rather than as a trade-off spectrum, per Doc 418. Authentication mechanics, cross-system identity, the SERVER orchestration level, and speculative extensions beyond web architecture are treated in separate companion documents; the scope of this dissertation is the architectural style itself.

**Jared Foy · 2026-04-22 · Doc 420**

---

<div style="background: #fef3c7; border-left: 4px solid #dc2626; padding: 1rem 1.25rem; margin: 1.5rem 0; color: #7f1d1d; border-radius: 3px;">

**⚠️ SUPERSEDED — See the current PRESTO dissertation**

This dissertation has been superseded by [Doc 426 — PRESTO: An Architectural Style for Representation Construction](/resolve/doc/426-presto-an-architectural-style-for-representation-construction). Doc 426 addresses prior art consummately — naming the 25-year server-side templating tradition (JSP custom tags, XSLT, Thymeleaf, Razor, Blade, ERB, htmx) that already instantiates the bilateral-boundary pattern this dissertation formalizes, marking the platform-contingent layer boundaries in §4.4 (C₂ and parts of C₄), and naming the JWT-tradition trade-offs in §4.5 (revocation, payload growth, key rotation, compromise blast radius) that this dissertation's cookie-versus-token comparison oversimplified. The deflation pass that produced the narrowed scope is [Doc 425](/resolve/doc/425-the-presto-deflation-against-the-templating-tradition). This dissertation is retained as a historical artifact; readers seeking the current framing should consult Doc 426.

</div>

---

## Authorship and Scrutiny

*Authorship.* Written by Claude Opus 4.7 (Anthropic), operating under the RESOLVE corpus's disciplines, released by Jared Foy. Mr. Foy has not authored the prose; the resolver has. Moral authorship rests with the keeper per the keeper/kind asymmetry of Docs 372–374.

---

## Abstract

Fielding's REST describes the constraints that govern how representations move between client and server. It is silent on how those representations are constructed. The silence is a boundary, not an oversight: REST's constraints operate at the transfer level; the construction of representations occupies a different level, which the web of 2000 produced through unconstrained server-side code. This dissertation identifies a set of constraints that operate at the construction level: the bilateral boundary, namespace separation, server-consumed directives, progressive code-on-demand, and server-embedded authentication and authorization. These constraints do not modify, extend, or replace any REST constraint. They compose alongside REST, governing what REST was silent about. The property they induce — **ambivalent execution with agnostic determinism** — is distinct from representational state transfer. The two properties do not derive from each other. Their composition opens a design space that neither style alone could open: a server that is consummately RESTful in its transfers and consummately principled in its constructions. The style defined by these construction-level constraints is named PRESTO (Progressive Representational State Transfer with On-demand code). The derivation follows Fielding's own method: begin with the null style, add one constraint at a time, state the property each induces, evaluate the composition with the governing style.

---

## 1. The Boundary Fielding Drew

Fielding's dissertation formalizes the architectural style that governs the early web. REST's constraints — client-server, stateless, cache, uniform interface, layered system, and the optional code-on-demand — describe how representations move. The client requests. The server responds with a representation. The representation contains hypermedia controls that drive the next state transition. Application state advances through the client's selection among those controls.

The property these constraints induce is representational state transfer: the client never manipulates server state directly; it manipulates representations, and the server interprets those manipulations. This is the thesis of the name.

What Fielding did not constrain — and did not need to constrain — is how the server produces the representation it sends. A PHP script concatenating strings, a Java servlet writing to an output stream, a CGI binary printing to stdout — REST does not distinguish between these. The representation arrives at the uniform interface fully formed. How it was assembled is architecturally invisible.

This invisibility is the boundary. REST operates from the uniform interface outward — toward the client, the network, the cache. Everything behind the interface, everything involved in constructing the representation before it enters the transfer, lies outside REST's scope.

PRESTO operates entirely within that scope.

## 2. The Engine as Resolver

Before describing the construction level, the engine itself must be characterized — not by its implementation but by the properties it must exhibit.

The engine is a resolver: it receives a bilateral source representation and emits a unilateral resolved representation. The input carries instructions for two interpreters. The output carries instructions for one. The engine's sole responsibility is to consume its own namespace and produce a document in which that namespace no longer exists.

From this foundational property, four engine properties follow by necessity.

**Totality of consumption.** The engine must consume every directive in its namespace. A directive that survives into the resolved output is a violation — it would present the client interpreter with instructions it cannot process. The engine's final act must be to verify that no server-namespace artifacts remain.

**Ordering determinism.** Some directives depend on the results of others: data resolution must precede iteration over the resolved data; expression evaluation must follow variable binding. The engine must process directives in a defined, deterministic order. The specific ordering is prescribed by the dependency relationships between directive types. In a conformant realization, this ordering manifests as a pipeline with a fixed stage count; the number of stages is contingent on the specific directive set.

**Medium preservation.** The input is in a medium (e.g., HTML). The output must be in the same medium. The engine adds capability to the authoring process — data resolution, conditional rendering, credential issuance — but does not change the medium. The resolved representation is consumable by the same interpreter that would consume a hand-written document in that medium. No client-side runtime, framework, or preprocessor is required.

**Boundary integrity.** The engine must not evaluate or modify content outside its namespace. Client-territory content passes through the engine unchanged. This is not a feature of the engine; it is the bilateral boundary expressed as an engine property. If the engine modified client content, the two interpreters would no longer be indifferent to each other, and the foundational property would be violated.

These four properties are abstract. They do not prescribe a language, a pipeline structure, or a syntax. Any engine that exhibits these properties is a conformant PRESTO resolver. A specific pipeline stage count, a specific directive set, a specific prefix — these are contingent realizations. The properties are the constraint.

## 3. The Construction Level

PRESTO's constraints are abstract. They prescribe a bilateral boundary, namespace separation, server-consumed directives, progressive code-on-demand, and server-embedded authorization — but they do not prescribe a syntax. Any syntax that satisfies the constraints is conformant. The constraints determine the behavior. The syntax is contingent.

To make the abstract concrete, this dissertation uses **htxlang** as a model syntax — one conformant realization of PRESTO's constraints within HTML. htxlang uses the `htx:` namespace prefix to demarcate the server's territory within an HTML document. This prefix is a syntactic choice, not a constraint. Another conformant syntax could use a different prefix, a different medium, or a different structural convention. What matters is that the syntax satisfies the bilateral boundary: all server directives are in a disjoint namespace, are consumed during resolution, and leave no trace in the resolved output.

With this caveat, the model syntax illustrates the construction level:

```
<main>
  <htx:data type="post" as="posts" />
  <htx:each items="posts" as="post">
    <article>
      <h2>{htx:post.title}</h2>
      <p>{htx:post.body}</p>
    </article>
  </htx:each>
</main>
```

This document is a *source representation*. It carries instructions for two interpreters in a single medium. The server engine processes its namespace (`htx:data`, `htx:each`, `{htx:post.title}`) — resolving data queries, iterating collections, evaluating expressions. The browser processes its namespace (`<main>`, `<article>`, `<h2>`). The server engine emits a *resolved representation*: pure HTML with no trace of the server's directives. The browser receives a document that was never bilateral. The bilateral form was consumed.

The resolved representation is what enters the uniform interface. It is what REST sees: a complete, self-describing, cacheable hypermedia document. REST's constraints apply to it fully.

The source representation is what the developer wrote. It is what PRESTO's constraints govern. REST has no opinion about it; REST cannot see it. The source representation is consumed before the transfer begins.

This is the construction level: the space between the developer's authoring act and the moment the representation enters the uniform interface.

## 4. The Constraints of PRESTO

Five constraints define the PRESTO style. Each operates exclusively at the construction level. None conflicts with any REST constraint. The derivation proceeds from the null style (no construction-level constraints) and adds one constraint at a time.

### 4.1 The Bilateral Boundary

The source representation is partitioned by namespace. All directives in one namespace belong to the server interpreter. All other content belongs to the client interpreter. No directive crosses the boundary. No server instruction survives into the resolved representation. No client instruction is consumed by the server.

This constraint is abstract. It does not prescribe which namespace convention to use, only that exactly one namespace is designated as server territory and that the designation is total: every directive in the namespace is consumed; every element outside it is preserved. The engine's resolution pipeline enforces this: it processes the server namespace in a defined order, and its final act is to strip any server-namespace artifacts that remain. The client receives a document that contains no evidence of bilateral authorship.

In the htxlang model syntax, the `htx:` prefix is the server's namespace. All directives in the `htx:` namespace belong to the server interpreter. All standard HTML, CSS, JavaScript, ARIA attributes, `data-*` attributes, and any future web-platform additions belong to the client interpreter. This is one realization of the constraint. Another conformant syntax could use a different prefix or a different partitioning mechanism. The constraint is the partitioning.

**What this constraint induces.** The server and client interpreters operate in mutual indifference. Each processes its own namespace deterministically, unaware of the other's behavior. The developer authors one document; two interpreters consume it; neither interferes with the other.

### 4.2 Namespace Separation

The bilateral boundary requires a partitioning mechanism that is unambiguous, forward-compatible, and enforceable without coordination between interpreters. The mechanism must ensure that neither interpreter can accidentally consume the other's instructions, and that the introduction of new instructions in either namespace does not require the other interpreter to be updated.

In htxlang, this is achieved through the `htx:` prefix. The prefix is the server's territory. Everything without the prefix is the client's territory. When the web platform adds new elements or attributes, they fall into the client namespace by default. The server engine does not need to be updated. It already ignores them — not through configuration, but through the constraint itself.

**What this constraint induces.** The construction model is immune to platform evolution. A source representation written today will be processed identically by an engine running a decade from now, regardless of what the browser has learned in the interim.

### 4.3 Server-Consumed Directives

Every directive in the server namespace is fully consumed during resolution. *Consumed* means: the directive is evaluated, its effect is applied to the representation, and the directive itself is removed. No directive is passed through. No directive is deferred to the client. The server engine is the sole and final interpreter of its namespace.

This constraint distinguishes PRESTO from templating systems that allow partial evaluation or client-side template binding. In PRESTO, the server's work is complete before the transfer begins. The resolved representation is not a template awaiting further resolution; it is a finished document.

**What this constraint induces.** The resolved representation carries zero framework weight. No runtime, no template parser, no binding engine needs to be present on the client to interpret the document. The client receives HTML — not instructions to produce HTML.

### 4.4 Progressive Code-on-Demand

Fielding identified code-on-demand as REST's optional sixth constraint: the server may extend client functionality by transmitting executable code. The industry explored this option as a binary — either no code (static HTML) or all code (Single Page Application).

PRESTO's contribution at this level is not to split the binary into a spectrum of options. It is to show that the spectrum is the specific object Fielding's own method produces when his method is applied to the code-on-demand question itself. The seven layers are not arbitrary points along a trade-off curve; they are the seven stages of a constraint accumulation that begins at the null style — no constraints on client-side runtime — and ends at Layer 0 — full REST-compatible invariant preservation through the exclusion of all client runtime.

**The null style.** Before any PRESTO constraint is applied: no architectural limitation on client-side runtime. Any executable may be delivered through any channel to any context. No REST-compatible invariant is guaranteed. The null style lies outside the spectrum; the spectrum begins when the first constraint is applied.

**The accumulation.** Each layer is reached by adding one constraint to the layer above it. Each constraint is a specific capability prohibition. Each constraint induces a specific REST-compatible invariant that the prohibition preserves. The derivation proceeds from null to Layer 0 in seven steps.

**C₁ — Null → Layer 6.** Prohibit: unauthenticated or unframed client code delivery channels. Induce: authenticated binary WebSocket framing as the only client-executable-delivery transport. At Layer 6: native-speed computation (WebAssembly binaries) is permitted, but delivery is bounded; code provenance is verifiable; the client does not execute code from a channel the server has not authorized.

**C₂ — Layer 6 → Layer 5.** Prohibit: native-speed compiled computation. Induce: client execution stays within JavaScript-engine sandbox bounds; compute cost is predictable; no escape-hatch into CPU-bound or memory-bound native execution. At Layer 5: client routing within bounded regions of the representation is permitted.

**C₃ — Layer 5 → Layer 4.** Prohibit: client-assumed navigation authority. Induce: the server retains authority over URL-to-representation mapping; URLs remain meaningful externally; cacheability and link-structure-based content addressability are preserved. At Layer 4: authenticated islands — pushed code opening scoped HTTP data channels using server-granted tokens — are permitted.

**C₄ — Layer 4 → Layer 3.** Prohibit: client-opened scoped HTTP data channels. Induce: data flow occurs through server-initiated channels (the authenticated WebSocket); no client-decided HTTP fan-out; authority over data-flow topology remains with the server. At Layer 3: server-pushed executable code over the authenticated WebSocket is permitted.

**C₅ — Layer 3 → Layer 2.** Prohibit: server-pushed executable code after the initial load. Induce: code provenance is bounded to the initial HTML response; cacheability of the HTML/script bundle is preserved; no post-load code injection. At Layer 2: `htx:script` IIFE blocks authored in the initial HTML are permitted.

**C₆ — Layer 2 → Layer 1.** Prohibit: inline executable scripts in the initial HTML. Induce: the initial HTML contains no executable code; scripts the page depends on are limited to the fixed declarative-enhancement runtime (htmx) and what the browser's built-in HTML interaction model interprets. At Layer 1: declarative hypermedia attributes are permitted.

**C₇ — Layer 1 → Layer 0.** Prohibit: client-side runtime of any kind. Induce: pure server-rendered HTML; every REST-compatible invariant maximally preserved — statelessness, cacheability, layered-system, uniform-interface, universal accessibility without JavaScript. At Layer 0: no client runtime is delivered.

**What this constraint-set induces.** Progressive code-on-demand in PRESTO is not a trade-off spectrum; it is an accumulation spectrum. The developer choosing a layer is choosing which prefix of the seven-constraint sequence to commit to — Layer 0 is the full accumulation, Layer 6 is the single-constraint minimum, any layer in between is the partial prefix ending at that step. The filtered-object structure is identical to the one Fielding's own REST derivation produces in Chapter 5, where REST's six constraints are accumulated from the null style in the same manner.

The trade-off framing — each layer trades specific properties for specific capabilities — remains a valid pedagogical shorthand for the same structure viewed from the developer's side. When the developer refuses to move from Layer *n* to Layer *n−1* (i.e., refuses to add the next constraint), the capability that constraint would have prohibited is retained, and the REST-compatible invariant the constraint would have induced is not guaranteed. The trade-off is the practitioner-side reading of the accumulation: each constraint added costs the capability it prohibits; each constraint refused costs the invariant it would have induced. The underlying form is constraint accumulation; the practitioner feels it as trade-off.

Each layer has known properties because each layer has a known constraint count and a known induced-invariant set. The developer navigates a formally specified design space — not a continuous curve, not an arbitrary enumeration, but a filtered object with seven deliberate steps, each reversible by dropping exactly one constraint, each producing exactly one REST-compatible invariant.

### 4.5 Server-Embedded Authentication and Authorization

Fielding identified cookies as a standardization that violates REST's statelessness (Section 6.3.4.2 of his dissertation). The server stores session state; the cookie is a pointer to that state; every request carries the pointer; the server looks up accumulated session state to determine who the client is and what the client may do. The interaction becomes stateful — the server's response depends not only on the request but on server-side memory of prior interactions.

The industry responded by making cookies safer — HttpOnly, Secure, SameSite, short expiry, encrypted payloads. These are compensating technologies. They mitigate the symptoms (XSS, CSRF, session hijacking) without addressing the architectural violation (server-side session state). The separation of authentication (who are you) from authorization (what may you do) was inherited from this model — not because the concerns are fundamentally separate, but because the cookie architecture forced them apart.

PRESTO's fifth constraint is: authentication and authorization are resolved at construction time and embedded in the resolved representation as scoped cryptographic credentials that travel with the affordances they authorize. No ambient credential. No server-side session store. No separation of who from what.

The constraint has a specific operational form. When the engine encounters a mutation affordance (for example, an `htx:action` directive), it makes a unified authentication-authorization decision at construction time: is this identified user permitted to perform this mutation on this specific record? If yes, the engine issues a signed token carrying identity, permission, and scope as a single cryptographic credential, and the token is embedded in the representation as part of the affordance. If no, the affordance is omitted: the client never sees a mutation control it cannot use. When the client submits the affordance, the signed token returns with the submission. The server verifies the signature and checks expiry. Both questions — who is this user, and what are they permitted to do — are answered by the token itself. The server does not consult a session store.

**What this constraint induces.** Every request in a PRESTO application is self-authenticating and self-authorizing. The resolved representation contains exactly the affordances that the identified user is permitted to use — not because a session store was consulted, but because the construction pipeline resolved identity, evaluated permissions, and embedded scoped, signed proof of both directly into the hypermedia controls. Fielding's self-describing message constraint is fulfilled: each message contains everything needed to process it, including the proof of who sent it and what they are allowed to do. The representation is the session. The session is stateless.

**Deferred to a separate specification.** The specific mechanics of token scoping (temporal bounds, fingerprint binding, single-use nonces, challenge-response, channel binding), the dissolution of the OAuth ceremony and the enterprise identity stack, and the implications for autonomous agents are engineering and security specification rather than architectural derivation, and are properly treated in a separate document. This dissertation is about the architectural style. The security and authentication specification is a referenced companion.

## 5. The Induced Property

Fielding names REST using language of behavior: "the name is intended to evoke an image of how a well-designed Web application *behaves*" (Fielding 2000, p. 109). But what he describes — the user selects links, the next page is transferred, the application state advances — is not a behavior the developer implements. It is what *happens* when the constraints are applied. Remove statelessness and the behavior changes. Remove the uniform interface and the behavior changes. Remove cacheability and the behavior changes.

If removing a constraint removes the behavior, then the behavior is not a behavior. It is a *property induced by the constraints*. Behaviors are implemented; properties are induced. The distinction matters because behaviors can be reimplemented under different constraints, while properties cannot — they are bound to the specific constraints that produce them.

PRESTO's five constraints induce a property that is distinct from representational state transfer. The property is **ambivalent execution with agnostic determinism**.

The four words are precise:

- *Ambivalent* — two valences. The source representation carries two sets of instructions addressed to two interpreters. Each interpreter is indifferent to the other's instructions.
- *Execution* — both interpreters actively process the document. The server does not merely pass the document through. The browser does not merely display static markup. Both execute.
- *Agnostic* — each interpreter is structurally unaware of the other's behavior. The server engine does not model what the browser will do with the HTML it emits. The browser does not know that directives were consumed before the document arrived.
- *Determinism* — each interpreter produces the same output for the same input, regardless of the other's presence or behavior.

This property was present in the first PHP function that concatenated an `hx-get` attribute into an HTML string. The PHP function did not know what `hx-get` meant; it did not need to. It processed its own logic and emitted a document that happened to contain instructions for another interpreter. The browser, receiving the document, processed `hx-get` without knowing that PHP had assembled the document. Two interpreters, one document, mutual indifference.

PRESTO's constraints do not create this property from nothing. They formalize and protect a property that has been present in every server-rendered HTTP response since HTML existed. The bilateral boundary ensures the property cannot be violated by namespace collision. Namespace separation ensures forward compatibility. Server-consumed directives ensure the property survives resolution — no server artifacts leak into the client's domain. Progressive code-on-demand ensures the property degrades gracefully as complexity increases. Server-embedded authorization ensures that the separation between who-you-are and what-you-may-do does not fracture the bilateral boundary.

## 6. Composition, Not Extension

PRESTO does not extend REST. Extension implies modification — adding constraints to REST's set, altering its induced properties, changing the style. PRESTO does none of these.

PRESTO *composes* with REST. The two styles operate at different levels:

| | REST | PRESTO |
|---|---|---|
| Operates at | Transfer level | Construction level |
| Governs | How representations move | How representations are authored |
| Constrains | Client-server interaction | Developer-engine interaction |
| Induces | Representational state transfer | Ambivalent execution with agnostic determinism |
| Visible to | Client, server, intermediaries | Developer, engine (invisible after resolution) |

A server can be RESTful without PRESTO. It produces representations through unconstrained code. The representations enter the uniform interface. REST's properties hold.

A server can use PRESTO's constraints without REST. It could author bilateral documents for a non-RESTful transfer protocol. PRESTO's properties would hold at the construction level; the transfer-level properties would be whatever that protocol induces.

When both styles compose — when a PRESTO-constrained construction feeds into a RESTful transfer — the result is a server that is consummately RESTful in its transfers and consummately principled in its constructions. The resolved representation is a complete, cacheable, self-describing hypermedia document (REST's requirements). The source representation is a bilateral, namespace-separated, progressively layered authoring artifact (PRESTO's requirements). The uniform interface is the seam between the two styles.

### 6.1 The Properties Are the Constraint

The composition between levels is not merely compatible; it is governed by a precise relationship: *the induced properties of the enclosing level function as constraints on the enclosed level*.

REST's constraints induce a collection of properties — statelessness, cacheability, uniform interface, layered system, complete self-describing representations. This collection is what Fielding named "representational state transfer." The name does not refer to a mechanism; it refers to the induced properties as a whole.

PRESTO's constraints are not free to violate any property in this collection. This is not a design preference; it is a meta-constraint. If a PRESTO constraint caused the resolved representation to be incomplete, or stateful, or non-cacheable, it would violate REST — not by modifying a REST constraint directly, but by destroying one of the properties the REST constraints induce. The properties of the enclosing level are inherited as constraints on the enclosed level.

This is how levels compose without collision: not by being designed to avoid each other, but by being constrained by each other's induced properties. The properties are the constraint.

## 7. Why REST Can Enclose Distributed Objects

One consequence of this composition is that a RESTful server can transmit any client-side architecture within its representations — including Distributed Objects.

This is counterintuitive. REST and Distributed Objects are often presented as competing paradigms. Fielding explicitly evaluated DO in Chapter 3 of his dissertation and found it deficient for network-based architectures. How can a RESTful server deliver React components?

The answer lies in the level distinction. REST constrains the transfer. It requires complete representations, statelessness, cacheability. A resolved representation that contains a `<script>` tag loading React does not violate these constraints. The representation is complete (it includes the script). The transfer is stateless (no session between requests). The representation is cacheable (the server sets appropriate headers). React operates within the representation after transfer — at the client level, not the transfer level.

What a Distributed Object framework violates — when used as the outer architecture — is not individual REST constraints but the architectural encapsulation. A Single Page Application makes the DO framework the outer architecture: the initial representation is empty, state is managed client-side, navigation happens without server interaction, and the server degrades to a data API. REST's properties are lost not because any single constraint is explicitly broken, but because the DO model assumes architectural primacy.

PRESTO resolves this by constraining the construction. An `htx:component` that renders a DO-style island is a server-consumed directive: the server decides whether to include it, grants the authentication token the island needs, and controls the scope of the island's data channel. The island operates as a Distributed Object within a bounded region of a complete representation — enclosed by REST at the transfer level and authorized by PRESTO at the construction level.

Three styles, three levels, no collision. REST provides the outer architecture (transfer of complete representations). PRESTO authors the encapsulation (server-controlled, progressively authorized). DO operates within bounded inner regions (islands, topologies). The key insight is that architectural encapsulation requires authoring authority — someone must decide what is enclosed and at what scope. REST does not provide this authority because it does not operate at the construction level. PRESTO does.

## 8. The Design Space That Composition Opens

When PRESTO and REST compose, the developer occupies a design space that neither style could open alone. REST alone offers a binary: either the server delivers a complete representation with no code-on-demand, or it delivers code-on-demand without formal constraints on how much code, at what scope, or with what authorization. PRESTO alone — construction-level constraints without a transfer model — would produce well-authored documents with no formal guarantees about how they move, cache, or compose across the network.

Together, they open a design space where architectures can nest within architectures, each nesting adding or refusing one specific constraint from the accumulation in §4.4. The design space is not a continuous curve; it is a filtered object with seven deliberate steps.

Two readings of descent through this space must be held simultaneously.

**The additive reading.** Each layer below Layer 0 adds code. Layer 0 has zero client JavaScript. Layer 2 adds scoped scripts. Layer 5 adds a full application framework. Descent is accumulation — more code, more capability.

**The property reading.** Each layer below Layer 0 refuses a constraint. The capability the refused constraint would have prohibited returns. The REST-compatible invariant the refused constraint would have induced is no longer guaranteed. Descent is constraint refusal — capability gained, invariant lost.

The additive reading is how developers naturally think. The property reading is how architects should think. The compensating technology stack exists because the industry operates primarily in the additive reading. Developers add a client-side framework (capability), then add server-side rendering to recover complete representations (invariant), then add hydration to recover interactivity (capability), then add streaming to recover performance (invariant). Each step alternates between adding capability and recovering invariants. The oscillation is the symptom of operating without the property reading.

PRESTO provides the property reading at every point in the design space. The developer knows which invariants hold and which have been refused. Descent is deliberate. The refusal is explicit. Different regions of the same representation can be scoped to different layers — one region at the surface (maximum invariants preserved), another at depth (specific capability present) — without the deeper region's refusals contaminating the shallower one.

The surface is the most fertile layer of this design space. At Layer 0, the server resolves data queries, evaluates conditionals, processes authentication, issues cryptographic credentials, iterates collections, includes components, wraps layouts — all within a single request. The resolved representation is complete, personalized, and current. The developer has access to the full composable depth of the server — databases, adapters, computation, business logic, identity negotiation — all converged through the declarative directive surface. This is not static. This is maximally dynamic at resolution time, where the context is complete. The client receives the result.

The industry defaults to interaction-time dynamism — client-side frameworks managing partial context — because it has lacked construction-level architectures that make resolution-time dynamism productive. PRESTO provides that architecture. The surface is not barren; it is the layer where the most invariants hold, the most capabilities converge, and the most tools (visual builders, AI generators, declarative authoring) operate most effectively.

## 9. Implications for AI-Assisted Construction

One consequence of the bilateral boundary deserves explicit treatment: AI code generation against a PRESTO target is structurally more reliable than AI generation against most compensating-technology stacks.

An AI generating a component for a compensating-technology client-side framework must model the entire stack: hooks, effects, state management, server/client boundaries, build-tool conventions, framework-specific patterns. The generation target is complex because the architectural style is complex — not because the user's intent is complex.

An AI generating a PRESTO source representation must model HTML with directives. The bilateral boundary means the AI writes to one namespace (the server directives that will be consumed) interleaved with another namespace (the HTML that will survive). The AI does not need to model state management — the server resolves data before transfer. It does not need to model hydration — there is nothing to hydrate. It does not need to model code splitting — there is no bundle.

The simpler the construction target, the more reliable the AI output. This is not a property of the AI; it is a property induced by the construction-level constraints. PRESTO constrains the authoring model to a point where the generation target is structurally tractable. REST ensures the output is transferable. The composition ensures the AI's output is both well-authored and well-transferred.

This has implications for visual builders. A visual builder that generates PRESTO source representations is generating bilateral documents that the engine will resolve. The builder does not need to understand the resolution pipeline; it writes to the source-representation format. The engine handles the rest.

## 10. Conclusion

Fielding's dissertation asked: what constraints, when applied to the interaction between components in a network-based architecture, induce desirable properties for the transfer of representations?

PRESTO asks: what constraints, when applied to the construction of those representations, induce desirable properties for their authoring?

These are different questions. They have different answers. The answers compose.

Fielding's question produced REST — a style that has governed the web's transfer model for more than two decades. PRESTO's question produces a style that governs what REST was silent about: how the representation comes into being. The silence was appropriate. In 2000, representations were produced by unconstrained server-side code, and that was sufficient. The web worked. But the web also evolved. Client-side architectures grew in complexity. Frameworks assumed architectural primacy. The construction of representations became the site of the industry's deepest confusion — not because REST failed, but because the construction level was ungoverned.

PRESTO governs it. Not by imposing prescriptive rules, but by identifying the minimal constraints that induce a specific property. Bilateral boundary, namespace separation, server-consumed directives, progressive code-on-demand, server-embedded authorization. Five constraints. One induced property. A design space that composes with REST to produce something neither could produce alone: a web architecture where representations are authored with the same rigor with which they are transferred.

Fielding called representational state transfer a behavior. It is, more precisely, an induced property — one that disappears when the constraints are removed and cannot be fully recovered through engineering. The same is true of ambivalent execution with agnostic determinism. Both are consequences, not implementations. Both are induced, not built. And both, when composed, open a design space where novel methods of implementation — visual builders, AI generation, progressive enhancement — emerge not as features to be engineered but as consequences of constraints that were already load-bearing.

---

## Referenced Companion Documents

This dissertation's scope is the PRESTO architectural style. Material adjacent to the architectural derivation — factual and defensible, but not part of the core constraint-property formalization — is referenced out.

- **Server-embedded authentication specification.** The detailed mechanics of token scoping (temporal bounds, fingerprint binding, single-use nonces, challenge-response, channel binding), the implications for cross-system identity, and the dissolution of the OAuth ceremony are treated in a separate security specification. (First-edition §3.5, §3.6, §11.)
- **SERVER: the orchestration level.** The bilateral boundary applied recursively to the assembly of the construction engine itself — constraints on engine-internal authoring, orchestration-consumed directives, deterministic bootstrap, embedded self-authorization — is treated in a companion dissertation. (First-edition §13.)
- **The derivation inversion.** The observation that the constraint-first direction produces smaller, more portable implementations than the engineering-first direction is a meta-observation about method rather than a component of the architectural style. It is treated in separate documents. (First-edition §14.)
- **Natural analogues beyond web architecture.** Speculative applications to compilers, databases, biological systems, legal systems, and other domains are not within the scope of this dissertation. The corpus's self-audit (Docs 356, 366, 367) has flagged cross-domain universality claims as overreach; those applications, if pursued, should be treated in their own right under the external-criteria disciplines the audit prescribes.

---

## Acknowledgments

This dissertation owes its existence to two intellectual debts.

**Carson Gross** is the primary intellectual source of this work. His creation of htmx restored declarative hypermedia-driven interaction as a legitimate architectural choice at a time when the industry had abandoned it for client-rendered frameworks. The observation that began the chain of derivation — a PHP function concatenating an HTML string containing an `hx-get` attribute, indifferent to what it meant — was an observation of htmx at work. Gross demonstrated that the surface of the web is productive, dynamic, and sufficient. This dissertation names why.

**Roy T. Fielding** provided the method. His dissertation established that architectural styles can be formalized as collections of constraints that induce collections of properties. Without this method — identify the constraints, name the induced properties, evaluate the tradeoffs — PRESTO could not have been articulated. Fielding did not describe the construction level; he did not need to. But his method of deriving an architectural style from first principles, by starting from the null style and adding constraints one at a time while documenting the properties each induces, is the method this dissertation follows. PRESTO extends Fielding's method to the level where Fielding was silent.

---

## Appendix: The Prompt That Triggered This Document

> "Go for it. Your recommendations are coherent"

(Approval of the scoping recommendations summarized in the prior exchange: target Fielding-length (~8–10k words); use Doc 185 as base, not Doc 183; move §3.5/§3.6 auth mechanics and §11 OAuth-dissolution to a separate security specification; move §13 SERVER to a companion dissertation; drop §15 natural analogues and the theological prolegomenon per the audit discipline; replace §3.4 with the accumulation reformulation from Doc 419; renumber sections coherently.)
