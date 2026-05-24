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

---

## Finding OSR.2 (For-loop / while-loop bytecode shapes have forward-exit jumps that target pcs outside the OSR-extracted slice — only do-while-shape loops extract cleanly without follow-on substrate work) *[new, 2026-05-23 via OSR-EXT 5d empirical readout]*

**Anchor**: OSR-EXT 5d wired the invoke path end-to-end. On json_parse_transform, the invoke does not fire because compile_function_osr fails. The compile failure has TWO independent structural causes:

1. **Alphabet gap** (already named, Finding VII.2): Op::GetProp + Op::CallMethod aren't in the JIT alphabet.

2. **Forward-exit out-of-bounds target** (NEW, this finding): for-loop and while-loop bytecode have the shape:
   ```
   loop_top:
     ... eval condition ...
     JumpIfFalse loop_exit  // forward jump out of loop
     ... body ...
     Op::Jump loop_top      // unconditional back-edge
   post_back_edge:
   loop_exit:                // outside the loop region
   ```

   compute_loop_region extracts `[loop_top, post_back_edge)`. The slice contains the JumpIfFalse, whose target (`loop_exit`) is OUTSIDE the slice. The JIT translator's parse_bytecode records `loop_exit` as a jump target; compile_function_inner allocates a Cranelift Block for it; the block is never filled (no opcodes at pcs past the slice end); Cranelift fails at finalize.

   Only **do-while-shape loops** (back-edge with conditional `JumpIfTrue`/`JumpIfFalse` at bottom; no forward exit jumping out of the loop body) extract cleanly. JS source code that produces this shape: `do { body } while (cond);` syntactically; some compilers also emit it for `for(;;){body;break;}` patterns. Typical for-loops and while-loops with a top-tested condition DO NOT extract cleanly.

**Substrate implication**: OSR loop extraction's structural reach is bounded by the bytecode-compiler's loop emission shape. For the cruftless engagement's most-common loop sources (for/while), the bound is binding — OSR doesn't fire without additional substrate work.

**Three structural alternatives** to close the forward-exit gap:

1. **Extend boundary detection to include the forward-exit target**: in compute_loop_region (or a new helper), forward-scan from loop_top to detect any forward Jump whose target is past the back-edge; if found, extend end_pc to include the target's pc and emit a synthetic exit block at that position. ~50 LOC; broadens OSR reach to for/while.

2. **Translator handling for out-of-bounds blocks**: in compile_function_inner, when a jump target falls outside the bytecode slice, emit a synthetic block at that target that immediately returns (synthesizes the loop-exit pc). The runtime dispatcher reads the JIT body's return + adjusts frame.pc to the forward-exit's original target pc (not end_pc). Requires JitFn signature extension to return the exit-pc alongside the f64 result, OR a side-channel via TLS. ~80 LOC; broader change.

3. **Compile-time loop-shape transform** (Option B from OSR-EXT 1 design): the bytecode compiler emits loop regions with explicit markers + ensures the marked region is self-contained (including the forward exit). Requires changes to the compiler at every loop construct. ~500-700 LOC; cross-cutting. (Originally Option B from OSR-EXT 1 design; ruled out for the (b-narrow) scope but reconsidered here.)

**Substrate implication (candidate engagement-wide promotion at Addendum VIII)**: OSR-style loop extraction has a STRUCTURAL DEPENDENCY on the bytecode-compiler's loop emission shape. Engines whose compilers emit "self-contained loop regions" (e.g., with explicit region markers; or with the forward-exit target inside the region followed by an immediate exit op) admit cleaner OSR. Engines whose compilers emit "fall-through forward exits" (the cruftless case) require either bytecode boundary extension at OSR time OR translator handling of out-of-bounds targets.

The structural pattern generalizes Finding VII.2 (op-set coverage) with a NEW axis: **emission-shape coverage**. Adding to standing rule 11's coverage axes:

| axis | finding | applies to |
|---|---|---|
| component A/B (Addendum IV) | VII.1 | any CRB-driven pilot |
| op-set coverage (V) | VII.2 | JIT-alphabet pilots |
| value-domain coverage (V) | VII.3 | JIT-IC non-Number/Object receivers |
| locals-marshaling coverage (VII) | VIII.2 | JIT-invoke from non-arg state |
| **emission-shape coverage** | **VIII.3 candidate** | **JIT region-extraction pilots (OSR; loop-tier; etc.)** |

**Verify** before spawning a region-extraction pilot: source-read the bytecode-compiler's emission of the target region's enclosing construct; verify the emitted region is self-contained (no forward jumps out of the region; no fall-through into other regions). If not, the pilot must close emission-shape coverage either via compile-time transform OR via runtime boundary extension before invoke can fire.

### Composition with prior findings

- **Finding VII.2 (op-set-coverage)**: applies to inner-loop alphabet; orthogonal to OSR.2 (a do-while loop with charCodeAt would still need VII.2 closure).
- **Finding VIII.2 (locals-marshaling)**: closed at OSR-EXT 5d; orthogonal to OSR.2.
- **OSR-EXT 5d empirical**: invoke verified correctness-clean; correctly doesn't fire on json_parse_transform per BOTH VII.2 + OSR.2.

### Forward implication for OSR pilot

OSR-EXT 6 alphabet extension (closes VII.2 for the inner loop ops) is necessary but not sufficient for json_parse_transform's CRB reclaim materialization. OSR-EXT 6b (closes OSR.2 forward-exit handling) is the additional tier.

Recommended option for closing OSR.2 at this engagement: **option 1 (boundary extension)** — smallest scope; closes the gap at runtime without compiler changes; matches the structural pattern of try_osr_compile's existing boundary detection (compute_loop_region returns a (entry, end) tuple; extension would return (entry, end, optional_exit_pc) and the translator handles the exit pc).

Defer to keeper deliberation on Addendum VIII promotion (emission-shape coverage axis) + OSR-EXT 6b implementation scope.
