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

---

## HS-EXT 5a — 2026-05-25 (agent-feedback hygiene closure)

Code-bearing hygiene round. This round applied the first `agent-feedback.md` review before expanding the authority and Compartment surface.

### Trigger

The cross-resolver review of HS-EXT 5 identified three small substrate-discipline gaps that should close before HS-EXT 6: enumerable engine sentinels, static string coercion at user-argument boundaries, and partial active-server lifecycle cleanup.

### Substrate landed

- `cruftless/src/http.rs`
  - Added `set_internal_slot(...)` and moved `__cruftless_http_*` server / response sentinels through non-enumerable `set_own_internal`.
  - Replaced HTTP response header/body `abstract_ops::to_string` coercion with `Runtime::coerce_to_string`, preserving object `toString` / `valueOf` dispatch.
  - Removed the write-only `closing` flag and `update_server`; `AsyncEvent::Closed` and `AsyncEvent::Error` now reclaim active-server registry slots via `remove_server`.
- `pilots/rusty-js-http-server/fixtures/hygiene-node-http.mjs`
  - Probe fixture for sentinel non-enumeration and object-valued response header/body coercion.
- `cruftless/tests/http_server.rs`
  - End-to-end binary test that starts the hygiene fixture, connects over loopback, and asserts the wire response.

### Probe result

```text
cargo test -p cruftless --test http_server --release -- --nocapture
1 passed
```

Compile lane:

```text
cargo check -p cruftless
PASS
```

### Findings

1. **Agent-feedback concerns 1-3 are closed.** `Object.keys(server)` no longer surfaces `__cruftless_http_*` slots, object-valued header names/values and body chunks use runtime ToString, and abnormal listener close/error paths no longer leave a deliberately retained slot.
2. **The live Mode-0 path remains intact.** The hygiene probe exercises the same `createServer(...).listen(0, "127.0.0.1")` path and receives `HTTP/1.1 200 OK`.
3. **EventEmitter remains the next compatibility blocker.** `server.on` / `server.once` are still placeholders; Express remains out of scope until that shape lands.

### Next scope

1. **HS-EXT 6**: authority probes: audit log assertion, sealed denial fixture, and capability-backed loopback allow path.
2. **HS-EXT 7a**: EventEmitter request listener shape on `server`, before Express.
3. **HS-EXT 7/8**: Compartment facade and Express minimal probe after the authority and EventEmitter gates hold.

---

*HS-EXT 5a closes. The HTTP server substrate is cleaner; the next coherent move is authority hardening without carrying the feedback hygiene debt forward.*

---

## HS-EXT 6 — 2026-05-25 (authority probes: audit, sealed denial, loopback allow)

Code-bearing authority round. This round hardened the Doc 736 authority seam that HS-EXT 5 made live: `createServer` remains shape construction, while `listen` is the capability-bearing operation.

### Trigger

The seed's next scope after HS-EXT 5a was authority hardening: Mode 1 audit assertion, Mode 3 sealed denial, and a capability-backed allow path narrow enough to demonstrate the facade shape without reopening ambient network authority.

### Substrate landed

- `pilots/rusty-js-runtime/derived/src/caps.rs`
  - Added a `net_grant` field to `CapDispatcher`.
  - Added `CapDispatcher::with_net_grant(...)`.
  - Updated `require_net` so sealed modes accept either the call-site capability argument or the host-granted `net_grant`.
- `cruftless/src/main.rs`
  - Added `--allow-net-loopback`, plus `CRUFT_ALLOW_NET_LOOPBACK` / `CRUFTLESS_ALLOW_NET_LOOPBACK` env aliases.
  - The grant installs `Net::loopback_server()` only; it does not grant public-interface listen or outbound connect.
- `pilots/rusty-js-http-server/fixtures/authority-fixed-node-http.mjs`
  - Fixed-port no-stdout fixture for sealed allow, avoiding any dependency on stdio authority.
