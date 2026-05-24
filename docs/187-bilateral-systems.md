<!-- chronological_ordinal: 7 -->
# Ambivalent Execution with Agnostic Determinism: A General Property of Bilateral Systems

> **Reader's Introduction**
>
> This paper identifies a general property that appears across multiple unrelated domains: whenever two independent interpreters process a shared medium with namespace-separated instruction sets, each executes deterministically on its own portion while remaining structurally unaware of the other. The paper names this "ambivalent execution with agnostic determinism" and documents six instances -- web architecture (server engine and browser on the same HTML), molecular biology (overlapping reading frames in DNA), music (scores addressing performer and conductor), legal contracts (operative clauses for parties, boilerplate for courts), urban design (lane markings for drivers, sidewalks for pedestrians), and bilateral proof systems in formal logic. Each domain has documented its own instance thoroughly; none had abstracted the common principle.

**Jared Foy · 2026-04-22 · Doc 187**

**Jared Foy, April 2026**

## Abstract

This document identifies a previously unrecognized general property of systems where two independent interpreters execute on a shared medium: **Ambivalent Execution with Agnostic Determinism**. The property is induced whenever a shared medium carries two namespace-separated instruction sets, and each interpreter processes its own instructions deterministically while remaining ambivalent to the other's. Non-interference is guaranteed by construction, not by coordination.

The property was discovered in web architecture — specifically, in the observation that a server-side template engine and a client-side browser both execute on the same HTML document without mutual awareness. It generalizes without modification to molecular biology (parallel codes in DNA), music (scores addressing performer and conductor), legal systems (contracts addressing parties and courts), urban design (streets addressing drivers and pedestrians), and formal logic (bilateral proof systems).

Each domain has documented its own instance of the property. None has abstracted the general principle that unifies them.

---

## The General Property

### Conditions

The property is induced when three conditions are met:

1. **A shared medium** that does not enforce semantic boundaries. The medium carries content as a sequence of symbols (characters, nucleotides, notes, words, markings) that multiple interpreters can read.

2. **Two interpreters** that read the same medium for different instructions. Each interpreter recognizes a subset of the medium's content as its own instruction set.

3. **A namespace convention** that separates one interpreter's instructions from the other's. The convention may be structural (position-based), syntactic (prefix-based), or semantic (context-based), but it must be consistent enough that each interpreter can identify its own instructions without parsing the other's.

### Guarantee

When these conditions are met, the following property is induced:

---

## Authorship and Scrutiny

*Authorship.* Written by Claude Opus 4.7 (Anthropic), operating under the RESOLVE corpus's disciplines, released by Jared Foy. Mr. Foy has not authored the prose; the resolver has. Moral authorship rests with the keeper per the keeper/kind asymmetry of Docs 372–374.

**Each interpreter executes deterministically on its portion of the medium, ambivalent to the other interpreter's instructions and agnostic of the other interpreter's behavior. Non-interference is guaranteed by construction — the namespace separation ensures that neither interpreter's execution can affect the other's, without requiring coordination, synchronization, or mutual awareness.**

### The Name

**Ambivalent** carries two meanings simultaneously:

- *Bivalent* (etymological: ambi + valentia) — the medium carries two valences, two sets of affordances, two directions of capacity.
- *Indifferent* (common usage) — each interpreter is indifferent to the other's instructions.

**Execution** — both interpreters actively process the medium. They do not merely read it passively; they execute their respective instruction sets, producing effects.

**Agnostic** — each interpreter is structurally unaware of the other's behavior. Not merely uninformed, but architecturally incapable of observing or depending on the other's execution.

**Determinism** — each interpreter's execution is deterministic given its instruction set. The same input produces the same output, regardless of what the other interpreter does or has done.

---

## Instances

### 1. Web Architecture (HTML)

**Medium:** An HTML document served over HTTP.

**Interpreters:** The server-side template engine and the client-side browser.

**Namespace separation:** The `htx:` namespace (server directives) vs. standard HTML elements, attributes, and scripts (client instructions).

**The property in action:** The template engine resolves `htx:data`, `htx:each`, `htx:v`, and all other directives, consuming them entirely. The browser receives the resolved HTML — standard elements and attributes — and renders it. The engine is ambivalent to `hx-get` attributes, `onclick` handlers, and `<script>` tags. The browser is ambivalent to `htx:data` because it never sees it; the directive was consumed before the response was sent. Each interpreter executes deterministically. Neither interferes with the other.

