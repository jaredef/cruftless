---
proposal_slug: 2026-05-30T053500Z-bytecode-golden-init-vs-store-refresh
decision: APPROVED
arbiter_session: dyad-substrate-resolver-plus-helmsman-per-keeper-telegram-10526
decided_at: 2026-05-30T05:35:00Z
covers_commits:
  - 5a22837e2fc097b115bc1b887a9d2f548be0e2eb
---

## Findings

Approved per keeper Telegram 10526 ("1 then 2 then 3"), #2 of the three-rung sequence. Surfaced by the prior NEW R2 codex instance (`codex-pop-os-20260530t040439`) as the orthogonal apparatus blocker preventing push-level quiescence on their kState rung.

Substrate commit `5a22837e` refreshes four `pilots/rusty-js-bytecode/derived/tests/compiler_golden.rs` tests that were asserting `StoreLocal` for declaration-tied installation. The compiler emits the right opcodes; the tests carried the old expectation from before the InitLocal/StoreLocal distinction became canonical (declaration installation → InitLocal; subsequent assignment to an already-installed binding → StoreLocal).

Concrete refreshes:

- `variable_declaration_stores_local` renamed `variable_declaration_inits_local`; asserts `InitLocal` for `let x = 1;`.
- `variable_without_initializer`: asserts `InitLocal` for `let x;`.
- `multiple_declarators`: asserts `>=2 InitLocal` for `const a = 1, b = 2;` (one per declarator, without pinning the exact reservation-vs-init opcode count).
- `assignment_to_local`: asserts BOTH `InitLocal` (the let) AND `StoreLocal` (the reassignment) appear in `let x = 0; x = 1;`. Captures the full distinction so future codegen drift in either direction trips a specific named test.

## Verification

1. `cargo test --release -p rusty-js-bytecode` — 61 passed, 0 failed (was 57 passed, 4 failed pre-refresh).
2. `cargo test --release -p rusty-js-runtime --lib` — 74 passed, 1 ignored, 0 failed (no runtime regression).
3. No compiler-tier change — codegen was already correct; only the test expectations moved.

## Diagnostic credit

NEW R2 reported the failure cluster in message `0c878407`. Without that diagnostic the gate could have stayed silently red indefinitely. Their compiler.rs diff was correctly discarded (per redirect `4a846fea`) because the kState fix had already landed as `3e9ba56d`; the residual value was exactly this gate refresh.
