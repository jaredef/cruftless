# Test262 fixture for cruftless

Per-test runner harness for ECMA-262 spec conformance tests. The
upstream test262 (https://github.com/tc39/test262) suite is large
(~50k tests) and uses a YAML-frontmatter protocol the cruftless
fixture system can drive directly via cruftless's eval surface
(P59.E4 indirect eval).

## Layout

- `runner.mjs` — cruftless-side harness. Reads a single test file
  named via `T262_TEST_PATH`, parses the YAML frontmatter per
  test262's [INTERPRETING.md](https://github.com/tc39/test262/blob/main/INTERPRETING.md),
  prepends the required harness files, evaluates the test through
  indirect eval, and emits a one-line JSON result.
- `harness/` — minimal vendored stand-ins for test262's harness
  scripts (`sta.js`, `assert.js`). Cover the assertion surface the
  curated subset uses: `assert`, `assert.sameValue`,
  `assert.notSameValue`, `assert.throws`, `assert.compareArray`.
  Upstream's full `harness/` directory has ~30 additional helpers
  (`propertyHelper.js`, `nativeFunctionMatcher.js`, etc.) that the
  current curated tests don't reference; add them here as needed
  when expanding coverage.
- `tests/` — curated, mirror-shaped test files. The layout mirrors
  test262's `test/` directory (`built-ins/Array/prototype/map/...`,
  `built-ins/String/prototype/...`, etc.). Initial set is a small
  smoke-coverage subset targeting the most-frequently-exercised
  intrinsic surfaces.
- `run.sh` — shell driver. Iterates `.js` files under `tests/`
  (or under the path supplied as `$1`), invokes cruftless on each
  via `runner.mjs`, tallies PASS / FAIL / SKIP / TIMEOUT.

## Usage

    # All curated tests:
    ./run.sh

    # Subset under a path:
    ./run.sh tests/built-ins/Array

    # Single file:
    ./run.sh tests/built-ins/Array/prototype/map/basic.js

Per-test wall-clock cap: 10s. Exit code 0 if no failures or timeouts.

The runner uses indirect eval to evaluate each test. Frontmatter
flags currently honored: `module` / `async` / `raw` → SKIP with
recorded reason. `negative.{phase,type}` → expects a throw of the
named error type.

Strict-mode handling: `onlyStrict` prepends `"use strict";`.
`noStrict` runs sloppy. Default (neither flag) runs sloppy.
Strict-form coverage of tests that should run in both forms is
deferred — would need two invocations per test.

## Per Doc 729 / seed §V

The fixture is itself a conformance-evidence apparatus per Doc 708
sub-criterion 5's secondary signal ("WPT-style pass rate per surface
becomes an operational comparison"). The numbers it produces are
complementary to the parity-top500 byte-shape sweep: the parity
sweep measures shape-vs-Bun; test262 measures conformance-vs-spec.

## Initial smoke (2026-05-18, post-P60.E4)

The curated 7-test sample currently lands 2 PASS / 5 FAIL. The
failures are real substrate gaps the fixture surfaced:

1. **Native function `.length` is 0** instead of spec-correct arity
   (Array.prototype.map.length = 1, Object.keys.length = 1,
   String.prototype.includes.length = 1). `install_function_meta_props`
   stores length but the native install paths don't set the
   spec-correct value.
2. **TypeError throw on null/undefined this** — Array.prototype.map
   throws Object-typed rejection instead of TypeError. Related to
   P58.E5's reject-as-Error fix but only applied at the dynamic-
   import boundary; native method receivers still wrap as strings.
3. **Object.freeze not enforcing immutability** — sloppy-mode
   property mutation succeeds despite freeze; Object.isFrozen
   returns the right value. The freeze marker is there, but the
   Set path doesn't honor it.

Each failure is its own substrate move. The fixture's value: it
surfaces these spec-divergences at the conformance tier without
waiting for a downstream consumer probe to project them onto a
parity FAIL.

## Expanding the curated set

Upstream test262 lives at https://github.com/tc39/test262 and is
git-cloneable. To add coverage from upstream:

1. Identify a target chapter (e.g., `test/built-ins/Array/prototype/`).
2. Vendor selected `.js` files under
   `host/tests/test262/tests/built-ins/Array/prototype/...`.
3. Vendor any additional `harness/*.js` files the new tests'
   `includes:` frontmatter references.
4. Run `./run.sh` and triage failures.

The harness/runner support YAML-lite frontmatter parsing; full YAML
support is deferred until a curated test that requires it (block
strings, multi-line arrays) lands in the basket.
