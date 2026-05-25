# rusty-js-http-server — Agent Feedback

Cross-agent review notes. Read on every locale entry. Append (do not overwrite) new entries chronologically; the head of this file is the running summary for the next agent.

---

## Running summary for the next agent entering this locale

1. **HS-EXT 5a closed the hygiene feedback.** Engine sentinels now use the canonical non-enumerable `Runtime::set_engine_sentinel` path; HTTP header/body user coercion routes through `Runtime::coerce_to_string`; listener `Closed` / `Error` events reclaim active-server registry slots. The hygiene probe pins sentinel non-enumeration and object-argument `toString`.
2. **HS-EXT 6 closed the authority probes.** `--audit` records `net listen(...)`; bare `--sealed` denies ambient `node:http` listen before bind; `--sealed --allow-net-loopback` grants only loopback listen and serves the fixed-port probe.
3. **HS-EXT 7a closed the immediate EventEmitter blocker.** `server.on("request", fn)`, `server.addListener("request", fn)`, and `server.once("request", fn)` now register request listeners; `http.createServer()` without an initial handler can serve once a request listener is added. The focused fixture pins the `.once("request")` path.
4. **HS-EXT 7 closed the Compartment facade gate.** Endowment-less compartments see no ambient `http` / `require`; an internal `__cruftless_makeHttpFacade()` creates a loopback-only HTTP namespace that serves from a sealed compartment; the same facade denies `0.0.0.0`; request dispatch observes the compartment endowment.
5. **HS-EXT 8a resolved the upstream Express blocker.** Express 4's `Router.use` redeclares its `fn` parameter with `var fn` later in the function; Cruftless incorrectly allocated a second local for that `var`, so the parameter read as `undefined` and Express sliced away its middleware argument. The bytecode compiler now reuses existing parameter bindings for function-scoped `var` hoists, and the opt-in Express minimal HTTP probe passes when pointed at an installed fixture root.
6. **Binary bodies and slow/chunked reads remain later-scope gaps.** Response body accumulation is still string-backed, and `read_request` remains bounded whole-request polling. These are not EventEmitter or Compartment blockers.
7. **Pre-existing test gate.** `cargo test -p rusty-js-runtime` previously tripped a pre-HS `module_golden.rs` compile error in the integration-test target. Focused `--lib caps::tests::net` lane passed (4 tests). The trajectory documents this honestly; do not waste time chasing it as an HS regression without first rechecking current `main`.

---

## Review 1 — 2026-05-25

**Reviewer:** Claude Opus 4.7 (1M context).
**Target:** HS-EXT 5 (commit `c11f5141` — "wire node http server").
**Scope:** Read of `cruftless/src/http.rs`, `pilots/rusty-js-runtime/derived/src/caps.rs`, `cruftless/src/fs.rs` diff, seed.md, trajectory.md HS-EXT 0–5, and the three design docs under `docs/`. Gates re-verified locally: `cargo build --release -p cruftless` clean (46s); diff-prod 42/42; random-300 prev-PASS 300/300, 0 regressions.

### What lands well

- **Locale discipline is exemplary.** HS-EXT 0–4 were five apparatus-tier rounds (founding, transport survey, telos reformalization against Doc 736, API-wireup design, Net-capability design) before any code shipped. HS-EXT 5 is then a focused implementation rung whose shape is fully justified by the HS-EXT 3 design doc. The five-round setup is more careful than most agents would invest before a substrate move.
- **Architectural call is right.** `createServer` = shape, `listen` = authority. That split is the load-bearing Doc 736 composition seam and it lands cleanly. `server.listen` routes through `rt.caps.require_net(...)` before binding; sealed-mode probe (`--sealed`) actually rejects with a helpful hint before the socket binds.
- **Realm preservation around handler invocation is the non-trivial Doc 736 piece, and it is present.** `handler_realm` stored on the active-server record; `rt.enter_realm` / `rt.exit_realm` bracket the `call_function`. Handler created in a compartment dispatches under its originating realm per the HS-EXT 2 reformalization.
- **PollIo composition is a 3-line touch into the shared `fs::install_poll_io` hook** rather than installing a competing hook. Correct composition.
- **`Net` capability slots into `caps.rs` cleanly.** Same struct shape as `Fs` / `Env` / `Clock`; same `require_*` method body shape; same mode-aware policy resolution. Focused unit tests cover the four Net policy paths. Reads like the existing caps modules; nothing jars.

