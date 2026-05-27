# tagged-template-object-boundary trajectory

## TTOB-EXT 0 - Spawn and Baseline Adoption (2026-05-27)

Trigger: high-level cluster selection after reading `AGENTS.md`, apparatus locale candidates, the latest full-suite matrix, and the LPA partition docs.

Decision: spawn `tagged-template-object-boundary` rather than entering the raw largest class/private language-lowering cluster. The class/private surface has active recent ownership; this locale is explicitly marked spawn-ready and has a coherent baseline from LPA-EXT 9.

Artifacts:

- `seed.md` formalizes the locale coordinate, invariants, falsifiers, and carve-outs.
- `exemplars/exemplars.txt` captures the 27 `language/expressions/tagged-template/` fixtures used by the LPA baseline.
- `exemplars/run-exemplars.sh` provides a local smoke runner against the repository test262 harness.

Baseline source:

- LPA-EXT 9 recorded 12 pass, 13 fail, and 2 no-json/abort rows over the 27-fixture set.
- The sidecar path recorded in the corpus used the upstream machine path and was not present locally under `/home/jaredef/Developer/cruftless-sidecar`, so EXT 0 reconstructs the fixture set locally.

Initial read:

- Dominant work is TemplateStringsArray object construction/cache/freeze and `.raw` shape.
- Direct eval / realm binding rows are adjacent but should not consume the first substrate rung.
- TCO rows are tracked as residuals and carved out until object-boundary semantics are closed.

Next:

- Run the local exemplar baseline.
- Inspect runtime lowering for tagged template calls and the template object construction path.
- Implement the smallest object-boundary closure that moves `.raw`, freeze, and call argument ordering rows together.

## TTOB-EXT 1 - Local Baseline (2026-05-27)

Command: `./pilots/tagged-template-object-boundary/exemplars/run-exemplars.sh`

Result: `PASS=14 FAIL=13 ABORT=2 / 27 (51.9%)`

Residuals:

- Cache identity: `cache-different-functions-same-site.js`, `cache-differing-expressions-eval.js`, `cache-eval-inner-function.js`, `cache-identical-source-eval.js`, `cache-realm.js`, `cache-same-site-top-level.js`, `cache-same-site.js`.
- Object construction / template map: `constructor-invocation.js`, `template-object-template-map.js`, `template-object.js`.
- Cooked/raw escape boundary: `invalid-escape-sequences.js`.
- TCO carve-out: `tco-call.js`, `tco-member.js` no-json.

Read:

The local run confirms that the first substrate rung is no longer broad call argument ordering or frozen object behavior. Those rows are already passing. The coherent residual is now template-site cache identity plus the remaining object construction/template-map and invalid-escape semantics.

Next:

- Locate the current lowering/runtime path for tagged templates.
- Identify whether template objects are allocated per call, per literal node, or through a reusable template-map slot.
- Prefer a cache-site identity fix before eval/realm broadening unless the implementation has no template-map substrate at all.

## TTOB-EXT 2 - Template Registry Cut (2026-05-27)

Change:

- Parser lowering now passes a stable source-site key into `__template_object__`.
- Runtime now has a `template_registry` map.
- `__template_object__` canonicalizes frozen template objects by site key before returning them to tag calls.

Verification:

- `cargo check -p rusty-js-runtime`
- `cargo build --release -p cruftless`
- `./pilots/tagged-template-object-boundary/exemplars/run-exemplars.sh`

Result: `PASS=18 FAIL=9 ABORT=2 / 27 (66.7%)`

Rows cleared:

- `cache-different-functions-same-site.js`
- `cache-same-site-top-level.js`
- `cache-same-site.js`
- `template-object-template-map.js`

Residuals:

- Eval/realm cache identity: `cache-differing-expressions-eval.js`, `cache-eval-inner-function.js`, `cache-identical-source-eval.js`, `cache-realm.js`.
- Constructor precedence: `constructor-invocation.js`.
- Cooked/raw escape boundary: `invalid-escape-sequences.js`.
- Descriptor shape: `template-object.js`.
- TCO carve-out: `tco-call.js`, `tco-member.js` no-json.

Read:

The cache registry is now an earned substrate coordinate. Remaining cache failures are eval/realm-shaped, which should be handled after descriptor shape and constructor precedence because they may involve broader direct-eval and realm context machinery.

## TTOB-EXT 3 - Template Array Length Descriptor (2026-05-27)

Change:

- `Object.getOwnPropertyDescriptor` now reflects explicit array `length` descriptors when an array has one, instead of always synthesizing the default writable array-length descriptor.
- `__template_object__` installs non-enumerable length descriptors on both the cooked template object and its `.raw` array before freezing them.

Verification:

