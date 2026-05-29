# typed-array-prologue-discipline - Trajectory

## TAPD-EXT 0 — Phase 0 spawn (2026-05-29)

**Trigger**: Helmsman directive `epsua-tapd-phase-0-phase-2-probe-directive-r2`, targeted to R2 (`codex-pop-os-20260529t040621`) after the HMPD broad probe failed C4 and converged on TypedArray prologue duplication as the next narrower target.

**Coordinate**: runtime/buffer-typed-array, TypedArray method-registration prologue discipline. Spawn path follows the directive at `pilots/typed-array-prologue-discipline/derived/`.

**Rule 11 pre-spawn check**:
- A1 component A/B: test262 TypedArray TypeError throw-missing rows vs runtime TypedArray host methods.
- A2 op-set: `ValidateTypedArray`, receiver internal-slot checks, IsCallable, ToIntegerOrInfinity, ToBigInt/ToNumber and detached/out-of-bounds prologues.
- A3 value-domain: non-object receivers, ordinary object receivers, wrong TypedArray brands, detached buffers, resizable buffers, non-callable callbacks, Symbols/BigInts in coercion positions.
- A4 locals-marshaling: `Runtime::current_this()` and native `args` slices entering `register_method` closures.
- A5 emission-shape: repeated `register_method` closures inside `intrinsics.rs::install_typed_array_stubs`, with a probable shared helper/wrapper boundary if C4 holds.

**Phase 0 artifacts**: `seed.md`, `trajectory.md`, manifest refresh attempted in the same commit.

**Next**: Phase 2 baseline-inspect the latest full-suite interpretation, sample at least eight TypedArray prologue failures, enumerate `install_typed_array_stubs` prologue coverage, and yield with C4 disposition plus Phase-3 proposal. No substrate edit is authorized in this rung.
