# rusty-tls — Trajectory

Chronological resume anchors for the TLS workstream. Reads seed.md first; this file is the time-ordered record of substrate moves and their probe results.

The pair was retroactively founded at TLS-EXT 0 after substantial prior substrate work (Π1.4.a through Π1.4.i, completed before the Pin-Art fractal-pair discipline of Doc 733 was articulated). The "Phase Π1.4 — prior substrate" entry summarizes that work in one block; subsequent TLS-EXT rounds carry full per-round entries per Doc 581 shape.

## Phase Π1.4 — Prior substrate (2025 / early 2026, pre-pair)

Nine substrate moves landed before the pair existed. Reconstructed from git history:

| commit | tag | recognition |
|---|---|---|
| `fac8fd91` | `Π1.4.a` | ASN.1/DER reader pilot — substrate for cert parsing |
| `327dfc5b` | `Π1.4.b` | X.509 v3 cert parsing + signature verification |
| `c6a67c18` | `Π1.4.c` | TLS record layer + system trust store + chain walk |
| `c824ada1` | `Π1.4.d` | TLS 1.3 handshake framing + key schedule + AEAD record wrap |
| `25a267e2` | `Π1.4.e` | ClientHello/ServerHello + extensions |
| `efd353ac` | `Π1.4.f.a` | ECDH ephemeral keygen + handshake driver skeleton + Certificate-message parser |
| `3f2ae9a2` | `Π1.4.f.b` | Full TLS 1.3 handshake state machine + CertificateVerify dispatcher |
| `cd683ef2` | `Π1.4.g` | TcpTlsTransport + tls_connect Rust adapter + live-handshake slow test |
| `8d02c5f8` | `Π1.4.i` | Middlebox-compat legacy_session_id (live-handshake bug fix) |
| `72276baf` | `Pi2.6.c.d` | TLS pump migrated to mio tryRead |

State at Phase Π1.4 close:
- TLS 1.3 handshake completes against simple TLS servers (the Π1.4.h–i live-handshake slow test).
- One cipher suite (AES-128-GCM-SHA256), one curve (P-256), three sigalgs (ECDSA-P256-SHA256, RSA-PKCS1-SHA256, RSA-PSS-RSAE-SHA256).
- No ALPN extension. No supported_versions audit. No close_notify graceful handling.
- Pair did not exist; substrate moves landed against the engagement-root trajectory only.

---

## TLS-EXT 0 — 2026-05-21 (workstream founding)

### Headline

Pair retroactively founded per Doc 733. PM-EXT 4 (commit 4d7115a2) surfaced the engagement-internal-TLS CDN-incompatibility gap as a load-bearing finding: cruftless cannot reach the npm registry through its own substrate. The diagnostic spans four distinct failure modes across five probed endpoints, none of which the Phase Π1.4 substrate had tested against. The pair captures the existing state, names the workstream's telos (CDN-tier interoperability), bounds the carve-outs (TLS 1.3 only, no session resumption, no client certs, no HTTP/2 transport), and produces the resume protocol.

### Commits

| commit | tag | recognition |
|---|---|---|
| (this commit) | (workstream founding) | `pilots/tls/seed.md` + `trajectory.md` written. Doc 733 fractal-pair rationale. Pin-Art tag prefix `Ω.5.P06.E1.tls-*` (handshake-tier), `Ω.5.P06.E2.tls-*` (driver-internal), `Ω.5.P06.E3.tls-*` (crypto-primitive additions). |

### Endpoint-coverage probe (frozen at TLS-EXT 0; baseline for TLS-EXT 1+)

Run via `pilots/rusty-js-pm/derived/examples/tls_probe.rs` on 2026-05-21:

| endpoint | observed | inferred root cause |
|---|---|---|
| `https://example.com/` | `Codec("Truncated")` | TCP EOF before response; possibly ALPN-required, possibly post-handshake exchange failure |
| `https://httpbin.org/get` | `Tls(SignatureFail("server alert: [1, 0]"))` | close_notify alert during or at end of handshake (warning level); driver returns Err |
| `https://www.google.com/` | `Codec("Truncated")` | same as example.com |
| `https://api.github.com/` | `Codec("Truncated")` | same as example.com |
| `https://registry.npmjs.org/lodash/4.17.21` | `Tls(SignatureFail("server alert: [2, 70]"))` | fatal protocol_version alert; CDN cannot agree on TLS 1.3 with our ClientHello as composed |

0/5 endpoints pass. This is the **engagement-internal-TLS CDN-passable baseline = 0** at TLS-EXT 0.

### Substrate at TLS-EXT 0 close

- Pair exists; existing Phase Π1.4 substrate is reconstructed into one trajectory block.
- Endpoint coverage probe is baselined and reproducible via `cargo run -p rusty-js-pm --release --example tls_probe`.
- Probe matrix is documented but the canonical artefact `pilots/tls/probes/endpoint-coverage.md` is not yet produced (queued as TLS-EXT 1).
- No code change in this round.

### Conjecture status

