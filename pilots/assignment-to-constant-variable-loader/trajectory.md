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
