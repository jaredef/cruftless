# Rederive: The Constraint Authoring Grammar

## A Comprehensive Entracement to the `.constraints.md` File Format — the Manifest Header for Composition and Pinning, the Constraint Block with Heading and Metadata and Body and Fenced Evidence, the Seven Constraint Types and Their Authorial Intents, the Six Fenced Evidence Kinds and the Backends They Reach, the Authoring Disciplines that Make Derivations Converge, the Common Pitfalls and How to Avoid Them, and Three Sample Files Read End-to-End — the First Branch off [Doc 659](/resolve/doc/659-rederive-for-the-working-engineer-hub) for the Working Engineer

<div style="background: #ecfeff; border-left: 4px solid #0e7490; padding: 1rem 1.25rem; margin: 1.5rem 0; color: #164e63; border-radius: 3px;">

**EXPLORATORY — practitioner-facing entracement, branch §A of [Doc 659](/resolve/doc/659-rederive-for-the-working-engineer-hub).**

*Taxonomy per [Doc 633](/resolve/doc/633-corpus-taxonomy-and-manifest-design):* ENGAGEMENT | ACTIVE | W-PI | THREAD-REDERIVE, THREAD-PRACTITIONER-FACING | PHASE-CROSS-PRACTITIONER

</div>

> **Reader's Introduction.** This is the first branch off the [rederive hub](/resolve/doc/659-rederive-for-the-working-engineer-hub). The hub introduced the platform's working surface; this document answers the engineer's first concrete question in full: *what does a constraint file actually look like, and what do I have to write?* The grammar is small, but small grammars rarely carry their depth on the surface, and the difference between a constraint file that derives cleanly on first attempt and one that takes five rounds to converge is in the disciplines this document catalogues. By the end of the read, the engineer should be able to sit down, write a constraint file for a small library, run the platform against it, and ship a signed materialization. The longer-term skill of authoring engine-scoped constraint sets that compose with each other has its own discipline catalogue (the *Pin-Art* practice, [Doc 659](/resolve/doc/659-rederive-for-the-working-engineer-hub) §F); this document is its precondition.

**Jared Foy · 2026-05-05 · Doc 660**

---

## Authorship and Scrutiny

*Authorship.* Written by Claude Opus 4.7 (Anthropic), operating under the RESOLVE corpus's disciplines, released by Jared Foy. The keeper has not authored the prose; the resolver has. The rhetoric is calibrated to a working-engineer audience under the keeper's directive that the corpus's philosophical vocabulary be set aside where the engineer's own vocabulary suffices, and properly entraced where it does not.

---

## 1. The File at a Glance

A constraint file is named `<thing>.constraints.md`. The Markdown convention is deliberate. A reviewer can render the file in any tool that handles Markdown and read it as a requirements document. The platform parses the same source as a structured constraint set. There is no parallel format to maintain, no IDE plugin to install, no schema to migrate. The text is the source of truth for both the human reader and the machine parser.

The file has two regions:

- *(optional)* a **manifest header** at the top, declaring what this file provides to other files, what it imports from other files, and what implementation phrases must be preserved across regenerations.
- An ordered sequence of **constraint blocks**, each one introduced by an H2 heading whose text is the constraint's identifier.

That is the whole structure. Everything else is the discipline of what goes inside each region.

## 2. The Manifest Header

If your file imports from other files or provides a property to other files, the file begins with a manifest header *before* the first H2. Three directives:

### 2.1 `@provides`

```
@provides: <property-name>
  threshold: <constraint-id>
  interface: [<exported-symbol>, <exported-symbol>, ...]
```

`@provides` declares that this file *induces* a named property when its threshold-constraint passes. The property name is a freely-chosen identifier (kebab-case is conventional). The threshold is one of the constraints in this file; the property is considered to emerge only if that constraint's verification passes. The interface enumerates the exported symbols (function names, type names, constant names) a consumer can rely on.

A consumer file declares `@imports` against this property name, references this file by path, and (optionally) pins the consumer to a specific content hash. The platform resolves the import to this file, verifies the threshold-constraint passes in the consumer's view of this file's most recent materialization, and threads the interface into the consumer's derive prompt.

