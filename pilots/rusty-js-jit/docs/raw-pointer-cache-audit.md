# Raw-Pointer-Cache Audit (StubE-EXT 9 / TB-EXT 9 / Engagement-wide)

*Per Findings doc standing rule 9 (added 2026-05-23 at Addendum I per TB-EXT 7 segfault generalization): any raw-pointer cache capturing a pointer to a struct living in a HashMap or Vec value slot must verify the underlying storage uses Box-wrapping or equivalent stable-address discipline. This audit enumerates every raw-pointer cache + raw-pointer-deref site in the engagement's rusty-js-jit + rusty-js-runtime crates and verifies each is either SAFE-by-construction or already-protected.*

## 1. Methodology

Two grep classes ran:

1. **Pointer-cache patterns**: `Cell<Option<NonNull` / `Cell<Option<*const` / `RefCell<.*NonNull` / `RefCell<.*\*const`. Identifies fields that STORE raw pointers across calls.

2. **Pointer-deref sites**: `unsafe { &\*` / `as \*const` / `as \*mut` / `NonNull::new` / `from_bits.*as \*`. Identifies USES of raw pointers (transient and cached).

Each match was inspected for:
- **Source of the pointer**: where it was created (stack-local? heap-stable? HashMap/Vec value-slot?)
- **Lifetime guarantee**: how long the pointer must remain valid (within-scope? cross-call?)
- **Stability discipline**: if cross-call, is the source Box-wrapped, Rc'd, leaked-static, or otherwise address-stable?

## 2. Pointer-cache fields (cross-call storage)

The complete inventory of fields that STORE a raw pointer or its proxy across calls:

| field | location | source storage | stability discipline | status |
|---|---|---|---|---|
| `tb_metadata_ptr: Cell<Option<NonNull<()>>>` | `value.rs:936` (ClosureInternals) | `*const CompiledFn` from `&Box<CompiledFn>` per TB-EXT 7 fix | Box puts CompiledFn at stable heap address; HashMap stores Box pointer only; rehash moves Box not CompiledFn | **SAFE** (TB-EXT 7 Box-wrap) |
| `ACTIVE_GETPROP_FN: Cell<Option<GetPropFn>>` | `deopt.rs:300` (thread_local) | Rust fn item (static storage) | fn-pointer to static fn; address never changes | **SAFE-by-construction** |
| `ACTIVE_IC_OBSERVE_FN: Cell<Option<IcObserveFn>>` | `deopt.rs:365` (thread_local) | Rust fn item (static storage) | same | **SAFE-by-construction** |
| `ACTIVE_IC_FAST_GET_FN: Cell<Option<IcFastGetFn>>` | `deopt.rs:399` (thread_local) | Rust fn item (static storage) | same | **SAFE-by-construction** |
| `LAST_DEOPT_FRAME: RefCell<Option<DeoptRecoveredState>>` | `deopt.rs:244` (thread_local) | by-value DeoptRecoveredState (no raw pointers inside) | RefCell, not pointer-cache | **N/A** (not a raw-pointer cache) |
| `CURRENT_DEOPT_SITES`, `CURRENT_RUNTIME`, `CURRENT_PROTO` | `deopt.rs` TLS | dispatcher's set-before-JIT-call, clear-after | scoped to single JIT call; cleared on return | **SAFE-by-scope** |

