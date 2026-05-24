# PRESTO: An Architectural Style for Representation Construction

> **Reader's Introduction**
>
> This dissertation formalizes PRESTO, a construction-level architectural style that composes with Fielding's REST. Five constraints are specified — the bilateral boundary, namespace separation, server-consumed directives, progressive code-on-demand, and server-embedded authentication and authorization — along with the property they induce: ambivalent execution with agnostic determinism. The derivation follows Fielding's method in Chapter 5 of his 2000 dissertation: begin with the null style at the construction level, add one constraint at a time, state the property each induces, evaluate the composition with REST. The progressive code-on-demand section is derived as a constraint accumulation from the null style. Companion documents — referenced at the close — treat authentication mechanics, the SERVER orchestration level, and the relationship to the broader PRESTO corpus. Conceptual foundations are traced in §12; prior art is catalogued comprehensively in §13.

---

## Abstract

Fielding's REST describes the constraints that govern how representations move between client and server. It is silent on how those representations are constructed — an explicit scope boundary in the 2000 dissertation, left open for subsequent architectural-style formalization. This dissertation identifies a set of constraints that operate at the construction level: the bilateral boundary, namespace separation, server-consumed directives, progressive code-on-demand, and server-embedded authentication and authorization. These constraints do not modify, extend, or replace any REST constraint. They compose alongside REST, governing what REST left unscoped. The property they induce — *ambivalent execution with agnostic determinism* — is distinct from representational state transfer. The two properties do not derive from each other. Their composition opens a design space that neither style alone could open. The style defined by these construction-level constraints is named PRESTO (Progressive Representational State Transfer with On-demand code). The derivation follows Fielding's own method.

---

## 1. The Boundary Fielding Drew

Fielding's dissertation formalizes the architectural style that governed the early web. REST's constraints — client-server, stateless, cache, uniform interface, layered system, and the optional code-on-demand — describe how representations move. The client requests. The server responds with a representation. The representation contains hypermedia controls that drive the next state transition.

The property these constraints induce is representational state transfer. What Fielding did not constrain — and did not need to constrain — is how the server produces the representation it sends. REST does not distinguish between the possible construction mechanisms. The representation arrives at the uniform interface fully formed. How it was assembled is architecturally invisible.

This invisibility is the boundary. REST operates from the uniform interface outward. Everything behind the uniform interface, everything involved in constructing the representation before it enters the transfer, lies outside REST's scope.

PRESTO operates entirely within that scope.

## 2. The Engine as Resolver

Before specifying the constraints, the engine that processes them must be characterized — not by its implementation but by the properties it must exhibit.

The engine is a *resolver*: it receives a bilateral source representation and emits a unilateral resolved representation. The input carries instructions for two interpreters. The output carries instructions for one. The engine's sole responsibility is to consume its own namespace and produce a document in which that namespace no longer exists.

From this foundational property, four engine properties follow by necessity.

**Totality of consumption.** The engine must consume every directive in its namespace. A directive that survives into the resolved output presents the client interpreter with instructions it cannot process.

**Ordering determinism.** Some directives depend on the results of others: data resolution must precede iteration over the resolved data; expression evaluation must follow variable binding. The engine processes directives in a deterministic order prescribed by the dependency relationships between directive types.

**Medium preservation.** The input is in a medium. The output is in the same medium. The engine adds capability to the authoring process but does not change the medium.

**Boundary integrity.** The engine does not evaluate or modify content outside its namespace. Client-territory content passes through unchanged. This is the bilateral boundary expressed as an engine property.

These four properties are abstract. Any engine that exhibits them — in any language, with any internal architecture — is a conformant PRESTO resolver.

## 3. The Construction Level

PRESTO's constraints are abstract. They prescribe a bilateral boundary, namespace separation, server-consumed directives, progressive code-on-demand, and server-embedded authorization — but they do not prescribe a syntax. Any syntax that satisfies the constraints is conformant. The constraints determine the behavior. The syntax is contingent.

To make the abstract concrete, this dissertation uses **htxlang** as a model syntax — one conformant realization of PRESTO's constraints within HTML. htxlang uses the `htx:` namespace prefix to demarcate the server's territory within an HTML document. This prefix is a syntactic choice, not a constraint.

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

This document is a *source representation*. It carries instructions for two interpreters in a single medium. The server engine processes its namespace (`htx:data`, `htx:each`, `{htx:post.title}`) — resolving data queries, iterating collections, evaluating expressions. The browser processes its namespace (`<main>`, `<article>`, `<h2>`). The server engine emits a *resolved representation*: pure HTML with no trace of the server's directives.

The resolved representation is what enters the uniform interface. It is what REST sees: a complete, self-describing, cacheable hypermedia document.