The corpus's broader vocabulary calls this *induced-property emergence* (the SIPE-T discipline, [Doc 541](/resolve/doc/541-systems-induced-property-emergence)); in the engineer's vocabulary it is closer to *exporting a contract under a precondition*. The contract is the interface; the precondition is the threshold; the platform machinery enforces both.

### 2.2 `@imports`

```
@imports:
  - property: <property-name>
    from: path
    path: ./<other-file>.constraints.md
    as: <local-alias>
```

`@imports` lists the properties this file depends on. Resolution by `from: path` is the common case in the MVE; resolution by `from: tag` and `from: hash` are roadmap. The `path` is relative to this file's location. The `as` is a local alias the prose body can reference when it talks about the imported interface.

A consumer that imports `canonicalize-property` from `./canonicalize-module.constraints.md` is making three claims at once: *I depend on the canonicalize-module's threshold passing*; *I expect its interface to be the one declared at @provides*; *I will use it in my prose under the alias `canonicalize-module`*. The platform checks all three at the resolve stage of the build pipeline ([Doc 661](/resolve/doc/661-rederive-stub-b-the-build-pipeline) §5).

### 2.3 `@pins`

```
@pins:
  - id: <pin-id>
    mustContain: "<exact-phrase>"
    why: "<one-line-reason>"
```

`@pins` declares phrases that must appear verbatim in the derived code. Use this when intent depends on a specific implementation detail being preserved across regenerations: an error message a downstream consumer parses, a function name a third-party tool depends on, a comment that flags a regulatory requirement, a numeric default that has been tuned in production. The `id` is for traceability in the verification report; the `mustContain` is the literal phrase; the `why` is the engineer's documented reason.

The discipline of pinning is covered in detail in [Doc 663 §5](/resolve/doc/663-rederive-stub-d-content-addressed-identity-and-pin-manifests). The short version: *pin only what intent depends on; document the why; prefer constraints over pins; retract stale pins*.

## 3. The Constraint Block

Each constraint block has four parts in order: heading, metadata, body, fenced evidence.

### 3.1 Heading

Exactly one H2 heading with a stable identifier. The convention is short uppercase tokens with a numeric suffix, scoped to the file: `## SLUG1`, `## CAN3`, `## A11Y2`. The identifier is what other constraints reference in `depends-on` and what the verification verdict prints alongside the per-constraint result. Stability matters because the identifier outlives any single derivation; if you renumber, the materialization history loses its anchors.

You may use any token shape the parser accepts (the parser matches `^##\s+(\S.*?)\s*$`), but the convention is uppercase + numeric for two reasons: it sorts predictably in canonical form, and it visually separates constraint identifiers from English prose in the body where they may be referenced inline.

### 3.2 Metadata

A small block of `key: value` lines immediately under the heading, ending at the first blank line. The parser reads five recognized fields. Unknown fields are preserved into the AST's `unknownFields` map but are not used by the platform; you may write internal-team metadata under the heading without breaking the grammar.

#### `type:`

One of the seven recognized constraint types. Each type signals the authorial intent of the constraint and is used by the engineer to route attention; the verification backend routing is by *fence kind* not by `type:`, but the two correspond closely in practice.

- **`specification`** — defines an interface or shape. *Example: "Export a function `slugify` with signature `export function slugify(input: string): string`."* The verification surface is typically the type checker; specification constraints rarely carry runtime evidence because the type checker is the cheap-and-fast first pass.

- **`predicate`** — states a property over inputs and outputs. *Example: "The function lowercases its input and replaces non-alphanumeric runs with a single hyphen."* The verification surface is typically `assert` blocks (canonical examples) and / or `property` blocks (fuzzed quantification).

- **`invariant`** — states a property that must always hold. *Example: "The function never throws and never returns a string containing two consecutive hyphens."* The verification surface is typically `property` blocks; an invariant invites quantified evidence.

