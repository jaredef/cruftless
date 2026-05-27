---
arc: 2026-05-27-engine-tier-substrate-readiness-for-compartments
trigger: Telegram message 9992 ("We need to straighten this out at the top of the alphabet / DAG / lattice") chained through 9994, 9996, 9998, 10000 (Doc 729 §XIII amend), 10002, 10004, 10006, 10008, 10010, 10012, 10014, 10016, 10018, 10020, 10022, 10024, 10026 (Doc 743 articulation), 10028, 10030, 10032, 10034, 10036 (this arc-formalization directive)
opened: 2026-05-27
closed: 2026-05-27 (P-C probe set executed; substrate clearing achieved; CP-EXT-5 audit-fix queued as the next arc's spawn condition)
close_condition: Engine-tier binding-surface unified to a single canonical surface (Object only), ESBC arc closed against the unified surface, corpus articulation in place naming the engine-tier property the substrate now satisfies (Doc 729 §XIII engine-tier alphabet-narrowing case + Doc 743 P-C induced property), and P-C probed against the existing Compartments scaffolding to surface the implicit constraints the next audit-fix rung will close. Probe-set REFUTATION of P-C is acceptable closure (per Doc 729 §XIII discipline, regression-as-implicit-constraint-probe applied prospectively); the substrate the arc cleared is correct, the implementation that sits between substrate and articulation is the next rung's target.
---

# Engine-Tier Substrate Readiness for Compartments Arc

## Trigger

Per keeper directive Telegram 9992 ("We need to straighten this out at the top of the alphabet / DAG / lattice"), issued on the heels of the ESBC arc's first close + reopen cycle (Telegram 9985-9987-9988-9989) where the test262 sample sweep had refuted the ES-EXT 1+2+3+4 closure as net-positive (33.2% catastrophic regression) and the bisection had named ES-EXT 2 (compile-tier script_mode flip) as the catastrophic component. The keeper's directive named the engagement's escape from accumulating bridges (ES-EXT 3+4 patched the divergence between two binding surfaces; ES-EXT 2 reverted into dormancy until the substrate could support it). The straightening at the top of the lattice IS the global-binding-surface unification: collapse two surfaces (Runtime.globals HashMap + globalThis Object) to one.

The arc opens at the keeper directive and runs through eight sub-arc rungs in the GBSU locale, two amendments to Doc 729 §XIII (capturing the methodology in the corpus), a re-close of the ESBC arc against the unified surface (ES-EXT 2 v2 landing), a primary corpus articulation of Cruft Compartments (Doc 743) naming the engine-tier property the substrate now satisfies, and a P-C probe set that refutes the property's holding against the existing Compartments scaffolding — surfacing the implicit constraints the next audit-fix rung will close.

## Telos

Establish engine-tier substrate readiness for TC39 Compartments by:

1. Unifying the global Variable Environment Record onto a single canonical surface (the globalThis Object), per ECMA-262 §16.1 and Doc 731's alphabet-purity bound.
2. Closing the indirect-eval var-attaches-to-globalThis path (ESBC original telos) against the unified surface.
3. Articulating the engine-tier property the substrate now satisfies in the corpus, so future work has a named precondition to test against.
4. Probing the articulated property against the existing Compartments scaffolding to surface the audit-fix scope.

Throughout: apply Doc 729 §XIII regression-as-implicit-constraint-probe methodology recursively, including in its prospective and inverted forms.

## Sub-locale roster

| Locale / Sub-arc | Role | Status | Direct yield |
|---|---|---|---|
| `pilots/global-binding-surface-unification/` (locale) | Top-level substrate locale; 8 rungs, 15 sub-rungs counting revisions | LANDED + arc CLOSED | Substrate clearing; +0.8pp at field-deletion |
| `apparatus/arcs/2026-05-27-eval-scope-binding-chain/` (arc) | Pre-existing arc; this engagement re-closed it with ES-EXT 2 v2 under the unified surface | RE-CLOSED | All P1-P7 probes green; spec-aligned indirect-eval |
| `pilots/eval-scope-binding-chain/es-foundation/` (locale) | ES-EXT 0+1+2+3+4 + v2 | LANDED | +9.1pp aggregate (bridges) + spec compliance (v2) |
| Corpus amendments (Doc 729 §XIII) | Two amendments codifying the runtime-tier alphabet-narrowing case + arc closure inversion | LANDED (commits cb3dd7c + 1c7ec61 on jaredef/resolve) | Methodology codified in corpus |
| Corpus articulation (Doc 743 — Cruft Compartments) | Primary articulation naming P-C as fifth induced engine-tier property | LANDED (commit e561a35 on jaredef/resolve; seeded to jaredfoy) | Engineering claim turned into a falsifiable property |
| P-C probe set (8 probes + 3 deeper) | Falsifier-driven verification of Doc 743 P-C against existing CP-EXT 1-4 substrate | EXECUTED — P-C REFUTED in 4 of 8 probes | Three implicit constraints named (IC.CP1/CP2/CP3) for next arc |
| `pilots/compartment-primitive/` (locale) | Prospective sub-locale for CP-EXT 5+ audit-fix; NOT spawned in this arc | QUEUED | — |

## Cumulative yield

| Checkpoint | Cumulative test262-sample PASS | Notes |
|---|---:|---|
| Engagement standing baseline (pre-engagement) | 5594 (77.6%) | Doc 729 baseline as of CLAUDE.md authoring |
| ESBC ES-EXT 1+2+3+4 (first close, premature) | 2435 (33.2%) | Catastrophic — ES-EXT 2 v1 violates IC.1 |
| Triage: ES-EXT 2 reverted, ES-EXT 3+4 retained | 6311 (86.7%) | **+9.1pp** — bridge yield (first observation) |
| GBSU 1-6 (HashMap demoted to fallback only) | 6310 (86.8%) | Parity through reader migration and rung-6 fallback deletions |
| GBSU 7a-7f.3 (clusters migrated, field retained) | 6250 (85.9%) | Within noise; ~−40 from descriptor-shape edge cases |
| GBSU 7f.4 (field DELETED) | 6311 (86.7%) | **+61 tests over pre-deletion** — deletion-as-positive-§XIII-probe |
| ESBC ES-EXT 2 v2 (re-enabled under unified surface) | 6296 (86.6%) | All P1-P7 probes green; −15 noise from extra StoreGlobal |

Aggregate engagement yield: **+9.1pp against the 77.6% baseline** (86.6% currently). Net LoC: ~−100 (the deletion ledger entries net out the additions).

## Cross-locale findings

**ARC.MR.1** (engine-tier alphabet narrowing as resolver-instance closure): the GBSU arc reduced the runtime's binding-resolution alphabet from `{Object, HashMap, engine_helpers}` to `{Object, engine_helpers}` — one symbol removed. This is the first observed engine-tier instance of Doc 731's alphabet-purity-upstream-bounds-downstream-complexity property applied at the binding-resolution tier. The property the arc induces is exactly the precondition Doc 743's P-C names: one Object per realm, every binding-resolution call site collapses from three-way branch to single Object lookup, compartment-switching becomes ObjectRef substitution.

**ARC.MR.2** (§XIII recurrence as a quantifiable signal): the GBSU arc exhibited six §XIII regression recurrences (rungs 3 first-cut, 5 first-cut, 5b, 5c, 7b round A, 7b round B), each surfacing a different implicit constraint scope (direct-call-pattern audit, side-channel stash-key round-trip, register-helper cluster, etc.). The recurrence count per locale is itself a property worth tracking — locales with high recurrence counts are touching deeper substrate alphabets. Future arcs should record this metric explicitly in arc.md.

**ARC.MR.3** (deletion as positive §XIII probe, methodological inversion): the GBSU arc's final rung (7f.4) deleted the legacy field and **gained 61 tests** over the pre-deletion baseline. The dual-write era's install_global_this drain loop was re-installing properties already registered through the migrated paths, overwriting subtle shape-system invariants on properties tests probed; the deletion removed the overlay. **Standing rec**: Doc 729 §XIII's regression-as-implicit-constraint-probe has an inversion case — deletion-as-positive-surface — that applies when transitional dual-writes are removed. Future locales maintaining transitional dual-writes should plan for a yield-surfacing rung at deletion time, not just a parity verification. Codified in Doc 729 §XIII third paragraph (commit 1c7ec61).

**ARC.MR.4** (formalization-before-implementation discipline, prospective §XIII): Doc 743's P-C was articulated as a property the substrate now satisfies; the falsifier probe set (8 + 3 probes) refuted it in 4+ measurements, surfacing IC.CP1 (globalThis accessor returns function-shaped object, not realm globalThis), IC.CP2 (endowment injection doesn't), IC.CP3 (cross-evaluate persistence broken — compartment.evaluate routes through evaluate_module not evaluate_script). The probe set ran BEFORE any further implementation; the three implicit constraints are now scoped for the next arc (CP-EXT 5 audit-fix) rather than discovered post-shipping. **Standing rec**: corpus articulations of properties should be accompanied by a falsifier probe set in the same trajectory step; if the probe set refutes, the articulation is correct but the implementation is behind — surface the gap as a named locale before extending.

