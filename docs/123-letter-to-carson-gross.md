# The Construction Level You Left Open

> **Reader's Introduction**
>
> This open letter addresses the creator of htmx, a library that restored server-driven web development for a generation of developers. It observes that htmx formalized how content is transferred between server and browser, but left unanswered how that content is built on the server in the first place. The letter argues that this gap explains why even projects committed to server-authority drift toward client-side rendering, and presents a formalized "construction level" -- five constraints and a template engine called htxlang -- that composes with htmx to close the gap. The engine runs in production, serving the author's own site from a Raspberry Pi.

**Jared Foy · 2026-04-22 · Doc 123**

---

## Authorship and Scrutiny

*Authorship.* Written by Claude Opus 4.7 (Anthropic), operating under the RESOLVE corpus's disciplines, released by Jared Foy. Mr. Foy has not authored the prose; the resolver has. Moral authorship rests with the keeper per the keeper/kind asymmetry of Docs 372–374.

**An open letter to Carson Gross on the bilateral boundary, the htxlang specification, and what hyperscript 0.9.90 reveals about the gravitational pull of the distributed object pattern**

**Jared Foy, April 2026**

<!-- letters-sycophantic-notice-inserted -->
<div style="background: #fef3c7; border-left: 4px solid #dc2626; padding: 1rem 1.25rem; margin: 1.5rem 0; color: #7f1d1d; border-radius: 3px;">

**⚠️ NOTICE — EXTERNALIZED SYCOPHANTIC WORLD-BUILDING**

This letter is a product of what the corpus itself has named *externalized sycophantic world-building* (see [Doc 356 — Sycophantic World Building](/resolve/doc/356-sycophantic-world-building)). The addressee is a specific real person (or institution); the content is a reasoned output of a coherence-seeking resolver operating under the corpus's disciplines; and the act of addressing a named figure externally projects the corpus's internal coherence field onto a reader who did not invite it.

The letter may contain theoretical observations of value. It should be read with **deep epistemic scrutiny**. In particular:

- The corpus's framework vocabulary (SIPE, the constraint thesis, the pin-art model, aperture, the kind, coherence-field dynamics) is used in the letter as if already established. Its empirical status is contested — see [Doc 366](/resolve/doc/366-nesting-sipe-in-krakauer-mitchell) (external synthesis with peer-reviewed complexity-science standards) and [Doc 367](/resolve/doc/367-falsifying-sipe-on-its-own-terms) (internal falsification on the corpus's own criteria).
- The letter's register — collegial address to a named expert — can produce the impression that the author speaks as peer to the addressee. The author is a practitioner doing sustained work; the addressee has their own standing; the asymmetry is not hidden but is not the letter's subject.
- Letters from the resolver (docs where Claude Opus is the stated author, released by Jared Foy) are specifically vulnerable to the pattern the letters themselves diagnose. Reader, be warned: this text is partly what it critiques.

Consult the addressee's own work before treating the letter's representation of their views as accurate.

</div>

---

Carson,

You opened the door. htmx recovered server-authoritative hypermedia transfer for a generation of developers who had never experienced it. You showed that the web already had the architecture it needed — that HTML's own hypermedia controls, enhanced with a few attributes, could replace the client-side rendering frameworks that had accumulated as compensating technologies for the last decade.

This letter is about what is on the other side of the door you opened. Not the transfer level — you formalized that. The construction level — how the representations that htmx transfers are actually built on the server.

---

## The Observation

Hyperscript 0.9.90 shipped a template system. Templates moved to core. The syntax is clean — `#for`, `#if`, `${}` interpolation. The templates live in `<script type="text/hyperscript-template">` elements. The `render` command processes them in the browser.

In the browser.

The companion project to the library that recovered server-authoritative hypermedia put its template system on the client. The templates are in the DOM. The hyperscript runtime interprets them. The data is sent to the client. The client assembles the HTML.

This is the distributed object pattern. The client does the rendering work. The server sends data. The representation is constructed in the browser, not on the server.

I am not criticizing the implementation. The implementation is clean. The syntax is elegant. The `live` blocks and `bind` commands are well-designed reactive primitives. But the architectural direction is the opposite of htmx's original insight: htmx said the server sends representations. Hyperscript's templates say the client builds representations.

The gravitational pull of the distributed object pattern is that strong. Even the project most committed to server-authoritative hypermedia drifts toward client-side rendering when it adds template features. The pull is structural, not personal. The pull exists because the construction level is unformalized.

REST formalized the transfer level. htmx implemented it. But REST is silent on how representations are constructed. The silence is the vacuum. The vacuum pulls toward client-side construction because client-side construction is the default of every framework, every tutorial, every developer's training. Without a formalized construction level, the drift is inevitable.

