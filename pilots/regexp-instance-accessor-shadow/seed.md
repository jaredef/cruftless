# regexp-instance-accessor-shadow — Seed

## Telos

`new_regexp` installs source/flags/global/ignoreCase/multiline/sticky/unicode/dotAll/hasIndices/lastIndex as own data properties on every RegExp instance. The first nine should be **accessor getters on RegExp.prototype** (per §22.2.6.{2..10}) that read internal slots, not own data properties on instances. lastIndex IS a data property per §22.2.5.1 but with descriptor `{w:t, e:f, c:f}` — currently `{w:t, e:t, c:t}`.

Identified by RES audit-2 (Gaps B+C+D).

Bridge-shape: engine state in `InternalKind::RegExp(internals)` already has flags + source. The instance shouldn't shadow.

## Apparatus

- `pilots/rusty-js-runtime/derived/src/regexp.rs::new_regexp` (lines 132-141): the over-installation site.
- `pilots/rusty-js-runtime/derived/src/regexp.rs::install_regexp_proto_accessor` (line 1240): the prototype accessor — currently reads `rt.object_get(this, name)` which finds the instance's own shadow. Must read from `internal_kind`'s flags/source directly.

## Methodology

1. Refactor `install_regexp_proto_accessor` to dispatch per-name: source → `re.source`; flags → `re.flags`; boolean flags → `re.flags.contains(c)`. Remove the dependency on `rt.object_get(this, name)`.
2. Delete the source/flags/global/.../hasIndices installations in `new_regexp`.
3. Install lastIndex with explicit descriptor `{w:t, e:f, c:f}` (use `dict_mut().insert` with PropertyDescriptor literal).

## Carve-outs

- Spec also says calling `Object.getOwnPropertyDescriptor(RegExp.prototype, 'source')` should return the accessor descriptor — already the case post-install_regexp_proto_accessor. No change needed.
- `re.source` raw string vs escaped form: spec §22.2.5.12 mandates a specific escape form (`/` → `\/`, line terminators escaped). Deferred — separate locale candidate.
- `flags` accessor spec §22.2.6.3 actually computes flags string from individual flag accessors (calls get global, get ignoreCase, etc.); we'll continue returning the raw flags string for v1.

## Resume protocol

Read `trajectory.md` tail.
