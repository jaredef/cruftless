# html-like-comment-lexing — Trajectory

## HLCL-EXT 0 — FOUNDING (2026-05-26)

Spawned per keeper directive (Telegram 9865) from the missing-syntax-feature disambiguation map. Pool: 11 fixtures (createdynfn-html-{open,close}-comment-{body,params} + siblings).

Baseline: 0/11 PASS. Verified probe: `Function("\n-->", "")` cruft rejects with `Punct(Gt)` / binding-identifier error.

## HLCL-EXT 1 — LANDED (2026-05-26)

### Edits

1. `pilots/rusty-js-parser/derived/src/lexer.rs::skip_trivia` (~30 LOC):
   - `<!--` branch: 4-byte peek, consume rest of line.
   - `-->` branch: 3-byte peek, gated on `self.at_start || self.saw_line_terminator`, consume rest of line.
   - Reuses `peek_lt_bytes` (LTC-EXT 1's helper) for LT termination.

2. `pilots/rusty-js-runtime/derived/src/intrinsics.rs` (~5 LOC):
   - Function ctor synthesis: `({params}) {{` → `({params}\n) {{`.
   - Spec-aligned per §20.2.1.1.1 step 13.

### Probes (Rule 23 verification at landing)

- `<!-- ignored\n` as line comment → ✓
- `\n--> ignored\n` as line comment → ✓
- `x --> y` (no LT before `-->`) → `x-- > y` operator chain ✓ (gate works)
- `Function("\n-->", "")` → parses ✓ (after Function-ctor fix)
- `Function("-->", "")` → REJECTS ✓ (no leading LT)

### Yield

- HLCL exemplar pool: **0 → 10/11 PASS (+10, 91%)**.
- Diff-prod: 42/42 maintained.
- Cross-locale regression sweep (7 locales): all unchanged.

### Two-stage fix discovered via Rule 23 verification

The lex-only first cut yielded 8/11. Rule 23 verification-probe at landing surfaced that 3 residuals were `createdynfn-html-*-comment-params` — the HTML comment is in PARAMS context, and cruft's Function ctor synthesis put `({params}) {` on one line. The HTML comment then consumed the `)`. Fixed via the second edit (intrinsics.rs newline-placement per spec). Yield rose 8 → 10.

### Residual (1 test)

`createdynfn-no-line-terminator-html-close-comment-params.js`:
- Test: `assert.throws(SyntaxError, () => Function("-->", ""))`.
- cruft's Function ctor `evaluate_module` returns Err on parse-fail; the host propagates the inner CompileError WITHOUT wrapping as a user-catchable SyntaxError. So user code can't catch it as `SyntaxError`.
- Belongs to a separate `dynamic-function-error-wrapping` follow-on (runtime-tier intrinsic fix in the Function ctor's catch arm).

### Findings

**Finding HLCL.1 (Annex-B B.1.3 spans two tiers)**: HTML-like comments cluster has TWO substrate sites: the lex tier (recognize `<!--`/`-->` as comments) AND the runtime tier (Function ctor synthesis must place `\n` per spec). The lex fix alone would have left 3 records failing because the runtime synthesis broke the comment's interaction with FormalParameters. Standing recommendation: when a cluster spans built-in synthesis sites (`Function`, `eval`, etc.), audit the synthesis source format against spec; a single misplaced newline can defeat a tier-correct lex implementation.

**Finding HLCL.2 (negative-test residuals surface error-wrapping gaps)**: The remaining residual is not a missing-syntax-feature record at all — it's a negative test that expects user-catchable `SyntaxError`. cruft throws (correctly rejects), but the throw is host-level CompileError, not user-tier SyntaxError. This pattern likely repeats wherever cruft uses `evaluate_module` from inside an intrinsic (Function, eval, etc.). Standing recommendation: spawn `dynamic-function-error-wrapping` as a follow-on to handle the SyntaxError wrapping across all such intrinsic-eval sites.

### Status

HLCL-EXT 1 CLOSED. Sole residual is dynamic-function-error-wrapping; spawn deferred.
