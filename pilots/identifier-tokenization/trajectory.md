# identifier-tokenization — Trajectory

## IDT-EXT 0 — founding + Rule-23 baseline-inspection (with iteration to corrected reading) (2026-05-25)

**Trigger**: Keeper directive (Telegram 9824) "continue with candidate tokenization." Second Tier-I substrate spawn from the tokenization-above-IR brief.

**Apparatus established**:

- `exemplars/exemplars.txt` — 268 fixtures from `test262/language/identifiers/`.
- Baseline measurement (Rule 23): **PASS=153, FAIL=115 (57.1%)**.

**Baseline-inspection iteration** (Rule 23 in action, including the self-correction this discipline is designed to catch):

**First read (INCORRECT)**: 100% of 115 failures show reason "expected SyntaxError, got String." Interpretation: this is Finding NLC.0 (eval-error-class wrapping issue from Addendum XIV) at extreme concentration. Recommended next move was to land NLC-EXT 1 (runtime eval-error fix).

**Verification probe surfaced the mis-read**: direct probe `(0,eval)("0b2;")` showed cruft eval already throws `SyntaxError` (ctor.name === "SyntaxError") for lex-rejected source. The eval-error-class wrapping was already correct; NLC.0's diagnosis was wrong.

**Corrected read**: "got String" traces to test262's `$DONOTEVALUATE()` harness function, which throws a literal STRING sentinel. For a negative-parse test, the assembled source is `<harness>; $DONOTEVALUATE(); <bad source>`. If cruft's parser correctly REJECTS `<bad source>`, the SyntaxError fires before `$DONOTEVALUATE()` — test passes. If cruft's parser ACCEPTS the source (incorrectly permissive), `$DONOTEVALUATE()` runs and throws its String — `thrown.constructor.name === "String"` — runner reports "expected SyntaxError, got String."

**"expected SyntaxError, got String" therefore means: cruft's parser accepted source that the spec requires it to reject.** The substrate gap is at the lex/parse tier — exactly where the identifier-tokenization locale was intended to operate.

**Direct audit confirms the actual scope** (Finding NLC.2 per findings.md Addendum XV):

```
var break = 1;       → ACCEPT  (spec: reject)
let break = 1;       → ACCEPT  (spec: reject)
const break = 1;     → ACCEPT  (spec: reject)
function break() {}  → ACCEPT  (spec: reject)
class break {}       → ACCEPT  (spec: reject)
function f(break) {} → ACCEPT  (spec: reject)
({break: 1}).break   → ACCEPT  (correct; property names allow ReservedWord)
```

Cruft accepts ReservedWord tokens as BindingIdentifier in virtually every binding context. The substrate work is the §11.6.2 ReservedWord exclusion at `parse_binding_identifier`.

**Rule 23 reflection**: the baseline-inspection rung surfaced the symptom in 100% concentration; the first-read interpretation was wrong; the deeper-probe verification corrected it. **The discipline worked exactly as designed** — inspection HAPPENED, the mis-read was caught BEFORE substrate work committed to the wrong target, and the correct substrate move is now well-scoped. Per Rule 23's design, the first-read is not a guarantee of correctness; the inspection is a guarantee of the check-and-correct cycle.

**Status**: IDT-EXT 0 FOUNDED with corrected scope per Finding NLC.2. IDT-EXT 1 (centralize ReservedWord check in parse_binding_identifier) is the next substrate move. 115 tests in this locale + likely additional fails in adjacent locales waiting.

**Findings**

**Finding IDT.0 (Rule 23 self-correction is a first-class outcome, not a failure)**: this locale's founding produced one mis-read + one correction within a single inspection cycle. The mis-read led to a (non-landed) substrate-move proposal that would have wasted runtime-tier cycles. Rule 23 caught the mis-read at the verification-probe stage before substrate work committed. Standing recommendation: when Rule 23's baseline inspection produces a hypothesis, the follow-on verification-probe step (direct probe in the substrate, NOT just re-reading the failure reasons) is REQUIRED before declaring the inspection complete. The probe is what distinguishes baseline-inspection-as-discipline from baseline-inspection-as-glance. Refines Rule 23's operational reading: "INSPECT a sample of failures AND verify the implied mechanism via direct probe."

