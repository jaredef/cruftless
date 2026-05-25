# rusty-js-http-server HS-EXT 3 — API Wireup Design

## Purpose

This design turns the HS-EXT 2 telos into an implementation-shaped plan. The HTTP server locale should wire `node:http.createServer(handler)` into the existing runtime without inventing a parallel HTTP stack. The load-bearing split is:

- `http.createServer(handler)` creates shape and stores callback identity.
- `server.listen(...)` binds a socket and is therefore the authority-bearing operation.
- Poll-driven request dispatch re-enters the handler's creating realm before invoking JS.

Mode 0 remains Node-compatible. Audit and sealed modes route the listener bind through a future `Net` capability check. Compartments receive no ambient HTTP by default; they can be endowed with an HTTP facade closed over a `Net` capability.

## Existing Hooks To Use

- Live namespace: `cruftless/src/http.rs`
- Event-loop hook: `HostHook::PollIo` in `pilots/rusty-js-runtime/derived/src/module.rs`
- Job queue: `Runtime::enqueue_macrotask`
- Handler invocation: `Runtime::call_function`
- Realm transition: `Runtime::enter_realm` / `Runtime::exit_realm`
- Compartment identity: instance slot `__compartment_realm`
- Capability dispatcher: `Runtime.caps`
- TCP substrate: `pilots/sockets/derived/src/lib.rs`
- HTTP codec: `pilots/http-codec/derived/src/lib.rs`
- Node data-layer reference: `pilots/node-http/derived/src/lib.rs`

## Runtime Storage

The first implementation should introduce a host-owned active-server registry. The registry can live in `cruftless/src/http.rs` for the first cut, but the design should keep it small enough to move into the runtime later if multiple host modules need shared PollIo composition.

Suggested records:

```rust
struct ActiveHttpServer {
    server_object: ObjectRef,
    listener_handle: u64,
    bound_addr: String,
    handler: Value,
    handler_realm: usize,
    authority: HttpAuthoritySource,
    state: HttpServerState,
}

enum HttpAuthoritySource {
    Ambient,
    Audit,
    NetCapability,
}

enum HttpServerState {
    Created,
    Listening,
    Closing,
    Closed,
}
```

The JS server object should store a private-ish numeric slot such as `__cruftless_http_server_id`. Existing runtime objects already tolerate internal-looking string slots, and this matches the current Compartment slot style. The slot points into the registry; the registry holds the actual listener and handler.

The active record must retain the handler `Value` so it survives across turns. It must also retain `handler_realm = rt.current_realm` at `createServer` time. That realm is the callback's dispatch home even when a request arrives later through `PollIo`.

## JS Object Surface

`http.createServer([options], handler)` first cut may accept only a function or `(options, function)` and ignore options. It returns a server object with:

- `listen(port[, host][, callback])`
- `close([callback])`
- `address()`
- minimal EventEmitter-compatible `on` / `once` only if direct or Express probes require them

Request object minimum:

- `method`
- `url`
- `headers`
- `httpVersion`
- `socket` can be absent initially unless a probe requires it

Response object minimum:

- `statusCode`
- `statusMessage`
- `headersSent`
- `setHeader(name, value)`
- `getHeader(name)`
- `removeHeader(name)`
- `writeHead(statusCode[, statusMessage][, headers])`
- `write(chunk)`
- `end([chunk])`

The method semantics should follow `pilots/node-http/derived/` rather than re-deriving Node behavior inside `http.rs`.

## Authority Flow

`createServer` performs no network bind. It should validate the handler shape and allocate a server object.

`listen` is the effectful point:

1. Normalize `(port, host, callback)` into a host/port tuple. Default host should be `127.0.0.1` for the first probe unless Node compatibility requires `0.0.0.0` later.
2. Build a caller identity. The existing pattern uses `caps::ModuleId::builtin("node:http")` for builtin host calls when no module provenance is available. A later refinement can thread the actual user module URL.
3. Route through `rt.caps.require_net(&net_cap, NetOp::Listen { host, port }, &caller)` once the `Net` class exists.
4. In Mode 0, `require_net` allows ambient.
5. In Mode 1, it allows and records an audit operation like `listen(127.0.0.1:0)`.
6. In Mode 2, application modules may use ambient authority; dependencies require an explicit capability.
7. In Mode 3, every listener bind requires an explicit `Net` capability.
8. Only after the capability check succeeds, call `sockets::listener_bind_async`.

Until `Net` is implemented, the first code round may keep the check as an explicit local seam:

```rust
// TODO(HS-EXT 4): replace with rt.caps.require_net(...).
let authority = match rt.caps.mode {
    CapMode::Compat => HttpAuthoritySource::Ambient,
    CapMode::Audit => HttpAuthoritySource::Audit,
    _ => return Err(RuntimeError::TypeError("CapabilityError: net.listen denied".into())),
};
```

That interim form is acceptable only if HS-EXT 4 immediately turns it into the real dispatcher shape. The better implementation is to add `Net`, `NetPolicy`, `NetOp`, `AmbientCaps.net`, and `CapDispatcher::require_net` before binding sockets.

## Compartment Composition

Compartments must not discover ambient `node:http` unless endowed with it. The existing `Compartment` constructor already copies `globals` into a new realm and marks it ambient-denied through `allocate_compartment_realm`.

The HTTP facade shape should be:

