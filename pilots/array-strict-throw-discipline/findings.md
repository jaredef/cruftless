# array-strict-throw-discipline — Findings

Per-locale empirical findings extracted from `trajectory.md`. Cross-referenced in `apparatus/docs/findings-ledger.md`. Append-only per Doc 727 §X basin-stability discipline.

First `findings.md` authored 2026-05-30 after ASTA-EXT 1 landed (the locale's second productive rung; per the per-locale convention, findings.md extraction is now due).

## Finding ASTA.1 — Narrow-dispatcher cascade-revival at engine-internal Result-threading site

**Source**: ASTA-EXT 0 (LANDED 2026-05-30, commit 00a73363).

**Class**: error-propagation.

**Statement**: when an engine-internal non-Result-returning helper (here `object_set`) must propagate spec-mandated errors to a subset of callers (here the Array.prototype mutating-method intrinsics that need to surface frozen-receiver / non-writable-length TypeErrors per ECMA §10.4.2.1 + §23.1.3), introduce a Result-returning narrow dispatcher (`object_set_checked`) consumed only at the Result-aware sites; the unchecked helper remains unchanged for non-Result internal callers. Mirrors the TAECSF-EXT 0 pattern (`typed_array_set_index_checked` over `typed_array_set_index`) and the TABSC substrate-prefix-amortization pattern.

**Predicts**: future Result-thread-through-non-Result-callsite work in the engine will adopt the same narrow-dispatcher shape; substrate rungs that lift the helper's signature globally incur 5-10× larger blast radius than necessary.

**Evidence**: ASTA-EXT 0 introduced `object_set_checked` (~35 LOC) + migrated 6 Array.prototype mutating-method intrinsics (push/pop/shift/unshift/splice/sort) to use it with `?`. Probe 7/7 PASS; cluster gates preserved; diff-prod +1 unexpected positive.

**Composes with**: Rule 27 (substrate-spec-correctness vs engine-architecture conflict), Rule 30 (narrow-dispatcher cascade-revival as preferred error-propagation discipline — ASTA.1 was the third corroborating instance that drove Rule 30 promotion), Rule 4 (never split a substrate move), Rule 21 (probe-first scoping).

**Promotion status**: standing-rule (Rule 30 promoted 2026-05-30 at findings-disposition cycle-3; ASTA.1 cited as 3rd instance alongside TAECSF.1 + TABSC.2). See findings-ledger Entry 019.

## Finding ASTA.2 — Parallel-emit-site throw-discipline drift (Rule 20 instance at the compiler tier)

**Source**: ASTA-EXT 1 (LANDED 2026-05-30, commit 371caf79).

**Class**: substrate-pattern (compiler-level Rule 20 specialization).

**Statement**: when a throw-discipline (e.g., TypeError on assignment to const binding per ECMA §13.15.4 + §15.2.7) is encoded at one bytecode emit site (`compile_plain_assign` Identifier arm, lines 5942-5954) and a parallel emit site doing the same logical operation (`assign_target_from_stack` Identifier arm at line 6273 — the destructuring path) omits the same check, the discipline drifts across the parallel sites and surfaces as cross-test failure shape coherence (here, all 4 for-of put-const cells exiting at "Expected TypeError"). The drift is a Rule 20 instance (substrate-discipline coherence drift across parallel helpers) specifically at the compiler tier: parallel emit sites that handle the same AST-node shape (Identifier as assignment target) must mirror the same per-emit checks (const-binding, writability, brand-check, etc.).

**Predicts**: when adding a new check at one bytecode emit site for a given AST-node shape, audit all parallel emit sites for the same node shape; otherwise the new check will silently bypass any downstream code path that routes through a different emit site (here, destructuring assignment bypassed the const-check that direct assignment caught).

**Evidence**: ASTA-EXT 1 surfaced the destructuring path's `assign_target_from_stack` Identifier arm missing the const-binding check that `compile_plain_assign` had since founding. Adding the parallel check (~10 LOC) closed 4 for-of put-const cells. The discipline-drift was invisible at sample/cluster instruments because the destructuring-into-const pattern is rare in real-world code; surfaced via test262 cross-family pattern aggregation (SAMPLE.1).

**Composes with**: Rule 20 (substrate-discipline coherence drift surfaces as cross-module reason-shape coherence — ASTA.2 is a specialization at the compiler-emit-site tier), Rule 17 (pre-scoping per-reason-pattern segmentation — the put-const sub-bundle is one Rule-17 segment of the SAMPLE.1 cross-family residual), Rule 6 (surface-completeness audit — ASTA.2's discipline is to audit parallel emit sites at substrate-introduction).

**Promotion status**: trajectory-and-findings-embedded; one-more-observation pending. Candidate apparatus standing-rule (Rule 20 specialization at the compiler-emit-site tier; promotable once a second cross-locale instance surfaces). Sibling candidate sites for the second observation: the bytecode emit sites for Member assignment, Spread destructuring, default-value initialization, all of which may have parallel-emit drift at distinct sub-shapes.

## Standing carry-forward

The narrow-dispatcher pattern that motivated ASTA.1's contribution to Rule 30 + the parallel-emit-site audit discipline that ASTA.2 names compose to form an engagement-tier "emit-site audit" discipline: at every substrate-introduction touching a bytecode emit site, audit (a) the Result-thread-through-non-Result-callsite pattern (Rule 30) AND (b) the parallel-emit-site discipline-drift pattern (ASTA.2). Both are at the same architectural layer (bytecode emit) and both surface via SAMPLE.1-style cross-family reason-shape coherence.
