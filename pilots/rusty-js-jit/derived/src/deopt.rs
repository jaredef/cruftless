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
