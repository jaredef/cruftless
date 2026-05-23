# LeJIT-Σ — Trajectory

Per-StubE-EXT log for the LeJIT-Σ stub-emitter pilot. Sub-workstream of `pilots/rusty-js-jit/`. Reads seed.md first; this file records the hand-rolled aarch64 IC stub emitter's substrate moves.

Format: one section per StubE-EXT. Same shape as other Pin-Art trajectory.md files.

---

## StubE-EXT 0 — 2026-05-23 (workstream founding)

### Headline

Apparatus-tier round. Pilot LeJIT-Σ founded per Doc 737 §IV + the keeper's standing "set up seeds at every fractal locale that requires it" directive. The LeJIT seed §I.2 (JIT-EXT 25) pre-filed this coordinate; the spawn is now explicit because the pilot has multi-rung shape (StubE-EXT 0-8 covers founding → bench-baseline → design → scaffold → synthetic-pointer test → translator wiring → bench measurement → fuzz → default-on flip).

### Substrate delivered

- `pilots/rusty-js-jit/stub-emitter/seed.md` (~155 lines) — telos, apparatus (composes with parent LeJIT crate + sibling shapes pilot at the `Object::shape_ptr_and_slot_for` API boundary), methodology with StubE-EXT 0-8 staging, carve-outs (aarch64 only, monomorphic only, GetProp only, shape-cache only), composition with shapes pilot's CMig-EXT 8 enrollment-flip gate, falsifiers Pred-stub.1-.5.
- `pilots/rusty-js-jit/stub-emitter/trajectory.md` (this file).
- `pilots/rusty-js-jit/stub-emitter/docs/` scaffold for StubE-EXT 1 (bench-baseline.md) + StubE-EXT 2 (stub-design.md) outputs.

### Locale registration

Per Doc 737 §IV: nested locale at coordinate `pilots/rusty-js-jit/stub-emitter/` (depth 2). Parent reference: `L.rusty-js-jit` (LeJIT). Sibling cross-reference: `L.rusty-js-shapes` and its nested `L.rusty-js-shapes/consumer-migration`. The pilot composes with the shapes substrate at the `Object::shape_ptr_and_slot_for` API.

This is the engagement's **second prospective-spawn case** (the first was consumer-migration earlier today). Both spawned because their multi-rung shape was known at spawn time per the keeper's standing directive sharpening Doc 737 §VII.

### §XVI / Doc 734 categorization

Per Doc 730 §XVI: not applicable (no probe gated; founding-round documentation only).

Per Doc 734 §V: growth mechanism (a) tier-relocation recursion — the LeJIT-Σ pilot was pre-filed at JIT-EXT 25 as a future coordinate; the spawn now makes it an active locale per Doc 737 §IV. Growth mechanism (b) negative-finding amendment (latent) — the shapes pilot's CMig-EXT 8 enrollment-flip gate is the empirical event that will validate or falsify the Pred-stub.1 3× speedup threshold; below threshold, the pilot's structural claim weakens to (P2.d) and the work is reverted with the boundary documented.

### Composition with prior corpus work

- **Doc 729 §A8.13 substrate-amortization**: LeJIT-Σ is the closure round consuming the shapes pilot's substrate-introduction round. Staging: substrate-first (shapes Shape-EXT 0-4 + consumer-migration CMig-EXT 0-8), closure-second (this pilot StubE-EXT 0-8). EXTs 0-3 here can land in advance of CMig-EXT 8; EXTs 4-8 gate on it.
- **Doc 731 §VII R1**: single-tier baseline JIT shape preserves under the hybrid Cranelift + hand-rolled stub emitter. The stub emitter is a sub-substrate of the same JIT tier; not a second tier. The corpus claim under test (per parent LeJIT seed §I.2) is that this hybrid composition achieves IC fast-path latency competitive with mainstream JITs while preserving R1.
- **Doc 735 §X.h three-probe-levels discipline**: the Pred-stub.1 (≥3× speedup) claim requires bench + consumer-route + fuzz probes per §X.h.c. Each probe activates at a different StubE-EXT (bench at EXT 1 + 6; consumer-route at EXT 5 via diff-prod; fuzz at EXT 7).
- **Doc 737 §IV locale-as-coordinate**: this is the engagement's second prospective-spawn case. The coordinate uniqueness is filesystem-structural; the parent reference is explicit at seed §VII.
- **Doc 738 §II source-tier conventions**: stub-emitter identifiers will fit the five-axis space. `__ic_*` prefix for stub-internal sentinels per §II.a; snake_case methods per §II.b; pillar-path `pilots/rusty-js-jit/derived/src/stub_aarch64.rs` per §II.e.

