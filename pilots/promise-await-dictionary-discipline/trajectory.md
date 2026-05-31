# promise-await-dictionary-discipline — Trajectory

## PAKD-EXT 0 — LANDED (2026-05-31) — Promise.allKeyed + Promise.allSettledKeyed

**Trigger**: Keeper Telegram 10755 (Stage 1 proposal triage).

**Substrate** (~280 LOC, interp.rs + promise.rs):
- `promise_all_keyed_via` / `promise_all_settled_keyed_via` static-method registrations on the Promise constructor with this-Object validation (matching PSCV-EXT 0).
- Shared `promise_all_keyed_interleave` helper parameterized by a `settled: bool` flag. Iterates own enumerable string keys via `ordinary_own_enumerable_string_keys`; per key, allocates a per-element resolve closure (allKeyed: writes raw value; allSettledKeyed: writes `{status:'fulfilled', value}`) and chains `.then(resolve_element, cap_reject)` (allKeyed) or `.then(resolve_element, reject_element)` (allSettledKeyed). Reject closure for allSettledKeyed wraps `{status:'rejected', reason}`. Remaining-counter primitive shared with PIID (`cell_array_new_via`).
- Result object: `Object::new_ordinary()` allocated then `proto = None` set explicitly (alloc_object auto-wires Object.prototype when proto is None; bypass needed for the null-proto requirement).

**Yield**:

```text
PAKD 14-cell probe: 14/14 PASS
  exists + length === 1 (4)
  happy path -> {a:1, b:2, c:3} null-proto
  empty obj -> empty null-proto
  non-Object arg (undefined/null/86/'s') -> TypeError (4)
  non-enumerable properties ignored
  allSettledKeyed mixed -> {a:{fulfilled,1}, b:{rejected,'boom'}, c:{fulfilled,3}}
  allKeyed reject on first reject
  allKeyed(fn) -> only own-enum {key:val}

cargo test --release -p rusty-js-runtime --lib: 74/0/1 preserved.
Full regression sweep preserved (12 PCSC + iter-protocol probes).
```

**Status**: PAKD-EXT 0 LANDED. Promise.allKeyed + Promise.allSettledKeyed test262 cells (35 expected) should now be eligible to PASS.
