# rusty-js-http-server HS-EXT 1 — Transport Survey

## Purpose

This survey grounds the HTTP server locale in the current repository state before any implementation work. The locale's first implementation target is `node:http.createServer(handler)` over plain HTTP/1.1. The key finding is that the HTTP server is not greenfield substrate: the repo already contains closed or usable pilots for the data layer, HTTP wire codec, and TCP transport. HS work should compose those substrates into the live runtime surface.

## Current Live Surface

The user-facing runtime still exposes `node:http` as a compatibility stub in `cruftless/src/http.rs`.

Current behavior:

- `http.request`, `http.get`, and `http.createServer` throw documented "not yet implemented" TypeErrors.
- `http.STATUS_CODES` and `http.METHODS` are present for module-load compatibility.
- `http.ServerResponse`, `http.IncomingMessage`, `http.Server`, and `http.ClientRequest` exist as class-shape stubs with `.prototype`.

Implication: HS implementation should replace the `createServer` stub first while preserving the import-time shape that existing package probes rely on.

## Existing Substrates

### Node HTTP Data Layer

Path: `pilots/node-http/derived/`

Status: closed data-layer pilot. It models the Node-style shapes:

- `IncomingMessage`: `method`, `url`, `headers`, `status_code`, `status_message`, `http_version`, `body`, `complete`.
- `ServerResponse`: `write_head`, `set_header`, `get_header`, `remove_header`, `write`, `end`, `headers_sent`, `status_code`, `status_message`, accumulated body.
- `ClientRequest`: request-side writer shape.
- `Server`: handler storage, `listen`, `close`, and pilot-only `dispatch`.
- `NodeHeaders`: lowercased, case-insensitive flat-object header behavior.

Scope note from the pilot: no transport, no socket binding, no actual wire format. This is exactly the shape HS needs to lift into live JS objects.

### HTTP/1.1 Codec

Path: `pilots/http-codec/derived/`

Status: usable whole-message HTTP/1.1 codec.

Available primitives:

- `parse_request(bytes) -> ParsedRequest`
- `parse_response(bytes) -> ParsedResponse`
- `serialize_request(method, target, headers, body) -> Vec<u8>`
- `serialize_response(status, reason, headers, body) -> Vec<u8>`
- `chunked_encode`, `chunked_decode`

Scope note: whole-message parser only. No streaming parser, trailers, compression, multipart, HTTP/2, HTTP/3, or WebSocket upgrade.

HS first cut should use whole-message parsing with fully buffered request bodies. Streaming request bodies are a later locale round.

### TCP Socket Substrate

Path: `pilots/sockets/derived/`

Status: TCP primitives exist.

Available listener/stream primitives:

- `listener_bind(addr) -> (handle_id, actual_addr)`
- `listener_accept(handle_id) -> (stream_id, peer_addr)`
- `listener_bind_async(addr) -> (handle_id, actual_addr)`
- `listener_poll(handle_id, max_wait_ms) -> Option<AsyncEvent>`
- `listener_stop_async(handle_id)`
- `stream_read`, `stream_try_read`
- `stream_write`, `stream_write_all`
- `handle_close`

The async-listener shape is the better fit for live runtime integration because accepted connections can be polled from the engine's foreground loop instead of calling JS from an OS thread.

### Runtime Event-Loop Hook

Path: `pilots/rusty-js-runtime/derived/src/module.rs`

Relevant hook: `HostHook::PollIo`.

The hook is called at `run_to_completion` idle time after microtasks and macrotasks drain. Host code can poll OS I/O, enqueue macrotasks, and return whether work was enqueued. This is the intended integration point for HTTP server request dispatch.

Implication: server accept/read/write should be routed through the runtime job queue. JS request handlers should run on the runtime thread via `rt.call_function`, not from a background listener thread.

### Capability Runtime

Path: `pilots/rusty-js-caps/`

Status: Pilot alpha first cut closed for fs/process/env/stdio/clock. Network/server APIs are not yet routed through capability checks.

HTTP server first cut may remain Mode 0 compatible, matching the HS seed. However, the implementation should keep a clean seam for a future `Net` capability gate:

- `http.createServer` or `server.listen` is the natural enforcement site.
- Under sealed modes, binding a listener should require an explicit net/listen capability.
- The first cut should avoid burying socket binding behind code paths that cannot later consult `rt.caps`.

### TLS / HTTPS

