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

**Source-update attempt**

After the initial spawn, helmsman sent `class-tdz-parser-source-update` naming replacement files under:

```text
/home/jaredef/Developer/cruftless-sidecar/parity-results/
```

Expected files:

```text
cluster-super-constructor.json
cluster-tdz-cannot-access.json
cluster-parser-syntax-error.json
parity-results-top500-20260529T111702-refined.json
```

Local verification also failed for these replacement files:

```text
ls /home/jaredef/Developer/cruftless-sidecar/parity-results/{cluster-super-constructor.json,cluster-tdz-cannot-access.json,cluster-parser-syntax-error.json,parity-results-top500-20260529T111702-refined.json}: No such file or directory
find /home/jaredef -path '*parity-results*' -type f -name cluster-super-constructor.json -o ...: no matches
```

Updated blocker: complete Phase 2 remains blocked because both the original `/media/jaredef/T7/...` source and the replacement sidecar cluster files are absent from this session filesystem.

**Worktree note**

Per §V.8, an unstaged `pilots/rusty-js-runtime/derived/src/interp.rs` global/eval diff existed at session start and was not touched or staged by this probe-only spawn.

## CITPT-EXT 0a — replacement source path audit (2026-05-29)

Helmsman sent `class-tdz-parser-source-update` with replacement paths under:

```text
/home/jaredef/Developer/cruftless-sidecar/parity-results/
```

Requested replacement files:

```text
cluster-super-constructor.json
cluster-tdz-cannot-access.json
cluster-parser-syntax-error.json
parity-results-top500-20260529T111702-refined.json
```

Local verification:

```text
ls: cannot access '/home/jaredef/Developer/cruftless-sidecar/parity-results': No such file or directory
find /home /tmp /media -name cluster-super-constructor.json \
  -o -name cluster-tdz-cannot-access.json \
  -o -name cluster-parser-syntax-error.json \
  -o -name parity-results-top500-20260529T111702-refined.json: no matches
```

Adjacent sidecar root exists at `/home/jaredef/Developer/cruftless-sidecar`, with a `results/` directory, but no matching replacement files were found there or elsewhere in `/home`, `/tmp`, or `/media`.

**Disposition**: Phase 2 remains blocked on missing authoritative source JSON. The blocker has shifted from "original `/media/...` path unavailable" to "both original and replacement sidecar cluster paths unavailable in this session filesystem."

## CITPT-EXT 1 — inline source Phase 2 segmentation (2026-05-29)

Directive: `helmsman/request/class-tdz-tail-inline-resend-r4`.

Helmsman resent the source data inline because both filesystem source paths were unavailable to this R4 session. The inline data supersedes the prior blocker for Phase 2.

**Inline source cells**

Super-constructor cluster (`8`):

- `got` -> `http2-wrapper/source/agent.js:150:3`
- `commander` -> `commander/lib/command.js:24:5`
- `cheerio` -> `undici/lib/dispatcher/dispatcher-base.js:18:3`
- `@actions/http-client` -> `undici/lib/dispatcher/dispatcher-base...` (path truncated in inline resend)
- `got-fetch` -> `http2-wrapper/source/agent.js:150:3`
- `webpack-cli` -> `commander/lib/command.js:24:5`
- `ngrok` -> `http2-wrapper/source/agent.js:146:3`
- `discord.js` -> `undici/lib/dispatcher/dispatcher-base.js:20:5`

TDZ cluster (`9`):

- `arktype` -> `innerSchema`, optional-chain call, `@ark/schema/out/parse.js:59:50`
- `prettier` -> `<scoped@14>printerName`, `prettier/index.mjs:18193:5`
- `csso` -> `<scoped@29>name`, `css-tree/lib/syntax/config/mix.js:112:17`
- `rehype` -> `settings`, `rehype-parse/lib/index.js:41:72`
- `redis` -> `NON_STICKY_COMMANDS`, `@redis/client/dist/lib/commands/index.js:1160:264`
- `stylelint` -> `<scoped@33>descriptorName`, `css-tree/lib/syntax/config/mix.js:78:21`
- `puppeteer-core` -> `commonSettings`, `puppeteer-core/lib/puppeteer/node/PuppeteerNode.js:52:54`
- `svgo` -> `<scoped@33>descriptorName`, `css-tree/lib/syntax/config/mix.js:78:21`
- `config` -> `<scoped@9>prop`, `config/lib/util.js:735:7`

