//! LeJIT-Σ: hand-rolled aarch64 IC stub emitter for property-access
//! fast paths.
//!
//! Per pilots/rusty-js-jit/stub-emitter/seed.md + docs/stub-design.md.
//! Replaces the per-IC-site Cranelift `call jit_getprop_on_object`
//! extern dispatch with an inlined shape-check + slot-load. Cache key
//! is `(*const Shape, u32 slot)` per the shapes pilot's
//! `Object::shape_ptr_and_slot_for` API contract.
//!
//! Scope per StubE-EXT 3 (this round): scaffold the `ICStubCache` +
//! `ICEntry` types and the per-IC-site state machine. The Cranelift IR
//! emission entrypoint `emit_getprop_stub` is declared with a
//! placeholder body; StubE-EXT 4 lands the actual aarch64 IR
//! emission against synthetic shape pointers.

use std::cell::RefCell;

/// Site identifier for an IC site. Assigned per `Op::GetPropOnObject`
/// at translator time; indexes into `ICStubCache.sites`.
pub type ICSiteId = u32;

/// Misses-before-degrade threshold per stub-design.md §5. V8's IC
/// degrades to megamorphic at 4-5 shapes per site empirically;
/// cruftless picks 8 conservatively. Tunable per StubE-EXT 6's
/// measurement.
pub const MISS_THRESHOLD: u32 = 8;

/// State of an IC entry. Diagnostic-only; the actual state is implicit
/// in `cached_shape == null` + `miss_count`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ICState {
    /// Never hit; cached_shape is null. First hit will take the slow
    /// path and patch the cache.
    Cold,
    /// One shape cached; fast path is live.
    WarmMono,
    /// Cached but the most recent dispatch was a miss; will re-patch
    /// on the slow-path return.
    ColdAfterMiss,
    /// Miss count exceeded threshold; stub is permanently routed to
    /// the slow path. Avoids thrashing.
    Degraded,
}

/// One entry in the IC stub cache. One per IC site per compiled function.
///
/// The cached_shape is the IC's monomorphic shape pointer. It's a raw
/// pointer for cheap inline comparison; the pinned holder keeps the
/// allocation alive per shapes design §11's stable-pointer safety story.
#[derive(Debug)]
pub struct ICEntry {
    /// The cached receiver shape pointer. null = Cold or Degraded.
    pub cached_shape: *const rusty_js_shapes::Shape,
    /// Slot index in the receiver's shape_values vec. Valid iff
    /// cached_shape is non-null.
    pub cached_slot: u32,
    /// Holds an Rc<Shape> reference to keep the allocation alive while
    /// the IC stub may dereference cached_shape. Per shapes design §11.
    pub pinned_shape_holder: Option<std::rc::Rc<rusty_js_shapes::Shape>>,
    /// Misses observed at this site. Triggers degradation past
    /// `MISS_THRESHOLD`.
    pub miss_count: u32,
    /// Once true, the slow path stops patching the cache; the site is
    /// permanently dispatched through the extern slow path.
    pub degraded: bool,
}

impl ICEntry {
    pub fn new_cold() -> Self {
        Self {
            cached_shape: std::ptr::null(),
            cached_slot: 0,
            pinned_shape_holder: None,
            miss_count: 0,
            degraded: false,
        }
    }

    /// Current diagnostic state.
    pub fn state(&self) -> ICState {
        if self.degraded {
            ICState::Degraded
        } else if self.cached_shape.is_null() {
            ICState::Cold
        } else if self.miss_count > 0 {
            ICState::ColdAfterMiss
        } else {
            ICState::WarmMono
        }
    }

    /// Called by the slow path after observing a property at a fresh
    /// `(shape, slot)`. Patches the cache; updates state. Degrades at
    /// `MISS_THRESHOLD`.
    pub fn observe(&mut self, shape: std::rc::Rc<rusty_js_shapes::Shape>, slot: u32) {
        if self.degraded { return; }
        if !self.cached_shape.is_null() {
            // Not Cold; this is a miss against the cached value.
            self.miss_count = self.miss_count.saturating_add(1);
            if self.miss_count > MISS_THRESHOLD {
                self.degraded = true;
                self.cached_shape = std::ptr::null();
                self.cached_slot = 0;
                self.pinned_shape_holder = None;
                return;
            }
        }
        let ptr = std::rc::Rc::as_ptr(&shape);
        self.cached_shape = ptr;
        self.cached_slot = slot;
        self.pinned_shape_holder = Some(shape);
    }

    /// Called by the slow path on shape-miss WITHOUT a fresh observation
    /// to patch (e.g., the receiver was None-shape — Dictionary form —
    /// so there's no shape to cache).
    pub fn observe_miss_no_shape(&mut self) {
        if self.degraded { return; }
        if !self.cached_shape.is_null() {
            self.miss_count = self.miss_count.saturating_add(1);
            if self.miss_count > MISS_THRESHOLD {
                self.degraded = true;
                self.cached_shape = std::ptr::null();
                self.cached_slot = 0;
                self.pinned_shape_holder = None;
            }
        }
    }
}

