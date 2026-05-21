# TLS-EXT 6 — C-NEW-4 Handshake Hang Localized to CertificateVerify

**Tag**: `Ω.5.P06.E1.tls-c-new-4-cv-hang` (TLS-EXT 6)
**Date**: 2026-05-21
**Companion**: [tls-ext-5-c-new-1-findings.md](./tls-ext-5-c-new-1-findings.md); [bidirectional-pin-art-probe-design.md](../bidirectional-pin-art-probe-design.md).

## §1. Probe executed

C-NEW-4 from the TLS-EXT 5 plan: add `CRUFTLESS_TLS_DEBUG`-gated step-by-step prints throughout `complete_handshake`. Phase 1 records, Phase 3 records + decrypts + handshake-message drain, plus per-iteration counters for the inner loops. Then run against api.github.com (a hanging CDN endpoint).

## §2. Trace (api.github.com)

```
[pm_http_get] start https://api.github.com/
[pm_http_get] connecting → api.github.com:443
[hs-phase1] iter=1 acc_len=0
[hs-phase1]   need more; read_some...
[hs-phase1]   read_some → 3129 bytes (acc=3129)
[hs-phase1] iter=2 acc_len=3129
[hs-phase1]   got record ct=Handshake frag_len=155
[hs-phase3] iter=1 acc_len=2969 hb_len=0 seq=0
[hs-phase3]   record ct=ChangeCipherSpec frag_len=1
[hs-phase3] iter=2 acc_len=2963 hb_len=0 seq=0
[hs-phase3]   record ct=ApplicationData frag_len=37
[hs-phase3]   decrypted: inner_ct=22 pt_len=20
[hs-phase3-drain] iter=1 hb_len=20
[hs-phase3-drain]   msg_type=EncryptedExtensions used=20
[hs-phase3-drain] iter=2 hb_len=0
[hs-phase3-drain]   decode_handshake Err: UnexpectedEnd (continue 'outer for more bytes)
[hs-phase3] iter=3 acc_len=2921 hb_len=0 seq=1
[hs-phase3]   record ct=ApplicationData frag_len=2758
[hs-phase3]   decrypted: inner_ct=22 pt_len=2741
[hs-phase3-drain] iter=1 hb_len=2741
[hs-phase3-drain]   msg_type=Certificate used=2741
[hs-phase3-drain] iter=2 hb_len=0
[hs-phase3-drain]   decode_handshake Err: UnexpectedEnd (continue 'outer for more bytes)
[hs-phase3] iter=4 acc_len=158 hb_len=0 seq=2
[hs-phase3]   record ct=ApplicationData frag_len=95
[hs-phase3]   decrypted: inner_ct=22 pt_len=78
[hs-phase3-drain] iter=1 hb_len=78
[hs-phase3-drain]   msg_type=CertificateVerify used=78
[NO FURTHER OUTPUT — process hangs]
```

## §3. Localization

The last printed event is `msg_type=CertificateVerify used=78`. The next event would have been `[hs-phase3-drain] iter=2 hb_len=0` or `[hs-phase3-drain]   decode_handshake Err: ...`. Neither appears.

This means the hang occurs **inside the CertificateVerify match arm** of the handshake-message drain loop, specifically at one of:

1. `verify_certificate_verify_signature(scheme, &leaf.subject_public_key_info, &tbs, signature)` — calls into `rusty_web_crypto` for the actual signature verification per scheme.
2. The `hash.digest(&transcript_through_cert)` call (unlikely — SHA-256 is short-cycled and well-tested).
3. The transcript clone + truncate operations (unlikely — pure memcpy).

By elimination, the hang is in **`rusty_web_crypto`'s signature-verify path** for the SignatureScheme api.github.com used. The CertificateVerify body length (78 bytes) = 4 bytes header + 74 bytes signature. GitHub's Fastly cert chain typically presents an ECDSA-P-256 leaf (74 bytes is consistent with DER-encoded ECDSA-P-256-SHA256: SEQUENCE { INTEGER r, INTEGER s } where each integer is ~32-33 bytes).