- **`bridge`** — connects this requirement to an external commitment. *Example: "This module's authentication flow conforms to the team's RFC-1234 commitment."* The verification surface is typically the language-model judge with `judgment` blocks, because bridge constraints often refer to documents or commitments outside the file.

- **`methodology`** — describes the way a thing is done rather than what is produced. *Example: "Use the imported `canonicalize` and `sha256Hex` rather than reimplementing them."* The verification surface mixes the language-model judge and `pin` manifest entries.

- **`example`** — declares a worked example as evidence in its own right. *Example: "When the input is `"---x---"`, the output is `"x"`."* The verification surface is `assert` blocks.

- **`counterexample`** — declares an input or scenario the implementation must *not* exhibit. *Example: "When the input is empty, the function does not throw."* The verification surface is `assert` blocks (the assertion frames the negative case explicitly).

The seven types are the platform's recognized vocabulary. You will write *specification*, *predicate*, *invariant* most of the time; *bridge*, *methodology*, *example*, *counterexample* are tools for cases where authorial intent does not fit the first three.

#### `authority:`

One of `human-authored`, `AI-suggested-pending`, `derived`. Authority is provenance.

- **`human-authored`** — a person wrote this constraint's prose and stands behind it.
- **`AI-suggested-pending`** — a substrate proposed it; a human has not yet ratified it. The platform does not refuse to verify these constraints, but a discipline-following team will treat them as tentative until reviewed.
- **`derived`** — the constraint follows mechanically from another constraint in this file or in an imported file. Derived constraints are bookkeeping; they let you state a logical consequence explicitly without rewriting.

Authority does not affect verification routing. It affects review-time triage: a reviewer scanning a long constraint set knows where their attention is most valuable.

#### `scope:`

A free-form string naming the constraint's reach. *Module*, *engine*, *site*, *protocol* are typical values. Scope is informational for the platform but architectural for the engineer; it lets a team filter, group, and reason about a large constraint set.

#### `status:`

One of `active`, `deprecated`, `retracted`. Inactive constraints are kept in the file for archaeological purposes. The platform does not enforce deprecated or retracted constraints, but they are preserved in the canonical form so the file's history stays auditable.

A discipline worth adopting early: when retiring a constraint, change its `status` to `retracted` rather than deleting the block. The retraction ledger is the record of what changed and why; deletion erases the record.

#### `depends-on:`

