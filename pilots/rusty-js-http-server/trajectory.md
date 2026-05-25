# rusty-js-http-server — Trajectory

Per-HS-EXT log for the HTTP server pilot.

---

## HS-EXT 0 — 2026-05-23 (workstream founding)

Apparatus-tier round. Pilot founded per keeper directive 2026-05-23 18:12-local. Standalone top-level locale per Doc 737 §IV. Fourth of four spawns in this round.

### Trigger

- Keeper-named substrate gap: HTTP server APIs currently stubbed
- Blocks Node packages (express, fastify, koa, etc.) from running on cruft
- Multi-session substrate workstream; this round is apparatus-tier founding only

### Substrate delivered

- `seed.md` (~85 lines): telos (HTTP/1.1; single-connection first cut; Express compat target), 5 falsifiers Pred-hs.1-.5, methodology HS-EXT 0-9, carve-outs (HTTP/1.1 only; buffered body; no concurrent connections at first cut)
- `trajectory.md` (this file)
- `docs/` + `fixtures/` scaffolds

### Locale registration

Locale count: 19 → 20 after this spawn (12 → 13 top-level; 7 nested unchanged). Manifest refresh queued.

### Open scope at HS-EXT 0 close

1. HS-EXT 1 — survey existing TCP/socket substrate
2. HS-EXT 2 — HTTP/1.1 parser design
3. HS-EXT 3-9 per seed §III

---

*HS-EXT 0 closes. Pilot founded. Largest scope of the 4 spawns this round; multi-session substrate workstream when activated.*

---

## HS-EXT 1 — 2026-05-25 (transport survey)

Apparatus-tier survey round. No runtime code changed. The locale was informed by the current cross-locale substrate state before implementation.

### Trigger

Keeper directive: before beginning the next HTTP server step, inform it with a comprehensive reading of the current locales. The survey narrowed the HTTP server work from "build server substrate" to "compose existing closed substrates into the live runtime surface."

### Survey delivered

- `docs/transport-survey.md`

### Findings

1. **HTTP server is a composition locale, not greenfield.** The repo already has `pilots/node-http/derived/` for Node HTTP data-layer semantics, `pilots/http-codec/derived/` for whole-message HTTP/1.1 parse/serialize, and `pilots/sockets/derived/` for TCP listener/stream handles.
2. **The live `node:http` surface remains stub-only.** `cruftless/src/http.rs` provides import-time shape, constants, and class stubs, but `request`, `get`, and `createServer` still throw.
3. **Foreground runtime dispatch is the right first-cut shape.** The runtime already exposes `HostHook::PollIo`; accepted connections should enqueue macrotasks and invoke JS handlers via `rt.call_function` on the runtime thread. Background listener threads must not call into JS directly.
4. **Caps integration is a future seam, not the first cut.** `rusty-js-caps` first cut is closed for fs/process/env/stdio/clock, but network/listen is not yet gated. HS first cut can stay Mode 0 compatible while keeping `server.listen` as the future `Net` capability enforcement site.

### Next scope

1. **HS-EXT 2**: `docs/api-wireup-design.md` — exact active-server registry shape, JS object internal slots, listener handle ownership, PollIo/macrotask flow, and bounded-read/write behavior.
2. **HS-EXT 3**: minimal `http.createServer(...).listen(0)` direct Node fixture, before Express.
3. **HS-EXT 4+**: Express minimal probe, then cap-passing `Net` integration.

---

*HS-EXT 1 closes. The implementation target is now explicitly a runtime wireup over existing data-layer, codec, socket, and event-loop substrates.*

---

## HS-EXT 2 — 2026-05-25 (telos reformalization: HTTP as capability + compartment authority)

Apparatus-tier reformalization round. No runtime code changed. The seed was rewritten against the sharper telos surfaced by the keeper's Doc 736 question: HTTP server support is not merely a compatibility gap. It is the first runtime surface where `Net` capability authority, Compartment endowments, realm-preserving callback dispatch, socket transport, and Node-compatible `http.createServer` must compose.

### Trigger

Keeper asked how the HTTP server locale composes with Doc 736's capability architectural constraints and Compartments as first-class JS API support, then directed: "reformalize the locale's seed and trajectory against this telos."

### Seed reformalized

- Status updated to **REFORMALIZED at HS-EXT 2**.
- Telos changed from "basic HTTP/1.1 server compatible with Node" to:
  - Node-compatible HTTP serving in Mode 0,
  - `server.listen` as the `Net` authority boundary,
  - sealed/audit behavior through the cap dispatcher,
  - capability-backed `http` facade for Compartment endowment,
  - request handlers created in Compartments dispatching under their originating realm.
- Falsifiers expanded from 5 to 8:
  - direct Node probe,
  - Express minimal,
  - wire floor,
  - sealed `Net` denial,
  - audit event,
  - compartment-endowed authority,
  - realm preservation,
  - fuzz/diff-prod regression gates.
- Methodology rewritten so HS-EXT 3 is `docs/api-wireup-design.md` with authority model included before implementation.

### Findings

