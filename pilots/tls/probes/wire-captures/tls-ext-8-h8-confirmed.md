# TLS-EXT 8 — H8 Confirmed: ECDSA-P-256-SHA256 Verify Hang

**Tag**: `Ω.5.P06.E1.tls-cv-scheme-captured` (TLS-EXT 8)
**Date**: 2026-05-21

## §1. Probe

Re-ran the `[hs-cv]`-instrumented binary against api.github.com. Last debug line before hang:

```
[hs-phase3-drain]   msg_type=CertificateVerify used=79
[hs-cv] scheme=0x0403 sig_len=71
[hang]
```

## §2. Confirmation

scheme=0x0403 = **SIG_ECDSA_SECP256R1_SHA256**. sig_len=71 = **DER-encoded ECDSA-P-256 signature** (SEQUENCE { INTEGER r, INTEGER s } where each integer is 32-33 bytes plus overhead).

**H8 CONFIRMED.** The hang is in our ECDSA-P-256-SHA256 verify path for api.github.com's CertificateVerify signature.

## §3. Substrate-move target

The hang is in one of:

1. `rusty_asn1_der::parse_single(signature)` at `pilots/tls/derived/src/driver.rs:181` — DER parse of the 71-byte SEQUENCE.
2. `rusty_web_crypto::ecdsa_verify(&curve, qx, qy, &hash, &sig_raw)` at `pilots/tls/derived/src/driver.rs:198` — the actual EC math.

Inside `ecdsa_verify` (`pilots/web-crypto/derived/src/lib.rs:1381`), the suspect sub-functions are:
- `mod_inv_fermat(&s, &c.n)` — modular inverse via Fermat's little theorem (exponentiation; could loop on bad input).
- `ec_scalar_mul(c, &u1, &c.g)` + `ec_scalar_mul(c, &u2, &q)` — scalar multiplication on the curve (double-and-add; could loop if scalar handling is wrong).
- `ec_add(c, &p1, &p2)` — point addition.

Most likely candidates (in order):
- **`mod_inv_fermat`** — Fermat's-little-theorem inversion uses an exponentiation loop that could non-terminate if the exponent encoding has a bug, or if it iterates over bit-length and the bit-length computation is off.
- **`ec_scalar_mul`** — double-and-add iterates over scalar bits; bit-length-off-by-one could cause infinite loop.

## §4. The fix lives in pilots/web-crypto

Per Doc 733 §V's open-scope analysis: the web-crypto pilot lacks its own seed.md/trajectory.md pair. The TLS workstream identified the bug; founding `pilots/web-crypto/seed.md + trajectory.md` and landing the fix as a `WC-EXT N` substrate move is the next round of structural work.

The probe set on the web-crypto pilot would be:
- WC-EXT 1: minimal unit test calling `ecdsa_verify` with api.github.com's actual sig + qx/qy/hash bytes (replay the failing input as a fixture).
- WC-EXT 2: bisect — replace each suspect sub-function with a known-good implementation in turn until the hang disappears. The disappearance EXT names the bug.
- WC-EXT 3: fix the bug; re-run TLS-EXT 4's 5-endpoint probe; expect E1/E3/E4 to flip from FAIL to ... whatever the post-handshake behavior is. (May yield more downstream findings.)

## §5. Workstream-tier status

The TLS pilot's substrate work is structurally complete for this CDN-hang investigation. The remaining work lives in `pilots/web-crypto`. Per Doc 733 the fractal coverage moves from 5/6 to 6/6 when the web-crypto pair is founded.

TLS-EXT 7 + 8 demonstrate the bidirectional Pin-Art's narrowing capability: 7 EXTs (1 design + 6 substrate-debug rounds + 1 confirmation) narrowed the original "0/5 PASS, four opaque failure modes" finding to "one function in one downstream pilot."
