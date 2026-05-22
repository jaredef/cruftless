# diff-prod — Trajectory

Pin-Art rung log for the differential prod-test locale. Each rung = one fixture landing OR one substrate gap closed under a fixture's signal.

See [`seed.md`](./seed.md) for telos, anti-telos, apparatus, and ceiling.

---

## Rung-0 — Founding (2026-05-22)

**Status**: closed.

**Substrate**: methodology landed at commit `be37c14b`. Three seed F-category fixtures shipped: `json-roundtrip`, `buffer-encode`, `string-ops`. T7 + nice wrapping at commit `1461efad`.

**First-run finding**: all three fixtures FAILed on a single shared substrate divergence — cruftless's `JSON.stringify` ignored the replacer function's return value (ECMA §25.5.2.4 step 2.b). Pinpointed by the comparator's first-divergent-byte report; a 3-line repro bisected immediately.

---

## Rung-1 — Replacer-honor fix (closed, commit `6cc6a438`)

Substrate fix in `interp.rs` + `generated.rs`:
- Added `json_replacer_stack: Vec<Value>` to Runtime (LIFO so nested stringify gets its own frame).
- `json_stringify_via` pushes args[1] when callable, pops on exit.
- `json_apply_replacer_via` called from `json_serialize_property` right after `json_apply_to_json_via`.

Effect: `string-ops` flipped to PASS.

---

## Rung-2 — Three substrate fixes (closed, commit `c6e546f8`)

Investigation continued through `json-roundtrip` + `buffer-encode` failure surfaces. Three fixes:

1. **JSON.parse UTF-8 mishandling** (intrinsics.rs `json_parse_string`). Pre-fix: `out.push(c as char)` decoded each byte as Latin-1, mangling multi-byte sequences (`中` → `ä¸­`). Fixed to accumulate raw bytes and decode at end via `from_utf8_lossy`. Unicode escapes also corrected.

2. **Buffer.toString and Buffer.from honor encoding** (node_stubs.rs). Pre-fix: both ignored encoding arg. Added case-dispatch on `hex` / `base64` / `base64url` / `latin1` / `binary` / `ascii` / utf8 default. Added `base64_decode` helper.

3. **JSON.stringify integer-key enumeration** (interp.rs `json_serialize_compound_via`). Pre-fix: iterated IndexMap insertion order; integer keys appeared lexically (`"1","10","2"`). Per ECMA §10.1.11 OrdinaryOwnPropertyKeys: integer keys first in numeric order, then string keys in insertion order. Reused the existing `enumerable_own_keys` ordering.

Effect: `json-roundtrip` and `buffer-encode` flipped to PASS.

---

## Rung-3 — Three new fixtures (closed, commit landed with Rung-4)

Three new F-category fixtures:
- `map-set-ops`: Map/Set construction, iteration, conversion, object keys.
- `async-promise`: async/await + Promise.all/allSettled/race/any + microtask ordering.
- `error-throws`: canonical error-throwing surfaces; comparator narrowed to ctor-only (engine-stable field).

---

## Rung-4 — Four more substrate fixes (closed, current HEAD)

Investigation across the three new fixtures surfaced four more substrate gaps:

1. **`new Array(N)` RangeError on invalid length** (intrinsics.rs Array ctor). Per ECMA §22.1.1.2 step 5: throw RangeError when single Number arg is < 0, > 2^32-1, non-finite, or non-integer. Pre-fix `new Array(-1)` silently coerced to length-0 via `n as usize`.

2. **`String.prototype.repeat` practical cap** (interp.rs `string_proto_repeat_via`). Per ECMA §22.1.3.16 step 6 + practical 512 MiB cap. Pre-fix `'x'.repeat(Number.MAX_SAFE_INTEGER)` triggered a 9 PB allocation and **SIGABRT'd cruftless**. This is the locale's first **prod-safety win** — a fixture surfaced a denial-of-service shape that no namespace-shape probe could catch.

3. **`String + Symbol` TypeError** (interp.rs `op_add_rt`). Per ECMA §13.15.3 + §7.1.17 ToString: Symbol cannot be coerced to string. Pre-fix silently emitted the Symbol's description.

4. **`new Map(otherMap)` iterates source storage** (intrinsics.rs Map ctor). Per ECMA §24.1.1.1: iterable arg is iterated for entries. Pre-fix treated arg as length-keyed array, so `new Map(otherMap)` returned empty (Map has no `.length`). Now checks for `__map_data` internal and iterates that storage.

Effect: `error-throws`'s rc-mismatch (SIGABRT) gone; comparator now reports clean 3 fixture-level divergences instead of the SIGABRT noise.

---

## Rung-5 — Pin-Art locale founding (this commit)

**Status**: closed; locale formalized.

**Substrate**: no engine change; `pilots/diff-prod/{seed.md, trajectory.md}` added. The locale's standing is now visible from the engagement's locale list rather than buried in commit messages.

---

## Rung-6+ — Deferred substrate gaps (queued)

Three fixture-level FAILs remain, each a substantive substrate gap warranting its own focused rung:

### Rung-6 — `async-promise` await-on-pending Promise (CLOSED, this commit)

**Substrate fix** (intrinsics.rs __await + job_queue.rs pump_one_tick):
when await hits a Pending promise, synchronously pump the event loop —
drain microtasks then advance one macrotask — until the awaited Promise
settles or queues idle. Bounded at 100k pumps; idle-and-still-pending
throws cleanly. This is a v1 stand-in for proper frame park/resume;
proper suspension is queued as its own substrate rung.

Effect: `async-promise` flipped to PASS. Promise.allSettled / race /
any / await-setTimeout now work end-to-end.

### Rung-7 — `error-throws` const-reassign (CLOSED, this commit)

**Substrate fix** (rusty-js-bytecode/compiler.rs):
Added `is_const_binding(name)` helper that walks the local/enclosing
descriptor chain checking for VariableKind::Const. Both compile_plain_assign
and compile_assign now call it at the Identifier-target branch; on
match, emit code that evaluates RHS for side-effects, pops, then
throws `new TypeError("Assignment to constant variable '<name>'")`.
JS-catchable per ECMA §13.15.4 + §15.2.7.

Effect: `error-throws` flipped to PASS. const-reassignment now throws
TypeError both top-level and inside eval (the same compiler path
handles both). Diff-prod 6/6 PASS — Telos A hit.

### Rung-8 — `map-set-ops` Map-with-object-keys (CLOSED, this commit)

**Substrate fix** (interp.rs `map_storage_key` + `map_decode_key`):
Map storage now encodes object keys as `__objkey@<heap-id>` and stashes the original Value in a parallel `__map_orig_keys` side-channel on the Map instance. set/get/has/delete consult the encoded key; iteration (keys/values/entries/forEach) decodes back through the side-channel so consumers see the original Value, not the encoded string.

Effect: `map-set-ops` flipped to PASS. Spec semantics: SameValueZero by identity for object keys, preserved on iteration.

---

## Discipline checkpoints

After every closed rung, update:
- the **Status** line in `seed.md` if the locale's rung count crosses a threshold worth surfacing
- the rung's status line in this file (`queued` / `closed`)

After every fixture addition, update the **Fixtures shipped** count in `seed.md` §IV.

After every fixture FLIPs from FAIL to PASS due to a substrate landing, append a one-line note to the matching rung in this file with the commit hash.

The locale's value compounds: each fixture is a re-runnable artifact; each substrate fix peels one layer; each deferred gap pre-files a future rung.
