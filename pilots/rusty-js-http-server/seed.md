# rusty-js-http-server — Resume Vector / Seed

**Locale tag**: `L.rusty-js-http-server` (top-level per Doc 737 §IV)

**Status as of 2026-05-25**: **DIRECT MODE-0 PROBE PASSING at HS-EXT 5**. Originally founded at HS-EXT 0 (2026-05-23) as "basic HTTP server." HS-EXT 1's transport survey and the subsequent Doc 736 / Compartment composition reading sharpened the telos; HS-EXT 3 translated that telos into the concrete API wireup plan; HS-EXT 4 decided the `Net` capability substrate can extend the existing dispatcher directly; HS-EXT 5 added the first live `node:http.createServer(...).listen(...)` path. HTTP server is the first runtime surface where Node-compatible network serving, capability authority, compartment realm dispatch, and PollIo event-loop integration must compose.

**Workstream**: implement an actual HTTP server surface for cruft, but with the authority boundary made explicit. `http.createServer(handler)` is API shape; `server.listen(...)` is the effectful network authority. In Mode 0 it remains Node-compatible. In sealed modes and inside Compartments it must be reachable only through an explicit `Net` capability or a capability-backed `http` facade.

**Author**: 2026-05-23 session, reformalized 2026-05-25.
**Parent**: cruftless engagement. Standalone top-level pilot.
**Composes with**:
- [Doc 736](../../docs/736-the-architecturally-impossible-supply-chain-attack-capability-passing-closed-import-graphs-and-load-time-integrity-as-the-design-that-removes-ambient-authority.md) — capability-passing runtime. HTTP listen is a `Net` capability operation.
- [compartment-primitive locale](../compartment-primitive/) — JS-visible `Compartment` API. Compartments are the application-facing way to endow a narrowed HTTP/network authority to untrusted code.
- [rusty-js-caps pilot](../rusty-js-caps/) — four capability modes, dispatcher pattern, audit/sealed semantics.
- [node-http pilot](../node-http/) — closed Node HTTP data-layer semantics.
- [http-codec pilot](../http-codec/) — whole-message HTTP/1.1 parse/serialize substrate.
- [sockets pilot](../sockets/) — TCP listener/stream substrate.
- [Findings doc IV.4 standing fuzz](../rusty-js-jit/findings.md) — default-on / route-through changes preserve canonical fuzz.
- [Doc 735 §X.h.b](../../docs/735-the-temporal-resolver-instance-stack-build-time-process-time-call-time-as-the-time-axis-dual-to-doc-729s-spatial-stack.md) — process-time / call-time authority boundary.

## I. Telos

**Empirical answer to**: can Cruftless serve HTTP/1.1 through a Node-compatible `http.createServer` surface while preserving Doc 736's capability-passing architectural property and executing compartment-created handlers inside their originating compartment realm?

The locale's load-bearing observation:

- `http.createServer(handler)` is mostly pure object construction.
- `server.listen(host, port)` is the authority-bearing operation.
- A server created inside a `Compartment` carries both the handler and the compartment realm identity.
- `PollIo` request dispatch must enter that stored realm before invoking the handler and exit it afterward.
- In sealed capability modes, binding a listener must require `Net` authority.

The first compatibility target remains Node-style HTTP serving. The sharpened telos is that compatibility must not introduce ambient network authority that bypasses Doc 736 or Compartment endowment discipline.

### I.1 First-cut scope

- **Node `http.createServer(handler)` API surface**.
- **HTTP/1.1 only**.
- **Request shape**: method, URL, headers, HTTP version, buffered body.
- **Response shape**: statusCode/statusMessage, setHeader/getHeader, writeHead, write, end, headersSent.
- **TCP listener via existing sockets substrate**.
- **Foreground handler invocation**: accepted requests enqueue runtime work and call JS via `rt.call_function` on the runtime thread, never directly from a background listener thread.
- **Realm-preserving dispatch**: handlers created in a compartment are invoked under that compartment's realm/global environment.
- **Authority seam**: `server.listen` routes through a `Net` capability check in sealed/audit modes, with Mode 0 preserving Node-equivalent compatibility.
- **Capability-backed HTTP facade**: design must support a compartment-endowed `http` object closed over a `Net` capability, even if the first implementation exposes only the ambient Mode-0 namespace.

Out of scope for first cut: HTTPS/server-side TLS, HTTP/2, HTTP/3, WebSocket upgrade, keep-alive, pipelining, streaming request bodies, chunked request streaming, multiple concurrent request handlers, full TC39 Compartment hooks, Module Source records.

### I.2 Falsifiers

**Pred-hs.1 (Node direct)**: Cruftless can serve a direct Node-style app:

```js
http.createServer((req, res) => res.end("hello")).listen(0, "127.0.0.1")
```

Falsifier: local TCP client cannot receive a valid `HTTP/1.1 200 OK` response with body `hello`.

**Pred-hs.2 (Express minimal)**: Cruftless can serve a minimal Express app (`app.get("/", (req, res) => res.send("hello"))`). Falsifier: Express does not run or cannot write a response after the direct Node probe succeeds.

**Pred-hs.3 (wire floor)**: simple HTTP/1.1 responses match Node/Bun on status line, Content-Length/body bytes, and close behavior after dynamic/header-order normalization. Falsifier: semantic wire divergence on the direct probe.

