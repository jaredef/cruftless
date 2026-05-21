# Bidirectional Pin-Art Probe Design

**Tag**: `Ω.5.P06.E1.tls-pin-art-design` (TLS-EXT 4 design artifact)
**Date**: 2026-05-21
**Composes with**: [Doc 619 — Pin-Art Canonical Formalization](../../../../corpus-master/corpus/619-pin-art-canonical-formalization.md); [Doc 691 — Polytopal Feature and the Pin-Art Bidirection](../../../../corpus-master/corpus/691-the-polytopal-feature-and-the-pin-art-bidirection.md); [Doc 730 §XVI](../../../../corpus-master/corpus/730-the-vertical-recurrence-of-the-lowering-compiler-closure-as-primitive-across-substrate-tiers.md) (bidirectional engine-diff as the deviation-pipeline's empirical instrument).

## §1. The situation

The TLS-EXT 4 localhost-openssl probe falsified the TLS-EXT 3 Case-1 reclassification for E1/E3/E4. Rusty-tls completes handshake, sends application data, receives the full HTTP response, and handles close_notify cleanly against `openssl s_server -tls1_3 -www`. The pilot's TLS is structurally correct against a spec-conformant reference. Yet the same pilot fails against five real CDN-hosted endpoints (E1/E3/E4 with TCP EOF before any response; E2 with close_notify mid-handshake; E5 with fatal protocol_version alert).

The reclassification: **Case 3 (both conform to spec but diverge on some shared discrimination)** for E1/E3/E4. The discrimination is most-likely post-handshake message handling — CDN servers likely emit some post-handshake record sequence that openssl s_server does not, and our pilot mishandles that sequence in a way that closes the connection silently.

The instrument the situation requires is Pin-Art's bidirectional probe apparatus (Doc 619 + Doc 691): a population of independent local probes whose joint reading surfaces what the asymmetry-of-failure-modes actually pins.

## §2. Probe assignments

Six probes; three in the **detection direction** (substrate → probes: passive observation of what each system actually does), three in the **composition direction** (probes → substrate: deliberate constraint applied to surface where the pilot breaks).

### Detection direction (D)

- **D1 — `openssl s_client -msg -trace` against CDN**. Run `openssl s_client -connect example.com:443 -tls1_3 -msg -trace -servername example.com 2>&1 < /dev/null | grep -E "TLS record|Sent|Received|Inner Content Type|Alert"`. Captures the post-handshake record sequence the CDN sends to a reference TLS-1.3 client. Per-endpoint variant for E3 (google.com), E4 (api.github.com), E5 (registry.npmjs.org).
- **D2 — `openssl s_client -msg -trace` against local openssl s_server**. Same as D1 but against the localhost reference. Establishes the post-handshake-record-sequence *control* against which CDN sequences are read for divergence.
- **D3 — `tcpdump`-equivalent via openssl `-debug` flag**. The `-debug` flag prints raw wire-level send/receive bytes. Run against CDN and against local openssl s_server to capture timing + raw byte stream signatures. (Substitute for actual tcpdump since the engagement runs without root.)

### Composition direction (C)

- **C1 — pilot post-handshake spontaneous-read**. Modify the localhost_tls_probe example to do `receive_application_data` *before* sending any request, with a short timeout. Observe what records (if any) arrive spontaneously from the server post-handshake. Run against (a) openssl s_server, (b) example.com. The asymmetry between (a) (probably nothing) and (b) (possibly NewSessionTicket, KeyUpdate, or HTTP-2 settings if CDN attempts HTTP/2 upgrade) pins what the CDN sends before our pilot sends its request.
- **C2 — pilot record-type-logging instrumentation**. Modify `pilots/tls/derived/src/driver.rs::receive_application_data` to log every record type seen and every inner_ct after AEAD decrypt, before returning Ok/Err. Run against the failing CDN endpoints. The log captures whether the pilot is (i) reading zero records before TCP EOF, (ii) reading records but mis-classifying inner_ct, (iii) reading records but throwing on something deeper in decrypt or codec.
- **C3 — open**. Synthetic-injection middlebox proxy that intercepts handshake completion and injects extra records between server Finished and the pilot's first read. Substantial work; deferred to a later EXT.

## §3. Joint-pattern reading

Detection outputs a discriminant: *"The CDN sends [record sequence X] post-handshake; openssl s_server sends [record sequence Y]; their difference is Z."* Composition outputs a complementary discriminant: *"The pilot crashes/EOFs at [substrate location L] when receiving [record sequence Z]."* The pair-reading names the substrate move precisely: which record class to handle differently, and where in the pilot's state machine the handling needs to land.

## §4. Falsifiers (per Doc 691 §V analog)

- **F1**: D1 = D2 (no post-handshake-record difference between CDN and openssl s_server). Redirect: the discrimination is not post-handshake records; investigate protocol-version handling, cipher negotiation, or TLS extension differences.
- **F2**: C2 logs the pilot reading zero records before TCP EOF on the failing endpoint. Redirect: the discrimination is below the TLS layer — TCP-level close from the CDN before any post-handshake TLS record is sent. Investigation moves to "what about our outbound app-data record causes the CDN to RST?" — possibly cipher mismatch on the application secret, possibly content-type misalignment, possibly something the CDN's TLS terminator rejects on first-byte inspection.
- **F3**: D1 reveals a record-type the pilot's code currently `continue`s past (e.g., NewSessionTicket) but C2 reveals the pilot is dying *during decrypt* of that record. Redirect: the decryption itself is fine; the post-handshake state-machine update (e.g., session-ticket-driven key update) is what we lack.
- **F4**: openssl s_client also fails identically against the same CDN endpoint. Redirect: the asymmetry isn't "CDN vs openssl" — it's CDN-policy refusing all TLS clients matching some signature. Investigate ClientHello content (extensions, GREASE, JA3 fingerprint) under the same Pin-Art apparatus.

## §5. Execution order

D1 and D2 are zero-cost openssl invocations; execute first. D3 in parallel.
C2 is a small driver instrumentation; execute second.
C1 is a small probe-example modification; execute third.
C3 deferred.

The probe set is sized to land in one EXT (TLS-EXT 4 proper). The joint pattern after running D1/D2/D3 + C1/C2 should pin the discrimination; the substrate move that follows is TLS-EXT 5.
