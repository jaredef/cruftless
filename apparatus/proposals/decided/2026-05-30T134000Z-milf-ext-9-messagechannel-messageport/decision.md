---
proposal_slug: 2026-05-30T134000Z-milf-ext-9-messagechannel-messageport
decision: APPROVED
arbiter_session: dyad-substrate-resolver-plus-helmsman-per-keeper-telegram-10536
decided_at: 2026-05-30T13:40:00Z
covers_commits:
  - 77e0209afb3512585a0fa703a6b9d08d8d610b7f
---

## Findings

Approved per keeper directive Telegram 10536 ("Let's do the next while we wait") authorizing a continuation rung during the in-flight top500 sweep.

Per-package audit of the postround11 top-FAIL cluster against the post-MILF-EXT-8.1 binary found seven packages already passing (redis, svgo, rehype, ramda, puppeteer-core, xlsx — newly green from the install-gaps + BigInt rungs) and seven still failing with seven distinct shapes. Cheerio's residual was the most tractable: `ReferenceError: MessagePort is not defined` in undici's `webidl.is.MessagePort = MakeTypeAssertion(MessagePort)` at module-init. MessageChannel was also missing.

Substrate commit `77e0209a` adds both stubs in `intrinsics.rs` next to the existing `BroadcastChannel` stub (same Tier-Ω.5.tttttt rationale):

- `MessagePort` constructor → `{postMessage, close, start, addEventListener, onmessage: null}`.
- `MessageChannel` constructor → `{port1: MessagePort-shaped, port2: MessagePort-shaped}`.

Each exposes a `prototype` with a `constructor` backref so `class X extends MessagePort {}` and webidl's MakeTypeAssertion brand-check both find the slots they need at module-init.

## Verification

1. `cargo build --release --bin cruft -p cruftless` — PASS.
2. `cargo test --release -p rusty-js-runtime --lib` — 74 passed, 1 ignored.
3. `cargo test --release -p cruftless --lib` — 11 passed.
4. Smoke: `typeof MessagePort === 'function'`, `typeof MessageChannel === 'function'`, `new MessageChannel()` returns `{port1, port2}`, port methods callable, `onmessage === null`.
5. **cheerio loads**: `import('cheerio')` now produces an Object with 7 keys (was `ReferenceError: MessagePort is not defined`).

## Named follow-ups (named, not landed)

From the same audit, remaining residuals at distinct coordinates:
- arktype: `ParseError: 'generic' is unresolvable` (parser tier).
- stylelint: `readFileSync` url-as-path bug (fs.url-shim coordinate).
- sequelize: `toString is not defined` (TDZ on module-context binding).
- slonik: `callee is not callable [argc=1] (callee='fn')` (CJS-export propagation).
- csv-parser: `value is not iterable (in-call='<destr.src>')` (destructure-of-non-iterable).
- mnemonist: missing intrinsic `MinFibonacciHeap`.

Each is a separate rung. The currently-running sweep (binary predates this rung) will not reflect MILF-EXT 9; a re-sweep would.
