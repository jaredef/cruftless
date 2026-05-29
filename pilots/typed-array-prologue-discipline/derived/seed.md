# typed-array-prologue-discipline - Seed

**Locale tag**: `L.typed-array-prologue-discipline` (EPSUA/HMPD child candidate; directive-spawned under `derived/`).

**Status**: FOUNDED at TAPD-EXT 0. Phase-0 spawn complete; Phase-2 baseline inspection in progress. No substrate landing is authorized in this rung.

**Workstream**: typed-array method prologue discipline. The locale narrows HMPD's broad TypeError-throw-missing surface to the duplicated runtime sites in `intrinsics.rs::install_typed_array_stubs`, especially missing or partial `ValidateTypedArray`, receiver-brand/internal-slot checks, and adjacent argument prologues on TypedArray prototype and constructor methods.

**Trigger**: Helmsman directive `epsua-tapd-phase-0-phase-2-probe-directive-r2` on 2026-05-29, keeper-approved per Telegram 10339(3). The HMPD four-resolver probe converged that broad HMPD fails C4 and that the typed-array prologue subcluster is the strongest duplicated mechanical shape.

**Composes with**:
- `apparatus/arcs/2026-05-25-ecmascript-parity-shared-upstream/arc.md`
- `pilots/host-method-prologue-discipline/derived/trajectory.md`
- `pilots/typed-array-missing-method/seed.md` and trajectory, especially TAMM-EXT 8
- `pilots/typed-array-wrong-result/seed.md`
- `pilots/typed-array-resizable-buffer-indexed-access/seed.md`
- ECMA-262 ValidateTypedArray and TypedArray built-in method prologue requirements

## I. Telos

Determine whether the TypedArray subset of the HMPD TypeError-throw-missing cluster is coherent enough for a focused runtime substrate move.

If coherent, the anticipated substrate is a resolver-instance-style helper or wrapper discipline for TypedArray method entry: validate the receiver has TypedArray internal state, reject incompatible receivers, preserve detached/out-of-bounds checks, and perform per-method argument prologues such as IsCallable and ToIntegerOrInfinity before body effects.

## II. Apparatus + Methodology

Phase 0 spawns this locale and refreshes the manifest.

Phase 2 baseline-inspects the latest full-suite interpretation:
1. Filter `interpreted.jsonl` for runtime/buffer-typed-array TypeError throw-missing rows that map to TypedArray constructor or prototype surfaces.
2. Sample at least eight failures across the filtered cluster.
3. Segment by method family, failure reason, and likely missing prologue check.
4. Enumerate `register_method` closures in `intrinsics.rs::install_typed_array_stubs` and classify each as having or missing receiver validation.
5. Apply EPSUA C4: if the narrowed cluster has a coherent dominant failure shape, propose Phase 3 Pin-Art probing over the duplicated registration sites. If not, split into smaller child coordinates.

## III. Carve-outs

- No runtime substrate edit in TAPD-EXT 0.
- ArrayBuffer and DataView constructor/accessor semantics are adjacent but out of TAPD unless the filtered probe shows they share the same TypedArray prologue helper boundary.
- ResizableArrayBuffer indexed access, detached buffer backing-store semantics, IntegerIndexedExotic element get/set, and typed-array value-semantics wrong-result rows remain sibling locales unless they are required to make ValidateTypedArray observability correct.
- Missing methods are TAMM scope unless their installed stubs now lack prologue discipline.

## IV. Resume Protocol

Read this seed, then `trajectory.md` tail, then HMPD trajectory. Re-run the Phase 2 filter against the latest full-suite result before proposing runtime edits. If the filtered cluster no longer meets C4, yield with a split recommendation rather than editing `intrinsics.rs`.