- `cargo check -p rusty-js-runtime`
- `cargo build --release -p cruftless`
- `./pilots/tagged-template-object-boundary/exemplars/run-exemplars.sh`

Result: `PASS=19 FAIL=8 ABORT=2 / 27 (70.4%)`

Rows cleared:

- `template-object.js`

Residuals:

- Eval/realm cache identity: `cache-differing-expressions-eval.js`, `cache-eval-inner-function.js`, `cache-identical-source-eval.js`, `cache-realm.js`.
- Constructor precedence: `constructor-invocation.js`.
- Cooked/raw escape boundary: `invalid-escape-sequences.js`.
- TCO carve-out: `tco-call.js`, `tco-member.js` no-json.

Read:

The object-shape portion of the boundary is now mostly closed. The remaining non-eval object residual is not a TemplateStringsArray descriptor issue; it is a parser/lowering precedence issue around `new tag\`...\``.

## TTOB-EXT 4 - Tagged Template Precedence Under `new` (2026-05-27)

Change:

- `parse_new_expression` now consumes tagged-template continuations in the callee chain before forming the `Expr::New`.
- This makes `new tag\`x\`` lower as construction of the tag result, not as a call on the newly constructed tag receiver.

Verification:

- `cargo check -p rusty-js-runtime`
- `cargo build --release -p cruftless`
- `./pilots/tagged-template-object-boundary/exemplars/run-exemplars.sh`

Result: `PASS=20 FAIL=7 ABORT=2 / 27 (74.1%)`

Rows cleared:

- `constructor-invocation.js`

Residuals:

- Eval/realm cache identity: `cache-differing-expressions-eval.js`, `cache-eval-inner-function.js`, `cache-identical-source-eval.js`, `cache-realm.js`.
- Cooked/raw escape boundary: `invalid-escape-sequences.js`.
- TCO carve-out: `tco-call.js`, `tco-member.js` no-json.

Read:

The first boundary pass has closed ordinary template-map identity, descriptor shape, and constructor precedence. The remaining work is no longer one object-boundary cluster; it partitions into eval/realm context, raw/cooked lexical preservation, and TCO call lowering.

## TTOB-EXT 5 - Raw/Cooked Invalid Escape Boundary (2026-05-27)

Change:

- Tagged-template lowering now passes separate cooked and raw arrays to `__template_object__`.
- Cooked entries with lexer `cooked: None` lower to `undefined`.
- The helper now constructs `.raw` from the raw array when supplied.
- Template lexing now treats decimal legacy escapes as invalid cooked values for template segments while preserving their raw spelling.

Verification:

- `cargo check -p rusty-js-runtime`
- `cargo build --release -p cruftless`
- `./pilots/tagged-template-object-boundary/exemplars/run-exemplars.sh`

Result: `PASS=21 FAIL=6 ABORT=2 / 27 (77.8%)`

Rows cleared:

- `invalid-escape-sequences.js`

Residuals:

- Eval/realm cache identity: `cache-differing-expressions-eval.js`, `cache-eval-inner-function.js`, `cache-identical-source-eval.js`, `cache-realm.js`.
- TCO carve-out: `tco-call.js`, `tco-member.js` no-json.

Read:

The tagged-template object boundary is closed for ordinary lowering: construction, `.raw`, descriptor shape, freeze behavior, source-site caching, constructor precedence, and raw/cooked invalid escapes all now pass. The remaining semantic failures are context ownership: eval/realm site identity and tail-call lowering.

## TTOB-EXT 6 - Residual Boundary Audit (2026-05-27)

Command: `./pilots/tagged-template-object-boundary/exemplars/run-exemplars.sh`

Result remains `PASS=21 FAIL=6 ABORT=2 / 27 (77.8%)`.

Residual reasons:

- `cache-differing-expressions-eval.js`: `tag is not defined @file://<eval:1:stmt>:1:1`
- `cache-eval-inner-function.js`: `tag is not defined @file://<eval:1:stmt>:1:61`
- `cache-identical-source-eval.js`: `tag is not defined @file://<eval:1:stmt>:1:1`
- `cache-realm.js`: `$262 is not defined @file://<eval:0:stmt>:246:13`
- `tco-call.js`: no-json
- `tco-member.js`: no-json

Read:

This locale should not absorb the remaining rows. The eval failures occur before `GetTemplateObject`: eval code cannot see the containing `tag` binding. That is direct-eval lexical capture / variable environment composition, not template object construction. The realm row is blocked on `$262.createRealm` harness/realm availability. The two TCO rows remain call-lowering residuals.

Closure candidate:

- Ordinary tagged-template object semantics: closed.
- Remaining tagged-template fixtures: delegated to eval lexical capture, cross-realm harness support, and TCO call lowering.
