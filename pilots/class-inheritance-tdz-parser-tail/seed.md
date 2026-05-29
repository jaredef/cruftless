---
name: class-inheritance-tdz-parser-tail
description: Phase-0/Phase-2 probe locale for top500 tail clusters spanning class inheritance/super constructor errors, TDZ before-initialization errors, and parser syntax errors.
type: project
---

# class-inheritance-tdz-parser-tail — Seed

**Status**: **SPAWNED / PHASE-2 COMPLETE**. Phase 0 scaffold exists. Phase 2 segmentation was completed from the inline resend data in `helmsman/request/class-tdz-tail-inline-resend-r4`; no runtime/parser substrate has landed from this locale.

## Telos

Segment the 2026-05-29 top500 tail cells into three candidate sub-clusters and decide whether they share a common substrate root or should proceed as independent Phase-3 rungs:

1. **Super-constructor/class-inheritance tail** — eight cells: `got`, `commander`, `cheerio`, `@actions/http-client`, `got-fetch`, `webpack-cli`, `ngrok`, `discord.js`.
2. **TDZ tail** — nine cells: `arktype`, `prettier`, `csso`, `rehype`, `redis`, `stylelint`, `puppeteer-core`, `svgo`, `config`.
3. **Parser-syntax tail** — originally requested as five cells, then superseded by inline source data where the parser cluster is empty (`0` rows).

This locale is Phase 0 + Phase 2 only. Do not land runtime/parser substrate from this locale without a Phase-3 directive.

## Required Source Data

Helmsman named:

```text
/media/jaredef/T7/rusty-bun/parity-results/parity-results-top500-20260529T111702-refined.json
```

That file is the authoritative cell list from the original directive.

Helmsman later supplied replacement paths:

```text
/home/jaredef/Developer/cruftless-sidecar/parity-results/cluster-super-constructor.json
/home/jaredef/Developer/cruftless-sidecar/parity-results/cluster-tdz-cannot-access.json
/home/jaredef/Developer/cruftless-sidecar/parity-results/cluster-parser-syntax-error.json
/home/jaredef/Developer/cruftless-sidecar/parity-results/parity-results-top500-20260529T111702-refined.json
```

These replacement files were also absent in this R4 session. Helmsman then resent the cluster rows inline in `class-tdz-tail-inline-resend-r4`; that inline JSON is the Phase 2 source for this locale.

## Phase-2 Questions

- Super-constructor: determine whether the cells are all `class X extends Y` constructor-call/lowering edges, or whether some are `super.method()`/super-property invocation edges.
- TDZ: determine whether the cells share one temporal-dead-zone shape, such as lexical declaration before use across module/function boundary, and cross-reference the existing `rusty-js-ir` TDZ trajectory plus `destructure.rs::t11_object_rest`.
- Parser-syntax: empty in the inline data; no parser Phase-3 rung is warranted from this cell set.
- Decide whether the sub-clusters share a common root. Phase 2 decision: **independent**, split into a class/super-constructor rung and a TDZ-declaration/evaluation-order rung.

## Apparatus References

- `pilots/rusty-js-ir/trajectory.md` — prior class-this TDZ rounds and broader IR TDZ work.
- `pilots/rusty-js-parser/derived/` — parser entry for syntax-error rows.
- `pilots/rusty-js-bytecode/derived/src/compiler.rs` and `pilots/rusty-js-runtime/derived/src/interp.rs` — likely class/super and TDZ lowering/runtime sites.
- `specs/parity-baselines/` and `legacy/host-rquickjs/tools/parity-results*.json` — older fallback context only; not a substitute for the named refined source data.

## Status

Spawned 2026-05-29 by R4. Phase 0 complete. Phase 2 complete from inline source data. Next work, if authorized, should enter Phase 3 as two independent sub-rungs: class/super constructor binding and TDZ declaration/evaluation order.