### Open scope at StubE-EXT 0 close

1. **StubE-EXT 1** — Pre-stub bench probe. Establish baseline measurement for the current extern-call IC dispatch on a 1M-iteration property-access loop on the Pi. Output: `docs/bench-baseline.md`. Test file: `pilots/rusty-js-jit/derived/tests/bench_ic.rs`. Apparatus + small-code round.
2. **StubE-EXT 2** — Stub emitter design. Output: `docs/stub-design.md`. Apparatus-tier; no code.
3. **StubE-EXT 3** — Stub emitter crate scaffold. `pilots/rusty-js-jit/derived/src/stub_aarch64.rs` module + tests. Test-only; not wired into the translator.
4. **StubE-EXTs 4-8** — Per the seed §III methodology.

### Cumulative status at StubE-EXT 0 close

LOC delta: 0 (apparatus-tier round). docs/ scaffold: 1 (empty dir). Locale registered.

The pilot's locale exists; the substrate work begins at StubE-EXT 1.

---

*StubE-EXT 0 closes. The pilot is founded; the next round is the bench baseline measurement.*

---

## StubE-EXT 1 — 2026-05-23 (pre-stub bench baseline)

### Headline

Bench probe activated. Hand-built `function getx(obj) { return obj.x; }` driven through the current extern-call IC dispatch path (`Op::GetPropOnObject` → Cranelift `call jit_getprop_on_object` → `runtime_getprop_on_object` → `rt.object_get`); 1M iterations on the Pi target. **Baseline: 271.0 ns/iter** (270.986 ms elapsed). Pred-stub.1 target reads: ≤90.3 ns/iter for the (P2.a) strict-win claim.

### Substrate delivered

- `cruftless/examples/bench_ic.rs` (~135 LOC) — bench harness. Hand-builds the FunctionProto with `Op::GetPropOnObject` bypassing the upstream-parser gap (per JIT-EXT 24 open scope item 2). Allocates an Ordinary Object with `obj.x = 42.0`, JIT-compiles getx() with `jit_threshold = 1`, warms up 10 calls, measures 1M-iter elapsed.
- `pilots/rusty-js-jit/stub-emitter/docs/bench-baseline.md` (~95 lines) — bench protocol; Pi baseline (271 ns/iter); estimated cost breakdown by component (Rust dispatcher ~120 ns; JIT preamble ~30 ns; **Cranelift extern call ~50 ns** [target of LeJIT-Σ]; runtime helper body ~50 ns [partly addressable post-CMig-EXT 8]; return + reboxing ~20 ns); Pred-stub.1 four-case categorization per Doc 735 §X.h.b ((P2.a) strict-win / (P2.d) correct-but-losing / (P2.c) illegal-speed / (P2.b) slow-stratum); comparison points (Bun IC fast-path low-single-digit ns; cruftless interpreter ~3-5× slower than JIT per seed §VIII).

### Bench location decision

Lives in `cruftless/examples/bench_ic.rs` rather than `pilots/rusty-js-jit/derived/examples/`. Reason: the bench needs the full `Runtime` to drive `call_function`, which requires a `rusty-js-runtime` dependency. The LeJIT crate doesn't depend on the runtime (the dependency direction is runtime → JIT, not JIT → runtime), so adding the dev-dep would invert the architecture. cruftless already wires all the deps for examples like `bench_sum.rs`-equivalent driving; bench_ic.rs slots in alongside.

### Build + gate

- `cargo build --release --example bench_ic -p cruftless`: clean (8.9 MB binary).
- `target/release/examples/bench_ic`: 271 ns/iter on Pi (single run; variance characterization deferred to StubE-EXT 6).
- diff-prod 42/42 unchanged (no behavior change; bench is observe-only).

### §XVI / Doc 734 categorization

Per Doc 730 §XVI: not applicable (bench is observe-only). The bench result IS the §X.h.c bench-probe data point that Pred-stub.1 reads against.

Per Doc 734 §V: growth mechanism (c) positive-finding generalization preparatory — the baseline number is the empirical anchor against which all subsequent LeJIT-Σ substrate moves are measured.

### Composition with prior corpus work