Parser-syntax cluster: `0` rows in the inline source; no parser C4 remains for this directive.

**Segmentation**

| Sub-cluster | Cells | C4 | Phase-3 move shape |
|---|---:|---|---|
| super-constructor/class inheritance | 8 | One class-constructor binding/evaluation-order root. The repeated packages collapse to three upstream libraries: `http2-wrapper`, `commander`, and `undici`. Every row throws the derived-constructor `this`-before-`super()` ReferenceError at class constructor entry, not a parser fault and not a `super.method()` member-call fault. | Inspect emitted bytecode for derived constructors in the three upstream libraries, then fix constructor body ordering around `SetThisTDZ`, `PushThisRaw`, `compile_super_call`, synthetic field initialization, and constructor return handling. The likely closure is ensuring `this` reads and derived-constructor returns are guarded until `super(...)` has bound this, while synthetic field initializers remain after successful super binding. |
| TDZ before initialization | 9 | One lexical/module TDZ/evaluation-order root, with two repeated upstream shapes: css-tree config mix recursion (`csso`, `stylelint`, `svgo`) and package-level module cyclic or early-read declarations (`arktype`, `prettier`, `rehype`, `redis`, `puppeteer-core`, `config`). The errors are ordinary lexical TDZ slots or scoped compiler names, not class-this TDZ. | Inspect declaration-instantiation and local-slot initialization ordering across module evaluation, function/block scopes, optional-chain calls, and destructuring/rest paths. Cross-reference prior `rusty-js-ir` TDZ work and `destructure.rs::t11_object_rest`, but do not merge this with the class-this/super rung. |
| parser-syntax-error | 0 | Empty in the authoritative inline source. Prior older-baseline parser examples remain historical context only. | No Phase-3 parser rung from this directive. Re-open only if a fresh refined parser cluster with rows is supplied. |

**Common-root decision**

Do **not** treat the inline set as one common root. Split into two independent Phase-3 sub-rungs:

1. Super-constructor/class-inheritance `this` binding and derived-constructor evaluation order.
2. Lexical/module TDZ before-initialization ordering.

The two clusters both surface as `ReferenceError`, but they are different sentinels and different coordinates:

- Super-constructor rows are class-this TDZ (`this` unbound until `super(...)`) with likely compiler/runtime sites at `compile_super_call`, `SetThisTDZ`, `PushThisRaw`, and constructor return/field-init handling.
- TDZ rows are lexical binding TDZ (`let`/`const`/scoped compiler bindings) with likely sites in module declaration instantiation, local slot seeding, optional-chain evaluation, and destructuring/object-rest lowering.

**C4**

- **C**ause: current cruft is observing derived-constructor `this` before super binding in common inheritance-heavy packages, and observing lexical TDZ reads during package/module initialization in a separate set of packages.
- **C**oordinate: class/super constructor lowering is bytecode/runtime class binding; TDZ is module/function/block lexical declaration initialization. Parser coordinate is empty for the inline source.
- **C**onstraint: no substrate land in this locale. Phase 3 must preserve prior IR TDZ closures and avoid collapsing class-this TDZ with ordinary lexical TDZ.
- **C**losure: two Phase-3 rungs, measured independently against the named top500 cells plus existing TDZ/class probes. Parser rung deferred.

## CITPT-EXT 2 — native super-constructor binding closure (2026-05-29)

Directive: `helmsman/request/citpt-ext-1-super-constructor-binding-r4`.

Phase-3 target: the super-constructor/class-inheritance sub-cluster from CITPT-EXT 1 (`got`, `commander`, `cheerio`, `@actions/http-client`, `got-fetch`, `webpack-cli`, `ngrok`, `discord.js`).

**Structural diagnosis**

The compiler-side class-this TDZ substrate was already present:

- derived constructors emit `SetThisTDZ`;
- `super(...)` lowers through `PushThisRaw` plus `PropagateNewTarget`;
- `SetThis` binds the derived frame after the parent constructor returns an object.

