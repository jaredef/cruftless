//! Deopt infrastructure (JIT-EXT 11, Doc 731 §VII R5).
//!
//! Type machinery + thunk skeleton for the deopt mechanism. At JIT-EXT
//! 11 close, no translator emits deopt sites — the infrastructure is
//! forward investment for JIT-EXT 14+ (ICs, broader Value coverage).
//!
//! Per the deopt audit doc (`docs/deopt-audit-and-design.md`):
//!   - DeoptReason is a closed enum of trip causes
//!   - DeoptSite carries the resume_pc + live-value layout per site
//!   - JitLocation tags where each live value lives at the trip moment
//!   - jit_deopt_thunk reconstructs an interpreter-resumable state
//!     from the trip's saved registers + the site's stack map
//!   - The thunk returns a `DeoptResult` the caller dispatches on
//!     (return-value sentinel pattern, not longjmp).
//!
//! The hand-rolled approach to stack maps (rejecting Cranelift's
//! GC-shaped stackmap) means each CompiledFn carries its own
//! `Vec<DeoptSite>` indexed by `site_id`. Site ids are u32 emitted
//! inline by the translator at each deopt-emitting op; the thunk
//! receives the id as its first argument and looks up the site.

/// Closed enum of trip causes. Adding a variant is a substrate
/// decision under Pin-Art discipline — each variant corresponds to a
/// distinct in-flight assumption the JIT may have made.
///
/// At JIT-EXT 11 close, no translator emits any of these. Variants
/// are wired one at a time as their respective JIT features land:
///
/// - `IntegerOverflow`: JIT-EXT 12 (first wired demonstrator).
/// - `BoundaryArgMismatch`: JIT-EXT 13 (replaces the `jit_disabled`
///   permanent-forfeit at the dispatcher's boundary check).
/// - `ICShapeMismatch` / `ICCallTargetChanged`: JIT-EXT 14+ when ICs
///   land for GetProp / SetProp / CallMethod.
/// - `TypeWidening`: when broader Value coverage (doubles, strings)
///   gives the JIT speculation surfaces beyond integer args.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeoptReason {
    /// JIT-EXT 12: a guarded arithmetic op detected i64 overflow.
    /// The `op_pc` is the bytecode pc of the failing arithmetic op.
    IntegerOverflow { op_pc: u32 },
    /// JIT-EXT 13: the boundary guard at `call_function` would have
    /// rejected the args, but we're already inside the JIT'd function
    /// (this variant exists for future-proofing in case we ever move
    /// the boundary check inside).
    BoundaryArgMismatch,
    /// JIT-EXT 14+: an IC site's cached hidden-class no longer matches
    /// the receiver. `ic_id` identifies which IC tripped.
    ICShapeMismatch { ic_id: u32 },
    /// JIT-EXT 14+: an IC's cached callee no longer matches.
    ICCallTargetChanged { ic_id: u32 },
    /// Future: a typed-i64 assumption tripped because a local widened.
    TypeWidening { local_slot: u32 },
}

/// Where a live value lives at the moment of trip. The thunk uses
/// this to extract the value from the trip-time saved state and
/// reconstruct the interpreter's local/stack slot.
#[derive(Debug, Clone, Copy)]
pub enum JitLocation {
    /// Value is in one of the thunk's saved-argument registers.
    /// Index into the thunk's varargs (0..=7 typically).
    Register(u8),
    /// Value is on the JIT'd function's stack at the given offset
    /// from the trip-time stack pointer.
    StackSlot(i32),
    /// Value was a compile-time constant. The thunk synthesizes it
    /// without consulting saved state.
    Constant(i64),
}

/// One live-value mapping at a deopt site: what interpreter slot it
/// goes into, and where to read it from the trip-time state.
#[derive(Debug, Clone, Copy)]
pub struct DeoptLiveLocal {
    /// Interpreter local-slot index (0..proto.locals.len()) or, for
    /// stack-slot entries, the operand-stack depth (0..stack_depth).
    pub interp_slot: u16,
    pub jit_location: JitLocation,
}

