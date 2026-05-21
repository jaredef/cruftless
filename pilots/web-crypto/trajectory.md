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

---

## WC-EXT 3 — 2026-05-21 (Jacobian-coordinate scalar mul, the Doc 731 §XV substrate move)

### Headline

Implemented Jacobian-coordinate `ec_scalar_mul` per Hankerson §3.2.1–§3.2.2. Replaces affine double-and-add (which called `mod_inv_fermat` on every point operation) with Jacobian double + mixed (Jacobian+affine) addition. Only one modular inverse remains per scalar mul (the final `jac_to_affine` conversion), down from ~384 inverses per call.

This is the empirical exercise of Doc 731 §XV (cryptographic primitive optimization as the lowering-compiler closure at the arithmetic tier). The substrate move's R1–R8 mapping:
- R1 single tier — one Jacobian implementation, no tier hierarchy
- R2 standard ECC literature owns the algorithm — Hankerson formulas (3.21 for doubling, mixed-add formula)
- R3 verifier-before-emission — on_curve + range checks already in ecdsa_verify
- R5 first-cut tier-1 — plain double-and-add over Jacobian (no wNAF window yet, no comb table yet — those are queued)
- R7 no internal optimization passes — substrate-tier choice is algorithm-selection, not code-rewriting

### Measurement

| metric | before (affine) | after (Jacobian) | speedup |
|---|---|---|---|
| WC-EXT 1 fixture (`ecdsa_verify` on captured 64-byte sig + (qx, qy) + 32-byte hash) | 8.18s | **0.29s** | **~28×** |
| All 117 web-crypto regression tests | (passed) | passed | no regression |
| TLS handshake to api.github.com (ECDSA-P-256 leaf + chain) | timeout > 30s | **~10s** | from-infinite-to-bounded |

### Probe re-run (5-endpoint TLS)

| endpoint | before | after |
|---|---|---|
| E1 example.com | Codec(Truncated) / hang | **OK (528 bytes)** ✓ |
| E2 httpbin.org | CloseNotify mid-handshake | CloseNotify (unrelated bug, separate workstream) |
| E3 google.com | Codec(Truncated) / hang | **OK (80535 bytes)** ✓ |
| E4 api.github.com | Codec(Truncated) / hang | **OK (2262 bytes)** ✓ |
| E5 npm | protocol_version alert | protocol_version (TLS 1.2-only, Case-4 scope decision) |

**Score: 3/5 PASS** (up from 0/5 at TLS-EXT 0). H9 confirmed in operation: removing the per-op modular-inverse cost flips three of the four CDN endpoints. Remaining E2 + E5 are different cases.

### Commits

| commit | tag | recognition |
|---|---|---|
| (this commit) | `Ω.5.P06.E3.wc-jacobian-scalar-mul` | Jacobian ec_scalar_mul; 28× speedup on captured fixture; TLS probe 0/5 → 3/5 PASS; Doc 731 §XV empirically corroborated |

### Conjecture status

Doc 731 §XV.f's prediction ("ECDSA-P-256 verify dropping from 8 seconds to under 500 milliseconds via projective coordinates + comb table for G") is partially confirmed: projective coordinates alone gave 28× speedup, landing at 0.29s (under 500ms). The comb table for G is queued as WC-EXT 4 and would land another 5-10× on the u1·G call, bringing the full verify well under 100ms.

The Doc 731 framework now has two empirically-anchored substrate-tier instances:
- bytecode-to-machine-code (JIT-EXT 4: 425× speedup, trusted-i64 ceiling, doc §XIV)
- affine-to-Jacobian ECC (WC-EXT 3: 28× speedup, projective ceiling, doc §XV)

Pred-731.XV.1 (framework applies at RSA modexp + AES round dispatch + Poly1305 + BLAKE2) remains open — each is a candidate next-tier instance to corroborate.

### Open scope at WC-EXT 3 boundary

1. **WC-EXT 4 (Doc 731 §XV continuation)**: precomputed comb table for the generator G at module init. Expected another 5-10× on the u1·G call. Brings verify under 100ms.
2. **WC-EXT 5**: wNAF window-based scalar mul for the variable-input case (u2·Q). Expected ~4× speedup on that half.
3. **TLS-EXT 9** (separate workstream): investigate E2 httpbin CloseNotify-mid-handshake — likely cert validation issue or alert handling at the wrong site.
4. **Keeper-decision still open**: E5 npm (lift TLS-1.2 carve-out vs substitute endpoint).