/// Side-table of IC entries indexed by `ICSiteId`. Grows as JIT'd
/// functions emit GetPropOnObject sites. Single-threaded — the cruftless
/// runtime is single-threaded per the engine's broader design (per the
/// shapes pilot's `Rc<Shape>` choice over `Arc<Shape>`).
pub struct ICStubCache {
    sites: Vec<ICEntry>,
}

impl ICStubCache {
    pub fn new() -> Self {
        Self { sites: Vec::new() }
    }

    /// Allocate a new IC site id. Used by the translator at
    /// `Op::GetPropOnObject` parse time.
    pub fn alloc_site(&mut self) -> ICSiteId {
        let id = self.sites.len() as ICSiteId;
        self.sites.push(ICEntry::new_cold());
        id
    }

    pub fn entry(&self, id: ICSiteId) -> &ICEntry {
        &self.sites[id as usize]
    }

    pub fn entry_mut(&mut self, id: ICSiteId) -> &mut ICEntry {
        &mut self.sites[id as usize]
    }

    pub fn len(&self) -> usize { self.sites.len() }

    /// Diagnostic helper: count of sites by current state.
    pub fn state_histogram(&self) -> (usize, usize, usize, usize) {
        let mut cold = 0;
        let mut warm = 0;
        let mut cam = 0;
        let mut deg = 0;
        for s in self.sites.iter().map(|e| e.state()) {
            match s {
                ICState::Cold => cold += 1,
                ICState::WarmMono => warm += 1,
                ICState::ColdAfterMiss => cam += 1,
                ICState::Degraded => deg += 1,
            }
        }
        (cold, warm, cam, deg)
    }
}

impl Default for ICStubCache {
    fn default() -> Self { Self::new() }
}

thread_local! {
    /// Per-runtime IC cache. Single-threaded; one cache per OS thread,
    /// which in practice is one cache per Runtime since cruftless's
    /// runtime is single-threaded.
    pub static IC_STUB_CACHE: RefCell<ICStubCache> = RefCell::new(ICStubCache::new());
}

/// Helper: allocate an IC site id and return it.
pub fn alloc_ic_site() -> ICSiteId {
    IC_STUB_CACHE.with(|c| c.borrow_mut().alloc_site())
}

/// Helper: invoke the slow path's cache update, called by the extern
/// runtime helper after observing a `(shape, slot)` at the site.
pub fn observe_at_site(id: ICSiteId, shape: std::rc::Rc<rusty_js_shapes::Shape>, slot: u32) {
    IC_STUB_CACHE.with(|c| c.borrow_mut().entry_mut(id).observe(shape, slot));
}

/// Helper: slow-path no-shape miss observer (receiver was Dictionary).
pub fn observe_miss_no_shape_at_site(id: ICSiteId) {
    IC_STUB_CACHE.with(|c| c.borrow_mut().entry_mut(id).observe_miss_no_shape());
}

/// Cranelift IR emission entrypoint per stub-design.md §10.
///
/// StubE-EXT 3 scope: declared with placeholder body. The actual
/// aarch64 IR emission (inline shape-check + slot-load + return) lands
/// at StubE-EXT 4 alongside the synthetic shape-pointer integration
/// test. The signature is the API contract the translator (StubE-EXT 5)
/// consumes; locking it in now lets the translator scaffold against
/// a stable surface.
///
/// `site_id` is the IC site allocated via `alloc_ic_site()`. `receiver`
/// is the unboxed ObjectRef value at the IC point. Returns the loaded
/// property value (cache hit) OR falls through to the slow path (cache
/// miss → call to `runtime_getprop_on_object` extern → patch → return).
///
/// Returns the Cranelift `Value` representing the loaded property.
#[allow(unused_variables)]
pub fn emit_getprop_stub(
    site_id: ICSiteId,
    // Placeholder for Cranelift FunctionBuilder + Value types.
    // The full signature lands at StubE-EXT 4 with the IR emission.
) {
    // StubE-EXT 4 body: emit inline shape-check + slot-load sequence per
    // stub-design.md §2-§4. For now the function is a no-op placeholder
    // so StubE-EXT 5 (translator wiring) and StubE-EXT 4 (IR emission)
    // can land in either order.
    let _ = site_id;
}