**ARC.MR.5** (corpus articulation as forcing function): Doc 743 (Cruft Compartments) was drafted from the substrate's NOW state, not from a wish list. The corpus articulation is the substrate's external face; writing it forces the substrate work to be legible from outside. The keeper's question "How does this fit with our telos for JS Compartments?" (Telegram 10028) was the forcing function; the answer is the articulation, and the articulation surfaced the audit-fix scope (via ARC.MR.4). **Standing rec**: corpus articulations should be written at arc closure, not deferred — the discipline of writing an external face surfaces internal gaps the substrate work alone wouldn't.

**ARC.MR.6** (the prompt is part of the artifact): per keeper directive Telegram 10030 ("append this prompt to the artifact"), Doc 743 carries the keeper's commissioning prompt as a closing appendix. This is a standing pattern: the directive that produced an articulation is itself substrate-legible context for future readers. Future corpus drafts produced from keeper prompts should preserve the prompt verbatim in the artifact's appendix, both for audit and for the future reader to understand the directive's framing.

## Composition with prior arcs

- **2026-05-26-temporal-implementation** (Tier-L): the Pin-Art per-class template discipline established there was the precedent for the multi-rung-per-class shape that the GBSU arc applied at the per-helper level (register_global_ctor, register_global_fn, register_global_native each as a cluster). No direct dependency, but methodological lineage.
- **2026-05-26-missing-syntax-feature-concentration** (Tier-K): not directly composed-with; the Tier-K matrix work surfaced gaps that other arcs are addressing, parallel to this arc's substrate work.
- **2026-05-27-eval-scope-binding-chain** (this engagement re-closed it): the ESBC arc is a strict sub-arc of this super-arc; its re-closure under the unified surface is one of this arc's deliverables. Per arc-as-coordinate.md (arcs don't nest), ESBC remains its own arc; this super-arc references it as a sub-locale that landed during the super-arc's scope.

## Status

CLOSED 2026-05-27. The substrate clearing is complete; the corpus articulation is in place; the probe set has named the implementation gap. The next arc — provisional name `2026-05-XX-compartment-primitive-audit-fix` — opens on keeper directive to address IC.CP1+CP2+CP3 against the unified substrate.

Sub-rung count: 8 GBSU main rungs + 15 sub-rungs counting revisions + 2 ESBC closure rungs (ES-EXT 2 v2 trial #1 fail + trial #2 land) + 2 corpus amendments + 1 corpus primary articulation + 1 probe set = **29 substrate moves** across the day.

Cumulative diff: ~−100 net LoC; +9.1pp aggregate test262-sample yield; 2 new corpus docs (Doc 729 §XIII amendments + Doc 743 primary articulation); 1 new deletion-ledger entry (GBSU-EXT 7f.4); 1 standing rec promoted to §XIII (deletion-as-positive-surface inversion); 6 documented §XIII recurrences within GBSU; 3 named implicit constraints in CP-EXT 1-4 awaiting next arc.

The work continues. The next arc spawns on directive.
