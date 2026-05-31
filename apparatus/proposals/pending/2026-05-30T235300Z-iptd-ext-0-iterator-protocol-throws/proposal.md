---
helmsman_session: helmsman-2026-05-30-drift-recovery
proposed_commits:
  - pending
target_branch: main
summary: "IPTD-EXT 0: iterator-protocol TypeError-throw discipline at `__destr_iter_next` (ECMA-262 §7.4.5 step 3) and `__destr_iter_close` (§7.4.9 step 2 via §7.3.10 GetMethod). Founds `pilots/iterator-protocol-throw-discipline/`. Third SAMPLE.1 sub-bundle per ASTA-EXT 0 decision §3."
risk_class: substrate
gates_pre:
  test262_full: null
  test262_sample: 88.7% (6816 PASS / 865 FAIL / 16 SKIP; 6817-6818 ±1 per Rule 29 falsifier)
  diff_prod: 65 PASS / 47 FAIL (post-ASTA-EXT 0)
  per_locale:
    tamm_cluster: 87 / 100
    tawr_cluster: 71 / 100
gates_post:
  build: "cargo build --release --bin cruft -p cruftless (pending)"
  runtime_lib_tests: "cargo test --release -p rusty-js-runtime --lib (pending)"
  diff_prod: ≥ 65/47 (regression gate); target +1 to +3 PASS from iterator-protocol cells
  per_locale:
    tamm_cluster: ≥ 87/100 (regression gate)
    tawr_cluster: ≥ 71/100 (regression gate)
  probe_cells:
    - "iter.next() returns 42 → for (x of {[Symbol.iterator](){return {next(){return 42}}}}) throws TypeError"
    - "iter.next() returns undefined → throws TypeError"
    - "iter.return is 42 (non-callable, non-null, non-undefined) → IteratorClose throws TypeError"
    - "iter.return is null → IteratorClose returns undefined silently (positive control)"
    - "iter.return is undefined → IteratorClose returns undefined silently (positive control)"
    - "iter.return is a callable function → IteratorClose invokes it (positive control)"
    - "iter.next() returns {value, done} object → for-of body runs normally (positive control)"
---

## Substrate Moves

Third SAMPLE.1 sub-bundle from the Doc 721 chain-bundle decomposition (findings-ledger Entry 016). ASTA-EXT 0 decision `apparatus/proposals/decided/2026-05-30T200000Z-asta-ext-0-array-frozen-throw/decision.md` §3 named the siblings; ASTA-EXT 0 closed the Array sub-bundle, ASTA-EXT 1 closed the put-const sub-bundle (in-locale). This rung closes the iterator-protocol sub-bundle.

**Tier-mismatch observation against parent locale's seed**: `pilots/array-strict-throw-discipline/seed.md` §Carve-outs predicted "Iterator-protocol TypeError throws (for-of cluster of 5 cells): deferred to a parser/IR locale." The actual emit sites are in the Runtime tier, not parser/IR. The runtime helpers `__destr_iter_next` and `__destr_iter_close` (registered via `register_engine_helper`) are invoked by the bytecode compiler's for-of lowering; the throw discipline lives at the helper site, not at the AST/IR site. The carve-out's tier prediction was wrong by one layer, surfaced at probe time. Trajectory entry will record this as a Phase 2 baseline-inspect observation per Rule 23 (founding-time substrate-coordinate correction).

### Closure A: IteratorNext result-type check (ECMA-262 §7.4.5 step 3)

Helper: `__destr_iter_next` in `pilots/rusty-js-runtime/derived/src/intrinsics.rs:7586`.

Pre-rung: helper returns whatever `iter.next()` produced, including non-Object values. Spec §7.4.5 step 3 mandates: "If Type(result) is not Object, throw a TypeError exception."

Substrate (+10 LOC):

```rust
let result = rt.call_function(next_fn, Value::Object(iter_obj), Vec::new())?;
if !matches!(result, Value::Object(_)) {
    return Err(RuntimeError::TypeError(
        "iterator.next() returned a non-object value".into(),
    ));
}
Ok(result)
```

Closes the for-of iterator-next-result-type test cell plus sibling iterator-protocol residual cells per the SAMPLE.1 chain-bundle decomposition. The destructuring lowering invokes `__destr_iter_next` once per element; a non-Object result silently propagated through downstream `value`/`done` access without the spec-mandated throw.

### Closure B: IteratorClose GetMethod throw (ECMA-262 §7.4.9 step 2 via §7.3.10 GetMethod)

Helper: `__destr_iter_close` in `pilots/rusty-js-runtime/derived/src/intrinsics.rs:7604`.

Pre-rung: helper returned `Value::Undefined` for any non-callable `return` value. Spec §7.4.9 step 2 routes through §7.3.10 GetMethod; GetMethod throws TypeError when the value is non-null, non-undefined, and non-callable. The prior silent-undefined path masked the spec-mandated throw.

Substrate (+6 LOC, replacing the prior 3-line silent-return):

```rust
if !matches!(ret, Value::Object(_)) {
    return Err(RuntimeError::TypeError(
        "iterator.return is not callable".into(),
    ));
}
```