1. **`createServer` is shape; `listen` is authority.** This is the load-bearing split for Doc 736 composition.
2. **Compartments require HTTP facade parameterization.** A compartment must receive a capability-backed `http` object or no HTTP object at all. Ambient `node:http` must not leak into an ambient-denied compartment.
3. **Request callbacks carry realm identity.** An active server record must remember the creating realm/compartment; PollIo-dispatched request work must enter that realm before `rt.call_function(handler, ...)`.
4. **Mode 0 compatibility and Mode 3 impossibility are both first-class.** The locale must preserve Node-like ergonomics while making the strict-mode authority claim probeable.

### Next scope

1. **HS-EXT 3**: `docs/api-wireup-design.md`, including active-server registry fields, listener ownership, PollIo/macrotask flow, handler realm re-entry, and `Net` authority check placement.
2. **HS-EXT 4**: decide whether `Net` capability additions live in HS or a small nested caps/network sub-locale. If multi-rung shape appears, spawn nested locale before code.
3. **HS-EXT 5+**: implement direct Node Mode-0 probe first, then authority/Compartment probes before Express.

---

*HS-EXT 2 closes. The locale is now framed as HTTP-serving authority composition, not only HTTP-serving compatibility.*

---

## HS-EXT 3 — 2026-05-25 (API wireup design)

Apparatus-tier design round. No runtime code changed. This round translated the HS-EXT 2 telos into an implementation-shaped design for the live `node:http` surface.

### Trigger

Keeper directive: "continue as coherent" after the seed reformalization. Per the seed resume protocol, the next coherent move was `docs/api-wireup-design.md` with the authority model included before code.

### Design delivered

- `docs/api-wireup-design.md`

### Findings

1. **The first live edit should be registry and object shape, not parser or transport.** Existing `http-codec`, `sockets`, and `node-http` substrates already cover those layers well enough for the direct probe.
2. **HTTP PollIo must compose with the existing shared hook.** `install_host_hook(HostHook::PollIo(...))` replaces the previous hook, so HTTP should expose a `http::poll_io(rt)` helper and be called from the current shared host PollIo path rather than installing a competing hook.
3. **Server records need realm identity.** `createServer` must store `rt.current_realm` with the handler; request macrotasks must `enter_realm(handler_realm)` before `rt.call_function` and `exit_realm` on every path.
4. **`Net` should be added to the existing capability dispatcher pattern.** The local design names `Net`, `NetPolicy`, `NetOp::Listen`, `AmbientCaps.net`, and `CapDispatcher::require_net` as the route-through before socket bind.
5. **Compartment support is facade-based.** A compartment should receive a capability-backed `http` facade or no HTTP binding. The facade closes over a `Net` capability and delegates to the same internal bind path as ambient `node:http`.

### Next scope

1. **HS-EXT 4**: add the `Net` capability design/substrate decision. If the code delta is larger than a small dispatcher extension, spawn a nested caps/network locale before editing runtime code.
2. **HS-EXT 5**: implement the Mode-0 direct server path: `createServer`, `listen`, `address`, `close`, active-server registry, bounded whole-request read, response serialization, and PollIo integration.
3. **HS-EXT 6**: route listen through `require_net` and add audit/sealed probes.
4. **HS-EXT 7**: add Compartment probes for no ambient HTTP, endowed HTTP facade, and realm-preserving handler dispatch.

---

*HS-EXT 3 closes. The locale has a concrete implementation map: registry + createServer object shape + listen bind + shared PollIo + realm/capability route-through.*

---

## HS-EXT 4 — 2026-05-25 (Net capability design decision)

Apparatus-tier design round. No runtime code changed. This round answered whether HTTP server needs a nested caps/network locale before code.

### Trigger

HS-EXT 3 left the next scope as the `Net` capability decision. The API wireup design made `server.listen` the authority-bearing operation, so the capability substrate needed a concrete first-cut shape before HTTP code starts.

### Design delivered

- `docs/net-capability-design.md`

### Findings

1. **No nested locale is needed for the first cut.** `Net` is small enough to add directly to the existing `CapDispatcher` pattern: one capability struct, listen/connect policy enums, one op enum, one `AmbientCaps` field, and one `require_net` method.
2. **Listen is the only routed operation for HS.** `connect` is included in the type so future client/socket work has a slot, but HTTP server only calls `NetOp::Listen`.
3. **Loopback server policy is the ergonomic sealed-mode default.** `LoopbackAnyPort` allows `127.0.0.1`, `localhost`, and `::1`, including port `0`, while denying public interface binds such as `0.0.0.0`.
4. **Ambient and facade paths share one bind function.** Ambient `node:http` and future capability-backed HTTP facades should both call an internal `listen_with_authority(...)`; the only difference is which `Net` policy/handle they pass.
5. **Audit strings should be stable.** The suggested operation form is `listen(host:port)`, for example `listen(127.0.0.1:0)`, so audit output can become manifest hints later.

### Next scope

