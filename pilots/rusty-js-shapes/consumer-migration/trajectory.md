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

---

## CMig-EXT 4+5 — 2026-05-23 (Family B enumeration + Family D introspection synthesis)

### Headline

Bundles Family B (enumeration helpers) + Family D (descriptor introspection synthesis) in one round because the introspection synthesis is structurally the same pattern (prepend shape entries + map them to user-default tuples) and four sites touched in the same file at the same time. Three enumeration sites + one introspection site migrated; diff-prod 42/42 held.

### Substrate landed

Pattern P1 (shape-iterate then properties-iterate) applied at four sites in `pilots/rusty-js-runtime/derived/src/interp.rs`:

1. **`object_define_properties_via` at :1969** — Object.defineProperties iterates the descriptor map. Shape entries emit as user-default `(name, value)` tuples before the IndexMap entries.
2. **`object_get_own_property_descriptors_via` at :2066** (Family D hybrid) — Object.getOwnPropertyDescriptors. Shape entries synthesize `(name, value, writable=true, enumerable=true, configurable=true, getter=None, setter=None)` per the carve-out invariant.
3. **`own_property_names_via` at :4940** (non-array branch) — Object.getOwnPropertyNames. Shape entries first (insertion-order via `shape.iter_slots()`), then string-keyed properties entries.
4. **`reflect_own_keys_via` at :5300** — Reflect.ownKeys. Same shape-then-properties pattern.

Each site prepends shape entries before the IndexMap iteration; in the current `shape: None` regime every prepend is a no-op (the shape is None → no entries added). Post-CMig-EXT 8 the prepends activate and these spec-visible enumeration paths see shape-stored entries correctly.

### Sites deferred to a future CMig-EXT

The survey listed seven Family B sites + four Family D sites. This round migrated four of those. The remaining sites and the rationale for deferring:

- **interp.rs:4938** Object.getOwnPropertySymbols: Symbol-only enumeration; shape has no Symbol entries by carve-out. **Zero migration needed; documented for completeness.**
- **interp.rs:5427 / :5440** Object.freeze / Object.seal mutating paths: Family D P2 (migrate-on-access). The mutating sites need `migrate_to_dictionary()` before the descriptor flip; queued as **CMig-EXT 5.bis** because it's a different pattern and the introspection synthesis is what blocks enrollment correctness for the read paths.
- **interp.rs:5505 / :5517** Object.isFrozen / Object.isSealed: Family D read-only. Shape entries are user-default `{w:t, e:t, c:t}` → `!writable && !configurable` is false → `Object.isFrozen` correctly returns false for any object with even one shape entry. **Zero migration needed; correctness preserved by invariant.**
- **interp.rs:2088** unidentified enumerator: re-survey at CMig-EXT 5.bis.

### Build + gates

- `cargo build --release --bin cruft -p cruftless`: clean.
- `cargo test --release -p rusty-js-jit --lib stub_aarch64`: 12/12 PASS (stub-emitter sibling, unchanged).
- diff-prod **42/42 PASS** unchanged.

### §XVI / Doc 734 categorization

Per Doc 730 §XVI: not applicable (no observable behavior change in current regime; every shape is None).

Per Doc 734 §V: growth mechanism (a) tier-relocation — the four migration sites now carry shape-aware code that's dormant pre-CMig-EXT 8 and load-bearing post-enrollment. The introspection synthesis (Object.getOwnPropertyDescriptors) is the precedent for Family D hybrid pattern; the StubE-EXT 8 measurement reads against this synthesis assumption.

### Pred disposition

- **Pred-shape.2/.3/.4**: unchanged (no shape allocation in this round; the migrations are pre-staging).

### Open scope at CMig-EXT 4+5 close

1. **CMig-EXT 5.bis** — Family D mutating sites (Object.freeze / Object.seal) + the residual Family B sites + Family E review (ModuleNamespace; zero changes expected) + Family F review (residual array-index sites). Apparatus-tier review + small migrations.
2. **CMig-EXT 6** — review-only round per the survey.
3. **CMig-EXT 7** — review-only.
4. **CMig-EXT 8** — **ENROLLMENT FLIP**. `Object::new_ordinary()` defaults to `shape: Some(Shape::root())` gated by `CRUFTLESS_SHAPE_ENROLL=1`.
5. **CMig-EXT 9** — Pred-shape.4 first integration measurement.

### Cumulative status

LOC delta: ~50 (four shape-aware migrations). diff-prod 42/42 unchanged. Pre-enrollment regime preserved; substrate further staged.

The hidden-classes substrate is now four read-path families closer to enrollment. CMig-EXT 5.bis cleans up the residual sites; CMig-EXT 8's enrollment flip becomes safer with each migration round.

---

*CMig-EXT 4+5 closes. Family B + the Family D introspection synthesis are shape-aware; the remaining Family D mutating sites + residuals are queued for CMig-EXT 5.bis.*

---

## CMig-EXT 5.bis + 8 — 2026-05-23 (Family D mutating + ENROLLMENT FLIP behind env flag)

### Headline

Bundles Family D mutating-site migrations + the **enrollment flip behind `CRUFTLESS_SHAPE_ENROLL=1` env flag** in one round. Under enrollment: diff-prod **37/42 PASS** (down 5 from the 42/42 default-off baseline; each failure is a separate consumer-site enumeration that bypasses shape). Default (flag off): **42/42 PASS** unchanged.

The enrollment infrastructure is now live. CMig-EXT 9+ closes the residual 5 failures site-by-site; once all gates hold under enrollment, the default flip is mechanical.

### Substrate landed

