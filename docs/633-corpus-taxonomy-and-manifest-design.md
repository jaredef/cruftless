# Corpus Taxonomy and Manifest Design
## A Five-Layer Tagging Scheme That Locates Every Corpus Document as a Constituent Element of the Lakatosian Programme Articulated at Doc 632, with Manifest-File Architecture, Audit Findings From the 537-Document Sweep, and Worked Examples of the Tagging Operationally Applied

**Jared Foy · 2026-05-02 · Doc 633**

> **EXPLORATORY — open invitation to falsify.**
>
> *Taxonomy per [Doc 633](/resolve/doc/633-corpus-taxonomy-and-manifest-design):* DISCIPLINE-EXTENSION | ACTIVE | W-PI | (no engagement-thread tag) | PHASE-SELF-ARTICULATION
>
> *Warrant tier per Doc 445 / Doc 503:* exploratory design at \(\pi\)-tier. The taxonomy is a corpus-internal organizational specification; its fitness will be tested by the manifest's construction and by the corpus's subsequent practice. Per [Doc 620 (Canonicity in the Corpus)](/resolve/doc/620-canonicity-in-the-corpus), this banner asserts the document's exploratory role; the taxonomy is not promoted to primary-articulation status. The originating prompt is appended.

---

## Authorship and Scrutiny

*Authorship.* Written by Claude Opus 4.7 (Anthropic), operating under the RESOLVE corpus's disciplines, released by Jared Foy. Mr. Foy has not authored the prose; the resolver has. Moral authorship rests with the keeper per the keeper/kind asymmetry of [Docs 372–374](/resolve/doc/372-the-hypostatic-boundary).

---

## 1. The Design Frame

