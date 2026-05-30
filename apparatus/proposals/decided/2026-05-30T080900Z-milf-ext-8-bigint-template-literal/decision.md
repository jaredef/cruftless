---
proposal_slug: 2026-05-30T080900Z-milf-ext-8-bigint-template-literal
decision: APPROVED
arbiter_session: dyad-substrate-resolver-plus-helmsman-per-keeper-telegram-10528
decided_at: 2026-05-30T08:09:00Z
covers_commits:
  - f66507bb74a3127ffe1054bd25a52ff0d54a891c
---

## Findings

Approved per keeper directive Telegram 10528 ("A") authorizing the named template-literal BigInt residual from the prior rung's mongoose smoke triage.

Substrate commit `f66507bb` gates the BigInt-mix rejection in `op_add_rt` on `!either_string`. ECMA-262 §13.15.3 step 8 specifies that when either operand of `+` is a String, the result is a string concatenation with `ToString` applied to both — the BigInt heterogeneous-mix throw is specific to the numeric `+` path. The bytecode compiler lowers template literals to a left-to-right `Op::Add` chain seeded by the first quasi (a String constant); the LHS-is-String invariant the comment relies on (`compile_template_literal` at `compiler.rs:4833`) was correct, but `op_add_rt` checked the BigInt mix BEFORE the String-concat fast path and threw.

`abstract_ops::to_string` already calls `JsBigInt::to_decimal`, so once the gate moved no other surface needed updating.

## Verification

1. `cargo build --release --bin cruft -p cruftless` — PASS.
2. `cargo test --release -p rusty-js-runtime --lib` — 74 passed, 1 ignored.
3. `cargo test --release -p cruftless --lib` — 11 passed.
4. `cargo test --release -p rusty-js-bytecode` — 61 passed (no test moved).
5. Smoke `/tmp/smoke/bigint_tpl.mjs`:
   - `` `range: ${MIN} to ${MAX}` `` → `"range: -9223372036854775808 to 9223372036854775807"`.
   - `"prefix" + 42n` → `"prefix42"`.
   - `42n + "suffix"` → `"42suffix"`.
   - `1n + 1` still throws `Cannot mix BigInt and other types` (numeric path unchanged — spec-correct).
6. Mongoose smoke `legacy/host-rquickjs/tests/fixtures/consumer-mongoose-app/main.mjs` fully PASSES shape check: `{"hasSchema":true,"hasModel":true,"hasConnect":true,"hasMongo":true,"schemaHasPath":true}`. The three-rung chain (MILF-EXT 7 readers → 7.1 install gaps → 8 BigInt template) closes the full mongoose import path.

## Compounding scope

The fix lives in the universal `+` operator path. Any package whose load chain interpolates a BigInt benefits implicitly. A top500 re-sweep is the natural follow-up to quantify the package-PASS gain across the BigInt-touching cluster.
