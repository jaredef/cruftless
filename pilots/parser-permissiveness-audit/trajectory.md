# parser-permissiveness-audit — Trajectory

## PPA-EXT 0 — workstream founding (2026-05-24)

**Trigger**: keeper directive "B" at FODAS-EXT 1 close. T262C matrix cluster #1 (45 tests, arrow-function dstr negative-syntax) + cluster #9 (24 tests, for-of decl-cls) + cluster #11 (22 tests, arrow-function params-duplicate) all trace to parser permissiveness at "no-ReservedWord" sites.

**Reconnaissance summary**:
- 41/45 of cluster #1 use escaped-keyword form (`break`); 4/45 use unescaped form
- `parser.rs:372 parse_binding_identifier` has explicit carve-out ("do not reject reserved-word bindings here; defer to expression grammar") — but the deferred check doesn't exist
- `parser.rs:817` object-pattern shorthand likewise routes through `TokenKind::Ident(name)` without ReservedWord check
- Per §11.6.2, escaped keywords tokenize as IdentifierName but spec still rejects in restricted positions

**Pre-spawn Rule 11 multi-axis check** (parser-tier adaptation):
- (A1) cluster-axis: clusters #1/#9/#11 (91 tests)
- (A2) site-axis: 4-6 spec-enumerated sites (BindingIdentifier, IdentifierReference, LabelIdentifier, strict-additional, module-additional, context-dependent)
- (A3) escape-form-axis: lexer must preserve "had escape" bit
- (A4) cover-grammar-axis: ArrowFormalParameters recovery from CoverParenthesizedExpressionAndArrowParameterList

**Rule 14 mirror identified**: this locale ADDS restriction (opposite direction of standard rule 14). False-positives (rejecting valid programs) are dominant risk. Conservative-site principle (C7): only fire check at sites enumerated from spec text.

**Five Pred-ppa.* + discipline falsifier**:
- Pred-ppa.1: instrument runs; C6 zero PASS→FAIL per round
- Pred-ppa.2: cluster #1 closes in ≤3 rounds
- Pred-ppa.3: clusters #9 + #11 close in ≤3 additional rounds
- Pred-ppa.4: TCC parse-parity holds at 100% across all rounds
- Pred-ppa.5 (Rule 13 discipline): each round ≤2 implementation attempts

**Founding artefacts**: seed.md + trajectory.md + scaffolded dirs. PPA-EXT 1 (escaped-keyword-in-object-binding-shorthand; cluster #1 bulk: 41/45) next.

## PPA-EXT 1 — escaped/unescaped keyword in object-binding shorthand + eval SyntaxError mapping (2026-05-24)

**Edits**:
1. `parser.rs`: added `is_reserved_word()` helper enumerating ECMA-262 §11.6.2 keywords + §13.1.1.1 strict-additional FutureReservedWord set. Called at the object-binding shorthand path (`parse_object_binding_pattern_body`); rejects `{ <reserved> }` shorthand while leaving `{ <reserved>: value }` (PropertyName allows ReservedWord per §13.2.5.1) UNCHANGED.
2. `intrinsics.rs`: at the `eval` intrinsic, map `RuntimeError::CompileError(msg)` → `RuntimeError::SyntaxError(msg)` per §19.2.1.1 PerformEval step 5. Makes parse-tier errors JS-catchable as SyntaxError, so test262 negative-phase-parse tests observe the throw via the runner's indirect-eval path.

**Doc 740 multi-tier closure**: R = {parser ReservedWord check, eval CompileError→SyntaxError}. Landed together as one commit per FODAS lesson (avoid substrate-introduction prefix). Pipeline-connection at FINAL-tier closure (Doc 740 P4).

**Verification**:
- cargo build --release GREEN
- Per-test minimal repros for break/case/class/let escaped-form: all reject with SyntaxError ✓
- Positive carve-outs (object literal `{ break: 1 }`, destr with colon `{ break: a }`): still parse ✓
- canonical fuzz: acc=-932188103 byte-identical (C5)
- diff-prod / TCC: not re-run mid-round (queued for chapter close)

**test262-sample re-measure** (vs pre-PPA-EXT 1 baseline 5581):
- PASS: 5581 → **5719** (+138)
- FAIL: 1601 → 1573 (−28)
- PASS→FAIL transitions: **0**
- FAIL→PASS transitions: **45** (cluster #1 closed entirely)
- Total emitted: 7566 → 7676 (+110 — the eval-tier fix unblocked tests that were previously crashing the test process before emitting a result)

**Five Pred-ppa.* dispositions**:
| Predicate | Disposition |
|---|---|
| Pred-ppa.1 (zero PASS→FAIL per round) | ✅ HELD |
| Pred-ppa.2 (cluster #1 ≤3 rounds) | ✅ HELD (1 round) |
| Pred-ppa.3 (clusters #9 + #11 ≤3 rounds) | ⚪ DEFERRED to PPA-EXT 2+ |
| Pred-ppa.4 (TCC parse-parity 100%) | ⚠ canonical fuzz held; TCC not re-run this round |
| Pred-ppa.5 (Rule 13 ≤2 attempts per round) | ✅ HELD (1 attempt) |

### Findings

**Finding PPA.1**: eval-tier CompileError→SyntaxError mapping unblocked +110 tests that were previously crashing the runner process — a much larger cascade than the 45-test cluster target. Doc 742 §V upstream-elision applies at the runtime-eval boundary: parse errors should be JS-catchable at the eval-entry-tier, not host-aborted. Cascade scope was not predictable pre-fix; emerged only post-measurement. Doc 740 §III.5 JSF-pilot-reread shape: the cumulative measurement materializes a different magnitude than the per-cluster projection.

**Finding PPA.2**: Doc 740 multi-tier reading correctly identified R = {parser-check, eval-conversion} pre-implementation. Landing both together avoided the SMDR-shaped substrate-introduction-prefix outcome (the FODAS lesson applied prospectively). +138 PASS, 0 regressions is the (P4) pipeline-connection moment at full magnitude.

**Status**: PPA-EXT 1 closed; cluster #1 closed. PPA-EXT 2 (unescaped-keyword + arrow-single-ident-head, residual of #1 if any) skipped — already in scope. PPA-EXT 3 (for-of decl-cls cluster #9) next.