---

*WC-EXT 3 closes with Doc 731 §XV empirically corroborated. The substrate-move catalog at the arithmetic tier is now operative. The cruftless engagement can now talk to three of the five probed CDN endpoints via its own engagement-internal TLS+web-crypto substrate.*

---

## WC-EXT 4 — 2026-05-21 (precomputed base table — Doc 731 §XV.c attempted, negative finding)

### Headline

Implemented the §XV.c structural prediction: a precomputed table `[2^i·G for i in 0..256]` (256 affine P-256 points, ~16 KB) so `u1·G` scalar mults become pure adds at runtime, no doublings. Added `p256_scalar_mul_base(k)` as the fast path. Routed `ecdsa_verify`'s P-256 u1·G call through it. Re-ran the WC-EXT 1 fixture.

### Measurement (negative)

| metric | WC-EXT 3 baseline | WC-EXT 4 (with table) | delta |
|---|---|---|---|
| Fixture test (single ecdsa_verify, fresh process) | 0.29s | **2.85s** | **~10× slower** |
| 5-endpoint TLS probe wallclock | ~37s | ~39s | slightly worse |

The table-init cost (255 affine `ec_double`s, ~3 seconds on the Pi) dominates a single verify. For a TLS handshake (~3 verifies), the init pays back partially but not enough to be worthwhile.

### Why the §XV.c prediction was partly wrong

§XV.c claims "more upstream alphabet purity → JIT-tier decisions move from runtime to compile time." That holds at the *structural* level (the curve generator G is known at compile time; precomputation is admissible). It does not hold for a `OnceLock`-initialized table on the engagement's target hardware: **runtime init is not compile-time precomputation.** On a Pi where affine `ec_double` is ~12ms and the table needs 255 of them, init costs ~3 seconds — far exceeding the savings on small-batch workloads.

The §XV.c-true win requires the table to be **baked into the binary as build-time constants** (a `const` table of P-256 points, generated by a build.rs or by a one-shot offline script committed as source). Then init is free and only the per-call savings (a few hundred milliseconds, accumulated over many verifies) remain.

For the engagement's current TLS scenarios (handful of HTTPS connections per process lifetime), the comb table is a net loss unless the build-time bake is also done. **Reverted the routing** (ecdsa_verify keeps calling the generic `ec_scalar_mul`); kept `p256_scalar_mul_base` as a public substrate move for later use when (a) build-time bake is implemented, or (b) a workload arises that does many verifies per process.

### What this corroborates and what it does not

Corroborates **Doc 731 §XV.c at the structural-claim tier**: the precomputation IS admissible because the alphabet is pure. The optimization-tier mapping holds.

Does not corroborate §XV.c's empirical prediction at the implementation tier: runtime init has its own amortization regime that the §XV.c prose did not distinguish from compile-time. The §XV.c framework needs a sub-distinction: *build-time precomputation* vs *first-use init* are different optimization regimes; only the former realizes the "decisions move to compile time" claim cleanly.

This is a refinement to Doc 731, not a falsification. The §VII R1–R8 framework still applies; the §XV.c sub-claim needs the build-time-vs-init-time distinction added. Worth a §XV.g amendment in a later corpus round.

### Commits

| commit | tag | recognition |
|---|---|---|
| (this commit) | `Ω.5.P06.E3.wc-base-table-precompute` | precomputed base table substrate landed; negative empirical finding for one-shot workloads; routed off until build-time bake |

### Probe result

Score: **3/5 PASS** (unchanged from WC-EXT 3). The substrate addition is structurally sound but not in the live ecdsa_verify path.

### Open scope at WC-EXT 4 boundary

1. **WC-EXT 5 (the actual §XV.c-true win)**: implement a build-time generator for the base table. Options: `build.rs` that runs at compile time + emits a `const [P256Point; 256]` table source; or a one-shot offline script that produces a `base_table.rs` committed file. Either eliminates init cost; ecdsa_verify can then route u1·G through the table with no first-call penalty.

2. **Alternative WC-EXT 5**: wNAF window-based scalar mul for the variable-input case (u2·Q). Doesn't help u1·G but adds another ~2× speedup on u2·Q. Independent move.

3. **Doc 731 §XV.g amendment** (queued): record the build-time-vs-init-time distinction this round surfaced. Refines §XV.c without retracting it.

