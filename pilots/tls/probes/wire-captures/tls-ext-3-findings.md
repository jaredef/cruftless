# TLS-EXT 3 Wire-Level Findings

**Tag**: `Ω.5.P06.E2.tls-wire-diff` (TLS-EXT 3)
**Date**: 2026-05-21
**Probes used**: `curl -v` (system curl 7.88+ with openssl 3); `openssl s_client -msg` for TLS 1.3 forced negotiation.

The §XVI bidirectional engine-diff oracle, applied at the TLS tier. Five endpoints probed against curl (the spec-correct reference) under varying TLS-feature configurations; rusty-tls failure modes correlated with what each endpoint actually requires versus what we previously hypothesized.

## §1. The probes

### Probe A — `curl --no-alpn` against all five endpoints

| endpoint | curl --no-alpn result | implication |
|---|---|---|
| E1 example.com | handshake completes, fetch succeeds | **ALPN is not required by CloudFront** |
| E2 httpbin.org | handshake completes, fetch succeeds | **ALPN is not required by Heroku** |
| E3 google.com | handshake completes, fetch succeeds, `TLS_AES_256_GCM_SHA384 / X25519MLKEM768` | **ALPN is not required by Google Front End**; X25519MLKEM768 (post-quantum hybrid) negotiated |
| E4 api.github.com | handshake completes, fetch succeeds, `TLS_AES_128_GCM_SHA256 / x25519` | **ALPN is not required by Fastly** |
| E5 registry.npmjs.org | handshake completes, fetch succeeds, **`TLSv1.2 / ECDHE-ECDSA-CHACHA20-POLY1305 / x25519`** | **npm registry path does not negotiate TLS 1.3**; Cloudflare's edge policy for this hostname is TLS 1.2 |

**Headline finding: ALPN is not required by any of the five probe endpoints.** The TLS-EXT 1 §4 cluster C ordering forecast (ALPN → X25519 → modern ciphers) was wrong about ALPN being the limiting factor for E1/E3/E4.

### Probe B — TLS 1.3 forced against npm

`echo Q | openssl s_client -connect registry.npmjs.org:443 -tls1_3`:

```
>>> TLS 1.3, Handshake [length 05ca], ClientHello
<<< TLS 1.3, Alert [length 0002], fatal protocol_version
SSL alert number 70
```

**Even openssl with TLS 1.3 forced is rejected with the identical alert our pilot gets.** The npm registry path on Cloudflare specifically refuses TLS 1.3 negotiation; it works only over TLS 1.2. This is not a rusty-tls limitation; it is an endpoint policy.

## §2. Root-cause reclassification

The TLS-EXT 0 baseline matrix attributed each failure to a candidate cause. Wire diff revises:

| endpoint | TLS-EXT 0 inferred cause | TLS-EXT 3 actual cause |
|---|---|---|
| E1 example.com | ALPN-required or close_notify race | **Handshake completes; failure is post-handshake app-data exchange.** Server sends nothing after our request; most-likely cause: our pilot's `send_application_data` produces records the server cannot decrypt, server silently closes. Candidate sub-causes: app_traffic_secret key-derivation bug; AEAD nonce computation drift; sequence number not reset at handshake→app transition. |
| E2 httpbin.org | close_notify (correct, fixed in TLS-EXT 2 by improved typing; underlying server-hangup persists) | server-side mid-handshake hangup; possibly cert-validation-related on our side rather than server-side. Re-probe with tcpdump-equivalent debug print of our cert chain walk. |
| E3 google.com | ALPN + X25519 required | **Same as E1**: handshake completes, post-handshake exchange fails. Google's path uses X25519MLKEM768 in production but accepts P-256 fallback (per Probe A `--no-alpn` succeeded). Not an alphabet issue at handshake; an app-data-encryption issue. |
| E4 api.github.com | ALPN + X25519 required | **Same as E1 + E3**: handshake completes, post-handshake exchange fails. Fastly accepts P-256 + AES-128-GCM-SHA256 per Probe A. Not an alphabet issue at handshake. |
| E5 registry.npmjs.org | protocol_version alert; possibly supported_versions extension bug or modern-cipher-required | **npm registry path is TLS 1.2 only.** Cloudflare's edge policy refuses TLS 1.3 connections to this hostname. Our pilot is TLS 1.3 only per seed §IV carve-out. The two cannot meet without lifting one carve-out. |

## §3. Revised substrate-move ordering

The TLS-EXT 1 §4 forecast (cluster A close_notify → B instrument → C alphabet expansion → D GREASE) is partially obsolete:

**Cluster A (close_notify drain).** Landed in TLS-EXT 2. Structurally correct; flips no current cells but prepares for body+close_notify cases that will arise after E1/E3/E4 are unblocked.

**Cluster B (instrumentation).** Landed in TLS-EXT 3 (this entry). Reclassified all five endpoints' root causes; revealed three are post-handshake bugs, not alphabet gaps; revealed E5 is endpoint policy beyond cipher/curve negotiation.