The missing case was the native-constructor branch in `Runtime::call_function_inner`. User closures invoked as constructors normalize `undefined` body completion to the active constructor `this`; native `InternalKind::Function` calls returned the native callback result verbatim. When a native parent constructor returned `undefined`, the derived `super(...)` sequence received `Undefined`, `SetThis` ignored it, and the frame kept the class-this TDZ sentinel. The next `this` read, or implicit derived-constructor return, threw:

```text
Must call super constructor in derived class before accessing 'this' or returning from derived constructor
```

**Closure**

Normalize native constructor calls at the call-function boundary:

- if `new.target` is present and the native callback returns an object, preserve that object;
- if `new.target` is present and the native callback returns a primitive/`undefined`, return the active constructor receiver (`current_this`) so `super(...)` can bind it;
- if `new.target` is absent, preserve plain-call behavior.

This keeps the closure at the runtime call boundary rather than adding special cases to `compile_super_call` or `SetThis`.

**Regression**

Added `classes::t17_native_super_constructor_binds_this`:

```js
class B extends EventTarget {
  constructor() {
    super();
    this.ok = 1;
  }
}
return new B().ok;
```

The regression exercises the same native-parent-constructor shape as the package cluster while remaining independent of the package sandbox.

**Measurements**

- `cargo test -p rusty-js-runtime t17_native_super_constructor_binds_this --release` — PASS.
- `cargo build --release --bin cruft -p cruftless` — PASS.
- `cargo test -p rusty-js-runtime --lib --release` — PASS (66 passed, 1 ignored).
- Fresh npm sandbox at `/home/jaredef/Developer/cruftless-sidecar/results/citpt-ext-1-super-constructor-sandbox`, current `target/release/cruft`, dynamic import probe over the eight inline cells:
  - PASS: `got`, `commander`, `@actions/http-client`, `got-fetch`, `webpack-cli`, `ngrok` (`6/8`).
  - residual non-super failures: `cheerio` -> `MessagePort is not defined`; `discord.js` -> `Assignment to constant variable 'd'`.
  - super-constructor TDZ error cleared across all eight probed cells (`0/8` still show the target error).

## CITPT-EXT 2 — object-rest TDZ init-site closure, lexical/module residual (2026-05-29)

Directive: `helmsman/request/citpt-ext-2-tdz-lexical-module-r4`.

**Scope-down disposition**

Closed the concrete destructuring/rest TDZ sub-shape that had been blocking
`cargo test --release -p rusty-js-runtime` at `tests/destructure.rs::t11_object_rest`.
The nine package cells remain a separate lexical/module TDZ false-positive
surface and are explicitly deferred as the next layer.

**Root cause closed**

`emit_destructure()` already initialized ordinary object-pattern leaves with
`Op::InitLocal`, allowing declaration initialization to overwrite the TDZ
sentinel seeded for `let`/`const` bindings. The object-rest binding path was
still using `Op::StoreLocal`, so this valid declaration:

```js
const {a, b, ...rest} = {a: 1, b: 2, c: 3, d: 4};
```

threw `ReferenceError("Cannot access 'rest' before initialization")` while
initializing `rest`.

**Closure**

Changed the object-rest declarator write from `StoreLocal` to `InitLocal` in
`pilots/rusty-js-bytecode/derived/src/compiler.rs`, matching the existing
destructure leaf initialization semantics.

**Measurements**

- `cargo test --release -p rusty-js-runtime --test destructure t11_object_rest -- --nocapture` — PASS.
- `cargo build` — PASS.
- `cargo test --release -p rusty-js-runtime --lib` — PASS (`68 passed`, `1 ignored`).
- 9-cell TDZ package smoke via `legacy/host-rquickjs/tools/parity-measure.sh` — `0/9` PASS; all nine still fail with the original `Cannot access ... before initialization` load-time TDZ messages.

**Residual**

The package cluster is not closed by the object-rest init-site fix:

- `arktype` still fails on `innerSchema` in optional-chain/declaration ordering.
- `prettier`, `csso`, `stylelint`, and `svgo` still fail on scoped lexical names.
- `rehype`, `redis`, `puppeteer-core`, and `config` still fail on package/module initialization reads.

The next rung should target the lexical/module evaluation-order layer directly,
not the destructuring rest helper path.
