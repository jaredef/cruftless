---
proposal_slug: 2026-05-30T131000Z-milf-ext-8-1-allocunsafe-inline-override
decision: APPROVED
arbiter_session: dyad-substrate-resolver-plus-helmsman-per-keeper-telegram-10532
decided_at: 2026-05-30T13:10:00Z
covers_commits:
  - 41661aa1820a2a088463c6e899244d1c9cfd859b
---

## Findings

Approved per keeper directive Telegram 10532 ("While that is running let's continue") authorizing candidate B from the prior next-rung menu (audit other buffer-creation sites for missing install_buffer_methods).

The audit found that every `__is_buffer__` creation site already had `install_buffer_methods` in scope after MILF-EXT 7.1. But one site (`Buffer.allocUnsafe`) had a worse pattern: inline `readUInt8` + `subarray` registrations bracketing the `install_buffer_methods` call, with the inline subarray running AFTER install and overriding the correct version. The inline subarray did not set `__is_buffer__` on its output or call `install_buffer_methods` recursively, so `nanoid.allocUnsafe(N).subarray(0, n)` returned a non-Buffer-shaped object.

Substrate commit `41661aa1` removes both inline registrations. `install_buffer_methods` is now the single source of truth for `Buffer.allocUnsafe` outputs — matching the alloc / allocUnsafeSlow / `Buffer.from` / `Buffer.concat` paths after MILF-EXT 7.1.

## Verification

1. `cargo build --release --bin cruft -p cruftless` — PASS.
2. `cargo test --release -p rusty-js-runtime --lib` — 74 passed, 1 ignored.
3. `cargo test --release -p cruftless --lib` — 11 passed.
4. Smoke `/tmp/smoke/subarr.mjs`:
   - `Buffer.allocUnsafe(8).subarray(2, 6).length` → 4.
   - Byte-indexed access through subarray (`s[0]` → 2, `s[3]` → 5).
   - **`Buffer.isBuffer(s)` → true** (was false before this rung).
   - `s.readUInt16BE(0)` → 515 (= 0x0203 BE).

## Compounding

Third buffer-shape closure rung in 24h after MILF-EXT 7 (numeric surface) + 7.1 (install gaps). Cumulative state: all five Buffer-construction paths (alloc, allocUnsafe, allocUnsafeSlow, Buffer.from, Buffer.concat) and the subarray path produce uniformly-shaped Buffers with the full prototype surface. nanoid is the named consumer; any package doing `allocUnsafe(...).subarray(...).readXxx(...)` benefits.
