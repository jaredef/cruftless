# statement-declaration-in-body-position — Trajectory

## SDIBP-EXT 0+1 — founding + closure (2026-05-25)

**Trigger**: keeper "Continue" + matrix recon. The for-of/for-in/etc. body-of-control-flow positions accept Declaration where spec requires Statement. 14 test262 fixtures (`for-{in,of}/decl-{cls,fn,let,const,gen,async-fun,async-gen}.js`) fail with "expected SyntaxError, got String" (because $DONOTEVALUATE throws a string after the test's invalid syntax was accepted).

**Edits** (~50 LOC):
- `stmt.rs`: new `parse_substatement()` that rejects `const`, `let<ident-start>`, `class`, `function`, `async function` tokens at body position before delegating to `parse_statement`.
- Replace `parse_statement` → `parse_substatement` at: for body (multiple head shapes), for-in body, for-of body, if consequent, if alternate, while body, do-while body, labelled body.

**Exemplar verification**:
- 14 targeted decl-* tests: PASS=14 FAIL=0 (was 0/14)
- Regression check on previously-passing tests in adjacent directories:
  - for-of (non-dstr): 95 → 95 (0 regressed)
  - for-in: 67 → 67 (0 regressed)
  - if: 0 → 0 (baseline)
  - while: 0 → 0 (baseline)
  - do-while: 0 → 0 (baseline)

Full test262-sample sweep deferred per keeper directive.

### Findings

**Finding SDIBP.1**: small focused parser-tier fix; 14 PASS gain, 0 regressions across 162 previously-passing tests in adjacent dirs. Per-substrate exemplar verification matches the keeper's "exemplar suite" directive — substrate change is bounded; verification scope is matched to substrate scope.

**Status**: CLOSED.
