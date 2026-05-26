# private-field-runtime-slots — Trajectory

## PFRS-EXT 0 — 2026-05-26 (founded from PNL/CESS residual)

Founded from the post-CESS focused private-name probe.

Prior state:

- PNL direct smoke: `40/40`.
- Focused PNL after CESS-EXT 1: `140/194`.
- Residual `test 3` family showed private fields leaking as ordinary own string properties (`hasOwnProperty(instance, "#x")` true).

The existing runtime acceptance file already documented the defect: private fields were name-mangled to ordinary `"#name"` properties.

## PFRS-EXT 1 — 2026-05-26 (private slot map, transitional method bridge)

Added `Object.private_fields` and routed compiler-generated `#name` property ops into it.

Implementation notes:

- `Object.private_fields` is traced by GC and excluded from ordinary `properties`, shape slots, `hasOwnProperty`, `Object.keys`, and descriptor paths.
- `Runtime::object_set` and `Op::SetProp` write `#name` keys to private storage.
- `Runtime::object_get` and `Op::GetProp` read `#name` keys from private storage.
- Private reads walk prototype private storage after own storage to preserve current private-method lowering.
- `__install_method__` writes `#name` methods into private storage.
- Private accessors continue to use the existing accessor descriptor path as a transitional bridge.

Verification:

- `cargo check -p rusty-js-runtime`
- `cargo build --release --bin cruft -p cruftless`
- `pilots/private-name-lexing/exemplars/run-exemplars.sh`
  - `PASS=40 FAIL=0 / 40`
- `PNL_EXEMPLARS_LIST=/private/tmp/pnl-focused.txt pilots/private-name-lexing/exemplars/run-exemplars.sh`
  - `PASS=160 FAIL=34 / 194`
- Representative formerly failing row:
  - `language/statements/class/elements/multiple-definitions-private-names.js` now `PASS`.

Movement:

- Focused PNL probe moved from `140/194` to `160/194`.
- The private-field ordinary-property leak family closed.

Residuals:

- optional-chain private-field runtime path,
- async/generator private method runtime semantics,
- remaining statement/expression duplicated rows around those runtime families.
