---
arc: 2026-05-28-typescript-resolver-arc
trigger: Plan agent's back-fit analysis 2026-05-28 (keeper directive Telegram 10158); empirical: keeper directive 2026-05-24 founding TSR + Doc 742 TS-parity arc closure
opened: 2026-05-28
closed: IN PROGRESS
close_condition: TCC failure-table green per sub-locale; TXC parity baseline established; per-TSR sub-locale exemplar suite passes; cross-arc relations with iterator-protocol + parser arcs explicit.
---

# TypeScript Resolver Arc

## Trigger

Plan agent's back-fit (2026-05-28, per keeper Telegram 10158) identified eight `ts-resolve*` locales plus the apparatus pilots TCC (ts-consumer-corpus) and TXC (ts-execute-corpus) sharing the TypeScript resolver-instance mouth-terminus: `.ts` source bytes → erasure to executable JS preserving TS-only constructs' runtime contracts. Empirically anchored in the 2026-05-24 TSR founding + Doc 742's full-strength resolver-instance pattern as the corpus articulation.

## Telos

Subsume the TS-resolve family under one arc with explicit (M, T, I, R) per Doc 744. The arc-tier mouth is "`.ts` source bytes per TypeScript spec subset cruftless engages"; the arc-tier terminus is "erasure-to-executable-JS preserving TS-only runtime contracts (enums, decorators, ctor-param shorthand, generics-erasure, namespace consolidation)". The arc-tier interior is the TSR resolver pillar's sub-locale chain. The arc-tier relations: DAG ↑ TCC / TXC apparatus pilots (apparatus consumes TSR's terminus to measure parity); lattice with parser-early-error arc (TS syntax extends the parser's grammar).

## Sub-locale roster

| Locale | Role in arc | Status pre-arc |
|---|---|---|
| `ts-resolve` (parent) | TSR pillar | LANDED at TSR-EXT 5 (canonical 4-round rule-13 closure under Pred-tsr.6 ≤6 budget) |
| `ts-resolve-class-and-param-shapes` (TRCAPS) | class + ctor-param shorthand erasure | LANDED |
| `ts-resolve-enums` (TRE) | enum lowering | LANDED |
| `ts-resolve-generics-calls` (TRGC) | generic-call site erasure | LANDED (multiple sub-rungs per Rule 14 conservative-strip) |
| `ts-resolve-module-loader-extension` (TRMLE) | module-loader extension surface | LANDED |
| `ts-resolve-string-literal-safety` (TRSLS) | string-literal-type safety | LANDED |
| `ts-resolve-type-only-imports` (TROI) | `import type` erasure | LANDED |

Apparatus pilots (sibling to arc):
- `apparatus/ts-consumer-corpus` (TCC) — parse-parity measurement instrument
- `apparatus/ts-execute-corpus` (TXC) — execute-parity measurement instrument

## Cumulative yield

Pre-arc: TSR arc closed at TSR-EXT 5 in 4 implementation rounds (under Pred-tsr.6 ≤6 budget). Cross-cluster yield aggregated in Doc 742 §VIII (resolver-instance boundary contract empirical closure).

Per-arc future measurement: TCC parse-parity rate + TXC execute-parity rate; tracked at arc fold.

## Cross-arc relations

- **DAG ↑ TCC + TXC apparatus pilots**: the apparatus consumes TSR's terminus; per Doc 744 §IV.1.a mouth-gating, TCC/TXC mouth IS TSR's terminus.
- **Lattice with `2026-05-28-parser-early-error-conformance`**: TS syntax extends the parser's grammar at lattice-meet on the parser tier.
- **Alphabet-exchange ↑ `rusty-js-bytecode`**: erased JS emits via the same bytecode emitter as the JS substrate.

## Cross-locale findings

To be promoted as the arc operates. Initial entries already in findings.md Addendum XIV (TSR canonical closure with rule-13 prospective application validation; finding TSR.C3 cost-positive-when-integrated as load-bearing rule-13 condition).

## Status

IN PROGRESS — scaffolded 2026-05-28 per keeper directive 10158. Per the Plan agent's recommendation, this is the third new arc to be back-fit (provides the canonical example of an arc whose roster is gated by an apparatus pilot — TCC/TXC — per Doc 744 mouth-gating; clarifies the resolver-instance-tier sibling-pilot relationship articulated in Doc 742).