/// One deopt site. The site_id is the index into `CompiledFn.deopt_sites`.
#[derive(Debug, Clone)]
pub struct DeoptSite {
    /// Reason this site exists. The thunk surfaces this in tracing.
    pub reason: DeoptReason,
    /// Bytecode pc at which the interpreter should resume execution
    /// after the trip. Typically the pc of the op that owns this
    /// deopt site, or the next op for "trip-after-effect" sites.
    pub resume_pc: u32,
    /// Live interpreter locals at the trip moment.
    pub live_locals: Vec<DeoptLiveLocal>,
    /// Operand-stack depth at the trip moment.
    pub stack_depth: u8,
    /// Live operand-stack values at the trip moment, indexed 0..stack_depth.
    pub stack_slots: Vec<DeoptLiveLocal>,
}

/// The state the thunk reconstructs for the interpreter to resume.
/// JIT-EXT 11 keeps this as a pure data structure; the dispatcher
/// (in rusty-js-runtime's interp.rs) consumes it to populate the
/// interpreter frame.
#[derive(Debug, Clone)]
pub struct DeoptRecoveredState {
    pub reason: DeoptReason,
    pub resume_pc: u32,
    /// Reconstructed local-slot values, sized to the live-locals set.
    /// Each entry is (interp_slot, raw_i64_value). The caller widens
    /// to Value::Number(f64) before populating the frame.
    pub local_values: Vec<(u16, i64)>,
    /// Reconstructed operand-stack values, in stack order (bottom first).
    pub stack_values: Vec<(u16, i64)>,
}

/// Trip-time saved state passed to the thunk. The JIT's deopt call
/// convention places the deopt site_id in `site_id` and packs up to
/// eight live values into `regs`. Sites with more than eight live
/// values use a `StackSlot` entry instead.
///
/// First cut uses a fixed-arity convention; an evolution can switch
/// to a variadic/stack-spill protocol if real sites exceed eight
/// registers (typical IC site has 2-3 live values).
#[derive(Debug, Clone, Copy)]
pub struct DeoptCallFrame {
    pub site_id: u32,
    pub regs: [i64; 8],
    /// Base of the JIT'd function's stack frame at trip time; used
    /// to resolve `JitLocation::StackSlot(offset)` reads. JIT-EXT 11
    /// keeps this as an i64 sentinel (0 = no stack reads attempted).
    pub frame_base: i64,
}

/// Look up the deopt site for a given id and reconstruct the
/// recovered state from the trip frame.
///
/// Returns `None` if the site_id is out of range (caller treats as a
/// hard failure; this should never happen if the JIT emitted the id
/// correctly).
pub fn reconstruct_state(
    sites: &[DeoptSite],
    frame: &DeoptCallFrame,
) -> Option<DeoptRecoveredState> {
    let site = sites.get(frame.site_id as usize)?;
    let local_values = site.live_locals.iter().map(|live| {
        let v = read_location(&live.jit_location, frame);
        (live.interp_slot, v)
    }).collect();
    let stack_values = site.stack_slots.iter().map(|live| {
        let v = read_location(&live.jit_location, frame);
        (live.interp_slot, v)
    }).collect();
    Some(DeoptRecoveredState {
        reason: site.reason,
        resume_pc: site.resume_pc,
        local_values,
        stack_values,
    })
}

fn read_location(loc: &JitLocation, frame: &DeoptCallFrame) -> i64 {
    match loc {
        JitLocation::Register(idx) => frame.regs.get(*idx as usize).copied().unwrap_or(0),
        // First cut: stack-slot reads are unimplemented. Sites with
        // more than 8 live values are not emittable yet; this branch
        // exists for future expansion.
        JitLocation::StackSlot(_) => 0,
        JitLocation::Constant(c) => *c,
    }
}

/// Thunk-side dispatcher entry. Called by the JIT when a deopt fires.
///
/// At JIT-EXT 11 this is a pure-Rust function: takes the trip frame,
/// looks up the recovered state, returns it. JIT-EXT 12 wires this
/// behind a Cranelift extern reference; JIT-EXT 13+ has the runtime
/// dispatcher consume `DeoptRecoveredState` to populate the
/// interpreter's locals + stack and resume at `resume_pc`.
pub fn jit_deopt_thunk(
    sites: &[DeoptSite],
    frame: DeoptCallFrame,
) -> Option<DeoptRecoveredState> {
    reconstruct_state(sites, &frame)
}

