# test262 Long-Tail: Shared-Upstream vs Mutually-Exclusive Cluster Analysis

**Date**: 2026-05-25 (post AEVPD+SDIBP+ASD landings; runnable rate 80.6%; 1416 fails remaining)
**Author**: 2026-05-25 session
**Status**: prospective — informs ECMAScript-parity arc prioritization for the next N rounds
**Composes with**:
- [Doc 740](../../docs/740-multi-tier-cascade-revival-when-the-hot-path-traverses-multiple-tiers-closing-one-tier-alone-is-insufficient.md) §III.5 — JSF-pilot-reread shape ("cumulative measurement materializes a different magnitude than the per-cluster projection")
- [Doc 742](../../docs/742-the-resolver-instance-pattern-at-full-strength-downstream-dispatch-and-upstream-elision-as-doc-729s-empirical-refinements-from-a-typescript-parity-research-arc.md) §V — upstream elision
- [T262C](../../pilots/test262-categorize/seed.md) — the matrix instrument this analysis reads
- [Finding AEVPD.2](../../pilots/array-exotic-virtual-property-discipline/trajectory.md) — cluster-level heterogeneity; matrix view over-aggregates when data-axis is heterogeneous within a cell

## I. Question

Given the post-ASD test262-sample distribution (1416 failures across ~30 named clusters), which clusters trace to a **shared upstream constraint** (one substrate fix → cascade across multiple clusters) versus which are **mutually-exclusive long-tail** (each cluster is its own heterogeneous pile of small-but-distinct bugs)?

The keeper's framing: discern induced-property structure vs orthogonal-bug accumulation.

## II. Method

1. Failure-reason histogram across all 1416 fails (lower-case-substituted to fold integer variants).
2. Per-reason cross-cluster spread: which test pipelines does each reason recur in?
3. Diagnostic rule:
   - **Shared-upstream signal**: a reason-shape that recurs across MULTIPLE pipelines/clusters with structural identity. One substrate fix at the upstream tier closes all instances.
   - **Mutually-exclusive signal**: reasons WITHIN one cluster are heterogeneous AND don't recur in other clusters. Each instance needs its own per-bug fix.
4. Per-shared-upstream candidate: name the substrate site, estimate the cascade, name the tier, name the blast-radius and risk.

## III. Findings

### III.1 Shared-upstream constraints (one substrate fix → wide cascade)

Five distinct upstream constraints account for an estimated **~340 of the 1416 fails** (~24% of remaining). Each is a single substrate site at a specific tier.

| # | Upstream constraint | Cascade (est.) | Tier | Cross-cluster spread |
|---|---|---:|---|---|
| 1 | **Strict-this-coercion + brand-check at host-method prologues** (RequireObjectCoercible / internal-slot brand check) | ~150 | runtime | 57 Array/proto + 35 Set/proto + 11 String/proto + 7 Map/proto + 5 WeakMap + 5 Object.assign + 4 WeakSet + 4 Promise + 11 misc |
| 2 | **Strict-mode tracking in parser** (yield/let/etc. gated by mode) | ~80 | parser | clusters yield-ident-{invalid,valid,expr} + generators+dstr + let-in-strict; ~5-6 matrix cells |
| 3 | **`$262` host-hook shim** (detachArrayBuffer, evalScript, global, gc, agent) | ~38 | host | spans Array/TypedArray/String/SharedArrayBuffer fixtures; one shim file unlocks most |
| 4 | **IteratorClose on abrupt completion** (§7.4.9; IPEP-EXT 2 deferred candidate) | ~25 | bytecode + runtime | for-of/dstr iter-nrml-close-err, iter-rtrn-close, iter-thrw-close subclusters |
| 5 | **More parser-permissiveness sites** (escaped-of, duplicate-arrow-params, for-in head-const-bound-names, for-in-with-destructure-head) | ~50 | parser | cluster 6 + 8 + 12 + 26 + scattered |

**Total shared-upstream cascade potential: ~340 tests across 5 substrate fixes (~70 PASS / fix amortized).**

#### III.1.1 Constraint #1 — strict-this-coercion + brand-check