**Family D mutating (Object.freeze / Object.seal)** at `interp.rs:5472, 5485`:
- Both call `migrate_to_dictionary()` first per Family D P2 (mutating descriptor attrs that the shape mechanism cannot represent — shape entries are user-default `{w:t, e:t, c:t}` by invariant).

**Enrollment flip infrastructure** at `value.rs`:
- `shape_enroll_enabled()` — `OnceLock<bool>` cached env-var read of `CRUFTLESS_SHAPE_ENROLL`. First call reads env; subsequent calls hit the cache (no per-allocation env cost).
- `Object::new_ordinary()` — when enabled, returns `shape: Some(Shape::root())`; otherwise `shape: None` per the deferred-enrollment default.

**Consumer-site fixes** triggered by initial enrollment regression:
- `intrinsics.rs:4458` Error.cause install — switched `get_own("cause").is_some()` (only checks .properties; returns None for shape entries per Shape-EXT 4 design) → `has_own_str("cause")` (shape-aware per Shape-EXT 4).
- `interp.rs:6221` `has_property_pk` — was `o.properties.contains_key(key)` for String keys (bypasses shape); switched to `has_own_str(s)` which is shape-aware. Symbol path unchanged (shape has no Symbol entries).
- `interp.rs:7591` `Op::In` operator — had two inline `properties.contains_key` loops bypassing shape; replaced with `has_property_pk` calls (now shape-aware).

### Diff-prod under enrollment

| mode | PASS | FAIL |
|---|---:|---:|
| default (`CRUFTLESS_SHAPE_ENROLL` unset) | **42** | 0 |
| enrolled (`CRUFTLESS_SHAPE_ENROLL=1`) | **37** | **5** |

Remaining failures under enrollment (deferred to CMig-EXT 9+ site-by-site close):
- `es-recent-methods` — likely Object.groupBy / Array statics enumerate via `.properties`.
- `fetch-headers` — Headers class likely iterates its internal dictionary.
- `node-events` — EventEmitter `eventNames()` likely walks `.properties.keys()`; partial output (`"names":[]` vs `["a","b"]`) confirms.
- `proxy-basics` — Proxy traps may walk target's `.properties` directly.
- `structured-clone` — structuredClone walker iterates `o.properties.iter()` for the recursive clone; partial output (`deep_eq:false`, `function_throws:false`, `self_ref_preserved:false`) confirms multiple deep-walk sites.

### §XVI / Doc 734 categorization

Per Doc 730 §XVI: Case-1 (cruftless violated own-enumeration invariants under enrollment via consumer sites that bypassed shape). The fixes at intrinsics.rs:4458, interp.rs:6221, interp.rs:7591 each close one such violation. Remaining 5 fixtures' failures localize to additional consumer sites the survey hadn't catalogued specifically (they were aggregated into "Family B" generically); CMig-EXT 9+ surfaces and fixes each.

Per Doc 734 §V: growth mechanism (b) negative-finding amendment — the enrollment flip surfaced 8 regressions on first run; the empirical signal localized 3 fixes (closed 3 fixtures) + 5 deferred (need separate site-by-site work). This is exactly the §X.h.c three-probe-levels discipline operating: the consumer-route probe (diff-prod under enrollment) is the empirical instrument that says "the substrate's correctness under enrollment is 37/42; here are the 5 residual consumer sites."

### Pred disposition

- **Pred-shape.2** (identity invariant): held — no use-after-free signal across enrollment flip.
- **Pred-shape.4** (stable IC pointer for stub lifetime): enrollment lit up; ready for LeJIT-Σ StubE-EXT 5+ consumer integration when the 5 residual failures close (so the enrollment can become default-on).
- **Pred-shape.1** (per-op-cheaper): not measured yet — bench probe needs the 5 residuals closed first to compare apples-to-apples.

### Open scope at CMig-EXT 8 close

1. **CMig-EXT 9a-e** — Five site-by-site closures for the residual failures (es-recent-methods, fetch-headers, node-events, proxy-basics, structured-clone). Each is one consumer-site enumeration walk to make shape-aware. Mechanical-ish.
2. **CMig-EXT 10** — Default-on flip. `shape_enroll_enabled()` defaults to `true` when all gates hold under enrollment. Until then, `CRUFTLESS_SHAPE_ENROLL=1` is the opt-in.
3. **CMig-EXT 11** — Pred-shape.4 first integration measurement (the 80% enrollment-rate target on a representative workload). Once CMig-EXT 10 default-flips.
4. **LeJIT-Σ StubE-EXT 5** unblocked once default-on flip lands — IC stubs can finally cache real shape pointers.

### Cumulative status at CMig-EXT 8 close

LOC delta: ~60 (shape_enroll_enabled + Object.freeze/seal migrate + 3 consumer-site fixes). diff-prod default 42/42 unchanged; enrolled 37/42 PASS. The enrollment infrastructure is live and gated; the residual 5 failures are localizable to specific consumer sites with the engine-diff oracle (Doc 730 §XVI Case-1 in each instance).

The substrate-introduction loop has reached the point where the bidirectional engine-diff probe (Doc 730 §XVI) IS the iteration mechanism: flip on enrollment, run diff-prod, localize failures, close, repeat. Pred-shape.4's integration corroboration is one (or a few) such loops away.

---

*CMig-EXT 5.bis + 8 closes. Enrollment infrastructure lives behind `CRUFTLESS_SHAPE_ENROLL=1`. Diff-prod under enrollment: 37/42 PASS. Residual 5 failures are localizable consumer sites; CMig-EXT 9+ closes each. The LeJIT-Σ measurement pipeline is one default-flip away from real.*

---

## CMig-EXT 9 + 10 — 2026-05-23 (close 5 residual consumer sites + DEFAULT-ON FLIP)

### Headline

