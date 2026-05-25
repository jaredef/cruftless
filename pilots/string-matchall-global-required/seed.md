# string-matchall-global-required — Seed

## Telos

Per ECMA-262 §22.1.3.13 step 4, `String.prototype.matchAll` throws TypeError when the first argument is a RegExp without the global (`/g`) flag. Currently cruft accepts non-global RegExps silently.

Identified by RES audit-2 (Gap E).

## Apparatus

- `pilots/rusty-js-runtime/derived/src/prototype.rs::String.prototype.matchAll` (line 503).

## Methodology

After detecting `Value::Object(id)` with `InternalKind::RegExp`, check the flags string for `g`. If missing, throw TypeError per spec.

## Carve-outs

- Non-regex first-arg coercion: spec §22.1.3.13 step 1 wraps with `new RegExp(arg, 'g')`. v1 just delegates to the existing path (the result still has `g`). Defer wrapping if test262 specifically requires it.

## Resume protocol

Read `trajectory.md` tail.
