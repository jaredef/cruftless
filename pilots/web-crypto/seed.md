# rusty-web-crypto — Resume Vector / Seed

**Workstream**: the cryptographic-primitive substrate (hashes, AEAD, ECC, RSA, HMAC, HKDF, ECDH, ECDSA) for cruftless. Founded per Doc 733 §V open-scope after the TLS workstream localized a non-terminating bug to this pilot's `ecdsa_verify` path (TLS-EXT 6–8, commits 484419c0 → cc6f6a3a).
**Author**: 2026-05-21 session.
**Parent**: cruftless engagement (`/home/jaredef/rusty-bun`).
**Composes with**:
- [Doc 733](../../../corpus-master/corpus/733-fractal-seeds-and-trajectories-recurrent-resume-vector-pairs-across-substrate-depth-as-the-operating-conditions-layer-for-pin-art-at-engagement-scale.md) (founding rationale: closes the web-crypto gap in §V open-scope; moves engagement fractal coverage from 5/6 to 6/6).
- [Doc 619](../../../corpus-master/corpus/619-pin-art-canonical-formalization.md) (Pin-Art apparatus).
- `pilots/tls/derived/` — the upstream consumer that surfaced this workstream's first substrate-move target via 8 EXTs of bidirectional Pin-Art narrowing.
- `pilots/x509/derived/` + `pilots/asn1-der/derived/` — co-tier siblings in the cryptographic-substrate cluster.
- All TLS, JOSE/JWT, WebCrypto, and node:crypto consumers downstream.

## I. Telos

Maintain a correct, terminating, side-channel-aware (where engagement-tier resources permit) suite of cryptographic primitives sufficient for cruftless's TLS, JOSE, node:crypto, and WebCrypto surfaces. The success criterion is bidirectional: (a) every consumer of this pilot's surface gets correct results in bounded time, on all inputs; (b) every input the engagement's reference engines (curl + openssl + node) accept, this pilot also accepts.

### I.1 Bounded first-cut telos

The immediate workstream telos: **close the TLS-EXT 8 finding** — the `ecdsa_verify` hang against a real-world fixture from api.github.com's CertificateVerify signature. Success = the captured 32-byte hash + (qx, qy) + 64-byte sig_raw fixture verifies (or fails-non-hanging) in bounded time. Probe set scales out from there to cover the cryptographic primitives the engagement's downstream pilots already exercise.

## II. Apparatus

Founded retroactively after substantial prior substrate work (the entire `pilots/web-crypto/derived/src/lib.rs` is already populated: SHA-2 family, AES-GCM, RSA-PSS, RSA-PKCS1, RSA-OAEP, ECDSA over P-256/P-384/P-521, ECDH over the same curves, HKDF, HMAC, Blake2b, Argon2id). The pair didn't exist before this round; the substrate is large but its structural state was illegible because of the missing pair. Founding the pair makes the existing surface workstream-shaped.

Per Doc 733 §V, the workstream is **resolver-instance at the cryptographic-primitive tier**, between the application-protocol pilots above (TLS, JOSE) and `BigUInt` arithmetic + raw byte-manipulation below. Composes upward as a substrate consumed by ~10 application pilots; composes downward on the in-crate BigUInt + raw primitives only.

Per Doc 730 §XII–§XVI, substrate moves at this tier are gated on the §XVI bidirectional engine-diff oracle. The reference is **rustcrypto + openssl + node**: any input these three accept correctly, this pilot must also accept correctly. Any input they reject, this pilot may reject (but the rejection must be loud, never hanging).

## III. Methodology

Substrate moves under standing Pin-Art discipline. The bidirectional probe at this tier:
- **Detection direction**: replay a captured input through this pilot's primitive (fixtures from real-world handshakes, JWT tokens, file encryption). Observe what the primitive does (returns, errors, hangs, panics).
- **Composition direction**: synthesize crafted inputs at the primitive's edge cases (max-length, min-length, zero, near-modulus, malformed-DER, etc.). Observe failure modes.