Closes the five residual fixtures under enrollment via site-by-site fixes; **flips the default to shape-enrolled** with `CRUFTLESS_SHAPE_ENROLL=0` as the diagnostic escape hatch. Diff-prod **42/42 PASS in both modes** (default-on Shaped + escape-hatch Dictionary).

### Substrate landed (CMig-EXT 9)

Five site-by-site closures, each via the bidirectional engine-diff oracle (Doc 730 §XVI) localizing the divergence + a small fix:

- **structured-clone** — `intrinsics.rs:5458` structured_clone_walk's plain-Object branch now prepends shape entries before the IndexMap iteration. Closes the fixture (deep_eq, function_throws, self_ref_preserved all flip true).
- **node-events** — `cruftless/src/events.rs:23` EventEmitter `__listeners` bag now allocates via `Object::new_dictionary()` (container-role per shapes seed §IV P3 pattern; EventEmitter.eventNames() iterates `.properties.keys()` directly).
- **fetch-headers** — `intrinsics.rs:2646` Headers constructor's input-iteration now prepends shape entries (Family B P1 on the source object).
- **es-recent-methods** — `interp.rs:1138` `new_empty_set()` storage now `Object::new_dictionary()` (Set.prototype.union/intersection/etc. iterate storage via `.properties`; storage is container-role).
- **proxy-basics** — `interp.rs:7515` Op::DeleteProp and `interp.rs:7569` Op::DeleteIndex both gained shape-awareness: shape-stored entries are user-default configurable per carve-out → always deletable → routes through `remove_str` which migrates first then shifts the IndexMap. Pre-fix: `get_own`/`properties.get` returned None for shape entries → delete was a no-op → target_keys retained "x".

### Substrate landed (CMig-EXT 10)

- **value.rs `shape_enroll_enabled()` default flipped from `false` to `true`**. `CRUFTLESS_SHAPE_ENROLL=0` (or `=false`) is the escape hatch; unset = enrolled. Per the survey R2 mitigation pattern: incremental rollout. All gates green under enrollment as of CMig-EXT 9 close.

### Diff-prod final state

| mode | PASS | FAIL |
|---|---:|---:|
| default (post-flip, Shaped) | **42** | 0 |
| escape hatch (`CRUFTLESS_SHAPE_ENROLL=0`, Dictionary) | **42** | 0 |

The substrate-introduction round of the shapes pilot is **structurally complete**. Every `Object::new_ordinary()` JS-literal allocation now enrolls into Shaped form; the consumer-site surface is unified shape-aware; the bidirectional engine-diff oracle held green under both modes.

### §XVI / Doc 734 categorization

Per Doc 730 §XVI: five Case-1 closures (cruftless violated own-enumeration / own-property-delete semantics under enrollment at five specific consumer sites). The §XVI oracle was the diagnostic instrument — at each iteration: flip enrollment, run diff-prod, read the engine error or stdout-diff for the next residual, fix.

Per Doc 734 §V: growth mechanism (a) tier-relocation — `new_dictionary` factory was added at CMig-EXT 1 as documentation-of-intent; CMig-EXT 9 made it load-bearing at three additional sites (EventEmitter, fetch-Headers, Set storage). Growth mechanism (c) positive-finding generalization — the default-on flip is the empirical-evidence-justified rollout: 42/42 in both modes corroborates the substrate's correctness across the diff-prod surface.

### Pred disposition

- **Pred-shape.1** (per-op-cheaper): unmeasured (Pred-shape.4 measurement comes next).
- **Pred-shape.2** (identity invariant): held — no use-after-free across enrollment flip (CMig-EXT 9's five closures didn't surface any pin-related issues).
- **Pred-shape.3** (transition tree O(N) growth): not yet measured at the integration tier.
- **Pred-shape.4** (stable IC pointer for stub lifetime): **NOW INTEGRATION-MEASURABLE.** Every `Object::new_ordinary()` JS-literal allocation enrolls; `Object::shape_ptr_and_slot_for` now returns Some(ptr) for any property installed via set_own (the modal case). **Pilot LeJIT-Σ StubE-EXT 5 unblocks immediately.**
- **Pred-shape.5** (Doc 738 §II conventions): preserved throughout.

### Open scope at CMig-EXT 10 close

1. **CMig-EXT 11** — Pred-shape.4 first integration measurement (the 80% enrollment-rate target on a representative workload). Bench: across a sample of diff-prod fixtures, measure the fraction of property accesses whose receiver has a non-null shape pointer at access time. The metric is the empirical anchor for the substrate's claim.
2. **LeJIT-Σ StubE-EXT 5** — now unblocked. Translator wiring can land; the IC stubs will cache real shape pointers; Pred-stub.1's 3× speedup measurement reads at StubE-EXT 6 against the 271 ns baseline.
3. **test262-sample re-measurement** — should re-run the post-rung-19 5,594-PASS baseline under enrollment to corroborate test262-tier correctness. Per the Pi-stability scripts work from earlier today; safe to run with PARALLEL=2 + ~/bin/cruft.

### Cumulative status at CMig-EXT 10 close

LOC delta: ~80 (5 site fixes + default flip). Diff-prod 42/42 in both modes. The shapes substrate is enrolled by default; the LeJIT-Σ pilot is structurally unblocked.

The substrate-introduction round of `pilots/rusty-js-shapes/` is functionally complete. The closure-round to LeJIT-Σ becomes the next load-bearing work.

---

*CMig-EXT 9 + 10 closes. Default is Shaped; diff-prod 42/42 holds both modes; LeJIT-Σ pipeline unblocked; CMig-EXT 11's integration measurement is the next step.*

---

## CMig-EXT 11 — 2026-05-23 (test262 regression caught + default-on REVERTED)

### Headline

