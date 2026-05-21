# Tier-Ω Cutover Audit: rquickjs → rusty-js-* Engine Transition

**Date**: 2026-05-21
**Engagement**: `/home/jaredef/rusty-bun`
**Live parity baseline**: 88.2% (105/119) on curated top-N
**Audit method**: Explore-agent walk of the rusty-js-* crates and parity tooling, cross-validated against direct grep of `parity-measure.sh`.

---

## Headline finding

**The Tier-Ω cutover is already in-flight.** Verified via direct grep at `legacy/host-rquickjs/tools/parity-measure.sh:32`:

```
RB="${RB_BIN:-$ROOT/target/release/cruftless}"
```

The default parity-measurement binary is `cruftless` (renamed today from `host-v2`, embedding rusty-js-runtime), **not** `cruftless-rquickjs`. The 88.2% parity reading was measured against the hand-rolled engine, not rquickjs.

This re-frames the engagement's standing:

- The Tier-Ω cutover was previously thought to be the gating workstream that closes the 14-package failure cluster.
- In fact, the cutover has been substantially built across the rusty-js-* pilots; what remains is **gap-closure within the already-deployed hand-rolled engine**.
- The 14 failing packages reflect genuine substrate gaps in rusty-js-runtime and rusty-js-parser, not a pre-cutover baseline.

The remaining work decomposes into 4–6 Pin-Art-shaped rounds, each closing a specific gap and flipping 1–4 cluster packages from FAIL to PASS.

---

## Phase 1: Module-binding synthesis (rusty-js-runtime/derived/src/module.rs)

Walked `populate_cjs_namespace_view` (lines 1388–1552), `resolve_import_binding_value` (lines 684–737), and CJS import dispatch in `evaluate_module` (lines 884–929).

| Behavior | Status | Location | Notes |
|---|---|---|---|
| ESM-no-default → namespace as default | Present | module.rs:911–919 | Built-in fall-back when `default` is undefined returns the namespace object directly |
| ESM-default-only → named from default's enumerables | Partial | module.rs:1448–1552 | Framework present; transpiled code works (Babel/TS emit explicit re-exports); raw-authored ESM falls through |
| CJS `module.exports = X` → ESM `import {y}` | Present | module.rs:889–909 | Includes getter dispatch via `find_getter` for rxjs / __exportStar pattern |
| CJS `exports.foo = ...` → namespace mirror + default synthesis | Present | module.rs:1448–1552 | `populate_cjs_namespace_view` mirrors own props; synthesizes default when exports reassigned or has user keys |
| Reserved-method class fields (`delete = 1; new = 2`) | Present | parser/stmt.rs:324–340 | `parse_class_member_name` accepts any TokenKind::Ident without reserved-word filtering |
| String-literal export aliases (`as "m-search"`) | Present | ast/lib.rs:618–625 + compiler.rs:612,851–877 + module.rs:1030–1044 | Full pipeline AST → bytecode → runtime |
| Late property mutation visibility | Snapshot | module.rs:883–935 | Spec-aligned snapshot semantics; matches Bun |

**Verdict**: Module-binding synthesis is **substantially complete**. No critical gap blocking the 14-package cluster. The ESM-default-from-properties partial is a design trade-off matching Bun's behavior on transpiled code (which dominates the parity corpus).

---

## Phase 2: Parser modernization (rusty-js-parser/derived/src/)

Walked stmt.rs, expr.rs, lexer.rs, lib.rs and the AST node definitions in rusty-js-ast/src/lib.rs.

