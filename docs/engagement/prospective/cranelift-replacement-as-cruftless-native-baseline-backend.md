# Cranelift Replacement as Cruftless Native Baseline Backend

**Date**: 2026-05-25
**Author**: 2026-05-25 session
**Status**: prospective — primary articulation for replacing Cranelift only after the backend role is made resolver-instance-clean
**Composes with**:
- [Doc 729](../../corpus-ref/729-cruftless-a-primary-articulation-of-the-resolver-instance-pattern-as-the-comprehensive-design-toward-which-rusty-bun-morphs.md) — resolver-instance pattern and zero-residue artifacts
- [Doc 730](../../corpus-ref/730-the-vertical-recurrence-of-the-lowering-compiler-closure-as-primitive-across-substrate-tiers.md) — P1-P4 lowering compiler, resolution pipeline, deviation pipeline, bidirectional engine-diff
- [Doc 731](../../corpus-ref/731-the-jit-as-a-lowering-compiler-tier-alphabet-purity-upstream-as-the-bound-on-jit-complexity.md) — JIT as lowering compiler tier, originally with Cranelift as backend
- [Doc 733](../../corpus-ref/733-fractal-seeds-and-trajectories-recurrent-resume-vector-pairs-across-substrate-depth-as-the-operating-conditions-layer-for-pin-art-at-engagement-scale.md) — locale/pair discipline for new substrate depth
- [Doc 734](../../corpus-ref/734-the-meta-resolution-pipeline-as-the-operating-instrument-of-the-engagement-recursion-with-the-framework-as-its-own-substrate.md) — meta-resolution pipeline and category-deterministic substrate moves
- [LeJIT](../../../pilots/rusty-js-jit/seed.md) — current JIT locale and its sublocale evidence

## I. Question

Cranelift currently supplies the machine-code lowering tier for LeJIT. The linked source-count reading makes the asymmetry visible: cruftless's local linked crates are much smaller than the Cranelift/JIT dependency substrate pulled into the binary. The obvious theory is that cruftless can remove Cranelift and hand-roll the smaller machine-code surface it actually needs.

That theory is directionally plausible but under-specified. The question is not "can we delete Cranelift?" The question is:

**At ECMA parity and LeJIT maturity, what backend alphabet must exist, and can cruftless own that backend as a P1-P4 resolver instance without reintroducing an opaque optimization tower below the already-legible language DAG?**

This document formalizes the prospective answer.

## II. Answer

Yes, but not as a casual dependency removal and not as "hand-roll Cranelift." The viable target is a **Cruftless native baseline backend**: a small, verified, stage-deterministic bytecode/JIT-IR-to-machine-code resolver whose alphabet is derived from the closed ECMAScript behavior DAG rather than from a general-purpose compiler backend.

Cranelift should remain the bootstrap backend and oracle until the native backend proves the same resolver role:

1. It consumes the same JIT alphabet.
2. It emits machine code with no unresolved directives or hidden dynamic policy.
3. It verifies before emission.
4. It preserves the same semantics as Cranelift on the supported subset.
5. It improves the pressure point Cranelift currently does not close: tight-loop machine code where dispatcher cost is no longer dominant.

The mature replacement is therefore not "Cranelift minus features." It is **the backend implied by cruftless's own language-behavior DAG**.

## III. Current LeJIT Evidence

The LeJIT locale family shows three different classes of Cranelift surface.

### III.1 Load-bearing now

Cranelift is currently load-bearing for:

- f64 arithmetic, comparisons, truthiness lowering, locals, branches, and returns.
- block construction, SSA/variable discipline, and verifier pressure.
- extern-call plumbing for deopt, IC helpers, hot intrinsics, runtime callbacks, and sentinel paths.
- executable memory, finalization, symbol linkage, and function pointer lifetime.
- OSR loop-body compilation through the locals-array ABI.
- cross-architecture abstraction, especially after the Φ f64 calling-convention shift.

This is the substrate that any replacement must preserve first.

### III.2 Previously suspected native surfaces that became less load-bearing

Several sublocales reduced the urgency of raw native emission:

- **Σ stub-emitter**: the first-cut win came from Rust-side IC fast paths and side-table patching. Full inline machine-code IC stubs are no longer first-cut load-bearing.
- **Τ tiny-baseline**: the winning move was metadata/cache-path collapse in `call_function`, not a native call thunk.
- **Ψ value-tag-inline**: the standalone tag-inline path was initially negative; Φ's f64 convention revived the value-domain benefit structurally.

The lesson is important: not every apparent "native codegen" problem is actually a backend problem. Some are higher-tier resolver residue.

### III.3 The remaining real replacement pressure

The hard pressure is tight loops. The LeJIT/CRB evidence reads that `arith_tight_loop` is about 3.4x slower than Bun while dispatcher cost is below 2%. Σ/Τ/Ψ cannot close that gap. The candidate closers are:

- better Cranelift configuration,
- a hand-rolled tight-inner-loop emitter,
- or a different backend.

This is the first honest site for a native backend proof. The replacement must start where Cranelift is actually failing the engagement's telos, not where dependency size merely looks offensive.

