# rusty-tls — Resume Vector / Seed

**Locale tag**: `L.tls` (per [Doc 737](../../../corpus-master/corpus/737-the-locale-as-coordinate-nested-seed-trajectory-pairs-as-pin-art-substrate-positions.md))

**Workstream**: the TLS 1.3 substrate of the Cruftless engine, structured per Doc 731 R1–R8 shape (one-tier, alphabet-bounded) and gated by [Doc 733](../../../corpus-master/corpus/733-fractal-seeds-and-trajectories-recurrent-resume-vector-pairs-across-substrate-depth-as-the-operating-conditions-layer-for-pin-art-at-engagement-scale.md) (the fractal-pair discipline applied to a previously pair-less pilot).
**Author**: 2026-05-21 session. Retroactively founded after PM-EXT 4 surfaced the engagement-internal-TLS CDN-incompatibility gap. The prior substrate-introduction phase (Π1.4.a through Π1.4.i) ran without an explicit pair; the workstream's existing state is reconstructed in trajectory.md's "Phase Π1.4 — prior substrate" entry.
**Parent**: cruftless engagement (`/home/jaredef/rusty-bun`).
**Composes with**:
- [Doc 733](../../../corpus-master/corpus/733-fractal-seeds-and-trajectories-recurrent-resume-vector-pairs-across-substrate-depth-as-the-operating-conditions-layer-for-pin-art-at-engagement-scale.md) (founding rationale: closes one of the three missing pairs §VII names).
- [Doc 729](../../../corpus-master/corpus/729-cruftless-a-primary-articulation-of-the-resolver-instance-pattern-as-the-comprehensive-design-toward-which-rusty-bun-morphs.md) (TLS pilot is the transport-tier resolver-instance enclosed by host-level HTTP; classification per §IV).
- [Doc 730](../../../corpus-master/corpus/730-the-vertical-recurrence-of-the-lowering-compiler-closure-as-primitive-across-substrate-tiers.md) §III–§VII (the per-Op translation-table shape Doc 731 names for the JIT specializes here to: per-cipher-suite, per-curve, per-sigalg dispatch tables; alphabet purity bounds the dispatch surface).
- `pilots/rusty-js-pm/derived/src/http.rs` — the *consumer* of this pilot whose PM-EXT 4 probe surfaced the gap that motivates this workstream.
- `pilots/http-codec/derived/` — co-tier sibling; together with this pilot they constitute the HTTPS transport stack the engagement composes on.
- `pilots/x509/derived/`, `pilots/asn1-der/derived/`, `pilots/web-crypto/derived/` — the certificate-parsing, DER, and cryptographic-primitive substrates this pilot's handshake consumes.

## I. Telos

Bring the engagement-internal TLS pilot to **CDN-tier interoperability**: end-to-end HTTPS exchange completes successfully against the standard set of CDN-hosted servers (Cloudflare, Fastly, Amazon CloudFront, GitHub Pages, npm registry, Google services), not only against legacy / single-server endpoints. The success criterion is empirical: `pm_http_get("https://registry.npmjs.org/lodash/4.17.21")` returns 200 with a parseable JSON body, and the analogous probe succeeds against ten further CDN-hosted endpoints chosen to span the major TLS-feature requirements (TLS 1.3 strict, ALPN-required, modern-cipher-required, modern-curve-required).

The success criterion is not feature-parity with rustls or BoringSSL. The success criterion is **CDN-passable**: the pilot completes handshake, sends application data, receives a complete response, and handles close_notify cleanly across the modern-CDN distribution. Specific cipher counts and curve counts are downstream of this; the work bounds itself to what passes the probe, not what passes a comprehensive TLS conformance suite.

### I.1 Bounded first-cut telos

