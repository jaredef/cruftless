# rusty-js-shapes/consumer-migration — Trajectory

Per-CMig-EXT (consumer-migration extension) log. Sub-workstream of `pilots/rusty-js-shapes/`. Reads seed.md first; this file records the family-by-family migration of direct-`.properties` consumer sites to be shape-aware.

Format: one section per CMig-EXT. Same shape as `pilots/rusty-js-shapes/trajectory.md` and other Pin-Art trajectory.md files in the engagement.

---

## CMig-EXT 0 — 2026-05-23 (workstream founding + survey)

### Headline

Sub-workstream founded per Doc 737 §IV's coordinate-uniqueness invariant. The keeper's "Continue + set up seeds at every fractal locale that requires it" directive (2026-05-23) made the spawn explicit: the consumer-migration substrate has multi-rung shape (one rung per consumer family × 5 families + an enrollment-flip rung + a measurement rung), so it earns its own coordinate per Doc 737 §II promotion threshold.

Survey output: `docs/consumer-site-survey.md` catalogs **81 direct `.properties` access sites** across the runtime crate, classified into six families:

- **Family A** (Map/Set internal storage iteration): 9 read sites, 4 alloc sites. Migration pattern P3 (migrate-on-construct via CMig-EXT 1's `new_dictionary` factory).
- **Family B** (enumeration helpers — Object.keys/values/entries/JSON/for-in): 7 sites needing P1 (shape-iterate then properties-iterate). `ordinary_own_enumerable_string_keys` is already shape-aware per Shape-EXT 4.
- **Family C** (direct accessor / non-default-descriptor installs): ~20 sites needing P2 (migrate-on-access).
- **Family D** (descriptor introspection — Object.getOwnPropertyDescriptor / freeze / isFrozen / isSealed): 4 mutating sites (P2), 2 read sites (synthesize default descriptor for shape entries).
- **Family E** (module namespace enumeration): zero migration needed; ModuleNamespace bypasses Shape per carve-out.
- **Family F** (residual direct-array-index sites): minimal; review only.

### Substrate delivered

- `pilots/rusty-js-shapes/consumer-migration/seed.md` (~120 lines) — telos, apparatus, three migration patterns (P1/P2/P3), methodology with CMig-EXT 0-9 staging, carve-outs, composition with parent.
- `pilots/rusty-js-shapes/consumer-migration/trajectory.md` (this file).
- `pilots/rusty-js-shapes/consumer-migration/docs/consumer-site-survey.md` (~150 lines) — full inventory with file × site × family × migration-choice classification, migration call-graph, risk register.

### Locale registration

Per Doc 737 §IV: nested locale at coordinate `pilots/rusty-js-shapes/consumer-migration/` (two-segment scope). Parent reference: `L.rusty-js-shapes` per seed §VII. The seed names the parent in composes-with; the parent trajectory's Shape-EXT 4 open-scope item 1 closes when this nested workstream reaches CMig-EXT 9.

Locale tree after CMig-EXT 0:
```
pilots/
  rusty-js-shapes/
    consumer-migration/    (this, founded CMig-EXT 0)
  rusty-js-jit/            (LeJIT)
  rusty-js-runtime/
  ... (etc.)
```

Thirteen top-level + one nested = 14 active Pin-Art locales in the engagement.

### Migration call-graph (from survey §"CMig ordering")

```
CMig-EXT 1  →  Object::new_dictionary() factory
CMig-EXT 2  →  Family C (~20 sites)            [P2, independent]
CMig-EXT 3  →  Family A (Map/Set storage)      [P3, depends on EXT 1]
CMig-EXT 4  →  Family B (enumeration)          [P1, independent]
CMig-EXT 5  →  Family D (introspection)        [hybrid, independent]
CMig-EXT 6  →  (Family E reviewed; zero changes)
CMig-EXT 7  →  Family F residual review
CMig-EXT 8  →  Enrollment flip
CMig-EXT 9  →  Pred-shape.4 first integration measurement
```

### §XVI / Doc 734 categorization

Per Doc 730 §XVI: not applicable (no probe gated; CMig-EXT 0 is documentation-only).

Per Doc 734 §V: growth mechanism (a) tier-relocation recursion — the consumer-migration tier was implicit in Shape-EXT 4's deferred-enrollment finding; CMig-EXT 0 makes it an explicit nested-locale coordinate per Doc 737. Growth mechanism (c) positive-finding generalization at the apparatus tier — the keeper's "set up seeds at every fractal locale that requires it" directive crystallized the rule that "sub-workstreams with multi-rung shape get nested locales preemptively when their fan-out is known" (a sharpening of Doc 737 §VII's "pre-file generously, spawn when the substrate calls").

### Composition with prior corpus work

- **Doc 733 fractal seeds-and-trajectories** + **Doc 737 locale-as-coordinate**: this nested locale is the engagement's first prospective-spawn case (vs reactive-spawn after rung-row growth). The arktype precedent (Doc 737 §I worked example) was reactive; this is prospective per the keeper's directive. Pred-738.4 cross-tier convergence applies: commit-tag `Ω.5.P04.E0.cmig-*` + locale-path `pilots/rusty-js-shapes/consumer-migration/` + source-identifier coordinates compose without conflict.
- **Doc 729 §A8.13 substrate-amortization**: the parent's Shape-EXT 4 is the substrate-introduction round; this nested workstream is the consumer fanout. Per §A8.13's staging: substrate-first + closure-rounds-second. CMig-EXTs 1-7 are the closure-rounds-second drainage; CMig-EXT 8 + 9 close the enrollment loop.
- **Doc 735 §X.h three-probe-levels discipline**: each CMig-EXT 2+ gates on diff-prod 42/42 (bench probe → consumer-route probe combined; diff-prod fixtures ARE the consumer route for the shape substrate) + test262-sample 77.6% (consumer-route probe across a much wider surface). Fuzz probe activates at CMig-EXT 9 with property-addition-history fuzz.

### Open scope at CMig-EXT 0 close

1. **CMig-EXT 1** — `Object::new_dictionary()` factory. ~10 LOC. Test-only verification that the factory returns an Object with `shape: None`.
2. **CMig-EXTs 2-7** — family migrations per the call-graph.
3. **CMig-EXT 8** — Enrollment flip; the load-bearing round.
4. **CMig-EXT 9** — Pred-shape.4 integration measurement.

### Cumulative status at CMig-EXT 0 close

LOC delta: 0 (apparatus-tier round). docs/ artifacts: 1 (survey). Locale registered.

The consumer-site landscape is mapped. CMig-EXT 1 begins when keeper directs.

---

*CMig-EXT 0 closes. The nested locale exists; the survey grounds the closure-round plan in measured site counts; CMig-EXT 1 carries the first code work.*
