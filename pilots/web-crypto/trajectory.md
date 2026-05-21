# rusty-web-crypto — Trajectory

Chronological resume anchors for the cryptographic-primitive workstream. Reads seed.md first.

Pair retroactively founded at WC-EXT 0 after the TLS workstream's TLS-EXT 6–8 narrowing localized a non-terminating bug to `ecdsa_verify`. Substantial prior substrate work (the existing ~2000-line crate) is treated as Phase Prior; subsequent WC-EXT rounds carry full per-round entries per Doc 581.

## Phase Prior — pre-pair substrate

The web-crypto pilot pre-existed Doc 733 and was developed under tags from `Phase-2-extension` (RSA, ECDSA, ECDH primitives) and `Π4.14` (Blake2b, Argon2id). State at Phase Prior close:

- SHA-2 family (256/384/512)
- HMAC-SHA-2 family
- HKDF
- AES-128-GCM, AES-256-GCM
- RSA-PKCS1-v1.5 sign/verify
- RSA-PSS sign/verify
- RSA-OAEP encrypt/decrypt
- ECDSA over P-256, P-384, P-521 sign/verify
- ECDH over P-256, P-384, P-521
- Blake2b (RFC 7693)
- Argon2id (RFC 9106)

All exercised by downstream consumer pilots; no formal probe-set under this workstream's discipline before WC-EXT 0.

---

## WC-EXT 0 — 2026-05-21 (workstream founding)

### Headline

Pair retroactively founded per Doc 733 after the TLS workstream's 8-round bidirectional Pin-Art narrowing isolated a non-terminating bug to `pilots/web-crypto/derived/src/lib.rs::ecdsa_verify` for one captured fixture from api.github.com's CertificateVerify signature. The web-crypto pilot was operating below Doc 733 §V's level-local operability threshold; the gap surfaced as opaque hang inside an upstream consumer (TLS) before the pair existed.

Per Doc 733 §V, founding this pair moves engagement fractal coverage from **5/6 to 6/6**.

### Commits

| commit | tag | recognition |
|---|---|---|
| (this commit) | (workstream founding) | `pilots/web-crypto/seed.md` + `trajectory.md` written. Doc 733 fractal-pair rationale. Pin-Art tag prefix `Ω.5.P06.E3.wc-*`. |

### Substrate at WC-EXT 0 close

- Pair exists; existing Phase Prior substrate is reconstructed into one trajectory block.
- One known bug: `ecdsa_verify` over P-256 hangs on the api.github.com fixture (TLS-EXT 8 capture).
- Fixture file `pilots/web-crypto/fixtures/ecdsa-p256-apigithub-2026-05-21.hex` to be produced in WC-EXT 1.

### Open scope at WC-EXT 0 boundary

1. **WC-EXT 1**: capture the fixture as a committed file + write a unit test that calls `ecdsa_verify` with it. Run the test under `cargo test -p rusty-web-crypto --release -- --include-ignored` (gated #[ignore] for a known-hanging test). Confirms bug is fully isolated to this pilot.
2. **WC-EXT 2**: instrument `ecdsa_verify` sub-functions with debug prints. Re-run unit test. Last printed line names the suspect (mod_inv_fermat / ec_scalar_mul / ec_add).
3. **WC-EXT 3**: substrate fix at the suspect sub-function. Re-run unit test → expect Err (signature doesn't verify, since github's leaf cert has rotated) or Ok. Either way the hang should be gone.
4. **WC-EXT 4**: re-run TLS-EXT 4's 5-endpoint probe. Expect E1/E3/E4 to flip from FAIL to whatever downstream behavior is next (probably some other case).

### Resume protocol

Read seed.md, then this trajectory's WC-EXT 0 entry. The next substrate move is WC-EXT 1 (fixture + unit test). The fixture data is already captured in the TLS-EXT 8 trace (`pilots/tls/probes/wire-captures/tls-ext-8-h8-confirmed.md` references the values; raw bytes are in this session's commits).

Pin-Art tag count: 0 substrate moves under the new prefix so far (workstream founding only).

---

*WC-EXT 0 closes the founding round. Subsequent rounds add substrate moves at the cryptographic-primitive tier.*

---

## WC-EXT 2 — 2026-05-21 (sub-function instrumentation — H8 falsified, H9 introduced)

### Headline

Added per-sub-function debug prints to `ecdsa_verify` (mod_inv_fermat, mod_mul × 2, ec_scalar_mul × 2, ec_add). Re-ran the WC-EXT 1 fixture test under `CRUFTLESS_WC_DEBUG=1`. **Major reframing:**

