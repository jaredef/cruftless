# Parity baseline log

Per-date snapshots of the canonical parity sweep (`legacy/host-rquickjs/tools/parity-measure.sh` against the `parity-top100.txt` basket of 119 packages).

| Date | Tag | Pass | Total | Parity | Delta |
|---|---|---|---|---|---|
| 2026-05-13 | (baseline) | 105 | 119 | 88.2% | — |
| 2026-05-21 | (post-rename, pre-Round-1) | 110 | 119 | 92.4% | +4.2 pp |
| 2026-05-21 | r1 (generator-prototype chain) | 112 | 119 | 94.1% | +1.7 pp |
| **2026-05-21** | **r2 (symbol-keyed Reflect.ownKeys + defineProperty)** | **113** | **119** | **94.9%** | **+0.8 pp** |

The 2026-05-21-r2 reading is post Tier-Ω Round 2 (commit 29ae498b: symbol-keyed property enumeration + defineProperty preservation). runtypes flipped FAIL → PASS.

**Cumulative trajectory**: 88.2% → 94.9% in three Pin-Art substrate moves. **+6.7 percentage points**.

6 remaining failures (2026-05-21-r2):
- **arktype** — load crash, `rawIn` method missing on Array prototype (Class C, distinct from runtypes root)
- **superstruct** — 1 missing export: `default` (Class A, smallest substrate move available)
- **node-fetch** — 2 missing exports: `fetch`, `FetchBaseError` (Class A)
- **superagent** — load crash, cuid2 needs `Object.values` on Function chain (Class C)
- **entities** — TIMEOUT (Class D, hang)
- **enquirer** — 21 extra exports (Class B, over-synthesis)

**Cutover decision gate** (≥96%, 114/119): not yet met. Need 1 more flip.

Round 3 candidates by leverage:
1. **superstruct** (Class A, smallest move): CJS default synthesis. Closes the cutover gate alone if it flips.
2. **arktype** (Class C): need to investigate `rawIn` method discrepancy. May share root with superagent.
3. **node-fetch** (Class A): ESM exports gap.

To reproduce a baseline:

```
RB_BIN=/home/jaredef/rusty-bun/target/release/cruftless \
  nice -n 19 ionice -c3 \
  bash legacy/host-rquickjs/tools/parity-measure.sh \
       legacy/host-rquickjs/tools/parity-top100.txt \
       out.json
```

Wall time: ~25 minutes on Pi-5 under nice -n 19.