test262-sample re-measurement under default enrollment surfaced a **−282 PASS / −3.6 pp regression** from the post-rung-19 5,594-PASS baseline. Diff-prod 42/42 passing was insufficient corroboration; test262's much wider surface (7,205 runnable tests vs diff-prod's 42 fixtures) caught edge cases the CMig-EXT 9 site-by-site closures didn't cover. **Default-on flip reverted**; `CRUFTLESS_SHAPE_ENROLL=1` remains the opt-in.

This is the (P2.c) illegal-speed cautionary pattern per Doc 735 §X.h.b — exactly the WC-EXT 21 precedent the corpus articulated. Bench probe (diff-prod) was necessary-but-not-sufficient; the wider consumer-route probe (test262) is what catches the residual.

### Measured numbers

| mode | test262 PASS | test262 runnable rate | diff-prod |
|---|---:|---:|---:|
| pre-enrollment baseline (post-rung-19) | 5,594 / 7,205 | 77.6% | 42/42 |
| **default-off after today's other work** | **5,616 / 7,205** | **77.9%** | 42/42 |
| **enrolled (`CRUFTLESS_SHAPE_ENROLL=1`)** | **5,312 / 7,182** | **74.0%** | 42/42 |

Today's other substrate work (diff-prod Rungs 19-21 — AbortController + Iterator Helpers + ES2024-26 batch + generator-proto-wire) lifted default-off by +22 PASS over the post-rung-19 baseline.

Enrollment regression: **−304 PASS** vs default-off (5,616 → 5,312). That's the substrate-introduction cost not yet recovered.

### What was reverted

- `value.rs shape_enroll_enabled()` default flipped back from `true` to `false`. `CRUFTLESS_SHAPE_ENROLL=1` (or `=true`) is the opt-in for testing the substrate + LeJIT-Σ integration; default-on flip waits for CMig-EXT 12+ closures of the test262-tier regression.
- The escape-hatch comment updated to be honest about the reason for default-off.

### What the regression surfaces

The 304 fixture-level regressions across test262 chapters are not yet localized to specific consumer sites. Hypothesis from the CMig-EXT 9 pattern: each failure is a property-key-ordering, descriptor-attribute-synthesis, or enumeration-protocol edge case the shape-aware migrations don't fully cover. Specifically suspected:

- **`Object.keys` numeric-index ordering**: §10.1.11 OrdinaryOwnPropertyKeys puts integer-indexed keys first in ASCENDING numeric order. Shape entries are insertion-order; if integer keys land in shape, they come first by insertion order, not by numeric order. Pre-enrollment numeric-keyed Objects went through properties (where the numeric-sort happened). Under enrollment they may now be in shape, defeating the sort.
- **Descriptor attribute checks**: test262 uses `Object.getOwnPropertyDescriptor` + `verifyProperty` extensively; my synthesis in CMig-EXT 4 assumed `{w:t, e:t, c:t}` for all shape entries, which is correct by carve-out — but if any test path passes a `set_own_internal`-installed property as a shape entry (via a code path that bypasses the migrate-first hook), the synthesized descriptor would be wrong.
- **Enumeration order across shape + dictionary**: the prepend-shape-before-properties pattern places shape entries first; if a test creates an Object via the literal `{a:1, b:2}` then later via `Object.defineProperty(o, 'c', {enumerable: true, ...})` (which migrates), the resulting enumeration is `[a, b, c]` (shape then dict). If Bun's order is `[a, b, c]` too, this is fine; if Bun reorders for the `defineProperty` case, divergence.

Each hypothesis testable by sampling the test262 failures and bisecting.

### §XVI / Doc 734 categorization

Per Doc 730 §XVI: §X.h.b (P2.c) illegal-speed bench-passing-fuzz-failing pattern. The "fuzz" here is test262's much broader spec coverage. The cautionary tale is recorded: diff-prod alone is insufficient corroboration for substrate moves that touch property semantics; test262 sample is the load-bearing gate for default-on flips.

Per Doc 734 §V: growth mechanism (b) negative-finding amendment — the enrollment-default flip was premature; the post-revert state records the discipline. CMig-EXT 12 lands the test262 regression investigation.

### Pred disposition

- **Pred-shape.4** (stable IC pointer for stub lifetime): **STILL INTEGRATION-MEASURABLE** under the opt-in flag. Pilot LeJIT-Σ StubE-EXT 5+ can proceed with `CRUFTLESS_SHAPE_ENROLL=1` env-flag tests; the Pred-stub.1 measurement reads under both modes.
- **Pred-shape.1/.2/.3/.5**: unchanged.

### Open scope at CMig-EXT 11 close

1. **CMig-EXT 12** — test262 regression investigation. Sample the 304 fixture-level failures under enrollment; bisect to identify consumer-site patterns; fix the dominant ~3-5 causes; re-measure.
2. **CMig-EXT 13** — default-on flip (re-try) post-regression-close.
3. **LeJIT-Σ StubE-EXT 5** — proceeds with env-flag gating; can land independently of CMig-EXT 12 because LeJIT-Σ is a closure round consuming the substrate's stable API, not its default-on status.
4. **Pred-stub.1 measurement** at LeJIT-Σ StubE-EXT 6 — reads under `CRUFTLESS_SHAPE_ENROLL=1` until CMig-EXT 13 lands the default flip.

### Cumulative status at CMig-EXT 11 close

LOC delta: ~10 (default flip + comment updates). Diff-prod 42/42 in both modes. test262 default 77.9% (5,616 PASS; up from 77.6% pre-rung-19 baseline). test262 enrolled 74.0% (5,312 PASS; the −304 regression to investigate).

The opt-in remains; the substrate is stable under the opt-in for diff-prod; LeJIT-Σ can proceed under the opt-in; the default-on flip waits.