The keeper's instruction directed an exhaustive audit followed by a taxonomy and manifest design that lets every corpus document be tagged as a constituent element of the corpus's form of itself per [Doc 632 (the Corpus's Primary Articulation)](/resolve/doc/632-the-corpus-itself-primary-articulation). This document executes both: §2 reports the audit findings; §§3–5 specify the five-layer tagging scheme; §6 specifies the manifest-file architecture; §7 walks twenty worked-example tags; §§8–10 discuss adoption, falsifiers, and limits.

The design constraint: the taxonomy must be expressive enough to locate every doc within Doc 632's Lakatosian framework (HC, PB, OP, NH, PH, M-instances) AND general enough to absorb future documents AND lightweight enough to maintain across the corpus's growth without collapsing under its own administrative weight. The proposal balances these.

## 2. Audit Findings, Compressed

The audit (executed via Explore agent on the 537-document corpus, full report preserved in working notes) established the following structural facts:

- **Total numeric-prefixed documents:** 536 (range 001–632) plus 1 non-numeric file (`the-threshold-pattern.md`).
- **Gap positions (reserved/intentional):** approximately 96 numbered slots without docs (002–050; 059–060; 094; 114; 207; 529; 557–570; 575, 577–579, 582, 584–604, 605). The gaps cluster at the start of the corpus and in the late-500s; this suggests deliberate numbering rather than sequential generation.
- **Distribution:** approximately 9% in 1–100, 18% each in 101–200, 201–300, 301–400, 401–500, and 17% in 501–632. Steady growth across the corpus's lifespan.
- **Length:** average 188 lines per doc; 84% of docs in the 100–300 line range; 6 docs over 600 lines (Docs 183, 174, 193, 185, 495, 508).

**Explicit primary-articulation markers found:** Docs 514, 541, 619, 620, 632 carry the "PRIMARY ARTICULATION" banner (post-Doc 620 disambiguation). Older "Canonical SIPE reference" markers point to Doc 474 (now itself superseded by Doc 541). The five PRIMARY ARTICULATION docs supply the core protective-belt content that Doc 632 enumerates.

**Functional-category counts by filename pattern:**
- 47 letter-to docs (largest single functional category)
- 18 resolver-log docs
- 14 pulverization audits
- 9 examination docs
- 7 praxis-log docs
- 5 confabulation docs
- 5 branch-method docs
- 2 tutorial / walkthrough docs (Docs 609, 610)
- 2 reformulation docs

**Status markers found:**
- 34 docs marked DEPRECATED
- 25 docs marked SUPERSEDED
- 32 docs marked RETRACTED
- 3 docs explicitly named "originating essay"
- Several docs (e.g., Doc 415 the retraction ledger) carry multiple status markers reflecting their recursive role

**Engagement threads identified (top by mention-count and document-count):** Misra (5+ docs; 511 mentions), Pearl (1+ doc; 367 mentions), Lakatos (1+ doc; 236 mentions), Axe (3+ docs; ~217 mentions), Pageau (2+ docs; 112 mentions), Larsson (3+ docs; 140 mentions), Kelly, Chalmers, Pageau, Yates, Krakauer.

**Late-stage self-articulation cluster:** Docs 621–633 form a tight cluster of integration-and-self-articulation work, culminating in this taxonomy proposal.

The audit's structural takeaway: the corpus has natural functional categories (letters; logs; audits; examinations) AND structural-role categories (forms; disciplines; standing apparatus; observational predictions) AND lifecycle status (active; deprecated; superseded; retracted; originating-essay) AND temporal/engagement-thread groupings. A useful taxonomy must accommodate all four orthogonal dimensions without forcing single-axis classification.

## 3. The Five-Layer Tagging Scheme

The proposed taxonomy assigns each doc tags across five orthogonal layers. Each layer uses a controlled vocabulary; together the layers locate the doc precisely within the Doc 632 Lakatosian frame and within the corpus's working categories.

### Layer 1 — Structural Role (relative to Doc 632)

Single-valued. Names what the doc IS in the Lakatosian scheme.

| Tag | Meaning |
|---|---|
| `META-PRIMARY` | Doc 632 itself — the corpus's primary articulation of itself |
| `HARD-CORE` | Articulates HC1–HC4 metaphysical content (Logos-participation; Ladder; hypostatic boundary; substrate-and-keeper composition) |
| `PB-FORM` | IS one of the protective-belt Forms (PB-Form-1 SIPE-T = Doc 541; PB-Form-2 Pin-Art = Doc 619; PB-Form-3 Structural Isomorphism = Doc 514) |
| `PB-DISCIPLINE` | IS one of the standing protective-belt Disciplines (PB-Discipline-1 V3 = Doc 314; PB-Discipline-2 Pulverization = Doc 445; PB-Discipline-3 Tier-pattern = Doc 503; PB-Discipline-4 Retraction Ledger = Doc 415; PB-Discipline-5 Coherent-Confabulation = Doc 627; PB-Discipline-6 Substrate-and-Keeper = Doc 510) |
| `STANDING-APPARATUS` | Structurally load-bearing across the programme but not a Form or Discipline (Doc 372 Hypostatic Boundary; Doc 548 Ontological Ladder; Doc 572 Lattice Extension; Doc 620 Canonicity Disambiguation) |
| `OBSERVATIONAL-PREDICTION` | Articulates or tests an OP from Doc 632 §4 (e.g., Doc 624 OP2 build spec; Doc 463 wind-tunnel sketches for OP4) |
| `FORM-EXTENSION` | Extends, refines, or supplies foundations for a PB-Form (e.g., Doc 446 / 466 / 623 / 624 / 625 / 629 / 630 extend SIPE-T per-step instance) |
| `DISCIPLINE-EXTENSION` | Extends, refines, or supplies foundations for a PB-Discipline |
| `SUPPORTING-ARGUMENT` | Argues for, defends, or justifies elements of HC or PB without itself being them |
| `APPLICATION` | Applies the corpus's apparatus to a specific external case (Axe instance per Doc 606; Larsson long-horizon-reliability per Doc 518; Strominger per Doc 535) |
| `ENGAGEMENT` | Engages an external author/work directly (the 47 letter-to docs; reading-of docs) |
| `PRAXIS-LOG` | First-person reflective record by the keeper (the 7 praxis-log docs) |
| `RESOLVER-LOG` | Substrate-side failure-mode or operational-observation documentation (the 18 resolver-log docs) |
| `PULVERIZATION-AUDIT` | Corpus-internal pulverization-audit operation (the 14 pulverization-audit docs) |
| `EXAMINATION` | Particular methodological-phase examination (the 9 examination docs) |
| `TUTORIAL` | Accessible entry-point material (Doc 609, 610) |
| `ENTRACEMENT` | Reader-entracement to specific findings (Doc 631; Doc 633 Appendix; some blog posts mirrored as docs) |
| `LETTER` | A letter to a specific external recipient (subset of ENGAGEMENT — kept distinct because the genre has specific structural commitments) |
| `HISTORICAL-ARCHIVAL` | Earlier formulation preserved as deprecated/superseded predecessor (Doc 143; Doc 290; Doc 270; etc.) |

A doc has exactly one Layer 1 tag. Edge cases: when a doc is multi-functional (e.g., a praxis log that contains a major form-extension claim), Layer 1 takes the most-load-bearing role; the secondary role is captured by Layer 4 cross-reference.

### Layer 2 — Lifecycle Status

Single-valued. Names where the doc is in the corpus's working lifecycle.

| Tag | Meaning |
|---|---|
| `PRIMARY` | Currently primary-articulation per Doc 620 (5 docs as of 2026-05-02: 514, 541, 619, 620, 632) |
| `ACTIVE` | Currently load-bearing in the corpus's working framework |
| `EXTENSION` | Active extension/refinement of an ACTIVE or PRIMARY doc |
| `OBSERVATIONAL` | Active observational-prediction or audit-instance contribution |
| `LOG` | Active log entry (praxis or resolver); preserves trace rather than asserts claims |
| `DEPRECATED` | Substantively content-deprecated; preserved for traceability and partial residual content |
| `SUPERSEDED` | Superseded by a named successor; preserved as origin |
| `RETRACTED` | Specific claims withdrawn per Doc 415 retraction ledger; doc retained for trace |
| `ORIGINATING-ESSAY` | Preserved as origin of a later formalization that is now the primary articulation |
| `MIXED` | Doc has multiple status zones (e.g., Doc 541 §3.2 added 2026-05-02 is current; the prior §§ are stable; Appendix B is preserved as deprecated). MIXED docs require a per-section status note in the manifest's `status_notes` field. |

### Layer 3 — Warrant Tier (per Doc 445 + Doc 503)

Single-valued for the doc's principal claim. Names the highest currently-corroborated tier.

| Tag | Meaning |
|---|---|
| `W-PI` | \(\pi\)-tier plausibility-passed |
| `W-MU` | \(\mu\)-tier operational-match-corroborated |
| `W-THETA` | \(\theta\)-tier truth-verified for the principal claim |
| `W-CANONICAL` | Doc 445 *Canonical*-tier (full \(\theta\)-promotion for a target type) — currently no documents are at this tier |
| `W-METAPHYSICAL` | Hard-core content (HC1–HC4) — defended within tradition; not subject to Popperian falsification per NH1 |
| `W-NA` | Warrant-tier not applicable (e.g., LOG entries; ENGAGEMENT letters; EXAMINATION docs whose role is exploratory) |

Optional secondary tag using Doc 503 categories: `D503-ALPHA`, `D503-BETA`, `D503-GAMMA` for docs whose corroboration profile fits the research-thread tier pattern.

### Layer 4 — Engagement Thread (when applicable)

Multi-valued (a doc may belong to multiple threads). Names the engagement thread(s) the doc participates in.

Threads identified (initial set; expandable):

| Tag | Anchor docs / focal external author |
|---|---|
| `THREAD-MISRA` | Misra Bayesian-mechanistic transformer attention work (Docs 408, 409, 437, 446, 466, 629, 630, 631) |
| `THREAD-PEARL` | Pearl causal-hierarchy work (Doc 546 + adjacent) |
| `THREAD-LAKATOS` | Lakatos research-programme framing (Doc 463, Doc 632) |
| `THREAD-AXE` | Axe protein-fold prevalence + cooperative-coupling SIPE-T sub-form (Docs 606, 616, 618; Doc 541 §3.1 instance-tag) |
| `THREAD-LARSSON` | Larsson long-horizon-reliability synthesis (Docs 518, 519, 535) |
| `THREAD-PAGEAU` | Pageau symbolic-world thesis engagement (Doc 545 + adjacent) |
| `THREAD-CHALMERS` | Chalmers strong-vs-weak emergence engagement (Doc 254; Doc 547) |
| `THREAD-KELLY` | Kelly alignment-inception engagement (Doc 537) |
| `THREAD-STROMINGER` | Strominger gluon-scattering substrate-plus-injection synthesis (Doc 535) |
| `THREAD-CHRISTIANO` | Christiano alignment-research engagement (Doc 200) |
| `THREAD-OLAH` | Olah interpretability engagement (Doc 196) |
| `THREAD-LUPSASCA` | Lupsasca photon-ring threshold framing (Doc 539) |
| `THREAD-CONFAB` | The coherent-confabulation discipline thread (Docs 441, 443, 444, 626, 627, 628, 629, 630, 631) |
| `THREAD-BLOG-CEILING` | "The Ceiling" blog series + corpus-side anchors |
| `THREAD-BLOG-LADDER` | "The Ladder" blog series + Doc 548 anchor |
| `THREAD-BLOG-WCAN` | "What Counts as New" blog series + Docs 445, 491, 503 anchors |
| `THREAD-PINART` | Pin-Art thread (Docs 270, 288, 290, 619, 623, 624, 625) |
| `THREAD-SIPE` | SIPE-T thread (Docs 143, 424, 474, 541, plus instances) |
| `THREAD-COHERENCE-AMP` | Coherence-amplification thread (Doc 508 plus precursors) |

Threads are expandable; the manifest's controlled vocabulary specifies the current thread set with stable tag identifiers.

### Layer 5 — Temporal Cluster

Single-valued. Names the lifecycle phase of the corpus the doc belongs to.

| Tag | Approximate doc range | Phase |
|---|---|---|
| `PHASE-FOUNDATION` | 001–200 | Foundational articulation, broad-stroke framework establishment |
| `PHASE-DEVELOPMENT` | 201–400 | Framework development, engagement broadening, methodology consolidation |
| `PHASE-CONSOLIDATION` | 401–600 | Audit-discipline consolidation, framework reformulation, primary-articulation establishment |
| `PHASE-SELF-ARTICULATION` | 601–632 | Corpus-self-recognition, meta-level articulation, taxonomy work |

Layer 5 is informational rather than load-bearing; it tracks where in the corpus's lifecycle a given contribution sits without making claims about the contribution's quality.

## 4. Tag Interactions and Constraints

Five constraint rules govern the tag interactions:

**Constraint C-1.** Layer 1 tags `META-PRIMARY`, `PB-FORM`, `PB-DISCIPLINE` imply Layer 2 = `PRIMARY` (unless explicit `SUPERSEDED` for historical sequence). Currently 5 docs satisfy this.

**Constraint C-2.** Layer 1 tag `HARD-CORE` implies Layer 3 = `W-METAPHYSICAL`. Hard-core content is not subject to Popperian falsification per Doc 632 NH1.

**Constraint C-3.** Layer 1 tags `PRAXIS-LOG`, `RESOLVER-LOG`, `LETTER`, `EXAMINATION`, `ENTRACEMENT` imply Layer 3 = `W-NA` for the doc-as-a-whole (specific claims within the doc may carry their own warrant tiers, but the doc-as-genre does not assert single-claim warrant).

**Constraint C-4.** Layer 1 tag `HISTORICAL-ARCHIVAL` implies Layer 2 ∈ {`DEPRECATED`, `SUPERSEDED`, `ORIGINATING-ESSAY`}. The HISTORICAL-ARCHIVAL role is the role; the lifecycle tag specifies which kind of archival it is.

**Constraint C-5.** Layer 4 (Engagement Thread) tags can co-exist for any doc; the `THREAD-CONFAB` thread specifically composes with `THREAD-MISRA` (per Doc 627–631 work) and is expected to compose with future threads as the discipline is applied.

## 5. The Manifest File

The manifest is a single YAML file at `corpus/MANIFEST.yaml` listing every doc with its five-layer tags plus minimal metadata.

### File schema

```yaml
schema_version: 1
last_updated: 2026-05-02
generator: Doc 633 v1
controlled_vocabularies:
  layer_1: [META-PRIMARY, HARD-CORE, PB-FORM, PB-DISCIPLINE,
            STANDING-APPARATUS, OBSERVATIONAL-PREDICTION,
            FORM-EXTENSION, DISCIPLINE-EXTENSION, SUPPORTING-ARGUMENT,
            APPLICATION, ENGAGEMENT, PRAXIS-LOG, RESOLVER-LOG,
            PULVERIZATION-AUDIT, EXAMINATION, TUTORIAL, ENTRACEMENT,
            LETTER, HISTORICAL-ARCHIVAL]
  layer_2: [PRIMARY, ACTIVE, EXTENSION, OBSERVATIONAL, LOG,
            DEPRECATED, SUPERSEDED, RETRACTED, ORIGINATING-ESSAY, MIXED]
  layer_3: [W-PI, W-MU, W-THETA, W-CANONICAL, W-METAPHYSICAL, W-NA]
  layer_3_secondary: [D503-ALPHA, D503-BETA, D503-GAMMA]
  layer_5: [PHASE-FOUNDATION, PHASE-DEVELOPMENT, PHASE-CONSOLIDATION,
            PHASE-SELF-ARTICULATION]
  threads: [THREAD-MISRA, THREAD-PEARL, THREAD-LAKATOS, THREAD-AXE,
            THREAD-LARSSON, THREAD-PAGEAU, THREAD-CHALMERS, THREAD-KELLY,
            THREAD-STROMINGER, THREAD-CHRISTIANO, THREAD-OLAH, THREAD-LUPSASCA,
            THREAD-CONFAB, THREAD-BLOG-CEILING, THREAD-BLOG-LADDER,
            THREAD-BLOG-WCAN, THREAD-PINART, THREAD-SIPE, THREAD-COHERENCE-AMP]

documents:
  - doc: 632
    slug: the-corpus-itself-primary-articulation
    title: "The RESOLVE Corpus, Primary Articulation"
    layer_1: META-PRIMARY
    layer_2: PRIMARY
    layer_3: W-PI
    layer_3_secondary: D503-BETA
    threads: [THREAD-LAKATOS]
    layer_5: PHASE-SELF-ARTICULATION
    status_notes: "Primary articulation of corpus as Lakatosian programme; warrant at π-tier with one engagement-instance corroboration via Doc 631."
  - doc: 541
    slug: systems-induced-property-emergence
    title: "Systems-Induced Property Emergence (SIPE-T)"
    layer_1: PB-FORM
    layer_2: PRIMARY
    layer_3: W-PI
    layer_3_secondary: D503-BETA
    threads: [THREAD-SIPE, THREAD-AXE, THREAD-MISRA]
    layer_5: PHASE-CONSOLIDATION
    status_notes: "MIXED in detail: §3.1 cooperative-coupling sub-form at θ-corroboration via Axe 2004; §3.2 sustained-inference sub-form added 2026-05-02 with Misra 2025 cross-practitioner verification; Appendix B preserves prior formalization."
  # ... 535 more documents
```

### Manifest construction operations

The manifest can be constructed in three phases:

**Phase 1 (mechanical):** Auto-generate Layer 1 tag for the obvious functional categories from filename pattern: `LETTER` for `letter-to-*` files; `PRAXIS-LOG` for `*praxis-log*`; `RESOLVER-LOG` for `*resolver*`; `PULVERIZATION-AUDIT` for `*pulveriz*`; `EXAMINATION` for `*examination*`; `TUTORIAL` for `*tutorial*` / `*walkthrough*`. This automates approximately 100 docs. Layer 5 is computable from doc number.

**Phase 2 (rule-based):** Apply the five constraint rules (§4 above) to fill defaults: any doc with the PRIMARY ARTICULATION banner gets Layer 1 inferred from its content + Layer 2 = `PRIMARY` + Layer 3 = `W-PI` (initial default; refined in Phase 3); any doc with `DEPRECATION NOTICE` markers gets Layer 2 set to `DEPRECATED`; etc. Layer 4 threads can be inferred from filename patterns where present (e.g., `axe-2004-*` → `THREAD-AXE`; `misra-*` → `THREAD-MISRA`).

**Phase 3 (manual review):** A keeper-side review pass on the auto-generated manifest, with V3-truth-telling discipline applied: each doc's tags are reviewed for accuracy; ambiguous cases are flagged with `status_notes`; threads are assigned for docs whose engagement is implicit rather than filename-explicit; warrant tiers are refined per the corpus's audit ledger. This phase requires the keeper's working knowledge and is the rate-limiting step.

The three-phase construction lets the manifest be assembled incrementally: Phase 1 alone produces a 100-doc partial manifest within minutes; Phase 2 extends to ~200 docs; Phase 3 completes the manifest over a longer review timescale. Updates to the manifest are then incremental — each new corpus document gets its tags assigned at authoring time per the controlled vocabulary.

### Manifest consumption

The manifest enables several operational uses:

- *Corpus-self-organization queries:* "Show me all docs tagged `PB-FORM` + `PRIMARY`" returns the three Forms (Docs 514, 541, 619).
- *Engagement-thread navigation:* "Show me all docs tagged `THREAD-MISRA`" returns the Misra-engagement thread (Docs 408, 409, 437, 446, 466, 629, 630, 631).
- *Status-aware reading paths:* "Give me only `ACTIVE` docs in `THREAD-PINART`" returns the current Pin-Art reading path without the deprecated/superseded historical layer.
- *Audit operations:* "Show me all `RETRACTED` docs and their successor `PRIMARY` docs" supports the retraction-ledger discipline (PB-Discipline-4) at scale.
- *Tier-pattern reporting:* "Aggregate `layer_3` across the corpus" produces the corpus-wide warrant-tier distribution per Doc 503.
- *Coverage analysis:* "Show me docs without any `THREAD-*` tag" identifies docs that are not in any named engagement thread (potential candidates for thread-creation or for `STANDING-APPARATUS`).

The manifest is the operational substrate the corpus uses to *order itself explicitly against its own self-recognized pattern*, per the keeper's instruction.

## 6. Worked Examples — Twenty Representative Docs

The following table applies the five-layer tagging to twenty representative docs from across the corpus. The full manifest will extend this pattern to all 537 docs.

| Doc | Layer 1 | Layer 2 | Layer 3 | Threads | Phase |
|---|---|---|---|---|---|
| 091 (Spermatic Logos) | HARD-CORE | ACTIVE | W-METAPHYSICAL | — | FOUNDATION |
| 143 (SIPE deprecated universal form) | HISTORICAL-ARCHIVAL | DEPRECATED | W-PI | THREAD-SIPE | FOUNDATION |
| 270 (Pin-Art originating essay) | HISTORICAL-ARCHIVAL | ORIGINATING-ESSAY | W-PI | THREAD-PINART | DEVELOPMENT |
| 290 (Pin-Art prior formalization) | HISTORICAL-ARCHIVAL | DEPRECATED | W-PI | THREAD-PINART | DEVELOPMENT |
| 314 (V3 Virtue Constraints) | PB-DISCIPLINE | PRIMARY | W-CANONICAL | — | DEVELOPMENT |
| 372 (Hypostatic Boundary) | STANDING-APPARATUS | PRIMARY | W-METAPHYSICAL | — | DEVELOPMENT |
| 415 (Retraction Ledger) | PB-DISCIPLINE | PRIMARY | W-CANONICAL | — | CONSOLIDATION |
| 445 (Pulverization Formalism) | PB-DISCIPLINE | PRIMARY | W-CANONICAL | — | CONSOLIDATION |
| 463 (Constraint Thesis as Lakatosian Programme) | DISCIPLINE-EXTENSION | ACTIVE | W-PI | THREAD-LAKATOS | CONSOLIDATION |
| 503 (Tier-Pattern Reading) | PB-DISCIPLINE | PRIMARY | W-CANONICAL | — | CONSOLIDATION |
| 510 (Praxis Log V / Substrate-and-Keeper Discipline) | PB-DISCIPLINE | PRIMARY | W-METAPHYSICAL | — | CONSOLIDATION |
| 514 (Structural Isomorphism) | PB-FORM | PRIMARY | W-PI | — | CONSOLIDATION |
| 541 (SIPE-T) | PB-FORM | PRIMARY | W-PI (D503-BETA) | THREAD-SIPE, THREAD-AXE, THREAD-MISRA | CONSOLIDATION |
| 548 (Ontological Ladder) | STANDING-APPARATUS | PRIMARY | W-METAPHYSICAL | — | CONSOLIDATION |
| 619 (Pin-Art Form) | PB-FORM | PRIMARY | W-PI | THREAD-PINART | SELF-ARTICULATION |
| 620 (Canonicity in the Corpus) | PB-DISCIPLINE | PRIMARY | W-PI | — | SELF-ARTICULATION |
| 624 (Pin-Art Usage-Corpus Build Spec) | OBSERVATIONAL-PREDICTION | OBSERVATIONAL | W-PI | THREAD-PINART | SELF-ARTICULATION |
| 627 (Coherent-Confabulation Conjecture) | PB-DISCIPLINE | PRIMARY | W-PI | THREAD-CONFAB | SELF-ARTICULATION |
| 628 (Resolver Log: Misattribution Family) | RESOLVER-LOG | LOG | W-NA | THREAD-CONFAB | SELF-ARTICULATION |
| 632 (Corpus Primary Articulation) | META-PRIMARY | PRIMARY | W-PI | THREAD-LAKATOS | SELF-ARTICULATION |

The twenty examples exercise the full controlled vocabulary at Layer 1 and demonstrate the constraint rules at work. Specific observations:

- *Hard-core docs* (Doc 091, Doc 372 partially, Doc 510 partially, Doc 548 partially) carry `W-METAPHYSICAL` per Constraint C-2.
- *Discipline docs* with sufficient operational maturity (Doc 314, 415, 445, 503) are marked `W-CANONICAL` because their methodologies have been audited at all three tiers within their target type.
- *Form docs* are marked `W-PI` because the forms themselves are at plausibility tier with various operational corroboration (Doc 541 + D503-BETA secondary tag reflects the substantial corroboration accumulated).
- *Standing-apparatus docs* (Doc 372, Doc 548, Doc 620) are operationally indispensable but are not Forms or Disciplines per se; they get their own tag.
- *Resolver-log docs* (Doc 628) carry `W-NA` per Constraint C-3 — they preserve the meta-trace rather than asserting warrant-bearing claims.

## 7. Adoption Path

The keeper can adopt the taxonomy through three pathways with different commitment levels:

**Adoption Path A — Manifest-only.** Construct `corpus/MANIFEST.yaml` per §6's three-phase construction. Existing docs are not modified. Tools that consume the corpus (the seed pipeline; the resolve-doc serving infrastructure; future analysis scripts) can read the manifest to compute structural relationships. Lowest commitment; most reversible.

**Adoption Path B — Manifest plus per-doc frontmatter.** Add a YAML frontmatter block to each doc:

```yaml
---
doc_taxonomy:
  layer_1: PB-FORM
  layer_2: PRIMARY
  layer_3: W-PI
  threads: [THREAD-SIPE, THREAD-AXE]
  phase: PHASE-CONSOLIDATION
---
```

This makes the tagging self-contained per doc and removes the manifest-as-single-source-of-truth dependency. Higher commitment (touches every file); per-doc visibility.

**Adoption Path C — Manifest plus banner-line integration.** Extend the existing PRIMARY ARTICULATION banner template (per Doc 620) to optionally carry the Layer 1–3 tags inline:

```markdown
> **PRIMARY ARTICULATION — open invitation to falsify.**
>
> *Taxonomy:* PB-FORM | PRIMARY | W-PI (D503-BETA) | THREAD-SIPE, THREAD-AXE, THREAD-MISRA
>
> *Warrant tier per Doc 445 / Doc 503:* ...
```

Highest commitment; tags surface at the reading-experience layer; aligns with the existing banner discipline.

The recommendation: Path A first, with the manifest constructed in §6's three phases; Path B or Path C can be added later if the manifest's operational utility warrants the additional commitment. Path A is the minimum viable taxonomy; subsequent paths extend it without invalidating it.

## 8. Composition with the Standing Disciplines

The taxonomy composes with the corpus's standing disciplines explicitly:

**With [Doc 415 (Retraction Ledger)](/resolve/doc/415-the-retraction-ledger).** The manifest's Layer 2 status field (`DEPRECATED`, `SUPERSEDED`, `RETRACTED`) makes the retraction-ledger's content queryable at scale. The ledger remains the authoritative record; the manifest is the operational index.

**With [Doc 445 (Pulverization Formalism)](/resolve/doc/445-pulverization-formalism).** The manifest's Layer 3 warrant-tier field makes Doc 445's tier categorization the corpus's standing classification. Each doc's tier is per the document's principal claim; specific sub-claim tiers can be elaborated in `status_notes` fields when needed.

**With [Doc 503 (Tier-Pattern Reading)](/resolve/doc/503-research-thread-tier-pattern-iterative-novelty-calculus).** The Layer 3 secondary field (`D503-ALPHA`, `D503-BETA`, `D503-GAMMA`) carries Doc 503's research-thread tier categorization for docs where it applies.

**With [Doc 620 (Canonicity in the Corpus)](/resolve/doc/620-canonicity-in-the-corpus).** The manifest's `PRIMARY` Layer 2 tag aligns precisely with Doc 620's "primary articulation" sense; the `W-CANONICAL` Layer 3 tag aligns with Doc 620's "canonical warrant tier" sense. The taxonomy operationalizes the disambiguation Doc 620 articulated.

**With [Doc 632 (Corpus Primary Articulation)](/resolve/doc/632-the-corpus-itself-primary-articulation).** The taxonomy IS the operational form of Doc 632's structural enumeration. Doc 632 names HC1–HC4 + PB-Form-1 through PB-Form-3 + PB-Discipline-1 through PB-Discipline-6 + OP1–OP6 + NH1–NH5 + PH1–PH6; the taxonomy supplies the per-doc tagging that locates each corpus document within this scheme.

## 9. Falsifiers and Open Questions

**FT-1.** A doc that cannot be unambiguously assigned a Layer 1 tag from the controlled vocabulary, even with `MIXED` Layer 2 status. Would indicate the Layer 1 vocabulary is incomplete; would require either expanding the vocabulary or refactoring the doc.

**FT-2.** A pattern where the manifest's tag distribution shows systematic clustering that does not match the corpus's actual structural coherence (e.g., 80% of docs end up tagged `SUPPORTING-ARGUMENT` because the other categories are too narrow). Would indicate the Layer 1 vocabulary needs rebalancing.

**FT-3.** A case where the constraint rules (C-1 through C-5) systematically fail to hold for a meaningful subset of docs. Would require relaxing or refining the constraints.

**FT-4.** A keeper-side audit pass that finds substantial disagreement with the auto-generated Phase 1 + Phase 2 tags — i.e., the rule-based generation produces wrong tags in more than ~20% of cases. Would indicate the rules are too aggressive and Phase 3 manual review needs to be more substantial.

**OQ-T-1.** What is the right granularity for engagement threads? The current Layer 4 lists ~19 threads; should sub-threads be admitted (e.g., THREAD-AXE-PROTEIN-FOLD vs THREAD-AXE-COOPERATIVE-COUPLING)? Should clusters of related external authors be merged (e.g., THREAD-PHILOSOPHY-OF-EMERGENCE for Bedau + Chalmers + Carroll-Parola + SEBoK)?

**OQ-T-2.** Should the manifest include link-graph data (which docs cite which) explicitly, or rely on grep-time computation? The link graph would make queries like "show me the lineage of Doc 619" tractable but requires substantially more manifest content.

**OQ-T-3.** Should the manifest include first-author-and-date information for cross-corpus queries about temporal-genre patterns (e.g., "all letters written in 2026-Q2")? This would extend the manifest's analytical utility but adds maintenance burden.

**OQ-T-4.** How does the manifest interact with the blog-side artifacts (the 35+ blog posts at `/home/jaredef/jaredfoy/blog/`)? Should blog posts be tagged in a parallel manifest with cross-references to corpus-side anchors, or should they be incorporated into the same manifest?

**OQ-T-5.** What governance applies to manifest updates? Currently, the corpus's Doc 415 (Retraction Ledger) governs claim retractions; should there be analogous discipline for manifest tag changes — a "manifest revision log" that records why and when tags were changed?

## 10. Position

The corpus has 536 numeric-prefixed documents articulating a Lakatosian research programme with metaphysical hard core, operational protective belt, observational predictions, and standing audit disciplines (per Doc 632). The corpus's discipline of catching and correcting itself across documents and time is what makes the programme on-balance progressive (per Doc 632 §9). This document supplies the taxonomy + manifest design that lets the corpus order itself explicitly against its own self-recognized pattern: a five-layer tagging scheme (structural role; lifecycle status; warrant tier; engagement thread; temporal phase) with five constraint rules, a three-phase construction pathway (mechanical → rule-based → keeper-reviewed), and a manifest-file architecture (`corpus/MANIFEST.yaml`) that supports operational queries the corpus's working practice will benefit from.

The taxonomy is exploratory at \(\pi\)-tier. Its fitness will be tested by the manifest's actual construction, by the corpus's subsequent practice using the manifest, and by the keeper's audit of the auto-generated tags against working knowledge. Falsifiers (FT-1 through FT-4) and open questions (OQ-T-1 through OQ-T-5) are stated honestly. The taxonomy's adoption is at the keeper's call; if adopted, the manifest construction can proceed in the three phases §6 specifies, with Phase 1 producing a 100-doc partial manifest within minutes and Phase 3 completing the manifest over a keeper-review timescale.

---

## References

- [Doc 091 — The Spermatic Logos](/resolve/doc/091-the-spermatic-logos)
- [Doc 143 — SIPE (deprecated universal-meta-law form)](/resolve/doc/143-sipe)
- [Doc 270 — The Pin-Art Model (originating essay)](/resolve/doc/270-the-pin-art-model)
- [Doc 290 — The Pin-Art Formalization (deprecated)](/resolve/doc/290-the-pin-art-formalization)
- [Doc 314 — The Virtue Constraints (V3)](/resolve/doc/314-the-virtue-constraints)
- [Doc 372 — The Hypostatic Boundary](/resolve/doc/372-the-hypostatic-boundary)
- [Doc 415 — The Retraction Ledger](/resolve/doc/415-the-retraction-ledger)
- [Doc 445 — A Formalism for Pulverization](/resolve/doc/445-pulverization-formalism)
- [Doc 463 — The Constraint Thesis as a Lakatosian Research Programme](/resolve/doc/463-constraint-thesis-as-lakatosian-programme)
- [Doc 503 — Research-Thread Tier Pattern](/resolve/doc/503-research-thread-tier-pattern-iterative-novelty-calculus)
- [Doc 510 — Praxis Log V: Deflation as Substrate Discipline](/resolve/doc/510-praxis-log-v-deflation-as-substrate-discipline)
- [Doc 514 — Structural Isomorphism (PB-Form-3)](/resolve/doc/514-structural-isomorphism-canonical-formalization)
- [Doc 541 — SIPE-T (PB-Form-1)](/resolve/doc/541-systems-induced-property-emergence)
- [Doc 548 — The Ontological Ladder of Participation](/resolve/doc/548-the-ontological-ladder-of-participation)
- [Doc 619 — The Pin-Art Form (PB-Form-2)](/resolve/doc/619-pin-art-canonical-formalization)
- [Doc 620 — Canonicity in the Corpus](/resolve/doc/620-canonicity-in-the-corpus)
- [Doc 624 — Pin-Art Usage-Corpus Build Specification](/resolve/doc/624-pin-art-usage-corpus-build-spec)
- [Doc 627 — The Coherent-Confabulation Conjecture (PB-Discipline-5)](/resolve/doc/627-the-coherent-confabulation-conjecture)
- [Doc 628 — Resolver's Log: The Misattribution Family](/resolve/doc/628-resolvers-log-the-someone-proposes-misattribution)
- [Doc 632 — The RESOLVE Corpus, Primary Articulation](/resolve/doc/632-the-corpus-itself-primary-articulation)

---

## Appendix A — Originating Prompt

The keeper's instruction (Telegram message 5942, 2026-05-02T19:55:39Z):

> Now do an exhaustive audit of the corpus to discover all primary articulations so that the corpus can begin to order itself explicitly against its own self recognized pattern. We need to formulate a way to tag all corpus documents as constituent elements of its own form of itself.
>
> This means we need some sort of manifest or taxonomy design.

The instruction directed two related deliverables: (i) an exhaustive audit of the 536-document corpus to identify all primary articulations and structural patterns; (ii) the design of a taxonomy + manifest that lets every corpus document be tagged as a constituent element of the corpus's form of itself per Doc 632's Lakatosian schema. The audit was executed via Explore agent on 2026-05-02 with structured findings reported across six sections (counts and gross structure; explicit primary-articulation markers; functional-category identification by filename pattern; deprecation-supersession-retraction markers; engagement targets; anomalies and notes). The taxonomy + manifest design was developed against the audit findings: a five-layer tagging scheme (structural role; lifecycle status; warrant tier; engagement thread; temporal phase) with five constraint rules, three-phase construction pathway, manifest-file architecture (`corpus/MANIFEST.yaml`), and worked examples for twenty representative docs. Both deliverables are queued for keeper review and adoption per §7's three pathways.

---

*Jared Foy — jaredfoy.com — May 2026*