So the suspect is `rusty_web_crypto::ecdsa_*_verify` over P-256 (`SIG_ECDSA_SECP256R1_SHA256 = 0x0403`).

## §4. Hypothesis cluster — narrowed

The TLS-EXT 5 hypothesis cluster:
- H5 handshake state machine deadlock (favored)
- H6 infinite read-decode-continue loop (favored)

is refined to:

**H8 — `rusty_web_crypto` ECDSA-P-256-SHA256 verification enters a non-terminating path on certain valid inputs.** The substrate site is in the `pilots/web-crypto/derived/` pilot, not the TLS pilot proper. The TLS pilot's complete_handshake is correct in structure; it's calling into a downstream pilot whose primitive hangs.

This re-categorizes the substrate move at one tier down. The TLS workstream identified the bug-site; fixing it is **web-crypto pilot work**, which is yet another resolver-instance lacking its own seed.md/trajectory.md pair per Doc 733 §V open-scope.

## §5. Why localhost openssl s_server didn't trigger this

Our self-signed cert at `/tmp/tls-ext-4/cert.pem` was generated with `openssl req -newkey rsa:2048`. The leaf is RSA, so the CertificateVerify scheme is RSA-PKCS1 or RSA-PSS, routed through `rsa_*_verify` — a different code path that does not hit the hanging ECDSA verify. That explains the local-vs-CDN asymmetry: it's not CDN-specific TLS protocol behavior; it's leaf-cert-key-type asymmetry. Our self-signed cert happened to be RSA, masking the ECDSA verify bug.

## §6. Falsifier confirmation needed

The H8 hypothesis is testable: generate a localhost cert with an ECDSA-P-256 key, restart openssl s_server with it, point rusty-tls at it. If the same hang reproduces, H8 is confirmed and the fix is in `pilots/web-crypto`. If it doesn't, the hang is CDN-specific to something else and H8 needs revision.

```bash
openssl ecparam -name prime256v1 -genkey -out /tmp/tls-ext-4/ec-key.pem
openssl req -new -x509 -key /tmp/tls-ext-4/ec-key.pem -out /tmp/tls-ext-4/ec-cert.pem -days 1 -subj '/CN=localhost'
# Then s_server -cert ec-cert.pem -key ec-key.pem and re-run localhost_tls_probe
```

This local-ECDSA probe is the TLS-EXT 7 first move; if H8 is confirmed, TLS-EXT 8+ is web-crypto substrate work (under a new `pilots/web-crypto/seed.md` + `trajectory.md` pair per Doc 733).

## §7. Wider picture

The bidirectional Pin-Art probe set has now run six rounds:
- TLS-EXT 1: matrix established
- TLS-EXT 2: close_notify graceful (structurally correct, probe-neutral)
- TLS-EXT 3: wire-diff probe (recategorized cases per §XVI)
- TLS-EXT 4: D-direction Pin-Art falsified post-handshake-records hypothesis
- TLS-EXT 5: C-direction Pin-Art surfaced handshake-tier hang; H7 falsified by bisect
- TLS-EXT 6: C-NEW-4 localized hang to CertificateVerify-time signature verify, likely ECDSA-P-256 in web-crypto

Each round either flipped a probe cell, falsified a hypothesis, or narrowed the substrate-move target. None made it 1/5 → 2/5 yet. The asymmetry is informative: probe cells flip only after a substantial substrate move; instrumentation and hypothesis-narrowing rounds do not. The probe count is the right metric for the workstream's telos (CDN-passable), but it does not reflect the diagnostic substrate already accumulated.

Doc 733 §V's threshold ordering predicts that engagement-internal substrate fitness grows in three tiers: level-local operability (we have it now for the TLS workstream — six EXTs with structured findings), cross-level navigation (the web-crypto pilot needs its own pair before that tier opens), compositional substrate refactoring (the long-term target). The TLS workstream is sitting at level-local; the web-crypto-pilot pair is the next move toward cross-level.