1. **HS-EXT 5**: code-bearing first round. Add `Net` to `caps.rs` with unit tests, then route HTTP `listen` through `require_net`.
2. Implement Mode-0 direct server path: `createServer`, `listen`, `address`, `close`, registry, bounded whole-request read, response serialization, and shared PollIo integration.
3. Add direct Node probe before Express.

---

*HS-EXT 4 closes. The authority substrate decision is made: extend the existing capability dispatcher directly, then begin HTTP server wireup.*

---

## HS-EXT 5 — 2026-05-25 (direct HTTP server wireup)

Code-bearing substrate round. This round moved the locale from apparatus into a live Mode-0 direct server path.

### Trigger

After HS-EXT 4 decided that `Net` could extend the existing dispatcher directly, the next coherent move was the first direct `node:http.createServer(...).listen(...)` implementation and probe.

### Substrate landed

- `pilots/rusty-js-runtime/derived/src/caps.rs`
  - Added `Net`, `NetEndpoint`, `NetListenPolicy`, `NetConnectPolicy`, and `NetOp`.
  - Added `AmbientCaps.net`.
  - Added `CapDispatcher::require_net`.
  - Added focused unit tests for loopback listen, exact listen, audit recording, and sealed-deps application/dependency behavior.
- `cruftless/Cargo.toml`
  - Added dependencies on `rusty-http-codec` and `rusty-sockets`.
- `cruftless/src/http.rs`
  - Replaced the `createServer` stub with a minimal live server object.
  - Added an active-server registry carrying listener handle, handler, handler realm, bound address, and server object.
  - Added `listen`, `address`, `close`, minimal `on`/`once`, request object, response object, and response methods: `setHeader`, `getHeader`, `removeHeader`, `writeHead`, `write`, `end`.
  - Routed `server.listen` through `rt.caps.require_net(... NetOp::Listen ...)` before binding sockets.
  - Used `rusty_sockets::listener_bind_async` and `rusty_http_codec::{parse_request, serialize_response}`.
  - Added `http::poll_io(rt)` to accept connections, enqueue request macrotasks, enter the stored handler realm, call JS, serialize response, write, and close.
- `cruftless/src/fs.rs`
  - Composed HTTP polling into the existing shared PollIo hook rather than installing a competing hook.
- `pilots/rusty-js-http-server/fixtures/direct-node-http.mjs`
  - Direct Node-style fixture: `createServer`, `writeHead`, `end`, `listen(0, "127.0.0.1")`, then `close` after one request.

### Probe result

Focused `Net` unit tests:

```text
cargo test -p rusty-js-runtime --lib caps::tests::net -- --nocapture
4 passed
```

Library compile:

```text
cargo check -p cruftless
PASS
```

Direct wire probe:

```text
cargo run -p cruftless --bin cruft -- pilots/rusty-js-http-server/fixtures/direct-node-http.mjs
HS_PORT:37225
```

Raw TCP request:

```text
GET / HTTP/1.1
Host: localhost
Connection: close
```

Response:

```text
HTTP/1.1 200 OK
content-type: text/plain
connection: close
Content-Length: 5

hello
```

Sealed ambient-denial check:

```text
cargo run -p cruftless --bin cruft -- --sealed pilots/rusty-js-http-server/fixtures/direct-node-http.mjs
```

Result: process exits with evaluation error before binding:

```text
net.listen(127.0.0.1:0): no net capability granted to module 'node:http' (mode: sealed)
```

### Findings

1. **Pred-hs.1 is corroborated for the direct floor.** Cruftless can now bind a loopback listener, accept a request, dispatch to a JS handler, and write a valid HTTP/1.1 response.
2. **The `listen` authority seam is live.** Mode 0 allows; Mode 3 sealed denies ambient `node:http` before binding.
3. **Shared PollIo composition works.** HTTP polling is called from the existing fs/timer/N-API PollIo hook, avoiding hook replacement.
4. **Realm identity is stored and used.** Active server records store `handler_realm`, and request dispatch enters that realm before `rt.call_function`.
5. **The first cut is intentionally buffered and one-request-per-connection.** This matches the seed carve-outs and direct probe target.

### Known gaps

1. `Net` capability-backed HTTP facade is not public yet; sealed allow-by-capability is still the next round.
2. Audit mode likely records through `require_net`, but a fixture-level audit assertion has not been added yet.
3. Express is not attempted yet; EventEmitter/prototype fidelity remains minimal.
4. Request/response streams are buffered object shapes, not Node stream implementations.
5. The broad `cargo test -p rusty-js-runtime caps::tests::net` command still trips pre-existing `module_golden.rs` integration-test compile errors; the focused library test lane passes.

### Next scope

1. **HS-EXT 6**: authority probes: audit log assertion, sealed denial fixture, and capability-backed loopback allow path.
2. **HS-EXT 7**: Compartment probes: no ambient HTTP, endowed facade works, handler realm preservation.
3. **HS-EXT 8**: Express minimal probe after authority/Compartment gates hold.

---

*HS-EXT 5 closes. The direct HTTP server path is live and probe-backed; the next work is authority hardening and Compartment facade composition.*