- `cruftless/tests/http_server.rs`
  - Extended the HTTP-server integration test:
    - `--audit --audit-log` records `net listen(127.0.0.1:0)`.
    - bare `--sealed` denies ambient `node:http` listen before bind.
    - `--sealed --allow-net-loopback` serves `authority-ok` on loopback.

### Probe result

```text
cargo test -p cruftless --test http_server --release -- --nocapture
2 passed
```

Focused capability unit lane:

```text
cargo test -p rusty-js-runtime --lib caps::tests::net --release -- --nocapture
4 passed
```

Compile lane:

```text
cargo check -p cruftless
PASS
```

### Findings

1. **Pred-hs.4 is pinned.** Bare sealed mode rejects `node:http` listen before the socket binds.
2. **Pred-hs.5 is pinned.** Audit mode records the `net` capability event for HTTP listen.
3. **The loopback allow path is narrow.** `--allow-net-loopback` grants `NetListenPolicy::LoopbackAnyPort` through the dispatcher, enough to prove the capability-backed allow shape without restoring ambient network authority.
4. **The next compatibility blocker is EventEmitter, not authority.** Express remains blocked by `server.on("request", ...)` semantics rather than by `Net`.

### Next scope

1. **HS-EXT 7a**: implement minimal EventEmitter request listener support on `server.on` / `server.once`.
2. **HS-EXT 7**: Compartment probes: no ambient HTTP, endowed facade works, handler realm preservation.
3. **HS-EXT 8**: Express minimal probe after EventEmitter and Compartment gates hold.

---

*HS-EXT 6 closes. The HTTP listen authority seam is now probe-backed in audit, sealed-deny, and loopback-granted allow modes.*

---

## HS-EXT 7a — 2026-05-25 (minimal request EventEmitter wiring)

Compatibility round for the next HTTP blocker. Express and many Node HTTP callers do not always pass the request handler directly to `createServer`; they register it through the server's EventEmitter surface. This round implements the bounded `request` listener path without claiming full `EventEmitter` parity for HTTP server instances.

### Trigger

Agent feedback concern (4) named `server.on` / `server.once` as the blocker between the direct HTTP probe and the Express probe. HS-EXT 6 showed authority was no longer the blocker, so the next coherent move was the listener-registration shape.

### Substrate landed

- `cruftless/src/http.rs`
  - Added a non-enumerable request-listener slot on server objects.
  - Added `server.on("request", fn)`, `server.addListener("request", fn)`, and `server.once("request", fn)`.
  - Relaxed `http.createServer()` so a server may be constructed without an initial handler and wired afterward through the request event.
  - Dispatch now walks the server's request-listener list; `once` listeners are removed before invocation so a recursive or follow-on dispatch cannot observe them as still live.
- `pilots/rusty-js-http-server/fixtures/eventemitter-node-http.mjs`
  - Probe fixture for `http.createServer()` followed by `server.once("request", ...)`.
- `cruftless/tests/http_server.rs`
  - Added the EventEmitter request probe beside the hygiene and authority probes.

### Probe result

```text
cargo test -p cruftless --test http_server --release -- --nocapture
3 passed
```

### Findings

1. **The immediate Express blocker is reduced.** Request handlers can now be registered after server construction through the Node-shaped EventEmitter methods.
2. **The move is intentionally bounded.** Only the HTTP `request` event has live semantics; unrelated events are no-ops in this HTTP surface, and broader EventEmitter parity remains outside this rung.
3. **The authority seam is unchanged.** `server.listen(...)` still routes through the `Net` capability check; request listener registration remains pure shape setup.

### Next scope

1. **HS-EXT 7**: Compartment probes: no ambient HTTP, endowed HTTP facade works, handler realm preservation.
2. **HS-EXT 8**: Express minimal probe now that `.on("request", ...)` has a live path.
3. **HS-EXT 9**: composition gates after Compartment and Express probes land.

---

*HS-EXT 7a closes. The request-listener compatibility path now exists without widening the network authority model.*