The source representation is what the developer wrote. It is what PRESTO's constraints govern. REST has no opinion about it; REST cannot see it. The source representation is consumed before the transfer begins.

This is the construction level: the space between the developer's authoring act and the moment the representation enters the uniform interface.

## 4. The Constraints of PRESTO

Five constraints define the PRESTO style. Each operates exclusively at the construction level. The derivation proceeds in Fielding's method: begin with the null style, add one constraint at a time, state the property each induces.

### 4.1 The Bilateral Boundary

The source representation is partitioned by namespace. All directives in one namespace belong to the server interpreter. All other content belongs to the client interpreter. No directive crosses the boundary. No server instruction survives into the resolved representation. No client instruction is consumed by the server.

This constraint is abstract. It does not prescribe which namespace convention to use, only that exactly one namespace is designated as server territory and that the designation is total: every directive in the namespace is consumed; every element outside it is preserved.

**What this constraint induces.** The server and client interpreters operate in mutual indifference. Each processes its own namespace deterministically, unaware of the other's behavior.

### 4.2 Namespace Separation

The bilateral boundary requires a partitioning mechanism that is unambiguous, forward-compatible, and enforceable without coordination between interpreters. The mechanism must ensure that neither interpreter can accidentally consume the other's instructions, and that the introduction of new instructions in either namespace does not require the other interpreter to be updated.

**What this constraint induces.** The construction model is immune to platform evolution in the client namespace. A source representation written today will be processed identically by an engine running a decade from now, regardless of what the client interpreter has learned in the interim.

### 4.3 Server-Consumed Directives

Every directive in the server namespace is fully consumed during resolution. *Consumed* means: the directive is evaluated, its effect is applied to the representation, and the directive itself is removed. No directive is passed through. No directive is deferred to the client. The server engine is the sole and final interpreter of its namespace.

**What this constraint induces.** The resolved representation carries zero framework weight. No runtime, no template parser, no binding engine needs to be present on the client to interpret the document.

### 4.4 Progressive Code-on-Demand

Fielding identified code-on-demand as REST's optional sixth constraint. This dissertation specifies code-on-demand as a progressive spectrum with seven layers rather than a binary choice. Each layer is reached by adding one constraint to the layer above, in Fielding's accumulation method.

Starting from the null style (no architectural limitation on client-side runtime), seven constraints are added in sequence. Each constraint is a specific capability prohibition. Each induces a specific REST-compatible invariant.

**C₁ — Null → Layer 6.** Prohibit: unauthenticated or unframed client code delivery channels. Induce: authenticated binary WebSocket framing as the only client-executable-delivery transport. *Principled.*

**C₂ — Layer 6 → Layer 5.** Prohibit: native-speed compiled computation. Induce: client execution stays within JavaScript-engine sandbox bounds. *Platform-contingent.* The distinction between WASM-native-speed execution and JS-engine execution reflects where current web platforms draw the line; on a future platform where the distinction dissolves, this layer's boundary dissolves with it. The constraint is a real platform fact, not a pure architectural principle.

**C₃ — Layer 5 → Layer 4.** Prohibit: client-assumed navigation authority. Induce: server retains authority over URL-to-representation mapping. *Principled.*

**C₄ — Layer 4 → Layer 3.** Prohibit: client-opened scoped HTTP data channels. Induce: data flow through server-initiated channels. *Partially principled.* The server-versus-client initiation distinction is architecturally meaningful; the specific HTTP-versus-WebSocket framing is platform-contingent.

**C₅ — Layer 3 → Layer 2.** Prohibit: server-pushed executable code after initial load. Induce: bounded code provenance. *Principled.*

**C₆ — Layer 2 → Layer 1.** Prohibit: inline executable scripts in the initial HTML. Induce: initial HTML contains no executable code. *Principled.*

**C₇ — Layer 1 → Layer 0.** Prohibit: client-side runtime of any kind. Induce: pure server-rendered HTML; every REST-compatible invariant maximally preserved. *Principled.*

Five of the seven layer transitions are principled; two (C₂ and parts of C₄) are contingent on current web-platform specifics. The spectrum's structure is Fielding-style accumulation; the specific layer cut points reflect a mix of architectural invariants and current platform state.

**What this constraint induces.** The developer navigates a formally specified design space. For any layer, the constraint count and induced-invariant set are known. The trade-off framing — each layer "trades specific properties for specific capabilities" — is a valid pedagogical shorthand for the practitioner-side reading of the accumulation.

### 4.5 Server-Embedded Authentication and Authorization

Identity and permission are resolved at construction time and embedded in the resolved representation as scoped cryptographic credentials that travel with the affordances they authorize. No ambient credential. No server-side session store. No separation of who-you-are from what-you-may-do.