```
[wc-ec] e = hash mod n
[wc-ec] → mod_inv_fermat(s, n)
[wc-ec]   mod_inv_fermat OK
[wc-ec] → mod_mul(e, w, n) = u1
[wc-ec] → mod_mul(r, w, n) = u2
[wc-ec] → ec_scalar_mul(u1, G) = p1
[wc-ec]   p1 OK
[wc-ec] → ec_scalar_mul(u2, Q) = p2
[wc-ec]   p2 OK
[wc-ec] → ec_add(p1, p2)
[wc-ec]   ec_add OK
[wc-ext-1] result: Ok(())
test result: ok. 1 passed; 0 failed ... finished in 8.18s
```

**The test PASSED in 8.18 seconds.** `ecdsa_verify` returned `Ok(())` — meaning the signature verified correctly. **There is no hang.** What we called a "hang" across TLS-EXT 5–8 is actually slow execution: ec_scalar_mul on the Pi takes roughly 4 seconds per call. ECDSA-P-256 verification requires 2 scalar muls, so ~8 seconds per verify.

In the TLS context: chain_walk performs additional ECDSA cert-signature verifications, AND there's the CertificateVerify verify itself. For api.github.com's 2-3-cert chain plus CV, total handshake time is in the 24-32+ second range. A 30-second timeout still falls short (confirmed empirically: TLS probe against api.github.com with `timeout 30` still hits 143).

### Commits

| commit | tag | recognition |
|---|---|---|
| (this commit) | `Ω.5.P06.E3.wc-h8-falsified-h9-introduced` | sub-function bisect produced 8s verify (not hang); H8 (non-terminating) falsified; H9 (ec_scalar_mul performance) introduced |

### Hypothesis space reframed

- ~~H8 ecdsa_verify enters non-terminating path~~ ❌ **FALSIFIED.**
- **H9** introduced: `ec_scalar_mul` on Pi is glacially slow (~4s/call). ECDSA-P-256 verify needs 2 calls ≈ 8s. TLS handshakes against ECDSA-leaf CDNs need additional verifications during chain_walk; total handshake time exceeds any reasonable probe budget. The "hang" was actually slow-but-terminating execution beyond the timeout horizon.

### Substrate-move target reframed

The fix is **performance**, not **correctness**. Standard ECC scalar-multiplication optimizations:

1. **Window-based scalar mul (wNAF)**: process k bits at a time instead of 1 bit at a time. 4-bit window reduces add operations by ~4×.
2. **Precomputed comb table for the fixed generator G**: G is constant; precompute `[G, 2G, 4G, 8G, ...]` at module init; scalar mul with G becomes table lookup + adds (no doubles). 5-10× speedup for the u1*G call.
3. **Constant-time Montgomery ladder**: alternative algorithm with predictable timing (also side-channel-friendly).
4. **Projective coordinates**: avoid modular inverse on every point operation. The biggest single win — typical 5-20× speedup over affine-only.

Any of (2) and (4) together would bring P-256 verify well under 1 second on the Pi, making CDN handshakes complete in normal probe budgets.

### Probe-set implication

Re-categorize the TLS workstream's 5-endpoint probe per the H9 reframing:

| endpoint | observed | under-H9 reading |
|---|---|---|
| E1 example.com (CloudFront, ECDSA leaf likely) | "Codec/hang" | slow ECDSA verify; would succeed in 30-60+ s |
| E3 google.com (Google FE, ECDSA leaf) | "Codec/hang" | same |
| E4 api.github.com (Fastly, ECDSA-P-256 confirmed) | "hang at CV" | 8s+ per verify × multiple verifies = 30+s |
| E2 httpbin.org | CloseNotify mid-handshake | unrelated to H9 |
| E5 registry.npmjs.org | TLS-1.2-only fatal alert | unrelated to H9 (Case-4 scope) |

H9 likely explains E1, E3, E4. E2 and E5 are different cases.

### Open scope at WC-EXT 2 boundary

1. **WC-EXT 3**: implement projective-coordinate scalar mul OR precomputed comb table for the generator. Either alone should bring verify below 2s; both together should land it under 500ms.
2. **WC-EXT 4**: re-run WC-EXT 1 fixture test, expect sub-second result.
3. **WC-EXT 5**: re-run TLS-EXT 4's 5-endpoint probe with budgets sized for new speed. Expect E1, E3, E4 to flip from FAIL to either PASS or some next-failure-mode (which is itself useful diagnostic).

Verified empirically that 30s TLS budget against api.github.com still hits SIGTERM, so the speedup is genuinely needed; expanding the probe timeout is not a substitute.

---

*WC-EXT 2 closes with a major reframing. The "hang" was glacial-but-terminating execution. Substrate-move target moves from "fix non-terminating bug" to "optimize ec_scalar_mul to a sane speed for ECDSA-P-256 verify on Pi."*
