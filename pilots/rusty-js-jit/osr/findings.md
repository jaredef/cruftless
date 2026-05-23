# rusty-js-jit/osr — Local Findings

Per Doc 737 §IV nested locale convention. Locale-scoped findings; promotions to LeJIT parent findings.md noted explicitly when they occur.

---

## Finding OSR.1 (JIT calling convention's params-only-as-args shape blocks OSR loop invoke without major substrate extension) *[new, 2026-05-23 via OSR-EXT 5 pre-implementation source-read]*

**Anchor**: pre-implementation source-read for OSR-EXT 5 (JIT body invoke) reveals the JIT calling convention's locals model:

- `compile_function_inner` (translator.rs) accepts params ∈ {0, 1, 2} (post TL-EXT 3 relaxation).
- At JIT body entry, locals 0..params are initialized from f64 args; locals params..N are initialized to 0.0 (translator.rs:341-346).
- The JIT body's LoadLocal(i) reads from `local_vars[i]` (a Cranelift Variable, register-allocated).

**For OSR loop invoke**, the synthetic FunctionProto wraps the loop body bytecode, which reads/writes the ENCLOSING FRAME's locals via LoadLocal/StoreLocal. The synthetic proto's locals layout matches the frame's (this round's try_osr_compile uses `frame.locals_names.to_vec()`). But:

- Frame.locals contains the runtime VALUES (Value enum); the JIT body needs f64-encoded versions at JIT entry.
- The JIT initializes locals to 0.0, NOT to the frame's values. After invoke, frame's local values would NOT match the JIT body's actual computations.

**Three structural alternatives** (none in (b-narrow) scope; each is its own substrate-extension pilot):

1. **Marshal all locals as args**: extend params to N (≥ frame.locals.len()); pass all locals as f64 (via unbox_arg_f64 per VD encoding); JIT body initializes from args. Requires relaxing the params check + extending JitFn signature to N-ary (or using a single `*const [f64]` pointer arg + extern decode). ~150 LOC; touches all existing Σ/Τ/Ψ/Φ paths.

2. **Pre-populate via extern at JIT entry**: synthesize a "load all locals" prologue in the JIT body that calls an extern (`runtime_load_osr_locals`) once at entry; the extern returns a pointer that JIT body reads to initialize each local Variable. Adds ~10-20 ns per invoke (one extern call); doesn't break existing paths. ~80 LOC.

3. **Frame-pointer access in JIT body**: replace per-local Variable allocation with memory access through the frame's locals Vec (frame as `*mut Runtime` arg; addresses computed via offset). Defeats register allocation; per-local op cost increases. ~120 LOC; significant perf regression for existing JIT paths.

**Substrate implication**: OSR-EXT 5's invoke step as designed at OSR-EXT 1 was structurally incomplete — the design enumerated "local-state copy-in/out" as a single bullet, but the empirical mechanism requires one of the above architectural changes. The design's R2 risk ("local-state sync overhead") named the cost dimension but missed the structural prerequisite (how to actually do the sync).

**Per Doc 740 §II.2 relevant-tier set R**: for OSR loop invoke on json_parse_transform's charCodeAt loop, R has a fifth tier: **JIT calling convention's locals-marshaling capability**. The original 4-tier reading at TL-EXT 1 + VD-EXT 0 missed this tier; the prior reading composed only with the loop-body alphabet, not the frame-state marshaling at the call boundary.

**The honest OSR-EXT 5 close**: land the cache structure (Frame::osr_cache replacing osr_attempted) + the skip-if-failed gate; surface this finding; defer invoke to a follow-on substrate-extension pilot per keeper signal. The cache structure has real engineering value (prevents repeated wasted compile cycles at every threshold-crossing on the same site); the invoke path is the architectural extension that requires keeper deliberation.

**Recommended pilot scope for the invoke closure** (post-OSR-EXT 5): option 2 (extern-pre-populate) per its non-invasive composition with existing JIT paths. ~80 LOC; can be the next round (OSR-EXT 5b or 6, depending on naming) without spawning a new locale.

**Generalization candidate (engagement findings.md Addendum VII)**: the "JIT calling convention's locals-marshaling capability" tier should be named in Doc 740 §II.2's relevant-tier-set apparatus alongside op-set coverage + value-domain coverage. Standing rule 11 extension: for any pilot that invokes JIT bodies that consume enclosing-frame state (OSR; coroutines; mid-function deopt-recovery resume), verify the calling convention supports the required state-marshaling shape.

### Composition with prior findings

- **Finding VII.2 (op-set-coverage)**: applies to the loop-body alphabet check.
- **Finding VII.3 (value-domain-coverage)**: applies to the receiver-Value encoding.
- **Finding OSR.1 (this)**: applies to the frame-state-marshaling check at the JIT-invoke boundary. **Co-equal with VII.2/VII.3** at the JIT-tier closure axis; an OSR pilot must close ALL THREE before its invoke step can deliver reclaim.

### Forward implication for OSR pilot

**OSR-EXT 5 (revised scope)**: land Frame::osr_cache (replacing osr_attempted) + skip-if-failed gate. Document the invoke blocker as Finding OSR.1. Don't attempt invoke implementation in this round.

**OSR-EXT 5b (new round, keeper-pending)**: implement locals-marshaling per option 2 (extern-pre-populate). ~80 LOC. Unlocks invoke for loops whose alphabet IS in JIT scope.

**OSR-EXT 6 (existing scope)**: alphabet extension (GetProp+length-IC + CallMethod+charCodeAt-IC consuming VD encoding). Closes the alphabet gap; combined with OSR-EXT 5b, json_parse_transform's charCodeAt loop becomes invocable.

**OSR-EXT 7 (existing scope)**: composition probe + CRB final disposition + Pred-osr.1 gate.

---

*This findings.md grows as OSR-locale-specific findings surface. Promotions to LeJIT parent findings.md (engagement-wide JIT discipline) or to engagement findings doc Addendum VII (corpus-level) noted explicitly.*
