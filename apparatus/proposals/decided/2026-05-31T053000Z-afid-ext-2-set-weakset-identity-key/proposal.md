---
helmsman_session: substrate-resolver-2026-05-31-iptd-chapter-close-carry-forward
proposed_commits:
  - pending
target_branch: main
summary: "AFID-EXT 2: Set + WeakSet ctor identity-stable storage key. Routes the AFID-EXT 1 ctor's storage write through Runtime::map_storage_key (the same encoding set_proto_add_via uses), so Object members compare by reference (`__objkey@{oid}`) instead of collapsing to abstract_ops::to_string ('[object Object]'). Closes Finding AFID.3."
risk_class: substrate
gates_pre:
  test262_full: null
gates_post:
  build: PASS
  probe_cells:
    - "new Set([o1,o2,1,2,2]).size === 4 (was collapsed to 3)"
    - "Set.has(o1) && .has(o2) on distinct objects"
    - "new WeakSet([o1,o2]).has(o1) && .has(o2) (was failing cell 6 in AFID-EXT 1 probe)"
    - "AFID-EXT 1 probe full: 7/7 PASS (was 6/7)"
---

## Substrate

Promotes `Runtime::map_storage_key` from private `fn` to `pub(crate) fn` (interp.rs:5221) and routes the AFID-EXT 1 ctor block (intrinsics.rs ~18220) through it instead of using `abstract_ops::to_string(&v).as_str().to_string()` directly. ~3 LOC delta.

## Verification

1. Build PASS (~1m 09s).
2. AFID-EXT 1 probe re-run: 7/7 PASS (cell 6 now passes; Finding AFID.3 closed).
3. Identity probe: `new Set([o1,o2,1,2,2]).size === 4`; has(o1), has(o2), has(1), has(2) all true; WeakSet([o1,o2]) preserves both.
4. cargo test --release -p rusty-js-runtime --lib: 74/0/1 preserved.
5. Regression sweep preserved: IPTD 7/7, cross-consumer 7/7, ICES-EXT 2 6/6, ICES-EXT 3.1 5/5, AFID-EXT 0 8/8.

## Authorization

Per keeper Telegram 10692 ("Continue") authorizing the AFID.3 follow-up.