**Mechanism**: per ECMA-262 §7.2.1 RequireObjectCoercible + per-host-method §X.X.Y step 1-3 brand checks (`If Type(this) is not Object, throw TypeError` / `If this does not have an [[InternalSlot]], throw TypeError`), the prologue of every host method must validate `this`. Cruft skips this prologue at most host methods — the method either no-ops silently, returns undefined, or attempts the operation and produces a downstream cruft-internal error (which surfaces in test262 as the host-error-bypass-JS-catch shape that PPA-EXT 1's eval-CompileError→SyntaxError lesson already addressed for one tier).

**Substrate site**: the host-method prologue. Either:
- (a) Augment each registered intrinsic method's wrapper with the prologue check (~150 sites, mechanical).
- (b) Add a discipline at `register_intrinsic_method` that auto-wraps with a configurable brand check (1 site, but requires per-method brand-spec metadata).

Option (b) is the Doc 729 §VI resolver-instance-style fix — uniform discipline at the registration site rather than per-method drift. Option (a) is the substrate-introduction-prefix per Doc 740 §IV.2.

**Blast radius**: high (touches every host-method registration). Risk: false-positive rejection of valid receivers if brand-check is too tight. Per Rule 14 mirror — adding restriction needs care.

**Cascade prediction**: ~150 tests; possibly larger if a previously-blocked tier surfaces.

#### III.1.2 Constraint #2 — strict-mode tracking in parser

**Mechanism**: ECMA-262 §13.2.1 Strict Mode Code + §13.1.1 Identifier Static Semantics: `yield`, `let`, `static`, `implements`, etc. are reserved in strict mode (and at module top-level). Cruft's parser doesn't track strict mode; it accepts `yield` and `let` as identifiers in all contexts (or rejects them in some specific contexts via the PPA `is_reserved_word` check which is mode-blind).

This produces TWO failure shapes simultaneously:
- onlyStrict tests where the parser accepts code that should be SyntaxError (cluster 3 yield-ident-invalid: ~27).
- noStrict tests where the parser rejects valid `var yield = 4` declarations (cluster 4 yield-ident-valid: ~26 — the runtime path for `var yield` silently breaks).

Both share the upstream constraint: parser must know whether it's compiling strict or sloppy source.

**Substrate site**: Parser state extension. Detect `"use strict"` prologue at function/script body start; propagate to inner contexts. Gate the reserved-word checks by mode.

**Blast radius**: medium (parser-tier; localized to identifier-reference / binding-identifier sites). Risk: stale-state on nested function scopes; per-script vs per-function strict tracking.

**Cascade prediction**: ~80 tests.

#### III.1.3 Constraint #3 — `$262` host-hook shim

**Mechanism**: test262 fixtures use `$262` for host-system probes — `$262.detachArrayBuffer`, `$262.evalScript`, `$262.global`, `$262.gc`, `$262.agent`. Cruft doesn't expose `$262` at all; tests that probe it die with "$262 is not defined".

**Substrate site**: one new helper file `cruftless/src/host_262.rs` (or inline in intrinsics.rs `install_test262_host_hooks`) installing `$262` as a global with stub or substrate-backed methods.

**Blast radius**: minimal (pure additive). Tests that only NAME `$262` (read it for capability detection) pass immediately. Tests that depend on its semantics (like `$262.detachArrayBuffer` actually detaching a buffer) need the underlying substrate too — but that's a separate downstream concern.

**Cascade prediction**: ~30-38 tests immediately; more if it unblocks test-prologue probes that were aborting.

#### III.1.4 Constraint #4 — IteratorClose on abrupt completion

**Mechanism**: per §7.4.9 IteratorClose, when a for-of body / destructure / spread / yield* exits abruptly (break, throw, return), the engine must call `iterator.return()` if it exists. Cruft's IPEP-EXT 1 closed iter-protocol for GetIterator + iter.next + iter-rest, but left IteratorClose for IPEP-EXT 2 (already-named candidate).

**Substrate site**: emit IteratorClose at the abrupt-completion paths in compile_stmt for-of body + emit_destructure / emit_destructure_assign Array paths. Add a `__destr_iter_close(iter)` helper.

**Blast radius**: bounded (touches the abrupt-completion paths only; normal-completion paths unchanged).

**Cascade prediction**: ~25 tests.

#### III.1.5 Constraint #5 — more parser-permissiveness sites

**Mechanism**: PPA-EXT 1 and SDIBP-EXT 1 closed two parser-permissiveness sites (object-binding-shorthand ReservedWord; Declaration-as-Statement-body). Several more remain:
- `for (var x of `escaped-of` …)` — `of` should be SyntaxError per §11.6.2 (contextual keyword can't be escape-formed) — ~15
- Arrow-function duplicate parameter names — ~22 (cluster 6)
- `for-in head-const-bound-names-in-stmt` — `for (const x in obj) var x;` should be SyntaxError per Early Errors — ~8
- `for-in with destructure head not yet supported` — explicit substrate gap (compile-time error, not parse) — ~7

Each is a small per-site fix. None share an upstream beyond PPA's general "parser-permissiveness-audit" coordinate.

**Cascade prediction**: ~50 tests across ~4 distinct sites.

### III.2 Mutually-exclusive long-tail (each cluster = its own heterogeneous bug pile)

These clusters do NOT share an upstream constraint with other clusters; the failures within each cluster are heterogeneous (different per-bug causes within the same cluster); and the closure cost is approximately one round per few-test win.

| Cluster | Tests | Heterogeneity |
|---|---:|---|
| Object.defineProperty edge cases | 37 + 8 | descriptor-semantics (writable, enumerable, configurable, accessor↔data, get-throws, etc.) — many distinct causes per AEVPD.2 |
| Array.prototype.reduce | 16 | accumulator handling, callable validation, length coercion — distinct sub-bugs |
| Array.prototype.indexOf | 16 | NaN handling, sparse arrays, negative-fromIndex — distinct |
| Array.prototype.map | 14 | callable validation, length coercion, sparse — distinct |
| Array.prototype.filter | 9 | similar to map |
| String.prototype.trim | 22 | Unicode whitespace classification — likely one substrate fix (~one cluster), but doesn't share with other clusters |
| String.prototype.split | 12 | regex/string heterogeneous |
| Promise edge cases | 12 | thenable handling, native-Promise detection, Promise.resolve identity — heterogeneous |
| JSON.stringify | 10 | replacer-array, replacer-function, indent, BigInt — distinct |
| JSON.parse | 7 | reviver-throws, Proxy interaction — distinct |
| RegExp.prototype.test | 13 | lastIndex semantics, named groups — distinct |
| Number constructor | 8 | coercion edge cases |
| for-of misc | 13 + 12 + 18 | each row a different completion-value / arguments-mapped / generator-close concern |

**Total mutually-exclusive: ~280 tests across ~30+ separate per-bug fixes.**

### III.3 Cascade-amortization comparison

| Surface | Tests | Substrate fixes | PASS / fix |
|---|---:|---:|---:|
| Shared-upstream (5 constraints) | ~340 | 5 | **~68** |
| Mutually-exclusive long-tail | ~280 | ~30+ | **~9** |
| **Leverage ratio** | | | **~7.5×** |

The shared-upstream surface dominates the long-tail's amortization profile. Per Doc 740 §III multi-tier R-identification, the five named constraints ARE the R-set for the remaining ECMAScript-parity arc.

## IV. Diagnostic methodology — generalization beyond this corpus

The discriminator that identifies shared-upstream vs mutually-exclusive is itself a corpus-grade methodology:

> **A cluster is shared-upstream when its dominant failure-reason shape recurs across MULTIPLE pipelines with structural identity. It is mutually-exclusive when reasons WITHIN one cluster are heterogeneous AND don't recur in other clusters.**

Operationalized as a two-step probe:
1. **Per-reason cross-cluster spread** — does this reason-shape appear in N>1 pipelines? If yes, candidate shared-upstream.
2. **Per-cluster reason-heterogeneity** — within this cluster, do the failure reasons form a tight pattern (shared-upstream) or scatter across many shapes (mutually-exclusive)?

A cluster passing both tests (cross-cluster recurrence + within-cluster homogeneity) has the highest amortization potential. A cluster failing both (within-cluster heterogeneity + reason-shape unique to this cluster) has the lowest amortization potential — and should be deferred or accepted as residual.

This methodology is general: any failure-table-driven test corpus (TCC, TXC, T262C, future corpora) can be probed the same way to identify shared-upstream substrate-fix candidates before committing to per-cluster work.

## V. Prospective arc strategy

Ordering by Doc 740 leverage × Rule 14-mirror risk × blast radius:

1. **`$262` host-hook shim** (constraint #3) — smallest blast radius; purely additive; ~38 cascade. *Quickest win.*
2. **IteratorClose on abrupt completion** (constraint #4) — modest scope, well-bounded; ~25 cascade. *Closes IPEP-EXT 2.*
3. **More parser-permissiveness sites** (constraint #5) — bounded per-site; ~50 cascade. *PPA-extension.*
4. **Strict-mode parser tracking** (constraint #2) — medium blast radius; ~80 cascade. *Parser-state extension.*
5. **Strict-this-coercion + brand-check discipline** (constraint #1) — largest blast radius; ~150 cascade. *Resolver-instance-style at register_intrinsic_method, or per-method-class.*

After these 5 substrates (estimated runnable rate gain: 80.6% → ~85%), the remaining work is the mutually-exclusive long-tail at ~9 PASS / round.

**Stopping decision**: each substrate round's empirical-vs-projected ratio is the discriminator for whether to continue or pivot. If a substrate's cascade comes in at <50% of projection, re-probe per Finding AEVPD.2 (per-cluster reason-heterogeneity check) before committing to the next.

## VI. Status

Prospective; awaits keeper authorization to proceed with constraint #3 or alternative.

---

*This doc may be promoted to the corpus tier as Finding T262C.4 (shared-upstream vs mutually-exclusive long-tail discrimination + leverage prediction) once the prospective predictions are empirically corroborated by at least 2 of the 5 named constraints.*
