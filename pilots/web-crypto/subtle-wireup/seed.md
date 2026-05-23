# web-crypto/subtle-wireup — Resume Vector / Seed

**Locale tag**: `L.web-crypto/subtle-wireup` (nested per Doc 737 §IV)

**Status as of 2026-05-23**: **WORKSTREAM FOUNDED (SW-EXT 0)**. No code yet. Spawned per keeper directive 2026-05-23 18:12-local. Nested under existing `pilots/web-crypto/` top-level locale.

**Workstream**: wire the web-crypto pilot's existing SubtleCrypto substrate (digest, encrypt, decrypt, etc.) to the global `crypto.subtle` surface on JS-side. Per CRB-EXT 9: cruft has `crypto.randomUUID` + `crypto.getRandomValues` but NOT `crypto.subtle.digest` → crypto_sha256_batch CRB fixture FAILed for cruft. Real Node packages routinely use SubtleCrypto for hashing.

**Author**: 2026-05-23 session.
**Parent**: `pilots/web-crypto/` (top-level locale).
**Composes with**:
- [CRB-EXT 9 crypto_sha256_batch FAIL](../../cross-runtime-bench/results/2026-05-23/summary.md) — empirical anchor
- [Findings doc VI.4](../../rusty-js-jit/findings.md) — LOW priority forward-derived (closes surface gap)
- [Doc 736 §IX](../../../../corpus-master/corpus/736-the-architecturally-impossible-supply-chain-attack-capability-passing-closed-import-graphs-and-load-time-integrity-as-the-design-that-removes-ambient-authority.md) — cap-passing modes for crypto access
- [Findings doc IV.4 standing fuzz](../../rusty-js-jit/findings.md) — extending canonical fuzz with crypto patterns is a forward-derived round

## I. Telos

**Empirical answer to**: wire SubtleCrypto so cruft can run crypto_sha256_batch + any Node package that calls `crypto.subtle.digest`.

### I.1 First-cut scope

- **SHA-256 only** at first cut (the CRB fixture's algorithm)
- **digest only** (encrypt/decrypt + sign/verify deferred to follow-on rounds)
- **Promise-based API** per Web Crypto spec (`subtle.digest(algo, data) → Promise<ArrayBuffer>`)
- **TextEncoder integration** assumed available (it is per CRB-EXT 9 fixture passing under node)

Out of scope: SHA-1/SHA-384/SHA-512; HMAC; AES; PBKDF2; HKDF; key generation; key export/import.

### I.2 Falsifiers

**Pred-sw.1**: post-wireup, crypto_sha256_batch CRB fixture cruft-runs successfully (not FAIL). Falsifier: still FAIL → wireup didn't reach the API or implementation is wrong.

**Pred-sw.2**: output byte-identical to node baseline on the CRB fixture (digest values match). Falsifier: divergence → SHA-256 implementation bug.

**Pred-sw.3**: diff-prod 42/42 holds (no regression).

**Pred-sw.4**: canonical fuzz holds (no shape-correctness regression from crypto wireup).

**Pred-sw.5**: cap-passing modes preserved (Doc 736 §IX). Under Mode 3 (sealed), `crypto.subtle.digest` is gated per the application's caps declaration.

## II. Apparatus

Composes with:
- **Existing web-crypto pilot substrate** (Rust SHA-256 implementation per the engagement state)
- **Existing crypto object** on globalThis (cruft already exposes randomUUID + getRandomValues)
- **Canonical fuzz** (standing)
- **Cap-passing dispatcher** (per Doc 736 §IX.6)

Per Doc 738 §II.e: wireup lands at `pilots/rusty-js-runtime/derived/src/intrinsics.rs` (extending the existing crypto namespace registration) + the SHA-256 algorithm lives in the web-crypto pilot's Rust substrate.

## III. Methodology

1. **SW-EXT 0** — workstream founding (this seed + trajectory).
2. **SW-EXT 1** — survey existing web-crypto substrate; identify what's already implemented (SHA-256 + others) + the JS-side gap.
3. **SW-EXT 2** — wire `crypto.subtle` object onto cruft's globalThis crypto namespace; bind `digest()` method.
4. **SW-EXT 3** — Promise integration: `subtle.digest` returns a resolved Promise<ArrayBuffer>.
5. **SW-EXT 4** — composition probes: crypto_sha256_batch CRB fixture should now run; diff-prod 42/42; canonical fuzz holds.
6. **SW-EXT 5** — cap-passing integration (Doc 736 §IX): `subtle.digest` honors Mode 3 sealed semantics.
7. **SW-EXT 6** — default-on (no flag needed; wireup is the substrate).

## IV. Carve-outs and bounded scope

- SHA-256 only; other algorithms deferred
- digest only; encrypt/decrypt/sign/verify/key-ops deferred
- Promise-based API only
- Mode 0 cap-passing (sealed-mode integration is SW-EXT 5)

## V. Standing artefacts

- `pilots/web-crypto/subtle-wireup/seed.md`, `trajectory.md`
- `pilots/web-crypto/subtle-wireup/docs/` for survey + design
- `pilots/web-crypto/subtle-wireup/fixtures/` for subtle-specific test cases
- Wireup lands in `pilots/rusty-js-runtime/derived/src/intrinsics.rs`

## VI. Resume protocol

Read this seed, then trajectory.md tail. Read parent `pilots/web-crypto/seed.md` for the web-crypto pilot's overall framing.
