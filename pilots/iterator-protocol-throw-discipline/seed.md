# iterator-protocol-throw-discipline (IPTD) — Seed

## Telos

Materialize the engine-DAG coordinate

```
runtime/iterator-protocol :: E2/internal-method:abstract-op :: throw-discipline :: TypeError-on-non-conforming-iterator-result-or-return
```

across the destructuring-iteration helper surface in the Runtime tier. The for-of destructuring lowering invokes engine-helper sites `__destr_iter_next` and `__destr_iter_close` (registered via `register_engine_helper` in `pilots/rusty-js-runtime/derived/src/intrinsics.rs`); these helpers consume `iter.next()` results and `iter.return` references on behalf of the bytecode-compiler emission. The spec mandates TypeError at two specific failure shapes that the helpers had previously swallowed silently. This locale closes both at the helper tier.

## Origin

Founded 2026-05-30 per keeper APPROVED of helmsman proposal `apparatus/proposals/pending/2026-05-30T235300Z-iptd-ext-0-iterator-protocol-throws/proposal.md` (Telegram 10650). Third substrate-spawn from findings-ledger Entry 016 (SAMPLE.1) Doc 721 chain-bundle decomposition, following ASTA-EXT 0 (Array sub-bundle, commit 00a73363) and ASTA-EXT 1 (put-const sub-bundle, commit 371caf79).

The parent ASTA-EXT 0 decision (`apparatus/proposals/decided/2026-05-30T200000Z-asta-ext-0-array-frozen-throw/decision.md` §3) enumerated this sub-bundle as "iterator-protocol TypeError throws (for-of 5 cells)"; the ASTA seed §Carve-outs predicted "parser/IR locale" but the actual emit sites are in the Runtime tier. The tier-mismatch is recorded in trajectory.md as a Rule 23 baseline-inspect observation.

## Work shape

**Heuristics §IV classification**: D (Runtime abstract-op semantics) at the iterator-protocol surface.

Two helper-tier closures:

- **`__destr_iter_next`** (intrinsics.rs:7586): add ECMA-262 §7.4.5 step 3 type check. After the iterator's `next` method returns, if the result is not an Object, throw TypeError. Previously the non-Object result propagated silently to downstream `value`/`done` access.
- **`__destr_iter_close`** (intrinsics.rs:7604): narrow the over-broad silent-undefined branch to match §7.4.9 step 2 / §7.3.10 GetMethod classification. The four-way: null returns undefined silently; undefined returns undefined silently; callable invokes; non-callable other throws TypeError. The prior code returned undefined for any non-callable, masking the spec-mandated throw.

## Apparatus

- **Direct probe**: `/tmp/probe-iptd-0.js` (7-cell assertion suite). 7/7 PASS post-rung.
- **Cluster regression instruments**: TAMM ≥87, TAWR ≥71, diff-prod ≥65/47.
- **Canonical sample**: test262-sample yield optional gate; per ASTA-EXT 0 decision §Doc 721 false-pass amendment, sample-path curation may not include the bulk of the 5 chain-bundle cells. Substrate correctness is verified by the direct probe.

## Methodology

Per Doc 744 four-tuple plus Doc 721 chain-walk plus Rule 17 segmentation: this locale scopes against the iterator-protocol sub-bundle of SAMPLE.1, NOT the full cross-family residual or the put-const adjacent sub-bundle (the latter landed in-locale at ASTA-EXT 1). Sibling Doc-721 sub-bundles (Map/Set/WeakMap/WeakSet frozen-receiver throw; Promise dispatcher receiver-validation; Object.assign throw-propagation) remain separate substrate-spawns.

## Carve-outs

- **Bytecode emission-site audit**: out of scope. The bytecode-compiler for-of lowering calls `__destr_iter_next` / `__destr_iter_close` from one emission site each; downstream emission-site drift (per Rule 20 cross-emission-site coherence) would surface as a separate rung if found.
- **Async iterator protocol**: out of scope at founding. `__destr_async_iter_*` helpers, if they exist as parallel sites, would be a sibling rung under the same locale; deferred until evidence surfaces.
- **`return` callable-but-throws path**: §7.4.9 step 4 specifies that an abrupt completion from the `return` call propagates; this is already handled correctly by the `?` operator in the existing helper code. No substrate change needed.

## Composes-with

- `apparatus/docs/predictive-ruleset.md` Rules 11 (5-axis pre-spawn), 13 (revert-then-deeper-layer-closure on negative), 17 (sibling sub-bundle segmentation; this locale scopes against the iterator-protocol sub-axis), 20 (parallel-emit-site discipline; latent Finding ASTA.2 instance), 23 (baseline-inspect at founding; surfaces the tier-mismatch observation against ASTA seed Carve-outs), 27 + 28 (substrate-spec-correctness; helper-tier rectification per cascade-revival shape), 29 (post-land measurement).
- Corpus Doc 721 (chain-walk applied to SAMPLE.1 iterator-protocol sub-bundle).
- `apparatus/docs/findings-ledger.md` Entry 016 (SAMPLE.1 chain-bundle).
- `apparatus/proposals/decided/2026-05-30T200000Z-asta-ext-0-array-frozen-throw/decision.md` §3 (parent decision enumerating siblings).
- `pilots/array-strict-throw-discipline/` (sibling locale; SAMPLE.1 Array + put-const sub-bundles).
- `pilots/iterator-close-emission-sites/`, `pilots/iterator-close-on-abrupt/` (adjacent locales at compiler-emission tier; carry-forward boundary).
