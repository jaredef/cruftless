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
