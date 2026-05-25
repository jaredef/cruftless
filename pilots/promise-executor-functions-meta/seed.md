# promise-executor-functions-meta — Seed

## Telos

ECMA-262 §27.2.1.3.1 (Promise Resolve Functions) and §27.2.1.3.2 (Promise Reject Functions) mandate that the resolve/reject functions passed to a Promise constructor's executor have:
- `name`: `""` (anonymous)
- `length`: `1`

cruft was creating them with `name="<promise-resolve>"` / `name="<promise-reject>"` and `length=0`, observable through `fn.name` / `fn.length`.

## Apparatus

- `pilots/rusty-js-runtime/derived/src/promise.rs` — Promise constructor's resolve_fn / reject_fn allocation (uses `make_native`).
- `pilots/rusty-js-runtime/derived/src/intrinsics.rs` — `make_native_with_length(name, length, f)` already exists at line 5957.

## Methodology

1. Probe: `new Promise(function(r,j){console.log(r.name, r.length, j.name, j.length)})` — observed `<promise-resolve> 0 <promise-reject> 0`, must be `"" 1 "" 1`.
2. Replace `make_native("<promise-resolve>", ...)` / `make_native("<promise-reject>", ...)` with `make_native_with_length("", 1, ...)`.

## Carve-outs

- Built-in Promise.length, Promise property-descriptor enumerability/configurability defaults: separate substrate (built-in function property-descriptor compliance — needs OrdinaryFunction infrastructure for built-ins).
- GetCapabilitiesExecutor function (for `Promise.resolve.call(NotPromise)`): separate substrate (subclassing path that allocates the 2-arg internal executor).

## Resume protocol

Read `trajectory.md` tail.
