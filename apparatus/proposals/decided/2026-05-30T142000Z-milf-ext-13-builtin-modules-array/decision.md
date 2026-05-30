---
proposal_slug: 2026-05-30T142000Z-milf-ext-13-builtin-modules-array
decision: APPROVED
arbiter_session: dyad-substrate-resolver-plus-helmsman-per-keeper-telegram-10546
decided_at: 2026-05-30T14:20:00Z
covers_commits:
  - a4c6094505feab482bb2ec28df695eefd7fa7ca6
---

## Findings

Approved per keeper directive Telegram 10546 ("Run a sweep and the next pick"). Sweep launched in parallel (PID 282681). Picked rollup-plugin-node-resolve's blocker which traced into the `builtin-modules` polyfill — a workhorse package used by every rollup plugin, ESLint config, and bundler internal that classifies module specifiers.

`require('module').builtinModules` was registered as an empty Object; consumers do `(builtinModules || ...).filter(...)` and dead-end with "filter is undefined" (Object lacks Array.prototype.filter, and the misleading error obscures the type-mismatch).

Substrate commit `a4c60945` replaces the empty-Object + duplicate-getter pair with an Array of 29 standard Node builtin-module names. Conservative listing: a module appears only if `require('<name>')` is expected to succeed in cruft.

## Verification

1. `cargo build --release --bin cruft -p cruftless` — PASS.
2. `cargo test --release -p rusty-js-runtime --lib` — 74 passed.
3. `cargo test --release -p cruftless --lib` — 11 passed.
4. `Array.isArray(require('module').builtinModules) === true`; length 29; `includes('fs') === true`.
5. rollup-plugin-node-resolve advanced past the filter failure into a fresh-shape downstream blocker (CJS-wrapper parse-error on `import` keyword) — named follow-up at the parser tier, distinct coordinate.

## Compounding scope

`builtin-modules` is one of the most-required npm utility packages in the bundler ecosystem. Anything that classifies specifiers (rollup plugins, esbuild plugins, ESLint resolvers, bundler externals filters) benefits. The in-flight sweep (PID 282681) was started on the binary BEFORE this rung; a follow-up sweep would capture the cumulative cross-package gain.