The PM-EXT 4 finding stands as the workstream-justifying observation: without TLS work, the package manager cannot reach the npm registry through engagement-internal substrate. The corpus articulation Doc 733 makes the lack-of-pair itself a structural finding: the TLS pilot was below Doc 733 §V's level-local operability threshold, which is why the gap surfaced as opaque (alert numbers and one-line failures) rather than as a structured workstream.

### Open scope at TLS-EXT 0 boundary

1. **First substrate move (TLS-EXT 1)**: produce `pilots/tls/probes/endpoint-coverage.md` per seed §V. Lift the four failure modes above into a structured matrix with one row per endpoint. Each cell tracks (last-probed-commit, status, inferred-cause, candidate-fix). This is reading + classifying; no code.

2. **TLS-EXT 2 (smallest substrate move)**: fix `receive_application_data`'s close_notify handling. Per seed §III move-class 1: distinguish close_notify (alert 1, 0) from fatal alerts; drain accumulator-buffered records before returning the close-clean signal. Probe: httpbin.org should flip from FAIL to PASS (or surface a different downstream failure that we can then address).

3. **TLS-EXT 3 (medium substrate move)**: instrument the application-data exchange path. The Codec("Truncated") on three of five endpoints is currently a black box; the root cause might be in our TLS driver, in our HTTP codec, in the server expecting ALPN, or in some combination. Add a `tcpdump` capture instruction to the probe and compare with `curl -v` output for the same endpoint. The structured diff is what surfaces the next substrate move's target.

4. **TLS-EXT 4 (largest move-cluster)**: alphabet expansion per seed §III move-classes 2, 3, 4, 5. Add X25519 + AES-256-GCM-SHA384 + ChaCha20-Poly1305 + ALPN + RSA-PSS-SHA384/512 + Ed25519. This is multi-EXT work that pulls in substrate moves at `pilots/web-crypto` (Curve25519 if not already present, Ed25519 sign-verify if not present) and `pilots/x509` (Ed25519 signature verification path). Each addition is its own EXT.

5. **TLS-EXT N (closure)**: 5/5 endpoints passing the probe. PM-EXT 5 can then proceed (PM-R1 specifier resolver against real npm registry).

### Resume protocol

Read seed.md, then this trajectory's Phase Π1.4 entry and TLS-EXT 0 entry. The next substrate move is the endpoint-coverage matrix; no code needed for that move. After it lands, TLS-EXT 2 (close_notify fix) is the smallest in-code move and the most likely to flip at least one probe-cell from FAIL to PASS.

Per Doc 733 §V, the workstream's founding moves the rusty-bun engagement from **fractal coverage 4/6** (root + IR + JIT + PM) toward **fractal coverage 5/6** (the TLS pilot now carries the level-local operability property the previous state lacked). This is observable against the §V threshold predictions: the TLS workstream becomes locally resumable from the pair, and the diagnosis Pred-733.1 names becomes empirically supported.

Pin-Art tag count: 0 substrate moves under the new prefix so far (workstream founding only; the prior Phase Π1.4 substrate carries its original Π1.4.* tags).

---

## TLS-EXT 2 — 2026-05-21 (close_notify drain semantics)

### Headline

Per seed §III move-class 1: distinguish close_notify (alert 1, 0) from fatal alerts; route through a uniform classifier at all three alert-receive sites (handshake-phase plaintext + two post-handshake encrypted variants in `receive_application_data`). Adds `TlsError::CloseNotify` variant + `record::classify_alert(bytes) -> TlsError` helper. PM's `pm_http_get` drain loop now treats CloseNotify as benign end-of-stream and parses the accumulated body, rather than silently breaking on opaque Err.

### Commits

| commit | tag | recognition |
|---|---|---|
| (this commit) | `Ω.5.P06.E2.tls-close-notify-graceful` | uniform alert classifier; CloseNotify variant; three driver-site updates; PM drain-loop typed-handling |

### Probe result

Score: **0/5 PASS** (unchanged). E2 httpbin's error type improved from raw `SignatureFail("server alert: [1, 0]")` to typed `CloseNotify`, but the underlying failure (server hung up mid-handshake) was not addressed by this move (it is upstream of close_notify graceful handling). E1/E3/E4 fail at UnexpectedEnd (no alert in play) and E5 fails at fatal alert — both are §4 cluster C (alphabet expansion + ALPN) work, not cluster A.

The move is structurally correct per RFC 8446 §6.1; its empirical payoff appears once E1/E3/E4 are unblocked and servers begin sending body + close_notify (the currently-fatal case in the old code path) on the response.

### Substrate at TLS-EXT 2 close

- `TlsError::CloseNotify` variant exists and is distinct from `SignatureFail`.
- `record::classify_alert` is the single classifier all three driver sites use.
- PM's `pm_http_get` distinguishes CloseNotify / UnexpectedEnd (both → break to parse) from other Tls errors (→ propagate).
- Probe score unchanged at 0/5; matrix updated in `probes/endpoint-coverage.md`.

### Open scope at TLS-EXT 2 boundary

The probe-flipping work moves to cluster B (instrumentation) and cluster C (alphabet expansion). TLS-EXT 3: tcpdump capture of E1/E3/E4 + curl-vs-rusty-tls diff to confirm whether the root cause is ALPN-required (the most likely hypothesis), missing modern cipher, or a different post-handshake-record-handling bug.