Path: `pilots/tls/`

Status: client-side TLS substrate exists and has propagated into PM HTTP client work. Server-side TLS is out of first-cut scope.

Implication: HS should target plain `http.createServer` first. `https.createServer` and server-side TLS handshake belong to a later workstream or later HS extension.

## Recommended First-Cut Architecture

1. `http.createServer(handler)` allocates a JS server object that stores the handler and server state in internal slots.
2. `server.listen(port[, host][, callback])` binds a TCP listener using the socket substrate. Port `0` should return the actual bound port.
3. The host/runtime installs or extends a `PollIo` hook that polls active HTTP listeners.
4. On accepted connection, the runtime reads a whole HTTP request into bytes. First cut can use a bounded read loop and `Content-Length`.
5. Parse bytes via `http-codec::parse_request`.
6. Build JS `IncomingMessage`-shape and `ServerResponse`-shape objects.
7. Invoke the JS handler using `rt.call_function(handler, server_or_undefined, vec![req, res])` on the runtime thread.
8. After handler returns, serialize response state via `http-codec::serialize_response`.
9. Write the response to the stream and close the connection.

First-cut intentional limitations:

- One request per connection.
- `Connection: close`.
- Fully buffered request body.
- No keep-alive.
- No chunked request-body streaming.
- No WebSocket upgrade.
- No HTTPS.
- No background-thread JS invocation.

## JS Object Surface

Minimum server object:

- `listen(port[, host][, callback])`
- `close([callback])`
- `address()`
- EventEmitter-compatible `on`/`once` can be deferred or delegated to existing event stubs if needed by Express.

Minimum request object:

- `method`
- `url`
- `headers`
- `httpVersion`
- buffered body exposure can initially be absent or minimal, because simple Express `GET /` does not require a request body.

Minimum response object:

- `statusCode`
- `statusMessage`
- `setHeader(name, value)`
- `getHeader(name)`
- `writeHead(statusCode[, statusMessage][, headers])`
- `write(chunk)`
- `end([chunk])`
- `headersSent`

The `node-http` data-layer pilot should be the behavioral reference for these fields and methods.

## First Probe Targets

### Probe 1: Direct Node HTTP Shape

JS fixture:

```js
import http from "node:http";

const server = http.createServer((req, res) => {
  res.writeHead(200, { "content-type": "text/plain" });
  res.end("hello");
});

server.listen(0, "127.0.0.1", () => {
  console.log("listening");
});
```

Harness expectation: a local TCP client sends `GET / HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n`; response starts with `HTTP/1.1 200 OK` and body is `hello`.

### Probe 2: Express Minimal

The HS seed's Pred-hs.1 names minimal Express compatibility:

```js
app.get("/", (req, res) => res.send("hello"));
```

This should follow the direct Node probe because Express likely exercises additional surfaces: EventEmitter details, `ServerResponse.prototype` methods, and possibly `setHeader`/`getHeader` expectations.

### Probe 3: Wire Comparison

Compare the direct Node HTTP shape against Node or Bun for simple responses:

- status line
- `Content-Length`
- body bytes
- close behavior

Header ordering should be treated carefully. Byte-identical output may require normalizing dynamic headers or asserting a minimal wire subset rather than the whole response on the first cut.

## Risks

1. JS handler lifetime and storage: the server object needs to retain a JS function value safely across `run_to_completion` turns.
2. Event-loop liveness: `run_to_completion` currently exits when no jobs remain unless `PollIo` reports work. Listening servers need a clean "I am still live" signal without spinning.
3. Blocking reads: whole-message request reads must not block the runtime indefinitely. First cut should use nonblocking reads or bounded timeouts.
4. Express surface expansion: Express may require more than bare `createServer`, especially EventEmitter and prototype method details.
5. Capability route-through: first cut is Mode 0, but binding must remain easy to gate through `Net` later.

## HS-EXT 1 Disposition

The next implementation round should not begin by writing a parser or independent HTTP data model. It should wire existing substrates:

- `node-http` for JS-visible data semantics.
- `http-codec` for wire parsing/serialization.
- `sockets` for TCP.
- `HostHook::PollIo` and macrotasks for foreground JS handler dispatch.

Recommended next artifact: `docs/api-wireup-design.md`, specifying the exact Rust storage shape for active servers, listener handles, queued connections, and JS object internal slots before editing `cruftless/src/http.rs`.
