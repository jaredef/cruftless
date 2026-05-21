# TLS-EXT 4 Bidirectional Pin-Art Readout — Detection Direction Complete

**Tag**: `Ω.5.P06.E1.tls-pin-art-detection` (TLS-EXT 4, partial — detection direction)
**Date**: 2026-05-21
**Companion**: [bidirectional-pin-art-probe-design.md](../bidirectional-pin-art-probe-design.md); [Doc 691](../../../../../corpus-master/corpus/691-the-polytopal-feature-and-the-pin-art-bidirection.md).

## §1. Probes executed (detection direction)

D1 + D2 + D3 ran as openssl s_client trace captures against the CDN endpoints and against local openssl s_server. Composition-direction probes (C1, C2) deferred to TLS-EXT 5 pending direction-setting from the detection readout.

## §2. Joint pattern (D-direction)

The post-handshake record sequence is **identical** between CDN endpoints (example.com, google.com, api.github.com) and local openssl s_server:

```
[after handshake completes]
<<< Handshake (NewSessionTicket) × 2
<<< ApplicationData (the HTTP response, possibly fragmented across records)
<<< Alert (warning close_notify)
```

Specifically:
- example.com: 2× NewSessionTicket (0x00d6 bytes each)
- google.com: 2× NewSessionTicket (0x0114 bytes each)
- api.github.com: 2× NewSessionTicket (0x0039 bytes each)
- openssl s_server (local): 2× NewSessionTicket (0x00e9 bytes each)

All four send the same SHAPE of post-handshake sequence. Sizes differ but the record-type sequence is invariant.

## §3. Falsifier F1 triggered

The probe design document's §4 F1: *"D1 = D2 (no post-handshake-record difference between CDN and openssl s_server). Redirect: the discrimination is not post-handshake records; investigate protocol-version handling, cipher negotiation, or TLS extension differences."*

This is the case. The post-handshake-record-sequence hypothesis falsified by the D-direction probes alone, before C-direction probes were executed.

## §4. Redirected hypothesis space

The discrimination is not post-handshake message types. Candidates that survive:

**H1 — Transcript-hash drift.** Our pilot's app-secret derivation uses `transcript_through_finished` (raw bytes of every handshake message added to a Vec). If a CDN's handshake includes any message (e.g., a CertificateRequest, an additional extension type, a HelloRetryRequest path our code handles differently) that the handshake state machine SKIPS or processes differently than openssl, the transcript hash diverges and our client_app_keys are derived from a wrong base. Server's decrypt then fails silently; server closes TCP without alert. **High likelihood given the local-vs-CDN asymmetry.**

**H2 — Cipher-suite negotiation difference.** Openssl s_server with `-tls1_3` defaults selects TLS_AES_128_GCM_SHA256 (matches our advertised). CDNs may select a different cipher and our pilot may parse the ServerHello but compute keys with the wrong cipher's hash/key/iv length. *Lower likelihood — ServerHello parsing would error visibly if the cipher were unrecognized.*

**H3 — ServerHello extension we silently ignore that mandates client action.** E.g., `early_data` indicator, `pre_shared_key` ack, `signed_certificate_timestamp` (CT extension). Our pilot might pass these through without action where action is required. *Medium likelihood.*

**H4 — Cert chain depth or unfamiliar intermediate.** Our `chain_walk` walks up to 8 levels. CDN cert chains can be 3-4 deep with intermediates rooted in CAs unfamiliar to our trust store. If the chain validates but with subtle state corruption affecting downstream secret derivation, the keys diverge. *Lower likelihood — secret derivation does not consume chain-walk state.*

## §5. TLS-EXT 5 plan (post-redirect)

The next probe set targets H1 + H2 + H3 in priority order:

**C-NEW-1 (was C2): record-content logging in receive_application_data.** Log every record received with its outer header, encrypted-fragment length, and (if decrypt succeeds) inner content type + plaintext length. Run against (a) openssl s_server (control), (b) example.com. The first record decryption that fails reveals whether decrypt itself fails (H1: wrong app keys) or the record is processed but unexpected (H3: extension or message type we silently ignored).

**C-NEW-2: transcript-hash logging at tls_connect close.** Dump `transcript_through_finished.len()`, `hash.digest(&transcript_through_finished)` (hex), `client_app_secret` (hex), `client_app_keys.key` + `iv` (hex) at the point where the session is returned. Compare hash bytes against an openssl-derived equivalent (via `openssl s_client -keylogfile`). If `client_app_secret` matches, H1 is falsified. If they differ, H1 is confirmed and we look for where transcripts diverge.

**C-NEW-3: ServerHello extensions audit.** Add a debug print in our ServerHello parser that lists every extension type byte seen, distinguishing those we process from those we silently skip. Run against example.com, google.com, api.github.com. If any extension appears that demands client-side action per RFC 8446 §4.2 and we skip it, H3 is confirmed.

## §6. Bidirection observation

The detection-direction readout alone falsified the leading hypothesis without requiring composition-direction probes. This is a Pin-Art operational property worth recording: the joint pattern from the detection side can be load-bearing on its own when one of the hypotheses is structurally distinguishable from the others. The composition direction is reserved for cases where detection's joint pattern is ambiguous between two or more hypotheses. Here it was not: the post-handshake-records-as-difference hypothesis was unambiguously falsified.

## §7. Closes

TLS-EXT 4 closes with the detection-direction readout in place, falsifier F1 triggered, and the hypothesis space redirected to transcript-hash / cipher-negotiation / ServerHello-extension drift. TLS-EXT 5 picks up with the C-NEW-1/2/3 composition-direction probes against the redirected hypothesis space. Probe is structurally complete; no code change in this EXT.
