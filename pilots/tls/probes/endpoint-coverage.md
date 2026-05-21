# TLS Endpoint-Coverage Matrix — CDN-Passable Baseline

**Tag**: `Ω.5.P06.E1.tls-endpoint-coverage` (TLS-EXT 1)
**Date**: 2026-05-21
**Companion**: [seed.md](../seed.md) §V; [trajectory.md](../trajectory.md) TLS-EXT 0
**Probe runner**: `cargo run -p rusty-js-pm --release --example tls_probe`

The endpoint-coverage matrix is the substrate-coverage instrument for the TLS workstream's CDN-passable telos (seed §I). One row per probed endpoint; one column-group per substrate-feature class the endpoint is known or suspected to require; one column for last-probed status. A cell flips from FAIL to PASS as substrate moves land. Closure of the workstream's first-cut telos = every row PASSing.

The five-endpoint set is chosen to span the major TLS-feature requirements the modern HTTPS distribution exposes. Each endpoint is documented with the public observation of what its CDN is known to require (collected by running `curl -v` and `openssl s_client -connect` against each, plus the TLS-version + cipher reports from public services like ssllabs.com).

## §1. The probe set

| # | endpoint | hosted on | known-required TLS features |
|---|---|---|---|
| E1 | `https://example.com/` | AWS CloudFront | TLS 1.2 or 1.3; ALPN advertised (h2,http/1.1); SNI required |
| E2 | `https://httpbin.org/get` | Heroku (variable) | TLS 1.2 or 1.3; lenient on cipher; close_notify on response |
| E3 | `https://www.google.com/` | Google Front End | TLS 1.3 preferred; X25519 preferred; ALPN required (h2,http/1.1) |
| E4 | `https://api.github.com/` | Fastly | TLS 1.3 strict; ECDSA-P-256 + X25519 + ALPN required |
| E5 | `https://registry.npmjs.org/lodash/4.17.21` | Cloudflare | TLS 1.3 strict; modern-baseline cipher set; ALPN h2/http1.1 required |

The set spans four distinct CDN providers (CloudFront, Heroku, Google Front End, Fastly, Cloudflare) and four distinct TLS-feature stress points (lenient → strict baseline progression).

## §2. Substrate-feature column groups

The columns below are the feature classes the workstream's substrate moves expand. A cell value indicates whether the engagement-internal TLS pilot currently supplies the feature.

| feature | currently supplied | seed.md §III move-class |
|---|---|---|
| TLS 1.3 1-RTT handshake | YES (Π1.4 substrate) | — |
| supported_versions extension | partial / unaudited | move-class 6 |
| Cipher: AES-128-GCM-SHA256 | YES (Π1.4) | — |
| Cipher: AES-256-GCM-SHA384 | NO | move-class 2 |
| Cipher: ChaCha20-Poly1305 | NO | move-class 2 |
| Curve: P-256 (secp256r1) | YES (Π1.4) | — |
| Curve: X25519 | NO | move-class 3 |
| Sigalg: ECDSA-P-256-SHA256 | YES (Π1.4) | — |
| Sigalg: RSA-PKCS1-SHA256 | YES (Π1.4) | — |
| Sigalg: RSA-PSS-RSAE-SHA256 | YES (Π1.4) | — |
| Sigalg: RSA-PSS-RSAE-SHA384 | NO | move-class 5 |
| Sigalg: RSA-PSS-RSAE-SHA512 | NO | move-class 5 |
| Sigalg: ECDSA-P-384-SHA384 | NO | move-class 5 |
| Sigalg: Ed25519 | NO | move-class 5 + new web-crypto |
| ALPN extension (advertise) | NO | move-class 4 |
| ALPN extension (parse server response) | NO | move-class 4 |
| close_notify graceful (warning, 0) | NO (treated as error) | move-class 1 |
| Middlebox-compat legacy_session_id | YES (Π1.4.i) | — |
| GREASE values (RFC 8701) | NO | move-class 7 |
| Session resumption / 0-RTT | NO | carved out (seed §IV) |
| OCSP stapling | NO | carved out (seed §IV) |

## §3. Matrix — current (re-run on every TLS-EXT close)

### TLS-EXT 2 (close_notify drain semantics) — 2026-05-21

Last-probed commit: (this commit).

| endpoint | observed | delta from TLS-EXT 0 |
|---|---|---|
| E1 example.com | `Codec("Truncated")` | unchanged |
| E2 httpbin.org | `Tls("CloseNotify")` | better-typed (was raw alert bytes [1,0]) |
| E3 google.com | `Codec("Truncated")` | unchanged |
| E4 api.github.com | `Codec("Truncated")` | unchanged |
| E5 registry.npmjs.org | `Tls("server alert: [2, 70]")` | unchanged (fatal alert, not close_notify) |

**Score: 0/5 PASS.** No cells flipped to PASS; the close_notify fix is structurally correct (RFC 8446 §6.1 conformant) and prepares the substrate for future cells whose body + close_notify is currently fatally misinterpreted, but the four currently-failing endpoints fail upstream of close_notify (UnexpectedEnd or fatal alert).

### TLS-EXT 0 baseline (2026-05-21, archived)

Last-probed commit: `4d7115a2` (pre-TLS-EXT-0 founding; identical to TLS-EXT 0 since the founding round committed no code).

