# host-method-prologue-discipline - Seed

**Locale tag**: `L.host-method-prologue-discipline` (EPSUA sub-locale #5).

**Status**: FOUNDED at HMPD-EXT 0. Phase-0 spawn complete; Phase-2 baseline inspection is in progress. No substrate landing is authorized in this rung.

**Workstream**: host-method prologue discipline for runtime built-ins. Many test262 failures appear to share a missing-or-wrong prologue shape: receiver validation, callable validation, argument coercion order, abrupt-completion propagation, arity/name/descriptor metadata, or method registration defaults across host-installed intrinsics.

**Trigger**: EPSUA row 5 (`host-method-prologue-discipline`), constraint #1, projected cascade ~150, runtime tier, high risk. Helmsman directive `epsua-hmpd-phase-0-phase-2-probe-directive` authorizes Phase 0 spawn plus Phase 2 baseline-inspect only.

**Composes with**:
- [EPSUA](../../../apparatus/arcs/2026-05-25-ecmascript-parity-shared-upstream/arc.md) - parent arc and constraint roster.
- [T262C](../../apparatus/test262-categorize/seed.md) - full-suite matrix source.
- ECMA-262 built-in method prologue conventions: RequireObjectCoercible, ToObject, IsCallable, this-value brand checks, argument coercion order, property descriptor/arity/name installation.
- Runtime intrinsic registration in `pilots/rusty-js-runtime/derived/src/intrinsics.rs`.

## I. Telos

Empirically identify whether the full-suite host-method-prologue cluster is coherent enough to support a shared runtime registration/prologue substrate, then propose the smallest resolver-instance-style move that can amortize across host-method registrations without per-method patching.

Success for the founding probe is not PASS movement. Success is a defensible segmentation of the current failures by missing prologue check and registration site, plus a C4 reason-coherence decision: proceed with a Pin-Art duplicated-site probe, or pivot to a sibling candidate if the cluster is too heterogeneous.

## II. Apparatus + Methodology

HMPD follows the five-phase substrate-shaped-work pipeline.

Phase 0 / EXT 0:

1. Spawn this locale at `pilots/host-method-prologue-discipline/derived/`.
2. Refresh `apparatus/locales/manifest.json`.
3. Commit only the spawn artifacts and manifest.

Phase 2 founding baseline:

1. Query latest full-suite interpreted results at `pilots/apparatus/test262-categorize/full-suite/results/test262-full-2026-05-28-123833-p2/`.
2. Identify the host-method-prologue-discipline failure cluster and reason distribution.
3. Sample at least eight failures across the cluster, with test path, reason, likely missing prologue check, and likely runtime registration/implementation site.
4. Cross-reference `intrinsics.rs` registration helper usage and ad hoc method installation patterns.
5. Yield to helmsman with a Phase-3 proposal. No substrate edit lands before approval.

Rule 11 pre-spawn check:

- A1 component-A/B: test262 built-in method failures versus runtime host-method registration/dispatch.
- A2 op-set: receiver brand checks, RequireObjectCoercible/ToObject, IsCallable, argument coercion order, abrupt completion, arity/name/descriptor installation.
- A3 value-domain: built-in prototypes, constructor functions, namespace objects, primitives boxed through this-value dispatch, callable/non-callable arguments.
- A4 locals-marshaling: JS call frame arguments and `this` value into Rust host closures.
- A5 emission-shape: intrinsic registration helpers and per-method Rust closures in `intrinsics.rs`.

## III. Carve-outs

- No substrate implementation in HMPD-EXT 0.
- Parser, lowering, iterator-close, and `$262` host-hook failures are out of scope unless the Phase-2 probe shows the candidate was misclassified.
- Individual built-in algorithm correctness is out of scope unless the failure is caused by a shared prologue/registration discipline.
- Full-suite reruns are out of scope without keeper authorization; use the latest full-suite result artifacts and focused exemplar inspection.

## IV. Resume Protocol

1. Read this seed.
2. Read `trajectory.md` tail.
3. Read the EPSUA arc row for `host-method-prologue-discipline`.
4. Inspect the latest full-suite results directory named in §II.
5. Before any substrate move, complete Rule 23 baseline-inspect and Rule 24 duplicated-site Pin-Art probe.

## V. Status

HMPD-EXT 0 has founded the locale. Awaiting the Phase-2 probe deliverable and helmsman approval before any substrate landing.
