# lex-error-propagation-to-eval-surface — Trajectory

## LEP-EXT 0 — founding (2026-05-25)

**Trigger**: Keeper directive (Telegram 9834) "spawn the substrate locale." Per Finding NLC.3 (NLC trajectory): the lex-tier strict-mode legacy-octal rejection rule fires correctly (eprintln-verified) but its Err return doesn't surface at test262's `(0,eval)` catch. A swallow site exists somewhere in cruft's parser/eval pipeline.

**Empirical anchor** (verified via direct probes during NLC-EXT 1-revised):

```
$ echo '0b2;' | cruft /dev/stdin
cruft: evaluation error: SyntaxError("parse: lex error: invalid radix-prefixed literal ...")
exit=70                            # ✓ direct script-mode error surfaces

$ echo '"use strict"; 00;' | cruft /dev/stdin
                                   # silent, exit=0 — error swallowed somewhere
exit=0                              # ✗ multi-statement source with strict-mode lex rejection: error silent

$ cat > /tmp/f.js <<EOF
function f() { "use strict"; 00; }
console.log("compile-and-run ok");
EOF
$ cruft /tmp/f.js
compile-and-run ok
exit=0                              # ✗ function-body strict-mode lex rejection also silent
```

eprintln debug at the legacy-octal check confirmed: `Lexer::strict_mode == true`, the `return Err(...)` IS executed. The Err does NOT reach `cruftless/src/main.rs:284`'s "cruft: evaluation error:" branch.

**Apparatus established**:

- Substrate sites named in seed (cruftless/src/main.rs, rusty-js-runtime/derived/src/module.rs:920+1451, rusty-js-parser/derived/src/parser.rs::parse_module + parse_statement, stmt.rs::Stmt::Opaque fallback).
- LEP-EXT 1 methodology: binary-search the swallow site via eprintln at each `?` propagation point + each `match { Err => ... }` arm.

**Status**: LEP-EXT 0 FOUNDED. LEP-EXT 1 is the immediate substrate move; the binary-search apparatus is documented. Scope ~10-30 LOC depending on where the swallow site is.

**Findings**

**Finding LEP.0 (Rule 23's verification-probe surfaces engagement-wide gaps)**: NLC's baseline-inspection produced NLC.0 (now-retracted; eval-class-wrapping was already correct). IDT's baseline-inspection refined Rule 23 with the verification-probe step (Finding IDT.0). NLC-EXT 1-revised then APPLIED the verification-probe at substrate landing (eprintln in lexer to confirm reach + Err return) and surfaced NLC.3 — a meta-substrate gap that affects every lex-tier strict-mode rejection. Standing recommendation extends the Rule-23 verification-probe discipline to substrate-landing time, not just baseline-inspection time: when a substrate move depends on cross-tier state (here: parser's strict_mode → lexer's strict_mode → lex Err → eval catch), instrument the substrate move with debug-probes at each tier-crossing to verify reach before declaring landed. The probe is the discipline; the eprintln is the instrument.
