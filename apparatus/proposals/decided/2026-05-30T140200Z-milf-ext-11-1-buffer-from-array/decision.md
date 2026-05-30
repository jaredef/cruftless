---
proposal_slug: 2026-05-30T140200Z-milf-ext-11-1-buffer-from-array
decision: APPROVED
arbiter_session: dyad-substrate-resolver-plus-helmsman-per-keeper-telegram-10542
decided_at: 2026-05-30T14:02:00Z
covers_commits:
  - 2e03c3e2cad5d8762a7125675b50ec3fe8e6aa00
---

## Findings

Approved per keeper directive Telegram 10542 ("Continue"). Closes the MILF-EXT 11 named follow-up: `Buffer.from(array)` only handled the String input shape; all object-shaped inputs (Array, Uint8Array, Buffer) fell through to an empty Vec, producing length=0 buffers.

Substrate commit `2e03c3e2` adds an Object branch to `Buffer.from` that reads `length` + indexed slots from the source. The uniform property-bag storage means one path covers Array, Uint8Array, Buffer, and any indexed view — no per-shape dispatch needed.

## Verification

1. `cargo build --release --bin cruft -p cruftless` — PASS.
2. `cargo test --release -p rusty-js-runtime --lib` — 74 passed.
3. `cargo test --release -p cruftless --lib` — 11 passed.
4. Smoke:
   - `Buffer.from([10,20,30]).length` → `3`; `[...Buffer.from([10,20,30])]` → `[10,20,30]`.
   - `Buffer.from(new Uint8Array([7,8,9]))` → `[7,8,9]`.
   - `Buffer.from(Buffer.from([1,2,3]))` (round-trip) → `[1,2,3]`.
   - `Buffer.from('abc')` (existing string path) unchanged → `[97,98,99]`.
5. Wider iteration smoke from MILF-EXT 11 now passes for array-input buffers: `for-of`, `Array.from`, spread, and destructure-with-rest all yield the correct bytes.

## Compounding

Buffer.from-array is a workhorse: stream parsers, binary protocols, crypto, image processing all use `Buffer.from([0x12, 0x34, ...])` constants. Combined with MILF-EXT 11's iterator, the full create→iterate path on array-input buffers is now correct.