Closes the for-of iterator-close-non-throw-get-method-non-callable test cell.

### Locale founding

Founds `pilots/iterator-protocol-throw-discipline/` with seed.md + trajectory.md per Doc 581 / Doc 737. Telos: materialize the engine-DAG coordinate

```
runtime/iterator-protocol :: E2/internal-method:abstract-op :: throw-discipline :: TypeError-on-non-conforming-iterator-result-or-return
```

across the destructuring-iteration helper surface. Locale scope is engine-helper registration site for `__destr_iter_*`; the bytecode-compiler emission sites that invoke these helpers are out of scope (carry-forward to a parser/IR sibling if downstream emission-site drift surfaces).

Updates `apparatus/locales/manifest.json` (231 → 232) via `apparatus/locales/discover.sh`.

## Verification

1. `cargo build --release --bin cruft -p cruftless`: must PASS.
2. `cargo test --release -p rusty-js-runtime --lib`: must PASS with no new failures.
3. Direct probe (`/tmp/probe-iptd-0.js`) 7 cells PASS:
   - 3 throw-cells (non-Object next result; non-callable return value via two shapes).
   - 4 positive controls (null return; undefined return; callable return; normal Object next result).
4. Regression gate TAMM ≥ 87/100.
5. Regression gate TAWR ≥ 71/100.
6. Regression gate diff-prod ≥ 65/47.
7. Optional yield gate: test262-sample re-run, target +1 to +3 PASS from iterator-protocol cells in scope. Per ASTA-EXT 0 decision §Doc 721 false-pass amendment, the sample-paths curation may not include the bulk of the 5 chain-bundle cells; gate is non-falsifying for substrate correctness.

## Risk Assessment

- **Blast radius**: contained to two engine-helper registrations at fixed offsets in intrinsics.rs. The helpers are dispatch-only consumers; no callers other than the bytecode-compiler for-of lowering exist.
- **Helper-only emission**: throw-discipline drift detected at the helper, not at the per-emission compiler call site. Closure at the helper centralizes; no per-call-site audit needed at this rung (a Rule 20 cross-emission-site audit at the compiler tier is carry-forward).
- **IteratorClose semantics**: §7.4.9 + §7.3.10 specify a four-way classification (null → undefined silently; undefined → undefined silently; callable → invoke; non-callable other → throw TypeError). Closure B's diff narrows the prior over-broad silent-undefined branch to match the classification. Verified against ASTA-EXT 1's similar narrow-check pattern (parallel-emit-site discipline per Finding ASTA.2).
- **Rule 13 negative-result discipline**: if the probe or regression gates fail, revert and surface the deeper-layer closure (likely IteratorClose's interaction with abrupt-completion threading per §7.4.9 step 6).
- **Rule 17 segmentation**: scoped to iterator-protocol per SAMPLE.1 sub-bundle decomposition. Sibling sub-bundles (Map/Set/WeakMap/WeakSet frozen-receiver, Promise dispatcher receiver-validation, Object.assign throw-propagation) remain separate substrate-spawns.
- **Determinism**: substrate is purely deterministic. Rule 29 ±1 sample variance is runner-side (DET.4 candidate) and orthogonal to this rung.

## Composes-With

- `apparatus/docs/predictive-ruleset.md` Rule 11 (5-axis pre-spawn coverage), Rule 13 (revert-then-deeper-layer-closure on negative), Rule 17 (sibling sub-bundle segmentation), Rule 20 (parallel-emit-site discipline coherence; Finding ASTA.2 instance candidate), Rule 23 (baseline-inspect at founding; surfaces the tier-mismatch observation against ASTA seed Carve-outs), Rule 27 + Rule 28 (substrate-spec-correctness; the silent-undefined helper was the architectural conflict; rectification at the helper tier is the cascade-revival shape), Rule 29 (post-land measurement).
- Corpus Doc 721 (chain-walk applied to SAMPLE.1 iterator-protocol sub-bundle).
- `apparatus/docs/findings-ledger.md` Entry 016 (SAMPLE.1 chain-bundle).
- `apparatus/proposals/decided/2026-05-30T200000Z-asta-ext-0-array-frozen-throw/decision.md` §3 (sibling sub-bundle enumeration; this rung executes the iterator-protocol entry).
- `pilots/array-strict-throw-discipline/` (sibling locale; ASTA-EXT 0 and EXT 1 closed Array + put-const sub-bundles).
- `pilots/iterator-close-emission-sites/`, `pilots/iterator-close-on-abrupt/` (adjacent locales at the compiler-emission tier; carry-forward boundary for any downstream emission-site work surfaced post-rung).

## Authorization

Awaiting keeper APPROVED decision. Substrate diff already drafted in worktree (intrinsics.rs uncommitted, the drift item recovered by helmsman drift-analysis at Telegram 10646); landing bundle on approval is one commit: locale scaffolding (seed.md + trajectory.md) + intrinsics.rs hunks + this proposal + the arbiter/keeper decision flip + manifest refresh.
