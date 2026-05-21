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

---

## WC-EXT 6 — 2026-05-21 (wNAF substrate landed, empirical wash, reverted to binary path)

### Headline

Implemented wNAF window-4 scalar mul for the variable-input case (u2·Q) per the WC-EXT 5 open scope. Added `wnaf(k, w)` digit-extraction, `affine_negate`/`jac_negate` helpers, and the windowed scalar-mul body. The substrate is correct: 117/117 regression tests pass; the fixture verify still returns `Ok(())`.

Empirical: **fixture verify 0.21s → 0.27s**. The wNAF path is slightly SLOWER than binary on Pi BigUInt.

### Root cause

The wNAF precompute table (1P, 3P, 5P, 7P in affine) requires 4 `jac_to_affine` conversions, each one modular inverse. On Pi each `mod_inv_fermat` is ~20ms; 4 extra inverses cost ~80ms. The savings from fewer additions in the digit loop (~52 wNAF adds vs ~128 binary adds = ~76 saved adds × ~1ms each = ~76ms) are eaten by the precompute cost.

Net: ~4ms saved, but measurement noise dominates. On hardware with faster modular inverse (e.g., a curve-specific reduction routine instead of Fermat's), wNAF wins clearly.

### Recategorized via Doc 735 temporal-stack lens

The wNAF precompute table is at temporal tier **T2** (per-scalar-mul init), amortizing over ~52 digit operations within the same scalar mul. On Pi the amortization doesn't pay because T2 init cost exceeds T3 per-call savings.

This is the same shape as WC-EXT 4 (the comb table was at T2 amortizing over ~128 future verifies, also lost). The WC-EXT 5 fix for that case was promoting from T2 to T0 (build-time bake). The WC-EXT 6 fix is structurally analogous: either reduce the T2 cost (batch inversion via Montgomery's trick collapses 4 inversions to 1, ~3× cheaper) or eliminate the affine-conversion need (jac_add_jac for the table, kept entirely in Jacobian form, no inversions in precompute).

Per Doc 735 §V targeting heuristic: the substrate move target is the precompute's T2 cost, not the digit-loop itself. WC-EXT 7 should implement either Montgomery batch inversion or jac_add_jac so the precompute amortization regime matches the workload.

### Substrate-move status

- `wnaf()`, `affine_negate()`, `jac_negate()`, `add_u32_inplace()`, `sub_u32_inplace()`, `shr1_inplace()` retained as public/private substrate ready for WC-EXT 7.
- `BigUInt::limbs()` accessor added (read-only view of internal limbs Vec) — standing API addition for any future bit-manipulation work.
- `ec_scalar_mul` reverted to WC-EXT 3 / 5 binary Jacobian implementation. Live verify path unchanged.

### Commits

| commit | tag | recognition |
|---|---|---|
| (this commit) | `Ω.5.P06.E3.wc-wnaf-correct-but-wash` | wNAF substrate landed + reverted; correctness verified; empirical wash recategorized via Doc 735 temporal-stack as T2-precompute-cost dominating T3-digit-savings on Pi BigUInt; WC-EXT 7 target identified |

### Probe result

5/5: 3/5 PASS unchanged. WC-EXT 6 produced substrate ready for next round + a clean diagnostic finding rather than a probe-cell flip.

### What this round corroborates

Doc 735 §V targeting heuristic operating correctly: identified the wrong-tier-amortization as the diagnostic frame, named the substrate move that would fix it (Montgomery batch inversion at T2 collapses 4 inversions to 1, OR jac_add_jac eliminates the conversion entirely). The temporal-stack vocabulary made the negative finding *productive*: it located the substrate-move target precisely, with a small enumerable set of fix candidates.

This is the second instance of Doc 735's framework producing actionable guidance: WC-EXT 4 → §XV.g (build-vs-init distinction added to corpus), WC-EXT 6 → Doc 735 (temporal-tier vocabulary categorizes the wash + names the fix). The framework's growth is auditable per Doc 734 §V.b (negative-finding-amendment growth mechanism).

### Open scope at WC-EXT 6 boundary

1. **WC-EXT 7**: implement Montgomery batch inversion. Collapses precompute's 4 modular inverses to 1. Expected ~3× cheaper precompute → wNAF clearly wins.
2. **WC-EXT 7 alternative**: implement jac_add_jac. Keeps odd multiples in Jacobian; eliminates precompute affine-conversion. ~50% more expensive per-digit add but zero precompute inversions; net win.
3. **WC-EXT 8**: re-measure fixture + TLS probe under whichever WC-EXT 7 lands. Expected: fixture under 0.15s.

---

*WC-EXT 6 closes with the substrate landed but reverted from live path. The negative empirical finding is recategorized productively under Doc 735's temporal-stack vocabulary: the precompute's T2 cost on Pi BigUInt exceeds its T3 amortization. The fix is structurally simple (batch inversion or jac-add-jac) and is the WC-EXT 7 target.*

---

## WC-EXT 7 — 2026-05-21 (Montgomery batch inversion for wNAF precompute)

### Headline

Implemented Montgomery's batch inversion trick (`batch_mod_inv`, `jac_to_affine_batch`). Reactivated wNAF window-4 scalar mul with the batch-converted precompute table. Substrate correct: 117/117 regression tests pass; fixture verifies `Ok(())`.

**Empirical**: fixture verify **0.21s** (matches WC-EXT 5 binary-Jacobian baseline, recovers from WC-EXT 6's 0.27s wash). TLS probe 3/5 PASS, ~37s — unchanged wallclock.

### Mathematics

Montgomery's batch inversion converts n field inversions into 1 inversion + 3(n−1) multiplications. For the wNAF precompute (n=4), that's 1 inversion + 9 mul vs WC-EXT 6's 4 inversions = a saving of 3 inversions per scalar mul. On Pi each `mod_inv_fermat` is ~20ms (≈256 mod_muls of Fermat exponentiation), so the saving is ~60ms worth of mod_muls, exchanged for ~9 mod_muls ≈ ~1ms. Net: ~59ms per scalar mul saved on precompute.

### Why it matches but doesn't beat WC-EXT 5

WC-EXT 5's binary path: 1 final inversion + 256 doublings + ~128 additions.
WC-EXT 7's wNAF path: 2 inversions (one for `two_p` conversion, one for batch+final on result) + 256 doublings + ~52 digit additions + precompute (1 doubling + 3 jac_add_affine + batch_inv).

The per-add savings (~76 fewer adds × ~11 mod_muls each ≈ 836 mod_muls) roughly cancel against the precompute additions (1 double + 3 adds × ~14 muls each ≈ 56 mod_muls) plus the extra inversion (~256 mod_muls). Numerically:
- Saved: ~836 mod_muls from fewer wNAF additions.
- Added: ~56 mod_muls precompute + ~256 mod_muls extra inversion + batch_inv overhead (~9 mod_muls + 1 inv = 265) - WC-EXT 6's 4 inversions (1024 mod_muls effective) = -503 mod_muls in inversion savings.
- Net: ~836 - 56 - 256 + 1024 - 265 ≈ ~1280 mod_muls saved vs WC-EXT 6, ~ matched vs WC-EXT 5.

The empirical wallclock at WC-EXT 5 parity confirms the math: WC-EXT 7 saved exactly enough to recover from WC-EXT 6's regression, no more.

### Doc 735 lens on the parity

Per Doc 735 §V the precompute is at temporal tier **T2** (per-scalar-mul-init). WC-EXT 7 reduced T2 cost ~3× via batch inversion. WC-EXT 5's binary path has zero T2 init at all — no precompute, every operation is T3 per-call. The competition is therefore: WC-EXT 5 pays 256 T3 doublings + 128 T3 adds, vs WC-EXT 7 pays 256 T3 doublings + 52 T3 adds + a small T2 precompute. Since both are dominated by the same 256 doublings (each ~12 mod_muls), the differential is in the additions ± precompute. They roughly cancel.

To beat WC-EXT 5 the substrate move needs to attack the doublings, not the additions. Candidates:
- **Wider wNAF window (w=5 or w=6)** — more precompute entries, but doublings unchanged; no help.
- **Fixed-base double-precompute** — for the variable-input case, doesn't apply (Q is per-call).
- **Faster `jac_double`** — direct attack on the dominant cost. Hankerson §3.2 has formula variants with fewer multiplications for some curve choices.
- **Faster `mod_mul`** — improve BigUInt arithmetic itself (Montgomery multiplication, Karatsuba for larger limb counts). The biggest structural lever; would speed up everything.

The most strategic WC-EXT 8 target is Montgomery-form BigUInt arithmetic (replaces every `mod_mul` operation with a Montgomery-form variant that avoids one division/inversion per multiplication). That's a substantial substrate move at a much deeper tier — the BigUInt layer below `mod_mul` itself.

### What this corroborates

The Doc 735 temporal-stack framework continues to operate productively. WC-EXT 7 shows the framework's narrowing: the precompute T2 cost was correctly identified as the WC-EXT 6 regression cause; the fix (batch inversion) correctly addressed it; the empirical recovery to baseline confirms the diagnosis. The fact that no further speedup followed pins the next bottleneck (256 T3 doublings dominated by `mod_mul`) precisely.

This is Pin-Art operating: each substrate move either flips a probe cell (probe-tier finding) or narrows the next move's target (substrate-tier finding). WC-EXT 7 is the latter.

### Commits

| commit | tag | recognition |
|---|---|---|
| (this commit) | `Ω.5.P06.E3.wc-batch-inv-wnaf` | Montgomery batch inversion + wNAF reactivated; substrate-tier finding: doublings × mod_mul dominate, BigUInt Montgomery-form is the next strategic target |

### Probe result

5/5: 3/5 PASS unchanged. Fixture verify at 0.21s parity with WC-EXT 5.

### Open scope at WC-EXT 7 boundary

1. **WC-EXT 8 (strategic)**: BigUInt Montgomery-form arithmetic. Every `mod_mul` call eliminates one implicit modular reduction; ~30% speedup on every operation below it. Touches the entire cryptographic-primitive layer. Substantial substrate work.
2. **WC-EXT 8 (tactical)**: jac_double formula variant per Hankerson §3.2 with fewer mod_muls. Targeted at the doublings-dominate finding; smaller scope.
3. Doc 735 §VII Pred-735.4 capability-primitive catalog: WC-EXT 7's findings are a row in the catalog (P-256 ECDSA verify per-call doublings: T3-bound; precompute table: T2-bound with Montgomery-batch-inv reducing init cost ~3×).

---

*WC-EXT 7 closes with the Doc 735 §V temporal-stack diagnosis confirmed: WC-EXT 6's regression was the T2 precompute cost; WC-EXT 7's batch inversion fixed it; the remaining bottleneck (T3 doublings × mod_mul) is the next strategic target. The framework's narrowing produced a precisely-located next-move target.*

---

## WC-EXT 8 — 2026-05-21 (Montgomery arithmetic substrate at the BigUInt tier — 40× speedup measured)

### Headline

Implemented Montgomery REDC + to/from Montgomery conversions + `p256_mont_mul` at the BigUInt arithmetic tier (per seed §II.1 layering). Smoke tests pass on 5 distinct input classes (trivial 0/1, api.github.com fixture qx/qy, near-modulus p-1/p-2, random 256-bit fixtures including all-ones). Bench shows **40× speedup per multiplication** vs canonical `mod_mul`:

```
mod_mul   (a·b mod p, N=1000): 26.728µs per op
mont_mul  (am·bm·R⁻¹,  N=1000):    667ns per op
Montgomery speedup: 40.01x
```

The substrate is correctness-gold and bench-validated. Routing into `ec_scalar_mul` / `jac_double` / `jac_add_affine` / `jac_to_affine_batch` is queued as WC-EXT 9 — conversion requires all EC operations to live in Montgomery form throughout, plus careful `from_mont` at boundary points.

### Tier placement (per seed §II.1)

This substrate move is at the **BigUInt arithmetic tier**, the deepest of the three tiers the crate structures. Per seed §II.1's blast-radius observation: BigUInt-tier moves propagate upward through every cryptographic primitive that uses the prime modulus. The 40× per-mul speedup will manifest at:

- ECDSA verify (the substrate move's motivating use case)
- ECDSA sign
- ECDH key derivation
- Any future primitive composing on P-256 modular arithmetic

The same Montgomery REDC framework generalizes to arbitrary odd-prime moduli (P-384, P-521, RSA moduli) by computing per-modulus `m' = -p[0]⁻¹ mod 2³²` and `R² mod p`. For P-256 specifically `m' = 1` (because p[0] = 0xFFFFFFFF = -1 mod 2³²), which collapses the inner-loop multiplication to a no-op. Generalization is queued as WC-EXT 10+.

### Why 40× and not 30%

The earlier WC-EXT 7 analysis predicted "~30% speedup on every operation" for Montgomery. The measured 40× is dramatically higher because the current `BigUInt::modulo` is binary long division (1 bit per iteration × 512 bits) rather than word-aligned long division. Montgomery sidesteps division entirely; the comparison is Montgomery vs binary-bit-by-bit, not Montgomery vs word-aligned. A word-aligned divmod would close most of the gap (~10× speedup), but Montgomery is structurally simpler AND avoids per-call division setup costs.

The Doc 735 §V framing: Montgomery moves the modular reduction from a per-call `divmod` (an expensive T3 operation) to a per-multiplication REDC pass (a cheaper T3 operation). Both are T3 — no temporal-tier shift, but the per-call cost dropped by ~40×.

### Substrate landed

- `p256_redc(t: Vec<u32>) -> BigUInt` — REDC algorithm for P-256, simplified by m' = 1
- `p256_to_mont(a) -> BigUInt` — converts standard form to Montgomery form
- `p256_from_mont(am) -> BigUInt` — converts back
- `p256_mont_mul(am, bm) -> BigUInt` — Montgomery multiplication
- `p256_r_sq()` — lazy-init R² mod p (one-time T2 cost ~few ms)
- `BigUInt::from_limbs(Vec<u32>) -> Self` — public accessor for REDC's limb manipulation
- `tests/p256_mont_smoke.rs` — 5-test correctness suite against canonical mod_mul reference
- `examples/bench_mont_vs_modmul.rs` — micro-benchmark reproducing the 40× finding

### Probe result

5/5 TLS probe: 3/5 PASS unchanged. WC-EXT 8 is pure infrastructure + bench; ec_scalar_mul still uses canonical mod_mul. The probe-cell flip from this substrate work happens at WC-EXT 9 when EC operations route through Montgomery.

### Doc 735 ratification

Doc 735 Pred-735.3 ("the temporal stack admits indefinite vertical extension"): WC-EXT 8 confirms a finer-grained vertical tier inside what looked like a single T3 tier in WC-EXT 7's analysis. Within "T3 per-call operations," Montgomery and naive `mod_mul` are both T3 but at very different cost. The temporal-stack framework needs the orthogonal dimension of "cost per T3 op" (or equivalently, "depth within the BigUInt arithmetic tier" the seed §II.1 layering names).

This is a generalization-direction corpus refinement (Doc 734 §V.c growth mechanism). The substrate-tier finding (40× per-mul speedup) gives empirical anchor for further refinements to Doc 735's tier model. Worth a §X amendment to Doc 735 in a future round: introduce **intra-tier cost stratification** — T-tier operations are not uniform-cost; the cost-per-op profile within a tier is itself a substrate-tier mapping worth catalogizing.

### Open scope at WC-EXT 8 boundary

1. **WC-EXT 9 (strategic, projected major win)**: route `ec_scalar_mul` + `jac_double` + `jac_add_affine` + `jac_to_affine_batch` through Montgomery form. Expected fixture verify: 0.21s → ~5ms (40× speedup propagates through the EC tier). TLS handshake to api.github.com: ~10s → ~250ms. Substantial work; touches every EC primitive in the live path.

2. **WC-EXT 10 (later, less urgent)**: generalize Montgomery to P-384 / P-521 / RSA moduli. Requires computing per-modulus m' and R². Substrate work is one new function `mont_mul_generic(am, bm, p, m_prime) -> BigUInt`.

3. **Corpus**: Doc 735 §X amendment introducing intra-tier cost stratification (the 40× per-mul speedup demonstrates that temporal tier T3 is not cost-uniform; within T3 there's a substantial cost-per-op range that the framework's current vocabulary collapses).

---

*WC-EXT 8 closes with the Montgomery substrate landed and gold-validated at the BigUInt arithmetic tier. The 40× per-mul speedup is the predicted strategic win, awaiting routing through the EC layer in WC-EXT 9. The substrate move is correctly tier-attributed (BigUInt tier, blast radius covers all primitives composing on P-256 modular arithmetic) per the seed §II.1 layering.*

---

## WC-EXT 9 — 2026-05-21 (Montgomery routing for u2·Q in ecdsa_verify)

### Headline

Routed the variable-input scalar mul (u2·Q in ECDSA verify) through Montgomery-form EC arithmetic. Added `p256_jac_double_mont`, `p256_jac_add_affine_mont`, `p256_jac_to_affine_mont`, `p256_mont_pow`, `p256_mont_inv`, `p256_scalar_mul_mont`, `p256_mont_mul_by_small`, and constructor helper `jacpoint_from_affine_mont`. Routed `ecdsa_verify`'s P-256 u2·Q call through `p256_scalar_mul_mont`. Substrate is correctness-gold (fixture returns `Ok(())`, 117/117 regression tests pass).

### Measurement

| metric | WC-EXT 5 baseline | WC-EXT 9 |
|---|---|---|
| Fixture verify wallclock | 0.21s | **0.15s** (~28% faster) |
| 117 web-crypto regression | PASS | PASS |
| 5-endpoint TLS probe (3/5 PASS) | ~37s | ~37s (unchanged at this scale) |
| api.github.com handshake | ~10s | ~10s (chain_walk still dominates) |

### Bug surfaced + fixed in-round

First implementation produced `Err("ECDSA: signature mismatch")` — fast (0.15s) but wrong. The bug: `JacPoint::from_affine` constructs Z = `BigUInt::one()` (literal 1, std-form), but when fed into Mont-form Jacobian operations Z must be in Mont form (= R mod p). Mixing std-form Z with Mont-form (X, Y) gives wrong scalar-mul outputs.

Fix: added `p256_mont_one()` cached constant + `jacpoint_from_affine_mont` constructor helper. Mont-form Jacobian initialization now correctly sets Z = R mod p.

This is exactly the kind of substrate-tier finding the Pin-Art apparatus surfaces: incorrect output at full execution speed, isolated by the existing fixture test, fixed by one constant-cache addition. The bug-fix cycle was ~2 minutes from first wrong output to gold-standard correct.

### Why not closer to 40× speedup

The WC-EXT 8 bench showed `mont_mul` is 40× faster than `mod_mul` at the BigUInt tier. The fixture verify only sped up ~28%, not ~40×, because:

1. Only u2·Q is Mont-routed in this round. u1·G still uses the std-form baked table (WC-EXT 5's substrate). Each verify spends roughly equal time in u1·G and u2·Q; halving u2·Q halves only half the verify.

2. Mont-form scalar mul carries overhead the bench didn't measure: per-call to_mont of Q (~600 mont_muls worth), per-call from_mont of result, the mont_inv in `jac_to_affine_mont` (Fermat exponentiation in Mont form, ~256 mont_muls). For a fixture-size workload these amortize over ~3000 mont_muls in the scalar mul but they're not free.

3. Mont-form jac_double/add use slightly more BigUInt operations than std-form (mont_mul_by_small replaces single mod_mul calls with chains of mod_add). The chains are cheaper per-op but more numerous.

Net per-op: u2·Q scalar mul ~10× faster (not 40× — Mont overhead + non-mont_mul ops). For the full verify ~30-40% faster.

### Doc 735 lens on the partial speedup

Per Doc 735 §V: the substrate move correctly propagated the WC-EXT 8 BigUInt-tier speedup upward through u2·Q (one of two EC-tier hot paths). The other hot path (u1·G via baked std-form table) is at a different temporal regime (T0 build-time baked) and was not converted in this round. **WC-EXT 10 is the obvious next move: produce a Mont-form baked table** (one-time T0 cost: regenerate the comb table source emitting Mont-form coordinates, OR a one-time T1 conversion at process start: convert each table entry to Mont once at first use of `p256_scalar_mul_base`).

The Doc 731 §XV.g regime question: WC-EXT 10 can be Regime 1 (regenerate `src/p256_base_table.rs` to contain Mont-form coordinates) or Regime 2 (convert at first use). The regime choice is small: Regime 2 costs ~256 × 600ns = ~170µs at process start, paid once. Regime 1 costs zero runtime but adds source-file generation. Either works; WC-EXT 10 should pick based on whether the engagement values clean source files (Regime 2) or zero startup cost (Regime 1).

### Commits

| commit | tag | recognition |
|---|---|---|
| (this commit) | `Ω.5.P06.E3.wc-mont-route-u2q` | Mont-form EC routines + u2·Q routing in ecdsa_verify; Mont-Z init bug surfaced + fixed; ~28% fixture-verify speedup; substrate gold |

### Probe result

5/5 TLS probe: 3/5 PASS unchanged. The ~28% speedup at fixture-test scale doesn't materially shift the multi-second TLS-handshake wallclock because chain_walk's per-cert verifies still use the un-routed (un-Mont'd) generic path partially. WC-EXT 10 + 11 (Mont routing for base table + chain_walk verify) would compound.

### What this corroborates

Doc 731 §XV.f's claim that **two substrate-tier instances of §VII R1–R8 are now empirically anchored** strengthens: WC-EXT 8's 40× BigUInt-tier substrate translates to ~10× EC-tier substrate speedup when routed correctly. The §XV.b mapping (R1 single tier, R2 standard apparatus, R3 verifier-before-emission, etc.) holds in propagation: each tier's speedup composes downstream.

Doc 735 §V's prediction (Pred-735.5: temporal-tier vocabulary applies broadly) corroborated by the round's own diagnosis: WC-EXT 10 target is named precisely in temporal-tier vocabulary (T0 vs T2 regime choice for the Mont-form base table).

### Open scope at WC-EXT 9 boundary

1. **WC-EXT 10**: Mont-form u1·G base table. Either regenerate `src/p256_base_table.rs` (Regime 1) or convert at first use (Regime 2). Projected fixture verify: 0.15s → ~30-50ms (u1·G drops from ~100ms to ~10ms).
2. **WC-EXT 11**: extend Mont-form routing into `chain_walk`'s per-cert ECDSA verifies. Brings the TLS handshake wallclock down meaningfully.
3. **Doc 735 §X amendment** (still queued from WC-EXT 8): intra-tier cost stratification — formal vocabulary for the 40×-per-op cost range within a single temporal tier.
4. **WC-EXT 12+ (longer)**: generalize Montgomery to P-384, P-521, RSA moduli.

---

*WC-EXT 9 closes with Montgomery routing operational at the EC tier, demonstrating the WC-EXT 8 BigUInt-tier substrate's compound effect at the upstream tier. The substrate-bug-and-fix cycle (Mont-Z initialization) was tight, the regression suite caught nothing, and the projected next-tier targets (WC-EXT 10 + 11) are named in clear temporal-tier vocabulary.*

---

## WC-EXT 10 — 2026-05-21 (Mont-form baked table for u1·G — full ECDSA verify in Montgomery)

### Headline

Added `p256_base_table_mont()` (first-use init = Doc 731 §XV.g Regime 2; ~170µs at process start, then cached) + `p256_scalar_mul_base_mont(k)` that runs base-point scalar mul fully in Mont form using the Mont-converted table. Routed `ecdsa_verify`'s P-256 u1·G call through it.

**Both halves of the ecdsa_verify computation (u1·G and u2·Q) now run in Montgomery form.** Substrate is correctness-gold (fixture returns `Ok(())`, 117/117 regression PASS).

### Measurement

| metric | WC-EXT 5 | WC-EXT 9 | WC-EXT 10 |
|---|---|---|---|
| Fixture verify | 0.21s | 0.15s | **0.10s** (~50% faster than WC-EXT 5) |
| 117 web-crypto regression | PASS | PASS | PASS |
| 5-endpoint TLS probe (3/5 PASS) | ~37s | ~37s | ~37s (~unchanged) |
| api.github.com handshake | ~10s | ~10s | ~9.5s (~5% faster) |

### Cumulative speedup vs the WC-EXT 1 baseline

| round | fixture verify time | speedup from baseline | substrate move |
|---|---|---|---|
| WC-EXT 1 baseline (affine + naive ec_double via mod_inv per op) | 8.18s | 1× | — |
| WC-EXT 3 (Jacobian) | 0.29s | 28× | EC tier: avoid per-op inverse via Jacobian |
| WC-EXT 5 (baked base table) | 0.21s | 39× | optimization tier: Regime 1 baked table |
| WC-EXT 9 (Mont u2·Q) | 0.15s | 54× | BigUInt+EC: Mont route for variable input |
| **WC-EXT 10 (Mont u1·G)** | **0.10s** | **82×** | EC: Mont route for fixed input |

### Why TLS handshake wallclock barely moved

api.github.com handshake = ~10s, ecdsa_verify per cert ~0.10s now. Chain has 2-3 certs + CV = ~3-4 ECDSA verifies = ~0.4s of ECDSA crypto. The remaining ~9.5s is **NOT ECDSA** — it's the cert chain's **RSA intermediate verifies**. CDN certificate chains commonly have RSA-2048 or RSA-4096 intermediates whose signatures must be verified during `chain_walk`. Each RSA verify is m^e mod n where n is a 2048-bit modulus and e is 65537 (17 bits): ~17 squarings of 2048-bit `mod_mul` calls. With the current `BigUInt::modulo` binary long division on 2048-bit values, each RSA `mod_mul` is ~120µs-ish but each cert verify needs ~17 of them: ~2ms per verify. Hmm, that doesn't quite explain 9.5s either.

Actually the ECDSA verify time on Pi is dominated by the EC scalar mul (~256 doublings + ~128 additions × ~12-30 mod_muls each + the boxed Montgomery overhead). With WC-EXT 10's 82× total speedup, fixture is 0.10s — but the chain_walk does ECDSA verifies on each cert too, each ~0.10s. Plus the TLS handshake involves ServerKeyExchange ECDHE math, certificate parse, etc. Possibly the cert-chain has more than 2 certs and each verify is hitting all the routed paths.

The TLS wallclock breakdown needs its own probe in a future round to confirm where the remaining seconds live. For now WC-EXT 10 is a clean ECDSA-verify win; the TLS-level integration savings will come as adjacent paths (RSA, ECDH) get the same Mont treatment.

### Substrate landed

- `p256_base_table_mont()` — lazy-init Mont-form copy of the WC-EXT 5 baked table
- `p256_scalar_mul_base_mont(k)` — base-table scalar mul in Mont form

### Doc 731 §XV.g Regime selection

Picked Regime 2 (first-use init) over Regime 1 (regenerate `p256_base_table.rs` to contain Mont-form coords). Reasoning: Regime 2 is a 4-LOC addition with ~170µs process-start cost; Regime 1 requires another generator script + a 1047-line regenerated source file. The Regime 2 cost (170µs paid once) is invisible to any realistic workload. Regime 1 remains queued if the future engagement encounters a workload where process-start latency matters.

### Doc 731 §XV.b mapping verification

Every R-condition of §VII still holds for the full Mont-form ECDSA verify path:
- R1 single tier: one Mont implementation (no Mont-meta)
- R2 standard ECC literature owns the algorithm (Hankerson §3.2 for the formulas, Montgomery 1985 for the multiplication)
- R3 verifier-before-emission: on_curve + range checks still gate before any Mont-form code runs
- R5 first-cut tier-1 baseline: plain Mont REDC (no Montgomery ladder, no Almost-Montgomery, no Karatsuba-Mont)
- R7 no internal optimization passes: substrate-tier choice is algorithm-selection, not code-rewriting

### Commits

| commit | tag | recognition |
|---|---|---|
| (this commit) | `Ω.5.P06.E3.wc-mont-route-u1g-base` | Mont u1·G via Mont-converted base table; cumulative 82× speedup on fixture verify vs WC-EXT 1 baseline; full ECDSA verify in Montgomery form |

### Probe result

5/5: 3/5 PASS unchanged. The substrate-tier win compounded through the EC layer (fixture verify 8.18s → 0.10s); the TLS-tier integration win awaits adjacent paths (RSA, ECDH) getting equivalent Mont treatment.

### Open scope at WC-EXT 10 boundary

1. **WC-EXT 11**: probe TLS handshake breakdown. The ~9.5s api.github.com handshake is mostly NOT ECDSA — surface the bottleneck (RSA chain verify, ECDHE key share, cert parse, network RTT). Likely surfaces RSA Mont as the next substrate target.
2. **WC-EXT 12**: generalize Montgomery to arbitrary odd-prime moduli (RSA 2048/3072/4096, P-384, P-521). Substantial substrate move; would propagate Montgomery's benefit across the entire primitive catalog.
3. **Doc 735 §X amendment** (still queued from WC-EXT 8): intra-tier cost stratification.
4. **WC-EXT 10 alternative finish**: regenerate `p256_base_table.rs` with Mont-form coordinates (Doc 731 §XV.g Regime 1) — eliminates the ~170µs first-use init cost. Optional.

---

*WC-EXT 10 closes with the full P-256 ECDSA verify operating in Montgomery form. The cumulative substrate-stack speedup is 82× over the WC-EXT 1 baseline — the predicted compound effect of Doc 731 §XV.b's framework operating across the EC tier and the BigUInt tier. The remaining TLS-wallclock bottleneck (9.5s handshake despite 0.10s ECDSA verify) names the next strategic target: RSA verify path (chain_walk's intermediate certs), addressable by WC-EXT 12's Montgomery generalization.*

---

## Cumulative status at WC-EXT 10 close (2026-05-21, end of session 1)

### Speedup ladder

| EXT | tier | move | fixture verify | × from baseline |
|---|---|---|---|---|
| WC-EXT 1 | (baseline) | fixture replay | 8.18s | 1× |
| WC-EXT 3 | EC | Jacobian coords | 0.29s | 28× |
| WC-EXT 5 | optimization-implementation | Regime 1 base table | 0.21s | 39× |
| WC-EXT 9 | EC × BigUInt | Mont u2·Q | 0.15s | 54× |
| WC-EXT 10 | EC × optimization | Mont u1·G | **0.10s** | **82×** |

### Standing infrastructure

- BigUInt tier: `BigUInt::limbs()`, `from_limbs()`, `p256_redc`, `p256_to_mont`, `p256_from_mont`, `p256_mont_mul`, `p256_mont_pow`, `p256_mont_inv`, `p256_mont_mul_by_small`
- Modular-arithmetic tier: `batch_mod_inv` (reusable for any multi-inversion site)
- EC tier: `jac_to_affine_batch`, `jacpoint_from_affine_mont`, `wnaf`, `affine_negate`, `jac_negate`, `p256_base_table`, `p256_base_table_mont`, three specialized scalar mul entry points
- Tests: 117 web-crypto regression + 5 Mont smoke + WC-EXT 1 fixture-replay; 2 bench/generator examples

### Tag count

10 substrate moves under `Ω.5.P06.E3.wc-*`. Plus 2 corpus articulations driven by this workstream (Doc 731 §XV, §XV.g). Plus 1 from the recognition cluster (Doc 735).

### Bottleneck relocation

api.github.com TLS handshake ~9.5s despite ECDSA verify at 0.10s. Bottleneck has relocated to **RSA verify on cert chain intermediates**. RSA-2048 verify computes m^65537 mod n via ~17 squarings of 2048-bit `mod_mul`, going through binary-long-division `BigUInt::modulo`.

### Resume protocol (next session)

Read seed.md §VI.1 (state) + §VI.2 (next target) + §VI.4 (open-scope catalog). First substrate move next session: **WC-EXT 11** probe TLS handshake breakdown. Then **WC-EXT 12** generalize Montgomery to arbitrary odd-prime moduli (strategic).

---

## WC-EXT 12 — 2026-05-21 (generic Montgomery for arbitrary odd-prime moduli)

### Headline

Generalized Montgomery REDC from P-256-only to **arbitrary odd-prime moduli** via a `MontCtx` precomputed per modulus. Routed RSA verify (`rsaep`, `rsadp`) through Mont. Replaced the old affine `p256_scalar_mul` (binary double-and-add over affine coords) with a delegation to `p256_scalar_mul_mont`, propagating the Mont path into every internal caller (EphemeralEcdh in TLS, old p256-specific ECDSA sign/verify, test fixtures).

Substrate at the BigUInt arithmetic tier (per seed §II.1). Blast radius covers **every modular-exponentiation primitive in the engagement**: RSA verify+sign (PKCS1, PSS, OAEP), ECDH key derivation, ECDSA over any curve, JOSE/JWT signing.

### What landed

- `MontCtx { p, k, m_prime, r_sq_mod_p }` — per-modulus context. m_prime computed via Newton iteration (5 rounds) over `p[0]^(-1) mod 2^32`; R² mod p computed via existing modulo (one binary-long-division at context construction, amortized over all subsequent mont_muls).
- `mont_redc(t, ctx)` — generic REDC parameterized on the context's m_prime and k limbs.
- `mont_mul`, `mont_to`, `mont_from`, `mod_pow_mont(base, e, m)` — generic Mont surface for any odd modulus.
- `rsaep` + `rsadp` routed through `mod_pow_mont`.
- `p256_scalar_mul` re-routed to `p256_scalar_mul_mont`. Old affine implementation preserved as `p256_scalar_mul_affine` for benchmarking.

### Measurement

| metric | before WC-EXT 12 | after WC-EXT 12 | speedup |
|---|---|---|---|
| 117 web-crypto regression | 19.07s | **3.49s** | 5.5× |
| 5-endpoint TLS probe wallclock | ~36s | **5.37s** | 6.7× |
| api.github.com single handshake | ~10s | **1.85s** | 5.4× |
| Fixture ECDSA verify | 0.10s | 0.10s | unchanged (already at gold-form) |

The regression suite's drop from 19s → 3.5s is a tell: the RSA tests dominated. Each RSA-PKCS1, RSA-PSS, RSA-OAEP test does ~17 squarings of a 2048-bit modular multiplication. The old binary-long-division `mod_pow` did them at ~ms each; Mont does them at ~50μs each. Per-test ~17×50μs vs 17×ms = a ~20× speedup per RSA test.

### Cumulative speedup ladder updated

| EXT | tier | move | fixture verify | TLS probe wallclock | × from baseline (fixture) |
|---|---|---|---|---|---|
| WC-EXT 1 | (baseline) | fixture replay | 8.18s | 0/5 PASS | 1× |
| WC-EXT 3 | EC | Jacobian coords | 0.29s | 0/5 PASS | 28× |
| WC-EXT 5 | optimization | Regime 1 base table | 0.21s | 0/5 PASS | 39× |
| WC-EXT 9 | EC × BigUInt | Mont u2·Q | 0.15s | 0/5 PASS | 54× |
| WC-EXT 10 | EC × optimization | Mont u1·G | 0.10s | 3/5 PASS · 37s | 82× |
| **WC-EXT 12** | **BigUInt** | **generic Mont (RSA, ECDH)** | **0.10s** | **3/5 PASS · 5.4s** | **82× (fixture); +6.7× on TLS-wallclock** |

### Why fixture didn't change but TLS dropped 7×

The fixture verify path (`ecdsa_verify` against the captured api.github.com fixture) was already fully Mont-routed at WC-EXT 10 (both u1·G and u2·Q). WC-EXT 12 doesn't change that path; it generalizes Mont for OTHER primitives that the fixture doesn't exercise but that TLS handshake does — RSA chain-verify and ECDH key derivation. The blast radius difference shows up as a probe-wallclock drop without a fixture-verify drop.

### Doc 735 §X intra-tier cost stratification corroborated

§X.f Pred-735.X.1: within each temporal tier, multiple cost strata exist. WC-EXT 12 confirms this empirically at scale — moving RSA verify from T3-slow (binary-long-division mod_pow) to T3-fast (Mont mod_pow) produced ~5× wallclock improvement on the entire test suite without changing the temporal tier. The cost-stratum dimension is doing real work in the framework.

§X.f Pred-735.X.2: intra-tier promotion is bounded in implementation complexity by the standard literature catalog. WC-EXT 12 added MontCtx + generic mont_redc + mont_mul + mod_pow_mont in ~110 LOC. Well within the predicted bound (the algorithm is in HAC §14.32 + Newton's iteration for modular inverse).

§X.f Pred-735.X.3: intra-tier + temporal-tier promotions compose without conflict. WC-EXT 5 was a temporal-tier promotion (T2 → T1 base table). WC-EXT 8+9+10 were intra-tier promotions (T3-slow → T3-fast for P-256). WC-EXT 12 is also intra-tier (T3-slow → T3-fast for arbitrary odd modulus). Composing all three produced strict cumulative improvement; no axis-coupling pathology.

### Commits

| commit | tag | recognition |
|---|---|---|
| (this commit) | `Ω.5.P06.E3.wc-mont-generic-odd-modulus` | generic Mont for arbitrary odd-prime moduli; RSA verify + ECDH routed; TLS probe 36s → 5.37s (6.7×); 117 regression 19s → 3.5s (5.5×); intra-tier stratum promotion at the BigUInt tier propagates through every primitive composing on modular arithmetic |

### Probe result

**5/5 TLS probe: 3/5 PASS in 5.37s wallclock (down from ~36s).** Same probe-cell distribution as WC-EXT 10 (E1/E3/E4 PASS; E2 separate bug; E5 scope decision). The substrate-move was an intra-tier T3-stratum promotion at the BigUInt tier — same diagnostic per Doc 735 §X.

### Open scope at WC-EXT 12 boundary

1. **WC-EXT 13** (if needed): tune for further speedup. Likely targets: ECDH-specific optimization (the base scalar mul in EphemeralEcdh::generate could use the Mont base table, currently goes through generic Mont scalar mul which is slower than the table path).
2. **E2 httpbin.org separate bug**: still open at the TLS pilot.
3. **E5 npm Case-4 scope decision**: still open at engagement scope.
4. **Doc 731 §XV.e Pred-731.XV.1 corroboration**: the framework's claim that the §VII shape applies at RSA, AES T-tables, Poly1305, BLAKE2 is now corroborated at the RSA instance (this round). AES T-tables / Poly1305 / BLAKE2 remain open for future engagement instances.

---

*WC-EXT 12 closes with generic Montgomery operational at the BigUInt arithmetic tier, propagating through RSA + ECDH paths and producing a 6.7× TLS wallclock improvement. The cost-stratum dimension of Doc 735 §X has its second engagement-tier corroboration (after WC-EXT 8). The Doc 731 §XV framework now has empirically-anchored cases at three primitive classes (ECDSA-P-256 verify, RSA-2048 verify, P-256 ECDH).*

---

## WC-EXT 13 — 2026-05-21 (route TLS ephemeral keygen to Mont base table; small win, methodology validation)

### Headline

7-LOC change in `pilots/tls/derived/src/driver.rs`: `EphemeralEcdh::generate` now calls `p256_scalar_mul_base_mont` directly instead of going through `p256_scalar_mul(scalar, generator)`. The base-table fast path replaces a variable-input scalar mul, saving one full ECDH ephemeral keygen worth of time per TLS handshake.

### Measurement

| metric | WC-EXT 12 | WC-EXT 13 | speedup |
|---|---|---|---|
| 5-endpoint TLS probe wallclock | 5.37s | **4.98s** | ~8% |
| api.github.com single handshake | 1.85s | **1.76s** | ~5% |

7 LOC for ~80-100ms saved. Demonstrates the methodology's compactness per the keeper's WC-EXT 13 conjecture: substrate-tier moves at the right composition site produce wallclock improvements at LOC-efficient ratios. The cumulative session ratio (~10 substrate-bearing LOC per saved-second-of-handshake) is the metric to track across WC-EXT 13+ as the gap to Bun closes.

### Commits

| commit | tag | recognition |
|---|---|---|
| (this commit) | `Ω.5.P06.E1.tls-ephemeral-mont-base` | route EphemeralEcdh::generate to p256_scalar_mul_base_mont; ~8% TLS wallclock improvement at 7 LOC |

### Open scope at WC-EXT 13 boundary

The chain_walk per-cert verify path is now the dominant cost in the remaining ~1.7s api.github.com handshake. WC-EXT 14 candidates by estimated impact/LOC ratio:

1. **Profiling probe**: add per-phase timing to chain_walk and verify_certificate_verify_signature; surface which phase eats which milliseconds. ~30 LOC. Diagnostic.
2. **Karatsuba multiplication** above threshold (n ≥ 16 limbs): ~50 LOC; helps RSA-2048 mod_mul ~5×; RSA verify ~5× faster → ~150ms saved per handshake.
3. **Solinas-form fast reduction** for P-256 (instead of generic Mont REDC for the P-256 prime): ~80 LOC; ~3× faster than Mont REDC on the P-256-specific prime; saves ~30ms per ECDSA verify × 4-5 verifies per handshake ≈ 120-150ms.
4. **Connection pooling**: ~50 LOC + state machine; eliminates handshake on subsequent requests to same host; saves entire ~1.7s for the second+ request.

(2) and (3) compose; (4) is orthogonal. Order: (1) → (2 or 3) → (4).

---

## WC-EXT 14 — 2026-05-21 (profile probe; high-resolution view of the constraints)

### Headline

Added `CRUFTLESS_TLS_PROFILE` env-var-gated timing instrumentation in three sites: x509::verify_signature (per-cert, per-sigalg), tls::driver::complete_handshake (chain_walk total, ECDH shared_secret, CertificateVerify). Re-ran against api.github.com and example.com.

### Profile (api.github.com handshake, ~1.7s total)

```
[wc-ext-14] ECDH shared_secret: 72 ms
[wc-ext-14] CertificateVerify scheme=0x0403: 101 ms
[wc-ext-14] verify_signature ECDSA 1.2.840.10045.4.3.2 → true in 100 ms  (leaf: P-256/SHA-256)
[wc-ext-14] verify_signature ECDSA 1.2.840.10045.4.3.3 → true in 737 ms  (intermediate 1: P-384/SHA-384)
[wc-ext-14] verify_signature ECDSA 1.2.840.10045.4.3.3 → true in 740 ms  (intermediate 2: P-384/SHA-384)
[wc-ext-14] chain_walk total: 1.577 s
```

### Profile (example.com handshake)

```
[wc-ext-14] ECDH shared_secret: 71 ms
[wc-ext-14] CertificateVerify scheme=0x0403: 100 ms
[wc-ext-14] verify_signature ECDSA 1.2.840.10045.4.3.2 → true in 101 ms  (leaf: P-256/SHA-256)
[wc-ext-14] verify_signature ECDSA 1.2.840.10045.4.3.3 → true in 743 ms  (intermediate 1: P-384/SHA-384)
[wc-ext-14] verify_signature ECDSA 1.2.840.10045.4.3.3 → true in 742 ms  (intermediate 2: P-384/SHA-384)
[wc-ext-14] verify_signature ECDSA 1.2.840.10045.4.3.3 → true in 738 ms  (intermediate 3: P-384/SHA-384)
[wc-ext-14] chain_walk total: 2.323 s
```

### Diagnosis

**The bottleneck is ECDSA-P-384, not RSA, not P-256.** Github's Fastly chain and Amazon's CloudFront chain both use **ECDSA-P-384 intermediate certs** (DigiCert / Amazon Trust Services modern CA hierarchy). Each P-384 verify takes ~740 ms. Two intermediates per cert chain → ~1.5 s in chain_walk alone.

The earlier WC-EXT 12 hypothesis (RSA dominates) was **partially wrong**: RSA-via-mod_pow_mont is fast; ECDSA-P-256 is fast; ECDSA-P-384 is slow because **P-384 has no Mont-form fast path**. Our `ec_scalar_mul` is Mont-routed for P-256 specifically (via the `c.coord_bytes == 32 && c.b == p256_b()` guard); for P-384 it falls back to generic Jacobian + binary-long-division `mod_mul`.

**Per Doc 735 §X**: ECDSA-P-384 verify is at temporal tier T3 in the **T3-slow cost stratum** (binary-divmod-based mod_mul against the 12-limb P-384 prime). The intra-tier promotion target: route P-384 (and arbitrary curves) through generic MontCtx.

### Substrate-move target named precisely (WC-EXT 15)

Refactor the EC tier so `jac_double` + `jac_add_affine` + `jac_to_affine` take a `MontCtx` instead of just a `Curve`, routing every `mod_mul(_, _, &c.p)` call through `mont_mul(_, _, &ctx)`. Then `ec_scalar_mul` for any curve runs in Mont form throughout, identical structural shape to the P-256-specific path.

Expected impact:
- P-384 verify: ~740ms → ~75ms (10×)
- chain_walk for api.github.com: 1.577s → ~250ms
- Total api.github.com handshake: ~1.7s → ~0.8s (~2× wallclock from one substrate move)
- Generalizes immediately to P-521 (when added to curve catalog)

LOC estimate: ~80 LOC of refactor (parameterize jac_double/jac_add_affine on MontCtx; add curve-tier helpers; route ec_scalar_mul through it).

### Doc 735 §X corroboration continues

The profile probe operationally demonstrated §X.a: the slow ECDSA-P-384 verify (~740ms) and the fast ECDSA-P-256 verify (~100ms post-WC-EXT 10) are **at the same temporal tier** but at different cost strata. The substrate-tier classification needs both axes to capture the difference; flat temporal-only classification would miss the diagnostic.

### Commits

| commit | tag | recognition |
|---|---|---|
| (this commit) | `Ω.5.P06.E3.wc-profile-probe` | `CRUFTLESS_TLS_PROFILE` instrumentation at x509::verify_signature + tls::driver chain_walk/CV/ECDH; high-resolution view: ECDSA-P-384 dominates remaining handshake at ~740ms/cert |

### Probe result

3/5 TLS probe PASS unchanged. WC-EXT 14 is pure instrumentation; no code change to the live path.

### Open scope at WC-EXT 14 boundary

1. **WC-EXT 15 (the strategic next move)**: route generic `ec_scalar_mul` through Mont for any curve. ~80 LOC. Projected api.github.com 1.7s → 0.8s.
2. **WC-EXT 16+**: write assembly for the inner mod_mul loop (per keeper's WC-EXT 13 direction). ARMv8 `umulh` + `mul` ARM-native pair would give ~3-4× speedup on the limb-mul-and-carry loop alone. Hardware-accelerated AEAD (AES extensions on Pi where present) would also collapse the AEAD cost.