/// Tagged return value the JIT'd function uses to communicate "I
/// deopted" to its caller (the dispatcher in interp.rs).
///
/// The current first-cut JIT returns a plain i64 (the function's
/// integer result). JIT-EXT 12 evolves this: the JIT'd entry stub
/// wraps the body and inspects a side-channel deopt flag before
/// returning. If the flag is set, the caller dispatches to
/// `jit_deopt_thunk`; otherwise the i64 is the function result.
///
/// The side-channel pattern (rather than tagging the i64 itself)
/// avoids losing one bit of the result space; the JIT's hot loops
/// can use the full i64 range. JIT-EXT 11 reserves a thread-local
/// `LAST_DEOPT_FRAME` for the flag-and-frame payload.
#[derive(Debug, Clone, Copy)]
pub enum JitCallOutcome {
    /// JIT'd function returned normally; the i64 is its result.
    Returned(i64),
    /// JIT'd function tripped a deopt; the recovered state must be
    /// applied to the interpreter to resume.
    Deopted(u32 /* site_id */),
}

/// Per-CompiledFn deopt-site table. Stored on `CompiledFn` once the
/// translator starts emitting sites (JIT-EXT 12+). At JIT-EXT 11 the
/// table is always empty.
pub type DeoptSiteTable = Vec<DeoptSite>;

// -----------------------------------------------------------------------
// JIT-EXT 12: extern-callable thunk.
//
// The thunk is `extern "C"` so Cranelift-emitted code can invoke it.
// The trip records its recovered state into a thread-local and returns
// a sentinel i64 (0) to the JIT'd code, which propagates the sentinel
// back to the caller (the dispatcher in `rusty-js-runtime/interp.rs`).
// The dispatcher checks the thread-local after every JIT call.
//
// The call shape is fixed-arity for Cranelift signature simplicity:
//   site_id + 4 register slots (extendable to 8 by widening the
//   signature; the current first cut uses 4 since no current site has
//   more than 4 live values).
//
// The thunk needs the deopt-site table to translate site_id into a
// concrete DeoptSite. The dispatcher sets the table pointer in the
// per-thread `CURRENT_DEOPT_SITES` slot before invoking the JIT'd
// function; the thunk consults that slot. This avoids passing the
// pointer as a Cranelift argument (which would require declaring
// the JITed function with an extra param the translator would have
// to thread through every code path).
// -----------------------------------------------------------------------

thread_local! {
    /// Set by the dispatcher before invoking a JIT'd function; read by
    /// `deopt_trip` if the JIT'd code trips. The pointer is the
    /// CompiledFn's `deopt_sites` table. Lifetime: only valid for the
    /// duration of the JIT call; dispatcher clears after.
    pub static CURRENT_DEOPT_SITES: std::cell::Cell<*const DeoptSiteTable> = const { std::cell::Cell::new(std::ptr::null()) };

    /// Populated by `deopt_trip` when a site fires. The dispatcher
    /// consumes (takes) this after every JIT call to detect deopts.
    /// `None` after consumption.
    pub static LAST_DEOPT_FRAME: std::cell::RefCell<Option<DeoptRecoveredState>> = const { std::cell::RefCell::new(None) };
}

/// JIT-EXT 17: a process-wide flag that causes JIT-compiled functions
/// emitted under `CRUFTLESS_JIT_FORCE_SHAPE_TRIP=1` to fire an
/// `ICShapeMismatch` deopt at function entry whenever this flag is
/// true. Tests toggle it to demonstrate the deopt path for a non-
/// arithmetic deopt reason. Real IC sites in JIT-EXT 18+ will read
/// per-site cache state instead of this global flag.
///
/// Uses `AtomicBool` because the JIT-emitted code reads a fixed memory
/// address and the runtime mutates it; ordering is relaxed because the
/// flag is purely for test orchestration.
pub static JIT_FORCE_SHAPE_TRIP: std::sync::atomic::AtomicBool =
    std::sync::atomic::AtomicBool::new(false);

pub fn set_force_shape_trip(v: bool) {
    JIT_FORCE_SHAPE_TRIP.store(v, std::sync::atomic::Ordering::Relaxed);
}

