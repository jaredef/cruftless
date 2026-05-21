# Parity baseline log

Per-date snapshots of the canonical parity sweep
(`legacy/host-rquickjs/tools/parity-measure.sh` against the
`parity-top100.txt` basket of 119 packages).

| Date | Tag | Pass | Total | Parity | Delta |
|---|---|---|---|---|---|
| 2026-05-13 | (baseline) | 105 | 119 | 88.2% | — |
| 2026-05-21 | (post-rename, pre-Round-1) | 110 | 119 | 92.4% | +4.2 pp |
| **2026-05-21** | **r1 (generator-prototype chain)** | **112** | **119** | **94.1%** | **+1.7 pp** |

The 2026-05-21-r1 reading is post Tier-Ω Round 1 (commit fac0855e: generator + async-generator prototype chain per ECMA §27.3-§27.5). got + get-stream flipped FAIL → PASS at this round.

7 remaining failures (2026-05-21-r1):
- **runtypes** — load crash (Class C; root cause not yet identified)
- **arktype** — load crash (Class C; possibly same root as runtypes)
- **superstruct** — 1 missing export (default; Class A)
- **node-fetch** — 2 missing exports (fetch, FetchBaseError; Class A)
- **superagent** — load crash (cuid2 `Object.values` missing on Function; Class C)
- **entities** — TIMEOUT (Class D)
- **enquirer** — 21 extra exports (Class B, over-synthesis)

**Cutover decision gate** (≥96%, per Tier-Ω audit): not yet met. Need 2 more passes (114/119). Round 2 candidates by leverage:

- Class C runtypes/arktype root-cause probe → potentially 2 flips at once
- Class C superagent (cuid2 Object.values) → 1 flip
- Class A superstruct (default synthesis) → 1 flip

To reproduce a baseline:

```
RB_BIN=/home/jaredef/rusty-bun/target/release/cruftless \
  nice -n 19 ionice -c3 \
  bash legacy/host-rquickjs/tools/parity-measure.sh \
       legacy/host-rquickjs/tools/parity-top100.txt \
       out.json
```

Wall time: ~25 minutes on Pi-5 under nice -n 19.