| endpoint | observed | inferred root cause | candidate-fix EXT |
|---|---|---|---|
| E1 example.com | `Codec("Truncated")` | TCP EOF before any application data; possibly ALPN-required (CloudFront commonly rejects no-ALPN handshakes silently mid-exchange); possibly close_notify race | TLS-EXT 2 (close_notify) probably reveals; TLS-EXT 4 (ALPN) if not |
| E2 httpbin.org | `Tls("server alert: [1, 0]")` | close_notify alert (warning level, description 0). Possibly mid-handshake (server intentionally hung up) or end-of-response (benign; our driver treats it as fatal) | TLS-EXT 2 (close_notify) is the direct fix |
| E3 google.com | `Codec("Truncated")` | Google Front End requires X25519 and ALPN; handshake may complete on P-256 + AES-128 + no-ALPN but server refuses to send response over it | TLS-EXT 3 (curve X25519) + TLS-EXT 4 (ALPN); TLS-EXT 2 may also surface this |
| E4 api.github.com | `Codec("Truncated")` | Fastly: TLS 1.3 strict + ALPN required; cipher requirements stricter than Google's | TLS-EXT 4 (ALPN) + TLS-EXT 3 (X25519); same root cause as E3 |
| E5 registry.npmjs.org | `Tls("server alert: [2, 70]")` | Fatal protocol_version alert. Cloudflare reads our ClientHello and cannot agree on TLS 1.3. Most-likely: `supported_versions` extension absent or malformed; second-most-likely: cipher/curve/sigalg combination doesn't satisfy Cloudflare's TLS 1.3 baseline (which requires modern cipher + X25519 + ALPN simultaneously) | TLS-EXT 6 (supported_versions audit) is the direct probe; TLS-EXT 4 + 3 + 2 may reveal as side-effect |

**Score: 0/5 PASS.** This is the workstream's baseline; every subsequent TLS-EXT records the new score in trajectory.md.

## §4. Inferred substrate-move ordering

From the matrix, the substrate moves cluster:

**Cluster A — close_notify drain (TLS-EXT 2).** Smallest move; directly addresses E2. Likely side-effect: clarifies whether E1, E3, E4's `Codec("Truncated")` is a close_notify-race or a genuine application-data-not-arriving failure. After TLS-EXT 2, the matrix's failure modes should partition more cleanly.

**Cluster B — instrumentation (TLS-EXT 3 per current trajectory; renumber as needed).** `tcpdump` capture + curl-vs-rusty-tls diff. Surfaces what server-side ALPN-required and protocol_version rejections actually look like on the wire. No fix; produces the diagnosis for cluster C.

**Cluster C — alphabet expansion (TLS-EXT 4 onward).** Multi-EXT cluster:
  - C1: ALPN extension (advertise + parse) → likely flips E1, E3, E4
  - C2: X25519 curve → likely flips E3, E5
  - C3: AES-256-GCM-SHA384 + ChaCha20-Poly1305 ciphers → likely flips E5 (Cloudflare appears to require modern ciphers for the path we hit)
  - C4: supported_versions audit → may be the missing piece for E5
  - C5: Extended sigalgs → not required by the five endpoints based on their public cert chains, but reassuring to include

**Cluster D — GREASE.** Not required by any of the five endpoints (Cloudflare's strict-baseline doesn't enforce GREASE rejection). Lowest priority.

The likely arc: TLS-EXT 2 (close_notify) → TLS-EXT 3 (instrument) → TLS-EXT 4 (ALPN) → TLS-EXT 5 (X25519) → TLS-EXT 6 (modern ciphers) → TLS-EXT 7 (supported_versions audit if E5 still fails) → 5/5 PASS at roughly TLS-EXT 6–8.

The arc is a forecast, not a commitment. The §XVI oracle (curl + openssl s_client) gates each move; if the diagnosis surfaces a different failure mode than predicted, the order changes accordingly.

## §5. Per-cell verification protocol

When a TLS-EXT lands, re-run the probe and update §3 in place:

1. `cargo run -p rusty-js-pm --release --example tls_probe`
2. For each row: if `observed` changed, update the cell. If the row now PASSes, replace the failure mode with `PASS (<sha-of-body-prefix>)` and record the commit hash in the row's last-probed column.
3. Update the score line at the bottom of §3.
4. Record the round's result in `trajectory.md` as part of the TLS-EXT N closing entry.

A row flipping from PASS back to FAIL is a regression signal that the round triggered an §XVI Case-3 implementation-freedom move that broke an earlier endpoint. The trajectory entry then opens a sub-EXT to absorb the regression before the next forward move.

## §6. Open questions

1. **Should the probe set grow?** Five endpoints span the major CDNs; a sixth (a self-hosted nginx with deliberately-modern TLS-1.3-only config) would give us a curl-controllable reference endpoint for diff-against-curl work in cluster B. Defer until cluster B needs it.

2. **Wire-capture storage.** `pilots/tls/probes/wire-captures/` is mentioned in seed §V; first cut keeps captures local-only (gitignored) until a canonical example justifies committing. Decision: gitignore the directory in TLS-EXT 3; commit specific examples only when they document a load-bearing diagnosis.

3. **Probe runner location.** Currently the probe lives in `pilots/rusty-js-pm/derived/examples/tls_probe.rs`. As the TLS workstream advances, it may want its own example or test. Defer; the cross-pilot location is fine while PM is the primary consumer.

## §7. Closes

TLS-EXT 1 closes with the matrix established, the four observed failure modes structured into the §3 table, the candidate-fix EXTs identified per cell, and the substrate-move ordering forecast in §4. The next substrate move is TLS-EXT 2 (close_notify drain semantics) per §4 cluster A: smallest in-code move, directly addresses E2, likely clarifies E1/E3/E4 partition.