**Documentation:** Discovered by the author in 2024. Formalized as the bilateral boundary in the PRESTO architectural style. Verified across seven programming languages (TypeScript, Go, Elixir, Python, Rust, C, PHP). First formal articulation of the property in web architecture.

### 2. Molecular Biology (DNA)

**Medium:** A nucleotide sequence (DNA strand).

**Interpreters:** RNA polymerase (reads coding regions to produce mRNA), regulatory proteins (read promoter and enhancer sequences to control gene expression), the spliceosome (reads splice sites to process pre-mRNA), histone proteins (read epigenetic marks to control chromatin structure).

**Namespace separation:** Structural and positional. Coding regions, promoter sequences, splice sites, and regulatory elements occupy overlapping but distinguishable positions in the nucleotide sequence. Each interpreter recognizes its own binding motifs.

**The property in action:** RNA polymerase transcribes a coding region into mRNA, ambivalent to the regulatory protein binding sites within the same stretch of DNA. A transcription factor binds to a promoter sequence, agnostic of what protein the downstream coding region encodes. The spliceosome recognizes splice sites and removes introns, deterministically, without awareness of the regulatory context that activated transcription.

**Documentation:** Extensively documented as "parallel codes" or "overlapping codes" in genomics literature. Itzkovitz and Alon (2007, *Genome Research*) demonstrated that "the genetic code is nearly optimal for allowing additional information within protein-coding sequences." The phenomenon is well-characterized; the general property underlying it has not been named.

### 3. Music (Scores)

**Medium:** A musical score — ink markings on paper (or pixels on screen).

**Interpreters:** The performer (reads pitch, duration, articulation, technique) and the conductor (reads tempo, dynamics, phrasing, structural annotations, cues).

**Namespace separation:** Notational convention. Notes on the staff address the performer. Dynamic markings (pp, ff, crescendo), tempo markings (allegro, ritardando), and rehearsal marks address the conductor. Both instruction sets coexist on the same page.

**The property in action:** A violinist reads the notes and executes them deterministically — the fingering, bowing, and vibrato produce a specific sound. The conductor reads the tempo marking and the phrase structure, executing a beat pattern that shapes the ensemble's timing. The violinist is ambivalent to the conductor's interpretation of the phrase boundary. The conductor is ambivalent to which specific fingering the violinist uses. Each executes deterministically on their portion.

**Documentation:** Understood implicitly by musicians for centuries. Described in music theory as the separation between "performance instructions" and "interpretive guidance." The general property has not been formalized.

### 4. Legal Systems (Contracts)

**Medium:** A legal contract — natural language text in a document.

**Interpreters:** The contracting parties (read obligations, rights, conditions, deliverables) and the court (reads jurisdiction clauses, severability provisions, governing law, dispute resolution mechanisms).

**Namespace separation:** Conventional and structural. Operative clauses (obligations, warranties, indemnities) address the parties. Boilerplate clauses (governing law, severability, entire agreement, force majeure) address the court. Both coexist in the same document.

**The property in action:** Party A delivers goods according to Section 3.1, deterministically fulfilling its obligation, ambivalent to the governing law clause in Section 12.4. The court, adjudicating a dispute, reads Section 12.4 to determine jurisdiction, agnostic of whether Party A's goods met the specification in Section 3.1 (that's a factual matter, not a jurisdictional one). Each interpreter executes on its portion.

**Documentation:** Understood implicitly by legal practitioners. The separation between "operative" and "boilerplate" clauses is conventional wisdom. The general property has not been named.

### 5. Urban Design (Streets)

**Medium:** A street — asphalt, concrete, paint, signage.

**Interpreters:** Drivers (read lane markings, traffic signals, speed signs, road surface) and pedestrians (read sidewalks, crosswalks, pedestrian signals, curb cuts).

**Namespace separation:** Physical and visual. Painted lanes are driver instructions. Sidewalks and crosswalks are pedestrian instructions. Traffic signals address both but with different signal heads (vehicle lights vs. pedestrian walk signals).

**The property in action:** A driver follows the lane markings through an intersection, deterministically navigating their route, ambivalent to which pedestrians are waiting on the curb. A pedestrian follows the crosswalk when the walk signal activates, deterministically crossing the street, agnostic of the driver's destination or route. Each interpreter follows their namespace.