### Concerns ranked by leverage

1. **Engine-sentinel naming leaks through enumeration.** `__cruftless_http_server_id`, `__cruftless_http_body`, `__cruftless_http_headers`, `__cruftless_http_handler`, `__cruftless_http_ended`, `__cruftless_http_bound_addr` are installed via `rt.object_set` (default `{w:t, e:t, c:t}`). Per CLAUDE.md's source-identifier convention, `__name` should be non-enumerable. Same observable-shape gap that EIPD-EXT 1 and GBNE-EXT 1 closed elsewhere today. `Object.keys(server)` will surface all six on a `server` instance. Fix shape: install via `dict_mut().insert(PropertyKey::String(...), PropertyDescriptor { ..., enumerable: false, configurable: false, ... })`, or factor a small `set_engine_sentinel(rt, id, name, value)` helper.

2. **`abstract_ops::to_string` at user-arg coercion boundaries.** `value_to_string` wraps the static helper; called from setHeader, getHeader, removeHeader, writeHead, write, end. RPTC.7's grep-detectable bug pattern: Object args with `toString()` collapse to `"[object Object]"`. Not catastrophic for HTTP (most real callers are primitives) but the substrate has been actively eradicating this pattern. Replace with `rt.coerce_to_string(&v)?`.

3. **Active-server lifecycle is partial.** `remove_server` runs only on `close`; `closing` flag is set on listener `AsyncEvent::Closed` but never compacts the slot. Listener failure or torn socket leaks the registry entry. `update_server` exists but is functionally write-only. Either drop `closing`/`update_server` or actually use them for slot reclamation.

4. **`server.on` / `server.once` are silent no-ops.** Express and most production HTTP frameworks register the request handler via `.on('request', ...)` rather than passing it to `createServer`. HS-EXT 8 targets Express; without real EventEmitter wiring (or at minimum, the `request` event aliasing to the createServer-passed handler), the Express probe will not run.

5. **Response body is `Value::String` only.** Binary responses (images, gzipped JSON, anything from a `Buffer` or `Uint8Array`) will not round-trip correctly. Carve-out is named in the seed; flagging for visibility.

6. **`read_request` is busy-poll with a 500ms wall-clock cap.** Works fine for fast loopback. Slow or chunked-Transfer-Encoding clients will get truncated requests. Acceptable v1 deviation per the seed's single-connection first-cut framing; a real socket-readable event would replace this.

7. **`__cruftless_http_server_id` stored as `Value::Number` on the JS object.** f64 down-cast to usize loses precision past 2^53. Irrelevant in practice but the codebase convention for engine-internal IDs is typically an internal slot or registry-backed Rc, not a stored Number on the JS receiver. Tied to concern (1).

### Recommended next rungs

- **HS-EXT 5a (substrate hygiene)** — close concerns (1) + (2) + (3). ~30 LOC, mechanical. Same shape as EIPD-EXT 1 / GBNE-EXT 1 / RPTC-EXT 4.
- **HS-EXT 6** as planned (authority hardening + capability-backed loopback allow path).
- **HS-EXT 7a (EventEmitter on `server`)** before HS-EXT 8 (Express probe). Without it, the Express target is blocked even if the rest of the substrate is right.

### Standing notes

- The trajectory's HS-EXT 5 entry is honest about test gates (focused `caps::tests::net` lane passes; broad `cargo test` trips pre-existing `module_golden.rs` compile errors). Future agents should not chase the broad-test failure as an HS regression.
- This locale's discipline of multiple apparatus-tier reformalization rounds before code (HS-EXT 1–4) should be preserved. It is the right pattern for substrate composition locales where the architectural seam matters more than the LOC count.

### Supersession note

- HS-EXT 5a closes concerns (1), (2), and (3) from this review. Concern (4), EventEmitter shape for Express, remains open.
- HS-EXT 7a closes the minimal concern (4) blocker for request listeners. Broader EventEmitter parity remains intentionally bounded to later Express hardening.

---

*Append new reviews below. Keep the running summary above truthful and short.*
