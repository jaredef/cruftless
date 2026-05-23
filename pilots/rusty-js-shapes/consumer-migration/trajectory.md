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

---

## CMig-EXT 1 — 2026-05-23 (Object::new_dictionary factory)

### Headline

First code round of the consumer-migration sub-workstream. Adds `Object::new_dictionary()` as the explicit-Dictionary Ordinary factory for container-role allocation sites (Map/Set storage, listener lists, forwarders). ~22 LOC (factory + doc comment) at `pilots/rusty-js-runtime/derived/src/value.rs:271+`.

In the pre-CMig-EXT 8 regime where `new_ordinary()` also returns shape: None, this factory is operationally identical to `new_ordinary`. The factory is forward-looking: it documents the dispatch intent of container-role allocations so the CMig-EXT 8 enrollment flip (which makes `new_ordinary` default to Shaped) leaves the container-role sites correctly Dictionary.

### Substrate delivered

- `pilots/rusty-js-runtime/derived/src/value.rs:271-294` — `Object::new_dictionary()` factory with doc comment explaining the forward-looking intent.

### Build + gate

- `cargo build --release --bin cruft -p cruftless`: clean.
- diff-prod: **42/42 PASS** unchanged.

### §XVI / Doc 734 categorization

Per Doc 730 §XVI: not applicable (no behavioral change in this regime).

Per Doc 734 §V: growth mechanism (a) tier-relocation recursion — the factory names the dispatch-intent distinction (container-role vs ordinary-role) that the parent Shape-EXT 4 didn't articulate. CMig-EXT 3's Family A migration consumes this factory.

### Open scope at CMig-EXT 1 close

1. **CMig-EXT 2** — Family C migration (~20 sites; P2 migrate-on-access). Independent; can land in parallel with EXT 3.
2. **CMig-EXT 3** — Family A migration (Map/Set storage; P3 migrate-on-construct using `new_dictionary`). Depends on this round.
3. **CMig-EXTs 4-9** per the migration call-graph.

### Cumulative status at CMig-EXT 1 close

LOC delta: 22. diff-prod 42/42 unchanged. The Dictionary-form explicit factory is available for CMig-EXT 3+ consumers.

---

*CMig-EXT 1 closes. CMig-EXT 2 (Family C, P2 migrate-on-access) begins next.*

---

## CMig-EXT 2 — 2026-05-23 (Family C: P2 migrate-on-access)

### Headline

Migrates ~28 direct `.properties.insert` / `.properties.shift_remove` sites to be shape-aware via a new `Object::dict_mut()` accessor that forces `migrate_to_dictionary()` before exposing the IndexMap. Mechanical sed-style edit; one line per site changed (`properties.insert` → `dict_mut().insert`).

In pre-CMig-EXT 8 regime: no-op (every receiver is already Dictionary). Post-CMig-EXT 8: load-bearing — ensures accessor / non-default-descriptor installs land in Dictionary form even when the receiver started Shaped.

### Substrate landed

- `pilots/rusty-js-runtime/derived/src/value.rs` — `Object::dict_mut(&mut self) -> &mut IndexMap<...>` accessor (~16 LOC) that calls `migrate_to_dictionary()` then returns a mutable view.
- 28 sites updated across `interp.rs` (12) + `intrinsics.rs` (6) + `iterator.rs` (4) + `prototype.rs` (3) + `regexp.rs` (2) + `promise.rs` (1).

### Build + gate

- `cargo build --release --bin cruft -p cruftless`: clean.
- diff-prod: **42/42 PASS** unchanged.

### §XVI / Doc 734 categorization

Per Doc 730 §XVI: not applicable (no behavioral change in current regime). Per Doc 734 §V: growth mechanism (a) tier-relocation — the `dict_mut` accessor names the dispatch-intent distinction (mutating-access-with-migration vs read-iteration) that the bare `.properties` field didn't.

### Open scope at CMig-EXT 2 close

CMig-EXT 3 (Family A Map/Set storage) ready to land.

---

## CMig-EXT 3 — 2026-05-23 (Family A: P3 migrate-on-construct)

### Headline

Swaps `Object::new_ordinary()` → `Object::new_dictionary()` at 8 container-role allocation sites in `intrinsics.rs`: AbortSignal listeners + forwarders, Map/Set/WeakMap/WeakSet internal storage, BroadcastChannel + structuredClone bags. These objects are dictionaries by role (consumers iterate `.properties` directly); allocating them via `new_dictionary` documents the intent and guarantees they remain Dictionary-form regardless of the post-CMig-EXT 8 enrollment default.

### Substrate landed

- `pilots/rusty-js-runtime/derived/src/intrinsics.rs` — 8 `new_ordinary` → `new_dictionary` swaps at `:2377, 2505, 2640, 2767, 3201, 3346, 5358, 5390`.

### Build + gate

- `cargo build --release --bin cruft -p cruftless`: clean.
- diff-prod: **42/42 PASS** unchanged.

### §XVI / Doc 734 categorization

Same as CMig-EXT 2.

### Open scope at CMig-EXT 3 close

CMig-EXT 4 (Family B enumeration helpers) is next. Read paths that walk `o.properties.iter()` / `.keys()` need to chain shape iteration first so post-EXT 8 enrollment correctly enumerates shape-stored entries.

---

*Three CMig-EXTs landed in one session. Diff-prod 42/42 PASS held at each. The substrate is staged for CMig-EXT 4-7's read-path migrations + CMig-EXT 8's enrollment flip.*
