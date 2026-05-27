# class-elements-static-semantics — Trajectory

## CESS-EXT 0 — 2026-05-26 (founded from PNL redirect)

Founded from PNL.1's Rule-23 finding:

- Direct PrivateIdentifier lexing is healthy (`private-accessor-name` 40/40).
- PNL-EXT 1 closed ZWNJ/ZWJ IdentifierStart, `#constructor`, and same-line class-field terminators.
- Remaining parse-phase failures in the focused private-name probe were 6 rows:
  - `private-literal-name-init-err-contains-arguments` (statement + expression)
  - `nested-private-literal-name-init-err-contains-arguments` (statement + expression)
  - `grammar-privatename-in-computed-property-missing` (statement + expression)

These are class-element static semantics, not private-name lexing.

Immediate move:

CESS-EXT 1 adds a class-body post-parse validator at `stmt.rs::parse_class_body`:

1. collect private names bound in the whole class body,
2. reject `arguments` in field initializer expression trees,
3. reject computed member names that reference undeclared private names.

Status: founded, substrate edit in progress.

## CESS-EXT 1 — 2026-05-26 (narrow class-body static validator)

Added `validate_class_static_semantics` after `parse_class_body` completes the member list.

Implemented checks:

1. collect private names bound by class methods and fields,
2. reject `arguments` reachable from class field initializer expression trees, including nested arrow bodies,
3. reject computed class member names that reference a private name absent from the class's private bound-name set.

Verification:

- `cargo check -p rusty-js-parser`
- `cargo build --release --bin cruft -p cruftless`
- `pilots/private-name-lexing/exemplars/run-exemplars.sh`
  - `PASS=40 FAIL=0 / 40`
- `PNL_EXEMPLARS_LIST=/private/tmp/pnl-focused.txt pilots/private-name-lexing/exemplars/run-exemplars.sh`
  - `PASS=140 FAIL=54 / 194`

Movement:

- Focused PNL probe moved from `134/194` to `140/194`.
- The six PNL-EXT 1 residual parse-phase rows (`expected SyntaxError, got String`) closed:
  - field initializer contains `arguments` (statement + expression),
  - nested field initializer contains `arguments` (statement + expression),
  - computed class member references undeclared private name (statement + expression).

Residuals are now out of this narrow rung:

- async harness SKIPs,
- private brand/runtime semantics,
- generator iterator/runtime gaps,
- possible duplicate-private-name static semantics, pending inspection of the remaining `test 3` assertion family.