The pair-reading discriminates spec-compliant correctness from implementation-specific bugs.

First substrate moves (queued):

1. **WC-EXT 1 (TLS-EXT 8 close)**: unit test in `tests/` (or in lib.rs `#[cfg(test)]`) calling `ecdsa_verify(curve_p256(), qx, qy, hash, sig_raw)` with the captured fixture. Expected: the test hangs (confirms the bug is in this pilot's code, not in TLS-pilot DER parsing or anything upstream). With test hang confirmed, the bug is fully isolated.

2. **WC-EXT 2 (bisect)**: instrument `ecdsa_verify` with per-sub-function debug prints (mod_inv_fermat, ec_scalar_mul × 2, ec_add). Re-run the unit test. Last printed line names the suspect.

3. **WC-EXT 3 (fix)**: substrate move at the suspect sub-function. Per the TLS-EXT 8 analysis, most-likely suspects (in order):
   - `mod_inv_fermat` — Fermat-little-theorem inversion via exponentiation; bit-length bug could cause infinite loop on certain s values.
   - `ec_scalar_mul` — double-and-add over scalar bits; off-by-one or wrong termination could loop.
   - `ec_add` — point addition; less likely.
   - DER parsing precursors are already eliminated (the parse_single + reader.read_tag debug at the TLS tier showed them completing).

4. **WC-EXT 4+**: regression check across the engagement's downstream consumer pilots (TLS, JOSE, web-crypto API tests). Confirm no regression in JWT signing, RSA-PSS verification, ECDH key exchange.

## IV. Carve-outs and bounded scope

- **No side-channel hardening in first cut.** Constant-time arithmetic is queued for a later workstream tier; the immediate work is correctness + termination.
- **No new primitives in first cut.** Existing surface (SHA-2, AES-GCM, RSA, ECDSA, ECDH, HKDF, HMAC, Blake2b, Argon2id) is what we maintain. Curve25519 / Ed25519 / X25519 are queued as TLS-workstream-driven extensions (TLS-EXT 4 cluster C).
- **No reformulation of BigUInt.** The in-crate `BigUInt` arithmetic is treated as substrate; any bug found there is its own sub-workstream.

## V. Standing artefacts

- `pilots/web-crypto/derived/Cargo.toml` + `src/lib.rs` (already exists, ~2000 lines).
- `pilots/web-crypto/fixtures/` — captured real-world inputs that trigger bugs; gitignored bulk + a curated subset committed.
- `pilots/web-crypto/fixtures/ecdsa-p256-apigithub-2026-05-21.hex` — the TLS-EXT 8 capture (qx, qy, hash, sig_raw). WC-EXT 1 fixture.
- `trajectory.md` — time-ordered record of substrate moves and probe results.

## VI. Resume protocol

Read Doc 733 (the fractal-pair rationale for this pair's founding) and the TLS workstream's TLS-EXT 6 + 8 findings (which surfaced this workstream's first substrate-move target). Then this seed and trajectory.md.

The first substrate move is WC-EXT 1: a unit test that calls `ecdsa_verify` with the captured fixture and confirms the hang reproduces in pure-Rust unit-test form. After that confirmation, WC-EXT 2 bisects, WC-EXT 3 fixes.

Pin-Art tag prefix for this workstream: `Ω.5.P06.E3.wc-*` for crypto-primitive-tier substrate moves (P06 = Host pipeline per dag-coordinates.json, E3 = Intrinsic object — the engine-layer that crypto primitives compose at).

Engine state at this workstream's founding (WC-EXT 0 / 2026-05-21):
- `pilots/web-crypto/derived/src/lib.rs` is populated with ~2000 lines covering the listed primitives.
- One known bug: `ecdsa_verify` for curve P-256, SHA-256 hash, captured fixture from api.github.com — hangs.
- All other primitives are exercised by downstream consumers without known hangs, but the broader probe set has not been run under this pair's discipline yet.
