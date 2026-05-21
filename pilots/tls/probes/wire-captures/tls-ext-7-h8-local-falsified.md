# TLS-EXT 7 — H8 Local Reproduction FAILED; Refined Hypothesis

**Tag**: `Ω.5.P06.E1.tls-ec-local-control` (TLS-EXT 7)
**Date**: 2026-05-21
**Companion**: [tls-ext-6-handshake-hang-localized.md](./tls-ext-6-handshake-hang-localized.md).

## §1. Probe executed

Per the TLS-EXT 6 plan: regenerated a localhost cert with ECDSA-P-256 key (`openssl ecparam -name prime256v1 -genkey` + `openssl req -new -x509`). Started `openssl s_server` with the ECDSA cert+key. Pointed rusty-tls at it.

Cert verification:
```
Signature Algorithm: ecdsa-with-SHA256
Public Key Algorithm: id-ecPublicKey
```

## §2. Result

The handshake **completed CertificateVerify processing** and progressed to Finished:

```
[hs-phase3-drain]   msg_type=CertificateVerify used=264
[hs-cv] scheme=0x0804 sig_len=256
[hs-phase3-drain] iter=2 hb_len=0
[hs-phase3-drain]   decode_handshake Err: UnexpectedEnd (continue 'outer for more bytes)
[hs-phase3] iter=5 acc_len=58 ...
[hs-phase3-drain]   msg_type=Finished used=36
[ec-probe] result: Err("SelfSignedNotInTrust")
```

No hang. Probe terminated cleanly with `SelfSignedNotInTrust` from our chain_walk (orthogonal issue — chain_walk rejects the leaf because the leaf and root are the same cert).

## §3. Two surprises

**Surprise 1: scheme=0x0804 (RSA-PSS-RSAE-SHA256), not 0x0403 (ECDSA-P-256-SHA256).** Even though the leaf cert has an EC public key, openssl's CertificateVerify reported RSA-PSS-RSAE-SHA256. Either:
- openssl s_server auto-selected RSA-PSS at the TLS layer for some independent reason (unlikely with an EC leaf)
- The leaf cert is internally signed with EC but the CertificateVerify scheme negotiation is reading something different
- Most likely: openssl falls back to a built-in RSA cert if the supplied cert doesn't match its preferred signing path for our advertised sigalg set (`SIG_ECDSA_SECP256R1_SHA256, SIG_RSA_PKCS1_SHA256, SIG_RSA_PSS_RSAE_SHA256`)

The sig_len=256 (RSA-2048-sized) corroborates: this is an RSA signature, so openssl is signing with an RSA key from somewhere, not the EC key we supplied. This means **the probe did not actually test the ECDSA verify path.** The probe's intended falsifier didn't fire.

**Surprise 2: H8 not yet either confirmed or falsified.** Because the local probe didn't actually exercise rusty-tls's ECDSA verify path, the question "does our ECDSA-P-256-SHA256 verify hang on certain inputs?" remains open.

## §4. The api.github.com signature shape revisited

api.github.com's CertificateVerify was 78 bytes (74 sig). 74 bytes is consistent with DER-encoded ECDSA-P-256 (SEQUENCE { INTEGER r, INTEGER s } where each integer is ~32-33 bytes plus DER overhead). The signature is almost certainly ECDSA.

Our pilot's code:
```rust
let scheme = ((msg.body[0] as u16) << 8) | (msg.body[1] as u16);
let sig_len = ((msg.body[2] as usize) << 8) | (msg.body[3] as usize);
```

If we read 0x0403 (ECDSA-P-256-SHA256), we route through the ECDSA-verify branch. Our ECDSA branch calls `rusty_web_crypto::ecdsa_verify_p256_sha256` or similar.

Without the [hs-cv] print from api.github.com, we don't know what scheme byte we read. The TLS-EXT 6 trace shows `msg_type=CertificateVerify used=78` then hang — at that point the scheme parsing has not yet happened (it's inside `verify_certificate_verify_signature`). Adding the [hs-cv] print BEFORE verify_certificate_verify_signature would reveal the scheme byte for the failing case.

## §5. Refined hypothesis

**H8 refined**:  the hang is in `verify_certificate_verify_signature` for the scheme api.github.com presents. The scheme may or may not be ECDSA-P-256-SHA256; we have inferred it from sig-length but not directly observed it for the failing case. The [hs-cv] print would close this gap.

The probe should be re-run against api.github.com with the same instrumentation that printed scheme=0x0804 for the local probe. Then the scheme byte for the failing case is in the trace.

## §6. TLS-EXT 8 plan

1. Run the [hs-cv]-instrumented binary against api.github.com (timeout 12s). The last [hs-cv] line before hang shows the failing scheme.
2. Once the scheme is known, the target code path is one match arm in `verify_certificate_verify_signature` (lines ~120-200 of `pilots/tls/derived/src/driver.rs`) which calls into a specific `rusty_web_crypto` function. That function is the substrate-move target.
3. Optional: produce a minimal Rust unit test that calls the suspect web-crypto function with the api.github.com sig bytes + leaf pubkey + tbs. If it hangs in unit-test form, the bug is fully isolated to the web-crypto pilot.

The substrate move is then in the web-crypto pilot. The fractal-pair work for web-crypto (per Doc 733) can be founded as part of TLS-EXT 8+ or as a parallel pilot-found.

## §7. Note: this is normal Pin-Art behavior

Six rounds of bidirectional Pin-Art produced a strong hypothesis (H8). The seventh round attempted local reproduction; the reproduction setup itself collapsed (openssl substituted an RSA cert / signature) and the probe didn't test what it intended. Per Doc 619, the value of Pin-Art is in the cumulative joint pattern: any single probe may fail to discriminate; the workstream proceeds by chaining many independent probes until the pattern is unambiguous. TLS-EXT 7's outcome — "the probe didn't test what it intended; refine and re-probe" — is normal Pin-Art operation, not workstream failure.