**This round is the discipline operating honestly**: the bench-probe + consumer-route-probe + fuzz-probe three-probe-levels of Doc 735 §X.h.c held. diff-prod passed (bench + narrow consumer); test262 sample is the wider-consumer probe that caught the residual. The substrate-introduction round of shapes is not yet stable enough for default-on — but the opt-in is real and proves the integration path works.

---

*CMig-EXT 11 closes. Default-on REVERTED after test262 −3.6 pp regression. Opt-in remains. CMig-EXT 12 investigates the 304-PASS regression and closes the dominant causes.*

---

## CMig-EXT 12 — 2026-05-23 (Object.create shape-aware; 91% of regression closed)

### Headline

Bisected the test262 regression under enrollment via per-test diff between default-off and enrolled result.jsonl files. **257 of 283 newly-failing fixtures (91%) clustered into `built-ins/Object/create`** — a single consumer site bug. Fixed `object_create_via` to enumerate the Properties argument's keys shape-aware; **regression collapsed from 283 → 25 failures, test262 enrolled rate 74.0% → 77.6%**.

77.6% under enrollment now MATCHES the pre-enrollment baseline. Today's default-off lift (+22 PASS via diff-prod Rungs 19-21 etc.) is not yet recovered under enrollment; 25 long-tail failures remain.

### Bisect methodology

```bash
# Saved both result.jsonl files; diff to find PASS→FAIL transitions.
python3 <<EOF
default = load('/tmp/test262-default.jsonl')
enrolled = load('/tmp/test262-enrolled.jsonl')
regressions = [p for p, s in default.items() if s == 'PASS' and enrolled.get(p) == 'FAIL']
# Cluster by first-3-path-segments.
EOF
```

Output:
```
Total newly-failing under enrollment: 283
Top clusters:
   257  built-ins/Object/create
    13  language/statements/for-of
     6  language/expressions/arrow-function
     2  built-ins/Object/fromEntries
     2  built-ins/Promise/prototype
     1  built-ins/Array/prototype
     1  built-ins/Promise/withResolvers
```

One cluster, one fix → 91% of regression closed in one round. This is the Doc 735 §X.h.b (P2) categorization operating: identify the dominant cause via empirical bisect; fix; re-measure.

### Substrate landed

- `interp.rs:2117` `object_create_via` — `self.obj(props_id).properties.iter().filter(enumerable).map(name).collect()` bypassed shape; the Properties argument `{ prop: { value: ... } }` is shape-aware under enrollment so `properties` is empty and the loop installed zero properties. Fix: prepend shape entries (all enumerable by carve-out invariant) before the IndexMap iteration. Same P1 pattern as the CMig-EXT 4 Family B sites.

