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

## TAPD-EXT 2 — callable/index prologue narrowing (2026-05-29)

**Trigger**: continuation after TAPD-EXT 1 and post-sample remeasurement. The new sample matrix left TypedArray prototype surfaces in the top residual set, and the EXT 1 receiver helper made the next duplicated prologue shape visible.

**Move**: extended the shared TypedArray prologue family in `pilots/rusty-js-runtime/derived/src/intrinsics.rs` with:
- `typed_array_callable_arg` for callback/predicate methods (`forEach`, `findIndex`, `every`, `some`, `map`, `filter`, `reduce`, `reduceRight`, `findLast`, `findLastIndex`).
- `typed_array_integer_index` for the index-bearing methods that can safely use the helper without crossing detached/resizable ordering boundaries (`slice`, `set` offset, `fill`, `lastIndexOf`, `copyWithin`).
- explicit callable checks for `sort` / `toSorted` compare functions.

**Boundary choice**: `includes` and `indexOf` `fromIndex` coercion were intentionally left out after verification found detached-buffer ordering regressions. Their symbol/fromIndex rows are real future TAPD work, but they compose with the deferred detached/resizable rung rather than this prologue-only move.

**Verification**:
- `cargo build --release --bin cruft -p cruftless`: PASS.
- Focused probes: PASS for callable predicate/callback rows, `toSorted` comparefn-not-callable, `fill` relative start/end and symbol start/end rows, and `copyWithin` symbol target/start/end rows.
- Touched-method sweep: 847 rows across `fill`, `copyWithin`, `includes`, `indexOf`, `lastIndexOf`, `find`, `findIndex`, `every`, `some`, `forEach`, `map`, `filter`, `reduce`, `reduceRight`, `findLast`, `findLastIndex`, `sort`, `toSorted`.
  - Baseline on current main (`/home/jaredef/Developer/cruftless-r3`): 497 PASS / 350 FAIL.
  - Candidate on R2: 521 PASS / 326 FAIL.
  - Delta: 24 newly passing rows, 0 regressions.
  - Artifacts: `/home/jaredef/Developer/cruftless-r3-sidecar/results/tapd-ext2-baseline-20260529T132714Z/` and `/home/jaredef/Developer/cruftless-r2-sidecar/results/tapd-ext2-targeted-final-20260529T132919Z/`.

**Findings**: this rung drains the safe callback/callability and relative-index subset without entering the detached/resizable state machine. The failed earlier attempt to add `includes`/`indexOf` `fromIndex` coercion produced detached-order regressions, confirming that those rows belong to the later buffer-state rung.

## TAPD-EXT 2 — argument callability and integer-index prologues (2026-05-29)

**Trigger**: Helmsman directive `tapd-ext-2-arg-coercion-r2`, targeted to R2 (`codex-pop-os-20260529t040621`) after TAPD-EXT 1 landed the shared receiver helper.

**Move**: added shared argument prologue helpers in `pilots/rusty-js-runtime/derived/src/intrinsics.rs`:
- `typed_array_callable_arg(rt, arg, method_name)` for callback-bearing methods.
- `typed_array_integer_index(rt, arg, len, default)` for ToInteger-style index arguments that must observe abrupt coercion.

**Routed sites**:
- Callability checks: `forEach`, `findIndex`, `every`, `some`, `map`, `filter`, `reduce`, `reduceRight`, `findLast`, `findLastIndex`, plus `sort`/`toSorted` comparator checks.
- Numeric/index coercion: `subarray` start/end, `set` offset, `fill` start/end, `slice` start/end, `indexOf` fromIndex, `includes` fromIndex, and `copyWithin` target/start/end.

**Boundary choice**: `lastIndexOf` fromIndex coercion was deliberately left on the pre-existing path after a probe showed it exposes a separate detached-buffer semantics gap (`lastIndexOf/detached-buffer-during-fromIndex-returns-minus-one-for-undefined.js`). That belongs with the later detached/out-of-bounds TAPD rung, not this argument-prologue rung.

**Verification**:
- `cargo build --release --bin cruft -p cruftless`: PASS.
- `cargo test --release -p rusty-js-runtime`: BLOCKED before test execution by pre-existing integration-test compile errors referencing removed `Runtime::globals` across stale tests (`labelled.rs`, `omega_5_x.rs`, `closure_upvalues.rs`, `gc_cycle.rs`, `destructure.rs`, `binding_capture.rs`, `complex_assign_target.rs`, `iteration.rs`).
- TAPD candidate cluster: 71 PASS / 197 FAIL / 0 SKIP out of the 268 Phase 2 baseline-failing rows. This is +25 PASS over TAPD-EXT 1's 46 PASS. Artifact: `/home/jaredef/Developer/cruftless-sidecar/results/tapd-ext2-verify-20260529T132809Z/summary.json`.
- Adjacent regression sample: 50 PASS / 0 FAIL. Artifact: `/home/jaredef/Developer/cruftless-sidecar/results/tapd-ext2-verify-20260529T132809Z/adjacent-regression-sample.jsonl`.
- Post-EPSUA `language.statements.for-of` / `feat:TypedArray;not-callable` spot-check remained 18 FAIL / 0 PASS, confirming that cell is not closed by TypedArray prototype argument prologues alone. Artifact: `/home/jaredef/Developer/cruftless-sidecar/results/tapd-ext2-forof-typedarray-not-callable-20260529T132827Z/summary.json`.

**Findings**: TAPD-EXT 2 closes the local callback/not-callable and numeric coercion subset (+25 rows) without adjacent regression. Remaining TAPD mass is now more strongly shaped by detached/resizable buffers, species/subclass allocation, constructor/static method shape, and method-surface neighbors rather than simple duplicated callback prologues.