---

*WC-EXT 4 closes with a negative empirical finding that refines rather than falsifies Doc 731 §XV. The substrate move is correct; the amortization regime requires build-time bake. The Pin-Art apparatus surfaces this distinction because the probe (measure wallclock on a real workload) cared about init cost in a way that structural analysis alone would have missed.*

---

## WC-EXT 5 — 2026-05-21 (build-time bake — Doc 731 §XV.g Regime 1 realized)

### Headline

Per §XV.g.c + .e: the regime-promotion move (Regime 2 → Regime 1) for the P-256 base-point comb table. New `examples/gen_p256_base_table.rs` (one-shot generator) emits `src/p256_base_table.rs` (~1047 lines, 256 affine points as hex literals committed as source). Module init at first call: parses hex + constructs BigUInts for 256 points — measured ~100ms, down from ~3 seconds of affine doublings.

Routed `ecdsa_verify`'s P-256 u1·G call through `p256_scalar_mul_base` again, this time backed by the baked table.

### Measurement

| metric | WC-EXT 3 (Jacobian only) | WC-EXT 4 (Regime 2) | WC-EXT 5 (Regime 1) |
|---|---|---|---|
| Fixture test wallclock | 0.29s | 2.85s | **0.21s** |
| 5-endpoint TLS probe | ~37s | ~39s | ~36s |

Fixture verify is ~28% faster than Jacobian-only. The remaining time on a single verify is dominated by the u2·Q half (variable-input, no precomputation possible without wNAF or similar).

### §XV.g.f Pred-731.XV.g.3 corroborated

The amendment predicted: "the regime-promotion move (Regime 2 → Regime 1 when amortization is insufficient) is bounded in complexity: it requires a build.rs script or a one-shot offline computation, not a redesign." Confirmed:

- Generator: 65 LOC (`examples/gen_p256_base_table.rs`).
- Generated source: 1047 lines but mechanically produced; commits as a single artifact.
- Module integration: 3-line change in lib.rs (`mod p256_base_table;` + replace OnceLock init).
- Routing change: 5-line ECDSA-verify dispatch.

Total substrate work: ~80 LOC of human-written code, well under the §XV.g.f Pred-XV.g.3 expected ceiling. The framework's regime-promotion shape is structurally cheap once the underlying primitive (Jacobian-coordinate scalar mul) is in place.

### Commits

| commit | tag | recognition |
|---|---|---|
| (this commit) | `Ω.5.P06.E3.wc-baked-base-table` | Regime 1 bake for P-256 base table; fixture verify 0.29s → 0.21s; Pred-731.XV.g.3 corroborated; init cost 2.85s → 100ms |

### Probe result

5/5 unchanged at 3/5 PASS. E2 + E5 remain as their separate cases (httpbin's mid-handshake bug; npm's TLS-1.2-only endpoint policy).

### What this round confirms about §XV.g

The build-time-vs-init-time distinction is empirically load-bearing. The same substrate move under different regimes produced wallclock differences of 10× in opposite directions (WC-EXT 4 was 10× slower than baseline; WC-EXT 5 is ~28% faster than baseline at the fixture-test scale, and the gap will widen with longer-running processes that do more verifies).

The keeper's conjecture ("we will run into other optimizations that have this same form") now has one engagement-tier corroboration. Each future precomputed-table optimization (AES T-tables when the key is reused, Poly1305 multiplication tables, RSA Montgomery tables per key) admits the same Regime 1 / Regime 2 choice, and the empirical break-even-count will need to be measured per primitive.

### Open scope at WC-EXT 5 boundary

1. **WC-EXT 6**: wNAF window scalar mul for the u2·Q (variable-input) half. Expected ~2× speedup on that half, ~30% on total verify.
2. **WC-EXT 7+**: extend baked-table approach to P-384 and P-521 base points if needed. Symmetric work.
3. **TLS-EXT 9** (separate workstream): investigate E2 httpbin CloseNotify-mid-handshake.
4. **Keeper-decision still open**: E5 npm scope.

---

*WC-EXT 5 closes with the Doc 731 §XV.g.f Pred-731.XV.g.3 substrate-cost prediction corroborated and the §XV.c-true optimization realized. The build-time-bake regime is operational at this tier; the framework now has the build-vs-init distinction empirically grounded at one substrate-tier instance.*
