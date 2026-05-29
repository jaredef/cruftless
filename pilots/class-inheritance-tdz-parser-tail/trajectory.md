# class-inheritance-tdz-parser-tail — Trajectory

## CITPT-EXT 0 — Phase 0 spawn + Phase 2 source-artifact blocker (2026-05-29)

Directive: `helmsman/request/class-tdz-parser-tail-phase-0-phase-2-r4`.

**Scope**: Phase 0 + Phase 2 only; no substrate land.

**Phase 0**

Created:

```text
pilots/class-inheritance-tdz-parser-tail/seed.md
pilots/class-inheritance-tdz-parser-tail/trajectory.md
```

Manifest refresh is part of this spawn commit.

**Required source artifact**

Helmsman named:

```text
/media/jaredef/T7/rusty-bun/parity-results/parity-results-top500-20260529T111702-refined.json
```

Local verification:

```text
ls: cannot access '/media/jaredef/T7/rusty-bun/parity-results/parity-results-top500-20260529T111702-refined.json': No such file or directory
find /media/jaredef/T7 -path '*parity*20260529*' -type f: no matches
find repo + /media/jaredef/T7 for '*top500*refined*.json' or '*20260529T111702*.json': no matches
```

This blocks the requested complete 22-cell Phase 2 C4. The row set and the refined current error messages are load-bearing for deciding whether these are one root or three independent sub-clusters.

**Available substitute evidence inspected**

Older snapshots:

- `specs/parity-baselines/results-2026-05-22-top500*.json`
- `legacy/host-rquickjs/tools/parity-results*.json`

Useful older observations:

- Parser-syntax older examples:
  - `tsdown`: `CompileError("parse: expected ')' in dynamic import() @byte27959 ... options-DUthngzZ.mjs")`
  - `gulp-uglify`: `CompileError("parse: expected RBracket @byte1018168 @url=file://<Function:0>")`
  - `pug`: `CompileError("parse: expected RBracket @byte40 @url=file://<Function:0>")`
  - `typeorm`: older rows alternate between CJS-wrapper slash parse and unrelated runtime `Reflect`/callability errors depending on snapshot.
- TDZ named packages in older snapshots do not reliably match the requested TDZ messages:
  - `arktype` older rows show undefined property/callability failures, not the Round 10 `Cannot access X before initialization` class.
  - `prettier`, `csso`, `rehype` are PASS in several older snapshots.
  - `redis` older rows show package.json resolution failure.
- Super-constructor named packages in older snapshots also do not match the requested class-inheritance cluster:
  - `got`, `commander`, `cheerio`, `@actions/http-client`, `got-fetch` are PASS in several older snapshots, or fail for older unrelated causes.

**Phase 2 segmentation (provisional, not C4-complete)**

| Sub-cluster | Count | Provisional C4 | Evidence strength | Phase-3 move shape |
|---|---:|---|---|---|
| super-constructor/class inheritance | 8 | Independent from TDZ/parser unless refined rows show class-field TDZ inside derived constructors. Need exact stack/message to distinguish missing `super()` call edge from `super.method` lowering. | Blocked by missing refined rows. | likely one class-lowering runtime/compiler rung; inspect bytecode around `Op::SetThisTDZ`, `Op::PushThisRaw`, `compile_super_call`, constructor-return handling. |
| TDZ | 9 | Likely independent TDZ tail. Not safe to merge with super-constructor because prior `rusty-js-ir` TDZ work closed class-this shapes while destructuring/object-rest TDZ remains separate. | Blocked by missing refined rows. | likely one lexical/module/declaration-instantiation rung plus separate destructuring TDZ rung if `t11_object_rest` shape appears. |
| parser-syntax-error | 5 | Independent parser/Function-constructor grammar tail. Older examples point to dynamic import grammar and computed/member/bracket parsing inside generated Function source. | Partial older evidence only. | likely one parser permissiveness/grammar rung after exact syntax extraction; may split if top-level-await/decorators/import attributes appear. |

**Common-root decision**

Current decision: treat as three independent sub-clusters. The missing refined source prevents a stronger common-root falsification. The older substitute evidence points to different tiers:

- class lowering/runtime;
- TDZ declaration/lexical initialization;
- parser grammar acceptance.

Those tiers are not a single C4 root unless the missing refined data proves an intersection such as class-field TDZ inside parser-rewritten Function constructors.

**Blocker**

Complete Phase 2 deliverable is blocked by missing source artifact. Concrete blocker is the absent refined JSON, not the dirty worktree.

**Worktree note**

Per §V.8, an unstaged `pilots/rusty-js-runtime/derived/src/interp.rs` global/eval diff existed at session start and was not touched or staged by this probe-only spawn.