pub fn get_force_shape_trip_addr() -> usize {
    &JIT_FORCE_SHAPE_TRIP as *const _ as usize
}

// ---------------------------------------------------------------------
// JIT-EXT 20: GetPropOnObject runtime-helper stub.
//
// At JIT-EXT 20 close, the helper is a deterministic stub that returns
// `(receiver_idx << 8) ^ prop_name_idx`. This proves the JIT can emit
// a two-argument extern call and read its return value. The real
// helper, which performs the actual hidden-class lookup against a
// Runtime instance, lands at JIT-EXT 21 alongside dispatcher consume-
// recovered-state (the round that wires the Runtime pointer through
// TLS so the helper can access object data).
//
// Test orchestration uses the stub to verify the call chain end-to-
// end without needing a Runtime instance:
//
//   compile bytecode containing GetPropOnObject(prop_idx)
//   invoke JIT'd function with arg = receiver_idx
//   result == (receiver_idx << 8) ^ prop_idx
// ---------------------------------------------------------------------

/// Function-pointer indirection for the GetPropOnObject helper.
///
/// The JIT crate cannot depend on `rusty-js-runtime` (it would form a
/// cycle: runtime already depends on jit for CompiledFn). The real
/// helper, which needs access to `Runtime` and `FunctionProto`, lives
/// in the runtime crate. It registers itself via
/// `set_active_getprop_fn` at startup; the JIT-emitted call lands in
/// `jit_getprop_on_object`, which consults the registered fn and
/// delegates. If no fn is registered, the deterministic stub fires
/// (used by JIT-crate-only tests).
pub type GetPropFn = extern "C" fn(i64, i64) -> i64;

thread_local! {
    static ACTIVE_GETPROP_FN: std::cell::Cell<Option<GetPropFn>> =
        const { std::cell::Cell::new(None) };
}

pub fn set_active_getprop_fn(f: GetPropFn) {
    ACTIVE_GETPROP_FN.with(|c| c.set(Some(f)));
}

pub fn clear_active_getprop_fn() {
    ACTIVE_GETPROP_FN.with(|c| c.set(None));
}