---

## HS-EXT 7b — 2026-05-25 (telos sharpening after request-listener closure)

Apparatus round, not a code-bearing round. HS-EXT 7a removed the immediate EventEmitter blocker, so this round re-sharpens the locale against its original Doc 736 / Compartment telos before the next implementation move.

### Trigger

After request listeners landed, the tempting next move would be to chase Express directly. That would answer a compatibility question, but it would not answer the locale's load-bearing architectural question: can Node-compatible HTTP serving be exposed to untrusted JS only by object-reference authority, with closed ambient discovery and realm-preserving dispatch?

### Sharpened Reading

1. **The target is capability reachability, not process flags.** `--allow-net-loopback` is a useful sealed-mode bridge, but Doc 736's strict property wants code to hold a specific object reference that carries narrowed network authority. A CLI flag proves the dispatcher can deny/allow; a facade proves the reference graph.
2. **The HTTP facade is the capability surface.** `new Compartment({ globals: { http } })` should receive a narrowed namespace object. That object may look like `node:http`, but its returned servers must close over the endowed `Net` policy and pass that policy to the same internal listen path as ambient `node:http`.
3. **No ambient discovery is the first Compartment assertion.** A compartment with no endowed `http` and no module-map entry for `node:http` must not be able to reach the ambient namespace by global lookup, import, or require-shaped fallback.
4. **`listen` remains the only network effect.** `createServer`, `on`, `once`, and `addListener` are shape-building operations. They may retain callbacks and realm identity, but they must not consume network authority.
5. **Request dispatch is part of the authority story.** A handler created in a compartment must run under the compartment's realm when a later PollIo macrotask fires. Otherwise the code was loaded in a closed graph but executed in an ambient world, which would violate the telos at call time.
6. **Express is downstream evidence.** Express should be attempted only after the no-ambient / endowed-facade / realm-preservation probes pass, because otherwise an Express pass could conceal an architectural shortcut.

### HS-EXT 7 Acceptance Shape

Minimum probes for the next code-bearing round:

1. **No ambient HTTP**: compartment code evaluating `typeof http` sees absence unless `http` is endowed.
2. **No builtin escape**: compartment code cannot import or require `node:http` unless the compartment module map or globals explicitly provides the facade.
3. **Endowed facade allows loopback**: a compartment endowed with a loopback HTTP facade serves `127.0.0.1:0`.
4. **Endowed facade denies wider bind**: the same facade denies `0.0.0.0:0`.
5. **Realm preservation**: a request handler installed inside the compartment observes the compartment's `globalThis` / endowed sentinel, not the outer host realm.

### Implementation Implication

The current HTTP server machinery is close but not done:

- `server.listen` currently calls `require_net(&Net::none(), ...)`; HS-EXT 7 needs a per-server stored `Net` authority so ambient namespace and facade namespace share the same bind path with different capabilities.
- The request listener registry introduced in HS-EXT 7a must carry the realm of each listener or otherwise preserve the server/facade creation realm. A single `handler_realm` per active server may be enough for first cut if all request listeners are registered from the endowed compartment before `listen`.
- Facade construction can remain host-internal for the probe. It does not need a public JS API yet; it only needs to demonstrate that the object passed into `Compartment({ globals })` is the authority-bearing reference.

### Next Scope

1. **HS-EXT 7**: implement the internal HTTP facade and Compartment probes above.
2. **HS-EXT 8**: attempt Express only after HS-EXT 7 proves object-reference authority and request-time realm preservation.
3. **HS-EXT 9**: run composition gates and decide whether the facade API should become public or remain an internal test harness until the broader capability-handle surface lands.

---

*HS-EXT 7b closes as apparatus. The next code should prove the capability graph, not merely broaden HTTP compatibility.*

---

## HS-EXT 7 — 2026-05-25 (Compartment HTTP facade: object-reference authority)

