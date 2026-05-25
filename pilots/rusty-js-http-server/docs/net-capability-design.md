# rusty-js-http-server HS-EXT 4 — Net Capability Design

## Purpose

This note decides the HS-EXT 4 question from the seed: HTTP server can extend the existing capability dispatcher directly. A separate nested locale is not needed for the first `Net` cut because the required shape is small and follows the existing `Fs`/`Env`/`Clock` pattern in `pilots/rusty-js-runtime/derived/src/caps.rs`.

The network capability should land as a general runtime capability named `Net`, but the first routed operation is only HTTP server listen. Client outbound sockets, UDP, DNS, TLS, WebSocket, and raw `node:net` APIs are deferred.

## Capability Shape

First-cut Rust shape:

```rust
#[derive(Debug, Clone)]
pub struct Net {
    pub listen: NetListenPolicy,
    pub connect: NetConnectPolicy,
}

#[derive(Debug, Clone)]
pub enum NetListenPolicy {
    None,
    Any,
    LoopbackAnyPort,
    Exact(Vec<NetEndpoint>),
}

#[derive(Debug, Clone)]
pub enum NetConnectPolicy {
    None,
    Any,
    Hosts(Vec<String>),
    Exact(Vec<NetEndpoint>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NetEndpoint {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Clone)]
pub enum NetOp {
    Listen { host: String, port: u16 },
    Connect { host: String, port: u16 },
}
```

`connect` exists in the type so future HTTP client and socket work has a slot, but `require_net` only needs to be called for `Listen` in the HTTP server round.

## Policy Semantics

`NetListenPolicy::LoopbackAnyPort` is the important developer-friendly default for local servers. It allows `127.0.0.1`, `localhost`, and `::1` on any port, including port `0`. It denies `0.0.0.0`, public interface binds, and non-loopback names.

Recommended constructors:

```rust
impl Net {
    pub fn full() -> Self;
    pub fn none() -> Self;
    pub fn loopback_server() -> Self;
    pub fn listen_exact(host: impl Into<String>, port: u16) -> Self;
}
```

`AmbientCaps::full()` should include `net: Net::full()`. Mode 0 and Mode 1 preserve Node-compatible behavior by allowing through the dispatcher. Mode 3 with `Net::none()` denies all listener binds.

## Dispatcher Route

Add `net: Net` to `AmbientCaps`, then add:

```rust
pub fn require_net(
    &self,
    cap: &Net,
    op: NetOp,
    caller: &ModuleId,
) -> Result<(), CapabilityError>
```

The mode match mirrors existing capability methods:

- `Compat`: allow.
- `Audit`: record `net` and allow.
- `SealedDeps` with application/builtin provenance: allow.
- `SealedDeps` for dependencies: require the passed `Net`.
- `Sealed`: require the passed `Net`.

Audit operation strings should be stable and parseable enough for future manifest suggestions:

- `listen(127.0.0.1:0)`
- `connect(example.com:443)`

Denial hint for listen:

```text
add to caps in package.json: { net: { listen: ['127.0.0.1:3000'] } }
```

For loopback development servers, a second hint form can be:

```text
add to caps in package.json: { net: { listen: ['loopback:*'] } }
```

## HTTP Server Integration

`server.listen(...)` calls `require_net` before `sockets::listener_bind_async`.

The first direct Mode-0 HTTP probe may pass `Net::none()` to the dispatcher because Mode 0 ignores the explicit cap. That matches current `fs` route-through practice. When the capability-backed facade lands, the internal bind path should receive the facade's narrowed `Net` instead.

Internal bind function shape:

```rust
fn listen_with_authority(
    rt: &mut Runtime,
    server_id: u64,
    host: String,
    port: u16,
    callback: Option<Value>,
    net_cap: &caps::Net,
    caller: caps::ModuleId,
) -> Result<Value, RuntimeError>
```

Ambient `node:http` calls this with `Net::none()` initially. Capability-backed facades call it with their endowed `Net`.

## Compartment Facade

The facade is a narrowed namespace object, not a global escape hatch:

```js
const http = makeHttpFacade({ net: net.loopbackServer() });
const c = new Compartment({ globals: { http } });
```

The facade's `createServer` can share the same server object implementation, but the returned server records `authority = NetCapability` and stores the narrowed `Net` policy or an opaque handle id to it. `listen` then uses that stored policy.

If a compartment has no endowed `http`, `http` is simply absent. If its module map contains no `node:http`, `import('node:http')` rejects. That keeps closed import graphs and capability endowments aligned.

## Why No Nested Locale

The `Net` first cut is a localized dispatcher extension:

- one new capability struct,
- two policy enums,
- one operation enum,
- one `AmbientCaps` field,
- one `require_net` method,
- unit tests matching the existing caps tests,
- HTTP `listen` route-through.

That is small enough to land inside HS as a prerequisite substrate move. A nested locale becomes appropriate only when network scope expands to outbound client APIs, DNS policy, raw sockets, TLS, WebSockets, or manifest/lockfile schema.

## Required Probes

Minimum HS probes after `require_net` exists:

1. Mode 0 direct HTTP listen succeeds.
2. Mode 1 direct HTTP listen succeeds and audit contains `net\tlisten(127.0.0.1:0)` or equivalent normalized form.
3. Mode 3 ambient `node:http.createServer(...).listen(...)` denies with capability `net`.
4. Mode 3 capability-backed facade with `loopback:*` allows `127.0.0.1:0`.
5. Mode 3 capability-backed facade denies `0.0.0.0:0` when endowed only with loopback.

## HS-EXT 4 Disposition

Proceed without spawning a nested locale. Add `Net` to the existing caps dispatcher before the code-bearing HTTP server round, then keep HTTP's `listen` implementation behind the same internal bind path whether it is reached from ambient `node:http` or a future capability-backed facade.