---

## IDT-EXT 1 — centralize ReservedWord check across the binding-identifier consumption sites (2026-05-25)

**Trigger**: Keeper directive (Telegram 9826) "Land it."

**Edits** (~30 LOC across 3 sites):

The §11.6.2.1 ReservedWord check (using the existing `is_unconditional_reserved_word` predicate) inserted at three consumption sites cruft uses for BindingIdentifier:

1. `pilots/rusty-js-parser/derived/src/parser.rs::parse_binding_identifier` (line 492+) — central site; covers function/class names via parse_default_*, namespace imports, etc.
2. `pilots/rusty-js-parser/derived/src/parser.rs::parse_binding_target` (line 1268+) — covers var/let/const declarators + destructure leaves + for-head bindings.
3. `pilots/rusty-js-parser/derived/src/stmt.rs::parse_for_statement` for-(var|let|const) plain-id inline-construction site (line 1027+) — bypasses parse_binding_target.

All three reject with SyntaxError-class ParseError; cruft's eval-error-class wrapping (verified in IDT-EXT 0's verification probe) presents these correctly as SyntaxError to JS-tier catch.

**Verification (probes)**:

| Probe | Pre-EXT-1 | Post-EXT-1 |
|---|---|---|
| `var break = 1` | ACCEPT | **REJECT SyntaxError** ✓ |
| `let break = 1` | ACCEPT | **REJECT SyntaxError** ✓ |
| `const break = 1` | ACCEPT | **REJECT SyntaxError** ✓ |
| `function f(break) {}` | ACCEPT | **REJECT SyntaxError** ✓ |
| `({break: 1}).break` (property name; should allow) | ACCEPT | ACCEPT ✓ (preserved) |
| `function break() {}` | ACCEPT | ACCEPT (residual; routes through different parser entry) |
| `class break {}` | ACCEPT | ACCEPT (residual; same shape) |

**Gates**:
- diff-prod: **42/42 PASS, 0 FAIL**
- Random 300 prev-PASS: **300/300, 0 regressions**
- SyntaxError curated cluster: **45/45 (held)**

**Yield**:

| Surface | Pre-EXT-1 | Post-EXT-1 | Δ |
|---|---:|---:|---:|
| IDT exemplars (268-fixture pool) | 153 | **261** | **+108 (+40 pp; 97.4% pass)** |

The 7 residual IDT fails trace to the function-decl + class-decl name paths (probe-confirmed). These go through `parse_function_decl_stmt` + `parse_class_decl_stmt` which inline-construct the BindingIdentifier without routing through the centralized check sites; the EXT 1 attempt added the check there too but the probe shows it didn't fire — implying the function/class-decl path is hit via a DIFFERENT parser entry (perhaps via parse_default_* or via the substatement path) that bypasses both edits. Defer to IDT-EXT 2.

**Findings**

**Finding IDT.1 (3-site centralized check delivers 97% of pool with one constraint)**: the §11.6.2.1 ReservedWord rule is conceptually one check; cruft's parser has 3+ entry points to BindingIdentifier construction; landing the check at 3 sites (parse_binding_identifier, parse_binding_target, for-head plain-id) closes 108/115 fails. Standing recommendation: when a spec rule has one definition but multiple substrate consumption sites, check coverage > rule restatement count. Each new site discovered means scoping IDT-EXT 2+ to find the residual entry points that bypass current sites. Cruft's parser has a hidden function/class-decl entry path that doesn't share the centralized BindingIdentifier consumption sites; IDT-EXT 2 will surface and close it.

**Finding IDT.2 (the eval-class-wrapping mechanism IS working as Addendum XV claims)**: post-EXT-1 probes show that cruft now correctly throws SyntaxError-class for var/let/const/param `break` cases. The eval-error wrapping carries the SyntaxError class to the JS-tier catch as a proper Error instance. This is the empirical confirmation of Addendum XV's NLC.0-revised: the eval-class wrapping was already correct; the substrate gap was at the lex/parse permissiveness.

**Status**: IDT-EXT 1 CLOSED. 261/268 (97.4% of pool). IDT-EXT 2 (close function-decl + class-decl name paths) is the next rung; small substrate move (~5-10 LOC) once the entry path is identified.
