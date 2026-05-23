# StubE-EXT 2 — Stub Emitter Design

*Apparatus-tier round. Chooses the cache layout, patching mechanism, state machine, and deopt-handoff for the hand-rolled aarch64 IC stub emitter. Output anchors StubE-EXT 3's scaffold against measured baseline + concrete decisions. No code change.*

**Design snapshot**: 2026-05-23, builds on `bench-baseline.md` (271 ns/iter Pi baseline).

## 1. Design summary

The LeJIT-Σ stub emitter inlines a per-IC-site 2-3-instruction shape-check + slot-load + return at every `Op::GetPropOnObject` site, replacing the current Cranelift `call jit_getprop_on_object` extern dispatch. The cache key is `(*const Shape, u32 slot)` per the shapes pilot's `Object::shape_ptr_and_slot_for` API contract (Shape-EXT 4).

```
Pre-stub (StubE-EXT 1 baseline, ~50 ns of the 271 ns/iter):
  GetPropOnObject  → Cranelift call jit_getprop_on_object → ... → return

Post-stub (LeJIT-Σ target, projected ~5-10 ns of the per-iter cost):
  GetPropOnObject  → inline shape-check + slot-load:
                       cmp  cached_shape, receiver_shape
                       b.ne stub_miss_slow_path
                       ldr  result, [receiver_values + slot * 8]
                       ret  (or fall through to next op)
                     stub_miss_slow_path:
                       (call extern; patch on return)
```

Net per-iter target: 271 → ≤90.3 ns/iter for the (P2.a) strict-win claim per Pred-stub.1.

## 2. Cache layout decision

**Chosen: side-table indexed by IC-site id (the "global slot vec" pattern).**

```
struct ICStubCache {
    sites: Vec<ICEntry>,            // indexed by ic_site_id (u32)
}
struct ICEntry {
    cached_shape: *const Shape,     // null = cold
    cached_slot: u32,                // valid iff cached_shape != null
    pinned_shape_holder: Option<Rc<Shape>>,  // keeps the Shape allocation alive
    miss_count: u32,                 // for degradation threshold
}
```

Alternative considered: inline literal in JIT-emitted code (one cached pointer + one offset patched directly into instructions per site). Rejected because:
- Inline literal requires `mprotect` between RX and RW to patch instructions, plus a full I-cache flush sequence per patch. The side-table only needs a memory store + data-cache flush (lighter).
- Cranelift's JITModule manages code memory; injecting patchable literal pools introduces a Cranelift integration concern that the side-table sidesteps.
- The extra indirection cost (one `ldr` from a known address) is ~1-2 ns on Pi — well within the budget.

**Per-IC-site id assignment**: each `Op::GetPropOnObject` parsed-op gets a unique `u32 site_id` at translator time. The id indexes into `ICStubCache.sites`. The cache grows on first parsing of each function; no per-call allocation.

**Cache lookup at IC site**:
```
adrp x_cache, ICStubCache.sites      // base of side-table
add  x_cache, x_cache, :lo12:cache
ldr  x_cached_shape, [x_cache, site_id * 24 + 0]
ldr  w_cached_slot,  [x_cache, site_id * 24 + 8]
```

The literal `site_id * 24` is patched into the Cranelift-emitted code at translator-emission time. Site IDs are assigned per JIT'd function; the cache vec grows as new functions JIT-compile.

## 3. Receiver-shape extraction

Before comparing cached shape to receiver shape, we extract the receiver's shape pointer. The receiver is a `Value` (NaN-boxed or tagged-pointer per cruftless's value encoding); the JIT already unboxes it for the extern-call path.

```
// Receiver is in x_recv. Already unboxed from Value::Object(id) to ObjectRef.
// ObjectRef is a u32 index into the heap; need to deref to get Object ptr.
ldr  x_obj_ptr, [x_heap, x_recv * 8]   // heap-table lookup
ldr  x_recv_shape, [x_obj_ptr + offset_of(Object, shape)]   // load Rc<Shape> ptr
// Rc<Shape> is Option<Rc<Shape>>; if None, recv_shape == null.
```

**Heap-table indirection cost**: ~3 ns on Pi (one load through the heap-table base; one load through the object ptr). The current extern-call path also does this; not a new cost.