/// JIT-EXT 22: TLS slots the real helper reads.
///
/// `CURRENT_RUNTIME` is a raw `*mut Runtime` packed into a `usize`
/// (we cannot name the `Runtime` type here without a circular dep).
/// `CURRENT_PROTO` is a raw `*const FunctionProto` similarly packed.
/// Both are set by the dispatcher pre-JIT-call and cleared after.
thread_local! {
    pub static CURRENT_RUNTIME: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
    pub static CURRENT_PROTO: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

pub fn set_current_runtime(rt_ptr: usize) {
    CURRENT_RUNTIME.with(|c| c.set(rt_ptr));
}
pub fn clear_current_runtime() {
    CURRENT_RUNTIME.with(|c| c.set(0));
}
pub fn get_current_runtime() -> usize {
    CURRENT_RUNTIME.with(|c| c.get())
}

pub fn set_current_proto(proto_ptr: usize) {
    CURRENT_PROTO.with(|c| c.set(proto_ptr));
}
pub fn clear_current_proto() {
    CURRENT_PROTO.with(|c| c.set(0));
}
pub fn get_current_proto() -> usize {
    CURRENT_PROTO.with(|c| c.get())
}

#[no_mangle]
pub extern "C" fn jit_getprop_on_object(receiver_idx: i64, prop_name_idx: i64) -> i64 {
    if let Some(f) = ACTIVE_GETPROP_FN.with(|c| c.get()) {
        f(receiver_idx, prop_name_idx)
    } else {
        // JIT-crate-only tests use this deterministic stub.
        (receiver_idx << 8) ^ prop_name_idx
    }
}

/// LeJIT-Σ StubE-EXT 5b: IC observer fn pointer registered by the
/// runtime. Reads the receiver's shape pointer + slot via
/// `Object::shape_ptr_and_slot_for` and calls `observe_at_site` or
/// `observe_miss_no_shape_at_site` so the IC cache state machine
/// progresses (Cold → WarmMono → ColdAfterMiss → Degraded). Lives in
/// the runtime because it needs heap-deref + shape-field access; the
/// JIT crate doesn't depend on the runtime, so the call indirects
/// through this thread-local function pointer slot (same pattern as
/// ACTIVE_GETPROP_FN).
pub type IcObserveFn = extern "C" fn(site_id: i64, receiver_idx: i64, prop_name_idx: i64);

thread_local! {
    static ACTIVE_IC_OBSERVE_FN: std::cell::Cell<Option<IcObserveFn>> =
        const { std::cell::Cell::new(None) };
}

pub fn set_active_ic_observe_fn(f: IcObserveFn) {
    ACTIVE_IC_OBSERVE_FN.with(|c| c.set(Some(f)));
}

pub fn clear_active_ic_observe_fn() {
    ACTIVE_IC_OBSERVE_FN.with(|c| c.set(None));
}

/// LeJIT-Σ StubE-EXT 5c: IC fast-path fn pointer registered by the
/// runtime. Reads the receiver's Object via Runtime; if its current
/// shape pointer matches `cached_shape_ptr_usize`, returns
/// `Some(shape_values[cached_slot] as i64)`; else returns None.
///
/// Encoded as raw i64 so the fast-path can return Some(value as i64)
/// or None — represented as a (hit_flag, value) pair via two outputs:
/// we use a sentinel-returning convention: returns 1 << 63 (negative)
/// on miss; otherwise returns the encoded i64 value. The high-bit
/// sentinel is unambiguous because cruft's typed-i64 alphabet uses
/// the lower 53 bits (f64 mantissa range) for actual values.
pub type IcFastGetFn = extern "C" fn(
    receiver_idx: i64,
    cached_shape_ptr_usize: i64,
    cached_slot: i64,
) -> i64;

/// Sentinel returned by IcFastGetFn on miss. High-bit set; cruft's
/// typed-i64 alphabet uses lower 53 bits per Doc 731 §XIV.d.
pub const IC_FAST_MISS_SENTINEL: i64 = i64::MIN;

thread_local! {
    static ACTIVE_IC_FAST_GET_FN: std::cell::Cell<Option<IcFastGetFn>> =
        const { std::cell::Cell::new(None) };
}

pub fn set_active_ic_fast_get_fn(f: IcFastGetFn) {
    ACTIVE_IC_FAST_GET_FN.with(|c| c.set(Some(f)));
}

pub fn clear_active_ic_fast_get_fn() {
    ACTIVE_IC_FAST_GET_FN.with(|c| c.set(None));
}

/// LeJIT-Σ StubE-EXT 5b: IC-aware variant of jit_getprop_on_object.
/// Called from JIT-emitted code when the translator was invoked with
/// `CRUFTLESS_LEJIT_STUB=1`. Same value semantics as the underlying
/// helper; additionally invokes the registered IC observer with the
/// site_id so the cache state machine progresses.
///
/// Pre-EXT 5c: the cache populates but isn't read inline at the IC
/// site (the JIT-emitted code still calls this extern). Per-iter cost
/// is the existing path + observer dispatch overhead (~few ns).
/// EXT 5c adds the inline compare-branch-load fast path that consults
/// the populated cache.
#[no_mangle]
pub extern "C" fn jit_getprop_with_ic(
    site_id: i64,
    receiver_idx: i64,
    prop_name_idx: i64,
) -> i64 {
    // LeJIT-Σ StubE-EXT 5c: fast path. When the IC cache entry is
    // WarmMono with a cached (shape_ptr, slot), consult the
    // runtime's fast-get fn pointer; if the receiver's current shape
    // matches the cached shape, return the value directly without
    // running the slow object_get OR the observer. Cache miss /
    // mismatch falls through to the existing slow+observe path,
    // which transitions the cache state.
    let (cached_shape_ptr, cached_slot, is_warm_mono) =
        crate::stub_aarch64::IC_STUB_CACHE.with(|cell| {
            let cache = cell.borrow();
            let e = cache.entry(site_id as u32);
            (e.cached_shape as i64, e.cached_slot as i64,
             matches!(e.state(), crate::stub_aarch64::ICState::WarmMono))
        });
    if is_warm_mono && cached_shape_ptr != 0 {
        if let Some(fast_get) = ACTIVE_IC_FAST_GET_FN.with(|c| c.get()) {
            let v = fast_get(receiver_idx, cached_shape_ptr, cached_slot);
            if v != IC_FAST_MISS_SENTINEL {
                return v;
            }
            // Sentinel: shape mismatch or non-Number value; fall through
            // to slow+observe so the cache state transitions correctly.
        }
    }

    let result = jit_getprop_on_object(receiver_idx, prop_name_idx);
    if let Some(observe) = ACTIVE_IC_OBSERVE_FN.with(|c| c.get()) {
        observe(site_id, receiver_idx, prop_name_idx);
    }
    result
}

/// Extern thunk callable from Cranelift-emitted code. Returns a
/// sentinel i64 (0) to the JIT'd caller; the dispatcher detects the
/// trip via `LAST_DEOPT_FRAME`.
///
/// The fixed-arity signature lets Cranelift describe it as a regular
/// AbiParam list. The translator passes up to 4 live i64 register
/// values; sites with more live values are not yet emittable.
#[no_mangle]
pub extern "C" fn deopt_trip(site_id: i64, r0: i64, r1: i64, r2: i64, r3: i64) -> i64 {
    let frame = DeoptCallFrame {
        site_id: site_id as u32,
        regs: [r0, r1, r2, r3, 0, 0, 0, 0],
        frame_base: 0,
    };
    let sites_ptr = CURRENT_DEOPT_SITES.with(|c| c.get());
    if sites_ptr.is_null() {
        // No active table — defensive fallback. This should never
        // happen if the dispatcher is wired correctly.
        return 0;
    }
    // SAFETY: dispatcher guarantees the pointer is valid for the
    // duration of the JIT call.
    let sites: &DeoptSiteTable = unsafe { &*sites_ptr };
    if let Some(state) = reconstruct_state(sites, &frame) {
        LAST_DEOPT_FRAME.with(|c| *c.borrow_mut() = Some(state));
    }
    0
}

/// Set the active deopt-site table for the current thread. Caller is
/// responsible for clearing this via `clear_current_deopt_sites()`
/// after the JIT call returns.
///
/// SAFETY: `sites` must remain valid for the duration of the JIT
/// call. Typical usage: hold a reference to the CompiledFn for the
/// whole dispatch path.
pub fn set_current_deopt_sites(sites: &DeoptSiteTable) {
    CURRENT_DEOPT_SITES.with(|c| c.set(sites as *const _));
}

pub fn clear_current_deopt_sites() {
    CURRENT_DEOPT_SITES.with(|c| c.set(std::ptr::null()));
}

/// Take and return whatever deopt the last JIT call recorded.
/// `None` if no deopt was recorded.
pub fn take_last_deopt() -> Option<DeoptRecoveredState> {
    LAST_DEOPT_FRAME.with(|c| c.borrow_mut().take())
}

#[cfg(test)]
mod thunk_tests {
    use super::*;

    #[test]
    fn deopt_trip_populates_last_frame() {
        let sites = vec![DeoptSite {
            reason: DeoptReason::IntegerOverflow { op_pc: 8 },
            resume_pc: 10,
            live_locals: vec![DeoptLiveLocal {
                interp_slot: 0,
                jit_location: JitLocation::Register(0),
            }],
            stack_depth: 0,
            stack_slots: vec![],
        }];
        set_current_deopt_sites(&sites);
        let result = deopt_trip(0, 42, 0, 0, 0);
        assert_eq!(result, 0, "thunk returns sentinel 0");
        let recovered = take_last_deopt().expect("trip recorded");
        assert_eq!(recovered.resume_pc, 10);
        assert_eq!(recovered.local_values, vec![(0, 42)]);
        clear_current_deopt_sites();
    }

    #[test]
    fn deopt_trip_without_table_no_panic() {
        clear_current_deopt_sites();
        // Should not panic, just return 0.
        let result = deopt_trip(0, 0, 0, 0, 0);
        assert_eq!(result, 0);
        // No frame populated.
        assert!(take_last_deopt().is_none());
    }

    #[test]
    fn last_deopt_clears_after_take() {
        let sites = vec![DeoptSite {
            reason: DeoptReason::BoundaryArgMismatch,
            resume_pc: 0,
            live_locals: vec![],
            stack_depth: 0,
            stack_slots: vec![],
        }];
        set_current_deopt_sites(&sites);
        deopt_trip(0, 0, 0, 0, 0);
        assert!(take_last_deopt().is_some());
        assert!(take_last_deopt().is_none(), "second take returns None");
        clear_current_deopt_sites();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn site_with_locals(reason: DeoptReason, resume_pc: u32, locals: Vec<(u16, JitLocation)>) -> DeoptSite {
        DeoptSite {
            reason,
            resume_pc,
            live_locals: locals.into_iter().map(|(slot, loc)| DeoptLiveLocal {
                interp_slot: slot, jit_location: loc,
            }).collect(),
            stack_depth: 0,
            stack_slots: Vec::new(),
        }
    }

    #[test]
    fn empty_site_reconstructs_to_empty_state() {
        let site = site_with_locals(
            DeoptReason::IntegerOverflow { op_pc: 42 },
            16,
            vec![],
        );
        let frame = DeoptCallFrame { site_id: 0, regs: [0; 8], frame_base: 0 };
        let r = reconstruct_state(&[site], &frame).expect("site found");
        assert_eq!(r.reason, DeoptReason::IntegerOverflow { op_pc: 42 });
        assert_eq!(r.resume_pc, 16);
        assert!(r.local_values.is_empty());
        assert!(r.stack_values.is_empty());
    }

    #[test]
    fn register_locations_reconstruct() {
        let site = site_with_locals(
            DeoptReason::BoundaryArgMismatch,
            0,
            vec![
                (0, JitLocation::Register(0)),
                (1, JitLocation::Register(2)),
                (2, JitLocation::Constant(99)),
            ],
        );
        let frame = DeoptCallFrame {
            site_id: 0,
            regs: [100, 0, 200, 0, 0, 0, 0, 0],
            frame_base: 0,
        };
        let r = reconstruct_state(&[site], &frame).expect("site found");
        assert_eq!(r.local_values, vec![(0, 100), (1, 200), (2, 99)]);
    }

    #[test]
    fn missing_site_id_returns_none() {
        let frame = DeoptCallFrame { site_id: 5, regs: [0; 8], frame_base: 0 };
        let r = reconstruct_state(&[], &frame);
        assert!(r.is_none());
    }

    #[test]
    fn thunk_routes_to_reconstructor() {
        let site = site_with_locals(
            DeoptReason::ICShapeMismatch { ic_id: 7 },
            128,
            vec![(0, JitLocation::Register(0))],
        );
        let frame = DeoptCallFrame {
            site_id: 0,
            regs: [42, 0, 0, 0, 0, 0, 0, 0],
            frame_base: 0,
        };
        let r = jit_deopt_thunk(&[site], frame).expect("thunk recovered");
        assert_eq!(r.reason, DeoptReason::ICShapeMismatch { ic_id: 7 });
        assert_eq!(r.resume_pc, 128);
        assert_eq!(r.local_values, vec![(0, 42)]);
    }

    #[test]
    fn stack_slot_locations_reconstruct() {
        let site = DeoptSite {
            reason: DeoptReason::IntegerOverflow { op_pc: 10 },
            resume_pc: 12,
            live_locals: vec![],
            stack_depth: 2,
            stack_slots: vec![
                DeoptLiveLocal { interp_slot: 0, jit_location: JitLocation::Register(0) },
                DeoptLiveLocal { interp_slot: 1, jit_location: JitLocation::Register(1) },
            ],
        };
        let frame = DeoptCallFrame {
            site_id: 0,
            regs: [7, 11, 0, 0, 0, 0, 0, 0],
            frame_base: 0,
        };
        let r = reconstruct_state(&[site], &frame).expect("site found");
        assert_eq!(r.stack_values, vec![(0, 7), (1, 11)]);
    }

    #[test]
    fn outcome_enum_discriminates() {
        let returned = JitCallOutcome::Returned(42);
        let deopted = JitCallOutcome::Deopted(3);
        match returned {
            JitCallOutcome::Returned(v) => assert_eq!(v, 42),
            _ => panic!("wrong variant"),
        }
        match deopted {
            JitCallOutcome::Deopted(id) => assert_eq!(id, 3),
            _ => panic!("wrong variant"),
        }
    }
}