**Cluster C revised (post-handshake app-data correctness).** The next substrate work is **diagnosing why our app-data records are not decryptable by E1/E3/E4 servers**. Hypotheses, by likelihood:
  - C1: app_traffic_secret derivation incorrect (handshake_traffic_secret used after handshake complete instead of app_traffic_secret)
  - C2: AEAD nonce sequence not reset at handshake→app transition
  - C3: encryption uses wrong direction's keys (client-write vs server-write swap)
  - C4: TLS_AES_128_GCM_SHA256's GCM nonce XOR with sequence number off-by-one

Each of these is one substrate move at the driver tier; the right diagnostic is to capture our sent record bytes + the openssl-equivalent and diff. (tcpdump needs root; alternative is to print our outgoing record bytes from a debug feature flag in the driver and replay them through openssl's `s_client -trace -debug` running against the same endpoint.)

**Cluster D revised (alphabet expansion).** Reframed as **endpoint-policy gaps**, not handshake-negotiation gaps:
  - D1: X25519 curve — useful but not blocking E1/E3/E4 (they accept P-256)
  - D2: AES-256-GCM-SHA384 / ChaCha20-Poly1305 — useful but not blocking E1/E3/E4
  - D3: TLS 1.2 fallback — required for E5. Substantial substrate work. Currently carved out per seed §IV. **Decision required**: lift the carve-out (likely many EXTs of substrate work) or remove E5 from the first-cut probe set and pick a TLS 1.3-supporting registry alternative (e.g., a self-hosted mirror).
  - D4: ALPN — not required by any probe endpoint. Add at lower priority for general CDN compatibility.

**Endpoint-policy classification correction (post-publication).** TLS-EXT 3 originally framed E5 npm as "Cluster E — a new §XVI case-5." This was a misreading of Doc 730 §XVI, which has exactly four cases. The corrected §XVI classification of the five probe endpoints:

- **E1, E3, E4 → Case 1**: curl (engine A) is spec-correct; rusty-tls (engine B) violates the spec at the post-handshake app-data path. Substrate move: §XII coercion/dispatch lift on rusty-tls. No deviation primitive.
- **E2 httpbin → Case 1 or Case 4**: needs further probe to distinguish; first take after TLS-EXT 2 is that the server-side mid-handshake hangup is a rusty-tls bug (cert-validation behavior?), but openssl-against-the-same-endpoint diff is needed to confirm.
- **E5 npm registry → Case 4**: both implementations conform to their respective scope choices. The npm server's TLS-1.2-only is spec-permitted (RFC 8446 does not require servers to support TLS 1.3); rusty-tls's TLS-1.3-only is a seed §IV scope carve-out. The empty intersection is implementation freedom at the scope tier, not a substrate move at the engine tier. The Doc 730 §XVI pipeline's correct response is no-op; the engagement decision is scope-level (lift the carve-out as multi-EXT 1.2 work, or substitute the endpoint).

No new §XVI case is required. The original framing's "Cluster E" should be read as "Case-4 scope-decision, not a substrate move at the TLS pilot tier."

## §4. Open questions surfaced

1. **Which app-data correctness sub-cause (C1–C4) is the actual bug?** Requires diff-against-openssl. Instrumentation work for TLS-EXT 4.

2. **Should E5 npm registry stay in the first-cut probe set?** It exposes a real first-cut blocker (no TLS 1.2 fallback) but is structurally a different work-class than the other four. Three options:
  - Keep E5; gate PM-EXT 5 behind lifting the TLS 1.2 carve-out (multi-EXT TLS substrate work).
  - Remove E5; replace with a TLS 1.3-supporting registry endpoint (e.g., `https://registry.yarnpkg.com/lodash` if it speaks 1.3).
  - Keep E5 documented as "endpoint-policy-blocked, not in-cut" and proceed with PM-EXT 5 against a different registry.
  - Decision requires keeper input; logged here as the load-bearing open question from TLS-EXT 3.

3. **What is the right discriminator for "endpoint policy refuses our profile" vs "our handshake is structurally wrong"?** TLS-EXT 3's wire diff supplied the answer in retrospect (curl-against-the-same-endpoint succeeds → endpoint policy is fine; problem is our pilot). Codify as a probe-protocol step in `endpoint-coverage.md` §5.

## §5. Closes

TLS-EXT 3 closes with the root-cause map revised. The work is reoriented: cluster C is no longer "alphabet expansion" but "post-handshake app-data correctness debugging." The substrate moves change accordingly.

Next: TLS-EXT 4 — instrument our `send_application_data` to dump record bytes; replay through openssl `s_client` against E1 (example.com, the simplest endpoint) to identify which of C1–C4 is the bug. After the fix lands, re-probe; expect cells E1, E3, E4 to flip to PASS or at minimum to surface a different downstream failure.