**Pre-stub-rollout fast path**: if `recv_shape == null` (Object is in Dictionary form, not Shaped), the stub's shape-compare branch fails and we fall through to the extern slow-path. This handles the entire pre-CMig-EXT 8 regime correctly: every Object has `shape: None`, every stub miss takes the slow path, the extern call proceeds as today.

## 4. Patching mechanism

**Chosen: memory-store-only patching with `dsb ish` data-memory barrier.**

The side-table stores `*const Shape` + `u32` slot at known offsets. Patching is:

```
str  x_new_shape, [x_cache, site_id * 24 + 0]
str  w_new_slot,  [x_cache, site_id * 24 + 8]
dsb  ish     // data-memory barrier: ensure stores visible before next read
```

**No instruction-cache flush needed** because we're patching DATA (the side-table), not INSTRUCTIONS. The JIT-emitted lookup loads from the side-table on every IC hit; the next load picks up the patched value. The aarch64 architecture guarantees data-store visibility after `dsb ish` for subsequent reads on the same core.

This is the key design simplification vs the inline-literal alternative. Cross-core visibility is not a concern (cruftless's runtime is single-threaded per the shapes pilot `Rc<Shape>` choice; no `Arc<Shape>` cross-thread).

**Patching call site**: the extern slow path returns the new `(shape, slot)` to the stub via the standard return ABI. The stub epilogue writes the values into the side-table before returning the result to the JIT caller. Pseudo:

```
stub_miss_slow_path:
    // call extern runtime_getprop_on_object (existing path)
    bl  runtime_getprop_on_object
    // returns (value, new_shape, new_slot) — the runtime helper is
    // extended to return the shape+slot it found, not just the value.
    str x_new_shape, [x_cache, site_id * 24 + 0]
    str w_new_slot,  [x_cache, site_id * 24 + 8]
    dsb ish
    mov x_result, x_value
    ret
```

**Runtime helper extension** (lands at StubE-EXT 3): `runtime_getprop_on_object` gains an out-parameter or extended return tuple carrying `(value, *const Shape, u32 slot)`. Pre-CMig-EXT 8 it returns `(value, null, 0)` (no shape); post-enrollment it returns the receiver's shape pointer + slot index from `Object::shape_ptr_and_slot_for`.

## 5. State machine

```
COLD                — cached_shape = null. First hit → take slow path → patch with first observed (shape, slot).
WARM-MONO          — cached_shape != null. Most hits match cache → fast path returns.
COLD-AFTER-MISS    — cached_shape != null but current receiver shape differs. Slow path → patch with new shape. Transitional state.
DEGRADED           — miss_count > MISS_THRESHOLD (8). Stop patching; stub permanently routes to slow path. Avoids thrashing.
```

The cache entry tracks `miss_count`; the stub epilogue increments it on miss before patching. At `miss_count > 8`, the slow path returns without patching — the IC has decided this site is polymorphic-or-megamorphic and the patching overhead exceeds the cache benefit. Polymorphic-IC support (a side-table holding N cached shapes per site, scanned linearly) is queued as the LeJIT-Σ.poly closure round.

**Threshold rationale**: V8's IC degrades to megamorphic at 4-5 shapes per site empirically; cruftless picks 8 conservatively (room for polymorphic patterns before degrading). Tunable per StubE-EXT 6's measurement.

## 6. Deopt handoff

The stub itself never deopts. On stub miss, the slow path is the existing `runtime_getprop_on_object` extern call — that path already handles deopt-on-non-Number (per JIT-EXT 24). The deopt machinery is unchanged; LeJIT-Σ adds a cache layer in front of it.

**On hard error** (e.g., the receiver isn't even a Value::Object — `shape_ptr_and_slot_for` returns None for non-Object receivers; the stub's cached_shape check fails and falls through to the slow path; the slow path's existing deopt routes back to the interpreter): standard EXT-12 deopt thunk path.

## 7. Source-tier coordinate (Doc 738 §II)

- **Module**: `pilots/rusty-js-jit/derived/src/stub_aarch64.rs` (Doc 738 §II.e pillar-path).
- **Types**:
  - `pub struct ICStubCache` (PascalCase per Rust convention).
  - `struct ICEntry` (module-private).
  - `pub enum ICState { Cold, WarmMono, ColdAfterMiss, Degraded }` (diagnostic-only; the state is implicit in `cached_shape == null` + `miss_count`).
- **Methods**:
  - `pub fn emit_getprop_stub(builder: &mut FunctionBuilder, site_id: u32, receiver: Value, cache_ptr: i64) -> Value` (snake_case per §II.b; no `_via` suffix because this is a JIT-side emitter, not a Runtime-side dispatching helper).
  - `pub extern "C" fn ic_stub_miss(site_id: u32, recv: i64) -> (i64, *const Shape, u32)` (the slow-path entrypoint the stub calls on miss).
- **Internal sentinels**: `__ic_site_id`, `__ic_cached_shape`, `__ic_cached_slot` for any per-Object state (not used in the side-table design; reserved for future inline-literal variant).

## 8. Pre-implementation budget vs measurement

| component | est. ns (post-stub) | source |
|---|---:|---|
| Rust dispatcher (call_function) | ~120 | invariant |
| JIT preamble (arg coercion) | ~30 | invariant in StubE-EXT 5; addressable by value-tag-inline sibling pilot |
| Side-table load (cached shape + slot) | ~3 | new |
| Receiver shape load (heap-table + Object ptr deref) | ~3 | shared with extern path |
| Compare + branch | ~1 | new |
| Hit: slot load (`ldr` from values + slot*8) | ~2 | new |
| Return + reboxing | ~20 | invariant |
| **Total (cache hit)** | **~180** | per-iter |

Below the 90.3 ns/iter (P2.a) strict-win threshold? Marginal — 180 ns vs the 90.3 ns target. The ~120 ns Rust dispatcher dominates. **Without value-tag-inline + dispatcher work, the stub alone may not hit Pred-stub.1's 3× target.**

**Implication for the roadmap**: StubE-EXT 6's measurement may show (P2.d) correct-but-losing if the dispatcher overhead dominates. Two paths if so:
- **(a)** Accept partial: document the 1.5-2× speedup as a result and pivot to the value-tag-inline sibling pilot to address the JIT preamble.
- **(b)** Refactor the dispatcher: `call_function`'s closure + arg-copy + Frame setup are 120 ns of pure Rust glue; a Cranelift-emitted fast-path-only dispatcher for monomorphic-IC-hot functions could collapse this.

Option (a) is the cleaner first cut. (b) is a follow-on closure round if the corpus claim requires the full 3× from the IC layer alone — currently it doesn't; Pred-stub.1's threshold is on the IC dispatch substrate move, not on the whole call path.

**Resolution**: StubE-EXT 6 reports the measured speedup; the (P2) categorization decides the next move per the four-case rubric.

## 9. Carve-outs (re-affirmed from seed §IV)

- aarch64 only (LeJIT-Σ' handles x86_64 separately).
- Monomorphic only (one cached shape per site; LeJIT-Σ.poly handles polymorphic).
- GetProp only (SetProp + Call/CallMethod are separate closures).
- No value-tag inline checks at the stub level (sibling pilot owns that).

## 10. Forward to StubE-EXT 3

StubE-EXT 3 scaffolds `pilots/rusty-js-jit/derived/src/stub_aarch64.rs`:

```
pilots/rusty-js-jit/derived/src/
  stub_aarch64.rs    (~250 LOC estimate)
    - ICStubCache + ICEntry types
    - lazy_static thread-local cache (one per Runtime)
    - emit_getprop_stub: Cranelift IR emission for the fast path
    - extend runtime_getprop_on_object signature (lands in
      rusty-js-runtime; coordinated cross-crate work)
  tests/
    stub_aarch64_test.rs  (~150 LOC)
      - synthetic shape pointer construction
      - emit + execute stub via Cranelift JITModule
      - assert cache miss → patch → cache hit cycle works
```

Estimated total: ~400 LOC for the implementation + tests. The crate-internal types (ICStubCache, ICEntry) are private; the public surface (emit_getprop_stub) is consumed by the translator at StubE-EXT 5.

---

*StubE-EXT 2 closes. The design is anchored against the 271 ns baseline; the cache + patching shape + state machine are chosen; the source-tier coordinates conform to Doc 738 §II. StubE-EXT 3 begins next.*
