# error-prototype-stack-accessor - Seed

**Locale tag**: `L.error-prototype-stack-accessor` (EPSA).

**Status**: FOUNDED at EPSA-EXT 0. Phase 0 spawn plus Phase 2 baseline probe only; no runtime substrate lands in this founding round.

**Parent arc**: ECMAScript parity / Error object substrate. Related prior locale: [`pilots/error-instance-property-descriptors/`](../error-instance-property-descriptors/seed.md), which fixed constructor-created Error instance descriptor shape but did not install the non-standard `Error.prototype.stack` accessor.

**Workstream**: V8/SpiderMonkey/JSC-compatible `Error.prototype.stack` accessor semantics for the test262 `feat:error-stack-accessor` surface. The post-EPSUA sample matrix surfaced `Error.prototype.stack` / `feat:error-stack-accessor` as rank 3 with 22 rows.

## I. Telos

Close the 22-cell `Error.prototype.stack` / `feat:error-stack-accessor` matrix cluster without folding it into unrelated stack-format work. The likely substrate is `pilots/rusty-js-runtime/derived/src/intrinsics.rs::install_error_globals`, where Error-family prototypes currently expose `name`, `message`, `constructor`, and `toString` but no own `stack` accessor.

The Phase 2 probe must discriminate whether the cluster is dominated by one accessor-presence defect or whether it splits into separate instance-data, accessor-behavior, setter, and stack-format coordinates.

## II. Apparatus

- Post-EPSUA matrix: `/home/jaredef/Developer/cruftless-r3/pilots/apparatus/test262-categorize/results/2026-05-29/{matrix.md,categorized.jsonl}`.
- Raw sample result source: `/home/jaredef/Developer/cruftless-r3-sidecar/results/test262-sample-2026-05-29/results.jsonl`.
- Test262 source: `/home/jaredef/test262/test/built-ins/Error/prototype/stack/*.js`.
- Runtime implementation: `pilots/rusty-js-runtime/derived/src/intrinsics.rs::install_error_globals` and `make_error_instance`.
- Prior related work: `pilots/error-instance-property-descriptors/{seed.md,trajectory.md}`.

## III. Methodology

1. Phase 0: create this locale and refresh `apparatus/locales/manifest.json`.
2. Phase 2: inspect the 2026-05-29 `Error.prototype.stack` matrix row and every exact `feat:error-stack-accessor` categorized entry.
3. Sample at least eight failures spanning getter retrieval, setter retrieval, prototype descriptor shape, Error instance own-data behavior, receiver validation, and assignment behavior.
4. Apply C4: proceed only if one mechanism bucket accounts for at least 40% of the narrowed 22-row cluster.
5. Propose a Phase 3 move shape but do not edit runtime code in EPSA-EXT 0.

## IV. Carve-Outs

- Human-readable stack string formatting and call-site frame capture are out of scope unless needed for the accessor tests' type/round-trip assertions.
- `Error.captureStackTrace`, `Error.prepareStackTrace`, and `Error.stackTraceLimit` remain in the existing V8-extension substrate unless a future probe shows they interact with the prototype accessor.
- Error instance descriptor shape already belongs to EIPD; this locale may reference it but should only change it when the inherited accessor makes it necessary.
- Cross-realm, Proxy, Reflect, and `__proto__` variants in the 35-row `Error.prototype.stack` pipeline marginal are adjacent probe coverage, not the exact 22-cell founding coordinate.

## V. Resume Protocol

Read this seed, then `trajectory.md`, then the EIPD trajectory. Resume by rechecking the latest test262 sample/full-suite matrix for `Error.prototype.stack`, then inspect `install_error_globals` before designing the Phase 3 accessor substrate.