---

*TLS-EXT 2 closes with the close_notify substrate in place and the probe unchanged. Subsequent rounds proceed to instrumentation (cluster B) and alphabet expansion (cluster C).*

---

## TLS-EXT 3 — 2026-05-21 (wire-level instrumentation; root-cause map revised)

### Headline

Per seed §III move-class equivalent of "instrument and diff against curl." Probed all five endpoints with `curl --no-alpn` and `openssl s_client -tls1_3 -msg`. **Three findings reorient the workstream:**

1. **ALPN is not required by any of the five endpoints.** curl with `--no-alpn` completes the fetch against E1–E5. The TLS-EXT 1 ordering forecast (cluster C: ALPN as the limiting factor for E1/E3/E4) was wrong.
2. **E1, E3, E4 fail post-handshake, not at handshake.** Curl traces confirm TLS 1.3 handshake completes against example/google/github. Our pilot's failure is in the application-data exchange that follows: send our request, server sends nothing back (TCP EOF). Most-likely cause: app-data record encryption bug — our records are produced under wrong keys, server cannot decrypt, server silently closes.
3. **E5 npm registry path does not support TLS 1.3.** Even openssl with `-tls1_3` forced is rejected with the identical `protocol_version` alert. Cloudflare's per-hostname edge policy is TLS 1.2 only for this endpoint. Our pilot is TLS 1.3 only per seed §IV carve-out. Reaching E5 requires lifting a carve-out or substituting a different probe endpoint.

The full wire-diff is at `pilots/tls/probes/wire-captures/tls-ext-3-findings.md`.

### Commits

| commit | tag | recognition |
|---|---|---|
| (this commit) | `Ω.5.P06.E2.tls-wire-diff` | endpoint diff vs curl; root-cause map revised across all 5 endpoints; new "endpoint policy" §XVI case-5 identified; substrate-move ordering reoriented |

### Probe result

Score: **0/5 PASS** (unchanged). TLS-EXT 3 is pure instrumentation; no code change. The value is the revised diagnosis, not a cell flip.

### Substrate at TLS-EXT 3 close

- `pilots/tls/probes/wire-captures/tls-ext-3-findings.md` produced with the five-endpoint diff and revised root-cause assignments.
- Cluster B (instrumentation) closed.
- Cluster C reframed: was "alphabet expansion (ALPN, X25519, modern ciphers)"; now "post-handshake app-data correctness debugging (key derivation, nonce sequencing, direction-swap, GCM nonce XOR)."
- Cluster E (endpoint policy) initially introduced as a new §XVI case; **corrected post-publication** (per the keeper's Doc 730 §XII–§XVI re-read): Doc 730 §XVI has exactly four cases. E5 is Case 4 (implementation freedom at the scope tier), not a new case. The engagement decision for E5 is scope-level, not a substrate move at the TLS pilot tier. The corrected classification is in `wire-captures/tls-ext-3-findings.md` and `probes/endpoint-coverage.md`.

### Open scope at TLS-EXT 3 boundary

1. **TLS-EXT 4 (post-handshake app-data debugging)**: instrument `send_application_data` to dump record bytes; replay through `openssl s_client -trace` against E1 (the simplest endpoint that fails post-handshake). Identify which of the four candidate sub-causes (C1 app_traffic_secret derivation; C2 nonce sequence reset; C3 direction-swap; C4 GCM nonce XOR off-by-one) is the bug.

2. **Load-bearing keeper-decision open**: should E5 npm registry stay in the first-cut probe set? Three options recorded in wire-captures §4 question 2. Decision required because it determines whether the TLS workstream pulls in a multi-EXT TLS 1.2 substrate sub-workstream (substantial; out of seed §IV carve-out) or whether PM-EXT 5 can proceed against a TLS 1.3-supporting registry alternative.

3. **Codify probe-protocol with curl-diff step**: `endpoint-coverage.md` §5 verification protocol should include "before classifying a row as a substrate-move target, run curl against the same endpoint; if curl succeeds, our pilot's handshake or app-data path has the bug; if curl also fails, the endpoint policy is the issue."

### Conjecture status

The TLS-EXT 1 ordering forecast (5/5 PASS at TLS-EXT 6–8) is unchanged in cardinality but reoriented in content. The next 4–6 EXTs are no longer "add ALPN, X25519, modern ciphers, supported_versions audit, expanded sigalgs"; they are "debug app-data correctness (TLS-EXT 4), close E1/E3/E4 (TLS-EXT 5), make a decision on E5 (TLS-EXT 6+ or carved out)." The reorientation increases confidence that the workstream is bounded (the cluster of bugs in app-data correctness is enumerable: ~4 candidates) and reduces confidence in any single-EXT close (the TLS 1.2 alternative for E5 is substantial work or a scope change).

---

*TLS-EXT 3 closes with the workstream reoriented. The remaining cells are not alphabet gaps; they are app-data correctness bugs (E1/E3/E4) plus one endpoint-policy blocker (E5).*
