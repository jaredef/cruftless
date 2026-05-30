---
proposal_slug: 2026-05-30T135500Z-milf-ext-11-buffer-iterator
decision: APPROVED
arbiter_session: dyad-substrate-resolver-plus-helmsman-per-keeper-telegram-10540
decided_at: 2026-05-30T13:55:00Z
covers_commits:
  - a53fdf363412c754ac049ee6aed41b3ad4090242
---

## Findings

Approved per keeper directive Telegram 10540 ("Pick next cluster"). Picked csv-parser's `value is not iterable (no @@iterator) (in-call='<destr.src>')` blocker.

Two-stage diagnosis:
1. Buffer instances had no `@@iterator` slot — destructure dead-ended on "value is not iterable (no @@iterator)".
2. Aliasing `@@iterator` to the pre-existing `values()` (which returns a bare Array, not an Iterator) hit the next protocol step: the destructure protocol's `__destr_iter_step` calls `.next()` on whatever `@@iterator` returned, and Arrays don't have `.next()`.

Substrate commit `a53fdf36` adds a real Iterator factory inside `install_buffer_methods`: each call to `buf[Symbol.iterator]()` returns a fresh iterator carrying closure state on itself via internal slots (`__i` cursor, `__src` source-buffer `Value::Object`, `__len` cached length) and exposes a `.next()` method returning `{value, done}`. Storing the source as `Value::Object(this_id)` avoids needing the `rusty-js-gc` crate dep — Object values ARE ObjectRefs wrapped.

## Verification

1. `cargo build --release --bin cruft -p cruftless` — PASS.
2. `cargo test --release -p rusty-js-runtime --lib` — 74 passed, 1 ignored.
3. `cargo test --release -p cruftless --lib` — 11 passed.
4. Smoke `/tmp/smoke/bufiter.mjs`: `typeof Buffer.from('\r')[Symbol.iterator] === 'function'` (was undefined); `const [cr] = Buffer.from('\r')` yields `cr === 13`.
5. **csv-parser loads**: 3 keys (was the destructure error).

## Named follow-up

`Buffer.from([10, 20, 30])` returns a buffer with length=0 — the indexed slots aren't being populated for the array-input shape. The current Buffer.from-array path is broken; that's a separate rung. csv-parser uses `Buffer.from(string)` which works correctly (gets length=1 + byte at slot 0), so this iterator-protocol rung is sufficient for the blocker at hand. The Buffer.from-array gap is named here for the next rung.

## Compounding

Any consumer that destructures, spreads, `for-of`'s, or `Array.from`'s a Buffer benefits. Pattern is broadly used in stream parsers, encoders, and CRC implementations across the npm ecosystem.
