# array-search-arg-strict-coerce — Seed

## Telos

`Array.prototype.{at, indexOf, lastIndexOf, includes}` user-arg coercion (index / fromIndex) uses static `abstract_ops::to_number` which returns NaN for Symbol args instead of throwing TypeError per §7.1.4. Spec mandates ToIntegerOrInfinity(arg), and ToNumber(Symbol) throws. Also `includes` is missing fromIndex support entirely.

Identified by top-failure-reason audit ("Expected a TypeError to be thrown but no exception was thrown at all", 36 in Array.prototype cluster).

Bug pattern is RPTC.7's extension to to_number — same shape, different abstract op.

## Apparatus

- `pilots/rusty-js-runtime/derived/src/interp.rs::array_proto_at_via` (line 4261)
- `pilots/rusty-js-runtime/derived/src/interp.rs::array_proto_last_index_of_via` (line 4299)
- `pilots/rusty-js-runtime/derived/src/interp.rs::array_proto_index_of_via` (line 4519)
- `pilots/rusty-js-runtime/derived/src/interp.rs::array_proto_includes_via` (line 4539) — missing fromIndex entirely.

## Methodology

Switch user-arg static `abstract_ops::to_number(v)` to dispatching `self.coerce_to_number(v)?`. Throws TypeError on Symbol; dispatches @@toPrimitive/valueOf/toString for Objects. Also add fromIndex support to includes per §23.1.3.14 step 4-5.

## Carve-outs

- Other Array methods with user-arg to_number coercion (slice, splice, etc.): separate sub-locale if they fail.
- ToIntegerOrInfinity vs ToNumber: spec mandates the former (truncates fractions, clamps ±Infinity). For at/indexOf/lastIndexOf, `as i64` truncates which is close enough; full ToIntegerOrInfinity helper deferred.

## Resume protocol

Read `trajectory.md` tail.