The first cut (this workstream's near-term target) is **the five-endpoint passing probe**: `example.com`, `api.github.com`, `registry.npmjs.org`, `httpbin.org`, and one Cloudflare-fronted endpoint each return 200 with a parseable body through `pm_http_get`. The PM-EXT 4 probe of 2026-05-21 surfaced four distinct failure modes across these five; closing each mode is a TLS-EXT.

## II. Apparatus

The TLS pilot is **resolver-instance at the transport tier**, between the package-install tier (Doc 732 §II) and the TCP / OS-socket tier (out of scope). It composes upward as a substrate consumed by `pilots/rusty-js-pm/derived/src/http.rs` and (eventually) `pilots/fetch-api/derived/`; it composes downward on `pilots/x509`, `pilots/asn1-der`, and `pilots/web-crypto`.

Per Doc 730 §XII–§XVI, every substrate move at this instance is gated on the §XVI bidirectional engine-diff oracle. The reference engine for TLS work is **the standard CDN distribution itself** rather than a specific implementation: a probe that passes against curl + openssl 3 + Pi system trust-store is the spec-correct outcome; a divergence between rusty-tls and curl-against-the-same-endpoint is a substrate move target.

Two failure-localization tools the workstream will use repeatedly:

- **Wire-level capture**. `tcpdump -w` against the loopback or against the live socket; the captured ClientHello / ServerHello / Alert can be diffed against curl's output for the same endpoint. The diff is the §XVI categorization tool at this tier.
- **Endpoint-discriminating probe table**. The five-endpoint probe is the substrate-coverage matrix; each row is a (endpoint, observed-failure-mode) cell; each TLS-EXT closes one or more cells. Closure is recorded in trajectory.md as the cell flipping from FAIL to PASS.

## III. Methodology

Each substrate move proceeds in the standard Pin-Art shape: surfaced residue → §XVI categorization → minimal substrate move → probe re-run → trajectory entry. The probe is the five-endpoint set, run after every substrate move.

The candidate move-classes (in expected dispatch order, not commitment order — order is whatever the §XVI diagnosis surfaces next):

1. **Close_notify drain semantics.** When `receive_application_data` returns Err on a benign close_notify, accumulator-buffered ciphertext is discarded. The fix: distinguish the close_notify alert (warning, 0) from fatal alerts; on close_notify, drain remaining buffered records first, then return a "session ended cleanly" signal that the caller can treat as Ok-with-no-more-data. See `pilots/tls/derived/src/driver.rs::receive_application_data`.

2. **Cipher-suite alphabet expansion.** Currently `&[CIPHER_AES_128_GCM_SHA256]`. CDN-tier servers commonly require negotiation against the modern set: AES-256-GCM-SHA384 (TLS_AES_256_GCM_SHA384) and ChaCha20-Poly1305-SHA256 (TLS_CHACHA20_POLY1305_SHA256). Adding these is alphabet expansion at the bytecode tier per Doc 731 §XIV.d shape: the substrate is the AEAD-construction generic; the alphabet adds entries; the dispatch table grows; the §VI carve-out of "one cipher" lifts.

3. **Elliptic-curve group expansion.** Currently `&[GROUP_SECP256R1]`. CDN-tier servers commonly prefer X25519 (group 0x001d) per RFC 8446 + RFC 7748. Adding X25519 requires a Curve25519 implementation in `pilots/web-crypto` and a new GROUP_X25519 entry in the driver. The substrate expansion at the web-crypto tier is its own move; the driver-tier addition is the dispatch entry.

4. **ALPN extension.** Many CDNs require ALPN advertisement of `http/1.1` (or `h2`) and reject the handshake when ALPN is absent. The pilot currently sends `alpn: None`. Adding ALPN is a ClientHello-extension addition + a ServerHello-extension parser path that records the negotiated protocol.

5. **Expanded signature-algorithm set.** Currently `&[SIG_ECDSA_SECP256R1_SHA256, SIG_RSA_PKCS1_SHA256, SIG_RSA_PSS_RSAE_SHA256]`. CDN-tier servers may present certificates signed under RSA-PSS-SHA384, RSA-PSS-SHA512, ECDSA-SECP384R1-SHA384, or Ed25519. Adding these requires verifier paths in `pilots/x509`'s `verify_signature` and dispatch entries in the driver.

6. **supported_versions extension correctness.** The protocol_version alert (RFC 8446 §6) fires when the server cannot agree on TLS 1.3 with the client. Verify the `supported_versions` extension is present and well-formed in the ClientHello; some early TLS 1.3 implementations had the version negotiation in legacy_version, which CDN servers reject. (Probe-driven: only investigate if PM-EXT 4's alert-70 against npm persists after items 2–5.)

7. **GREASE values**. Per RFC 8701, modern Chromium / Firefox handshakes include GREASE values in cipher suites, groups, and extensions to detect implementations that fail to ignore unknown values. CDN servers may treat ALL-known-good as a synthetic-traffic signal and apply heavier scrutiny. Adding GREASE is low-priority but cheap.

The move-class numbering is structural; the EXT numbering will be whatever order the diagnosis dispatches. Doc 581's Pin-Art discipline is the gating mechanism.

## IV. Carve-outs and bounded scope

Per Doc 731 §VI shape:

- **No TLS 1.2 fallback.** TLS 1.3 only. Servers that do not support TLS 1.3 are out of scope. Modern CDN tiers all support TLS 1.3; legacy / self-hosted servers may not, and the failure is allowed to surface as "this endpoint requires TLS 1.2."
- **No client certificate auth.** Server-cert validation only.
- **No session resumption / 0-RTT.** Full handshake every connection. PM uses `Connection: close` per request anyway.
- **No SSL renegotiation.** Out of scope (TLS 1.3 does not support it; mentioned for completeness against TLS 1.2 reflexes).
- **No PSK / pre-shared key.** Only certificate-based auth.
- **No OCSP stapling.** Cert validity is checked against `notBefore` / `notAfter` only; revocation is deferred.
- **No HTTP/2 over TLS.** ALPN may advertise h2 but the negotiated protocol's transport is HTTP/1.1 only in the pm_http_get path. fetch-api may later need HTTP/2 transport; that is a separate workstream.
- **No middlebox traversal beyond the existing 32-byte legacy_session_id padding** (already landed in Π1.4.i; mentioned as covered).

These carve-outs are spec-aligned: each is a region where the complexity-vs-yield ratio is unfavorable for the CDN-passable first cut.

## V. Standing artefacts

- `pilots/tls/derived/Cargo.toml`, `src/{lib,driver,handshake,record,client,store}.rs` — the existing crate, unchanged in shape; modified per substrate moves.
- `pilots/tls/probes/endpoint-coverage.md` — the five-endpoint probe matrix (to be created in TLS-EXT 0+). One row per (endpoint, expected-feature-set, observed-status, last-probed-commit).
- `pilots/tls/probes/wire-captures/` — `tcpdump` captures of failing handshakes, kept for diff against curl-against-the-same. (Gitignored beyond a small set of canonical examples.)
- `trajectory.md` — time-ordered record of substrate moves and probe results.

## VI. Resume protocol

Read Doc 733 (the fractal-pair rationale for founding this pair), Doc 729 §IV–§V (the resolver-instance frame), Doc 731 §VI (the carve-out shape this workstream mirrors), this seed, then trajectory.md. The seed.md / trajectory.md pair was retroactively founded after substantial substrate work (Π1.4.a through Π1.4.i, ~9 prior substrate moves) had already landed; the trajectory's "Phase Π1.4 — prior substrate" entry reconstructs that state in summary form. Subsequent rounds (TLS-EXT 1 onward) carry full per-round entries per Doc 581 shape.

First substrate move (when implementation begins): produce `pilots/tls/probes/endpoint-coverage.md` with the five-endpoint probe matrix and the four failure modes PM-EXT 4 surfaced. Equivalent to JIT-EXT 1's P4 enumeration and PM-EXT 1's manifest field-coverage: bound the failure surface before any code lands.

Engine state at this workstream's founding (TLS-EXT 0 / 2026-05-21):
- TLS 1.3 handshake completes against simple TLS servers (per Π1.4.h–i live-handshake slow test).
- Handshake FAILS against npm registry with alert [2, 70] (protocol_version, fatal).
- Application-data exchange fails against example.com / google.com / api.github.com (TCP EOF before response — root cause undiagnosed, possible ALPN-required, possible close_notify drain race).
- Handshake fails against httpbin.org with alert [1, 0] (close_notify, warning) mid-handshake.
- One cipher suite, one curve, three sigalgs, no ALPN. Modern-CDN baseline not met.

Pin-Art tag prefix for this workstream: `Ω.5.P06.E1.tls-*` for handshake-tier moves (P06 = Host pipeline per dag-coordinates.json; E1 = Algorithmic step — the layer at which TLS handshake steps live); `Ω.5.P06.E2.tls-*` for record-layer / driver-internal moves (E2 = Internal method); `Ω.5.P06.E3.tls-*` for cryptographic-primitive additions (E3 = Intrinsic object — covers cipher-suite and curve additions). Per host/tools/tag-grammar.md §1, `<handle>` is `tls-close-notify`, `tls-cipher-aes256gcm`, `tls-curve-x25519`, `tls-alpn-http11`, etc.
