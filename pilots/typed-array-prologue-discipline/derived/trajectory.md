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

## TAPD-EXT 1 — receiver prologue helper (2026-05-29)

**Trigger**: Helmsman approval `tapd-ext-1-approval-for-r2`, related to the TAPD Phase 2 response. The authorized substrate move was Rung 1 only: shared receiver/ValidateTypedArray prologue across the duplicated `install_typed_array_stubs` closures, deferring argument/callability and detached/resizable/species work.

**Move**: added `validate_typed_array_this(rt, method_name)` in `pilots/rusty-js-runtime/derived/src/intrinsics.rs` and routed the approved method closures through it: `set`, `fill`, `slice`, `values`, `keys`, `entries`, `@@iterator`, `reverse`, `indexOf`, `forEach`, `findIndex`, `every`, `some`, `join`, `map`, `filter`, `reduce`, `reduceRight`, `toString`, `at`, `lastIndexOf`, and `findLast`.

**Boundary choice**: the helper checks the current receiver is an object and carries the existing non-enumerable `__ta_kind` sentinel. It intentionally does not require a `typed_array_views` backing entry yet because current typed-array-like construction paths expose the sentinel before a complete backing-view discipline exists. Tightening that boundary belongs with the later detached/resizable/internal-slot rung.

**Verification**:
- `cargo build --release --bin cruft -p cruftless`: PASS.
- `cargo test --release -p rusty-js-runtime`: BLOCKED before test execution by pre-existing integration-test compile errors referencing removed `Runtime::globals` in `object_create.rs`, `omega_5_x.rs`, `binding_capture.rs`, `destructure.rs`, `template_literal.rs`, `gc_cycle.rs`, and `omega_5_y.rs`.
- TAPD candidate cluster: 46 PASS / 222 FAIL / 0 SKIP out of the 268 Phase 2 baseline-failing rows, measured with the R2 `target/release/cruft` binary. Artifact: `/home/jaredef/Developer/cruftless-sidecar/results/tapd-ext1-verify-20260529T061610Z/summary.json`.
- Adjacent regression sample: 50 PASS / 0 FAIL from 430 baseline PASS rows across adjacent TypedArray prototype method directories. Artifact: `/home/jaredef/Developer/cruftless-sidecar/results/tapd-ext1-verify-20260529T061610Z/adjacent-regression-sample.jsonl`.

**Findings**: helperizing the receiver prologue closes the duplicated TypeError-missing class for method calls whose only blocker was wrong-receiver acceptance. The remaining failures are dominated by later TAPD rungs (argument/callability order, constructor/static methods, detached/resizable buffers, species/subclass allocation, and unimplemented neighbor methods such as `sort`, `subarray`, `with`, and copyWithin).