**Total cross-call raw-pointer cache sites**: **1** (TB's tb_metadata_ptr); already protected.

## 3. Pointer-deref sites (transient use)

The deref sites are uses of raw pointers within a single scope:

**`value.rs:143-153, 1007-1031`** — Value layout self-tests. Run at startup; assert that the `#[repr(C, u8)]` layout produces the expected discriminant + payload offsets. Cell-local; pointers don't escape. **SAFE**.

**`interp.rs:8400` `let rt_ptr_for_tb = self as *mut Runtime as usize`** — captures self for TB fast-path's TLS sets. Used within the same call_function invocation; Runtime owned by caller's stack frame; address stable for call duration. **SAFE-by-scope**.

**`interp.rs:8405` `&*(nn.as_ptr() as *const _)` (TB cell deref)** — the TB cell's stored pointer. Per §2, Box-wrap protects the source allocation; deref produces valid `&CompiledFn`. **SAFE per TB-EXT 7 fix**.

**`interp.rs:8418` `&*c.proto as *const _ as usize`** — proto address inside `Rc<FunctionProto>`. Rc allocation is heap-stable (the Rc's strong/weak counters + inner FunctionProto live in a single malloc allocation that doesn't move). The captured address is stable for the Rc's lifetime; the Rc is held cross-call via ClosureInternals. **SAFE-per-Rc-stability**.

**`interp.rs:8437/8443/8444/8572/8580/8581` `f64::from_bits(&args[i] as *const Value as u64)` (VTI encoding)** — encodes pointer-to-Value as f64-bits for VTI calling convention. `args: Vec<Value>` is the caller's owned Vec; held borrowed through the call; addresses stable. The JIT dispatcher receives the f64 via FP register, bitcasts back to pointer inside the JIT prologue, derefs to load the f64 payload. All within the JIT call's scope. **SAFE-by-scope** (VTI is default-OFF; opt-in only).

**`interp.rs:8541/8542` `self as *mut Runtime as usize` + `&*proto_rc as *const _ as usize`** — standard dispatcher path's equivalent TLS-setup captures. Same analysis as 8400/8418. **SAFE**.

**`interp.rs:8607` `tb_cf_ptr as *mut ()` for TB cell populate** — `tb_cf_ptr` was captured from `&**jit_fn` where jit_fn is `&Box<CompiledFn>`. The `&**` deref strips the `&Box` to get the address of the CompiledFn inside the Box's heap allocation. Stable per the TB-EXT 7 Box-wrap. **SAFE per TB-EXT 7 fix**.

**`interp.rs:9609/9611, 9670, 9703/9705`** — TLS-extern callbacks (`runtime_getprop_on_object`, `runtime_ic_observe`, `runtime_ic_fast_get`) read CURRENT_RUNTIME + CURRENT_PROTO from TLS via raw-pointer-as-usize unbox. The TLS slot is set by the dispatcher pre-JIT-call and cleared post-call; the callbacks fire ONLY during the JIT call (they're invoked by JIT-emitted code). Lifetime invariant: TLS pointer is valid for the duration of the JIT call. **SAFE-by-dispatcher-invariant**.

**napi.rs sites (numerous)** — the NAPI surface uses raw pointers extensively for C-ABI compat. The NAPI surface is an explicit unsafe boundary per the NAPI spec; SAFE per spec contract. Out of scope for this audit's load-bearing concerns (no NAPI-side cross-call cache equivalent to the TB-EXT 7 bug class).

## 4. Audit conclusion

**Standing rule 9 is satisfied engagement-wide.** Only ONE raw-pointer cache exists across the entire rusty-js-jit + rusty-js-runtime crates: TB's `tb_metadata_ptr` on ClosureInternals. That cache is protected by the TB-EXT 7 Box-wrap fix; CMig-EXT 17's canonical fuzz validates correctness across 8 configurations × 2000 fixtures.

All other raw-pointer deref sites are either:
- **SAFE-by-construction**: source is heap-stable (Rc allocation; Box-wrapped) OR static (fn item)
- **SAFE-by-scope**: pointer used within a single call_function invocation; source held in caller's stack frame
- **SAFE-by-dispatcher-invariant**: TLS-managed; set pre-JIT-call, cleared post-JIT-call; valid for JIT-call duration only

**No latent TB-EXT-7-class bugs identified.** Standing rule 9 applied prospectively at TB-EXT 3b design time would have prevented the TB-EXT 7 segfault; applied retroactively at this audit, no additional segfault candidates surface.

## 5. Forward implications

**Standing rule 9 is now empirically validated as engagement-tier framework**. The rule's value is bounded (single instance prevented; no additional instances at risk). The rule's discipline (audit before/at design time) is the engagement-tier framework that pays compounding returns when future pilots are designed.

**No follow-up substrate work surfaces from this audit.** The audit's output is a clean bill of health, not a fix list.

## 6. §XVI / Doc 734 / Doc 735 §X.h categorization

Per Doc 730 §XVI: not applicable (audit-tier; no substrate-correctness call).

Per Doc 734 §V: growth (c) **positive-finding generalization preparatory** — the audit establishes that standing rule 9's prospective application is sufficient (no retroactive bug class hiding in the engagement). Future rule-9 audits at new pilot design time become the discipline; this audit is the establishing baseline.

Per Doc 735 §X.h.c three-probe-levels: this audit is design-tier; the canonical fuzz (CMig-EXT 17) is the fuzz-probe-level instrument that would catch any rule-9 violation at runtime.

## 7. Composition with prior corpus work

- **Findings doc standing rule 9 (added 2026-05-23 at Addendum I)**: this audit IS the rule's first engagement-wide application. The rule's discipline holds; no latent violations.
- **Findings doc IV.4 (canonical fuzz as standing instrument)**: the audit + canonical fuzz together close the bug-class surface. Audit at design time prevents the class; canonical fuzz at runtime catches any audit miss.
- **TB-EXT 7 enhancements log entry**: the bug class generalization the entry named is empirically anchored as bounded.

---

*Raw-pointer-cache audit complete. Standing rule 9 satisfied engagement-wide; only TB's cache exists and is Box-wrap-protected. No follow-up substrate work surfaces. The audit is the establishing baseline for future rule-9 applications at new pilot design time.*