Code-bearing Compartment round. This round turns the HS-EXT 7b telos sharpening into probes: no ambient HTTP in an endowment-less compartment, a loopback-only endowed HTTP facade that works under sealed mode, wider binds denied by that same facade, and request dispatch observing a compartment endowment.

### Trigger

HS-EXT 7b established that Express should not be attempted until the object-reference authority story is probe-backed. The existing Compartment primitive already had `evaluate`, globals endowments, modules/import, and cap endowment probes, so the HTTP locale only needed a narrowed facade path over the existing server machinery.

### Substrate landed

- `cruftless/src/http.rs`
  - Factored HTTP namespace construction through `make_http_namespace(rt, net_cap)`.
  - Ambient `node:http` still installs with `Net::none()`, preserving Mode 0 behavior and sealed denial unless another grant path applies.
  - Added an internal probe factory, `__cruftless_makeHttpFacade`, that returns the same HTTP namespace shape closed over `Net::loopback_server()`.
  - `server.listen(...)` now uses the namespace/facade's captured `Net` policy when routing through `rt.caps.require_net(...)`.
- `pilots/rusty-js-http-server/fixtures/compartment-no-ambient-http.mjs`
  - Verifies an endowment-less compartment sees `typeof http === "undefined"` and `typeof require === "undefined"`.
- `pilots/rusty-js-http-server/fixtures/compartment-facade-loopback-fixed.mjs`
  - Sealed-mode fixed-port fixture: host creates the internal facade, endows it into a compartment, the compartment serves loopback, and the request callback returns an endowed marker.
- `pilots/rusty-js-http-server/fixtures/compartment-facade-deny-wide.mjs`
  - Sealed-mode denial fixture: the same loopback facade attempts `0.0.0.0:0` and is rejected before bind.
- `cruftless/tests/http_server.rs`
  - Added `http_server_compartment_facade_authority` covering the three fixtures above.

### Probe result

```text
cargo test -p cruftless --test http_server --release -- --nocapture
4 passed
```

### Findings

1. **Pred-hs.6 is pinned for the internal facade shape.** A compartment without an endowed HTTP object has no ambient HTTP surface; a compartment with the loopback facade can bind loopback under sealed mode.
2. **Pred-hs.8 is pinned.** The loopback facade denies `0.0.0.0:0`, proving the facade carries a narrowed `Net` policy rather than reopening ambient `net`.
3. **Pred-hs.7 is partially pinned through endowment observation.** The request callback returns the compartment-endowed `marker`, proving request-time dispatch sees the compartment environment. The current Compartment substrate did not expose the same marker via `globalThis.marker`, so the probe uses direct endowed binding observation.
4. **The public API is intentionally deferred.** `__cruftless_makeHttpFacade` is an internal harness proving the reference-graph shape; a public facade constructor should land with the broader capability-handle API, not as an HTTP-only user API.

### Next scope

1. **HS-EXT 8**: Express minimal probe, now that direct HTTP, authority modes, request listener shape, and Compartment facade authority are all probe-backed.
2. **HS-EXT 9**: composition gates: focused capability lane, compile lane, and any locale-standard regression sample.
3. **Later**: replace fixed-port sealed fixtures with dynamic-port authority handoff once stdout/stdio capability handles are available for sealed probes.

---

*HS-EXT 7 closes. HTTP serving can now be endowed into a Compartment as object-reference authority rather than discovered ambiently.*

---

## HS-EXT 8 — 2026-05-25 (Express minimal probe: blocked before listen)

Compatibility probe round. This round attempted the first real Express serving fixture after the HTTP substrate, authority gates, request EventEmitter path, and Compartment facade were all probe-backed.

### Trigger

HS-EXT 7 closed the architectural authority gate. The next trajectory target was therefore Express minimal: prove that a real Express app can sit on top of the live `node:http` server path without changing the authority model.

### Substrate landed

- `pilots/rusty-js-http-server/fixtures/express-minimal/package.json`
  - Minimal package manifest for Express 4.
