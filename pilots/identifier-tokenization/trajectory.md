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