```js
const httpForCompartment = makeHttpFacade({ net });
new Compartment({ globals: { http: httpForCompartment } });
```

`makeHttpFacade` can be internal at first. Its only requirement is that `listen` closes over a `Net` capability and passes that capability to the same internal bind path used by ambient `node:http`. The facade should not give the compartment a way to import ambient `node:http` through the builtin resolver.

When code inside a compartment calls `http.createServer(handler)`, the server record stores the compartment realm id. Later request dispatch must:

1. read `handler_realm` from the server record,
2. call `let prior = rt.enter_realm(handler_realm)`,
3. invoke the handler with `rt.call_function(handler, Value::Object(server_object), vec![req, res])`,
4. call `rt.exit_realm(prior)` on both success and error paths.

This preserves Doc 736's property at request time, not only at load time.

## PollIo And Request Dispatch

`listener_bind_async` starts an accept loop and returns listener events through `listener_poll`. The host PollIo path should poll active HTTP listeners with a small timeout, translate each ready connection into one macrotask, and return `true` when it enqueues any work.

First-cut loop:

1. Poll each `ActiveHttpServer` in `Listening` state.
2. On `AsyncEvent::Connection { stream_id, peer }`, enqueue a macrotask containing `server_id` and `stream_id`.
3. The macrotask reads a bounded full request from the stream.
4. Parse via `http_codec::parse_request`.
5. Allocate JS request/response objects.
6. Re-enter `handler_realm` and call the handler.
7. Serialize the response.
8. `stream_write_all` to the stream.
9. `handle_close(stream_id)`.

The macrotask should not hold a registry borrow while invoking JS. Pull the data needed for dispatch into local clones first, then release the registry lock/borrow. This matters because the handler may call `server.close()` or other HTTP methods that need the same registry.

`PollIo` liveness should return `true` while an active server is listening only when work was enqueued or a near-term timer/other host hook needs another pass. It must not spin forever merely because a server exists. The existing async listener has a background accept loop; `listener_poll(id, max_wait_ms)` can block briefly at idle without busy looping.

## Bounded Reads

The first cut should be whole-message only:

- Read until `\r\n\r\n`.
- Parse `Content-Length` if present.
- Continue reading until that many body bytes are present.
- Enforce a small request cap, for example 1 MiB, until streaming exists.
- Return `400 Bad Request` on parse failure.
- Return `413 Payload Too Large` on cap violation.

No keep-alive in the first cut. Always emit `Connection: close` and close the stream after write.

## Response Serialization

Response state can be backed by a Rust struct and exposed through JS methods, or it can be reconstructed from object slots after handler return. The Rust-backed struct is preferable because it matches the `node-http` pilot and makes `headersSent`, body accumulation, and `writeHead` behavior explicit.

Minimum serialization rules:

- Default status: `200 OK`.
- Default body: empty.
- If no `Content-Length` was set, compute it from accumulated body bytes.
- If no `Connection` was set, write `Connection: close`.
- Header names should be lowercased internally, matching `NodeHeaders`.
- `end(chunk)` appends `chunk` then marks the response finished.

If the handler returns without calling `end`, the first cut may serialize the current response state. A follow-on should model Node's open response lifecycle more closely.

## Install And Builtin Resolver

`install(rt)` in `cruftless/src/http.rs` should keep the existing module-load shape:

- `STATUS_CODES`
- `METHODS`
- `Agent`
- class constructors with `.prototype`
- `default === http`

Only `createServer` changes behavior first. `request` and `get` remain client-side stubs until the HTTP client locale elects to route them.

The builtin resolver already resolves `node:http` to the installed namespace. Compartment sealed import graphs should not add `node:http` to compartment modules unless explicitly endowed or mapped through a capability facade.

## Implementation Order

1. Add `docs/api-wireup-design.md` and trajectory entry.
2. Add `Net` capability types and `require_net` if the code round begins in sealed-aware mode.
3. Replace `createServer` stub with a JS server object and registry entry.
4. Add `listen`, `address`, and `close`.
5. Install or extend the shared PollIo hook so HTTP polling composes with fs/timers/N-API instead of replacing them.
6. Add request/response object builders.
7. Add direct Node Mode-0 probe.
8. Add audit/sealed probes.
9. Add Compartment no-ambient/endowed-facade/realm-preservation probes.
10. Attempt Express minimal only after direct and authority probes pass.

## Open Questions

1. **Shared PollIo composition**: `install_host_hook(HostHook::PollIo(...))` replaces the previous hook. HTTP must extend the existing host PollIo path rather than install a second independent hook. The cleanest first cut may be adding `http::poll_io(rt)` and calling it from `cruftless/src/fs.rs`'s already-shared hook.
2. **Caller provenance**: current cap gate patterns often use builtin caller identities. HS needs a later route to the actual module URL so Mode 2 can distinguish application code from dependencies for `listen`.
3. **Response lifetime**: direct probes can serialize after handler return. Express may expect a longer-lived `ServerResponse` object. That is a likely HS-EXT 8 expansion.
4. **Facade construction API**: the first public JS API for capability-backed HTTP facade should probably land with the broader capability-handle API, not as an HTTP-only global.

## HS-EXT 3 Disposition

The next code-bearing round should avoid parser, TCP, or HTTP data-model invention. The work is registry + `createServer` object shape + `listen` bind + PollIo dispatch + realm/capability route-through.
