# regexp-proto-test-coercion — Seed

## Telos

Align RegExp.prototype.test with ECMA-262 §22.2.5.5:
1. length = 1 (was 0)
2. ToString-coerce the argument via the full §7.1.17 path (@@toPrimitive → toString → valueOf), not the static abstract_ops::to_string (which returned "[object Object]" for any Object).
3. Route through RegExpExec so sticky/global lastIndex bookkeeping matches the .exec path.

## Apparatus

- `pilots/rusty-js-runtime/derived/src/regexp.rs` lines 493+ — RegExp.prototype.test registration.
- `regexp_exec` (same file) — the shared §22.2.5.2 RegExpBuiltinExec used by .exec.

## Methodology

1. Probe: `var re=/\d+/; re.test({toString(){return "abc456"}})` returned `false` (no coercion); must return `true`.
2. Probe: `RegExp.prototype.test.length` returned 0; must return 1.
3. Switch register_method → register_intrinsic_method(..., 1, ...). Replace body with: coerce_to_string + regexp_exec + `!= Null`.

## Carve-outs

- y-fail-lastindex / y-fail-return (2 tests): sticky-flag failure-path lastIndex reset semantics — separate substrate fix at regexp_exec, not at the .test surface.

## Resume protocol

Read `trajectory.md` tail.
