---
helmsman_session: substrate-resolver-2026-05-31-iptd-chapter-close-carry-forward
proposed_commits:
  - pending
target_branch: main
summary: "AFID-EXT 0: rewrite Array.from's iterable branch to interleave next() + mapfn per ECMA-262 §23.1.2.1 step 6, with IteratorClose on mapfn abrupt completion per §7.4.9 step 4 (original throw preserved). Founds new locale pilots/array-from-interleave-discipline/. Adds runtime helper iter_close_rt for Rust-caller IteratorClose. Closes the Pi-OOM case surfaced during ICES residual probe."
risk_class: substrate
gates_pre:
  test262_full: null
  test262_sample: 88.7%
  diff_prod: 65 / 47
gates_post:
  build: "cargo build --release --bin cruft -p cruftless"
  probe_cells:
    - "Array.from with mapfn throwing on first element -> close iter + propagate mapfn error (no OOM)"
    - "Interleaved order n0,m0,n1,m1,n2,m2 on finite iter"
    - "Array.from([1,2,3]) regression"
    - "Array.from(Set) regression"
    - "Array.from('abc') regression"
    - "mapfn (value, index) args correct"
    - "mapfn throw + non-callable iter.return -> ORIGINAL error preserved per §7.4.9 step 4"
    - "next() non-Object -> TypeError (no OOM)"
---

## Substrate

### Runtime helper iter_close_rt

`iter_close_rt(rt, iter_id) -> Result<(), RuntimeError>` (intrinsics.rs, adjacent to `collect_iterable`). Rust-caller surface for ECMA-262 §7.4.9 IteratorClose. Returns silently for null/undefined .return; throws TypeError for non-callable non-null/undefined; calls .return() with this=iter and checks Object result otherwise. Mirrors the JS-callable `__destr_iter_close` helper from IPTD-EXT 1.

### array_from_via rewrite (~55 LOC)

Branch on iterable-vs-array-like once upfront. Iterable branch becomes a per-element loop:
- Call iter.next(); check result Object (else TypeError); read .done; if done, break.
- Read .value; if mapping, call mapfn(value, k); on Err, call `iter_close_rt(self, iter_id)` best-effort and return the ORIGINAL Err.
- Write result into out at index k; k += 1.

§7.4.9 step 4 preservation: the close call's error is discarded (`let _ = iter_close_rt(...)`) so the original throw shadows. AFID-EXT 0 ships the spec-correct semantics at the Array.from surface; ICES-EXT 3's analogous bytecode-tier shape has the inverse bug and is surfaced as Finding AFID.1 for follow-up.

Array-like + string branches preserved unchanged.

## Findings surfaced (chapter-close-inspect, Rule 15)

- **AFID.1** (cross-locale): ICES-EXT 3 close-throw spec divergence. The synthetic catch stub at the bytecode tier propagates the close-thrown error instead of preserving the body-thrown error per §7.4.9 step 4. Probe `/tmp/probe-ices-ext-3.js` cell 4 asserts the wrong shape and was confirmed PASS by the wrong-shape substrate. Both need correction; fix is to wrap the close call in a nested synthetic try-catch that swallows close-thrown values before re-throwing the spilled original. Surfaces as a separate rung in the ICES locale (candidate ICES-EXT 3.1).
- **AFID.2**: eager-collect-then-process is a recurring anti-pattern at runtime-tier intrinsics consuming iterables. Confirmed at array_from_via (pre-rung). Probe-pending at array_proto_concat_via, array_proto_flat_via. Candidate for cross-intrinsic audit.

## Verification

1. `cargo build --release --bin cruft -p cruftless` PASS (~1m 06s).
2. Direct probe `/tmp/probe-afid-0.js` (8 cells): 8/8 PASS.
3. Regression sweep (4 prior probes, all under `ulimit -v 2 GiB`):
   - Original 7-cell IPTD: 7/7 preserved
   - Cross-consumer 7-cell: 7/7 preserved
   - ICES-EXT 2 6-cell: 6/6 preserved
   - ICES-EXT 3 9-cell: 9/9 preserved (cell 4 asserts wrong-shape per Finding AFID.1)
4. Manifest refresh: 232 → 233 (AFID locale founded).

## Composes-With

- `pilots/iterator-protocol-throw-discipline/` IPTD-EXT 1 (helper-tier `__destr_iter_close` discipline mirrored at Rust-caller surface)
- `pilots/iterator-close-emission-sites/` ICES-EXT 1/2/3 (bytecode-tier sibling; AFID is runtime-tier)
- `apparatus/docs/predictive-ruleset.md` Rule 15 (chapter-close-inspect surfaced this), Rule 17 (segmentation: Array.from-only)
- Doc 721 SAMPLE.1 chain-bundle decomposition (this rung is the Array.from-specific sub-bundle)

## Authorization

Per keeper Telegram 10684 ("Continue") authorizing the residual surface from the ICES chapter-close-inspect probe.
