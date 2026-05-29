# cjs-ns-shape-diff-residual - Seed

**Locale tag**: `L.cjs-ns-shape-diff-residual` (CNSDR).

**Status**: FOUNDED at CNSDR-EXT 0. Phase 0 spawn plus Phase 2 inline-data probe only; no runtime substrate lands in this founding round.

**Parent arc**: CommonJS namespace shape parity. Adjacent to CJS namespace exclusion/default-shape work and top-500 parity measurement, but scoped to "OK under both runtimes, Object.keys shape differs" residuals.

**Workstream**: post-2026-05-29 parity sweep shape-diff-no-error cluster: 56 packages that import cleanly enough to compare namespace key sets but diverge from Bun on namespace shape.

## I. Telos

Discriminate the 56-row CJS namespace shape-diff residual into coherent mechanisms before any substrate change. The goal is to avoid treating every `Object.keys(ns)` mismatch as one mechanism when the inline data already suggests both missing-in-cruft and extra-in-cruft families.

## II. Apparatus

- Inline CAACP source data from helmsman/request/cjs-ns-shape-diff-inline-resend-r3, message `4a44dcc0-5e3a-4d3b-afdf-3f73d7a26ce1`.
- Extracted local scratch copy during FOUNDER probe: `/tmp/cjs-ns-shape-inline.json` (not a repository artifact).
- Runtime/module surfaces for follow-up cross-reference: CJS import namespace finalization and built-in/module shim namespace construction.
- Prior external-path attempts failed because `/media/jaredef/T7/...` and `/home/jaredef/Developer/cruftless-sidecar/parity-results/...` were not visible in this Codex filesystem namespace.

## III. Methodology

1. Phase 0: create this locale and refresh `apparatus/locales/manifest.json`.
2. Phase 2: use the inline 56-row JSON as the empirical source.
3. Segment rows by `extra_in_rb`, `missing_in_rb`, mixed, and no-key-diff/null-count artifacts.
4. Sample at least eight packages across large positive, large negative, mixed, and small-diff ranges; record exact extra/missing keys.
5. Apply C4: one mechanism bucket must account for at least 40% of the 56 rows to justify a Phase 3 move.
6. Do not edit runtime substrate in CNSDR-EXT 0.

## IV. Carve-Outs

- Native package install/runtime errors are not fixed here; rows with `rb_kc: null` are treated as namespace absent/completeness symptoms for segmentation only.
- Built-in module shim shape and userland package namespace extraction may require separate Phase 3 moves if C4 only passes at a broad family level.
- No package-manager or resolver substrate change is authorized in CNSDR-EXT 0.

## V. Resume Protocol

Read this seed, then `trajectory.md`. Resume by checking for a durable copy of the 2026-05-29 refined parity JSON or use the inline message ID above if the sidecar retains it. Then inspect CJS namespace finalization and built-in shim namespace construction before proposing runtime changes.