## IV. Backend as Resolver Instance

Per Doc 729 and Doc 730, the backend is a resolver instance:

```
JIT alphabet source -> backend resolver -> executable machine-code artifact
```

The backend's source is not arbitrary Rust structs. It is the typed alphabet LeJIT has earned: f64 ops, locals, control flow, extern calls, OSR locals-array entry, deopt/sentinel exits, and eventually the finite P4 dispatch sites the parity DAG exposes.

The backend artifact is not "some bytes." It is executable code whose directives have been fully consumed:

- register assignment has been decided,
- labels have been resolved,
- call ABI has been materialized,
- branch patches have been applied,
- executable memory has been finalized,
- deopt metadata has been attached,
- and no hidden compiler policy remains observable by the next tier.

The four P1-P4 properties specialize at this boundary:

| Property | Backend specialization |
|---|---|
| P1 typed primitives | every backend op has declared input/output machine-level types |
| P2 stage-deterministic compilation | same JIT source + same target emits byte-identical code or explainably equivalent code |
| P3 verifier-before-emission | unsupported op, bad stack shape, bad ABI edge, or unpatched label rejects before executable memory |
| P4 implementation freedom | register choice, instruction sequence, and spill strategy are free only under the JIT op's semantic contract |

A native backend that lacks these is not Cruftless. It is just a smaller opaque backend.

## V. Why Full Parity Changes the Replacement Question

Before parity, the backend alphabet is provisional. Replacing Cranelift too early risks baking the current incomplete subset into the backend and then paying rework when test262 maturity reveals missing language behavior.

At parity, the situation changes. Full ECMA behavior parity provides an empirically closed basis for the language behavior DAG. With engine262/spec-correspondence work layered on top, that basis becomes a computable inference path for future implementation. The backend then receives a bounded alphabet:

- which value-domain distinctions actually reach native code,
- which dispatch sites remain P4 dynamic sites,
- which deopt reasons are real,
- which extern calls remain necessary,
- which loop forms dominate performance,
- and which machine-level effects must be represented directly.

The replacement should therefore be understood as a parity-maturity move: **the native backend is derived from the closed alphabet, not guessed before the alphabet closes**.

## VI. The Cranelift Role After This Reframing

Cranelift currently has three legitimate roles:

1. **Bootstrap backend**: it lets LeJIT exist while the engine alphabet is still moving.
2. **Oracle backend**: it supplies a second executable lowering for bidirectional comparison against the native backend.
3. **General fallback**: it can retain coverage for functions outside the native backend's first-cut subset.

This reframes Cranelift from permanent substrate to scaffold. The intended terminal relation is:

```
          +-> CraneliftBackend  (bootstrap/oracle/fallback)
JIT IR ---|
          +-> NativeBaselineBackend (default once proven)
```

Cranelift should not be removed until it has first served as the bidirectional engine-diff counterpart for the backend boundary.

## VII. Native Backend Minimum Surface

The first native backend should be a baseline emitter, not an optimizing compiler. Its minimum surface:

- code buffer with explicit relocation records,
- W^X executable memory discipline,
- instruction encoding for one target first,
- label definition and branch patching,
- call ABI for f64/i64/usize extern functions,
- return ABI for f64 and sentinel encodings,
- small virtual-register or stack-slot discipline,
- verifier for stack shape, local index bounds, ABI signatures, and label resolution,
- deopt metadata side table,
- OSR locals-array entry ABI,
- instruction-cache flush where required by target architecture,
- stable lifetime for executable pointers.

The first target should be the hardware that produced the sharpest reading, likely aarch64 Linux. x86_64 follows only after the backend role is proven.

## VIII. Non-Goals

The native backend is not:

- a general-purpose replacement for Cranelift,
- an optimizing compiler,
- a new multi-tier JIT,
- a register-allocation research project,
- a hidden deopt/deviation layer,
- a second language runtime below the runtime.

Its purpose is narrower: lower the closed cruftless JIT alphabet into machine code with less dependency bulk, better tight-loop behavior, and greater architectural legibility than Cranelift provides for this specific engine.

## IX. Deviation Pipeline Constraint

The backend must not absorb semantic deviations. Per Doc 730 §XIV-XVI, deviations belong in a typed deviation alphabet above the backend, with protected invariants named.

Backend-level "fixes" like "emit this odd machine sequence because Bun tolerates X" are forbidden unless the deviation has already been promoted upstream as:

- a named pattern,
- a strict rejection,
- a tolerant lowering,
- a diagnostic,
- and a protected-invariants list.

The backend consumes the alphabet it is given. It does not invent language semantics.

This is load-bearing. A native backend that silently encodes ecosystem tolerance would destroy the diagnostic legibility gained by the resolver pipeline.

## X. Bidirectional Backend Oracle

The transition from Cranelift to native backend should use Doc 730 §XVI's bidirectional engine-diff discipline at the backend boundary.

For each supported function shape:

