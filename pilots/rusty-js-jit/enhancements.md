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
- Manifest: `scripts/locales/manifest.json` (refreshed; tiny-baseline at depth 2, parent L.rusty-js-jit, status WORKSTREAM FOUNDED)
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