**Pred-hs.4 (capability authority)**: under Mode 3 sealed, `server.listen` without a `Net` capability throws `CapabilityError` before binding a socket. Falsifier: a sealed program binds a listener through ambient `node:http`.

**Pred-hs.5 (audit authority)**: under Mode 1 audit, successful `server.listen` records a `net.listen(host, port)`-class event with caller/module provenance. Falsifier: network binding occurs without an audit record.

**Pred-hs.6 (compartment authority)**: a `Compartment` without an endowed HTTP/Net capability cannot bind a server; the same compartment with a capability-backed HTTP facade can bind only within that capability's host/port scope. Falsifier: compartment code reaches ambient `node:http` or binds outside the endowed authority.

**Pred-hs.7 (realm preservation)**: a handler created inside a compartment runs with that compartment's realm/globalThis and cannot observe host ambient globals during request dispatch. Falsifier: request callback dispatch escapes to the outer realm or sees denied ambient bindings.

**Pred-hs.8 (regression gates)**: canonical fuzz remains byte-identical, and diff-prod remains at its current all-pass baseline after each implementation round. Falsifier: unrelated runtime-semantics regression.

## II. Apparatus

This locale is a composition locale over existing substrates:

- **Data semantics**: `pilots/node-http/derived/`
- **Wire codec**: `pilots/http-codec/derived/`
- **TCP transport**: `pilots/sockets/derived/`
- **Event loop**: `HostHook::PollIo` + runtime macrotask queue
- **Capability authority**: `Runtime.caps` dispatcher from `rusty-js-caps`
- **Compartment realm isolation**: `Compartment` + `allocate_compartment_realm` from `compartment-primitive`

Implementation likely lands in:

- `cruftless/src/http.rs` for the live `node:http` namespace and JS-visible server/request/response objects.
- `pilots/rusty-js-runtime/derived/src/` if new runtime-level server registry, realm dispatch, or PollIo/macrotask support is needed.
- `pilots/rusty-js-runtime/derived/src/caps.rs` if a `Net` capability class/op is not already present or needs extension.

The design should prefer a small runtime-owned active-server registry over ad hoc JS-side state. Each active server record should carry at least:

- listener handle
- handler `Value`
- creating realm id / compartment realm id where applicable
- authority source (`Ambient`, `Audit`, `NetCapability`)
- bound address
- open/closed state

## III. Methodology

1. **HS-EXT 0** — original workstream founding.
2. **HS-EXT 1** — transport survey. Identified HTTP server as composition locale over `node-http`, `http-codec`, `sockets`, `PollIo`, caps, and Compartments.
3. **HS-EXT 2** — this reformalization. Seed/trajectory rewritten so capability + Compartment semantics are load-bearing rather than late garnish.
4. **HS-EXT 3** — `docs/api-wireup-design.md`. Specified active-server registry, object slots, authority check flow, realm-preserving handler dispatch, and PollIo/macrotask behavior.
5. **HS-EXT 4** — `docs/net-capability-design.md`. Decided no nested locale is needed for the first `Net` cut; extend the existing cap dispatcher directly.
6. **HS-EXT 5** — code-bearing first round: added `Net` to `caps.rs`, routed HTTP listen through `require_net`, and implemented direct Node HTTP probe: create/listen/respond over localhost in Mode 0.
7. **HS-EXT 6** — authority probes: Mode 1 audit, Mode 3 sealed denial, capability-backed allow.
8. **HS-EXT 7** — Compartment probes: no ambient HTTP, endowed HTTP facade works, realm preservation on request callback.
9. **HS-EXT 8** — Express minimal probe.
10. **HS-EXT 9** — composition gates: canonical fuzz, diff-prod, targeted regression around Compartment probes and caps probes.
11. **HS-EXT 10** — default-on disposition. The API is opt-in by calling `createServer`; no flag needed for Mode 0. Sealed behavior follows the existing capability mode flags.

## IV. Carve-outs and bounded scope

- Plain HTTP/1.1 only; HTTPS/server-side TLS deferred.
- One request per connection; keep-alive and pipelining deferred.
- Buffered request body; streaming deferred.
- Single-process runtime-thread JS dispatch; concurrent request handlers deferred.
- Node `http.createServer` first; `Bun.serve` live wireup follows after the authority model is proven.
- WebSocket upgrade deferred.
- Full TC39 Compartment hooks and Module Source records deferred to their own Compartment follow-on locales.
- Capability-backed facade can initially be a Cruftless-internal construction path rather than public API, as long as the seam is explicit and probed.

## V. Standing Artefacts

- `pilots/rusty-js-http-server/seed.md`
- `pilots/rusty-js-http-server/trajectory.md`
- `pilots/rusty-js-http-server/docs/transport-survey.md`
- `pilots/rusty-js-http-server/docs/api-wireup-design.md`
- `pilots/rusty-js-http-server/docs/net-capability-design.md`
- `pilots/rusty-js-http-server/fixtures/` for direct HTTP, authority, compartment, and Express probes

## VI. Resume Protocol

Read this seed, then the tail of `trajectory.md`, then `docs/transport-survey.md`, `docs/api-wireup-design.md`, and `docs/net-capability-design.md`. Do not start from parser or data-layer work: those substrates already exist. The direct Mode-0 probe now passes. The next coherent move is HS-EXT 6: harden the authority probes (`--audit`, `--sealed`, loopback capability facade shape) and begin Compartment-facing facade work without regressing the direct probe.
