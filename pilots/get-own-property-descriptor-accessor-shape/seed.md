# get-own-property-descriptor-accessor-shape — Seed

## Telos

`Object.getOwnPropertyDescriptor(o, k)` must return an accessor-shape descriptor (with `get` and `set` fields, possibly undefined values) when the property was defined as an accessor — even if both getter and setter values are undefined. Currently cruft returns data-shape (value/writable) for `Object.defineProperty(o, "k", {get:undefined, set:undefined, ...})` because storage uses `Option<Value>` where `Some(_)` requires a non-Undefined value (matched only when `matches!(getter, Value::Object(_))`).

Identified by EIPD.1 sweep extended to "desc1.hasOwnProperty('get') !== true" cluster.

## Apparatus

- `pilots/rusty-js-runtime/derived/src/interp.rs::object_get_own_property_descriptor_via` (line 2194).
- `pilots/rusty-js-runtime/derived/src/interp.rs::find_getter` / `find_getter_pk` / `find_setter` / `find_setter_pk`.
- `pilots/rusty-js-runtime/derived/src/interp.rs::define_property` accessor branch (line 1925-1930).
- Promise.catch "then" lookup at line 2497 (consumes getter via call_function).

## Methodology

Change semantics of PropertyDescriptor.getter/setter from "Some(v) iff callable" to "Some(v) iff field-present-in-descriptor":
- `defineProperty` accessor branch: `final_getter = if has_get_key { Some(getter) } else { existing_getter }` (preserves Value::Undefined as Some-Undefined).
- `find_getter` / `find_getter_pk`: skip entries with `getter=Some(Value::Undefined)` — only return callable getters for the invocation path.
- `find_setter` / `find_setter_pk`: same.
- Promise.catch "then" call site (line 2498): add is_callable check before invoking.
- gOPD: already discriminates via `getter.is_some() || setter.is_some()` — now correctly returns accessor-shape.

## Carve-outs

- This change does not introduce `is_accessor: bool` to PropertyDescriptor (would touch many construction sites). The Option semantics now carry the discriminator.
- defineProperty's `existing_is_accessor` check at line 1877 already uses `is_some()` — correct under new semantics.

## Resume protocol

Read `trajectory.md` tail.
