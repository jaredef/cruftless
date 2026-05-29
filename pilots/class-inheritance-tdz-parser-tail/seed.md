---
name: class-inheritance-tdz-parser-tail
description: Phase-0/Phase-2 probe locale for top500 tail clusters spanning class inheritance/super constructor errors, TDZ before-initialization errors, and parser syntax errors.
type: project
---

# class-inheritance-tdz-parser-tail — Seed

## Telos

Segment the 2026-05-29 top500 tail cells into three candidate sub-clusters and decide whether they share a common substrate root or should proceed as independent Phase-3 rungs:

1. **Super-constructor/class-inheritance tail** — eight cells including `got`, `commander`, `cheerio`, `@actions/http-client`, `got-fetch`, plus three unnamed cells in the refined source data.
2. **TDZ tail** — nine cells including `arktype`, `prettier`, `csso`, `rehype`, `redis`, plus four unnamed cells in the refined source data.
3. **Parser-syntax tail** — five cells: `typeorm`, `parse-duration`, `tsdown`, `gulp-uglify`, `pug`.

This locale is Phase 0 + Phase 2 only. Do not land runtime/parser substrate from this locale without a Phase-3 directive.

## Required Source Data

Helmsman named:

```text
/media/jaredef/T7/rusty-bun/parity-results/parity-results-top500-20260529T111702-refined.json
```

That file is the authoritative cell list for this locale. Phase 2 is blocked until it is present or a replacement refined top500 dataset is explicitly supplied.

## Phase-2 Questions

- Super-constructor: determine whether the cells are all `class X extends Y` constructor-call/lowering edges, or whether some are `super.method()`/super-property invocation edges.
- TDZ: determine whether the cells share one temporal-dead-zone shape, such as lexical declaration before use across module/function boundary, and cross-reference the existing `rusty-js-ir` TDZ trajectory plus `destructure.rs::t11_object_rest`.
- Parser-syntax: identify the exact syntax the parser rejects in each package, not just the package name.
- Decide whether the three sub-clusters share a common root. Default hypothesis is **independent** until the refined source data proves a shared upstream.

## Apparatus References

- `pilots/rusty-js-ir/trajectory.md` — prior class-this TDZ rounds and broader IR TDZ work.
- `pilots/rusty-js-parser/derived/` — parser entry for syntax-error rows.
- `pilots/rusty-js-bytecode/derived/src/compiler.rs` and `pilots/rusty-js-runtime/derived/src/interp.rs` — likely class/super and TDZ lowering/runtime sites.
- `specs/parity-baselines/` and `legacy/host-rquickjs/tools/parity-results*.json` — older fallback context only; not a substitute for the named refined source data.

## Status

Spawned 2026-05-29 by R4. Phase 0 complete. Phase 2 blocked on missing authoritative source JSON.