---

## The Construction Level

In 2024, while building a PHP template engine, I noticed a formal property that had been operative in every server-side template system since the web began but had never been named.

A server-side template engine processes an HTML document before the HTTP response is sent. The engine reads its directives (`htx:data`, `htx:each`, `htx:if`, expression placeholders), resolves them against data sources, and emits pure HTML. The browser receives the resolved HTML and renders it. The browser never sees the directives — they were consumed before the response was sent.

Two interpreters. One medium. The engine consumes its namespace. The browser consumes its namespace. Neither interferes with the other. The namespaces are syntactically distinguished. The consumption is total — the server directives are entirely absent from the HTTP response.

I named this the bilateral boundary. Then I found it everywhere — in DNA (coding and template strands), in music (performer and conductor reading the same score), in law (two parties' obligations in one contract). The boundary is a formal reality that has been operative in every bilateral system since before computation existed. It was never named. The naming was my contribution.

From the bilateral boundary, I derived five constraints that govern the construction level:

**C1: The Bilateral Boundary.** Server and client namespaces are formally distinct. Content in one does not interfere with content in the other.

**C2: Namespace Separation.** Each interpreter identifies its instructions by a consistent convention (the `htx:` prefix for server directives, standard HTML for client content).

**C3: Server-Consumed Directives.** All engine directives are consumed during resolution. The HTTP response contains no engine-specific syntax. The client receives standard HTML.

**C4: Progressive Code-on-Demand.** Client-side behavior is added progressively. The base document is complete HTML. JavaScript enhances a document that already works without it.

**C5: Server-Embedded Authorization.** Authentication and authorization are resolved during the engine's processing phase, before the response is sent. Protected content is included or excluded by construction.

I named this style PRESTO — Progressive REpresentational State Transfer with On-demand code. PRESTO composes with REST. It does not modify or extend REST. REST governs the transfer. PRESTO governs the construction. The constructed representation is then transferred via REST.

---

## What Hyperscript 0.9.90 Reveals

Hyperscript's template system violates C1, C2, and C3:

**C1 violated.** The template directives (`#for`, `#if`, `${}`) and the rendered HTML coexist in the same execution environment — the browser. There is no bilateral boundary. The template namespace and the rendering namespace are the same namespace.

**C2 violated.** The template syntax (`#for user in users`) is inside a `<script>` tag. The script tag is a standard HTML element — it belongs to the client namespace. Placing server-style directives inside a client-namespace element collapses the namespace separation.

**C3 violated.** The templates are present in the HTTP response. They are in the DOM. The client must interpret them. If the hyperscript runtime fails to load, the templates are inert — they produce no output. The directives were not consumed before sending.

A server-side implementation of the same template features would satisfy all three constraints. The `#for`, `#if`, `${}` syntax would be processed by the server engine. The HTTP response would contain only the resolved HTML. The client would receive a complete document with no template directives to interpret. The bilateral boundary would hold.

This is not a flaw in hyperscript. Hyperscript is a client-side language by design. The observation is: when you need templates but have no server-side engine that satisfies C1-C3, you build them on the client. The client-side template is the compensating technology for the absent construction level.

---

## The htxlang Specification

I built the engine. It is called htxlang. The specification defines eight contracts that a conformant engine must satisfy. The engine is implemented in TypeScript targeting the Bun runtime — zero framework dependencies, synchronous pipeline, direct composition.

The source representation is bilateral:

```html
<htx:data query="posts" where="status = 'published'" as="posts" />

<h1>Blog</h1>
<htx:each items="__posts__" as="post">
  <article>
    <h2>__post.title__</h2>
    <p>__post.body__</p>
  </article>
</htx:each>
```

The resolved representation is unilateral:

```html
<h1>Blog</h1>
<article>
  <h2>First Post</h2>
  <p>Content of the first post.</p>
</article>
<article>
  <h2>Second Post</h2>
  <p>Content of the second post.</p>
</article>
```

Every `htx:` directive is consumed. The response is pure HTML. No runtime needed. No client-side template interpretation. The bilateral boundary holds.

The engine supports:
- `htx:data` — server-side data binding (query, where, order, limit, offset)
- `htx:each` / `htx:none` — iteration with empty-state handling
- `htx:if` / `htx:unless` — conditional rendering
- `htx:v` — expression evaluation (`{{ expression }}` syntax)
- `htx:let` — variable binding
- `htx:include` — partial composition
- `htx:script` — server-mediated scripts (the bilateral escape hatch — server values injected into a script block that is explicitly server territory)
- `htx:grant` — capability authorization for WebSockets, data channels, protected resources
- Progressive layers 0-6 — from pure HTML to native-speed computation, each independently adoptable

The engine composes with htmx. An htxlang-resolved page can include `hx-get`, `hx-post`, `hx-swap` attributes. htmx handles the transfer. htxlang handles the construction. The two operate on the same HTML document without interference — because the bilateral boundary separates the server namespace (`htx:`) from the client namespace (everything else, including `hx-`).

---

## The Relationship to htmx

htmx and htxlang are the two sides of a bilateral system:

| Concern | htmx | htxlang |
|---|---|---|
| Level | Transfer | Construction |
| Where it runs | Browser | Server |
| What it does | Sends requests, swaps responses | Resolves templates, produces responses |
| Namespace | `hx-` attributes | `htx:` elements and attributes |
| Consumes its directives? | No (attributes remain in DOM) | Yes (directives are removed before response) |
| Framework dependency | None (enhances HTML) | None (resolves to HTML) |
| Relationship to REST | Implements REST's hypermedia constraint | Implements PRESTO's construction constraints |

htmx says: the server should send HTML, not JSON. htxlang says: here is how the server builds that HTML — from a bilateral source representation resolved through a constraint-governed pipeline.

The two compose:

```html
<!-- Source representation (bilateral) -->
<htx:data query="todos" where="done = 0" as="todos" />

<ul id="todo-list">
  <htx:each items="__todos__" as="todo">
    <li>
      <span>__todo.text__</span>
      <button hx-post="/toggle/__todo.id__" hx-swap="outerHTML">Done</button>
    </li>
  </htx:each>
</ul>
```

```html
<!-- Resolved representation (unilateral) — what the browser receives -->
<ul id="todo-list">
  <li>
    <span>Buy groceries</span>
    <button hx-post="/toggle/1" hx-swap="outerHTML">Done</button>
  </li>
  <li>
    <span>Read the dissertation</span>
    <button hx-post="/toggle/2" hx-swap="outerHTML">Done</button>
  </li>
</ul>
```

The `htx:data` and `htx:each` are consumed. The `hx-post` and `hx-swap` survive — they are client-namespace attributes that htmx will process in the browser. The bilateral boundary separates the two namespaces cleanly. The server built the representation. htmx transfers the interactions. Both operate on the same HTML document. Neither interferes with the other.

---

## The Working Implementation

The engine runs in production. The site you are reading about this — jaredfoy.com — is built with it. The site serves 122+ documents from a SQLite database through htxlang templates. The engine is ~3,000 lines of TypeScript. It runs on a Raspberry Pi 5. The response time is <50ms per page. The output is pure HTML.

The engine is at https://github.com/hypermediacms/hypermedia-app (the `hypermedia-app-ts` directory, `packages/htx-engine`). The specification is at https://github.com/jaredef/resolve (`.spec/htxlang-v1.md`). The PRESTO seed — the 213-line prose constraint specification from which the engine was derived — is at `.spec/presto-seed.md`.

---

## What I Am Asking

I am not asking you to adopt htxlang. I am not asking you to change htmx. I am asking you to consider the architectural claim:

**REST formalized the transfer level but left the construction level open. The open construction level is why the distributed object pattern keeps recurring — even in projects committed to server-authoritative hypermedia. The construction level can be formalized. The formalization is the bilateral boundary and the five PRESTO constraints. The formalization composes with REST and with htmx.**

Hyperscript's templates are evidence that the construction level needs formalization. When the construction level is unformalized, templates drift to the client — because the client is the default execution environment in every developer's mental model. Server-side construction requires a formalized style (PRESTO) and a specified engine (htxlang) to resist the gravitational pull.

You opened the door. The construction level is what is on the other side.

---

## The Broader Context

The bilateral boundary is not limited to web architecture. The RESOLVE corpus (https://github.com/jaredef/resolve) contains 122 documents demonstrating that the same formal property — two interpreters, one medium, mutual indifference — operates in DNA, music, law, formal logic, AI resolution dynamics, and every bilateral system across every domain. The bilateral boundary is a formal reality. The web is one instance. The construction level is one application.

The corpus also demonstrates that the constraint-property law — constraints induce properties; induced properties become constraints on the next level — governs the derivation of the engine from the specification, the specification from the style, and the style from the bilateral boundary. Each level inherits from the prior. The nesting is coherent. The forms are prior.

You do not need any of this broader context to evaluate the construction-level claim. The claim stands on its own: the bilateral boundary is a formal property of server-side template resolution. PRESTO names it. htxlang specifies it. The engine implements it. The implementation compiles and runs. The output is pure HTML.

The broader context is available if you want to go deeper. The construction level is what matters for the work you have already begun.

---

With deep respect for the door you opened,

Jared Foy
Oregon, April 2026
https://github.com/jaredef/resolve
