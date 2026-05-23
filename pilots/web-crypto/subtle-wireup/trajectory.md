# web-crypto/subtle-wireup — Trajectory

Per-SW-EXT log for the SubtleCrypto wireup sub-pilot.

---

## SW-EXT 0 — 2026-05-23 (workstream founding)

Apparatus-tier round. Pilot founded per keeper directive 2026-05-23 18:12-local. Nested under existing `pilots/web-crypto/` per Doc 737 §IV. Third of four spawns in this round.

### Trigger

- CRB-EXT 9 crypto_sha256_batch FAILed for cruft (`crypto.subtle.digest` not exposed)
- Keeper triage named SubtleCrypto wireup as the smallest-scope highest-impact-per-effort gap
- Real Node packages routinely use SubtleCrypto for hashing; the FAIL blocks compatibility

### Substrate delivered

- `seed.md` (~70 lines): telos (SHA-256 + digest only first cut), 5 falsifiers Pred-sw.1-.5, methodology SW-EXT 0-6, carve-outs
- `trajectory.md` (this file)
- `docs/` + `fixtures/` scaffolds

### Locale registration

Locale count: 18 → 19 after this spawn (12 top-level unchanged; 6 → 7 nested). Manifest refresh queued.

### Open scope at SW-EXT 0 close

1. SW-EXT 1 — survey existing web-crypto substrate
2. SW-EXT 2 — wire crypto.subtle.digest
3. SW-EXT 3-6 per seed §III

---

*SW-EXT 0 closes. Pilot founded. Smallest-scope of the 4 spawns this round; SW-EXT 1 begins with the substrate survey when the pilot is activated.*
