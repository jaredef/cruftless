# weakmap-upsert-methods — Seed

## Telos

Per TC39 upsert proposal (advanced to Stage 4 ES2025), both `Map.prototype.getOrInsert(key, value)` and `Map.prototype.getOrInsertComputed(key, callbackfn)` exist on `WeakMap.prototype` as well. cruft installed them on Map.prototype only (gated by `if !is_weak_proto` at MGOI-EXT 1).

Identified by EIPD.1 sweep extended to top-failure reasons: 32 test262 failures with "Map.prototype.getOrInsert: callee is not callable" — all under WeakMap path.

## Apparatus

- `pilots/rusty-js-runtime/derived/src/intrinsics.rs` Map/WeakMap install loop (line ~3485, MGOI registration at ~3517).
- `pilots/rusty-js-runtime/derived/src/interp.rs::map_proto_get_or_insert_via` + `map_proto_get_or_insert_computed_via` (works on `__map_data` storage; both Map and WeakMap share this storage shape per the existing dual-proto pattern at line 1571).

## Methodology

Remove the `if !is_weak_proto` gate; install on both. Brand check in the install loop is per-proto so WeakMap brand-check rejects Map receivers and vice versa.

## Carve-outs

- Spec mandates WeakMap.getOrInsert keys must be Object (or symbol-with-registry); the underlying impl doesn't enforce this for now — same carve-out as existing WeakMap.set.

## Resume protocol

Read `trajectory.md` tail.
