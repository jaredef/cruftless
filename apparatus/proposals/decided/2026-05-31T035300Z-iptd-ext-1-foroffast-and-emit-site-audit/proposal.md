---
helmsman_session: pending
proposed_commits:
  - pending
target_branch: main
summary: "IPTD-EXT 1: deeper-layer closure per Rule 13 following IPTD-EXT 0 NEGATIVE. Re-instates the helper-tier IteratorNext/IteratorClose throw discipline AND adds parallel coverage at the bytecode for-of slow-path emission (Op::ForOfFastNext fall-through site) plus a Rule 20 cross-emission-site audit of other iter.next() / iter.return consumers (spread-call, array-spread, Array.from, for-await, yield*). Closes the OOM-on-non-conforming-iterator regression that took out the Pi during IPTD-EXT 0."
risk_class: substrate
gates_pre:
  test262_full: null
  test262_sample: 88.7% (6816 PASS / 865 FAIL / 16 SKIP per 2026-05-30 canonical; unchanged by 7a1435d4 + af5326b7 net-zero)
  diff_prod: 65 PASS / 47 FAIL (post-ASTA-EXT 0)
  per_locale:
    tamm_cluster: 87 / 100
    tawr_cluster: 71 / 100
gates_post:
  build: "cargo build --release --bin cruft -p cruftless (under ulimit -v 2GiB during dev)"
  runtime_lib_tests: "cargo test --release -p rusty-js-runtime --lib"
  diff_prod: ≥ 65/47 (regression gate)
  per_locale:
    tamm_cluster: ≥ 87/100
    tawr_cluster: ≥ 71/100
  probe_cells:
    - "for (x of {[S.iterator](){return {next(){return 42}}}}) {} → TypeError (the IPTD-EXT 0 cell that OOMed)"
    - "for (x of {[S.iterator](){return {next(){return undefined}}}}) {} → TypeError"
    - "[...{[S.iterator](){return {next(){return 42}}}}] → TypeError"
    - "Array.from({[S.iterator](){return {next(){return 42}}}}) → TypeError"
    - "function* g(){ yield* {[S.iterator](){return {next(){return 42}}}}; } [...g()] → TypeError"
    - "let [a]={[S.iterator](){return {next(){return 42}}}} → TypeError (helper-tier; IPTD-EXT 0 substrate re-instated)"
    - "Positive control: normal iterator with {value, done} object → loop body runs"
    - "Positive control: iter.return is callable → IteratorClose invokes it"
    - "Positive control: iter.return is null/undefined → IteratorClose returns undefined silently"
  forensic_gates:
    - "All dev probes run under ulimit -v 2097152 (2 GiB) until the locale closes; non-throw paths must not allocate unbounded"
    - "Persistent journald enabled (already done 2026-05-31 03:46Z) so any future hard-fault leaves a trace"
---

## Substrate Moves

Per Rule 13: IPTD-EXT 0 NEGATIVE was reverted in commit `af5326b7`; trajectory entry at `pilots/iterator-protocol-throw-discipline/trajectory.md` records the diagnosis. The deeper-layer closure is parallel emit-site coverage: helper site (covers destructuring) + opcode site (covers plain for-of) + cross-consumer audit (covers spread/Array.from/yield*/for-await).

### Re-instated from EXT 0 (substrate prefix per Rule 13 amortization)

- `__destr_iter_next` (intrinsics.rs:7586): ECMA-262 §7.4.5 step 3 — throw TypeError when iter.next() returns non-Object.
- `__destr_iter_close` (intrinsics.rs:7604): §7.4.9 step 2 via §7.3.10 GetMethod — throw TypeError when iter.return is non-null, non-undefined, non-callable.

Substrate identical to af5326b7's reverted hunks.

### Closure C: for-of slow-path IteratorNext type check at the bytecode dispatch site

`Op::ForOfFastNext` (interp.rs:14960) is a fast-path for the well-known ArrayIterator shape; on non-ArrayIterator (e.g., custom user-defined iterables), it falls through to the slow path emitted immediately after by the compiler at `pilots/rusty-js-bytecode/derived/src/compiler.rs:2292`. The slow-path emission must enforce §7.4.5 step 3 at the iter.next() result consumption site.

The exact closure shape (new dedicated opcode `Op::IteratorNextChecked`, or guard at the existing dispatch with a Result-threaded TypeError) is to be determined during Phase 2 baseline-inspect against the slow-path compiler emission. Either way the throw must occur BEFORE the runtime reads `.value` / `.done` on the result, since non-Object .done coerces to undefined→false and drives the unbounded-iteration regression that took out the Pi.

### Closure D: Rule 20 cross-emission-site audit

Iter.next() / iter.return are consumed at multiple emission sites within the Runtime tier. Audit and bring each to spec at the same rung (Rule 20 parallel-emit-site coherence, Finding ASTA.2 instance candidate):

