---
proposal_slug: 2026-05-30T134800Z-milf-ext-10-fs-url-path-coercion
decision: APPROVED
arbiter_session: dyad-substrate-resolver-plus-helmsman-per-keeper-telegram-10538
decided_at: 2026-05-30T13:48:00Z
covers_commits:
  - 72a66ebf0731ea400cc8899426128c335bc38bbf
---

## Findings

Approved per keeper directive Telegram 10538 ("Pick the next cluster; no sweep yet"). Picked stylelint's `readFileSync ... (in-call='url')` blocker — Node accepts `URL | string | Buffer` for the path argument across the whole fs surface, but cruft's `arg_string` called `to_string` on URL objects yielding `"[object Object]"`, which then ENOENTed.

Substrate commit `72a66ebf` adds `arg_path_or_url(rt, args, i)` in `cruftless/src/fs.rs`. The helper recognizes the URL shape via the `href` slot starting with `"file://"` and strips the prefix; falls back to `arg_string` for non-Object values and Objects lacking the href slot. Substituted at all 33 fs path-arg sites (readFileSync, writeFileSync, statSync, realpathSync, mkdirSync, lstatSync, accessSync, etc.). Five closures declared `|_rt, args|`; renamed to `|rt, args|` to hand the runtime handle into the helper.

`href.strip_prefix("file://")` is preferred over reading `pathname` because the latter unwraps `file://host/p` into `/p` on Windows-style drive layouts (consumers in scope are POSIX so this is moot, but it's the safer default).

## Verification

1. `cargo build --release --bin cruft -p cruftless` — PASS.
2. `cargo test --release -p rusty-js-runtime --lib` — 74 passed, 1 ignored.
3. `cargo test --release -p cruftless --lib` — 11 passed.
4. Smoke `/tmp/smoke/fs_url.mjs`: `readFileSync(new URL('./fs_url.mjs', import.meta.url), 'utf8')` returns 545 bytes (was ENOENT); pathname-string path still works at the same length.
5. **stylelint loads**: 2 keys returned (was `readFileSync ... (in-call='url')`).

## Compounding

Any ESM package using `readFileSync(new URL('./bundled', import.meta.url), 'utf8')` benefits — a very common pattern for packages that read bundled data files at module-init. URL-as-path is the same helper for the entire fs sync + async surface (all 33 sites), so the gain ripples implicitly to consumers of statSync, existsSync, readdirSync, mkdirSync, etc.

Cumulative cluster-A gain pending re-sweep.
