# 2026-05-28-lowering-feature-gap-triage — log

## 2026-05-28 — arc opens

- Keeper selected the `cruft-lowering-feature-gaps` candidate after top-level
  arc survey.
- Required apparatus orientation loaded: `AGENTS.md`,
  `apparatus/docs/repository-apparatus.md`,
  `apparatus/docs/predictive-ruleset.md`,
  `apparatus/docs/standing-rule-13-prospective-application.md`,
  `apparatus/docs/agent-feedback-schema.md`,
  `apparatus/docs/arc-as-coordinate.md`, `apparatus/locales/manifest.json`,
  and `apparatus/locales/CANDIDATES.md`.
- Latest matrix source:
  `pilots/apparatus/test262-categorize/full-suite/results/test262-full-2026-05-27-161641/interpreted.jsonl`.
- Tolerant JSONL pass found 113 current
  `availability/missing-lowering-feature` rows:
  - 68 `compile: super reference outside of a class`
  - 18 `compile: bare \`super\` reference is only valid as \`super(...)\` or \`super.method(...)\``
  - 8 `compile: super reference in a class with no \`extends\` clause`
  - 2 `compile: super(...) outside of a class`
  - 12 `compile: for-in with destructure head not yet supported`
  - 4 `compile: update on non-identifier non-member target not yet supported`
  - 1 `compile: complex assignment target not yet supported`
- Founded `pilots/cruft-lowering-feature-gaps/` as the parent Rule-23
  baseline and partition locale.

## 2026-05-28 — CLFG-EXT 1 baseline

- Ran `T262_ROOT=/Users/jaredfoy/Developer/cruftless-sidecar/test262 pilots/cruft-lowering-feature-gaps/exemplars/run-exemplars.sh`.
- Initial exemplar list had two stale paths; replaced them with current matrix
  paths from the same `super reference outside of a class` cluster.
- Clean baseline result: `CLFG exemplars: PASS=0 FAIL=32 SKIP=0 NOJSON=0 / 32`.
- Finding: the first nested locale should be `super` lowering/context
  propagation unless an existing locale already owns the exact compiler
  diagnostics.

## 2026-05-28 — SRL-EXT 0 child spawn

- Collision check found no active exact owner:
  `for-head-this-super-target/` is parser-only and closed; object/class
  candidates are broader deferred/audit-first surfaces.
- Founded `pilots/cruft-lowering-feature-gaps/super-reference-lowering/`.
- Child suite contains the 22 `super` rows from the CLFG baseline.

## 2026-05-28 — SRL-EXT 1 child baseline

- Ran `T262_ROOT=/Users/jaredfoy/Developer/cruftless-sidecar/test262 pilots/cruft-lowering-feature-gaps/super-reference-lowering/exemplars/run-exemplars.sh`.
- Result: `CLFG exemplars: PASS=0 FAIL=22 SKIP=0 NOJSON=0 / 22`.
- Split surfaced:
  object-literal HomeObject, direct-eval context capture,
  delete/bare-super early-error routing, super property assignment/update, and
  base-class no-extends runtime behavior.
- Standing next move: prefer object-literal HomeObject or super property
  assignment/update; defer direct-eval rows until eval-environment work is
  settled.

## 2026-05-28 — SRL-EXT 2 object-literal HomeObject bridge

- Implemented object-literal HomeObject lowering in the bytecode compiler.
- Added runtime helpers `__install_method_obj__` and `__super_get_home`.
- Object-literal accessors now record home through `__install_accessor_obj__`.
- Focused object rows moved `PASS=0/3` to `PASS=3/3`.
- Full SRL child suite moved `PASS=0 FAIL=22` to `PASS=3 FAIL=19`.
- Verification: `cargo check -p rusty-js-bytecode`, `cargo check -p rusty-js-runtime`,
  `cargo build --bin cruft -p cruftless`, and debug `diff-prod` (`62/112 PASS`).

## 2026-05-28 — SRL-EXT 3 super PutValue base/key ordering

- Implemented object-method `super` compound assignment and update lowering.
- Added `__super_base_home`, `__super_get_base`, and `__super_set`.
- Captured `HomeObject.[[Prototype]]` before computed-key evaluation/coercion,
  so key `toString()` side effects cannot redirect the super base.
- Focused PutValue rows moved `PASS=0/2` to `PASS=2/2`.
- Full SRL child suite moved `PASS=3 FAIL=19` to `PASS=5 FAIL=17`.
- Verification: `cargo check -p rusty-js-bytecode`, `cargo check -p rusty-js-runtime`,
  `cargo build --bin cruft -p cruftless`, and debug `diff-prod` (`62/112 PASS`).