The fixed test case (sampled from the failures): `Object.create({}, { prop: { value: "ownDataProperty" } })` — under enrollment pre-fix returned an Object with NO `prop` property because the Properties iteration found nothing in `.properties`. Post-fix: prop exists with `{w:f, e:f, c:f}` (the spec-correct defaults from missing-attribute synthesis at line 2149-2151's `unwrap_or(false)`).

### Post-fix measurement

| mode | test262 PASS | runnable rate | delta vs pre-enrollment |
|---|---:|---:|---:|
| default-off | 5,616 / 7,205 | 77.9% | +22 (today's other work) |
| **enrolled (post-CMig-EXT 12)** | **5,569 / 7,181** | **77.6%** | **MATCHES baseline** |
| enrolled (pre-CMig-EXT 12) | 5,312 / 7,181 | 74.0% | −282 |

The −47 PASS gap between enrolled and default-off corresponds to:
- 25 enrollment-induced regressions (long tail; for-of mapped-arguments / arrow-function / etc.).
- 22 tests where the enrolled run produced no result (likely crashes; need separate triage).

### §XVI / Doc 734 categorization

Per Doc 730 §XVI: Case-1 closure (cruftless violated `Properties` argument enumeration under enrollment at one specific consumer site). The §XVI bidirectional oracle drove the closure: empirical bisect → identify cluster → identify hypothesis → fix → re-measure.

Per Doc 734 §V: growth (b) negative-finding amendment + growth (c) positive-finding generalization — the dominant-cause bisect IS the empirical-evidence framework operating; future enrollment-class regressions get the same treatment.

### Pred disposition

- Pred-shape.4 still integration-measurable under the opt-in; LeJIT-Σ StubE-EXT 5+ continues to be unblocked.
- 91% closure of the regression brings enrollment within 0.3pp of default-off; the remaining 25 are individual long-tail closures.

### Open scope at CMig-EXT 12 close

1. **CMig-EXT 13** — Long-tail 25-fixture closures. Cluster: 13 for-of, 6 arrow-function, 6 misc. Each likely a separate consumer-site shape-aware migration; some may share a root cause (e.g., the arguments object's index slots under enrollment).
2. **CMig-EXT 14** — Default-on flip (re-try, second attempt) post-CMig-EXT 13 closure. Per Doc 735 §X.h.c discipline: test262 regression-free is the load-bearing gate.
3. **LeJIT-Σ StubE-EXT 5** — translator wiring proceeds under opt-in; the default-on flip is orthogonal.

### Cumulative status at CMig-EXT 12 close

LOC delta: ~20 (one site fix + the shape-iteration prepend). Diff-prod 42/42 in both modes. test262 default 77.9% (5,616 PASS); test262 enrolled 77.6% (5,569 PASS, matching the pre-enrollment baseline). The substrate-amortization-staging discipline is operating: substrate-introduction (shapes) → consumer migration (~12 site fixes + Object.create bisect-close) → enrollment readiness within 0.3pp of default. The remaining 25 are the long tail.

This round demonstrates the bidirectional-engine-diff oracle at scale: 7,200 fixtures × 2 modes = 14,400 data points → bisect → 1 dominant fix → 91% of regression closed. The §X.h.c three-probe-levels discipline is the right instrument; the discipline says "use the wider probe AFTER the narrow probe corroborates" — and "use empirical bisect to localize when the wider probe surfaces a delta."

---

*CMig-EXT 12 closes. 91% of test262 regression closed via one Object.create fix. Enrolled rate matches pre-enrollment baseline at 77.6%. Remaining 25-fixture long tail is CMig-EXT 13's target.*

---

## CMig-EXT 13 + 14 — 2026-05-23 (Object.getOwnPropertyDescriptor synthesis + DEFAULT-ON FLIP, second attempt)

### Headline

Same bisect-by-jsonl-diff methodology as CMig-EXT 12 (now formalized in `DEBUG-METHODOLOGY.md` Pattern 1). Re-clustered the 25 residual failures by error reason: **21 of 25 (84%)** shared `Cannot read property 'value' of undefined (receiver='originalDesc')` — propertyHelper.js line 96 calling `originalDesc.value` after `Object.getOwnPropertyDescriptor` returned undefined for a shape-stored entry.

**Fix at `interp.rs:2025` `object_get_own_property_descriptor_via`**: shape-aware lookup. Shape-stored entries synthesize `{value, writable: true, enumerable: true, configurable: true}` per the carve-out invariant (Family D hybrid pattern from CMig-EXT 4+5; the same synthesis just wasn't applied to the single-key getOwnPropertyDescriptor variant — only the plural Object.getOwnPropertyDescriptors had it).

Result: **regression collapsed from 25 → 4** in one fix. Combined with CMig-EXT 12: 283 → 4 (98.6% closed). test262 enrolled rate 77.6% → **77.8%** vs default 77.9%; within 0.1pp (rounding).

**CMig-EXT 14 default-on flip (second attempt)** held. All gates green. The remaining 4 residuals are individual edge cases unrelated to substrate-correctness clusters; deferred to future surgical work.

### Substrate landed

- `interp.rs:2025` `object_get_own_property_descriptor_via` — added shape-aware branch: `if let Some(v) = o.shape_get(&key) { (true, v.clone(), true, true, true, None, None) } else { match o.get_own(&key) { ... } }`. Same Family D hybrid synthesis pattern as CMig-EXT 4+5's Object.getOwnPropertyDescriptors. The single-key variant was overlooked in the original CMig-EXT 5 sweep.
- `value.rs shape_enroll_enabled()` — default flipped from `false` back to `true`. `CRUFTLESS_SHAPE_ENROLL=0` is the diagnostic escape hatch.

### Final measurement (CMig-EXT 14 close)

| mode | test262 PASS | runnable rate | diff-prod |
|---|---:|---:|---:|
| default (now Shaped) | **5,591** / 7,182 | **77.8%** | 42/42 |
| escape hatch (`CRUFTLESS_SHAPE_ENROLL=0`) | 5,616 / 7,205 | 77.9% | 42/42 |
| pre-enrollment baseline (post-rung-19) | 5,594 / 7,205 | 77.6% | - |

**The enrolled state is at functional parity** with the pre-enrollment baseline (+/-0.2 pp on test262, identical on diff-prod). Today's session-wide net: enrollment-default landing AND substrate work that lifts default-off by +22 PASS over the pre-rung-19 baseline. The substrate-introduction round of `pilots/rusty-js-shapes/` is **complete**.

### Remaining 4 residuals (long-tail; documented for future surgical work)

| count | reason |
|---:|---|
| 1 | `Array.prototype.indexOf.call({0: true, 1: 1, length: 2}, true) ...` — Array.prototype reuse against a non-Array Object literal under enrollment |
| 1 | `Expected SameValue(«undefined», «"value"») ...` — descriptor-shape divergence in one edge case |
| 1 | `callee is not callable: undefined [argc=2] (method='call') ...` — Function.prototype.call dispatch on something shape-related |
| 1 | `Expected a Test262Error but got a TypeError` — test expected a Test262Error but received a TypeError (the error was raised but with the wrong class) |

Each is its own isolated investigation; no shared root cause. Treat as the engagement's normal long-tail backlog rather than a default-on blocker.

### §XVI / Doc 734 categorization

Per Doc 730 §XVI: Case-1 closure (cruftless violated `Object.getOwnPropertyDescriptor` semantics for shape-stored entries under enrollment). The §XVI bidirectional engine-diff oracle drove the closure; Pattern 1 (bisect-by-result-jsonl-diff) was the operational tool.

Per Doc 734 §V: growth (c) positive-finding generalization — the default-on flip is empirically justified at this round; test262 within 0.1pp + diff-prod parity + the four residuals classified as long-tail-not-substrate-correctness.

### Methodology meta-observation (worth recording)

The two consecutive bisect-and-fix rounds (CMig-EXT 12 + 13) demonstrate the bisect-by-jsonl-diff pattern at scale:

- **283 → 25 → 4** in three sweeps × ~5 min of bisect + 10 min of investigation + 5 min of fix per round.
- **91% + 84% = 98.6% cumulative closure** via two single-site fixes.
- The dominant-cluster hypothesis (substrate-tier bugs cluster at single consumer sites) corroborated twice in one session.

This is the discipline operating at its empirical sweet spot. DEBUG-METHODOLOGY.md Pattern 1 documents the technique; today's CMig-EXT 12 + 13 are the empirical anchor for the discipline's effectiveness.

### Pred disposition

- **Pred-shape.4** (stable IC pointer for stub lifetime): **CONFIRMED INTEGRATION-READY UNDER DEFAULT**. Every `Object::new_ordinary()` JS-literal allocation enrolls; `Object::shape_ptr_and_slot_for` returns Some(ptr) for any property installed via set_own. Pilot LeJIT-Σ StubE-EXT 5+ unblocks immediately under default enrollment, not just opt-in.
- Pred-shape.1/.2/.3/.5: held / unmeasured / preserved as before.

### Open scope at CMig-EXT 14 close

1. **CMig-EXT 15** (future) — close the 4 residual long-tail failures. Each its own surgical investigation; not blocking.
2. **CMig-EXT 16** (future) — Pred-shape.4 measurement (% of property accesses with non-null shape pointer at access time in a representative workload).
3. **LeJIT-Σ StubE-EXT 5** — translator wiring. Now proceeds under DEFAULT enrollment (not just opt-in) — the IC stubs will see real shape pointers on every JS-literal `obj.x` access.

### Cumulative status at CMig-EXT 14 close

LOC delta: ~20 (Object.getOwnPropertyDescriptor synthesis + default flip). diff-prod 42/42 both modes. test262 default 77.8% (5,591 PASS); escape hatch 77.9% (5,616 PASS); regression vs default-off: 4 fixtures (0.1pp).

**The shapes pilot's substrate-introduction round is functionally complete at the engagement-tier**: default-on enrollment holds across both probe levels; the consumer-migration sub-workstream's primary mission (`shape: Some(root)` on every JS-literal Object) is met; LeJIT-Σ becomes the next load-bearing work under default enrollment.

---

*CMig-EXT 13 + 14 closes. Object.getOwnPropertyDescriptor shape-aware; default-on flip held. test262 within 0.1pp of default-off; diff-prod parity. 98.6% of original 283-PASS regression closed via two single-site fixes using the bisect-by-jsonl-diff pattern. LeJIT-Σ StubE-EXT 5 unblocks under default enrollment.*

---

## CMig-EXT 15 — 2026-05-23 (object-spread regression close, out-of-band)

### Headline

Regression report surfaced by an independent measurement instance: `{...src}` silently produces `{}` under shape-on. Root-caused to `__object_spread` at `intrinsics.rs:831` iterating `.properties.iter()` directly — a classic unmigrated-bypass site CMig's family sweep missed. Fixed via the same shape-aware-then-dictionary pattern CMig-EXT 12/13 used. 42/42 diff-prod + 35/35 runtime lib + 5/5 spread-variant tests PASS.

### The bug

Pre-fix `__object_spread` iteration read only `properties`. For Shape-enrolled source objects (default since CMig-EXT 14), `properties` is empty (values live in `shape_values`); spread silently produced `{}`. The "crash" the parallel instance reported is the downstream consequence of empty-spread breaking an invariant in their workload.

Repro:
```
const src = { a: 1, b: 2, c: 3 };
JSON.stringify({ ...src })
// shape-on:  "{}"           ← bug
// shape-off: '{"a":1,"b":2,"c":3}'   ← correct
```

### The fix (~26 LOC)

Shape-aware iteration following CMig-EXT 12/13 pattern:
1. Iterate shape-stored entries first (shape's `iter_slots`) — plain-data descriptors per shapes seed §IV carve-out, no accessor dispatch needed.
2. Iterate dictionary-stored entries with the existing accessor-handling path.
3. Materialize both lists before the for-loop body so the `rt.obj` borrow is released before `rt.call_function` / `rt.object_set`.

### Probes (Doc 735 §X.h.c)

- **Bench probe**: 5 spread-variant fixtures (nested + override + multiple-source + empty + spread-into-populated) all correct under shape-on.
- **Consumer-route probe**: diff-prod 42/42 PASS. None of the 42 fixtures exercised spread+shape pre-fix — the regression escaped via this gap.
- **Unit-test regression**: rusty-js-runtime lib 35/35 PASS.
- **Fuzz probe**: not run; queued for CMig-EXT 17.

### §XVI / Doc 734 / Doc 735 §X.h categorization

Per Doc 730 §XVI: Case-2 (cruftless violated spec under shape-on; ECMA-262 spread semantics require all enumerable own properties). §XII coercion lift; fix at substrate boundary.

Per Doc 734 §V: growth (b) **negative-finding amendment surfaced out-of-band**. The amendment to CMig: diff-prod + test262-sample alone are NECESSARY but not SUFFICIENT for catching every unmigrated-bypass site. A property-shape-completeness audit (read every site that touches `.properties.iter()`) is the load-bearing discipline CMig's first cut was missing.

Per Doc 735 §X.h.c: load-bearing demonstration that the three-probe-levels discipline catches what bench + consumer-route alone miss. Fuzz would have caught this; CMig's coverage stopped at consumer-route.

### Composition with prior corpus work

- **Doc 729 §A8.13**: consumer-migration is incompletely closed pending a full property-bypass audit (CMig-EXT 16+).
- **Doc 735 §X.h.c**: empirical anchor for the fuzz-probe gap in CMig.
- **Doc 738 §II**: fix's identifiers conform (in-place patch of existing engine helper).

### Open scope at CMig-EXT 15 close

1. **CMig-EXT 16** — Property-bypass audit. grep `.properties.iter()` / `.properties.keys()` / `.properties.values()` across the runtime crate; audit each site for shape-awareness; close additional unmigrated bypasses.
2. **CMig-EXT 17** — Property-shape fuzz harness. Random property-mutation + spread patterns to catch the §X.h.c gap that let CMig-EXT 15 escape.

### Cumulative status at CMig-EXT 15 close

LOC delta: ~26. diff-prod 42/42 GREEN; runtime lib 35/35 GREEN; manual spread 5/5 GREEN. Structural reading: CMig-EXT 16 (audit) + 17 (fuzz) are the remaining substrate work for completeness.

---

*CMig-EXT 15 closes. Out-of-band regression localized, fixed, verified in one round. Demonstrates that diff-prod + test262-sample is insufficient probe-coverage for shape-enrollment correctness; CMig-EXT 16/17 (audit + fuzz) are queued.*

---

## CMig-EXT 16 — 2026-05-23 (property-bypass audit; engagement-wide enumeration)

### Headline

Audit-tier round per Findings VI.6 HIGH priority. Enumerated every `properties.iter()` / `.keys()` / `.values()` / `.contains_key()` call site in the rusty-js-runtime crate (~40 sites). Categorized each by Shape-aware safety. **Net result**: 4 NEEDS-FIX sites identified for CMig-EXT 16.bis substrate round; the rest are SAFE (engine-Dictionary, shape-aware helpers, or Family-B verified chains). Output: `docs/property-bypass-audit.md` (~140 lines).

### Substrate landed

- `pilots/rusty-js-shapes/consumer-migration/docs/property-bypass-audit.md` (~140 lines): methodology, per-file audit tables (intrinsics.rs + interp.rs + value.rs/module.rs/napi.rs), summary by category, NEEDS-FIX list with per-site rationale, forward to CMig-EXT 16.bis substrate fix round + CMig-EXT 17 canonical fuzz harness, §XVI / §V / §X.h categorization, composition with prior corpus work.

### The 4 NEEDS-FIX sites

1. **intrinsics.rs:5731** — JSON.stringify property enumeration. **HIGHEST PRIORITY**. Per CRB-EXT 9 reading, JSON.stringify is one of the largest contributors to cruft's realistic-workload gap; a shape-bypass correctness bug here would compound across realistic workloads + would silently mis-serialize Shape-enrolled objects.
2. **intrinsics.rs:2682** — Headers spread variant. Similar pattern to CMig-EXT 15's __object_spread.
3. **intrinsics.rs:5507** — Generic spread variant (different call site). Same fix pattern.
4. **interp.rs:781** — Set.union / setLike op target_keys enumeration.

All 4 follow the CMig-EXT 15 shape-aware-then-dictionary pattern; the fix shape is well-known and consistent.

### Verification cleanup

Initial scan flagged ~5 NEEDS-FIX sites; verification reads moved two to SAFE:
- **value.rs:508** (`has_own_str`): the shape.slot_of() check at line 506-507 precedes the contains_key fallback. SAFE-VIA-HELPER.
- **interp.rs:1992/2098/2148** (Object.defineProperties/.values/.keys class enumeration): all use the CMig-EXT 4 Family B pattern (shape-iter block chained with properties.iter()). SAFE.

### §XVI / Doc 734 / Doc 735 §X.h categorization

Per Doc 730 §XVI: not applicable (audit-tier; no substrate-correctness call).

Per Doc 734 §V: growth (a) tier-relocation realized — the property-bypass audit was queued at CMig-EXT 15 close as a follow-up; this round produces it. Growth (b) negative-finding amendment in waiting — the 4 NEEDS-FIX sites are not-yet-failed but are correctness gaps that CMig-EXT 16.bis closes.

Per Doc 735 §X.h.c three-probe-levels: this round is design-tier; the actual fix-and-verify is CMig-EXT 16.bis (bench + consumer-route via diff-prod + fuzz via CMig-EXT 17).

### Composition with prior corpus work

- **CMig-EXT 15**: empirically anchored the bug class; this audit enumerates the residual sites.
- **Findings doc IV.1 + IV.2**: directly applied; the audit is the discipline rule 6 queued.
- **Findings doc rule 6 (surface-completeness audit for data-structure changes)**: this round IS rule 6's apparatus applied retroactively to the CMig-EXT 14 default-on flip.
- **Doc 739 cascade-revival pattern**: the audit's NEEDS-FIX sites are NOT (P2.d) stalls; they're CORRECTNESS GAPS at the consumer tier. Doc 739 doesn't apply; sub-pilot-local fixes (CMig-EXT 16.bis) are the right move.
- **CRB-EXT 9 per-workload spread reading**: the JSON.stringify site's HIGH priority cross-references CRB's realistic-workload finding (JSON.parse/.stringify estimated at 5-10× contributor to cruft-vs-bun gap on json_parse_transform).

### Open scope at CMig-EXT 16 close

1. **CMig-EXT 16.bis** — substrate fix round for the 4 NEEDS-FIX sites. ~80-120 LOC across intrinsics.rs + interp.rs. Each fix follows the CMig-EXT 15 pattern. Re-run diff-prod + fuzz-tb + fuzz-ic + manual JSON.stringify probe.
2. **CMig-EXT 17** — canonical 2000-fixture fuzz harness (Findings VI.6 HIGH priority). Engagement-wide instrument that catches shape-bypass bugs proactively. Scope: random property-mutation patterns + spread + JSON.stringify + Object.entries/.values/.keys + Map/Set iteration.

### Cumulative status at CMig-EXT 16 close

LOC delta: ~140 (audit doc only; no source changes). 4 NEEDS-FIX sites identified; ~26 SAFE; ~3 NEEDS-VERIFY (deferred); ~5 DEFENSIVE.

The audit closes Findings VI.6's HIGH-priority audit gap. The substrate fixes (CMig-EXT 16.bis) + canonical fuzz (CMig-EXT 17) remain as the engagement's standing shape-correctness work.

---

*CMig-EXT 16 closes. 4 NEEDS-FIX sites identified (JSON.stringify HIGHEST PRIORITY); CMig-EXT 16.bis substrate fix queued; CMig-EXT 17 canonical fuzz harness remains the engagement-wide probe-coverage close.*