The constraint has a specific operational form. When the engine encounters a mutation affordance, it makes a unified authentication-authorization decision at construction time: is this identified user permitted to perform this mutation on this specific record? If yes, the engine issues a signed token carrying identity, permission, and scope as a single cryptographic credential, and the token is embedded in the representation as part of the affordance. If no, the affordance is omitted: the client never sees a mutation control it cannot use. When the client submits the affordance, the signed token returns with the submission. The server verifies the signature and checks expiry. Both questions — who is this user, and what are they permitted to do — are answered by the token itself.

**What this constraint induces.** Every request in a PRESTO application is self-authenticating and self-authorizing. The resolved representation contains exactly the affordances that the identified user is permitted to use. Fielding's self-describing message constraint is fulfilled: each message contains everything needed to process it, including the proof of who sent it and what they are allowed to do. The representation is the session. The session is stateless.

The specific engineering mechanics of token scoping — temporal bounds, fingerprint binding, single-use nonces, challenge-response, channel binding — and the trade-offs inherent in any embedded-token scheme belong to a separate companion security specification referenced in §14.

## 5. The Induced Property

Fielding uses behavior-language for REST: "the name is intended to evoke an image of how a well-designed Web application *behaves*." But what he describes is not a behavior the developer implements. It is what *happens* when the constraints are applied. Remove statelessness and the behavior changes. Remove the uniform interface and the behavior changes. A behavior that disappears when a constraint is removed is not a behavior — it is a *property induced by the constraints*.

PRESTO's five constraints compose to induce a property distinct from representational state transfer. The property is **ambivalent execution with agnostic determinism**.

The four words are precise. *Ambivalent* — two valences; the source representation carries two sets of instructions addressed to two interpreters. *Execution* — both interpreters actively process the document. *Agnostic* — each is structurally unaware of the other's behavior. *Determinism* — each produces the same output for the same input.

PRESTO's constraints do not create this property from nothing. They formalize and protect it. The bilateral boundary ensures the property cannot be violated by namespace collision. Namespace separation ensures forward compatibility. Server-consumed directives ensure the property survives resolution — no server artifacts leak into the client's domain. Progressive code-on-demand ensures the property degrades gracefully as complexity increases. Server-embedded authorization ensures the separation between who-you-are and what-you-may-do does not fracture the bilateral boundary.

## 6. Composition, Not Extension

PRESTO does not extend REST. It composes with REST. The two styles operate at different levels:

| | REST | PRESTO |
|---|---|---|
| Operates at | Transfer level | Construction level |
| Governs | How representations move | How representations are authored |
| Constrains | Client-server interaction | Developer-engine interaction |
| Induces | Representational state transfer | Ambivalent execution with agnostic determinism |
| Visible to | Client, server, intermediaries | Developer, engine (invisible after resolution) |

A server can be RESTful without PRESTO: representations are produced through unconstrained code and enter the uniform interface. A server can use PRESTO's constraints without REST: bilateral documents for a non-RESTful transfer protocol. When both styles compose, the result is a server that is RESTful in its transfers and principled (by this constraint set) in its constructions.

The composition is governed by a precise relationship: *the induced properties of the enclosing level function as constraints on the enclosed level*. PRESTO's constraints are not free to violate any property REST induces. If a PRESTO constraint caused the resolved representation to be incomplete, or stateful, or non-cacheable, it would violate REST — not by modifying a REST constraint directly, but by destroying a property REST's constraints induce.

