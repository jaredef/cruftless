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

### II.1 The substrate layering inside the crate

The crate's source file structures three distinct substrate tiers that the seed.md / trajectory.md pair treat as a unit but that admit separate substrate-move discipline:

- **BigUInt arithmetic tier**: `BigUInt` type + `add`, `sub`, `mul`, `divmod`, `modulo`, `mod_pow`. The lowest tier; every cryptographic primitive's per-op cost is bounded by this tier's `mod_mul` performance. Substrate moves here propagate upward through every primitive simultaneously. **WC-EXT 8+ landed Montgomery-form arithmetic for P-256 at this tier.**
- **Modular-arithmetic tier**: `mod_add`, `mod_sub`, `mod_mul`, `mod_inv_fermat`, `batch_mod_inv`. Composes on the BigUInt tier. Most substrate moves at the engagement's optimization rounds operate here (WC-EXT 7's Montgomery batch inversion is at this tier).
- **Elliptic-curve tier**: `JacPoint`, `jac_double`, `jac_add_affine`, `jac_to_affine_batch`, `ec_scalar_mul`, `ecdsa_verify`, plus the P-256 base-point table at `src/p256_base_table.rs` (built-time bake per Doc 731 §XV.g Regime 1). Composes on the modular-arithmetic tier.

Substrate moves are recorded in trajectory.md with explicit tier-attribution. A WC-EXT round that targets `BigUInt` arithmetic (e.g., Montgomery multiplication, word-aligned division, Karatsuba) is at the BigUInt tier; a round that targets EC algorithm choice (e.g., wNAF, comb tables) is at the EC tier. The two have very different blast radii — BigUInt moves touch all primitives; EC moves touch only the primitive they're applied to.

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

### VI.1 State at WC-EXT 10 close (2026-05-21, end of session 1)

Substantial substrate work landed over 11 rounds (WC-EXT 0 founding + WC-EXT 1–10 substrate moves). The pilot now has:

- **Working P-256 ECDSA verify in Montgomery form, 82× cumulative speedup over the WC-EXT 1 baseline.** Fixture `ecdsa_verify(api.github.com qx/qy/sig)`: 8.18s → 0.10s.
- **Three substrate tiers operating** (per §II.1): BigUInt (Montgomery REDC, batch inversion, from_limbs accessor); modular arithmetic (`batch_mod_inv` via Montgomery's trick); elliptic-curve (Jacobian coords + base-point comb table + wNAF infrastructure + Mont-form EC routines).
- **Two regimes operative on the baked base table**: WC-EXT 5 standard-form Regime 1 (1047-line generated source); WC-EXT 10 Mont-form Regime 2 (lazy first-use init from the Regime 1 table).
- **Standing infrastructure for future substrate moves**: `batch_mod_inv` reusable for any multi-inversion site; Mont-form helper primitives (`p256_mont_pow`, `p256_mont_inv`, `p256_mont_mul_by_small`) reusable for any future EC-tier work.
- **5-endpoint TLS probe at 3/5 PASS** (engagement-internal HTTPS reaches example.com, google.com, api.github.com). Remaining: E2 httpbin (separate bug), E5 npm (Case-4 scope, TLS-1.2-only endpoint).

### VI.2 Bottleneck has relocated — next strategic target

After WC-EXT 10, the api.github.com TLS handshake still takes ~9.5s wallclock despite ECDSA verify dropping to 0.10s. Bottleneck has relocated to **RSA verify on cert chain intermediates** (chain_walk runs ECDSA on the leaf but RSA on intermediates). RSA-2048 verify computes m^65537 mod n via ~17 squarings of 2048-bit `mod_mul` calls, all going through the current binary-long-division `BigUInt::modulo`.

Next strategic substrate-move target: **WC-EXT 12 generalize Montgomery to arbitrary odd-prime moduli** — RSA 2048/3072/4096, P-384, P-521. Per Doc 731 §XV.e Pred-731.XV.1, the framework should apply directly. Expected: 30-40× speedup on every RSA operation, propagating through chain_walk and JOSE/JWT primitives.

### VI.3 Doc 731 §XV.b empirical corroboration table

Each R-condition of §VII R1–R8 verified across the 10 substrate rounds:

- **R1 single tier**: no Mont-meta; one Mont implementation per substrate site (BigUInt REDC, EC mul/double/add).
- **R2 standard apparatus owns heavy lifting**: Hankerson §3.2 (Jacobian); Montgomery 1985 (REDC); Hankerson §3.3 (wNAF).
- **R3 verifier-before-emission**: on_curve + scalar range checks gate before Mont code runs.
- **R4 small enumerable deopt set**: ~4 deopt categories — (a) non-P-256 curve, (b) Identity, (c) Y-zero, (d) std-form upstream caller.
- **R5 first-cut tier-1 sufficient**: Jacobian + Mont without Montgomery ladder / Almost-Mont / Karatsuba gave 82×.
- **R7 no internal optimization passes**: every speedup came from algorithm-selection or substrate-tier choice.
- **R8 no async/generator**: cryptographic primitives are synchronous, single-call.

### VI.4 Open-scope catalog for the next session

| EXT | name | tier | target | projected |
|---|---|---|---|---|
| 11 | TLS handshake breakdown probe | TLS+web-crypto | locate the 9.5s | surface RSA paths |
| 12 | Mont generalize | BigUInt | arbitrary odd-prime moduli | 30-40× on RSA |
| 13 | Mont RSA route | RSA layer | apply Mont to RSA verify/sign | TLS handshake → <2s |
| 14 | Mont ECDH | ECDH layer | route key derivation through Mont | TLS handshake → <1s |
| — | Doc 735 §X amendment | corpus | intra-tier cost stratification (T-tier ops not uniform-cost; queued since WC-EXT 8) | corpus refinement |
| — | TLS-EXT 9 | TLS | E2 httpbin close-notify mid-handshake bug | independent, 4/5 PASS |

### VI.5 State at WC-EXT 26 close (session 1 full)

WC-EXT 11-26 landed all the items in §VI.4 plus 13 more substrate rounds. Cumulative speedup:

| metric | session start | WC-EXT 26 | × |
|---|---|---|---|
| ECDSA verify (P-256 fixture) | 8.18s | 0.07s | **117×** |
| 5-endpoint TLS probe | hang | 2.45s · 3/5 PASS | **14.7×** |
| api.github.com handshake | hang | 0.78s | **12.8×** |

**Substrate tiers now operating** (per §II.1 layering):
- BigUInt: Comba mul, Karatsuba above 24 limbs, generic MontCtx, P-256 + P-384 Solinas reduction (programmatic constants per §X.h.f)
- Modular arithmetic: batch inversion (Montgomery's trick), generic mod_pow_mont
- EC: per-curve Mont contexts cached; Solinas-form Jacobian for P-256 and P-384; generic Mont-form Jacobian for any curve (used for P-521)

**Corpus articulations driven by this workstream**: Doc 731 §XV (cryptographic optimization as lowering-compiler closure), Doc 731 §XV.g (build-time vs first-use init regimes), Doc 730 §XVII (performance-axis deviation pipeline), Doc 735 §X.h (cost-stratum tuple + three probe levels + four (P2) sub-cases). Each driven by one or more productive negative findings.

**Vs Bun**: 691ms 5/5 vs 2,450ms 3/5 = ~3.5× probe gap; per-endpoint ~16× gap. Down from infinite at session start.

### VI.6 Session 2 resume protocol

Read this section, then trajectory.md's "Cumulative status at WC-EXT 26 close" block. Session 2 priorities (by impact / LOC):

1. **Connection pooling** at TLS pilot tier — (P3) orthogonal. ~50 LOC. Biggest single-LOC workload-level win.
2. **WC-EXT 27 P-384 micro-bench** — quantify why P-384 Solinas's wallclock gain was 4%.
3. **TLS 1.2 fallback** — opens E5 npm. Substantial (~500 LOC).
4. **u64-limb BigUInt** — fundamental (P2) escalation. ~300 LOC.
5. **AES-NI / ARM crypto extensions** — (P5) hardware-bound.

The framework's amendment discipline is internalized; substrate moves at the BigUInt arithmetic tier should now ship with bench + consumer-route + fuzz probes per Doc 735 §X.h.c discipline, and use programmatic constants per Doc 735 §X.h.f to avoid the WC-EXT 25-style typo class.