- spread-call argument list (`...iter` in CallExpression arguments)
- array-spread literal (`[...iter]`)
- Array.from with iterable argument
- yield* expression in generators
- for-await-of (async iteration; §7.4.6 Async IteratorNext has the same step-3 type check, applied to the awaited result)
- destructuring rest element (`[a, ...rest] = iter`) — the `__destr_iter_rest` helper at intrinsics.rs:7635 already has its own iter.next() loop; verify the same type-check discipline.

Per-consumer the rung either confirms spec-correct (no change), adds the type-check (substrate diff), or surfaces a sub-locale for a separable dispatch concern (e.g., async-iter has its own helper surface).

### Locale founding (continuation of IPTD-EXT 0 scaffold)

`pilots/iterator-protocol-throw-discipline/seed.md` already exists from EXT 0's scaffold; scope is unchanged. `trajectory.md` exists with the EXT 0 NEGATIVE entry. EXT 1 lands as the second trajectory entry.

Updates `apparatus/locales/manifest.json` via `apparatus/locales/discover.sh` (manifest refresh deferred from EXT 0 since the negative reverted before refresh).

## Verification

1. `cargo build --release --bin cruft -p cruftless` — must PASS.
2. `cargo test --release -p rusty-js-runtime --lib` — must PASS.
3. Direct probe (regenerated `/tmp/probe-iptd-1.js`, 9+ cells covering helper + opcode + cross-consumer surfaces) — all PASS, **all dev runs under `ulimit -v 2097152`** until the locale closes.
4. Regression gates: TAMM ≥ 87/100, TAWR ≥ 71/100, diff-prod ≥ 65/47.
5. Forensic gate: confirm no allocation spike >100 MiB on any probe cell (rules out residual unbounded-loop paths at adjacent emit sites the audit may have missed).
6. Optional test262-sample re-run, target +1 to +3 PASS from iterator-protocol cells (Doc 721 false-pass amendment caveat from EXT 0 still applies).

## Risk Assessment

- **Blast radius**: wider than EXT 0 (multi-emit-site audit + opcode dispatch change). Mitigated by Rule 20 parallel coherence: discovering all consumers at one rung is the apparatus-rational shape; per-consumer staggered rungs would accumulate the same forensic risk repeatedly.
- **Pi-survival under dev**: `ulimit -v 2097152` is mandatory on every dev probe until the locale closes. The EXT 0 OOM took out the host; the same probe under `ulimit` aborts cleanly with backtrace. Persistent journald now enabled for any future hard-fault traceability.
- **Opcode dispatch change**: `Op::ForOfFastNext` itself is unchanged (fast-path stays). The closure attaches at the slow-path emission immediately following the opcode, which is currently compiler-emitted bytecode that calls iter.next() through the normal call-function path. Either a new dedicated opcode or a guard at the existing dispatch — design choice deferred to Phase 2 baseline-inspect.
- **Rule 13 application**: EXT 0's reverted substrate is the cheap enabler (helper-tier discipline is already drafted-and-validated against spec; re-applying it costs only re-committing the diff). The deeper-layer closure is the audit + opcode site, not a redesign of the helper-tier closure.
- **Rule 17 segmentation**: scope is iterator-protocol throw discipline across Runtime-tier consumers. Async-iter (§7.4.6) is in scope if the audit surfaces a parallel pattern; if it requires a different dispatch surface (e.g., its own helper), surface a sub-locale at that rung's discovery.

## Composes-With

- `apparatus/docs/predictive-ruleset.md`: Rule 13 (revert-then-deeper-layer-closure — this proposal IS the deeper-layer closure of EXT 0), Rule 17 (sibling sub-bundle segmentation), Rule 20 (parallel-emit-site discipline — Finding ASTA.2 instance), Rule 23 (baseline-inspect; the EXT 0 founding-time correction was incomplete — this rung completes it), Rule 27 + Rule 28 (substrate-spec-correctness).
- Corpus Doc 721 (SAMPLE.1 chain-bundle decomposition; iterator-protocol sub-bundle).
- `pilots/iterator-protocol-throw-discipline/trajectory.md` — IPTD-EXT 0 NEGATIVE diagnosis.
- `apparatus/proposals/pending/2026-05-30T235300Z-iptd-ext-0-iterator-protocol-throws/proposal.md` — EXT 0 proposal (should be moved to a `decided/` with NEGATIVE disposition by arbiter at this rung's landing).
- Adjacent locales by tier (carry-forward boundaries): `pilots/iterator-close-emission-sites/`, `pilots/iterator-close-on-abrupt/`.

## Authorization

Awaiting keeper APPROVED decision. The EXT 0 NEGATIVE landed via the keeper's authorization of the revert + this proposal (Telegram 10667). Landing bundle on approval: re-instated helper-tier hunks + slow-path opcode-site closure + cross-consumer audit results + trajectory.md EXT 1 entry + this proposal flipped to `decided/` + manifest refresh.

**Forensic constraint**: every dev probe run under `ulimit -v 2097152` until the locale closes. Non-negotiable for this rung given the EXT 0 Pi-reset.
