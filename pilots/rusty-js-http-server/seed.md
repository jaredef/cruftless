# rusty-js-http-server — Resume Vector / Seed

**Locale tag**: `L.rusty-js-http-server` (top-level per Doc 737 §IV)

**Status as of 2026-05-23**: **WORKSTREAM FOUNDED (HS-EXT 0)**. No code yet. Spawned per keeper directive 2026-05-23 18:12-local. Fourth of four spawns in this round.

**Workstream**: implement an actual HTTP server surface for cruft. Per keeper observation: HTTP server APIs (`bun:serve`, Node `http.createServer`) are currently stubbed. Real Node packages (express, fastify, koa, etc.) cannot run without a real HTTP server.

**Author**: 2026-05-23 session.
**Parent**: cruftless engagement (`/home/jaredef/rusty-bun`). Standalone top-level pilot.
**Composes with**:
- [Findings doc IV.4 standing fuzz](../rusty-js-jit/findings.md) — any default-on flip uses canonical fuzz per rule 10
- [Doc 736 §IX](../../../corpus-master/corpus/736-the-architecturally-impossible-supply-chain-attack-capability-passing-closed-import-graphs-and-load-time-integrity-as-the-design-that-removes-ambient-authority.md) — cap-passing modes: HTTP server is a NET capability per the design; gated under Mode 2+ sealed
- [Doc 735 §X.h.b](../../../corpus-master/corpus/735-the-temporal-resolver-instance-stack-build-time-process-time-call-time-as-the-time-axis-dual-to-doc-729s-spatial-stack.md) — (P2) categorization
- [Doc 731 §VII R1](../../../corpus-master/corpus/731-the-jit-as-a-lowering-compiler-tier-alphabet-purity-upstream-as-the-bound-on-jit-complexity.md) — preserved (HTTP server is a runtime surface, not JIT)

## I. Telos

**Empirical answer to**: can cruft serve a basic HTTP/1.1 request-response cycle compatible with Node's `http.createServer` API?

This is a substantial multi-session substrate workstream. First cut: minimal viable HTTP/1.1 server that accepts a request, dispatches to a JS-side handler, writes a response. Body streaming + keep-alive + chunked encoding follow in subsequent rounds.

### I.1 First-cut scope

- **Node `http.createServer(handler)` API surface** (vs Bun-specific `Bun.serve` which is the same shape under different name)
- **HTTP/1.1 only** (no HTTP/2; no HTTP/3)
- **Request: method + URL + headers + body** parsed and exposed as IncomingMessage-shape
- **Response: writeHead + write + end** exposed as ServerResponse-shape
- **Single-connection at a time** (no concurrent request handling at first cut; tokio runtime addition is forward work)
- **Mode 0 cap-passing** (sealed-mode integration per Doc 736 §IX.6 is HS-EXT 8+)

Out of scope: WebSocket upgrade; HTTPS; HTTP/2; keep-alive; chunked encoding; pipelining; body streaming (request body buffered fully first); concurrent connections.

### I.2 Falsifiers

**Pred-hs.1**: cruft can serve a minimal Express app (`app.get('/', (req, res) => res.send('hello'))`). Falsifier: Express doesn't run.

**Pred-hs.2**: HTTP/1.1 round-trip wire-equal to node's response on simple cases (HTTP version + status line + standard headers + body match). Falsifier: divergence in wire format.

**Pred-hs.3**: canonical fuzz remains byte-identical post-implementation (no shape-correctness regression from HTTP server wireup).

**Pred-hs.4**: diff-prod 42/42 holds.

**Pred-hs.5**: cap-passing modes preserved (Doc 736 §IX). Under Mode 3 (sealed), HTTP server requires explicit `caps: { net: [...] }` per the application's caps declaration.

## II. Apparatus

Substantial new substrate at the rusty-js-runtime tier + likely a new sub-crate or module. Composes with:
- **Existing socket / TCP substrate** (cruft has a `tls` pilot per the manifest; may have or need a basic-tcp substrate)
- **Canonical fuzz** (standing): correctness gate
- **Cap-passing dispatcher** (per Doc 736 §IX.6)

Per Doc 738 §II.e: implementation likely lands at `pilots/rusty-js-runtime/derived/src/http_server.rs` (new module) or a new sub-crate `pilots/rusty-js-http-server/derived/` if scope grows.

## III. Methodology

1. **HS-EXT 0** — workstream founding (this seed + trajectory + scaffold).
2. **HS-EXT 1** — survey existing TCP/socket substrate; identify what's available + the gap.
3. **HS-EXT 2** — minimal HTTP/1.1 parser. Output: `docs/http-parser-design.md`.
4. **HS-EXT 3** — Server / Request / Response object surface design. Output: `docs/api-design.md`.
5. **HS-EXT 4** — `http.createServer` + `server.listen` API binding to rusty-js-runtime.
6. **HS-EXT 5** — minimal request-handler dispatch (single-connection, synchronous handler).
7. **HS-EXT 6** — Express compatibility probe: build a minimal Express-style app; verify cruft serves it.
8. **HS-EXT 7** — composition probes (canonical fuzz; diff-prod).
9. **HS-EXT 8** — cap-passing integration (Mode 2+ sealed).
10. **HS-EXT 9** — default-on (substrate is opt-in via createServer API call; no flag).

## IV. Carve-outs and bounded scope

- HTTP/1.1 only; HTTP/2 + HTTPS + HTTP/3 deferred
- Single-connection at first cut; concurrent connections deferred
- Buffered request body; streaming deferred
- Node `http.createServer` API only; Bun.serve compatibility is a follow-on
- WebSocket upgrade deferred
- Per Findings rule 5 + standing rule 10: canonical fuzz at default-on flip

## V. Standing artefacts

- `pilots/rusty-js-http-server/seed.md`, `trajectory.md`
- `pilots/rusty-js-http-server/docs/` for http-parser-design + api-design
- `pilots/rusty-js-http-server/fixtures/` for HTTP-specific test cases
- Implementation lands in `pilots/rusty-js-runtime/derived/src/http_server.rs` (or new sub-crate if scope grows)

## VI. Resume protocol

Read this seed, then trajectory.md tail. Multi-session pilot; substantive substrate work follows the survey + design rounds.