- `pilots/rusty-js-http-server/fixtures/express-minimal/express-minimal.mjs`
  - Minimal route fixture:
    - `import express from "express"`
    - `const app = express()`
    - `app.get("/", (req, res) => res.status(200).set("x-from", "express").send("hello express"))`
    - `http.createServer(app).listen(39733, "127.0.0.1")`
- `cruftless/tests/http_server.rs`
  - Added an opt-in Express probe. It skips unless `node_modules/express` exists in the fixture root.
  - `CRUFTLESS_EXPRESS_FIXTURE_ROOT=/tmp/cruftless-hs-express-minimal` can point the test at a disposable installed fixture.

### Probe result

Baseline HTTP lane without Express deps:

```text
cargo test -p cruftless --test http_server --release -- --nocapture
5 passed
```

The Express probe skipped in that lane because no in-repo `node_modules/express` is committed.

Disposable dependency materialization:

```text
npm install --prefix /tmp/cruftless-hs-express-minimal
PASS
```

Focused Express probe:

```text
CRUFTLESS_EXPRESS_FIXTURE_ROOT=/tmp/cruftless-hs-express-minimal \
  cargo test -p cruftless --test http_server http_server_express_minimal_probe --release -- --nocapture
FAIL: connection refused
```

Direct fixture execution showed the real pre-listen blocker:

```text
cruftless /tmp/cruftless-hs-express-minimal/express-minimal.mjs
cruft: evaluation error: Thrown: TypeError: Router.use() requires a middleware function
```

The same error occurred even after trimming the fixture to a single `app.get("/")` route, so the failure is in Express router/middleware registration during route setup rather than in HTTP listener bind, request dispatch, or response serialization.

### Findings

1. **Express remains blocked, but not by HTTP authority or listen.** The process exits before `server.listen(...)`, so the current blocker is upstream of the HTTP server.
2. **The likely next locus is Express's router/middleware shape.** Express 4 reports `Router.use() requires a middleware function`, which suggests an argument/callability/prototype/rest-argument mismatch around router registration.
3. **The fixture is intentionally opt-in.** We do not commit `node_modules`; the test skips unless a current fixture root has dependencies installed. This avoids reintroducing dependence on legacy fixture paths.
4. **HTTP substrate gates still hold.** The ordinary HTTP integration lane passes with direct, hygiene, authority, EventEmitter, and Compartment facade probes.

### Next scope

1. **HS-EXT 8a**: debug Express router registration with a narrow fixture that imports Express and exercises `express().get("/", fn)` without starting a server.
2. Inspect Express 4's `application.get`, `Router`, and `Route` call path under Cruftless to identify whether the bad value is the route callback, a generated router function, or an array/rest wrapper.
3. Only after route registration succeeds, re-enable the serving probe as a passing HS-EXT 8 closure.

---

*HS-EXT 8 did not close Express serving by itself. It usefully moved the blocker upstream: Express route registration, not HTTP listen authority.*

---

## HS-EXT 8a — 2026-05-25 (Upstream Express blocker resolved: var/parameter aliasing)

This round zoomed into the upstream Express failure instead of widening the HTTP server surface.

### Finding

Express 4.22.2's `Router.use` is shaped like:

```js
proto.use = function use(fn) {
  if (typeof fn !== "function") { ... }
  var callbacks = flatten(slice.call(arguments, offset));
  ...
  for (...) {
    var fn = callbacks[i];
    ...
  }
}
```

The `arguments` object was intact, but the formal parameter `fn` read as `undefined` before the loop. The compiler had allocated a second function-scoped `var fn` local even though ECMAScript `var` redeclaration of a parameter must reuse the parameter binding. Express therefore misclassified the middleware function as a path, set `offset = 1`, sliced away the only argument, and threw `Router.use() requires a middleware function`.

### Substrate change

- `pilots/rusty-js-bytecode/derived/src/compiler.rs`
  - Function-body hoist preallocation now skips a `var` name when an existing local already resolves that name, preserving parameter/var aliasing.
