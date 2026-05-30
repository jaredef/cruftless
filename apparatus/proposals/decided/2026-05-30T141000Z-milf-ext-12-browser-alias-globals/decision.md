---
proposal_slug: 2026-05-30T141000Z-milf-ext-12-browser-alias-globals
decision: APPROVED
arbiter_session: dyad-substrate-resolver-plus-helmsman-per-keeper-telegram-10544
decided_at: 2026-05-30T14:10:00Z
covers_commits:
  - d0321266aab3eda7194afb0648f196d0c81356e2
---

## Findings

Approved per keeper directive Telegram 10544 ("Next pick"). Picked workerpool's `self is not defined` blocker. Two-stage gap: `self` missing, then immediately `navigator.hardwareConcurrency` after the first hurdle.

Substrate commit `d0321266` adds three globals next to the existing `globalThis` / `global` defines in `intrinsics.rs`:
- `self` = `globalThis` (HTML5 WorkerGlobalScope alias; isomorphic npm packages dispatch on `typeof self`).
- `window` = `globalThis` (browser-target compatibility shim).
- `navigator` = stub Object with `hardwareConcurrency: 1` (single-threaded engine) and `userAgent: "cruft/0.1 (node-compat)"`.

Same `{w:t, e:f, c:t}` descriptor as `global`/`globalThis` so libraries that reassign for testing are not blocked.

## Verification

1. `cargo build --release --bin cruft -p cruftless` — PASS.
2. `cargo test --release -p rusty-js-runtime --lib` — 74 passed, 1 ignored.
3. `cargo test --release -p cruftless --lib` — 11 passed.
4. Smoke: `typeof self === 'object'`, `self === globalThis`, `window === globalThis`, `navigator.hardwareConcurrency === 1`, `navigator.userAgent` string set.
5. **workerpool loads**: 10 keys (was `self is not defined` → `navigator.hardwareConcurrency` undefined).

## Compounding scope

Any browser-isomorphic package using `typeof self !== 'undefined'` or reading `navigator.userAgent` for environment detection benefits. Common across React-companion libs, web-worker pool managers, polyfill bundles, and packages built with `--target=neutral` bundlers.
