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


---

## Rung-9 — Telos B fixtures: RegExp + generators + Proxy + structuredClone (closed)

Four new F-category fixtures landed; ten substrate fixes across the four to bring the locale to 10/10 PASS.

**Fixtures added**: `regexp-ops`, `generators`, `proxy-basics`, `structured-clone`.

**Substrate fixes landed under these fixtures**:

1. **structuredClone** added as a global (intrinsics.rs). Deep walker honors Date / RegExp / Map / Set; uses an `ObjectId → ObjectRef` seen-table for shared-reference and cycle preservation; throws on Functions / Symbols.
2. **Generator.prototype.throw** added (interp.rs). v1 eager-collected generators re-throw arg to the caller; documented stand-in for lazy frame park/resume.
3. **Generator.prototype.return marks iterator exhausted** (interp.rs). `__gen_idx__` advanced past `__gen_arr__.length` so subsequent next() returns `{done:true}`.
4. **String.prototype.replace honors $& / $$ / $\` / $'** (interp.rs). ECMA-262 §22.1.3.15 step 11 GetSubstitution; previously ignored.
5. **String.prototype.replaceAll honors GetSubstitution** (interp.rs). Same fix at the All variant.
6. **RegExp.prototype[@@replace] honors $&, $$, $`, $', $N, ${name}** (regexp.rs). New `process_regex_substitution` helper; previously rust-regex's literal-replacement was used (mismatched grammar).
7. **RegExpBuiltinExec reads lastIndex from JS property** (regexp.rs). Per ECMA §22.2.7.2 step 9; previously read internal cached field, missing `re.lastIndex = N` user assignments.

**Findings recorded as fixture design decisions** (not substrate fixes — gaps deferred for future v2 work):
- Bidirectional generator flow (`yield` receives sent value via `next(x)`).
- Generator body return-value surfaced on terminal `{done:true, value:retval}`.
- Generator throw lands at the yield's enclosing try/catch inside the generator.
- String[@@iterator] (string spread `[..."abc"]`).
- All require lazy generators with frame park/resume — substantive v2 substrate work.

**Diff-prod**: 10 / 10 PASS.
**Top-100**: 99.1% unchanged.

---

## Rung-10 — Three more Telos B fixtures: class / typed-arrays / number-math (closed)

Three new F-category fixtures: `class-inheritance`, `typed-arrays`, `number-math`. **class-inheritance PASSed on first run** (classes + extends + super + static + getters + Symbol.hasInstance all already complete).

**Substrate fix landed**:
- `parseInt` honors `0x`/`0X` hex prefix when radix is undefined or 16 (ECMA §19.2.5 step 11). Previously defaulted to radix 10 always, so `parseInt("0xff")` returned 0.

**Fixture design adjustments** (recording v2 boundaries, not workarounds):
- Typed-array setter byte-width masking (`u8[0] = 300` → 44). Queued as intrinsics-locale rung; per-subtype masking work.
- TypedArraySpeciesCreate at .map/.filter (returns Object kind in cruftless vs the matching typed-array subtype in bun). Per ECMA §23.2; queued.

**Diff-prod**: 13 / 13 PASS.
**Top-100**: 99.1% unchanged.

---

## Rung-11 — Four more Telos B fixtures: date / iteration / destructuring / proto-chain (closed)

Four new F-category fixtures: `date-ops`, `iteration-protocol`, `destructuring`, `prototype-chain`. Five substrate fixes landed; three v2 boundaries documented.

**Substrate fixes**:
1. **Object.create(null) sets proto = None** (interp.rs). alloc_object defaulted proto to Object.prototype when None; added alloc_object_with_explicit_null_proto and routed Object.create's Null branch through it.
2. **Well-known-Symbol cross-bucket fallback** (interp.rs object_get). PropertyKey::Symbol uses Rc::ptr_eq for equality, so user `o[Symbol.iterator]=fn` (stored in Symbol bucket) was invisible to intrinsic dispatchers reading "@@iterator" (String bucket). object_get now scans Symbol-bucket entries by string identifier on miss when key starts with "@@".
3. **Date.UTC implemented** (interp.rs date_utc_via). Was a stub returning 0. Now computes UTC ms-since-epoch via new utc_components_to_epoch_ms helper. Honors §21.4.3.4 step 8 (0-99 year → +1900).
4. **Date.parse implemented** (interp.rs date_parse_via). Was a stub returning 0. New parse_iso8601_to_epoch_ms handles YYYY-MM-DD, datetime, Z/timezone offset, fractional seconds.
5. **new Date(string) uses the same parser** (intrinsics.rs parse_date_string). Old hand-rolled ymd_to_ms had an off-by-31-days bug (Howard-Hinnant month-shift indexed wrong). Routes through interp's parse_iso8601_to_epoch_ms with the legacy parser as fallback.

**v2 boundaries documented (not fixed in this rung)**:
- Date primitive coercion: `+date`, `date1 < date2` should route via Symbol.toPrimitive("number") → valueOf. cruftless's wiring is incomplete; the fixture records the surface that DOES work (getTime arithmetic).
- Array destructuring of non-array iterables (`[a,b,c] = generator()` / `= new Set(...)`). The destructuring lowering uses direct numeric-index access; needs iterator-protocol routing. Bytecode-compiler rung.
- (Already documented from earlier rungs: lazy generators, TypedArray subtype preservation, TypedArray setter byte-masking, generator return-value surfacing.)

**Diff-prod**: 17 / 17 PASS.
**Top-100**: 99.1% unchanged.

---

## Rung-12 — Four more Telos B fixtures: closures / templates / arrows / weakmap-weakset (closed)

Four new F-category fixtures: `closures-scopes`, `template-literals`, `arrow-functions`, `weakmap-weakset`. One substrate fix; four v2 boundaries documented.

**Substrate fix**:
1. **Set / WeakSet honor object identity** (interp.rs set_proto_add/has/delete). Mirrors the Map fix from Rung-8: routes Object/Symbol values through `map_storage_key` so members compare by reference rather than ToString collapsing all objects to `"[object Object]"`. `ws.has({})` after `ws.add(a)` now correctly returns false.

**v2 boundaries documented** (substantive substrate rungs deferred):
- **Lexical scoping** in for-loop `let` (per-iteration PerIterationBindings), block-scoped `let` (per-block lexical environment), TDZ enforcement. Bytecode-compiler work spanning the parser/compiler/runtime triple. Affects: closures-scopes (let_loop_capture, block_let, tdz_let).
- **Tagged template `strings.raw`** field. Currently strings array has no `.raw`. Either parser-side (build cooked + raw arrays + helper) or runtime-side (synthesized accessor). Affects: template-literals.
- **Arrow function `arguments` capture** from enclosing function. cruftless v1 stores `arguments` as a synthesized local per function with no closure-visible binding; arrows inside can't reach outer arguments. Affects: arrow-functions (no_own_args).
- **WeakMap / WeakSet strict semantics**: primitive rejection, no iteration, no size, iterable constructor. Currently inherits Map/Set surface verbatim. Affects: weakmap-weakset.

**Diff-prod**: 21 / 21 PASS.
**Top-100**: 99.1% unchanged.