- `pilots/rusty-js-runtime/derived/tests/run_golden.rs`
  - Added `var_redeclaration_reuses_parameter_binding` so the Express-shaped rule stays pinned.

### Probe result

```text
cargo test -p rusty-js-runtime var_redeclaration_reuses_parameter_binding -- --nocapture
1 passed
```

```text
CRUFTLESS_EXPRESS_FIXTURE_ROOT=/tmp/cruftless-hs-express-minimal \
  cargo test -p cruftless --test http_server http_server_express_minimal_probe --release -- --nocapture
1 passed
```

Baseline no-deps HTTP lane remains green; Express skips there unless dependencies are installed:

```text
cargo test -p cruftless --test http_server --release -- --nocapture
5 passed
```

### Closure

Express minimal now composes over the existing apparatus:

- no ambient listen authority is introduced;
- the opt-in fixture uses the same live `node:http` server path;
- the fix is a general JS semantics correction, not an Express shim.

---

*HS-EXT 8a closes the upstream Express router-registration blocker. The next coherent move is broader Express behavior, still under sealed authority and Compartment discipline.*

---

## HS-EXT 8b — 2026-05-25 (Express middleware/params/query probe)

Compatibility probe round. This round widened Express evidence without changing the HTTP authority path: middleware registration, route params, query parsing, response headers, and `status(...).send(...)` now sit under the same live `node:http` server surface.

### Trigger

After HS-EXT 8a resolved the upstream Express route-registration blocker, the next coherent move was a slightly broader Express fixture that still exercises the same `http.createServer(app).listen(...)` path. The test remains opt-in on installed Express dependencies and does not commit `node_modules`.

### Substrate landed

- `pilots/rusty-js-http-server/fixtures/express-minimal/express-middleware.mjs`
  - Adds `app.use((req, res, next) => ...)` middleware.
  - Adds `app.get("/user/:id", ...)` route params.
  - Reads `req.query.q`.
  - Writes middleware and route headers.
  - Responds through `res.status(201).set(...).send(...)`.
- `cruftless/tests/http_server.rs`
  - Adds `read_http_path(...)` so probes can request non-root paths.
  - Adds `http_server_express_middleware_probe`.

### Probe result

Disposable installed fixture root:

```text
CRUFTLESS_EXPRESS_FIXTURE_ROOT=/tmp/cruftless-hs-express-minimal \
  cargo test -p cruftless --test http_server http_server_express_middleware_probe --release -- --nocapture
1 passed
```

Full HTTP-server lane with installed Express deps:

```text
CRUFTLESS_EXPRESS_FIXTURE_ROOT=/tmp/cruftless-hs-express-minimal \
  cargo test -p cruftless --test http_server --release -- --nocapture
6 passed
```

### Findings

1. **Express middleware composition works for the first chained case.** A middleware can set response headers, attach request state, call `next()`, and the later route sees the request mutation.
2. **Route params and query parsing are live enough for the fixture.** `/user/42?q=abc` reaches `req.params.id === "42"` and `req.query.q === "abc"`.
3. **The authority story is unchanged.** The fixture still reaches network only through `http.createServer(app).listen(...)`; no ambient Compartment or sealed-mode bypass is introduced.
4. **The fixture remains dependency-opt-in.** The tracked repo contains source fixtures only; tests skip unless `CRUFTLESS_EXPRESS_FIXTURE_ROOT` points at a root with `node_modules/express`.

### Next scope

1. Add a sealed/facade Express probe once dynamic port handoff and stdio authority are less awkward, proving the same Express app can be endowed rather than ambiently imported.
2. Exercise error middleware / 404 fallthrough as the next Express behavior edge.
3. Keep binary/streaming response bodies and slow/chunked request reads as later HTTP substrate work, not Express-specific blockers.

---

*HS-EXT 8b closes a broader Express compatibility rung while preserving the Doc 736 authority model.*