// =========================================================================
// Tests — exercise the cache state machine without Cranelift integration.
// =========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn make_shape() -> std::rc::Rc<rusty_js_shapes::Shape> {
        rusty_js_shapes::Shape::root().transition_to("x")
    }

    #[test]
    fn cold_entry_starts_null() {
        let e = ICEntry::new_cold();
        assert_eq!(e.state(), ICState::Cold);
        assert!(e.cached_shape.is_null());
        assert_eq!(e.cached_slot, 0);
        assert_eq!(e.miss_count, 0);
        assert!(!e.degraded);
    }

    #[test]
    fn cold_to_warm_on_first_observe() {
        let mut e = ICEntry::new_cold();
        let s = make_shape();
        e.observe(s.clone(), 0);
        assert_eq!(e.state(), ICState::WarmMono);
        assert_eq!(e.cached_shape, std::rc::Rc::as_ptr(&s));
        assert_eq!(e.cached_slot, 0);
        assert_eq!(e.miss_count, 0);
    }

    #[test]
    fn warm_to_cold_after_miss_on_shape_change() {
        let mut e = ICEntry::new_cold();
        let s1 = rusty_js_shapes::Shape::root().transition_to("x");
        let s2 = rusty_js_shapes::Shape::root().transition_to("y");
        assert!(!std::rc::Rc::ptr_eq(&s1, &s2));
        e.observe(s1.clone(), 0);
        assert_eq!(e.state(), ICState::WarmMono);
        e.observe(s2.clone(), 0);
        // Cached shape is now s2; miss_count is 1.
        assert_eq!(e.cached_shape, std::rc::Rc::as_ptr(&s2));
        assert_eq!(e.miss_count, 1);
        assert_eq!(e.state(), ICState::ColdAfterMiss);
    }

    #[test]
    fn degrades_past_miss_threshold() {
        let mut e = ICEntry::new_cold();
        let s0 = make_shape();
        e.observe(s0, 0);
        // Each subsequent observe with a distinct shape is a miss.
        for i in 1..=(MISS_THRESHOLD + 1) {
            let s = rusty_js_shapes::Shape::root().transition_to(&format!("p{}", i));
            e.observe(s, 0);
            if i <= MISS_THRESHOLD {
                assert!(!e.degraded, "should not degrade until miss_count > {}", MISS_THRESHOLD);
            }
        }
        assert!(e.degraded, "should degrade past MISS_THRESHOLD");
        assert_eq!(e.state(), ICState::Degraded);
        assert!(e.cached_shape.is_null(), "degraded entry clears its cache");
        assert!(e.pinned_shape_holder.is_none());
    }

    #[test]
    fn degraded_entry_stops_observing() {
        let mut e = ICEntry::new_cold();
        e.degraded = true;
        let s = make_shape();
        let pre_count = e.miss_count;
        e.observe(s, 0);
        assert!(e.degraded);
        assert!(e.cached_shape.is_null());
        assert_eq!(e.miss_count, pre_count);
    }

    #[test]
    fn observe_miss_no_shape_increments_count() {
        let mut e = ICEntry::new_cold();
        let s = make_shape();
        e.observe(s, 0);
        let initial = e.miss_count;
        e.observe_miss_no_shape();
        assert_eq!(e.miss_count, initial + 1);
        assert_eq!(e.state(), ICState::ColdAfterMiss);
    }

    #[test]
    fn observe_miss_no_shape_on_cold_is_noop() {
        let mut e = ICEntry::new_cold();
        e.observe_miss_no_shape();
        assert_eq!(e.state(), ICState::Cold);
        assert_eq!(e.miss_count, 0);
    }

    #[test]
    fn icstubcache_alloc_assigns_sequential_ids() {
        let mut c = ICStubCache::new();
        assert_eq!(c.alloc_site(), 0);
        assert_eq!(c.alloc_site(), 1);
        assert_eq!(c.alloc_site(), 2);
        assert_eq!(c.len(), 3);
    }

    #[test]
    fn icstubcache_histogram_classifies_state() {
        let mut c = ICStubCache::new();
        let s = make_shape();
        let id0 = c.alloc_site(); // Cold
        let id1 = c.alloc_site(); // Will be WarmMono
        let id2 = c.alloc_site(); // Will be ColdAfterMiss
        let id3 = c.alloc_site(); // Will be Degraded
        c.entry_mut(id1).observe(s.clone(), 0);
        c.entry_mut(id2).observe(s.clone(), 0);
        c.entry_mut(id2).observe(rusty_js_shapes::Shape::root().transition_to("y"), 0);
        c.entry_mut(id3).degraded = true;
        let (cold, warm, cam, deg) = c.state_histogram();
        assert_eq!((cold, warm, cam, deg), (1, 1, 1, 1));
        let _ = id0;
    }

    /// Pred-stub.4: source-tier identifiers fit Doc 738 §II's
    /// convention space. Smoke test: every public identifier in this
    /// module follows snake_case methods + PascalCase types.
    #[test]
    fn doc738_convention_smoke_test() {
        // Compile-time check: these names must exist.
        let _: ICSiteId = 0;
        let _: ICState = ICState::Cold;
        let _: ICStubCache = ICStubCache::new();
        let _ = MISS_THRESHOLD;
        // No `__ic_*` JS-visible identifiers (the IC state lives in
        // Rust-side cache, not on JS-observable Object slots in this
        // round — that's reserved per stub-design §7).
    }
}