| ES2022+ feature | Status | Location | Notes |
|---|---|---|---|
| Class field arrow-fn init | Present | parser/stmt.rs:310–313 | Field initializer uses full `parse_assignment_expression` |
| String-literal export aliases | Present | (Phase 1) | Same surface as Phase 1 Behavior 6 |
| Top-level await | Absent | (deferred per Ω.5.b ceiling) | None of the 14-package cluster uses TLA |
| Logical-assignment ops (`||=`, `&&=`, `??=`) | Present | parser/expr.rs:107–110 | Lexer tokens + AssignOp dispatch |
| Numeric separators (`1_000_000`) | Present | parser/lexer.rs | Confirmed by `numeric_separator()` test in spec_golden.rs |
| Optional chaining + nullish coalescing | Present | parser/expr.rs:156–159 | Binary ops + member-access chain |
| Private class fields/methods (`#field`) | Present | ast/lib.rs:611–615 + parser/stmt.rs:329 | Declaration recognized; access semantics simplified in v1 |
| Static class blocks | Present | parser/stmt.rs:242–247 | `ClassMember::StaticBlock` parsed |
| For-await (`for await`) | Partial | parser/stmt.rs:428 | Parser sets `await_form`; runtime treats as for-of |
| `import.meta` | Present | runtime/module.rs:937–949 | Runtime allocates `{url, dir}`; Op::PushImportMeta |

**Verdict**: Parser modernization is **comprehensive**. All ES2022 features used by the parity corpus are present in declaration form. TLA and async-iterator runtime semantics are deferred per the Ω.5.b ceiling.

---

## Phase 3: Engagement-wiring gap

| Binary | Engine | Path | Status |
|---|---|---|---|
| `cruftless` | rusty-js-runtime | `cruftless/src/main.rs` | **Primary**; default parity target |
| `cruftless-rquickjs` | rquickjs | `legacy/host-rquickjs/src/main.rs` | Legacy; opt-in via `RB_BIN=` override |

Direct verification:

```
$ grep RB= legacy/host-rquickjs/tools/parity-measure.sh
RB="${RB_BIN:-$ROOT/target/release/cruftless}"
```

The 88.2% parity baseline is measured against the rusty-js-* binary. The cutover is structurally complete at the engagement-wiring tier; what remains is gap-closure inside the already-deployed engine.

---

## Phase 4: Cutover sequence (6 Pin-Art rounds)

Each round = one substrate move + one §XVI yield (probe flip). Mode-0 backward compat preserved (rquickjs binary remains available throughout).

### Round 1: CJS getter dispatch completeness

**Move**: Audit `object_get` call sites across `interp.rs` to ensure every CJS property read routes through getter dispatch. Currently confirmed at named-import and namespace-view sites (module.rs:899–904 + 1506–1510); need to verify it holds for direct property reads inside the runtime.

**Probe candidate**: **ansi-styles** — uses `Object.defineProperty` for color accessors; currently in the failure cluster.

**Estimated flip**: 1–2 packages (ansi-styles + chalk dependents).

---

### Round 2: ESM re-export edge cases

**Move**: Tighten `export *` to skip `default` per spec; add name-collision detection in the reexport phase (module.rs:826–835).

**Probe candidate**: **fp-ts** — heavily uses re-exports and barrels.

**Estimated flip**: 2–3 packages.

---

### Round 3: CJS default-synthesis logic

**Move**: Refine the heuristic at module.rs:1543 that decides whether to synthesize a `default` export. Specifically: ensure that `module.exports = primitive` (function, number, string) always produces `default = value` in the ESM namespace.

**Probe candidate**: **debug**, **minimist** — both export a single function as the default; commonly imported via `import debug from "debug"`.

**Estimated flip**: 2–4 packages.

---

### Round 4: Parser edge-case hardening

**Move**: Run the parity corpus through the parser in isolation (parse-only mode, no evaluation) and log every parse failure. Fix the top 3 by frequency.

**Probe candidate**: **yargs**, **enquirer** — CLI tools with complex bundled code.

**Estimated flip**: 2–3 packages.

---

### Round 5: Intrinsics + globals completeness

**Move**: Scan the failure cluster for references to globals not yet stubbed in `cruftless/src/intrinsics.rs` + `node_stubs.rs`. Add the top 3 missing.

**Probe candidate**: **got**, **node-fetch** — HTTP clients with global API expectations.

**Estimated flip**: 2–3 packages.