An array of constraint identifiers in this file that this constraint depends on. The dependency graph is explicit. The platform uses it to order verification (a dependent constraint's evidence runs after its dependencies have passed). It also uses it to short-circuit (if a depended-on constraint fails verification, dependent constraints are skipped, with a `skip` verdict reported separately from `pass` and `fail`).

State the dependency graph honestly. If `SLUG2` depends on `SLUG1`'s function being defined, write `depends-on: [SLUG1]`. The graph is one of the cheaper pieces of structure the platform offers; using it correctly removes whole categories of misleading verification reports.

### 3.3 Body

Free prose. State what the requirement is, in the words you would use to brief a colleague. The prose is what the language-model substrate reads when it generates code; it is also what a human reviewer reads when judging whether the requirement has been authored well.

Three disciplines on the body, named here at a useful granularity (the full *Pin-Art* discipline catalogue lives at [Doc 659](/resolve/doc/659-rederive-for-the-working-engineer-hub) §F):

*Say the requirement once, plainly.* A two-sentence body that names the requirement directly is better than a five-paragraph body that hedges. The substrate is not a colleague who needs to be persuaded; it is a derivation function whose precision is bounded by yours.

*State the inline interface explicitly when types are at issue.* If your requirement depends on a TypeScript type definition, include the type definition inline as a fenced ` ```ts` block in the body. The substrate threads the inline types into the derived module, which prevents the derivation from inventing a structurally adjacent but incompatible type.

*Name the substrate-quirky thing if you have observed one.* If the substrate has, in past derivations, produced subtly wrong code for a particular interaction (a Web Crypto API quirk, a TypeScript strict-mode mismatch, a particular module-resolution corner), name the quirk in the body so the substrate is forewarned. The canonicalize-module sample in `samples/canonicalize-module.constraints.md` does this for the `crypto.subtle.digest` BufferSource quirk; engineers who do not warn the substrate find themselves debugging the same issue across regenerations.

### 3.4 Fenced Evidence

Zero or more fenced code blocks, with the fence language tag selecting the verification backend. The platform recognizes six evidence kinds:

- **` ```assert` ** (or ` ```assertion`) — assertion blocks. Each line is an executable expression that must evaluate true. The platform wraps them with a small harness and runs them against the derived code. Use for canonical examples.

- **` ```property` ** (or ` ```properties`) — property-test blocks. The block contains a property predicate; the platform fuzzes inputs against it. Use for invariants over a large input space.

- **` ```judgment` ** (or ` ```judgement`) — language-model-judge blocks. The block contains a prose criterion; a separate substrate call evaluates the derived code against the criterion. Use for properties that resist mechanical encoding.

- **` ```a11y` ** (or ` ```accessibility`) — accessibility-checker blocks. The block contains an expression that returns the rendered HTML; the platform applies a small static accessibility ruleset against the result. Use for UI work where basic a11y rules apply.

- **` ```flow` ** (or ` ```interaction`) — DOM flow blocks. The block contains a sequence of user actions and observation points; the platform instantiates the derived UI in a DOM and checks the flow. Use for UI work with declarative interaction patterns.

- **` ```ts` ** — inline TypeScript. Not an evidence kind in itself; this fence is read by the substrate as part of the body's inline-type declarations. It is included here because engineers will see it in samples and should know its role.

A constraint may carry several fenced blocks of different kinds. The platform routes each block to its backend in parallel; the constraint's verdict is *pass* only if every block passes. The detailed semantics of each backend live in [Doc 662](/resolve/doc/662-rederive-stub-c-the-verification-backends).

## 4. Three Sample Files, Read End-to-End

Three samples from the platform's repository, walked with annotation.

### 4.1 `slugify.constraints.md` — single-file, no manifest

```
# Slugify — sample constraint file

A minimal constraint set used to smoke-test the rederive engine.

## SLUG1
type: specification
authority: human-authored
scope: module
status: active
depends-on: []

Export a single function named `slugify` that takes a string and returns
a string. The function must have signature
`export function slugify(input: string): string`. The module must
type-check under strict TypeScript with no errors.

## SLUG2
type: predicate
authority: human-authored
scope: module
status: active
depends-on: [SLUG1]

The slugify function lowercases its input and replaces every run of
non-alphanumeric characters with a single hyphen. Leading and trailing
hyphens are stripped from the result.

```assert
slugify("Hello World") === "hello-world"
slugify("  Foo Bar  ") === "foo-bar"
slugify("Foo!!!Bar") === "foo-bar"
slugify("---x---") === "x"
slugify("UPPER") === "upper"
slugify("a1b2c3") === "a1b2c3"
```

## SLUG3
type: invariant
authority: human-authored
scope: module
status: active
depends-on: [SLUG1]

The slugify function never throws and never returns a string containing
two consecutive hyphens.

```assert
typeof slugify("") === "string"
!slugify("foo  bar  baz").includes("--")
!slugify("!!!@@@###").includes("--")
```
```

This is the platform's smoke test. Three constraints, no manifest, forty-six lines. SLUG1 is a *specification* constraint with no fenced evidence; the type checker is its evidence. SLUG2 is a *predicate* constraint with six canonical examples in an `assert` block. SLUG3 is an *invariant* constraint with three boundary-case assertions. SLUG2 and SLUG3 both depend on SLUG1; the dependency tells the platform to verify SLUG1 first and to skip the others if SLUG1 fails. The whole loop runs in a few seconds end-to-end.

### 4.2 `composed-hasher.constraints.md` — composition with imports

```
# Composed Hasher — composition primitive demo

@provides: composed-hasher-property
  threshold: HASH2
  interface: [constraintSetHash]

@imports:
  - property: canonicalize-property
    from: path
    path: ./canonicalize-module.constraints.md
    as: canonicalize-module

A demonstration of the composition primitive at module scope. ...

## HASH1
type: specification
authority: human-authored
scope: module
status: active
depends-on: []

Export `export async function constraintSetHash(ast: ConstraintAst):
Promise<string>` that imports `canonicalize` and `sha256Hex` from
`./canonicalize-module` and returns the SHA-256 hex hash of the
canonical bytes.

Define inline (must structurally match the canonicalize module's
expected input shape):

```ts
type CT = "specification" | "predicate" | "invariant" | "bridge"
        | "methodology" | "example" | "counterexample";
type CA = "human-authored" | "AI-suggested-pending" | "derived";
interface Constraint {
  id: string; type: CT; authority: CA;
  scope: string; status: string; dependsOn: string[]; body: string;
  unknownFields: Record<string, string>; sourceLine: number;
}
interface ConstraintAst {
  constraints: Constraint[];
  manifest?: { provides?: unknown; imports: unknown[] };
}
```

The implementation is brief: import the two functions from
`./canonicalize-module`, invoke them on the input AST, return the hex
string. Module must type-check strict.

## HASH2
type: invariant
authority: human-authored
scope: module
status: active
depends-on: [HASH1]

Identical ASTs produce identical hashes. Different ASTs produce
different hashes. The hash is always 64 lowercase hex characters.

```assert
const ast1 = { constraints: [...], manifest: { imports: [] } };
const ast2 = { constraints: [...], manifest: { imports: [] } };  // identical
const ast3 = { constraints: [...], manifest: { imports: [] } };  // different
const h1 = await constraintSetHash(ast1);
const h2 = await constraintSetHash(ast2);
const h3 = await constraintSetHash(ast3);
__assert(h1 === h2, "identical asts → identical hashes");
__assert(h1 !== h3, "different asts → different hashes");
__assert(/^[0-9a-f]{64}$/.test(h1), "64-char hex");
```
```

This file imports a property from another file. The manifest header declares both directions: `@provides` (this file emits a *composed-hasher-property* once HASH2 passes) and `@imports` (this file depends on the *canonicalize-property* from the canonicalize-module file). The HASH1 body declares the inline TypeScript interfaces explicitly with a ` ```ts` fence so the substrate threads them into the derivation; this is the *state the inline interface explicitly* discipline from §3.3 made concrete. HASH2 uses an explicit `__assert(<expr>, <message>)` form for richer evidence than the bare-expression form (the bare form is the default; the `__assert` form is available when you want to attach a label to a particular check).

### 4.3 `a11y-demo.constraints.md` — UI work with the a11y backend

```
# A11y demo — accessible button + form via static-rule a11y backend

Front-end stage-4 component. Demonstrates the `a11y-checker` backend by
deriving a small accessible-form component and asserting its rendered
HTML passes a11y rules: `img-alt`, `button-name`, `link-name`,
`input-label`, `lang-attr`.

## A11Y1
type: specification
authority: human-authored
scope: module
status: active
depends-on: []

Export `export function renderEmailForm(opts?: EmailFormOpts): string`.

Define inline:

```ts
interface EmailFormOpts {
  formId?: string;
  inputId?: string;
  helpText?: string;
}
```

The fragment must contain an accessible email input form. Specifically:

- A `<form>` with the form id (default `email-form`).
- A `<label for="...">Email address</label>` whose `for` references the
  input's id.
- An `<input type="email">` with the input id (default `email-input`),
  `name="email"`, `required`.
- A `<small id="email-help">` containing the help text (default `"We
  will not share your email."`), and the input has
  `aria-describedby="email-help"`.
- A `<button type="submit">Send</button>`.

Module must type-check strict.

## A11Y2
type: predicate
[...]

```assert
const html = renderEmailForm();
/<form\s+[^>]*id="email-form"/.test(html)
/<label\s+for="email-input"[^>]*>[^<]*Email address/.test(html)
[...]
```

## A11Y3
type: invariant
[...]

Accessibility check via the new `a11y-checker` backend. The rendered
HTML must pass the static a11y rules.

```a11y
return renderEmailForm();
```

```a11y
return renderEmailForm({ formId: "alt-form", inputId: "alt-input",
                        helpText: "Custom help." });
```
```

A UI constraint set, demonstrating the ` ```a11y` fence. A11Y1 declares the interface and the structural requirements in prose; A11Y2 verifies structural patterns with regex assertions over the rendered HTML; A11Y3 runs the accessibility checker against the rendered HTML by writing two `a11y` blocks, each returning a different parameterization of the rendered form. The discipline of separating *structural patterns* (A11Y2, regex-based) from *accessibility rules* (A11Y3, ruleset-based) is worth absorbing; the two surfaces are different and both useful.

## 5. Authoring Disciplines That Make Derivations Converge

Five disciplines for the engineer who has just written their first constraint file and wants the second one to derive in fewer rounds.

*State the requirement once, in prose.* Rewriting the same requirement three times in the body in slightly different words gives the substrate three slightly different things to satisfy and increases the chance one of them is satisfied at the expense of the others. Pick the one phrasing you think is clearest, write it, move on.

*Front-load the inline types.* If the constraint depends on TypeScript types, declare them in a ` ```ts` fence near the top of the body. The substrate's first action when generating the module is to copy these types in; if they are buried later in the body, the substrate may invent its own version that almost matches but does not.

*Use `assert` for canonical examples and `property` for invariants.* The discipline is: examples in `assert`, quantification in `property`, prose-y criteria in `judgment`. Mixing these confuses the verifier and produces evidence that is harder to read at review.

*Keep `depends-on` accurate.* A missing `depends-on` produces verification reports where dependent constraints fail with cryptic errors because their dependencies were not actually satisfied yet. An over-broad `depends-on` (every constraint depends on every prior constraint) serializes verification and slows the build for no benefit.

*Warn the substrate about quirks you have observed.* If a previous derivation produced a subtly wrong implementation for a known reason (an API quirk, a strict-mode interaction, a module-resolution corner), name it in the body. The discipline is *write what you know about the failure mode you have already debugged*, so you do not pay the same debug cost twice.

These five disciplines accelerate convergence noticeably. The fuller catalogue (eight disciplines, D1 through D8) lives at [Doc 659](/resolve/doc/659-rederive-for-the-working-engineer-hub) §F under the *Pin-Art* name; this is the practitioner-friendly subset.

## 6. Common Pitfalls

Six pitfalls the engineer will encounter before they have internalized the disciplines, with the recovery for each.

**Forgetting `depends-on`.** Symptom: dependent constraint fails with an error suggesting its dependency is undefined. Recovery: add the missing dependency to `depends-on:`.

**Reordering constraints in the file.** Symptom: the canonical hash changes even though "nothing changed." Recovery: this is intentional. Constraint order is observable to the substrate during derivation. If you reordered for clarity, accept the new hash; if you reordered by accident, revert the order.

**Writing a `judgment` block when an `assert` block would do.** Symptom: judgment passes but the implementation is subtly wrong; or judgment fails with prose reasoning that points to nothing concrete. Recovery: replace the judgment with assertions. If you find yourself reaching for a judgment because the criterion *resists* mechanical encoding, that is the right use; if you are reaching for it because writing assertions feels tedious, that is the wrong use.

**Pinning a behaviour rather than a phrase.** Symptom: the pin fails after a regeneration that produced semantically-equivalent but textually-different code. Recovery: convert the pin to a constraint. Pins are for surface text that downstream consumers parse; they are not for behaviour.

**Importing a file that is not yet itself materialized passing.** Symptom: resolve stage fails reporting that the imported threshold has not passed. Recovery: derive the imported file first; the platform refuses to import unverified properties.

**Over-broad `scope`.** Symptom: a small constraint set claims `scope: engine` and gets reviewed at engine-team velocity rather than module-team velocity. Recovery: scope honestly. *Module* for module-local concerns, *engine* for engine-level commitments, *site* / *protocol* for cross-cutting.

## 7. Three Conventions Worth Adopting Before You Need Them

*Filename mirrors the deliverable.* `slugify.constraints.md` derives `slugify`. `canonicalize-module.constraints.md` derives the canonicalize module. The convention is `<thing>.constraints.md`. Materializations land at `<thing>.constraints.md.materialization.json`. The platform tooling expects this layout for cache-key resolution.

*Use the file's top-level prose paragraph for the human reader.* Markdown allows free prose between the H1 title and the first H2 constraint heading. The platform ignores this prose; the parser starts looking for constraints at the first H2. Use the space for the reviewer who is reading the file cold: a one-paragraph summary of what this constraint set is for.

*Keep the manifest header above every comment-only paragraph.* The parser treats the first non-comment, non-whitespace line as either a manifest directive (if it starts with `@`) or part of the H1. If you write a long preamble before the manifest, the parser will read it correctly, but a reviewer scanning the file may miss the manifest. Convention: H1 + one-paragraph summary + manifest + body constraints.

## 8. What the Engineer Should Take Away

Three pieces.

*The grammar is small.* H2 + metadata + body + fenced evidence, plus an optional manifest header for composition and pinning. There is no DSL. There is no schema migration. A constraint file is a Markdown file a colleague can read in any text editor without tooling.

*The metadata is load-bearing for the platform's behaviour.* The `type:` field signals authorial intent (and corresponds to the fence-routing convention). The `depends-on:` field orders verification and short-circuits. The `authority:` field is for review-time triage. The `status:` field is for the retraction ledger. Get these right and the platform handles the rest.

*The fenced evidence is what carries the contract.* If a requirement matters, it has evidence. If a requirement does not have evidence, the platform cannot verify it, and the engineer is back in the world where requirements drift away from code. The discipline of writing evidence next to the prose is what makes the constraint file the durable artifact rather than the comment-on-the-code in the requirements folder.

The next document in the series ([Doc 661](/resolve/doc/661-rederive-stub-b-the-build-pipeline)) covers what the platform does with this file once you run the CLI: the eight stages of the build pipeline, what each one produces, what failure looks like, and how to debug. After that, [Doc 662](/resolve/doc/662-rederive-stub-c-the-verification-backends) covers each backend the metadata routes to. Together those two documents close the loop from *file on disk* to *signed materialization*.

---

## References

- [Doc 247 — Derivation Inversion](/resolve/doc/247-derivation-inversion)
- [Doc 290 — Pin-Art Derivation](/resolve/doc/290-pin-art-derivation)
- [Doc 415 — The Retraction Ledger](/resolve/doc/415-the-retraction-ledger)
- [Doc 541 — Systems-Induced Property Emergence (SIPE-T)](/resolve/doc/541-systems-induced-property-emergence)
- [Doc 619 — Pin-Art: Forced-Press and Gentle-Press](/resolve/doc/619-pin-art)
- [Doc 656 — Treat Agent Output Like Compiler Output: The Lights-Out Codebase as Rederive](/resolve/doc/656-treat-agent-output-like-compiler-output-the-lights-out-codebase-as-rederive)
- [Doc 658 — Hierarchical Pin-Art Constraint Specifications and the Erasure of Edge-Case Bugs](/resolve/doc/658-hierarchical-pin-art-constraint-specs-and-the-erasure-of-edge-case-bugs)
- [Doc 659 — Rederive for the Working Engineer: A Hub](/resolve/doc/659-rederive-for-the-working-engineer-hub)

## Appendix: Originating Prompt

> *"Now I want you to examine the entire spec and implementation of /home/jaredef/rederive — From this I want you to create entracement docs in the corpus for an audience of software engineers. ... First, create a document in the corpus that acts as a hub for all stubs that will branch off ... Append this prompt to each artifact. Also, where you could use corpus jargon, instead entrace the reader through rhetoric that is not novel to the corpus. Where you must state the corpus concept in its own terms; provide proper entracement."*
>
> Followed by: *"Continue with the first branch of the hub derived as a corpus doc. Continue through each as is coherent."*
>
> Followed by: *"Now fill out stub A as a comprehensive entracement. Remove 'stub' from file name and any mention in the doc. Report back before doing likewise to the next doc."*