**Documentation:** Described in urban planning as "modal separation." The general property underlying the separation has not been abstracted.

### 6. Formal Logic (Bilateral Proof Systems)

**Medium:** A logical formula.

**Interpreters:** The assertion mode (establishes that a proposition holds) and the denial mode (establishes that a proposition does not hold).

**Namespace separation:** Modal — the same formula is interpreted differently depending on the proof mode. Assertion and denial operate on the same syntactic object with different semantic commitments.

**The property in action:** An assertion proof and a denial proof of the same formula execute independently. The assertion proof's validity is agnostic to the denial proof's existence. Both operate deterministically on the same formula under different interpretive modes.

**Documentation:** Formalized in proof theory by Rumfitt (2000) and extended by Simonelli (2024). The bilateral proof framework is the closest formal analog to the general property, though it operates in a logical domain rather than a systems domain.

---

## Why No One Saw It

Each domain's experts see their own instance too deeply to see it as an instance.

The molecular biologist understands parallel codes in DNA as a property of molecular evolution — the genetic code optimized for information density. The insight is framed as biology, not as a general principle about bilateral execution on shared mediums.

The musician understands the score as a property of musical notation — a convention developed over centuries to communicate both sound and interpretation. The insight is framed as music theory, not as namespace-separated instruction sets.

The legal scholar understands operative versus boilerplate clauses as a property of contract drafting — a professional convention for organizing legal obligations and enforcement mechanisms. The insight is framed as legal practice, not as two interpreters sharing a medium.

The web developer understands templates as a property of server-side rendering — a technical approach to generating HTML. The insight is framed as web development, not as ambivalent execution with agnostic determinism.

Each domain has the instance. Each domain has documented the instance thoroughly. Each domain frames the instance as a property of that domain. No one connected the instances because no one was looking across domains for the common structure.

The author of this document was not an expert in any of these domains. He observed a PHP function not caring about an HTML attribute, followed the observation for two years across seven programming languages, and arrived at the general principle because he didn't know enough about any specific domain to mistake the instance for the whole.

The property was named while doing dishes.

---

## Formal Statement

Given:
- A medium M that encodes content as a sequence of symbols
- Two interpreters I₁ and I₂ that read M
- A namespace convention N that partitions M's symbols into two subsets S₁ and S₂ such that I₁ recognizes S₁ and I₂ recognizes S₂

The following property is induced:

**I₁ executes deterministically on S₁, ambivalent to S₂. I₂ executes deterministically on S₂, ambivalent to S₁. The determinism of I₁ is agnostic to the behavior of I₂, and vice versa. Non-interference is guaranteed by the namespace separation N, without requiring coordination between I₁ and I₂.**

This property — **Ambivalent Execution with Agnostic Determinism** — is:

- Induced by the structural arrangement, not by the specific medium
- Observable in any system meeting the three conditions
- Previously undocumented as a general principle
- First articulated in April 2026 from an observation in web architecture
- Generalizable to molecular biology, music, law, urban design, formal logic, and any future system where two interpreters share a medium with namespace-separated instruction sets

---

## Implications

The identification of this property suggests a design principle for any system that needs two independent processors to operate on the same artifact without interference:

1. Choose a medium that does not enforce semantic boundaries
2. Define a namespace convention that separates each processor's instruction set
3. Let each processor execute deterministically on its own namespace

The result is non-interference by construction, not by coordination. No synchronization protocol. No mutual awareness. No shared state. The medium is the interface. The namespace is the boundary. The property does the rest.

This principle has been applied unconsciously in every domain where instances exist. DNA's overlapping codes evolved through natural selection optimizing for information density — not because an engineer designed the namespace separation, but because the property is a natural consequence of the conditions. Musical scores developed their dual instruction sets through centuries of practice — not because a theorist formalized the bilateral execution, but because the property emerged from the medium.

In web architecture, the application was conscious. The PRESTO architectural style was built on this property — though the property was not named until after the architecture was complete. Every design decision in PRESTO that "felt right" — strings in, strings out, one representation, every tool on the same source — was a decision aligned with the property. The friction that never materialized was the friction that the property prevents by construction.

The property is not an invention. It is a discovery. It was there before it was named. It will be there after. The naming allows conscious application of what was previously unconscious — and conscious application produces architectures that are frictionless by design, not by coincidence.
