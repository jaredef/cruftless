---
helmsman_session: substrate-resolver-2026-05-31-iptd-chapter-close-carry-forward
proposed_commits:
  - pending
target_branch: main
summary: "PIID-EXT 4: wire make_aggregate_error_via to AggregateError.prototype so the resulting object is a proper AggregateError instance per §20.5.7.4 / §27.2.4.3. Sets [[Prototype]] to AggregateError.prototype + internal_kind Error; installs message + errors as non-enumerable own; name inherits from prototype. Closes Finding PIID.2."
risk_class: substrate
gates_post:
  build: PASS
  probe_cells:
    - "constructor.name === 'AggregateError' (was 'Object')"
    - "instanceof AggregateError"
    - "instanceof Error (AggregateError extends Error)"
    - "message canonical"
    - "errors array shape"
    - ".name on prototype, .message + .errors own non-enumerable"
gates_post_results:
  probe: 9/9 PASS
  piid_123_re_run: 12/12 PASS (was 11/12)
---

## Substrate

~40 LOC at `make_aggregate_error_via` (interp.rs:3435). Lookup `AggregateError.prototype` via `global_get` + `object_get(cid, "prototype")`. New object's `internal_kind = Error`, `proto = Some(pid)`. Install `message` and `errors` as non-enumerable own per spec; `name` inherits from prototype.

## Verification

1. Build PASS (~1m 12s).
2. PIID-EXT 4 9-cell probe: 9/9 PASS.
3. PIID-EXT 1+2+3 re-run: 12/12 PASS (was 11/12; closes the AggregateError shape cell).
4. cargo test 74/0/1 preserved.
5. Regression sweep preserved: IPTD 7/7, cross-consumer 7/7, ICES-EXT 2 6/6, ICES-EXT 3.1 5/5, AFID-EXT 0 8/8, AFID-EXT 1 7/7, PIID-EXT 0 6/6.

## Authorization

Per keeper Telegram 10698 ("Push and continue") authorizing AFID.2 follow-up.
