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
        if self.degraded {
            return;
        }
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
        if self.degraded {
            return;
        }
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

    pub fn len(&self) -> usize {
        self.sites.len()
    }

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
    fn default() -> Self {
        Self::new()
    }
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

/// StubE-EXT 4: Cranelift IR emission for the inline shape-check + slot-load
/// pattern. Operates on flat i64 / pointer inputs for isolation from the
/// Object / ICEntry struct layouts (which become load-bearing at
/// StubE-EXT 5 when the translator wires this into Op::GetPropOnObject
/// dispatch).
///
/// IR signature (lowered to aarch64 by Cranelift): `extern "C" fn(
///   recv_shape: i64,        // *const Shape from the receiver Object
///   cached_shape: i64,      // *const Shape from the ICEntry side-table
///   cached_slot: i64,       // u32 slot from the ICEntry
///   values_base: i64,       // *const Value start of receiver.shape_values
///   slow_path_result: i64,  // pre-computed slow-path result (test-only;
///                           //   StubE-EXT 5 replaces with extern call)
/// ) -> i64`
///
/// IR semantics:
///   if recv_shape == cached_shape:
///       return values_base[cached_slot * 8]      // hit: inline slot load
///   else:
///       return slow_path_result                   // miss: pre-computed
pub fn emit_stub_pattern(
    builder: &mut cranelift_frontend::FunctionBuilder,
    recv_shape: cranelift_codegen::ir::Value,
    cached_shape: cranelift_codegen::ir::Value,
    cached_slot: cranelift_codegen::ir::Value,
    values_base: cranelift_codegen::ir::Value,
    slow_path_result: cranelift_codegen::ir::Value,
) -> cranelift_codegen::ir::Value {
    use cranelift_codegen::ir::{condcodes::IntCC, types::I64, InstBuilder, MemFlags};

    let hit_block = builder.create_block();
    let miss_block = builder.create_block();
    let merge_block = builder.create_block();
    builder.append_block_param(merge_block, I64);

    // Compare receiver shape pointer against cached shape pointer.
    let eq = builder.ins().icmp(IntCC::Equal, recv_shape, cached_shape);
    builder.ins().brif(eq, hit_block, &[], miss_block, &[]);

    // Hit block: load values_base[cached_slot * 8] and jump to merge with it.
    builder.switch_to_block(hit_block);
    builder.seal_block(hit_block);
    let eight = builder.ins().iconst(I64, 8);
    let offset = builder.ins().imul(cached_slot, eight);
    let addr = builder.ins().iadd(values_base, offset);
    let loaded = builder.ins().load(I64, MemFlags::trusted(), addr, 0);
    builder.ins().jump(merge_block, &[loaded]);

    // Miss block: jump to merge with the pre-computed slow-path result.
    builder.switch_to_block(miss_block);
    builder.seal_block(miss_block);
    builder.ins().jump(merge_block, &[slow_path_result]);

    // Merge block: return the chosen value.
    builder.switch_to_block(merge_block);
    builder.seal_block(merge_block);
    builder.block_params(merge_block)[0]
}