1. Compile with CraneliftBackend.
2. Compile with NativeBaselineBackend.
3. Execute both over identical argument/state fixtures.
4. Compare result, deopt reason, side effects, locals-array output, and thrown/error state.
5. If they diverge, classify:
   - native backend bug,
   - Cranelift/fallback bug,
   - upstream JIT alphabet ambiguity,
   - implementation freedom below observable semantics.

The oracle must cover both positive and negative paths: normal returns, branch-heavy loops, extern-call hits, deopt sentinels, IC misses, OSR copy-in/copy-out, and unsupported-op rejection.

## XI. Phased Arc

### Phase 0 — Backend trait

Introduce a backend abstraction without changing behavior:

- `CraneliftBackend` as current implementation.
- `Backend` trait over compile input, executable output, symbols, and metadata.
- same tests, same benchmarks, no native emitter yet.

Gate: no behavior change; Cranelift path remains byte-for-byte or result-equivalent.

### Phase 1 — f64 arithmetic/control native subset

Implement native lowering for the smallest Class A subset:

- constants,
- f64 add/sub/mul/div where present,
- comparisons,
- locals,
- jumps,
- returns.

Gate: dual-backend oracle green for arithmetic hot-loop fixtures; native compile latency and runtime measured against Cranelift.

### Phase 2 — extern calls and deopt sentinel

Add ABI support for runtime helper calls and sentinel returns.

Gate: GetProp helper paths, IC fast-get miss/hit sentinels, and deopt state all compare cleanly against Cranelift.

### Phase 3 — OSR locals-array ABI

Add native support for the OSR `extern "C" fn(*mut f64) -> f64` shape.

Gate: OSR loop-region fixtures copy locals in/out identically to Cranelift.

### Phase 4 — hot-loop backend proof

Target the actual pressure point: tight loops where Cranelift lags Bun.

Gate: native backend materially improves `arith_tight_loop` without reducing conformance or destabilizing LeJIT default-on paths.

### Phase 5 — default switch and feature gate

Make NativeBaselineBackend default for its proven subset; Cranelift remains fallback/oracle behind a feature flag.

Gate: full relevant test262 slice, LeJIT benches, CRB fixtures, and dual-backend regression suite green.

### Phase 6 — dependency retirement

Only after fallback usage is zero or explicitly accepted, remove Cranelift from the default binary.

Gate: linked LoC drops, binary still passes the same gates, and Cranelift remains optionally available for differential runs if useful.

## XII. Falsifiers

The replacement should be abandoned or reframed if any of these hold:

- Native backend cannot match Cranelift correctness on the Class A subset.
- Native backend compile latency is not materially lower.
- Native backend tight-loop runtime does not improve the measured 3.4x Bun gap.
- The verifier grows weaker than Cranelift's protection and lets malformed code reach executable memory.
- Cross-architecture burden dominates the gained simplicity.
- The backend accumulates semantic policy that belongs in the upstream alphabet.
- The native backend becomes larger and less legible than the Cranelift surface it replaces.

These are not failure moods; they are the probe conditions that keep the project honest.

## XIII. Interaction with Full ECMA Parity

The native backend is downstream of the ECMA parity apparatus, not parallel to it. The test262 matrix and Pin-Art apparatus tell us which language behavior remains unresolved. The backend replacement should proceed only on alphabet regions that are already stable:

- Class A arithmetic/control operations,
- f64 calling convention,
- OSR locals-array ABI once stable,
- hot intrinsic table entries whose extern signatures are fixed,
- P4 sites whose deopt reasons have been enumerated.

As parity advances, the backend alphabet grows. As the backend alphabet grows, the native emitter becomes a stronger proof of the core thesis: a closed language behavior DAG can guide implementation by computable inference instead of blind discovery.

## XIV. Prospective Locale

If authorized, spawn a new locale:

```
pilots/rusty-js-jit/native-baseline-backend/
  seed.md
  trajectory.md
  docs/backend-alphabet.md
  docs/dual-backend-oracle.md
  docs/aarch64-emitter-design.md
```

Seed telos:

> Derive and validate a Cruftless native baseline backend for the stable LeJIT alphabet, using Cranelift as bootstrap/oracle until the native backend is stage-deterministic, verifier-gated, and measurably superior on the tight-loop pressure point.

First trajectory round:

- read LeJIT parent + Φ/Σ/Τ/Ψ/OSR/HI/TL/VD locales,
- extract backend alphabet v0,
- introduce backend trait with Cranelift implementation unchanged,
- add dual-backend fixture harness with native backend initially absent.

## XV. Recommendation

Proceed, but in resolver order:

1. Do not delete Cranelift first.
2. Abstract the backend role first.
3. Preserve Cranelift as oracle.
4. Implement the native backend only against the stable, measured alphabet.
5. Use tight-loop performance as the first proof, not dependency size.
6. Retire Cranelift from default linkage only after the native backend satisfies P1-P4 and the dual-backend oracle.

The telos is not a smaller binary by subtraction. The telos is a backend whose existence follows from the same principle as the rest of Cruftless: once the language behavior DAG is legible, implementation follows a computable resolution path.

## XVI. Status

Prospective. No substrate changes authorized by this document alone. The next action is keeper authorization to spawn the native-baseline-backend locale and begin Phase 0.
