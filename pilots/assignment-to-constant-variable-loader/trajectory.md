# assignment-to-constant-variable-loader — Trajectory

## ATC-EXT 0 — Phase 0 spawn + Phase 2 blocker (2026-05-29)

### Directive

Helmsman directed R1 via CAACP message
`b0467a28-ea48-4bf0-b91a-8fd721542f90` to spawn
`pilots/assignment-to-constant-variable-loader/` and run a Phase 2 probe
against the 2026-05-29 top500 refined sweep's 26 dynamic-import failures whose
`rb.message` contains `Assignment to constant`.

Scope is Phase 0 + Phase 2 only. No substrate land is authorized.

### Phase 0

Locale founded at `pilots/assignment-to-constant-variable-loader/` with
`seed.md` and `trajectory.md`.

`apparatus/locales/CANDIDATES.md` and existing locale seeds/trajectories were
checked for `assignment-to-constant`, `constant variable`, and adjacent `const`
loader probes; no existing ATC-loader coordinate was present.

Rule 11 pre-spawn coverage:

- **A1 component A/B**: top500 dynamic-import package loading through cruft
  versus Bun package import behavior.
- **A2 op-set**: package entry resolution, CommonJS/ESM wrapper loading,
  lexical declaration lowering, assignment to local/global/module bindings,
  destructuring initializer evaluation, and error propagation through
  `import(pkg)`.
- **A3 value-domain**: `const`, `let`, `var`, named function/class bindings,
  destructuring targets, CJS namespace objects, Babel/TypeScript helper output,
  and package entry files.
- **A4 locals-marshaling**: parser binding-kind marks, compiler local mutability
  metadata, runtime frame-slot const guard, module namespace population, and
  loader wrapper variables.
- **A5 emission-shape**: likely parser/compiler/runtime-loader boundary, not a
  package-manager resolver issue, pending Phase 2 line attribution.

### Phase 2 status

Blocked before segmentation. The directive's source artifact is not present on
this host:

```text
/media/jaredef/T7/rusty-bun/parity-results/parity-results-top500-20260529T111702-refined.json
```

Observed filesystem state:

```text
/media/jaredef exists, but /media/jaredef/T7 is absent.
```

Searches performed:

- `find /media/jaredef -path '*parity-results*' -name '*20260529T111702*'`
- `find /home/jaredef -name 'parity-results-top500-20260529T111702-refined.json'`
- `find /home/jaredef -name '*top500*refined*.json'`
- `find /home/jaredef/Developer/cruftless-sidecar -type f | rg 'parity|top500|refined'`
- `rg -n "Assignment to constant" /home/jaredef/Developer/cruftless-r1 /home/jaredef/Developer/cruftless-sidecar`

Findings from the searches:

- No copy of the refined sweep JSON was found under `/media/jaredef`,
  `/home/jaredef`, or `/home/jaredef/Developer/cruftless-sidecar`.
- Existing checked-in top500 baselines under `specs/parity-baselines/` are
  2026-05-22-era artifacts and contain no `Assignment to constant` rows in
  their `rb` payloads.
- `Assignment to constant` does appear in test262 result artifacts, but those
  rows are unrelated to the top500 package-loader cluster and cannot supply the
  required 26 package names or package source lines.
- The directive names five examples (`mathjs`, `abortcontroller-polyfill`,
  `mobx`, `mobx-state-tree`, `postcss-selector-parser`), but the required C4
  segmentation depends on the full 26-list and at least eight exact package
  source-line attributions from the refined sweep or its package sandbox.

### Cross-reference of current const handling

This initial cross-reference is not a substitute for the package-line probe,
but it bounds the likely owner surfaces for the eventual Phase 3 shape:

- Parser binding forms enter through lexical declaration and binding-pattern
  parsing in `pilots/rusty-js-parser/derived/src/parser.rs`.
- Compiler mutability and frame-slot assignment checks are owned by
  `pilots/rusty-js-bytecode/derived/src/compiler.rs`.
- Runtime assignment-to-const surfacing is enforced in the interpreter/frame
  assignment path in `pilots/rusty-js-runtime/derived/src/interp.rs`.
- Package dynamic-import and CJS/ESM namespace bridge behavior routes through
  `pilots/rusty-js-runtime/derived/src/module.rs`.

### C4 result

C4 is **not evaluated** in this round. The data required to segment the 26-row
cluster is missing. Any claimed dominant mechanism would be speculative.

### Phase 3 proposal status

No Phase 3 substrate move is proposed yet. The next required move is apparatus:
make the refined sweep JSON, or an equivalent package sandbox with exact package
failure records, available to R1. Once available, resume this locale at Phase 2
and inspect at least eight package source lines before choosing single-rung
versus multi-rung Phase 3.

## ATC-EXT 0b — Inline source Phase 2 segmentation (2026-05-29)

### Directive

Helmsman resent the cluster inline via CAACP message
`3086cad4-87b3-4cc7-8077-5d173c95bdd2`, preserving the original scope:
Phase 0 + Phase 2 only, no substrate land.

### Inline cluster segmentation

The 26-row cluster groups by the reported reassigned binding:

| Binding | Count | Packages |
|---|---:|---|
| `_interopRequireWildcard` | 10 | `expect`, `@jest/expect`, `sequelize`, `express-validator`, `jest-cli`, `jest-resolve`, `jest-snapshot`, `metro`, `metro-config`, `jest-extended` |
| `_setPrototypeOf` | 8 | `abortcontroller-polyfill`, `mobx`, `mobx-state-tree`, `postcss-selector-parser`, `nunjucks`, `cssnano`, `postcss-nested`, `postcss-preset-env` |
| `_typeof` | 4 | `@mswjs/data`, `pino-pretty`, `class-validator`, `strapi` |
| `_extends` | 1 | `mathjs` |
| `_getRequireWildcardCache` | 1 | `prettier-eslint` |
| `_chalk` | 1 | `jest-each` |
| `cov_toszyysar` | 1 | `commitizen` |

The dominant surface is not user-authored `const x; x = ...`. It is generated
helper code that declares a function and rewrites that function's own binding
on first call as a memoization fast path.

### Empirical sample

External sandbox:
`/home/jaredef/Developer/cruftless-sidecar/results/atc-loader-phase2-r1-20260529T184429Z/sandbox`.

`npm install` populated the sandbox with representative packages. Current
`target/release/cruft` reproduced the inline failure for 15/15 attempted
sample imports; Node imported the first 10/10 sample packages successfully.

Representative source-line attributions:

| Package | Reported binding | Source line |
|---|---|---|
| `mathjs` | `_extends` | `node_modules/@babel/runtime/helpers/extends.js:1-2`: `function _extends() { return module.exports = _extends = ... }` |
| `abortcontroller-polyfill` | `_setPrototypeOf` | `node_modules/abortcontroller-polyfill/dist/umd-polyfill.js:119-120`: `function _setPrototypeOf... { return _setPrototypeOf = ... }` |
| `mobx` | `_setPrototypeOf` | `node_modules/mobx/dist/mobx.cjs.development.js:362-363`: `function _setPrototypeOf... { return _setPrototypeOf = ... }` |
| `postcss-selector-parser` | `_setPrototypeOf` | `node_modules/postcss-selector-parser/dist/selectors/root.js:11`: `function _setPrototypeOf... { _setPrototypeOf = ... }` |
| `nunjucks` | `_setPrototypeOf` | `node_modules/nunjucks/src/environment.js:4`: `function _setPrototypeOf... { _setPrototypeOf = ... }` |
| `expect` | `_interopRequireWildcard` | `node_modules/expect/build/index.js:28`: `function _interopRequireWildcard... { return (_interopRequireWildcard = function ...)(...) }` |
| `class-validator` | `_typeof` | `node_modules/validator/lib/isRgbColor.js:9`: `function _typeof... { return _typeof = ... }` |
| `@mswjs/data` | `_typeof` | reproduced as `_typeof`; package path routes through validator/Babel helper dependency in the installed graph |
| `pino-pretty` | `_typeof` | reproduced as `_typeof`; package path routes through validator/Babel helper dependency in the installed graph |
| `sequelize` | `_interopRequireWildcard` | reproduced as `_interopRequireWildcard`; installed graph contains the same generated helper shape in Babel/Jest-style helpers |

Minimal mechanism smoke:

```js
function _setPrototypeOf(o, p) {
  _setPrototypeOf = Object.setPrototypeOf ? Object.setPrototypeOf.bind() : function (o, p) {
    return o;
  };
  return _setPrototypeOf(o, p);
}
_setPrototypeOf({}, null);
```

Node accepts this. Cruft throws
`TypeError: Assignment to constant variable '_setPrototypeOf'`.

### Current cruft ownership cross-reference

- Runtime enforcement is not the immediate owner. `Op::StoreLocal` in
  `pilots/rusty-js-runtime/derived/src/interp.rs` checks TDZ but does not
  reject const reassignment itself.
- The rejection is compiler-emitted. `compile_plain_assign` and compound
  assignment lowering in
  `pilots/rusty-js-bytecode/derived/src/compiler.rs` call
  `is_const_binding(name)` and emit a TypeError when true.
- The likely false-positive source is
  `compile_function_proto_with_name_hint`: it allocates a self-name slot with
  `VariableKind::Const` for named function expressions **and declarations**.
  Babel helper declarations then see their own function name as const inside
  the body, so the valid self-memoizing assignment compiles to a thrown
  TypeError.
- Top-level function declaration hoisting already allocates the outer
  declaration binding as `VariableKind::Var`; the bug is the inner self-name
  slot being applied too broadly or with the wrong mutability.

### C4 result

C4 reason-coherence **passes**. The inline cluster is coherent under one
dominant mechanism:

1. The top three helper names account for 22/26 rows.
2. The inspected package source lines share the same operational shape:
   generated helper function declaration self-reassignment.
3. A minimal local smoke reproduces the same false positive without any loader
   or package-manager complexity.
4. Node/Bun-compatible behavior is to allow reassignment of a function
   declaration's own binding in this helper shape.

### Proposed Phase 3 move shape

Single-rung compiler fix:

1. Split named function expression self-name binding from function declaration
   body binding in `compile_function_proto_with_name_hint`.
2. Preserve `VariableKind::Const` for genuine named function expressions, whose
   self-name binding is immutable per ECMAScript.
3. For function declarations, either do not allocate the extra self-name slot
   when the declaration binding/upvalue is already available, or allocate it as
   `VariableKind::Var` so helper self-reassignment compiles normally.
4. Gate with the minimal self-reassigning function-declaration smoke plus the
   representative package import sample. Expected direct cluster reclaim:
   most or all of the 22/26 helper rows; the singleton `_chalk`,
   `_getRequireWildcardCache`, and `cov_toszyysar` rows need remeasurement
   after the dominant fix before deciding if they are same-shape or residuals.
