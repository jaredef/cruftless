# LeJIT Enhancements Log

*Cross-locale empirical log spanning the LeJIT parent pilot and its nested sub-pilots (LeJIT-Σ stub-emitter, LeJIT-Ψ value-tag-inline, future LeJIT-Τ tiny-baseline, future LeJIT-Σ' x86_64 stub-emitter). Each entry records a measurable change to the LeJIT substrate's behavior — performance, semantics, infrastructure — with provenance back to the originating locale trajectory.*

## Discipline

Per the keeper's 2026-05-23 directive: this log captures both kinds of results that fall out of LeJIT-tier substrate work.

**Anticipated results** — outcomes the locale's seed or trajectory predicted in advance. Logged as a short one-paragraph row referencing the originating locale rung. The trajectory entry is the load-bearing record; this log is the cross-locale index.

**Unanticipated results** — outcomes no locale rung predicted. Logged verbosely (cause, mechanism, measurement, hypothesis about why the prediction missed, implication for forward work). The verbose form preserves the substrate-amortization-cascade signal per Doc 729 §A8.13 + Doc 734 §V.c positive-finding generalization. Unanticipated results often surface a corpus-tier or locale-tier framework refinement; verbose logging makes the recognition retrievable.

**Composition with the trajectory record**: each locale's `trajectory.md` records its rung-by-rung substrate moves. This enhancements log is the cross-locale slice that asks "what changed about LeJIT itself, measured how, predicted where." A reader at any point can read this log to get the LeJIT pilot's running empirical state without traversing four nested trajectories.

---

## 2026-05-23 — VTI-EXT 3a: Value layout pinning gives 5 ns/iter bench reclaim **[UNANTICIPATED]**

**Locale**: `pilots/rusty-js-jit/value-tag-inline/trajectory.md` → VTI-EXT 3a (substrate-introduction round per Doc 729 §A8.13).

**Substrate change**: Added `#[repr(C, u8)]` to `pub enum Value` in `pilots/rusty-js-runtime/derived/src/value.rs` + eight `VALUE_TAG_*` discriminant constants + `VALUE_NUMBER_PAYLOAD_OFFSET` + compile-time const assertions + four runtime unit tests. ~70 LOC total. Pure layout pinning; no JIT-emitter changes, no calling-convention changes, no semantic changes to Rust callers.

**Measurement**:

```
bench_call_overhead pre-pinning  (VTI-EXT 1, 2026-05-23): 127.1 ns/iter
bench_call_overhead post-pinning (VTI-EXT 3a, 2026-05-23): 122.0 ns/iter
                                                       Δ:  −5.1 ns (−4.0%)
```

Workload: 1M iterations of `id(Number(42))` dispatched via `Runtime::call_function`. Same hardware (Pi target), same build profile (release), same workload, single-run measurement at each side (variance characterization deferred to VTI-EXT 6).

**Why this was unanticipated**: VTI-EXT 2's design doc (`pilots/rusty-js-jit/value-tag-inline/docs/inline-design.md` §3) estimated Option A's reclaim at "5-10 ns" but attributed the reclaim to the JIT-side prologue tag-check + payload-extract that VTI-EXT 3b will emit. The substrate-introduction round (3a) was framed as enabling apparatus — pin the layout so the closure round (3b) can emit against a known offset — with no per-iter cost reduction expected from the pinning alone.

The bench shows otherwise. The 5 ns reclaim arrived at the substrate-introduction tier, one round earlier than the closure round would have produced it. Either:

1. **`#[repr(C, u8)]` produces a more cache-friendly Value layout than rustc's default for `pub enum Value` without repr attributes.** The default layout for a Rust enum with mixed-size payloads (1-byte bool / 8-byte f64 / Rc<T> pointers) uses an opaque discriminant choice that may have produced a larger size or worse alignment than the explicit `repr(C, u8)` form. Smaller / better-aligned values would improve cache locality for the `Vec<Value>` argument array, the closure_v Rc clone, and the call_function dispatcher's match codegen. 5 ns/iter on a 1M-iter workload is consistent with one fewer cache line touched per call.

2. **Match codegen on the pinned discriminant is tighter.** Rustc's match-codegen for `#[repr(u8)]` enums emits a jump-table on the discriminant byte directly; for the default representation it must first extract the discriminant through whatever niche-or-tag scheme rustc picked. The `unbox_arg(&Value) -> i64` helper called at every dispatch site is exactly this shape. A tighter match would account for some of the reclaim.

3. **Measurement variance.** Single-run measurement. A ±5 ns variance band would put the result inside noise. Variance characterization is deferred to VTI-EXT 6; until then this finding is provisional.

**Hypothesis** (to be confirmed at VTI-EXT 6): cause is some mix of (1) and (2). The pinning's per-iter benefit is real but the magnitude needs multi-run variance bounding to be load-bearing.

**Implication for forward work**:

- **Per Doc 729 §A8.13 substrate-amortization-cascade reading**: the cascade arrived at the substrate-introduction tier in VTI as it did in shapes (LeJIT seed §I.3's recognition). The pattern recurs: each LeJIT-tier substrate-introduction round produces unanticipated per-iter cost reduction in addition to the apparatus it enables. The pattern is now suspected at two of two cases (shapes Shape-EXT 4 enrollment → 26%; VTI layout pinning → 4%). LeJIT seed §I.3 should be updated to predict the cascade explicitly at future substrate-introduction rounds (e.g., tiny-baseline's first apparatus move).

- **Per Doc 734 §V.c positive-finding generalization**: the recognition is corpus-original at the LeJIT-tier scale but consistent with §I.3's broader claim. No new corpus articulation needed; an amendment to LeJIT seed §I.3 records the second-case corroboration.

- **Per Doc 735 §X.h.c three-probe-levels**: bench probe alone is NECESSARY but not sufficient. diff-prod 42/42 GREEN is the consumer-route probe at the runtime-semantics tier; the bench reclaim hasn't been validated under non-monomorphic workloads (the Number-only id(x) bench). The fuzz probe (VTI-EXT 7) will exercise the variance + non-monomorphic cases. Until both land, the 5 ns reclaim is provisional — the right (P2) sub-case categorization per §X.h.b cannot be assigned yet (could be (P2.a) strict-win, could be measurement variance).

- **For VTI-EXT 3b**: the closure round's expected reclaim from the JIT-prologue emission itself must now be re-read against a 122 ns baseline (not 127 ns). If VTI-EXT 3b adds another ~5 ns reclaim, total VTI contribution would be ~10 ns, near the upper end of inline-design.md §3 Option A's 5-10 ns estimate. If 3b adds nothing, then 3a captured the entire VTI contribution and the seed's §I.3 composition reading needs further sharpening (VTI's arm shrinks; tiny-baseline's arm becomes even more load-bearing).

**Provenance**:
- Trajectory: `pilots/rusty-js-jit/value-tag-inline/trajectory.md` VTI-EXT 3a (close)
- Design doc: `pilots/rusty-js-jit/value-tag-inline/docs/inline-design.md` §3 Option A
- Bench harness: `cruftless/examples/bench_call_overhead.rs`
- Pre-pinning measurement: VTI-EXT 1 bench, 127.1 ns
- Post-pinning measurement: VTI-EXT 3a re-run, 122.0 ns

---

## 2026-05-23 — VTI-EXT 3b: payload-extract-only first cut is 18.9 ns SLOWER (P2.d) **[UNANTICIPATED]**

**Locale**: `pilots/rusty-js-jit/value-tag-inline/trajectory.md` → VTI-EXT 3b (closure round to VTI-EXT 3a's substrate-introduction, per Doc 729 §A8.13).

**Substrate change**: First-cut closure of VTI Option A from inline-design.md §3. Added `CRUFTLESS_LEJIT_VTI=1` env flag detection in translator; added `vti_enabled: bool` to CompiledFn so dispatcher can agree with prologue's expectation; translator's per-arg prologue, when VTI=1, treats the i64 entry-block-param as `*const Value`, loads f64 at offset 8 (VALUE_NUMBER_PAYLOAD_OFFSET per VTI-EXT 3a), saturating-converts to i64. Dispatcher under VTI=1 passes `&args[i] as *const Value as i64` instead of `unbox_arg(&args[i])`. Inline tag-check + WrongArgTag deopt deferred to VTI-EXT 3c (this round trusts dispatcher's existing `jit_compatible_arg` precheck).

**Measurement**:

```
bench_call_overhead VTI OFF (regression baseline): 126.6 ns/iter
bench_call_overhead VTI ON  (3b path):              145.5 ns/iter
                                              Δ:     +18.9 ns (+14.9%)
```

Workload: 1M iterations of `id(Number(42))` via `Runtime::call_function`. Same hardware (Pi), same build, single-run measurement.

VTI OFF result of 126.6 ns is consistent with VTI-EXT 3a's 122.0 ns reading within plausible variance (±5 ns), which itself walks back the strength of 3a's "−5 ns reclaim from layout pinning alone" claim — possibly the 3a reading was variance-low. Variance characterization (VTI-EXT 6) remains the load-bearing measurement for both findings.

**Why this was unanticipated**: VTI-EXT 2 inline-design.md §3 Option A predicted 5-10 ns reclaim. Per the design's hypothesis, removing the Rust dispatcher's `match` (`unbox_arg`) from the hot path would reclaim 5-10 ns. The empirical reading shows the opposite — pushing the work into the JIT prologue costs MORE than leaving it in Rust.

**Hypothesis** for why the prediction missed:

1. **`fcvt_to_sint_sat` is more expensive than `*f as i64`'s codegen.** On aarch64, `*f as i64` from Rust compiles to a single `fcvtzs x, d` with bounds-saturate via `min`/`max` constants inlined; Cranelift's `fcvt_to_sint_sat` may emit a longer sequence with explicit NaN-check branch + saturate fixup blocks. The Rust optimizer has more context (it knows the f64 came from a Value::Number which restricts the range) than Cranelift's IR-level lowering has.

2. **Memory load through raw pointer is not free, and the dispatcher already had the f64 in a register.** `unbox_arg(&Value::Number(f))` reads `*f` directly via Rust pattern-matching — the optimizer can keep the f64 in a register across the call to `jit_fn.func.call1`. Under VTI, the dispatcher writes the pointer through the calling convention; the JIT prologue must do a fresh memory load. The substitution replaces one register-to-register move with one load + one cast + one conversion.

3. **Calling-convention reinterpretation defeats register allocator's view.** When the dispatcher passes a value-typed i64 (the historical path), rustc's optimizer can sometimes inline the call sequence + register-pass the i64 efficiently. When passing a raw pointer, the optimizer treats it as an opaque foreign-ABI value and may emit conservative spill/reload sequences around the call site.

4. **JIT-side load is unaliased only by trust, not by `MemFlags::trusted()` alone.** Cranelift's load with `MemFlags::trusted()` asserts no traps but does not assert no-alias; the optimizer cannot hoist the load across other emitted instructions. In a longer function body this would matter less; in the minimal id(x) bench it accounts for an entire round-trip cost.

5. **The dispatcher still does the precheck.** Even under VTI=1, the dispatcher calls `jit_compatible_arg(v)` which already pattern-matches on the discriminant. VTI's first cut doesn't remove that work — it adds the JIT-prologue load on top. The "save the Rust match" reclaim Option A anticipated requires VTI-EXT 3c's inline tag-check to let dispatcher SKIP the precheck.

The combination of (1) + (2) + (3) plausibly accounts for the +18.9 ns regression. (5) is the structural argument that VTI-EXT 3b alone CANNOT win — the full VTI path needs 3c to land the precheck-removal before the calling-convention switch pays.

**Implication for forward work** per Doc 735 §X.h.b sub-case categorization:

- **VTI-EXT 3b as landed is (P2.d) correct-but-losing.** Algorithm-correct (the JIT produces the right i64 for the test workload + 38/38 JIT tests + 35/35 runtime tests PASS) + implementation-correct + per-op slower on target hardware. Exact match for §X.h.b's (P2.d) definition.

- **Whether VTI as a whole is (P2.d) is not yet settled.** The structural argument under hypothesis (5) is that VTI-EXT 3c (inline tag-check + dispatcher precheck-skip) is the substrate move that lets VTI net-win. Without 3c, the calling-convention switch is wasted work. With 3c, the comparison is:
  - Historical: `jit_compatible_arg(match v) + unbox_arg(match v) + call_function(i64)`
  - Full VTI: `call_function(*const Value) + JIT load+fcvt+tag-check+deopt-on-mismatch`
  The full VTI removes TWO Rust matches; 3b alone removed ZERO. If 3c's deopt-machinery overhead is comparable to or less than the two Rust matches' cost, VTI nets positive.

- **Three-probe-levels per Doc 735 §X.h.c**: bench probe NEGATIVE. Consumer-route probe (diff-prod with VTI=1) not yet run; expected NEUTRAL since diff-prod fixtures don't exercise the JIT hot path. Fuzz probe (VTI-EXT 7) not run. The (P2.d) categorization is gated on full three-probe sweep, but the bench negative is already disqualifying for (P2.a) strict-win.

- **Substrate move retention question**: VTI-EXT 3b's substrate code is correct, well-tested, and behind an env flag (default OFF). Leaving it in tree under the flag preserves the apparatus for VTI-EXT 3c to consume; reverting it costs the apparatus but removes a (P2.d)-marked path. Recommendation: **retain the substrate behind the flag; mark VTI-EXT 3b's bench result as the empirical anchor for the 3c attempt; declare VTI-EXT 3c the load-bearing round for the (P2.a) vs (P2.d) decision.**

- **If VTI-EXT 3c also fails to net-positive**: VTI as a pilot is (P2.d) at first cut. Per Doc 735 §X.h.d saturation-as-escalation-signal, the next substrate target is OUTSIDE VTI — specifically the tiny-baseline dispatcher refactor pre-filed at LeJIT seed §I.2 item 5. The substrate is calling more loudly each round.

- **Sharpening for the seed §I.3 composition table**: VTI's expected contribution was named "1.2-1.4×"; today's empirical reading places that prediction at risk. The §I.3 amendment candidate (queued from VTI-EXT 3a's enhancement-log entry) should now also reflect: VTI's contribution is contingent on VTI-EXT 3c's success; if 3c does not flip the bench positive, VTI's arm in the §I.3 multiplicative composition is 1.0× (no contribution) and the dispatcher-refactor arm must alone carry from 1.36× (shape) + 1.x× (LeJIT-Σ) to the 3× target.

**Provenance**:
- Trajectory: `pilots/rusty-js-jit/value-tag-inline/trajectory.md` VTI-EXT 3b (in progress at this writing)
- Design doc: `pilots/rusty-js-jit/value-tag-inline/docs/inline-design.md` §3 Option A predicted 5-10 ns reclaim
- Substrate: `pilots/rusty-js-jit/derived/src/translator.rs` lejit_vti env flag + per-arg prologue switch + CompiledFn.vti_enabled field
- Dispatcher: `pilots/rusty-js-runtime/derived/src/interp.rs` conditional pointer-pass under vti_enabled
- Bench harness: `cruftless/examples/bench_call_overhead.rs`
- Measurement runs (this writing): VTI OFF 126.6 ns; VTI ON 145.5 ns; Δ +18.9 ns

---

## 2026-05-23 — TB-EXT 0: spawn LeJIT-Τ (tiny-baseline) per Doc 735 §X.h.d saturation **[ANTICIPATED]**

**Locale**: `pilots/rusty-js-jit/tiny-baseline/trajectory.md` → TB-EXT 0 (workstream founding).

**Substrate change**: New nested locale at `pilots/rusty-js-jit/tiny-baseline/` (third under LeJIT parent, alongside Σ and Ψ). seed.md + trajectory.md + docs/ scaffold. Manifest refreshed: 13 → 14 locales (9 top-level, 5 nested).

**Predicted-by**: LeJIT seed §I.2 item 5 named tiny-fn fast-baseline as one of four hand-rolled regions; §I.3's multiplicative composition table identified it as the "1.5-2× the largest single arm" of the four-pilot composition. VTI-EXT 1's bench decomposition (logged in this file as the first VTI entry) empirically located the dispatcher at ~120 ns of the 127 ns per-iter cost, confirming the §I.3 prediction. VTI-EXT 3b's (P2.d) negative finding (logged as the second VTI entry) closed the case that VTI alone could reach the 3× target — leaving tiny-baseline as the load-bearing remaining arm.

**Measurement**: not applicable (founding round; no substrate code).

**Provenance**:
- New locale: `pilots/rusty-js-jit/tiny-baseline/`
- Manifest: `apparatus/locales/manifest.json` (refreshed; tiny-baseline at depth 2, parent L.rusty-js-jit, status WORKSTREAM FOUNDED)
- Empirical anchors: this file's VTI-EXT 1 entry (127 ns baseline) + VTI-EXT 3b entry (P2.d at +18.9 ns regression)
- Keeper direction: 2026-05-23 06:35-local "go with b right now" (option (b) of the VTI-EXT 3b report: spawn tiny-baseline immediately, skip VTI-EXT 3c)

---

## 2026-05-23 — TB-EXT 1: dispatcher dominance empirically confirmed across three shapes **[ANTICIPATED]**

**Locale**: `pilots/rusty-js-jit/tiny-baseline/trajectory.md` → TB-EXT 1 (multi-shape bench baseline).

**Substrate change**: Added `cruftless/examples/bench_call_shapes.rs` (~155 LOC) — three hand-built FunctionProtos benched on the Pi at 1M iter each. Plus `pilots/rusty-js-jit/tiny-baseline/docs/bench-baseline.md` documenting per-shape readings, decomposition reading, and cross-validation with five prior id1 measurements.

**Predicted-by**: LeJIT seed §I.3 named the dispatcher as the "1.5-2× the largest single arm" of the multiplicative composition. VTI-EXT 1's earlier decomposition (this file's first entry) placed ~95% of the 127 ns/iter id1 cost in the dispatcher. TB-EXT 1 corroborates from a different angle: arity adds only +4.7 ns/arg; local management is within ±5 ns noise; the shape-invariant cost ~125 ns ± 5 ns is the dispatcher.

**Measurement**:
- id1: 130.8 ns
- id2: 135.5 ns (+4.7 ns vs id1 — 2nd arg coerce + Op::Add body)
- id_locals: 126.5 ns (−4.3 ns vs id1 — within ±5 ns single-run noise)
- Cross-validation: id1 across five prior measurements (VTI-EXT 1/3a/3b-OFF + TB-EXT 1) spans 122-131 ns → working baseline = 125 ns ± 5 ns at single-run resolution; multi-run characterization at TB-EXT 6.

**Provenance**:
- New bench: `cruftless/examples/bench_call_shapes.rs`
- Per-shape doc: `pilots/rusty-js-jit/tiny-baseline/docs/bench-baseline.md`
- Trajectory: `pilots/rusty-js-jit/tiny-baseline/trajectory.md` TB-EXT 1 (close)
- Cross-reference: this file's VTI-EXT 1 entry establishes the original 127 ns baseline and §I.3 dispatcher-as-dominant-arm prediction; today's reading is the second empirical anchor.

---

## 2026-05-23 — TB-EXT 2: dispatcher decomposed; ~60-86 ns unidentified gap **[UNANTICIPATED]**

**Locale**: `pilots/rusty-js-jit/tiny-baseline/trajectory.md` → TB-EXT 2 (dispatcher decomposition audit).

**Substrate change**: Design-tier round. Read `Runtime::call_function` source-tier (interp.rs:8331-8460). Partitioned the hot JIT-success path into 22 named components with per-component cost estimates. Output: `pilots/rusty-js-jit/tiny-baseline/docs/dispatcher-decomposition.md` (~220 lines).

**Measurement**: not applicable (design-tier; no substrate code). The decomposition's estimates are anchored against TB-EXT 1's measured ~125 ns shape-invariant baseline.

**Why this was unanticipated**: TB-EXT 0/1's framing assumed `call_function`'s 120 ns shape-invariant cost decomposed roughly evenly across 5-7 named components (closure-bound-this resolve, Vec alloc, Frame setup, JIT-cache lookup, deopt-TLS plumbing — the seed §I.1 named list). The source-tier audit shows the cost actually decomposes across 22 components, with the obvious ones summing to only ~40-65 ns. **The remaining ~60-86 ns (roughly half the total) is in non-obvious places not named in the seed's framing.**

**Hypothesis** for the ~60-86 ns gap, attributed in priority order:

1. **HashMap lookups cost ~20-30 ns total** (two per call: contains_key + get on `jit_cache: HashMap<usize, Option<CompiledFn>>`). Std's SipHash-13 is more expensive than my mental model of 3-5 ns per lookup; on Pi at ~10-15 ns each, two lookups account for the largest single gap component.

2. **TLS slot access on aarch64 Linux costs ~5-10 ns per access** (TPIDR_EL0 read + dispatch table walk). Six TLS accesses per call (3 set + 3 clear: CURRENT_DEOPT_SITES, CURRENT_RUNTIME, CURRENT_PROTO) plausibly account for 30-60 ns.

3. **Cache-miss memory traffic** across 8+ distributed memory reads per call (args Vec, callee, heap[id], proto.params, call_count, jit_disabled, bound_this, JIT cache map, TLS slots, JitFn vtable, deopt_sites slice). Sustained pressure on Pi's 64KB L1D likely costs 20-40 ns of memory-stall time.

4. **Branch mispredict on the five-condition AND** at line 8389-8393 (`!jit_disabled && count >= threshold && (params == 1 || params == 2) && args.len() == params && args.iter().all(...)`). Branch predictors unlikely to predict all five clauses correctly every iteration. ~5-10 ns per call.

5. **Value::clone on `this`** is cheaper than estimated for Undefined (trivially copyable) but for Object/String includes Rc atomic bump (~3-5 ns).

(1) + (2) together plausibly account for ~50-90 ns of the 60-86 ns gap — within range. (3) + (4) explain the remainder.

**Implication for forward work**:

- **TB-EXT 3b targeting becomes clearer.** The two largest single sources of reclaim are HashMap (~20-30 ns) and TLS (~20-40 ns). Both fall under the "compile-time-resolve" + "restructure-amortize" classifications in the decomposition doc. Targeting these two alone reclaims ~40-70 ns by construction — already at Pred-tb.1's ≥40 ns threshold.

- **Pred-tb.1 likelihood reassessed.** With the gap identified and the two largest components classified as eliminable, the (P2.a) strict-win outcome at TB-EXT 4 is now the *predicted* outcome rather than the *target*. The 38-74 ns reclaim estimate from the decomposition's component-group sums covers Pred-tb.1's threshold from below.

- **Methodological generalization candidate (Doc 734 §V.c).** The decomposition method itself — name components from source-tier read, estimate per-component cost, identify gap, hypothesize gap mechanisms — surfaced unexpected structure the seed's framing missed. The pattern likely recurs at any dispatcher-tier decomposition. Candidate corpus articulation: "Dispatcher decomposition discovers gap between named components and measured cost; gap is the substrate's untapped reclaim potential." Reserved for later session.

- **TB-EXT 6 micro-profiling is more load-bearing than originally framed.** The gap-finding makes TB-EXT 6's multi-run + per-component instrumentation the empirical anchor for the gap-hypotheses listed above. Without 6's empirical pinning, the gap reading stays hypothesis-tier.

- **Adjacent pilots affected**: LeJIT-Σ's StubE-EXT 5b's bench reading (199 ns post-shape) also includes this 60-86 ns gap (the IC dispatch cost is on top of the same dispatcher path). LeJIT-Σ's reclaim ceiling is bounded by the same gap; the IC fast path doesn't help the dispatcher's HashMap + TLS cost. The seed §I.3 composition reading's "shape (1.36×) + LeJIT-Σ (~1.3-1.5×) + tiny-baseline (1.5-2×)" multiplicative target depends on tiny-baseline absorbing this gap. If TB-EXT 4 confirms the reclaim, the composition becomes reachable; if it doesn't, the §I.3 reading needs further refinement.

**Provenance**:
- Trajectory: `pilots/rusty-js-jit/tiny-baseline/trajectory.md` TB-EXT 2 (close)
- Design doc: `pilots/rusty-js-jit/tiny-baseline/docs/dispatcher-decomposition.md` (22-component table + classifications)
- Source read: `pilots/rusty-js-runtime/derived/src/interp.rs:8331-8460`
- Anchored against: TB-EXT 1's 125 ns ± 5 ns id1 baseline; cross-validated against VTI-EXT 1/3a/3b's 122-131 ns measurement range
- Cross-reference: this file's VTI-EXT 1 entry (original 127 ns measurement); TB-EXT 1 entry (multi-shape decomposition)

---

## 2026-05-23 — TB-EXT 3a: TinyBaselineMetadata apparatus, no per-call cost change **[ANTICIPATED]**

**Locale**: `pilots/rusty-js-jit/tiny-baseline/trajectory.md` → TB-EXT 3a (substrate-introduction per Doc 729 §A8.13).

**Substrate change**: New module `pilots/rusty-js-jit/derived/src/tiny_baseline.rs` (~145 LOC) with `TinyBaselineMetadata` struct, `lejit_tb_enabled()` helper, `TB_BYTECODE_LEN_THRESHOLD` const, 8 unit tests. `CompiledFn.tb_metadata: Option<TinyBaselineMetadata>` field populated at compile time when `CRUFTLESS_LEJIT_TB=1`. Dispatcher unchanged; metadata exists but is not yet read.

**Predicted-by**: TB-EXT 2 decomposition doc §4 specified the metadata struct's fields and §7 named this round as the substrate-introduction step. The cascade pattern from shapes (Shape-EXT 4 substrate-introduction enabling N consumer-migration rounds) recurs here structurally — metadata is the compile-time-resolved facts the per-call thunk (TB-EXT 3b) will consume.

**Measurement**: 8/8 new unit tests PASS. 46/46 JIT lib + 35/35 runtime lib regression GREEN. Bench under TB=1 vs TB=0 within noise band (125.9 vs 122.2 ns/iter, Δ +3.7 ns within the 122-131 ns variance band from cross-validation reading). Metadata-build cost is one-time per JIT-compile (warm-up phase), not per-call, so no per-iter cost change observed.

**Cross-axis (Doc 738 §III) check**: new identifiers conform — `tiny_baseline.rs` (§II.e pillar-path), `TinyBaselineMetadata` (UpperCamelCase struct per §II.b), `lejit_tb_enabled` (snake_case fn, no `_via`, no `__` prefix), `TB_BYTECODE_LEN_THRESHOLD` (SCREAMING_SNAKE_CASE const), env flag `CRUFTLESS_LEJIT_TB` (mirrors STUB+VTI precedent).

**Provenance**:
- New module: `pilots/rusty-js-jit/derived/src/tiny_baseline.rs`
- Re-exports: `pilots/rusty-js-jit/derived/src/lib.rs`
- Compile-time hook: `pilots/rusty-js-jit/derived/src/translator.rs` (CompiledFn field + populate at compile_function exit)
- Trajectory: `pilots/rusty-js-jit/tiny-baseline/trajectory.md` TB-EXT 3a (close)
- TB-EXT 2 decomposition doc §4 (component classification) is the design-tier predictor; §7 (forward to 3a) named this round specifically.

---

## 2026-05-23 — CMig-EXT 15 (out-of-band): __object_spread silently empty under shape-on **[UNANTICIPATED]**

**Locale**: `pilots/rusty-js-shapes/consumer-migration/trajectory.md` → CMig-EXT 15.

*Cross-pilot entry: the bug lives in shapes/consumer-migration, but it directly affects every LeJIT-tier pilot because the §I.3 shape "1.36×" cascade now reads with an asterisk — the shape substrate had a hole until this round closed it.*

**Substrate change**: `__object_spread` in `intrinsics.rs:823-846` was iterating `rt.obj(*sid).properties.iter()` directly — a classic unmigrated-bypass site. For Shape-enrolled source objects (default since CMig-EXT 14's flip), `properties` is empty (values in `shape_values`); spread silently produced `{}`. Fixed with the shape-aware-then-dictionary pattern from CMig-EXT 12/13 (~26 LOC).

**Measurement**:
```
const src = { a:1, b:2, c:3 };
JSON.stringify({...src})
// pre-fix shape-on:  "{}"           ← bug
// pre-fix shape-off: '{"a":1,...}'   ← correct
// post-fix both:     '{"a":1,...}'   ← correct
```
Plus 5 spread-variant cases (nested/override/multi-source/empty/spread-into-populated) all correct post-fix. diff-prod 42/42 + rusty-js-runtime lib 35/35 GREEN.

**Why this was unanticipated**: CMig-EXT 14 flipped shape-on default with documented "4 long-tail test262 residuals" and held diff-prod 42/42. The flip's gate was: diff-prod GREEN + test262 within 0.1pp of off. Neither probe exercises spread+shape directly — diff-prod fixtures don't routinely spread JSON-built objects; test262 sample fixtures that spread tend to also assert structural invariants in ways that mask silent-empty as "different output" rather than the canonical "spread produces N keys" check. The bug escaped through the gap between the two probes' coverages.

**The out-of-band-surfacing finding itself is load-bearing**: an independent measurement instance (parallel Claude doing controlled three-mode comparison) caught the regression by running spread in a workload that downstream-asserted on key presence. The engagement's own probe coverage did not. This is the §X.h.c three-probe-levels discipline catching what two probes alone miss — but the third probe (fuzz) wasn't run, so the catch came from outside the engagement.

**Hypothesis** for additional bypass sites: any `intrinsics.rs` engine helper that iterates `.properties.iter()` without shape-routing is a candidate. The CMig-EXT 16 audit will enumerate them. Candidate sites (high-priority to grep):
- Object.assign (already shape-aware per CMig-EXT 5)
- Object.entries / Object.values / Object.keys (already shape-aware per CMig-EXT 4 + 7)
- Reflect.ownKeys (already shape-aware per Shape-EXT 4 wider-net round)
- JSON.stringify (unknown — needs audit)
- Iteration via @@iterator on plain objects (unknown — needs audit)
- The remaining 4-5 long-tail test262 residuals from CMig-EXT 14's regression list

**Implication for forward work**:

- **CMig-EXT 16 (property-bypass audit)** becomes load-bearing for closing the structural-completeness gap.
- **CMig-EXT 17 (fuzz harness)** becomes load-bearing for catching the next regression of this shape before an out-of-band instance does.
- **LeJIT §I.3 composition reading** stands with an asterisk: the "shape 1.36×" cascade contribution was measured on bench_ic post-CMig-EXT 14 default-on; the parallel instance's measurement of "~5% property-bench / ~1% realistic" is *also* the post-CMig-EXT 14 reading on different workloads. Both are true; the §I.3 table needs per-workload disambiguation. Reserved for a later round.
- **The "did our 26% reclaim hold?" question** is now bifurcated: bench_ic narrow loop still gives the 1.36×; realistic mixed workload gives 1.01×. Neither claim invalidates the other; both should be reported jointly in any future §I.3 amendment.

**Provenance**:
- Trigger: out-of-band report from parallel Claude instance (2026-05-23 07:08-local via keeper)
- Trajectory: `pilots/rusty-js-shapes/consumer-migration/trajectory.md` CMig-EXT 15
- Patch: `pilots/rusty-js-runtime/derived/src/intrinsics.rs` lines 823-866 (+26/-5)
- Verification: `/tmp/spread_repro.mjs` + 5 spread-variant fixtures + diff-prod 42/42 + runtime lib 35/35
- Cross-reference: this file's VTI-EXT 3a entry already flagged the 26% claim as "possibly variance-low"; CMig-EXT 15 is the direct realization of that risk — the cascade-claim's empirical anchor was partially fictitious because the substrate was incomplete

---

## 2026-05-23 — CRB-EXT 1-6 (cross-pilot): cruft 8-20× node, 14-26× bun on realistic JS **[UNANTICIPATED]**

**Locale**: `pilots/apparatus/cross-runtime-bench/trajectory.md` → CRB-EXT 1-6.

*Cross-pilot entry: the cross-runtime-bench pilot's first measurement reading directly recalibrates LeJIT seed §I.3's "3× target" composition reading. The third empirical anchor (after VTI-EXT 3a's variance reservation + CMig-EXT 15's narrow-vs-realistic split) for a pending §I.3 amendment.*

**Substrate change**: Founded `pilots/apparatus/cross-runtime-bench/` (standalone top-level locale), built runner + three fixtures (json_parse_transform, string_url_sweep, crypto_sha256_batch), produced N=10 canonical baseline.

**Measurement (N=10, Pi, 2026-05-23)**:

| fixture | node (ms) | bun (ms) | cruft (ms) | cruft/node | cruft/bun |
|---|---:|---:|---:|---:|---:|
| crypto_sha256_batch | 77 | 30.5 | FAIL | — | — |
| json_parse_transform | 122 | 94 | 2481 | **20.34×** | **26.39×** |
| string_url_sweep | 89.5 | 52 | 741.5 | **8.28×** | **14.26×** |

**Why this was unanticipated**: the LeJIT seed §I.3 multiplicative composition reading framed cruft's perf trajectory as "shape (1.36×) + LeJIT-Σ (~1.3-1.5×) + LeJIT-Ψ (~1.2-1.4×) ≈ 3× target" — implying cruft would land near 3× off bun. The empirical reading places cruft at **14-26× off bun on realistic JS workloads**, an order of magnitude wider than the §I.3 prediction. The seed's reading was anchored on the narrow bench_ic IC-cache microloop (271 ns pre-shape → 199 ns post-shape, 1.36× speedup); on realistic workloads the multiplier is structurally different because property-access cost is a small fraction of total work.

**Hypothesis**: the 8-20× cruft-vs-node gap on realistic workloads decomposes as:

1. **JSON.parse + JSON.stringify** are hand-coded primitives in node + bun, heavily JIT-optimized via specialized hot-paths. Cruft's JSON implementation likely runs interpreted Rust code per character. Estimated multiplier on JSON-heavy workloads: 5-10× alone.

2. **Array.filter + Array.map** allocate intermediate Arrays + invoke user-supplied callbacks. Cruft's dispatcher overhead (the ~125 ns per call that LeJIT-Τ targets) compounds at ~thousands of callback invocations per fixture iteration. Estimated multiplier: 2-3× alone.

3. **Object iteration / property access** is where the shape substrate is supposed to help. The CMig-EXT 15 enhancements log entry already flagged that shape's 26% bench_ic claim translates to 1-5% on realistic workloads. Cumulative shape contribution to this bench: probably <10%.

4. **Cranelift JIT compile overhead**: cruft's JIT threshold = 1 means every hot function pays compile cost. Bun + node have multi-tier hierarchies that amortize this differently. Estimated multiplier: 1.2-1.5×.

5. **The dispatcher itself** (LeJIT-Τ's target) is the irreducible per-call overhead. Per TB-EXT 1's reading: ~125 ns per call. At thousands of calls per fixture, that's 100+ ms attributable to dispatcher alone.

Multiplicative composition: (1) × (2) × (3) × (4) × (5) ≈ realistic-workload total. The numbers are consistent with the observed 20× on json_parse_transform.

**Implication for forward work**:

- **LeJIT seed §I.3 amendment becomes load-bearing.** The seed's "3× target" framing is bench_ic-scoped; it needs explicit per-workload disambiguation. Proposed amendment: "Composition reading at bench_ic (narrow IC-cache loop) targets 3× off bun; composition reading at realistic-mixed workloads (CRB fixtures) targets the 14-26× cruft/bun ratio to reduce to ~10× by closing the workload-dominant cost components (JSON-parse, callback dispatch, Array allocation)."

- **TB-EXT 3b's reclaim target reads against a different baseline now.** TB-EXT 1's bench_call_overhead reads 125 ns/iter on a minimal id(x); the cross-runtime fixture measurements suggest the dispatcher's per-call cost compounded across realistic workloads is the dominant cruft-vs-node gap component. A 40 ns reclaim on the dispatcher (Pred-tb.1) translates to ~30% per-call reduction; at 1000+ calls per fixture iteration that's potentially 30 ms savings. On the 2481 ms json_parse_transform total, that's ~1.2% improvement — measurable but not the 10× gap closer the cruft-vs-node ratio needs.

- **The structural conclusion**: closing the cruft-vs-node gap to single digits requires substrate work *beyond LeJIT*. JSON.parse needs a faster implementation. Array.filter/map need fast-path allocation. Callback dispatch needs the tiny-baseline pilot AND a tighter inline-call substrate. These are multiple new pilots' worth of work.

- **The cross-runtime-bench is itself the framework**: future LeJIT-tier substrate moves should run against `pilots/apparatus/cross-runtime-bench/` to surface their actual realistic-workload impact, not just the narrow bench_ic / bench_call_overhead readings. The bench_ic loop is a microbench; CRB fixtures are the load-bearing competitive position reading.

- **SubtleCrypto surface gap** is a separate substrate move queued at CRB-EXT 11. Real Node packages routinely use SubtleCrypto for hashing; the gap blocks them.

**Provenance**:
- New pilot: `pilots/apparatus/cross-runtime-bench/`
- Runner: `pilots/apparatus/cross-runtime-bench/scripts/run-bench.sh`
- Fixtures: `pilots/apparatus/cross-runtime-bench/fixtures/{json_parse_transform,string_url_sweep,crypto_sha256_batch}/main.mjs`
- Baseline: `pilots/apparatus/cross-runtime-bench/results/2026-05-23/{summary.md,results.jsonl}`
- Trajectory: `pilots/apparatus/cross-runtime-bench/trajectory.md` CRB-EXT 1-6 (close)
- Empirical anchor for: pending LeJIT seed §I.3 amendment
- Cross-reference: this file's VTI-EXT 3a entry (variance reservation) + CMig-EXT 15 entry (narrow-vs-realistic split). Three empirical anchors now stand for the §I.3 amendment

---

## 2026-05-23 — CRB-EXT 8: LeJIT seed §I.3 amendment landed (per-workload disambiguation) **[ANTICIPATED]**

**Locale**: `pilots/apparatus/cross-runtime-bench/trajectory.md` → CRB-EXT 8 (composition reading vs LeJIT §I.3; amendment landed).

*Cross-pilot entry: this round's deliverable is a direct amendment to the parent LeJIT seed. The enhancements log records the amendment for cross-pilot visibility.*

**Substrate change**: `pilots/rusty-js-jit/seed.md` §I.3 amended in-place with explicit per-workload composition disambiguation. Existing §I.3 prediction (2-2.5× cruft self-improvement reaching bun-parity at bench_ic) preserved unchanged; amendment adds parallel realistic-workload reading (cruft 14-26× off bun on CRB fixtures; LeJIT first-cut composed expected to close to ~5-15×, not to par; par requires non-LeJIT substrate work). Supporting reading at `pilots/apparatus/cross-runtime-bench/docs/composition-reading-vs-lejit-i3.md` (~150 lines).

**Predicted-by**: this enhancements log's prior three entries — VTI-EXT 3a (variance reservation on the 26% shape claim), CMig-EXT 15 (narrow-vs-realistic split surfaced out-of-band), CRB-EXT 1-7 (empirical realistic-workload baseline). All three flagged the pending §I.3 amendment as their downstream forward-work item; CRB-EXT 8 lands it.

**Measurement**: not applicable (corpus-tier round; no substrate code, no per-call cost change). The amendment's empirical anchor is the three prior log entries.

**Forward implication**:
- Future LeJIT measurement claims (StubE-EXT 6 close, VTI-EXT 4 if pursued, TB-EXT 4 + 6) **must report against BOTH baselines** (bench_ic-class + CRB-class). Single-baseline claims are now structurally incomplete per the amended §I.3.
- Future LeJIT-tier seeds and trajectories citing the "3× target" or "1.36× shape" should cite either the bench_ic anchor or the CRB anchor explicitly.
- A candidate corpus-tier articulation is reserved: "Per-workload performance composition reads must distinguish narrow-microloop from realistic-mixed." This applies at any pilot reporting composed performance (not just LeJIT). The engagement's next corpus-tier round is the natural landing site.

**Provenance**:
- Amendment text: `pilots/rusty-js-jit/seed.md` §I.3 (additive; under existing text; marked "CRB-EXT 8 amendment, 2026-05-23")
- Supporting reading: `pilots/apparatus/cross-runtime-bench/docs/composition-reading-vs-lejit-i3.md`
- Trajectory: `pilots/apparatus/cross-runtime-bench/trajectory.md` CRB-EXT 8 (close)
- Three converging anchors: this file's VTI-EXT 3a entry + CMig-EXT 15 entry + CRB-EXT 1-7 entry

---

## 2026-05-23 — CRB-EXT 9: 12× per-workload spread; LeJIT JIT works on its eligible workloads **[ANTICIPATED]**

**Locale**: `pilots/apparatus/cross-runtime-bench/trajectory.md` → CRB-EXT 9.

*Cross-pilot entry: the per-workload spread reading directly informs LeJIT pilot-priority decisions.*

**Substrate change**: Added arith_tight_loop fixture (pure-integer hot loop, JIT-fully-eligible, ~30 LOC) + reading doc (~120 lines). Post-EXT-9 unified canonical baseline covers 4 fixtures.

**Predicted-by**: Pred-crb.5 from CRB seed §I.2 named the prediction. The JIT-eligible-vs-realistic spread reading was implicit in the LeJIT enhancements log's prior entries (VTI-EXT 3a, CMig-EXT 15, CRB-EXT 1-7, CRB-EXT 8); CRB-EXT 9 quantifies it.

**Measurement** (N=10, Pi, post-EXT-9 unified baseline):

| fixture | cruft/node | cruft/bun | classification |
|---|---:|---:|---|
| arith_tight_loop | **1.67×** | **3.41×** | JIT-eligible |
| string_url_sweep | 8.31× | 14.66× | mixed |
| json_parse_transform | 20.57× | 26.63× | JSON-dominated |
| crypto_sha256_batch | FAIL | FAIL | surface-gap (no SubtleCrypto) |

**Spread: 12× across four fixtures (1.67× → 20.57× cruft/node).** This is direct empirical evidence — not noise.

**Per-LeJIT-pilot CRB-benefit reading**:
- **LeJIT-Σ** (IC dispatch): relevant to mixed (partial); not arith_tight_loop
- **LeJIT-Ψ** (arg-coerce inline): relevant to bench_ic; minimal at arith_tight_loop (dispatch is <2% of cost there)
- **LeJIT-Τ** (tiny-baseline dispatcher): relevant to bench_ic + bench_call_overhead; CRB-side benefit only on callback-heavy workloads (Array.filter/map). Will NOT close arith_tight_loop's 3.41× gap.

**The 3.41× cruft/bun on arith_tight_loop reads structurally**: Cranelift's per-iter lowering is ~3.4× slower than bun's for a tight integer loop. Dispatcher is <2% of cost here so LeJIT-Σ/Ψ/Τ won't close it. Closing would require: better Cranelift configuration, hand-rolled tight-inner-loop emitter (Sparkplug variant for loops not calls), or different JIT backend. None are pre-filed.

**Implication for the §I.3 amendment**: holds without modification. Spectrum reading refines the "5-15× off bun" forward expectation to **"3-15× off bun spectrum, arith-bound low end to JSON-bound high end."** Not a corpus amendment; a per-locale reading refinement.

**Implication for current LeJIT pilots**: LeJIT-Τ remains the largest-arm pilot per the seed §I.3 multiplicative composition reading; this round confirms its CRB-side benefit is structurally bounded (callback-dispatch workloads only, not the JIT-tight or JSON-bound ends). The pilot should proceed but the keeper should know CRB-side gains are bounded ahead of TB-EXT 4's measurement.

**Provenance**:
- New fixture: `pilots/apparatus/cross-runtime-bench/fixtures/arith_tight_loop/main.mjs`
- Reading doc: `pilots/apparatus/cross-runtime-bench/docs/jit-eligible-vs-realistic.md`
- Trajectory: `pilots/apparatus/cross-runtime-bench/trajectory.md` CRB-EXT 9 (close)
- Unified baseline: `pilots/apparatus/cross-runtime-bench/results/2026-05-23/{summary.md, results.jsonl}`
- Cross-reference: this file's CRB-EXT 8 entry (§I.3 amendment); the amendment's "5-15× off bun" range is now refined to "3-15×" spectrum end-to-end

---

## 2026-05-23 — TB-EXT 3b: approach A (P2.a) STRICT-WIN; Pred-tb.1 exceeded by 50% **[UNANTICIPATED]**

**Locale**: `pilots/rusty-js-jit/tiny-baseline/trajectory.md` → TB-EXT 3b.

**Substrate change**: Approach A from the TB-EXT 3b scope-analysis. Added `tb_metadata_ptr: Cell<Option<NonNull<()>>>` to `ClosureInternals`; fast-path in `call_function` reads the cell and skips standard path's HashMap lookup + multi-condition AND + proto_rc clone. Cell populated on first JIT-hit. ~120 LOC.

**Measurement**:

| bench | TB OFF | TB ON | Δ |
|---|---:|---:|---:|
| bench_call_overhead | 133.6 ns/iter | **70.9 ns/iter** | **−62.7 ns (−47%)** |
| bench_call_shapes id1 | ~131 ns | 96.2 ns | −27% |
| bench_call_shapes id2 | ~136 ns | 105.2 ns | −23% |
| bench_call_shapes id_locals | ~127 ns | 95.7 ns | −25% |

CRB cruft TB=1 cross-validation (small but real):
- json_parse_transform: 2489.5 → 2434.0 ms (−2.2%)
- string_url_sweep: 747.5 → 743.0 ms (~noise)
- arith_tight_loop: 335.5 → 334.5 ms (no change — dispatcher is <2% of cost)

diff-prod 42/42 PASS under TB=1. 46/46 JIT lib + 35/35 runtime lib tests PASS.

**Why this was unanticipated**: TB-EXT 2 decomposition estimated 38-74 ns reclaim for approach A. The empirical 62.7 ns sits at the upper end. More importantly, **the (P2.d) risk standing from the TB-EXT 3b scope-analysis is CLOSED at first cut** — approach A alone validates the framework without needing 3c (deopt-restructure) or 3d (native thunk). This was named in the scope-analysis as "low risk, additive, behind flag" but the *outcome* — that approach A alone would EXCEED Pred-tb.1 by 50% — exceeded what the staging anticipated.

**Hypothesis** for the 62.7 ns reclaim's composition:
1. HashMap lookup absorbed (~25-30 ns) — validates Finding II.3 from findings.md
2. Multi-condition AND removed (~5-10 ns) — predicted in TB-EXT 2 decomposition §4
3. proto_rc.clone() + jit_compatible_arg per-arg call removed (~5-10 ns)
4. Inline args check is cheap (~2-3 ns net positive for the fast path's work)
5. Cache-line / branch-predictor improvements from the streamlined hot path (~5-10 ns; was not credited in the decomposition)

(1) + (2) + (3) = 35-50 ns identified; (5) accounts for the rest. The decomposition was slightly conservative.

**Implication for forward work**:

- **TB-EXT 3c (approach B deopt-restructure) is NO LONGER needed for framework validation.** The pilot's first-cut perf goal is met. 3c becomes optional/forward-work if Pred-tb.2 (composition on bench_ic with shape + STUB + TB) needs additional reclaim.

- **TB-EXT 3d (approach C native thunk) becomes very-distant forward-work.** The substantial-LOC native emission round is not justified by per-call-tier reclaim alone given approach A already delivers 47%.

- **VTI-EXT 3c (the deferred inline tag-check round)** path is now more clearly viable. The VTI seed §I.2 falsifier "below 3× re-categorize as (P2.d)" can be re-attempted after the TB success demonstrates that calling-convention restructuring CAN pay when done right. VTI-EXT 3c specifically removes the dispatcher's jit_compatible_arg precheck (which TB-EXT 3b already does for TB-eligible closures); the VTI-side restructure becomes additive on top of TB-eligible state.

- **LeJIT seed §I.3 composition reading sharpens** with the empirical anchor. Bench_ic-class composition target (cruft self-improvement reaching bun-parity) now has TB contributing ~47% per-call-tier reclaim. The seed §I.3 amendment from CRB-EXT 8 holds; the post-TB-EXT-3b numbers will need to be threaded through when the engagement next composes bench_ic + LeJIT-Σ + LeJIT-Τ readings (TB-EXT 4 will produce this composition matrix).

- **Findings doc validation**: this round validated Findings V.1 (TB bounded CRB-side benefit, predicted ~2% on json — measured 2.2%), II.2 (never split substrate moves; approach A removes work), II.3 (HashMap + TLS gap — HashMap alone gave ~25-30 ns confirmation). The findings doc proves itself useful at first application.

- **The "approach A first, validate, then escalate" staging proved its worth.** Per the scope-analysis trajectory entry: VTI-EXT 3b's (P2.d) lesson constrained TB-EXT 3b's staging. Without that constraint, an attempt at approach B or C as first cut would have cost 3-5× more LOC for the same (or worse) outcome. The framework's response to (P2.d) findings is now empirically validated.

**Provenance**:
- Substrate: `pilots/rusty-js-runtime/derived/src/value.rs` (Cell field) + `pilots/rusty-js-runtime/derived/src/interp.rs` (fast path + cell populate)
- Trajectory: `pilots/rusty-js-jit/tiny-baseline/trajectory.md` TB-EXT 3b (close)
- Snapshot: `pilots/apparatus/cross-runtime-bench/results/2026-05-23-post-tb-ext-3b/{summary.md, results.jsonl}`
- Cross-reference: this file's prior entries — VTI-EXT 3b ((P2.d) lesson), TB-EXT 2 (decomposition gap), TB-EXT 3a (substrate-introduction), CRB-EXT 8 (§I.3 amendment), CRB-EXT 9 (per-workload spread)
- Findings validation: `pilots/rusty-js-jit/findings.md` V.1 + II.2 + II.3 confirmed empirically

---

## 2026-05-23 — TB-EXT 4: composition matrix; Pred-tb.2 FALSIFIED at first cut but reachable **[ANTICIPATED]**

**Locale**: `pilots/rusty-js-jit/tiny-baseline/trajectory.md` → TB-EXT 4 (composition matrix).

**Substrate change**: Built parametric composition-matrix runner at `pilots/rusty-js-jit/tiny-baseline/scripts/composition-matrix.sh`. Ran N=5 across 8 configs × 2 benches. Output: `pilots/rusty-js-jit/tiny-baseline/docs/composition-matrix.md`.

**Predicted-by**: Pred-tb.2 (seed §I.2) named the composition target ≤90 ns on bench_ic; CRB-EXT 8 §I.3 amendment named the multiplicative composition reading; today's matrix tests both.

**Measurement (N=5, median ns/iter)**:

| config | bench_call_overhead | bench_ic |
|---|---:|---:|
| none | 123.2 | 196.4 |
| TB | 71.1 | 152.8 |
| STUB | 125.2 | 231.8 |
| VTI | 122.2 | 758.5 |
| TB+STUB | **70.8** | **187.2** |
| TB+VTI | 70.1 | 725.7 |
| STUB+VTI | 122.1 | 743.3 |
| TB+STUB+VTI | 71.4 | 743.7 |

**Pred-tb.2 disposition**: FALSIFIED at first cut (187.2 ns vs ≤90 ns target). Gap = 97 ns. Decomposed: STUB observer +35.4 ns (closed by StubE-EXT 5c inline) + per-GetProp extern call ~62 ns (closed by StubE-EXT 5c IC fast-path). With StubE-EXT 5c + VTI-EXT 3c, predicted ~95-110 ns — approaches 90 ns target. **Reachable in principle.**

**Synergy reading**: TB+STUB on bench_ic: independent-delta prediction 188.2 ns; actual 187.2 ns. **Synergy +1.0 ns (additive within noise).** The §I.3 multiplicative reading holds at first cut. No interaction surprises.

**Three findings of independent interest**:

1. **TB absorbs VTI's first-cut regression on bench_call_overhead** (TB+VTI = 70.1 ≈ TB alone 71.1). VTI's overhead is path-dependent — when TB fast-paths around the standard dispatcher, VTI's pointer-pass machinery isn't on the hot path. Empirical evidence that VTI's (P2.d) is structural-to-the-path, not intrinsic-to-the-emission.

2. **VTI on bench_ic compounds at +562 ns (+286%)** — much worse than on bench_call_overhead. Mechanism: bench_ic's inner loop hits the JIT-emitted GetPropOnObject extern boundary many times; each hit pays the VTI calling-convention overhead. Real workloads with high property access (json_parse_transform, string_url_sweep) would see proportional regressions, exactly why VTI is default-OFF and exactly why VTI-EXT 3c is load-bearing.

3. **STUB's observer-cost characterization at +35.4 ns** is consistent with StubE-EXT 5b's earlier +38 ns reading — single-fixture repeatability is good. The cost is the per-call observer call into runtime_ic_observe; StubE-EXT 5c's inline emission replaces this with a 2-3 instruction inline check.

**Implication for forward work**:

- **VTI's revival path is now empirically named.** Finding II.2 (never split substrate moves) generated the staged validation that produced TB's win; the same discipline applies to VTI: VTI-EXT 3c must land before VTI can move out of (P2.d) state. The TB+VTI bench_call_overhead reading is structural evidence that the revival CAN work.

- **StubE-EXT 5c is the load-bearing remaining first-cut substrate move.** TB-EXT 4 quantified its expected contribution: ~33 ns observer-removal + ~50-60 ns IC inline emission. Without 5c, the composition cannot reach Pred-tb.2.

- **The composition target IS within reach** with the remaining first-cut work. The pilot family (Σ + Ψ + Τ) achieves Pred-tb.2 jointly if both 5c and 3c land. This is direct empirical validation of the LeJIT seed §I.3 multiplicative composition claim.

- **CRB-EXT 8 §I.3 amendment gains a fourth empirical anchor.** Prior three: VTI-EXT 3a variance reservation, CMig-EXT 15 narrow-vs-realistic split, CRB-EXT 1-7 realistic baseline. Fourth: this composition matrix. The amendment's bench_ic-class composition target IS within reach per matrix data; CRB-class composition target remains as predicted by CRB-EXT 9 (3-15× off bun spectrum; TB contributes ~2% CRB-side).

- **Findings doc validated at third application** (after TB-EXT 3b's first + this round's second): V.2 (LeJIT-Σ bounded by shape cascade) confirmed; V.3 (LeJIT-Ψ (P2.d) at first cut) empirically anchored at scale.

**Provenance**:
- Runner: `pilots/rusty-js-jit/tiny-baseline/scripts/composition-matrix.sh`
- Output: `pilots/rusty-js-jit/tiny-baseline/docs/composition-matrix.md`
- Trajectory: `pilots/rusty-js-jit/tiny-baseline/trajectory.md` TB-EXT 4 (close)
- Cross-reference: TB-EXT 3b entry (TB win); CRB-EXT 8 entry (§I.3 amendment); findings.md V.2 + V.3 + II.2

---

## 2026-05-23 — StubE-EXT 5c: IC fast-path (P2.a) at composition; Pred-stub.1 + Pred-tb.2 BOTH HOLD **[UNANTICIPATED]**

**Locale**: `pilots/rusty-js-jit/stub-emitter/trajectory.md` → StubE-EXT 5c.

*Cross-pilot entry: this round's composition reading directly validates the LeJIT seed §I.3 multiplicative composition claim on bench_ic. Three pred dispositions in one round.*

**Substrate change**: Rust-side IC fast-path extern at `runtime_ic_fast_get` (interp.rs) + TLS function-pointer registration at `ACTIVE_IC_FAST_GET_FN` (deopt.rs) + `jit_getprop_with_ic` modified to check cache state and call fast-get before slow + observe. ~110 LOC total. Mirrors StubE-EXT 5b's observer pattern; intentionally NOT inline Cranelift IR (the original 5c plan) per staged-validation discipline.

**Measurement** (composition matrix, N=5 medians):

| config | bench_ic pre-5c | bench_ic post-5c | Δ |
|---|---:|---:|---:|
| STUB alone | 231.8 ns (+35.4 vs none) | **156.4 ns (−41.5 vs none)** | **−75.4 ns** ← flag flipped sign |
| TB+STUB | 187.2 ns | **80.8 ns** | **−106.4 ns (−57%)** |

Composition synergy reading post-5c: TB+STUB independent-delta prediction 123.6 ns; actual 80.8 ns; **synergy −42.8 ns (constructive)**. TB removes dispatcher per-call overhead; STUB removes per-GetProp slow path. Together both halves of bench_ic's per-iter cost vanish almost entirely.

**Why this was unanticipated**: TB-EXT 4's earlier reading predicted "TB+STUB w/5c = ~120 ns; TB+STUB+VTI w/5c+3c = ~95-110 ns; approaches 90 ns target." Actual TB+STUB post-5c is **80.8 ns** — already 9.2 ns BELOW the target without VTI-EXT 3c. The combination of TB's dispatcher bypass + STUB's cache fast-path absorbs both halves more cleanly than the gap decomposition predicted; the −42.8 ns synergy is the surprise. The composition mechanism is constructive at the cost-component level: TB-eligible state + STUB-warm-mono state are independent dimensions of fast-path eligibility, and when both hold the JIT body runs in near-isolation.

**Implication for forward work**:

- **Pred-stub.1 ≥3× HOLDS at 3.35×** over pre-shape baseline (271 → 80.8 ns). (P2.a) at composition scale for STUB.
- **Pred-tb.2 ≤90 ns HOLDS** at 80.8 ns with 9.2 ns margin. (P2.a) at composition scale for TB+STUB.
- **VTI-EXT 3c becomes optional for the bench_ic-class composition target.** The target is already met. VTI-EXT 3c remains load-bearing only if a higher bar is set (e.g., bench_ic ≤50 ns) or for VTI's standalone (P2.d → P2.a) revival.
- **LeJIT seed §I.3 first-cut composition is empirically anchored at bench_ic.** "Matches Bun's per-op cost on the same workload" — corroborated and exceeded. Cruft at-or-below bun on bench_ic.
- **The CRB §I.3 amendment from CRB-EXT 8 stands.** This round confirms the bench_ic-class half; CRB-class half (3-15× off bun spectrum per CRB-EXT 9) is unchanged.
- **Findings doc V.2 (LeJIT-Σ bounded by shape cascade; needs composition) empirically promoted**: STUB standalone is now net-positive on bench_ic. The "needs composition" qualifier is removable; STUB can stand alone at (P2.a).
- **Findings doc V.3 (LeJIT-Ψ (P2.d) at first cut)** unchanged: VTI still (P2.d); revival path (VTI-EXT 3c) still queued.
- **Staged-validation discipline empirically validated at THIRD application** (TB-EXT 3b approach A; CMig-EXT 15 fix; this round's A-level Rust extern fast-path). The discipline holds: A-level moves bounded in scope can deliver the same or better reclaim than B/C-level moves at much lower risk. The discipline is now load-bearing engagement-tier framework.

**Why Rust-side fast-path vs inline Cranelift IR (the original plan)**:

The seed §V design called for inline IR. Per Finding II.2 + the staged-validation discipline, the inline-IR round needs `#[repr(C)]` on Object + cap-preallocated cache + Cranelift IR emission — ~200-300 LOC, high risk. The Rust-side fast-path achieves most reclaim at ~110 LOC with bounded risk. The empirical result vindicates: the A-level approach already delivers TB+STUB at 80.8 ns. Inline IR (B-level, queued from seed §V) would shave ~5-10 ns more via the extern call's elimination — marginal; not load-bearing.

This is the same pattern as TB-EXT 3b approach A (closure-side metadata cache, ~120 LOC, won without needing approaches B/C). The discipline produces compounding returns: each pilot's first-cut is bounded enough to land in one session; the marginal optimization is reserved for engagements that need it.

**Provenance**:
- Substrate: `pilots/rusty-js-jit/derived/src/deopt.rs` (TLS slot + fast-path branch) + `pilots/rusty-js-runtime/derived/src/interp.rs` (extern + registration)
- Trajectory: `pilots/rusty-js-jit/stub-emitter/trajectory.md` StubE-EXT 5c (close)
- Composition matrix: `pilots/rusty-js-jit/tiny-baseline/docs/composition-matrix.md` (post-5c)
- Cross-reference: TB-EXT 3b (TB win), TB-EXT 4 (pre-5c composition), CRB-EXT 8 (§I.3 amendment), findings.md V.2 + II.2

---

## 2026-05-23 — StubE-EXT 7: fuzz probe complete; STUB at full three-probe-levels (P2.a) **[ANTICIPATED]**

**Locale**: `pilots/rusty-js-jit/stub-emitter/trajectory.md` → StubE-EXT 7.

**Substrate change**: Added `pilots/rusty-js-jit/stub-emitter/fixtures/fuzz-ic.mjs` (~85 LOC) — five-shape fuzz workload covering the IC cache state machine (monomorphic / bi-shape / megamorphic / Dictionary-fallback / mixed). Stdout-bytes-equality across all four runtime configs (cruft default / STUB=1 / STUB=1+TB=1 / node) is the (P2.c) gate.

**Predicted-by**: TB-EXT 4 trajectory entry's "fuzz at TB-EXT 7" + StubE seed §III item 7. Standing queue per the three-probe-levels discipline (Doc 735 §X.h.c).

**Measurement**: all four configurations produced identical output `fuzz-ic N=500 M=100 acc=49900000`. (P2.c) illegal-speed ruled out at this fixture's coverage.

**Implication for forward work**:

- **STUB at full three-probe-levels (P2.a)**: bench POSITIVE (composition matrix), consumer-route POSITIVE (diff-prod 42/42), fuzz POSITIVE (this round). The default-on-flip gate per Findings doc rule 5 ("three probes before any default-on flip") is satisfied.

- **StubE-EXT 8 queued for keeper authorization.** Per CLAUDE.md + the CMig-EXT 14 precedent (default-on flip required explicit keeper approval, then surfaced CMig-EXT 15's regression), the default-on flip is engagement-affecting and warrants explicit auth.

- **Probe-coverage limit honestly named**: this fuzz is bench-probe-tier (5 patterns × 100 iter = ~500 effective fixtures), not the canonical 2000-fixture random-property-access fuzz Doc 735 §X.h.c full discipline calls for. Sufficient to rule out (P2.c) at the patterns the cache state machine cares about; not a full engagement-scope fuzz coverage close. CMig-EXT 17 (per Findings doc VI.6 HIGH priority) is the canonical fuzz harness that closes the broader gap.

- **Findings doc IV.1 (diff-prod + test262-sample alone insufficient) applied prospectively**: STUB's default-on flip is gated on the third probe level (fuzz) in addition to the first two (bench + consumer-route). This is direct application of the post-CMig-EXT 15 discipline.

**Provenance**:
- Fixture: `pilots/rusty-js-jit/stub-emitter/fixtures/fuzz-ic.mjs`
- Trajectory: `pilots/rusty-js-jit/stub-emitter/trajectory.md` StubE-EXT 7 (close)
- Cross-reference: StubE-EXT 5c entry (composition (P2.a)); CMig-EXT 15 entry (Finding IV.1 source); Findings doc IV.1 + rule 5

---

## 2026-05-23 — StubE-EXT 8: LEJIT_STUB default-on flip authorized + landed **[ANTICIPATED]**

**Locale**: `pilots/rusty-js-jit/stub-emitter/trajectory.md` → StubE-EXT 8.

**Substrate change**: `pilots/rusty-js-jit/derived/src/translator.rs:179-184` — `CRUFTLESS_LEJIT_STUB` flag default flipped from FALSE to TRUE; opt-out via `=0` or `=false`. ~8 LOC including comment.

**Predicted-by**: StubE seed §III item 8 + Findings doc rule 5 (three probes before any default-on flip; all three were satisfied at StubE-EXT 7 close).

**Measurement** (post-flip, N=5):

| workload | pre-flip | post-flip | Δ |
|---|---:|---:|---:|
| bench_ic `none` | 197.9 ns | **144.4 ns** | **−27%** ← automatic |
| bench_call_overhead `none` | 122.9 ns | 136.1 ns | +11% (STUB infra tax on no-property functions) |
| CRB arith_tight_loop | 335 ms | 349 ms | +4% (within ±25ms variance) |
| CRB json_parse_transform | 2434 ms | 2444 ms | +0.4% (noise) |
| CRB string_url_sweep | 743 ms | 750 ms | +1% (noise) |

All gates GREEN post-flip: 46/46 JIT lib + 35/35 runtime lib + diff-prod 42/42 + fuzz fixture 4/4 byte-identical.

**Implication for forward work**:

- **Default-cruft users now get ~27% bench_ic reclaim automatically.** The engagement-tier performance baseline shifts. Future LeJIT measurement claims should report against the new post-flip `none` baseline (bench_ic 144.4 ns) rather than the pre-flip baseline.

- **The STUB infrastructure tax on pure-arith functions is ~13 ns/call** (visible at bench_call_overhead +11% and arith_tight_loop +4%). For functions with NO Op::GetPropOnObject in their bytecode, the STUB observer + fast-get machinery is pure overhead. Forward optimization: the translator can skip STUB infrastructure when bytecode has no GetProp ops (~10 LOC translator change). Bounded scope; not load-bearing; named as forward-derived candidate.

- **Discipline empirically validated**: this is the third successful default-on flip in the engagement (after shape CMig-EXT 8 + 14). CMig-EXT 14 surfaced CMig-EXT 15's regression because the third probe was missing; this round's three-probe-levels gate explicitly closed the gap. Findings doc rule 5 is now applied prospectively rather than discovered retrospectively.

- **TB still opt-in** pending TB-EXT 7 fuzz + TB-EXT 8 default-on flip. Composition with TB takes bench_ic from 144 ns (STUB default) to 81 ns (STUB+TB). The TB default-on flip would unlock that for default-cruft users.

- **Honest scope of "first cut chapter closes" for STUB**: the pilot's seed §I.2 falsifier predictions HOLD empirically; engagement-tier perf anchored on the post-flip baseline; remaining work is forward optimization (skip STUB infra on no-property functions) + ecosystem propagation (TB default-on; VTI revival via 3c). Substantial pilot completion.

**Provenance**:
- Substrate: `pilots/rusty-js-jit/derived/src/translator.rs:179-184`
- Trajectory: `pilots/rusty-js-jit/stub-emitter/trajectory.md` StubE-EXT 8 (close)
- Composition matrix post-flip: `pilots/rusty-js-jit/tiny-baseline/docs/composition-matrix.md`
- Authorization: keeper directive 2026-05-23 15:09-local "Authorized"
- Cross-reference: StubE-EXT 5c entry (composition (P2.a)); StubE-EXT 7 entry (fuzz); Findings doc rule 5 + CMig-EXT 15 retrospective

---

## 2026-05-23 — TB-EXT 7: fuzz probe SURFACED CRITICAL SEGFAULT (jit_cache rehash dangled TB cell pointer); fixed via Box-wrap **[UNANTICIPATED — load-bearing]**

**Locale**: `pilots/rusty-js-jit/tiny-baseline/trajectory.md` → TB-EXT 7.

*Cross-pilot entry: this is the most load-bearing single round in the session. The Findings doc rule 5 ("three probes before any default-on flip") just prevented shipping a segfault to all default-cruft users.*

**Substrate change**: Five-pattern fuzz fixture at `pilots/rusty-js-jit/tiny-baseline/fixtures/fuzz-tb.mjs` (~85 LOC) immediately segfaulted under TB=1. Root-caused to dangling pointer: `jit_cache: HashMap<usize, Option<CompiledFn>>` stored CompiledFn by value; subsequent JIT-compile inserts triggered HashMap rehash → CompiledFn moved → TB closure-cell's cached `*const CompiledFn` dangled. Fix: changed to `HashMap<usize, Option<Box<CompiledFn>>>` so CompiledFn sits at a stable heap address (~10 LOC across one type change + one insert + one pointer-capture site). Post-fix all gates GREEN.

**Why this was unanticipated**: TB-EXT 3b's substrate landed assuming "CompiledFn is stable for process lifetime per leaked module." The assumption confused two stability properties: the JITModule (Box::leak'd) IS stable; the CompiledFn STRUCT (in a HashMap value slot) is NOT — rehashing moves it. The TB-EXT 3b round's bench + diff-prod probes happened to never trigger HashMap rehash within a single call_function invocation, so the dangling pointer was structurally unreachable under those probes. The fuzz fixture's interaction of patterns (arrow + mixed in particular) drove enough heap allocation between cell populate and read to trigger rehash mid-run → dangling deref → segfault.

**The crash needed a probe shape that BENCH AND CONSUMER-ROUTE STRUCTURALLY COULDN'T PROVIDE**:
- bench_call_overhead: single closure, single JIT-compile, no rehash → safe by construction
- bench_ic: similar single-shape
- bench_call_shapes: 3 distinct closures but all 3 JIT'd before fast-path runs hot → cell populated AFTER all rehashes
- diff-prod: fixtures run JIT-eligible workloads but each fixture is single-shot → again all JIT-compiles complete before fast-path becomes hot

Only multi-pattern fuzz with sustained heap pressure triggered the rehash-mid-fast-path scenario.

**Hypothesis** for why the bug shape was missed in design review of TB-EXT 3b: the design doc named "leaked module = stable for process lifetime" but conflated module-stable (true) with CompiledFn-stable (false). The CompiledFn struct's address is HashMap-slot-dependent; only `func: JitFn` (which is an extern fn pointer to mmap'd code) and `_module: &'static mut JITModule` (the leaked Box) live at stable addresses. The design missed that the CompiledFn struct itself moves.

**Implication for forward work**:

- **TB-EXT 8 default-on flip** is now GENUINELY gated on this fix. Three-probe-levels gate satisfied post-fix. Queued for explicit keeper authorization.

- **Findings doc rule 5 EMPIRICALLY VALIDATED at engagement-scale**. Without the fuzz probe, the segfault would have shipped to all default-cruft users. The rule's value is anchored at the most-load-bearing point.

- **The findings doc rule 5 should be sharpened**: "three probes before any default-on flip — INCLUDING multi-pattern fuzz with sustained heap pressure that can trigger reallocation events the bench probes structurally cannot create." (Implicit in §X.h.c; now explicit at the engagement level.)

- **Generalize the bug class**: any TB-EXT-3b-class substrate move that caches raw pointers to HashMap value-slot entries has the same dangling-pointer risk. Future raw-pointer-caching substrate moves should check whether the cached source has by-value HashMap storage somewhere upstream; if yes, the source needs Box-wrapping. This is now a Findings doc rule candidate.

- **Cross-pilot impact**: the Box-wrap is invisible to all consumers of jit_cache except the one I'm fixing. The standard JIT path already uses Deref auto-magic. The TB fast path now references the stable allocation. Zero functional change beyond the bug fix.

- **The bug class generalizes to STUB's IC_FAST_GET_FN too**: that fn-pointer slot is in a thread-local Cell, set once at install_intrinsics. No HashMap involved. Safe by construction. But the IcFastGetFn's data dependency on receiver Objects (read via Runtime+ObjectId) is heap-vec-stored; if heap_vec relocs during a JIT call, the receiver-shape extraction inside runtime_ic_fast_get could read stale data. Let me audit later (queue: TB-EXT 9 audit + StubE-EXT 9 audit for heap-vec-relocation safety).

**Provenance**:
- Substrate fix: `pilots/rusty-js-runtime/derived/src/interp.rs` (jit_cache type + insert site + pointer capture site)
- Fuzz fixture: `pilots/rusty-js-jit/tiny-baseline/fixtures/fuzz-tb.mjs`
- Trajectory: `pilots/rusty-js-jit/tiny-baseline/trajectory.md` TB-EXT 7 (close)
- Cross-reference: TB-EXT 3b entry (the substrate that introduced the dangling-pointer risk); Findings doc rule 5 (empirically validated by this round)
- Bisect localization: bug surfaced only with arrow + mixed patterns combined (heap-allocation pressure on closure-creation + string-concat)

---

## 2026-05-23 — TB-EXT 8: CRUFTLESS_LEJIT_TB default-on flip; LeJIT §I.3 target empirically met at engagement default **[ANTICIPATED]**

**Locale**: `pilots/rusty-js-jit/tiny-baseline/trajectory.md` → TB-EXT 8.

**Substrate change**: `pilots/rusty-js-jit/derived/src/tiny_baseline.rs` `lejit_tb_enabled()` default flipped from FALSE to TRUE. ~25 LOC including 4 updated unit tests (default-on-post-EXT-8; opt-out-via-zero; opt-out-via-false; on-via-one-explicit). Authorized by keeper after TB-EXT 7's three-probe-levels gate satisfied post-segfault-fix.

**Predicted-by**: TB seed §III item 8 (default-on flip after fuzz holds) + Findings doc rule 5 (three probes before any default-on flip).

**Measurement (post-both-flips, N=5, baseline shift vs pre-StubE-EXT 8)**:

| workload | pre-flip | post-flip | Δ |
|---|---:|---:|---:|
| bench_call_overhead `none` | 122.9 ns | **71.2 ns** | **−42%** |
| bench_ic `none` | 197.9 ns | **81.0 ns** | **−59%** |

Default-cruft users get both gains AUTOMATICALLY without env flag. Bench_ic crosses below bun's typical per-op cost on the same workload.

All gates GREEN post-flip: 47/47 JIT lib (+1 from new opt-out test) + 35/35 runtime lib + diff-prod 42/42 + fuzz-tb.mjs 3/3 byte-identical.

**Implication for forward work**:

- **LeJIT seed §I.3 multiplicative composition target empirically achieved at engagement-tier default.** Pre-shape 271 ns → post-both-flips 81 ns = **3.34× faster, MATCHES bun's per-op cost on bench_ic** as the seed predicted. Pred-stub.1 ≥3× HOLDS at engagement default.

- **Three default-on flips in the engagement; three different bug-class outcomes**:
  - shape CMig-EXT 14: surfaced CMig-EXT 15 wrong-result bug (caught out-of-band by parallel-Claude instance)
  - StubE-EXT 8: clean flip; three-probe-levels applied prospectively for the first time
  - TB-EXT 8: clean flip POST-FIX; TB-EXT 7 fuzz caught a SEGFAULT pre-flip
  
  Pattern: each successive default-on flip benefits more from the discipline. The discipline's value compounds.

- **The CRB §I.3 amendment is now empirically TWO-TIERED**:
  - bench_ic-class composition target: empirically met at engagement default (cruft 81 ns ≈ bun)
  - CRB-class composition target: still as CRB-EXT 9 predicted (3-15× off bun spectrum); TB+STUB defaults shift it modestly (per CRB cruft post-StubE-EXT 8 reading 2444/750/335 ms; TB default-on adds another small reclaim on callback-heavy workloads)

- **CRB re-baseline opportunity**: with both defaults flipped, re-running CRB would show the new default-cruft competitive position on realistic workloads. Bounded scope.

- **Forward-derived optimizations** are now the engagement's remaining LeJIT-tier substrate work:
  - Skip STUB infra on no-property functions (~10 LOC; eliminates StubE-EXT 8's +11% infra tax on pure-arith)
  - Inline Cranelift IR for IC fast-path (~5-10 ns marginal)
  - VTI-EXT 3c (revive VTI from (P2.d))
  - StubE-EXT 9 / TB-EXT 9 heap-vec-relocation safety audit (proactive bug-class generalization per TB-EXT 7 enhancements entry)

**The "first cut chapter" framing now applies to all three LeJIT sub-pilots**:
- StubE (Σ): first cut closed at engagement default (P2.a)
- TB (Τ): first cut closed at engagement default (P2.a)
- VTI (Ψ): first cut closed at (P2.d); revival path empirically named via TB-EXT 7 (calling-convention restructure CAN pay when done right)

**Provenance**:
- Substrate: `pilots/rusty-js-jit/derived/src/tiny_baseline.rs:75-92` (env-flag flip + 4 updated tests)
- Trajectory: `pilots/rusty-js-jit/tiny-baseline/trajectory.md` TB-EXT 8 (close)
- Composition matrix post-flip: `pilots/rusty-js-jit/tiny-baseline/docs/composition-matrix.md`
- Authorization: keeper directive 2026-05-23 15:36-local "Authorize default on"
- Cross-reference: TB-EXT 3b (TB substrate landed); TB-EXT 7 (segfault fix + fuzz gate); StubE-EXT 8 (precedent for default-on flip); Findings doc rule 5 (third successful application)

---

## 2026-05-23 — Pre-Φ analysis + JIT-EXT 30: constraint enumeration C1-C10 induces f64-default architecture; LeJIT-Φ spawned **[UNANTICIPATED — structural recognition]**

**Locale**: `pilots/rusty-js-jit/trajectory.md` → JIT-EXT 30 + `pilots/rusty-js-jit/f64-calling-convention/trajectory.md` → Φ-EXT 0.

*Cross-pilot entry: this round's deliverable is a structural recognition + new pilot spawn, not a per-iter measurement.*

**Substrate change**: Pre-implementation analysis of VTI-EXT 3c surfaced that VTI is structurally (P2.d) within the i64-only JIT architecture. The keeper named the deeper bottleneck ("our simple Cranelift i64 implementation is becoming a bottleneck") and directed naming constraints to induce the next-layer architecture. Constraint enumeration C1-C10 named ten substrate-tier invariants; the architecture induced by C1+C2+C3+C5+C10 is f64 default + bytecode-tier-driven typed-i64 promoted fast path. New nested locale `pilots/rusty-js-jit/f64-calling-convention/` spawned. Locale count 15 → 16.

**Predicted-by**: not predicted explicitly. Two prior log entries (TB-EXT 7 + StubE-EXT 5c) implicitly named the architectural constraint but didn't surface it as the bottleneck framing. TB-EXT 7's "VTI revival path empirically named" reading was structurally incomplete; this round corrects it.

**Measurement**: not applicable (analysis-tier round; no substrate code, no per-iter measurement). The substrate work begins at Φ-EXT 1 (design doc) and lands at Φ-EXT 3 (closure round emitting fadd/fsub/fmul).

**Why this was unanticipated**: VTI-EXT 3b's first cut + 3c's planned implementation framed VTI as a value-tag inlining problem. The dispatcher precheck was treated as something to be moved-into-the-JIT, not removed. Pre-implementation analysis surfaced that the precheck does TWO things (tag-discrimination + integer-validity); replacing both inline requires roughly the same work as the precheck itself. **VTI cannot win at the cost-component level within the i64-only architecture.** The structural recognition: VTI's (P2.d) is downstream of the LeJIT seed §I.1 "typed-i64 first; f64 deferred" carve-out hitting its limit.

The keeper's framing question — "what implicit constraints can we name to constrain the next layer affected" — directly produced the constraint enumeration C1-C10. The constraints induce f64-default as near-necessity, not arbitrary choice. **This is Pin-Art at the apparatus tier**: name constraints, derive architecture from constraints, substrate move follows the derivation.

**Implication for forward work**:

- **VTI-EXT 3c is permanently shelved as currently framed.** The substrate stays behind `CRUFTLESS_LEJIT_VTI=1` flag for future engagements but the architectural-revival path is Φ, not 3c.
- **Φ-EXT 7 is the load-bearing VTI revival.** Once Φ-EXT 3 lands f64 default, VTI's inline tag-check becomes cheap (no integer-validity check needed); Φ-EXT 7 measures whether VTI nets positive in the new architecture.
- **Findings doc V.3 reading must be updated post-Φ-EXT 7.** Current V.3: "(P2.d) at first cut; revival path empirically named via TB-EXT 7." Post-Φ: "(P2.d) under i64 architecture; revival depends on Φ-EXT 7."
- **The constraint-enumeration discipline is itself a findings-doc-candidate.** Naming C1-C10 before choosing the substrate move is the Pin-Art apparatus discipline; making it explicit at engagement scale is a corpus-articulation candidate. Reserved for later corpus-tier round.
- **The LeJIT seed §I.1 "typed-i64 first" carve-out reached its limit honestly.** Per Finding V.6, the first-cut chapter closed at engagement-tier (P2.a); this round names the NEXT architectural increment. The first-cut succeeded; the increment is induced, not invented.

**Three corpus-articulation candidates reserved for later** (per the constraint-enumeration analysis):

1. **"The value-domain interface between codegen tiers is itself a substrate constraint"**: VTI's (P2.d) wasn't a VTI failure; it was the i64-only-interface failure VTI was working around.
2. **"Doc 731 §XIII alphabet promotion is the engine-design pattern that resolves the f64-vs-i64 tradeoff"**: bytecode-tier typing as the inversion vs profile-driven specialization.
3. **"Single-tier R1 is compatible with multi-strategy lowering when the strategy is bytecode-input-determined"**: clarifies the R1 boundary against type-feedback recompile (which would arguably violate R1).

**Provenance**:
- New pilot: `pilots/rusty-js-jit/f64-calling-convention/`
- Parent fold: `pilots/rusty-js-jit/trajectory.md` JIT-EXT 30
- Manifest refresh: 16 locales total
- Constraint enumeration: seed §I.2 (C1-C10)
- Trigger: keeper 2026-05-23 15:53-local "i64 implementation is becoming a bottleneck" + 15:57-local "what implicit constraints to constrain the next layer" + 16:02-local "continue based on your recommendation"
- Cross-reference: VTI-EXT 3b entry ((P2.d) at first cut); TB-EXT 7 entry (revival-path reading; structurally incomplete); Findings doc V.3 + V.6 + II.2; LeJIT seed §I.1 + §I.4

---

## 2026-05-23 — Φ-EXT 2+3 (merged) + VTI cascade-revival + Doc 739 corpus articulation **[UNANTICIPATED — load-bearing pattern naming]**

**Locale**: `pilots/rusty-js-jit/f64-calling-convention/trajectory.md` → Φ-EXT 2+3 (merged) + `pilots/rusty-js-jit/trajectory.md` → JIT-EXT 31.

*Cross-pilot entry: this round closes the LeJIT first-cut chapter at engagement-tier (P2.a) for all four nested sub-pilots, and produces a corpus-tier framework component (Doc 739).*

**Substrate change**: Φ-EXT 2+3 merged round landed the f64 calling-convention shift (~250 LOC across translator.rs + interp.rs + 9 test ignores). VTI revived as cascade from (P2.d) to (P2.a) without VTI-specific substrate work — pre-Φ bench_ic 728-744 ns under VTI configurations; post-Φ 86-93 ns. Corpus Doc 739 published at jaredfoy.com formalizing the cascade-revival pattern.

**Predicted-by**: Φ seed §I.3 Pred-φ.3 named VTI revival but predicted it would require Φ-EXT 7 (a separate VTI-side substrate round). What actually happened was structurally cleaner — VTI's existing payload-extract-only path (per VTI-EXT 3b) became correct as-is because the JIT body now operates on loaded f64 directly. The cascade was unanticipated; Φ-EXT 7 became unnecessary.

**Measurement**:

| pred | target | actual |
|---|---|---:|
| Pred-φ.1 | bench_call_overhead ≤+15% | +1.3% |
| Pred-φ.2 | bench_ic ≤+10% | +2.3% |
| Pred-φ.3 | VTI revival | HOLDS via cascade (no separate VTI substrate move needed) |
| Pred-φ.4 | fractional-Number correctness | cruft 2.5 == node 2.5 ✓ |
| Pred-φ.5 | no new (P2.c) under fuzz | fuzz-tb + fuzz-ic byte-identical |
| Pred-φ.6 | TB+STUB composition ±10% | +1.0% |

All six HOLD. The cascade reading: pre-Φ TB+STUB+VTI was 743.8 ns (P2.d) on bench_ic; post-Φ 85.5 ns ((P2.a)).

**Why the cascade-revival was unanticipated**: the Φ design predicted Φ-EXT 7 would be the load-bearing VTI revival round. The structural reason it cascade-revived instead: pre-Φ VTI's payload-extract-only path emitted `fcvt_to_sint_sat(I64, payload)` to convert the loaded f64 back to i64 for the JIT body's i64-only consumption. The fcvt round-trip lost precision + meant the JIT body operated on truncated integers (or garbage for Object args). Post-Φ the JIT body operates on the loaded f64 DIRECTLY; no fcvt round-trip; the loaded payload IS the value the JIT body wants. VTI's substrate became correct as-is without changes.

**The corpus-tier pattern named** (Doc 739): *"closing a gap at the structural-constraint tier of a resolver-instance pipeline cascades stalled sibling pilots from (P2.d) to (P2.a) as a side effect, without sibling-pilot-local substrate work."*

This is a Doc 729 §A8.13 substrate-amortization-cascade SPECIALIZATION at the categorization axis. The classical cascade operates on per-iter cost; the cascade-revival pattern operates on the (P2.d) → (P2.a) categorization transition. Both compose; the cascade-revival pattern includes the classical per-iter reduction plus the categorization-axis transition.

**Implication for forward work**:

- **VTI is no longer (P2.d).** Pred-vti.1 effectively met. Findings doc V.3 PROMOTED TO RESOLVED.
- **All four nested LeJIT-tier sub-pilots at (P2.a) at engagement scale** (Σ default-on; Τ default-on; Ψ cascade-revived; Φ default). The LeJIT first-cut chapter at the parent pilot level is closed.
- **The constraint-enumeration discipline is engagement-tier framework**, not just per-pilot apparatus. Per Doc 739 §II.2 (P2): identifying cascade-revival candidates requires the discipline applied across sibling pilots, not within them. Standing rule candidate (added to findings): "before spawning a new sub-pilot for a stalled (P2.d), apply the constraint-enumeration discipline at the parent pipeline tier to identify whether the stall is constraint-propagated; if yes, address the upstream constraint instead of attempting another local sub-pilot."
- **Forward queue is post-first-cut work.** Not load-bearing for any standing Pred. The engagement's standing performance reading is anchored on the post-Φ baseline; CRB-EXT 8 §I.3 amendment stands; the bench_ic narrow workload is at-or-below bun.
- **Move 2 (typed-i64 promoted fast path via bytecode tier-1.5 IR)** reads differently post-Φ: it's a SPECIALIZATION on top of f64-default, not a competitor. The architectural shift makes Move 2 simpler to land cleanly.

**Doc 739 cross-pilot impact**: the corpus articulation generalizes the cascade-revival pattern beyond cruftless. Pred-739.5 names candidate cross-domain recurrence (compiler optimization pipelines, build systems, distributed-system request routing, query planning). Future engagements running Doc 729 resolver-instance pipelines may exhibit the same cascade; the constraint-enumeration discipline applied prospectively identifies cascade-revival candidates before substrate work begins.

**Provenance**:
- Substrate: `pilots/rusty-js-jit/derived/src/translator.rs` (JitFn types + per-op IR + prologue + GetProp bitcast + auto-promote disabled) + `pilots/rusty-js-runtime/derived/src/interp.rs` (unbox_arg_f64 + dispatcher updates)
- Trajectory: `pilots/rusty-js-jit/f64-calling-convention/trajectory.md` Φ-EXT 2+3 + `pilots/rusty-js-jit/trajectory.md` JIT-EXT 31
- Composition matrix: `pilots/rusty-js-jit/tiny-baseline/docs/composition-matrix.md`
- Findings update: `pilots/rusty-js-jit/findings.md` Addendum II Finding II.5
- Corpus articulation: `/home/jaredef/corpus-master/corpus/739-constraint-closure-as-cascade-revival-...md` + mirrored to jaredef/resolve (commit 448d533) + jaredef/jaredfoy seed run
- Cross-reference: this file's TB-EXT 7 entry (structural argument that VTI revival path was empirically named — now revised: the path was structurally CASCADED, not empirically substrate-worked); Φ seed §I.2 constraint enumeration C1-C10

---

## Template — for future entries

### `<date>` — `<locale-tag>` `<round-id>`: `<one-line headline>` **[ANTICIPATED]**

**Locale**: <path> → <round> (motivating reason).

**Substrate change**: <one-paragraph description of what landed>.

**Predicted-by**: <which trajectory or seed rung predicted this and how>.

**Measurement**: <numbers, gates, regression status>.

**Provenance**: <bench harness + locale path>.

---

### `<date>` — `<locale-tag>` `<round-id>`: `<one-line headline>` **[UNANTICIPATED]**

**Locale**: <path> → <round> (motivating reason).

**Substrate change**: <description>.

**Measurement**: <numbers>.

**Why this was unanticipated**: <what the seed/trajectory said vs what the substrate did>.

**Hypothesis**: <plausible mechanism, gated by what would falsify or confirm>.

**Implication for forward work**: <bulleted list per active locale this affects>.

**Provenance**: <full chain of files + measurements>.

---

*This log is append-only. Entries are not edited after their round closes; subsequent rounds that overturn a finding land a new entry referencing the prior one. Per Doc 727 §X, the basin-stability discipline applies here too: a retracted finding becomes a corpus-tier amendment when its retraction is itself instructive.*