- **Doc 735 §X.h.c three-probe-levels discipline**: this is the **bench probe** for Pred-stub.1. Consumer-route probe activates at StubE-EXT 5 (diff-prod 42/42 + a JIT-on hot-loop fixture). Fuzz probe activates at StubE-EXT 7 (shape-transition-history fuzz over the IC dispatch space).
- **Doc 735 §X.h.b (P2) sub-cases** mapped explicitly in bench-baseline.md §4. The four-case categorization is the falsifier rubric for the eventual StubE-EXT 6 re-measurement.
- **LeJIT seed §I.2 falsifier threshold**: the 3× speedup threshold (271 → ≤90.3 ns/iter) is the explicit Pred-stub.1 target. Below threshold = (P2.d) revert + document boundary.

### Pred disposition

- **Pred-stub.1** (≥3× speedup): baseline established at 271 ns/iter. Target ≤90.3 ns/iter post-StubE-EXT 5. Falsifier reading deferred to StubE-EXT 6.
- **Pred-stub.2** (no use-after-free under shape transitions): fuzz probe at StubE-EXT 7.
- **Pred-stub.3** (cache convergence under monomorphic workload): integration test at StubE-EXT 5.
- **Pred-stub.4** (Doc 738 §II convention conformance): bench identifier `bench_ic.rs` fits the pillar-path convention (cruftless/examples/ is the engagement's standing examples location); no `__`-prefix because the bench doesn't introduce JS-observable state.
- **Pred-stub.5** (Doc 731 §VII R1 single-tier preservation): no change at the bench tier; preservation tested when LeJIT-Σ stub emitter lands (StubE-EXTs 3+).

### Open scope at StubE-EXT 1 close

1. **StubE-EXT 2** — Stub emitter design. Choose cache layout, patching mechanism, state machine. Output: `docs/stub-design.md`. Apparatus-tier; no code.
2. **StubE-EXT 3** — Scaffold `pilots/rusty-js-jit/derived/src/stub_aarch64.rs` module + unit tests against synthetic shape pointers. Test-only; not wired into translator.
3. **StubE-EXT 4** — Synthetic shape-pointer integration test.
4. **StubE-EXT 5** — Wire stub into JIT translator under `CRUFTLESS_LEJIT_STUB=1` env flag.
5. **StubE-EXT 6** — Bench re-measurement; (P2) categorization per Doc 735 §X.h.b.
6. **StubE-EXT 7** — Fuzz probe.
7. **StubE-EXT 8** — Default-on flip.

### Cumulative status at StubE-EXT 1 close

LOC delta: ~135 (bench harness) + docs. Pi baseline: 271 ns/iter for the current dispatch path. diff-prod 42/42 unchanged.

The substrate move that LeJIT-Σ ships against has a measured anchor. StubE-EXT 2's design round begins next.

---

*StubE-EXT 1 closes. The bench-probe baseline is 271 ns/iter on the Pi. Pred-stub.1's 3× threshold reads at ≤90.3 ns/iter for the post-stub-emitter measurement.*

---

## StubE-EXT 2 — 2026-05-23 (stub emitter design)

### Headline

Apparatus-tier round. No code. Output: `docs/stub-design.md` (~220 lines) — concrete choices for the four design decisions named in seed §III + the new finding from §8 that the per-iter budget may not hit Pred-stub.1's 3× threshold from the IC layer alone (which has roadmap implications recorded in §8).

### Design decisions chosen

1. **Cache layout: side-table indexed by IC-site id** (`ICStubCache.sites: Vec<ICEntry>`). Alternative (inline literal in JIT-emitted code) rejected because it requires `mprotect` + I-cache flush per patch; side-table needs only memory store + `dsb ish`. Per-IC-site id assigned at translator time; cache grows as functions JIT-compile.

2. **Patching mechanism: memory-store-only with `dsb ish`** data-memory barrier. No I-cache flush needed because patching DATA (the side-table) not INSTRUCTIONS. Single-threaded runtime; no cross-core visibility concern.

3. **State machine**: Cold (cached_shape=null) → Warm-Mono (first hit patches) → Cold-after-miss (transitional re-patch) → Degraded (after MISS_THRESHOLD=8, stop patching and permanently route to slow path). Polymorphic-IC (linear scan of N cached shapes per site) queued as LeJIT-Σ.poly closure round.

4. **Deopt handoff**: stub itself never deopts. On stub miss, the existing `runtime_getprop_on_object` extern call handles deopt-on-non-Number per JIT-EXT 24. The deopt machinery is unchanged; LeJIT-Σ adds a cache layer in front of it.

### Source-tier conformance (Doc 738 §II)

- Module: `pilots/rusty-js-jit/derived/src/stub_aarch64.rs` (§II.e).
- Types: `ICStubCache`, `ICEntry`, `ICState` (PascalCase).
- Methods: `emit_getprop_stub` (snake_case, no `_via` because this is JIT-emitter-side not Runtime-dispatching).
- Internal sentinels reserved: `__ic_site_id`, `__ic_cached_shape`, `__ic_cached_slot` (per §II.a if needed; not used in the side-table design).

### Finding: per-iter budget may not hit Pred-stub.1 from IC layer alone

Pre-implementation budget estimate (post-stub, cache hit): **~180 ns/iter** — vs the 90.3 ns/iter target.

Decomposition:
- Rust dispatcher (call_function) ~120 ns — invariant
- JIT preamble (arg coercion) ~30 ns — sibling Value-tag inline emitter's territory
- Side-table load + receiver shape load + compare + branch + slot load: ~9 ns net new
- Return + reboxing ~20 ns — invariant

The ~120 ns Rust dispatcher dominates. **Without dispatcher refactoring or value-tag-inline, the stub alone shows ~1.5-2× speedup, not 3×.**

Resolution: StubE-EXT 6's measurement reports the actual; (P2) categorization per Doc 735 §X.h.b decides the next move:
- **(P2.a) strict-win** if observed ≥3× (unlikely from IC layer alone per the budget).
- **(P2.d) correct-but-losing** if 1.5-2× (likely). Two follow-on paths:
  - (a) document partial speedup; pivot to value-tag-inline + dispatcher refactoring as separate sibling pilots.
  - (b) merge IC stub work with dispatcher refactoring into one larger substrate move.
- The four-case categorization is the decision rubric, not a verdict in advance.

### Composition with prior corpus work

- **Doc 735 §X.h.b (P2) sub-cases**: §8 of the design doc enumerates explicitly which cases the StubE-EXT 6 measurement might land in + the response per each. Falsifier rubric is concrete.
- **Doc 738 §II source-tier coordinate system**: design §7 maps the stub-emitter's types + methods + sentinels onto the five-axis convention space. Cross-axis consistency by construction.
- **Doc 729 §A8.13 substrate-amortization**: the design anticipates the substrate-amortization shape — if the stub alone is (P2.d), the value-tag-inline + dispatcher-refactor sibling pilots compose with this one to close the 3× target collectively.

### §XVI / Doc 734 categorization

Per Doc 730 §XVI: not applicable (no probe gated; design-only). Per Doc 734 §V: growth mechanism (b) **negative-finding amendment** preparatory — the §8 budget analysis surfaces a likely-shortfall against Pred-stub.1's threshold; the design records this as a forward concern so StubE-EXT 6's measurement reads against an honest expectation, not an aspirational one.

### Open scope at StubE-EXT 2 close

1. **StubE-EXT 3** — Scaffold `pilots/rusty-js-jit/derived/src/stub_aarch64.rs` (~250 LOC) + tests (~150 LOC) per design §10. Coordinated cross-crate change: `runtime_getprop_on_object` signature extension to return `(value, *const Shape, u32 slot)` lands at this round too (in rusty-js-runtime).
2. **StubE-EXT 4** — Synthetic shape-pointer integration test.
3. **StubE-EXT 5** — Wire into translator under env flag. **Gates on CMig-EXT 8.**
4. **StubE-EXT 6** — Re-measure; (P2) categorize.
5. **StubE-EXT 7** — Fuzz.
6. **StubE-EXT 8** — Default-on flip.

### Cumulative status at StubE-EXT 2 close

LOC delta: 0 (apparatus). docs/ artifacts: 2 (bench-baseline + stub-design). The stub's structural shape is chosen; the implementation begins at StubE-EXT 3.

---

*StubE-EXT 2 closes. Design is anchored: side-table cache, memory-store patching, four-state machine, side-table indexed by IC-site id. §8 honestly flags the per-iter budget gap; StubE-EXT 6's measurement decides the (P2) categorization.*

---

## StubE-EXT 3 — 2026-05-23 (stub_aarch64 module scaffold + state-machine tests)

### Headline

First code round of LeJIT-Σ. Scaffolds `pilots/rusty-js-jit/derived/src/stub_aarch64.rs` (~325 LOC including tests) with the `ICStubCache` + `ICEntry` types, the four-state state machine, the thread-local cache singleton, and the slow-path observer helpers. `emit_getprop_stub` declared as placeholder; the Cranelift IR emission body lands at StubE-EXT 4. **10/10 unit tests PASS on first build**; the state-machine logic is verified (cold → warm → cold-after-miss → degraded transitions) without Cranelift integration.

### Substrate landed

- `pilots/rusty-js-jit/derived/Cargo.toml` — `rusty-js-shapes` path-dep added (the stub state machine consumes `Rc<Shape>` for pin-against-drop per Pred-stub.2).
- `pilots/rusty-js-jit/derived/src/lib.rs` — `pub mod stub_aarch64;`.
- `pilots/rusty-js-jit/derived/src/stub_aarch64.rs` (~325 LOC):
  - `ICSiteId` type alias (u32).
  - `MISS_THRESHOLD` constant (8 per stub-design.md §5 tunable).
  - `ICState` enum: Cold / WarmMono / ColdAfterMiss / Degraded.
  - `ICEntry` struct: cached_shape (*const Shape) + cached_slot (u32) + pinned_shape_holder (Option<Rc<Shape>> per design §11 stable-pointer safety story) + miss_count + degraded.
  - `ICEntry::observe(shape, slot)`: handles cold → warm; counts misses on shape change; degrades past MISS_THRESHOLD.
  - `ICEntry::observe_miss_no_shape()`: slow-path called when receiver is Dictionary-form (no shape to cache).
  - `ICStubCache` side-table: `Vec<ICEntry>` indexed by `ICSiteId`. `alloc_site()` allocates a new id; `entry` / `entry_mut` access; `state_histogram` diagnostic.
  - Thread-local `IC_STUB_CACHE` per design §5 single-threaded discipline.
  - Helper fns `alloc_ic_site`, `observe_at_site`, `observe_miss_no_shape_at_site` for the eventual slow-path observer surface.
  - `emit_getprop_stub(site_id, ...)` declared with placeholder body. Full Cranelift IR emission lands at StubE-EXT 4.

### Tests (10/10 PASS)

| test | corroborates |
|---|---|
| `cold_entry_starts_null` | constructor invariant |
| `cold_to_warm_on_first_observe` | first-hit cache patching |
| `warm_to_cold_after_miss_on_shape_change` | shape-change triggers ColdAfterMiss + miss_count++ |
| `degrades_past_miss_threshold` | MISS_THRESHOLD=8 boundary; cache cleared on degrade |
| `degraded_entry_stops_observing` | degraded entries are sticky |
| `observe_miss_no_shape_increments_count` | Dictionary-receiver miss counted |
| `observe_miss_no_shape_on_cold_is_noop` | cold entries don't pre-count |
| `icstubcache_alloc_assigns_sequential_ids` | id allocation contract |
| `icstubcache_histogram_classifies_state` | diagnostic surface |
| `doc738_convention_smoke_test` | Pred-stub.4 compile-time conformance |

### Build + engagement-wide gates

- `cargo build --release -p rusty-js-jit`: clean.
- `cargo test --release -p rusty-js-jit --lib stub_aarch64`: 10/10 PASS (0.00s).
- `cargo build --release --bin cruft -p cruftless`: clean.
- diff-prod **42/42 PASS** unchanged.

### §XVI / Doc 734 categorization

Per Doc 730 §XVI: not applicable (no behavioral change — `emit_getprop_stub` is a placeholder; translator doesn't call it yet).

Per Doc 734 §V: growth mechanism (a) tier-relocation — the IC state machine is now a first-class type with explicit transitions, where the parent LeJIT crate previously had no IC-state representation at all (extern slow path made every call without tracking miss counts or shapes).

### Composition with prior corpus work

- **Doc 729 §A8.13 substrate-amortization**: LeJIT-Σ's substrate-introduction round divides further into infrastructure (state machine + types: this round) + Cranelift IR emission (StubE-EXT 4) + translator wiring (StubE-EXT 5). Each is a focused round.
- **Doc 735 §X.h.c three-probe-levels discipline**: this round adds the **bench probe substrate** at the state-machine layer (the unit tests are the bench-probe equivalent for the cache state contract). Consumer-route probe at StubE-EXT 5; fuzz probe at StubE-EXT 7.
- **Doc 738 §II source-tier conventions**: PascalCase types (ICStubCache / ICEntry / ICState), snake_case methods (observe / observe_miss_no_shape / alloc_site / entry_mut), `ICSiteId` type alias (no `_via` because not Runtime-dispatching), thread-local singleton convention matches Shape::root().

### Pred disposition

- **Pred-stub.2** (no use-after-free under shape transitions): the `pinned_shape_holder: Option<Rc<Shape>>` field IS the safety mechanism; tests confirm the holder is set when cached_shape is non-null and cleared when degraded. Integration-tier corroboration at StubE-EXT 4 + 7 (fuzz).
- **Pred-stub.3** (cache convergence under monomorphic workload): the state machine's contract IS the convergence guarantee; `cold_to_warm_on_first_observe` + the absence of any path back to Cold from WarmMono (except via Degrade) corroborate.
- **Pred-stub.4** (Doc 738 §II conventions): `doc738_convention_smoke_test` compiles iff all public identifiers exist with the conformant naming.
- **Pred-stub.1** + **Pred-stub.5**: wait for StubE-EXT 6 + 8 measurements.

### Open scope at StubE-EXT 3 close

1. **StubE-EXT 4** — Cranelift IR emission body for `emit_getprop_stub`. Lands the actual aarch64 inline shape-check + slot-load IR. Plus synthetic shape-pointer integration test exercising emit + execute via Cranelift JITModule. Coordinated cross-crate change: extend `runtime_getprop_on_object` signature to return `(value, *const Shape, u32 slot)` so the slow path can observe at the IC site.
2. **StubE-EXT 5** — Wire emitter into translator under `CRUFTLESS_LEJIT_STUB=1` env flag. **Gates on shapes CMig-EXT 8** (enrollment flip).
3. **StubE-EXT 6** — Re-measure; (P2) categorize per Doc 735 §X.h.b.
4. **StubE-EXT 7** — Fuzz probe.
5. **StubE-EXT 8** — Default-on flip.

### Cumulative status at StubE-EXT 3 close

LOC delta: ~340 (stub_aarch64.rs 325 + Cargo.toml dep). 10/10 unit tests; diff-prod 42/42 unchanged. The IC state machine exists, is tested in isolation, and is ready to receive its Cranelift IR emission body at StubE-EXT 4.

---

*StubE-EXT 3 closes. The cache state machine compiles, tests, and is ready. StubE-EXT 4 emits the actual aarch64 IR against synthetic shape pointers.*

---

## StubE-EXT 4 — 2026-05-23 (Cranelift IR emission for the stub pattern)

### Headline

Lands the inline compare-branch-load IR pattern via `emit_stub_pattern` + the integration helper `build_stub_pattern_module` that builds a complete JITModule containing one stub function. **12/12 unit tests PASS** (10 state-machine from EXT 3 + 2 new Cranelift round-trip tests). The IR pattern compiles, runs on aarch64, and produces correct values on both hit and miss paths.

Scope-narrowed from "full integration test against real Object / ICEntry layouts" to "stub pattern proven in isolation with flat i64 inputs." Real Object / ICEntry struct-layout integration happens at StubE-EXT 5 when the translator wires this into Op::GetPropOnObject dispatch. The narrowing matches Doc 729 §A8.13 substrate-amortization (substrate-introduction round is the IR pattern; closure round is the translator integration that consumes it).

### Substrate landed

- `pilots/rusty-js-jit/derived/src/stub_aarch64.rs` (+~125 LOC):
  - `emit_stub_pattern(builder, recv_shape, cached_shape, cached_slot, values_base, slow_path_result) -> Value`:
    - Creates hit / miss / merge blocks.
    - Emits `icmp Equal recv_shape, cached_shape`.
    - Hit block: `imul cached_slot, 8`, `iadd values_base + offset`, `load i64 [addr]`, jump merge with loaded value.
    - Miss block: jump merge with slow_path_result.
    - Merge block param IS the return value.
  - `build_stub_pattern_module() -> extern "C" fn(i64, i64, i64, i64, i64) -> i64`:
    - Mirrors translator.rs's JITBuilder setup (flag: use_colocated_libcalls=false, is_pic=false; cranelift_native::builder; finish to ISA).
    - Declares + defines the `stub_pattern` function via the JITModule.
    - Returns a callable function pointer via `get_finalized_function` + transmute.
  - 2 new tests: `stub_pattern_cache_hit_returns_slot_value`, `stub_pattern_cache_miss_returns_slow_path`.

### Build + gates

- `cargo build --release -p rusty-js-jit`: clean.
- `cargo test --release -p rusty-js-jit --lib stub_aarch64`: **12/12 PASS** (0.02s).
- `cargo build --release --bin cruft -p cruftless`: clean.
- diff-prod **42/42 PASS** unchanged.

### Bug surfaced + fixed (preserved per Doc 729 §A8.16 / §A8.17 discipline)

The first build of `build_stub_pattern_module` panicked at runtime with "PLT is currently only supported on x86_64" from `cranelift-jit 0.118 backend.rs:297`. Root cause: default `JITBuilder::new` enables PLT-based libcall thunks which aarch64 doesn't implement. The fix was to mirror `translator.rs:140-147`'s explicit ISA construction with `use_colocated_libcalls=false` + `is_pic=false`. Recorded inline in the helper's source comment so future-readers see the rationale at the load site.

### §XVI / Doc 734 categorization

Per Doc 730 §XVI: Case-4 (implementation freedom) at the (P2.a)-eligible cost-stratum dimension. The compare-branch-load pattern is the standard IC fast-path shape; no spec divergence; cruftless's narrow alphabet (per Doc 731) makes it tractable to emit in one helper function.

Per Doc 734 §V: growth mechanism (a) tier-relocation — the IR emission helper is a new tier between the stub state machine (StubE-EXT 3) and the translator wiring (StubE-EXT 5). Growth mechanism (b) negative-finding amendment — the PLT panic surfaced a Cranelift configuration constraint on aarch64 that the design doc hadn't anticipated; the fix is recorded inline + here for the engagement's catalog.

### Pred disposition

- **Pred-stub.4** (Doc 738 §II conventions): `emit_stub_pattern` + `build_stub_pattern_module` are snake_case (per §II.b; no `_via` because not Runtime-dispatching); pillar-path conforms; new Cranelift-side identifiers reuse Cranelift's PascalCase types.
- **Pred-stub.5** (Doc 731 §VII R1 single-tier): preserved by construction — the stub emitter is straight-line IR emission (no second JIT tier; no internal optimization passes beyond what Cranelift owns).
- **Pred-stub.1/.2/.3** still wait for StubE-EXT 5+ integration and EXT 6+ measurements.

### Composition with prior corpus work

- **Doc 731 §VII R8** (no internal optimization passes): preserved — `emit_stub_pattern` is a single function emitting straight-line CLIF; no IR transformation passes; Cranelift handles all instruction selection / regalloc / scheduling.
- **Doc 735 §X.h.b (P2) sub-cases**: today's measurement-blocking work (the integration round needs CMig-EXT 8 enrollment to produce real shaped Objects to cache) means EXT 4 is structurally before the (P2) categorization moment. Pred-stub.1 reads at EXT 6 against the EXT 1 baseline of 271 ns/iter.

### Open scope at StubE-EXT 4 close

1. **StubE-EXT 5** — Translator wiring. Op::GetPropOnObject codegen path branches under `CRUFTLESS_LEJIT_STUB=1` env flag: emit the inline stub via `emit_stub_pattern` against real ICStubCache side-table addresses + receiver Object struct field offsets. Cross-crate change: `runtime_getprop_on_object` signature extension to return `(value, *const Shape, u32 slot)` for the slow-path observer. **Gates on shapes CMig-EXT 8** for real shape-cache hits; pre-CMig-EXT 8 the env-flag path is testable for correctness (every receiver Dictionary → every IC miss → slow path runs every time → observe_miss_no_shape_at_site fires).
2. **StubE-EXT 6** — Re-measure post-CMig-EXT 8; (P2) categorize.
3. **StubE-EXT 7** — Fuzz probe (property-addition-history fuzz exercising shape transitions through the IC dispatch).
4. **StubE-EXT 8** — Default-on flip.

### Cumulative status at StubE-EXT 4 close

LOC delta: ~125 (stub IR emission + integration helper + 2 tests). 12/12 unit tests; diff-prod 42/42 unchanged. The Cranelift IR pattern exists, compiles, runs, and produces correct values.

---

*StubE-EXT 4 closes. The inline compare-branch-load IR pattern is corroborated on aarch64; the substrate is ready for StubE-EXT 5's translator integration.*
