# error-instance-property-descriptors — Seed

## Telos

ECMA-262 §20.5.7.1 ErrorConstructor + §20.5.6.1 InstallErrorCause + §20.5.3.2 NativeError.prototype:
- `message` on Error instance: created via `CreateNonEnumerableDataPropertyOrThrow` → `{w:t, e:f, c:t}`.
- `cause` on Error instance: same, when opts.cause is provided.
- `stack` (V8/Node extension): typically `{w:t, e:f, c:t}`.
- `name` lives on Error.prototype (already set non-enumerable); should NOT be installed per-instance.

cruft installs all four via `rt.object_set` (default `{w:t, e:t, c:t}`), making them enumerable. test262 `built-ins/Error/{message_property, cause_property, prop-desc}` fail with "descriptor should not be enumerable".

Identified by RES audit-2 follow-on sweep.

## Apparatus

- `pilots/rusty-js-runtime/derived/src/intrinsics.rs::install_error_globals` (~line 4810), specifically the per-instance constructor body (lines 4873-4892).

## Methodology

1. Replace `rt.object_set(id, "message", ...)` with `dict_mut().insert(PropertyKey::String("message"), PropertyDescriptor{w:t, e:f, c:t})`.
2. Same for `cause` and `stack`.
3. Delete the per-instance `name` install (already on Error.prototype via `set_own_internal`).

## Carve-outs

- AggregateError's `errors` property: spec §20.5.7.4 says non-enumerable too; include in this rung.
- `Error.captureStackTrace` and stack-string formatting: out of scope.
- Subclass `extends Error` receiver path: preserve the in-place mutate behavior at line 4854-4863.

## Resume protocol

Read `trajectory.md` tail.
