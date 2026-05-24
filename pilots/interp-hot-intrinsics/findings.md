# interp-hot-intrinsics — Local Findings

Per Doc 737 §IV nested locale convention. Locale-scoped findings; promotions to engagement findings doc (`pilots/rusty-js-jit/findings.md` addenda) noted explicitly when they occur.

---

## Finding IHI.1 (Per-call-site IC dispatch cache must live on a Runtime-lifetime store, not per-Frame; Frame-local caches lose all amortization when each call recreates a fresh Frame) *[new, 2026-05-24 via IHI-EXT 7 empirical readout]*

**Anchor**: IHI-EXT 7 implemented a per-call-site IC dispatch cache as `Frame::ic_dispatch_cache: HashMap<usize, Option<&'static IhiEntry>>`. The intent was to eliminate the per-CallMethod table-lookup overhead (~40-80ns linear scan + string compares) on hot loops where the same call sites fire thousands of times.

**Empirical readout**: A/B header_loop went from 314 ms (IHI-EXT 5, no cache) → 337 ms (IHI-EXT 7, with cache) = **+7% worse**. The cache made performance worse, not better.

**Diagnosis**: the bench fixture's `variant()` shape invokes a fresh closure `fn(i)` PER iter (550 invocations total). Each `fn(i)` invocation creates a fresh Frame with an empty `ic_dispatch_cache`. Within fn()'s 7-CallMethod body, only 6 cache hits happen (the first CallMethod populates; the next 6 read). 7 HashMap.insert + HashMap.get operations per Frame × 550 Frame invocations = 3,850 HashMap ops + 35,000 cache reads — but each HashMap.get is ~30-50ns. Cache reads exceed the linear-scan-bypass savings.

**The structural shape**: per-Frame caches are amortized only when many CallMethods run within the same Frame's lifetime. For tight inner loops in a single Frame (e.g., `for (i=0; i<N; i++) s.charCodeAt(i)` per JSF/CharCode chain), the cache amortizes (35K hits within 1 Frame). For closure-invocation-per-iter fixtures (variant() shape; many real-world hot loops do this), the cache is reset before it can amortize.

**Substrate implication**: the cache's lifetime must span MULTIPLE Frame invocations. Three structural options:

1. **Runtime-keyed cache** — `Runtime::ic_dispatch_cache: HashMap<(bytecode_ptr, pc), Option<&'static IhiEntry>>`. Keys: (bytecode_ptr as usize, pc as usize) to disambiguate same-pc-different-proto. Survives across all Frame invocations of all functions. Per-call overhead: HashMap.get with composite key (~50-80ns). May still not beat linear scan with 4 entries.

2. **FunctionProto-side-table** — extend FunctionProto with `ic_dispatch_cache: Vec<Option<IcDispatchEntry>>` indexed by pc. Vec lookup is O(1) array index (~5ns). Cache populated lazily on first dispatch at each pc; persists for proto's lifetime. **Fastest option but requires mutable FunctionProto** (currently FunctionProto is borrowed shared); could use `OnceCell<Vec<...>>` or interior mutability.

3. **Inline bytecode rewrite** — after first dispatch at a pc, REWRITE the Op::CallMethod byte to Op::CallMethodIcCached(idx) where idx is the IcEntry index. Subsequent fetches recognize the rewritten opcode and skip lookup. Requires mutable bytecode + new opcode. Largest scope; matches V8's IC pattern.

**Recommended next step**: option 2 (FunctionProto-side-table). Vec<Option<...>> indexed by pc is O(1) + cache-friendly + persists across Frame invocations. ~30-50 LOC implementation. Expected reclaim: closes the ~12-15ms dispatch-overhead floor on string_url_sweep header loop; brings reclaim from -7.5% toward -10-12%.

**Composition with prior findings**:
- **Finding II.2-bis substrate-introduction signature**: IHI-EXT 7's +7% regression is NOT a substrate-introduction signature; it's a structural mis-match between the cache design and the bench's Frame-creation shape. Empirical fail; revert + redesign.
- **Doc 740 §VIII coverage axes**: this isn't a coverage gap; it's a per-call dispatch-overhead tier optimization. Different axis from the multi-tier reading.

**Forward implication**:
- Option 2 hardening is a future round (HI-EXT 7-equivalent at hardening tier; or a follow-on IHI-EXT 7'/8).
- The Pred-ihi.5 ≥30% target requires this hardening + for-of iteration optimization + likely 5+ entries.
- For first-cut IHI pilot's value: revert + accept -5-7.5% reclaim as the IC-table-alone ceiling; book Pred-ihi.5 as DEFERRED.

**Generalization candidate** (engagement findings doc Addendum IX candidate):

**Finding VIII.4 (proposed): Per-call-site IC caches need a cache-lifetime ≥ the hot-path's call-site-revisit frequency**. A Frame-local cache amortizes only within a single Frame; for fixtures that span Frames (closure-per-iter; method dispatch per-event-callback; etc.), the cache must live on a longer-lifetime store (Runtime, FunctionProto, or bytecode-rewrite). The cost-benefit threshold: cache amortizes when the per-site revisit count exceeds the cache-population cost / cache-read savings ratio.

For IHI's 4-entry IHI_TABLE: linear-scan cost ~50ns/call; HashMap-cache cost ~30-50ns/read; cache-population cost ~80ns. Break-even at ~3-4 revisits per cache entry. For per-Frame caches in closure-per-iter benches: 6 revisits per Frame × 550 Frames; but cache reset means each Frame is fresh ~6 revisits which is borderline. Runtime-lifetime cache: revisits accumulate across all Frames; amortizes faster.

---

*This findings.md grows as IHI-locale-specific findings surface. Finding IHI.1 documents a negative empirical result + the structural diagnosis; the closure path (option 2 FunctionProto-side-table) is queued as future hardening work.*