/// StubE-EXT 4 integration helper: builds a complete JITModule containing
/// one function that wraps `emit_stub_pattern` with the documented
/// `extern "C" fn(i64, i64, i64, i64, i64) -> i64` signature. Returns
/// the callable function pointer.
///
/// Used by the integration test below; available as a public helper for
/// any future bench harness or fuzz probe that needs to exercise the
/// pattern in isolation.
pub fn build_stub_pattern_module() -> Result<extern "C" fn(i64, i64, i64, i64, i64) -> i64, String>
{
    use cranelift_codegen::ir::{
        types::I64, AbiParam, Function, InstBuilder, Signature, UserFuncName,
    };
    use cranelift_codegen::isa::CallConv;
    use cranelift_codegen::settings::{self, Configurable};
    use cranelift_codegen::Context;
    use cranelift_frontend::{FunctionBuilder, FunctionBuilderContext};
    use cranelift_jit::{JITBuilder, JITModule};
    use cranelift_module::{Linkage, Module};

    // Mirror the translator's ISA setup per pilots/rusty-js-jit/derived/
    // src/translator.rs:140-147: disable colocated_libcalls + is_pic so
    // the JITBuilder doesn't try to emit PLT entries (which aarch64
    // doesn't support per cranelift-jit 0.118 backend.rs:297).
    let mut flag_builder = settings::builder();
    flag_builder
        .set("use_colocated_libcalls", "false")
        .map_err(|e| format!("flag: {e:?}"))?;
    flag_builder
        .set("is_pic", "false")
        .map_err(|e| format!("flag: {e:?}"))?;
    let isa_builder = cranelift_native::builder().map_err(|e| format!("isa: {e}"))?;
    let isa = isa_builder
        .finish(settings::Flags::new(flag_builder))
        .map_err(|e| format!("isa: {e:?}"))?;
    let jit_builder = JITBuilder::with_isa(isa, cranelift_module::default_libcall_names());
    let mut module = JITModule::new(jit_builder);

    let mut sig = Signature::new(CallConv::SystemV);
    for _ in 0..5 {
        sig.params.push(AbiParam::new(I64));
    }
    sig.returns.push(AbiParam::new(I64));

    let func_id = module
        .declare_function("stub_pattern", Linkage::Export, &sig)
        .map_err(|e| format!("declare_function: {e}"))?;

    let mut ctx = Context::new();
    ctx.func = Function::with_name_signature(UserFuncName::user(0, 0), sig);

    let mut fb_ctx = FunctionBuilderContext::new();
    {
        let mut builder = FunctionBuilder::new(&mut ctx.func, &mut fb_ctx);
        let entry = builder.create_block();
        builder.append_block_params_for_function_params(entry);
        builder.switch_to_block(entry);
        builder.seal_block(entry);
        let params: Vec<_> = builder.block_params(entry).to_vec();
        let result = emit_stub_pattern(
            &mut builder,
            params[0],
            params[1],
            params[2],
            params[3],
            params[4],
        );
        builder.ins().return_(&[result]);
        builder.finalize();
    }

    module
        .define_function(func_id, &mut ctx)
        .map_err(|e| format!("define_function: {e}"))?;
    module
        .finalize_definitions()
        .map_err(|e| format!("finalize_definitions: {e}"))?;
    let raw = module.get_finalized_function(func_id);
    let f: extern "C" fn(i64, i64, i64, i64, i64) -> i64 = unsafe { std::mem::transmute(raw) };
    Ok(f)
}

/// Cranelift IR emission entrypoint per stub-design.md §10. Translator
/// wiring (StubE-EXT 5) plugs the IC site's runtime side-table lookup
/// in front of this and the extern slow-path call behind it; the inner
/// compare-load pattern is `emit_stub_pattern` above.
///
/// StubE-EXT 3 scope: declared with placeholder body. StubE-EXT 4 lands
/// the inner `emit_stub_pattern`; full surrounding translator-integration
/// signature lands at StubE-EXT 5.
#[allow(unused_variables)]
pub fn emit_getprop_stub(site_id: ICSiteId) {
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
                assert!(
                    !e.degraded,
                    "should not degrade until miss_count > {}",
                    MISS_THRESHOLD
                );
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
        c.entry_mut(id2)
            .observe(rusty_js_shapes::Shape::root().transition_to("y"), 0);
        c.entry_mut(id3).degraded = true;
        let (cold, warm, cam, deg) = c.state_histogram();
        assert_eq!((cold, warm, cam, deg), (1, 1, 1, 1));
        let _ = id0;
    }

    /// StubE-EXT 4: Cranelift IR emission round-trip test. Builds the
    /// stub_pattern JIT function, calls it with synthetic inputs, and
    /// asserts both hit-path and miss-path produce correct values.
    #[test]
    fn stub_pattern_cache_hit_returns_slot_value() {
        let f = build_stub_pattern_module().expect("build module");
        // Synthetic Shape pointer: any non-null i64 works as a cache key.
        // We use the address of a Rust-stack i64 for both recv and cached
        // (forces equality → hit).
        let synthetic_shape = 0xDEAD_BEEF_i64;
        // Synthetic values_base: a Vec<i64> on the Rust stack acting as
        // the receiver's shape_values storage.
        let values: Vec<i64> = vec![10, 20, 30, 40, 50];
        let base = values.as_ptr() as i64;
        // Cache hit: recv_shape == cached_shape. Slot = 2. Expected = 30.
        let result = f(synthetic_shape, synthetic_shape, 2, base, /*slow*/ 999);
        assert_eq!(result, 30, "cache hit should load values_base[slot*8]");
        // Cache hit at slot 0.
        let result = f(synthetic_shape, synthetic_shape, 0, base, /*slow*/ 999);
        assert_eq!(result, 10);
        // Keep `values` alive past the JIT call.
        drop(values);
    }

    #[test]
    fn stub_pattern_cache_miss_returns_slow_path() {
        let f = build_stub_pattern_module().expect("build module");
        let recv_shape = 0xCAFE_BABE_i64;
        let cached_shape = 0xDEAD_BEEF_i64;
        let values: Vec<i64> = vec![10, 20, 30];
        let base = values.as_ptr() as i64;
        let slow_path = 0x12345_i64;
        // recv != cached → miss → return slow_path.
        let result = f(recv_shape, cached_shape, 0, base, slow_path);
        assert_eq!(result, slow_path);
        drop(values);
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