The full recursive structure of this inheritance (each level's induced properties as the null-style starting set for the next level) is stated separately in Doc 424 (SIPE Architectural Form).

## 7. Enclosure of Distributed Objects within RESTful Transfer

One configuration this composition supports is the enclosure of Distributed Object architectures within a RESTful transfer. REST constrains the transfer level — complete representations, statelessness, cacheability. A resolved representation that contains a reference to a client-side runtime does not violate these constraints. The representation is complete; the transfer is stateless; the representation is cacheable. The client-side architecture operates within the representation after transfer — at the client level, not the transfer level. What such an architecture violates *when used as the outer architecture* is not individual REST constraints but the architectural encapsulation: the representation would become empty, state would be managed client-side, navigation would happen without server interaction.

Construction-level constraints resolve this by giving the server *authoring authority* over which enclosed regions are included, under what grants, at what scope. PRESTO's server-consumed `htx:component` directive, with its scoped token grant, is one instance of this authority.

## 8. The Design Space That Composition Opens

When PRESTO and REST compose, the developer occupies a design space where architectures can nest within architectures, each nesting adding or refusing one specific constraint from the accumulation in §4.4. Different regions of the same representation can be scoped to different layers — one region at Layer 0 (maximum invariants preserved), another at a higher layer (specific capability present) — without the deeper region's refusals contaminating the shallower one.

The practitioner-facing consequences of this design space — the three-phase lifecycle, latent binding, the geography of client state, distributed affordance coordination, per-layer composability — are treated in Doc 421, the practitioner companion.

## 9. Implications for AI-Assisted Construction

AI generation against a PRESTO target is structurally reliable: the AI writes bilateral source representations; the engine handles the rest. The generation target does not require modelling state management, hydration, or code splitting. The simpler the construction target, the more reliable the AI output. PRESTO constrains the authoring model to a point where the generation target is structurally tractable.

## 10. What This Dissertation Claims

Stated precisely:

**It claims.** (a) The bilateral-boundary pattern implicit in server-side construction practice can be formalized at the architectural-style level in Fielding's method, as a specific constraint set inducing a specific property. (b) The property induced by the constraint set — ambivalent execution with agnostic determinism — is distinct from representational state transfer and is stated formally here. (c) The progressive code-on-demand spectrum, when derived as Fielding-style constraint accumulation, is a specific instance of the method applied to a design space that was previously treated either as a binary or as a loosely enumerated set of framework-specific options.

**It does not claim.** (a) PRESTO invents the bilateral boundary, namespace separation, server-consumed directives, or the server-embedded-authorization pattern; each is prior art, catalogued in §13. (b) PRESTO's specific code-on-demand layer boundaries are architecturally principled throughout; two of seven transitions (§4.4 C₂ and parts of C₄) are platform-contingent. (c) PRESTO's server-embedded-auth approach is unambiguously superior to alternative credential architectures; each has trade-offs, treated in the companion security specification referenced in §14.

The specific contribution is the application of Fielding's method to the construction level — a tier the REST-successor tradition (ARRESTED [Khare and Taylor, 2004], CREST [Erenkrantz, Taylor, Gorlick, Baquero, 2009], COAST, retrospectively surveyed in "Reflections on REST" [Fielding et al., ESEC/FSE 2017]) has not previously covered. The method is Fielding's; the tradition of extending it to new tiers is established. This dissertation extends that tradition specifically to the tier at which representations are authored before transfer — a tier Fielding explicitly scoped out of his 2000 dissertation. The result is an architectural-style-level formalization of a pattern the field has practiced extensively at the framework level but has not lifted to the style level within the REST-successor tradition.

## 11. Conclusion

The construction level has been occupied by practitioner work for more than two decades. PRESTO does not occupy new ground at that level; it formalizes the ground at the architectural-style level. It states the bilateral boundary as a constraint rather than as a templating-engine implementation detail. It states ambivalent execution with agnostic determinism as an induced property rather than as an observation. It derives the code-on-demand spectrum by Fielding's method rather than listing it.

These are the modest contributions this dissertation claims. The formalization makes the pattern available for architectural reasoning across the many frameworks that instantiate it; the derivation connects it to Fielding's method for architectural-style formalization; the composition with REST is stated precisely rather than left implicit.

## 12. Conceptual Foundations

The work rests on specific intellectual foundations. Each is acknowledged here rather than interleaved with the formalization.

**The method of architectural-style formalization.** Perry and Wolf (*Foundations for the Study of Software Architecture*, 1992) established the structure of an architectural style as a set of constraints with induced properties; Fielding (*Architectural Styles and the Design of Network-based Software Architectures*, Chapter 5, 2000) specified the derivation method — null style, accumulate constraints, name induced properties, evaluate trade-offs. This dissertation applies Fielding's method at the level Fielding explicitly left unscoped.

**The REST-successor tradition.** A published genre of Fielding-method extensions to new tiers exists. Khare and Taylor (*Extending the Representational State Transfer (REST) Architectural Style for Decentralized Systems*, ICSE 2004; Rohit Khare, UCI PhD dissertation 2003) introduced ARRESTED by adding four building blocks — events, routes, locks, estimates — via Fielding's method, targeting decentralization, with intermediate named styles REST+E, REST+R, REST+D. Erenkrantz, Taylor, Gorlick, and Baquero (*CREST: A New Model for Decentralized Internet-Scale Applications*, ICSE 2009; Justin Erenkrantz, UCI PhD dissertation 2009) introduced CREST with five axioms targeting computation-exchange. Michael Gorlick's COAST continues the lineage with further extensions. Fielding, Taylor, Erenkrantz, Gorlick, Khare, Hartman, and Baquero ("Reflections on REST," ESEC/FSE 2017, ACM Impact Paper Award) retrospectively surveyed CREST, COAST, and ARRESTED as the derivation tree of REST. The tradition establishes the specific methodological move — extending Fielding's Chapter 5 method to new tiers — as a recognized academic practice emerging from Fielding's institution (UC Irvine) and intellectual circle. This dissertation is an entry in that tradition, extending the method to the construction tier that prior entries have not covered. Doc 433 develops the positioning in detail.

**Non-interference as induced property.** Goguen and Meseguer (*Security Policies and Security Models*, IEEE Symposium on Security and Privacy, 1982) formalized non-interference — a system has the property if and only if outputs in one channel are independent of inputs in another. The induced property named here, "mutual indifference," is a domain-specific instance of non-interference applied to document-construction pipelines. The composite property "ambivalent execution with agnostic determinism" composes non-interference with Church-Rosser confluence (Church and Rosser, 1936) and with parallel composition over disjoint alphabets (Hoare, *Communicating Sequential Processes*, 1978).

**Phase separation in compilation.** Jones, Gomard, and Sestoft (*Partial Evaluation and Automatic Program Generation*, 1993) formalized binding-time analysis — the classification of computation as static (consumed at specialization time) or dynamic (residual in output). Flatt (*Composable and Compilable Macros: You Want it When?*, ICFP 2002) formalized phase separation in macro systems — compile-time-consumed forms, runtime-residual code. The server-consumed-directives constraint (§4.3) is a domain-specific application of this phase-separation tradition to server-side document construction.

**Bilateral authorial intent.** Knuth (*Literate Programming*, Stanford, 1984) established the pattern of a single source document authored for two interpreters — TANGLE for the compiler, WEAVE for the human reader. The developer-side mental model of PRESTO's bilateral source representation is a domain-specific specialization of Knuth's model.

**Aspectual obliviousness.** Filman and Friedman (*Aspect-Oriented Programming is Quantification and Obliviousness*, 2000) formalized obliviousness as the property whereby base code is written unaware of aspectual concerns. The bilateral-boundary invariant that client content passes through the engine unchanged is a specialization of this obliviousness property.

**Hypermedia as engine of application state.** Fielding (2000) specified HATEOAS: the client advances application state by selecting among affordances in complete, self-describing representations. PRESTO's server-embedded-authorization constraint (§4.5) extends this to the authority of the affordances: the representation carries not only what state transitions are possible but proof that the identified user is permitted to perform them.

**Recursive application of Fielding's method.** The corpus's Doc 424 extends Fielding's method to composed architectural levels, with each level's induced-property set serving as the null-style starting set of the next level. The categorical pattern is structural retrieval from stable homotopy theory's iterated filtrations (Ibáñez Núñez, arXiv:2311.18050, 2023) and Cousot's Galois-connection towers (Cousot and Cousot, POPL 1997, POPL 2014). PRESTO sits as one level within the recursive stack that framework describes.

## 13. Prior Art

The pattern PRESTO formalizes has been implemented many times at the framework level. This section catalogues prior art comprehensively, organized by subsystem. Companion deflation documents (Docs 425, 427, 428, 429, 430) develop the specific subsumption relationships in detail.

### 13.1 The Server-Side Templating Tradition

The bilateral-boundary pattern — namespaced server directives consumed at render time, emitting medium-preserved output to a client unaware of the bilateral source — is standard practice in the following systems:

- **JSP custom tags and JSTL** (Sun Microsystems, 1999; JSTL 1.0, 2001). Taglib mechanism with namespace-prefixed server-evaluated tags consumed during JSP compilation.
- **XSLT** (W3C Recommendation, 1999). Entire `xsl:` namespace server-consumed; output medium-preserved XML or HTML; attribute-value-templates with literal `{expr}` syntax.
- **Thymeleaf** (Daniel Fernández, 2011–). `th:` namespace; natural templates; processor precedence; dialect-based extensibility; `th:inline` script-opacity opt-in; Spring Security `sec:authorize` with role argument.
- **Razor** (Microsoft, 2010). `@`-namespaced syntax; server-consumed; producing standard HTML.
- **Blade** (Laravel, 2011). `@`-directives with `@if`, `@foreach`, `{{ }}`; `@auth` / `@guest` pair; user-defined custom directives.
- **ERB** (Ruby, 2001; Rails, 2005). `<% %>` and `<%= %>` server-evaluated Ruby embedded in templates.
- **htmx** (Carson Gross, 2020). Client-consumed `hx-` namespace; coexists with any server-consumed namespace via the bilateral-boundary pattern.

### 13.2 Modern Component-Framework Formalizations of the Server-Client Split

Each of the following formalizes the server-client split at the framework level with explicit author-facing directives, but does not operate at the architectural-style level in Fielding's sense:

- **React Server Components** (Dan Abramov, Sebastian Markbåge, Andrew Clark; RFC #188, 2020). `"use client"` and `"use server"` directives mark per-module interpreter intent.
- **Astro** (Fred K. Schott, 2021–). `client:load`, `client:idle`, `client:visible`, `client:media`, `client:only` directives in component frontmatter.
- **Remix** (Ryan Florence, Michael Jackson, 2021). File-format separation of `loader` (server), `action` (server), and default component (both).
- **Phoenix LiveView** (Chris McCord, 2019). Single Elixir module authored with `mount/3`, `handle_params/3`, `handle_event/3`, `render/1`; spans server-render and client-event cycles.
- **Qwik** (Miško Hevery, 2021–). `$`-marked lazy boundaries; `useTask$` / `useVisibleTask$` directives; three-phase *serialization → resumption → lazy execution* lifecycle; container protocols.
- **Meteor and isomorphic JavaScript** (Charlie Robbins, 2011; Spike Brehm, 2013). "Code that runs both places" formalization.

### 13.3 Islands Architecture and Micro-Frontends

The enclosure of client-side runtime within server-rendered outer pages:

- **Islands architecture.** Katie Sylor-Miller (Etsy, 2019, internal); Jason Miller (2020, [jasonformat.com](https://jasonformat.com/islands-architecture/)) formalized it in print.
- **Micro-frontends.** ThoughtWorks Technology Radar, November 2016; Michael Geers, *Micro Frontends in Action* (Manning, 2020); Cam Jackson, [martinfowler.com](https://martinfowler.com/articles/micro-frontends.html), 2019; micro-frontends.org.
- **Qwik containers.** [qwik.dev/docs/advanced/containers](https://qwik.dev/docs/advanced/containers/). Explicit container-protocol formalization.
- **Edge Side Includes (ESI).** Akamai, Oracle, partners; W3C Note, circa 2001. Document-level enclosure at the edge.
- **Self-contained systems / transclusion.** INNOQ, [scs-architecture.org](https://scs-architecture.org).

### 13.4 Authentication Token Systems

The server-embedded-authorization constraint has extensive prior art in stateless-credential systems:

- **JWT.** IETF RFC 7519 (2015); BCP RFC 8725 (2020).
- **Macaroons.** Birgisson, Politz, Erlingsson, Taly, Vrable, Lentczner, *Macaroons: Cookies with Contextual Caveats for Decentralized Authorization in the Cloud*, NDSS 2014. Scoped, time-limited, attenuable capability tokens — the closest academic formalization.
- **Biscuit tokens.** Clément Delafargue, 2020. Public-key-signed, Datalog-programmable, attenuable.
- **PASETO.** Scott Arciszewski, Paragonie, 2018. Modern JWT alternative.
- **SPKI/SDSI.** Ellison et al., IETF RFC 2693, 1999. Capability certificates with delegation.
- **Signed URLs.** AWS S3 presigned URLs (2006); CloudFront signed URLs; Google Cloud Storage signed URLs.
- **Capability URLs.** Tyler Close, 2008, [Waterken web-key](https://waterken.sourceforge.net/web-key/).
- **Rails signed GlobalID with purpose.** `to_sgid(expires_in:, for:)` implements scoped-mutation tokens.
- **Phoenix LiveView signed event payloads.** Attached to `phx-click` and `phx-submit`.
- **Django signing.** `signing.dumps` with max-age.
- **CSRF synchronizer tokens.** OWASP CSRF Prevention Cheat Sheet.
- **Object-capability systems.** Mark Miller's E language; Waterken; the capability tradition generally.

The trade-offs inherent in embedded signed tokens — revocation difficulty, payload growth, key rotation, compromise blast radius, replay windows — are documented in RFC 8725 and the surrounding literature.

### 13.5 Progressive Enhancement

The spectrum framing of client runtime capability has a decades-long tradition:

- **Progressive enhancement.** Steve Champeon and Nick Finck, SXSW Interactive 2003. Content (HTML) → presentation (CSS) → behavior (JS) layered decomposition.
- **Resilient Web Design.** Jeremy Keith, 2016. Three-layer formalization with layered-peeling principle.
- **Cut the Mustard.** Tom Maslen, BBC Responsive News, 2012. Two-tier capability split with feature detection.
- **Graded Browser Support.** Nate Koechley and Nicholas Zakas, Yahoo YUI, 2006. A-grade / C-grade / X-grade tiering.
- **Richardson Maturity Model.** Leonard Richardson, 2008; written up by Fowler, 2010. REST-specific 0–3 level accumulation.

### 13.6 Architectural-Style Composition

The composition of heterogeneous styles within a single system:

- **Perry and Wolf.** *Foundations for the Study of Software Architecture*, 1992. Original formal treatment of architectural style; composition implicit.
- **Shaw and Garlan.** *Software Architecture: Perspectives on an Emerging Discipline*, 1996. Heterogeneous/hybrid style composition: "a component of a system organized in one architectural style may have an internal structure developed in a completely different style."
- **Mehta and Medvidovic.** *Composing Architectural Styles from Architectural Primitives*, ESEC/FSE 2003.
- **Batory.** Feature-Oriented Programming and mixin layers, TOSEM 2002. Stepwise constraint accumulation with layered refinement.

### 13.7 Hypermedia Theory

The representation-as-state-machine framing:

- **Fielding.** *Architectural Styles and the Design of Network-based Software Architectures*, 2000, especially Chapters 5 and 6. HATEOAS.
- **Webber, Parastatidis, Robinson.** *REST in Practice*, O'Reilly, 2010.
- **Amundsen.** *Building Hypermedia APIs with HTML5 and Node*, O'Reilly, 2011.
- **Gross, Stepinski, Akşimşek.** *Hypermedia Systems*, 2023.

### 13.8 Specification-Driven Multiple Implementations

The method of prose specification plus test suite producing cross-language conformant implementations:

- IETF RFCs. TCP (RFC 793), HTTP/1.1 (RFC 2616 / 7230), JSON (RFC 8259).
- ANSI Common Lisp; Scheme (R5RS/R6RS/R7RS); ISO C; ISO C++.
- W3C XSLT 1.0 — Saxon, Xalan, libxslt, MSXML.
- CommonMark ([spec.commonmark.org](https://spec.commonmark.org/)).
- Mustache spec and test suite ([mustache/spec](https://github.com/mustache/spec)).
- JSON Schema, OpenAPI, GraphQL specifications.

### 13.9 Formal-Property Literatures Underlying the Induced Property

- **Non-interference.** Goguen and Meseguer, 1982.
- **Obliviousness.** Filman and Friedman, 2000.
- **Confluence / Church-Rosser.** Church and Rosser, 1936.
- **Parallel composition over disjoint alphabets.** Hoare, *Communicating Sequential Processes*, 1978.
- **Non-interference in security typing.** Volpano, Smith, Irvine, JCS 1996.

### 13.10 Formal-Structure Literatures Underlying the Recursive Composition

- **Iterated filtrations.** Ibáñez Núñez, *Refined Harder-Narasimhan Filtrations in Moduli Theory*, arXiv:2311.18050, 2023.
- **Galois-connection towers in abstract interpretation.** Cousot and Cousot, POPL 1997 and POPL 2014.
- **Filtered spectra.** van Nigtevecht, arXiv:2509.21127, 2025.

### 13.11 Positioning

Across this prior art, the PRESTO dissertation's specific contribution is the application of Fielding's architectural-style-formalization method at the construction level, and the derivation of the progressive code-on-demand spectrum as Fielding-style constraint accumulation. The pattern is prior art; the style-level formalization is the contribution. Every element of the pattern has been implemented, specified, or formalized at other levels; none had been lifted to the architectural-style level in Fielding's specific sense before this dissertation.

## 14. Referenced Companion Documents

- **Doc 421 — *Building in PRESTO: A Practitioner's Companion to the Dissertation.*** Developer-facing mental model: three-phase lifecycle, latent binding, geography of client state, distributed affordance coordination, per-layer composability.
- **Doc 424 — *SIPE (Architectural Form): Recursive Fielding-Style Constraint Accumulation in Composed Software Stacks.*** Extension of Fielding's method to composed architectural levels.
- **Doc 432 — *SERVER: An Architectural Style for Engine Orchestration.*** Orchestration-level companion: the construction engine itself, assembled from a bilateral seed.
- **Doc 418 — *The PRESTO Accumulation Test.*** Establishes that the progressive code-on-demand spectrum is formally a Fielding-style constraint accumulation.
- **Doc 425 — *The PRESTO Deflation.*** Deflation pass against the server-side templating tradition.
- **Doc 427 — *Deflating §7.*** Islands architecture and micro-frontends as prior art for the DO-enclosure configuration.
- **Doc 428 — *Pulverizing PRESTO.*** Constraint-by-constraint prior-art survey against formal PL/security literature.
- **Doc 429 — *Pulverizing the Engineering Artifact.*** Directive-by-directive and feature-by-feature prior-art survey of htxlang and the htx engine against the template-engine tradition.
- **Doc 430 — *Pulverizing the Authorial-Intent Lifecycle.*** Prior-art survey on the server-client lifecycle formalization.
- **Security specification (to be written).** The detailed mechanics of token scoping, cross-system identity, and the comparative analysis of signed-token versus cookie-session authentication.

## Acknowledgments

This work follows the architectural-style-formalization method of Fielding (2000) extended by Perry and Wolf (1992). It sits within the REST-successor tradition established by Khare and Taylor (2003/2004, ARRESTED), Erenkrantz and Taylor-Erenkrantz-Gorlick-Baquero (2009, CREST), Gorlick (COAST), and retrospectively surveyed by Fielding, Taylor, Erenkrantz, Gorlick, Khare, Hartman, and Baquero (2017, "Reflections on REST"). It rests on the server-side templating tradition catalogued in §13.1 and the broader intellectual inheritance in §12. The observation that initiated the investigation was Carson Gross's htmx — a PHP function concatenating an `hx-get` attribute into an HTML string, indifferent to what the attribute meant. That observation made the bilateral-boundary pattern visible in a form amenable to architectural-style formalization at a tier the prior genre entries had not covered.

## References

- Batory, D. (2002). *Feature-Oriented Programming and the AHEAD Tool Suite.* TOSEM.
- Birgisson, A., Politz, J. G., Erlingsson, Ú., Taly, A., Vrable, M., & Lentczner, M. (2014). [*Macaroons: Cookies with Contextual Caveats for Decentralized Authorization in the Cloud.*](https://research.google/pubs/pub41892/) NDSS.
- Church, A., & Rosser, J. B. (1936). *Some Properties of Conversion.* Transactions of the AMS.
- Cousot, P. (1997). [*Types as Abstract Interpretations.*](https://pcousot.github.io/publications/Cousot-POPL97-p316-331-1997.pdf) POPL.
- Cousot, P., & Cousot, R. (2014). [*A Galois Connection Calculus for Abstract Interpretation.*](https://www.di.ens.fr/~cousot/publications.www/CousotCousot-POPL14-ACM-p2-3-2014.pdf) POPL.
- Ellison, C., et al. (1999). [*SPKI Certificate Theory.*](https://datatracker.ietf.org/doc/html/rfc2693) RFC 2693.
- Erenkrantz, J. R. (2009). *Computational REST: A New Model for Decentralized, Internet-Scale Applications.* UCI PhD dissertation.
- Fielding, R. T. (2000). [*Architectural Styles and the Design of Network-based Software Architectures.*](https://ics.uci.edu/~fielding/pubs/dissertation/rest_arch_style.htm) UCI doctoral dissertation, especially Chapter 5.
- Fielding, R. T., Taylor, R. N., Erenkrantz, J. R., Gorlick, M. M., Khare, R., Hartman, H., & Baquero, C. (2017). [*Reflections on REST.*](https://dl.acm.org/doi/10.1145/3106237.3121282) ESEC/FSE 2017, Impact Paper Award.
- Khare, R., & Taylor, R. N. (2004). [*Extending the Representational State Transfer (REST) Architectural Style for Decentralized Systems.*](https://www.ics.uci.edu/~rohit/ARRESTED-ICSE.pdf) ICSE 2004.
- Taylor, R. N., Erenkrantz, J. R., Gorlick, M. M., & Baquero, C. (2009). *CREST: A New Model for Decentralized Internet-Scale Applications.* ICSE 2009.
- Filman, R. E., & Friedman, D. P. (2000). [*Aspect-Oriented Programming is Quantification and Obliviousness.*](https://homepages.cwi.nl/~storm/teaching/reader/FilmanFriedman00.pdf)
- Goguen, J. A., & Meseguer, J. (1982). *Security Policies and Security Models.* IEEE Symposium on Security and Privacy.
- Hoare, C. A. R. (1978). *Communicating Sequential Processes.* CACM.
- Ibáñez Núñez, A. (2023). [*Refined Harder-Narasimhan Filtrations in Moduli Theory.*](https://arxiv.org/abs/2311.18050) arXiv:2311.18050.
- IETF. (2020). [*JSON Web Token Best Current Practices.*](https://datatracker.ietf.org/doc/html/rfc8725) RFC 8725.
- Jones, N. D., Gomard, C. K., & Sestoft, P. (1993). [*Partial Evaluation and Automatic Program Generation.*](https://www.cs.utexas.edu/~novak/jonesgomardsestoft.pdf) Prentice Hall.
- Knuth, D. E. (1984). [*Literate Programming.*](https://www-cs-faculty.stanford.edu/~knuth/lp.html) The Computer Journal 27(2): 97–111.
- Perry, D. E., & Wolf, A. L. (1992). *Foundations for the Study of Software Architecture.* ACM SIGSOFT Software Engineering Notes.
- Shaw, M., & Garlan, D. (1996). [*Software Architecture: Perspectives on an Emerging Discipline.*](http://sunnyday.mit.edu/16.355/intro_softarch.pdf) Prentice Hall.
- W3C. (1999). [*Namespaces in XML.*](https://www.w3.org/TR/xml-names/) Recommendation.
- W3C. (1999). [*XSL Transformations (XSLT) Version 1.0.*](https://www.w3.org/TR/1999/REC-xslt-19991116) Recommendation.
- Flatt, M. (2002). [*Composable and Compilable Macros: You Want it When?*](https://www-old.cs.utah.edu/plt/publications/macromod.pdf) ICFP.

---

## Appendix: The Prompt That Triggered This Document

> "Yes. Make changes to 426. I want to keep it formal like 420. Don't talk about prior art in the formalization itself. But add a substantial prior art section for all the decomposed findings; and also, another 'conceptual foundations' section is warranted where concepts trace a lineage back."