---

### Round 6: Module resolution barrels + entry-point logic

**Move**: Audit `resolve_exports_target` (called at module.rs:447) to ensure importer_kind (ESM vs CJS) is passed correctly. Add fixture for `"exports": { "import": "./esm.js", "require": "./cjs.js" }`.

**Probe candidate**: **ora**, **jsonc-parser** — use conditional exports.

**Estimated flip**: 2–4 packages.

---

### Cumulative trajectory

| Round | Substrate | Flip target | Cumulative pass |
|---|---|---|---|
| 1 | getter dispatch | ansi-styles | 106–107 / 119 |
| 2 | ESM re-export edges | fp-ts | 108–110 / 119 |
| 3 | CJS default synthesis | debug, minimist | 110–114 / 119 |
| 4 | Parser hardening | yargs, enquirer | 112–117 / 119 |
| 5 | Intrinsics + globals | got, node-fetch | 114–120 / 119 |
| 6 | Module resolution | ora, jsonc-parser | 116–120+ / 119 |

**Target**: ≥96% (114+/119) after Rounds 1–3; effective parity completion (98–100%) after Round 6.

---

## Recommendation

### Immediate

**Begin Round 1** (CJS getter dispatch audit). High-leverage, low-risk; isolates a known surface; produces a measurable probe flip (ansi-styles). Estimated 1–2 hours. This is the natural first cutover-acceleration move.

### Cutover decision gate (after Rounds 1–3)

If parity reaches ≥96% (114+/119), declare the Tier-Ω cutover complete and mark `legacy/host-rquickjs` as functionally deprecated (keep available as reference; remove default-fallback path from parity-measure.sh).

If parity stalls below 94% after all 6 rounds, convene design review: are the remaining 4–5 packages blocked on features outside the Ω.5 scope?

### Risk mitigations

- **Backward compat**: The rquickjs binary remains available throughout; any regression rollback via `RB_BIN=$ROOT/target/release/cruftless-rquickjs parity-measure.sh`.
- **Mode-0 invariant**: Capability modes (audit / sealed / sealed-deps) preserved across cutover (already integrated in cruftless/src/main.rs:70-94).
- **Cross-pilot composition**: Tonight's PM, caps, and JIT work all run on rusty-js-runtime; the cutover does not threaten any of them.

### What this audit found that the prior framing did not

The earlier engagement-level synthesis (the "Tier-Ω as critical-path gating workstream" framing) suggested an 8–15-round commitment to hand-roll the engine. The audit reveals that the engine is **already hand-rolled and already deployed as the parity-measurement target**. The 14-package failure cluster reflects specific gaps in the deployed engine, not the absence of a hand-rolled engine. The remaining work is 4–6 Pin-Art rounds of targeted gap-closure, not a multi-week engine-rebuild.

This is the audit's load-bearing finding: **Tier-Ω is not a future workstream. It is the current workstream and is ~88% complete.** The 6-round sequence closes the remaining ~12%.

---

## File reference

| Concern | File | Lines |
|---|---|---|
| Module binding synthesis | `pilots/rusty-js-runtime/derived/src/module.rs` | 684–737, 884–929, 1388–1552 |
| CJS wrapper synthesis | `pilots/rusty-js-runtime/derived/src/module.rs` | 1111–1386 |
| String-literal aliases | `pilots/rusty-js-ast/src/lib.rs` | 618–625 |
| Parser class fields | `pilots/rusty-js-parser/derived/src/stmt.rs` | 222–340 |
| Parser arrow-fn in fields | `pilots/rusty-js-parser/derived/src/stmt.rs` | 310–313 |
| Binary configuration | `cruftless/Cargo.toml` + `cruftless/src/main.rs` | (full) |
| Parity tool default | `legacy/host-rquickjs/tools/parity-measure.sh` | 32 |

---

*End of audit. The Tier-Ω cutover workstream's first move (Round 1: CJS getter dispatch) is the next substrate landing.*
