//! Intrinsic prototype objects per Tier-Ω.5.a.
//!
//! Allocates and installs %Object.prototype%, %Array.prototype%,
//! %String.prototype%, %Function.prototype%, %Promise.prototype%, and
//! %Number.prototype%. Each prototype is an ordinary Object stashed on
//! `Runtime` so the alloc-time proto-wiring path (see
//! `Runtime::alloc_object`) finds it by InternalKind. Primitive method
//! dispatch (string.toUpperCase(), (5).toFixed(2)) routes through the
//! property-read paths in `interp.rs`'s GetProp handler, which look up
//! `string_prototype` / `number_prototype` directly when the receiver is
//! a primitive — no wrapper allocation.
//!
//! `this` reaches each prototype method via `Runtime::current_this()`,
//! which `call_function` stashes around every NativeFn invocation.

use crate::abstract_ops;
use crate::interp::{Runtime, RuntimeError};
use crate::value::{
    BoundFunctionInternals, FunctionInternals, InternalKind, NativeFn, Object, ObjectRef,
    PromiseReaction, PromiseStatus, Value,
};
use std::collections::HashMap;
use std::rc::Rc;

impl Runtime {
    /// Allocate and wire every intrinsic prototype. Must run before any
    /// other intrinsic so subsequent `alloc_object` calls pick up the
    /// stashes. Idempotent in practice (each call would clobber prior
    /// stashes — install_intrinsics calls it exactly once).
    pub fn install_prototypes(&mut self) {
        // The Object.prototype object is itself an Ordinary, but it must
        // not inherit from itself — explicitly install with proto: None
        // before any stash is set, which the alloc-time wiring respects
        // since no proto is installed yet.
        let object_proto = self.alloc_object(Object::new_ordinary());
        self.object_prototype = Some(object_proto);

        // Now allocate the rest; each will pick up object_prototype as
        // its `proto` automatically via the alloc-time wiring, which is
        // exactly what Array/Function/Promise/String/Number prototypes
        // want per spec (every prototype inherits from Object.prototype).
        let array_proto = self.alloc_object(Object::new_ordinary());
        let function_proto = self.alloc_object(Object::new_ordinary());
        let promise_proto = self.alloc_object(Object::new_ordinary());
        let string_proto = self.alloc_object(Object::new_ordinary());
        let number_proto = self.alloc_object(Object::new_ordinary());
        self.array_prototype = Some(array_proto);
        self.function_prototype = Some(function_proto);
        self.promise_prototype = Some(promise_proto);
        self.string_prototype = Some(string_proto);
        self.number_prototype = Some(number_proto);

        // RS-EXT 2b: mirror into realm 0 (the primordial RealmRecord).
        // Later commits will flip the dispatch direction so that the
        // Runtime's intrinsic-prototype fields are reads-from-current-
        // realm, but for now both views are kept in sync; the realm-
        // record is the canonical source-of-truth once enter_realm /
        // exit_realm swap them in RS-EXT 2e.
        self.realms[0].object_prototype = Some(object_proto);
        self.realms[0].array_prototype = Some(array_proto);
        self.realms[0].function_prototype = Some(function_proto);
        self.realms[0].promise_prototype = Some(promise_proto);
        self.realms[0].string_prototype = Some(string_proto);

        install_object_proto(self, object_proto);
        install_array_proto(self, array_proto);
        install_string_proto(self, string_proto);
        install_function_proto(self, function_proto);
        install_promise_proto(self, promise_proto);
        install_number_proto(self, number_proto);

        // Tier-Ω Round 1 (2026-05-21): generator + async-generator
        // prototype chain per ECMA-262 §27.3 / §27.4 / §27.5.
        //
        // Each *Function.prototype is the [[Prototype]] of fn objects
        // created from generator / async-generator declarations. Its
        // .prototype property is the *Generator prototype assigned to
        // each generator's fn.prototype. The *Generator prototype's
        // [[Prototype]] is the *Iterator prototype (sync) or
        // %AsyncIteratorPrototype% (async).
        //
        // First-cut: bare allocation with [[Prototype]] chain. Method
        // installation (next/return/throw, [Symbol.iterator], etc.) is
        // queued; ponyfills that only walk the chain (e.g.
        // @sec-ant/readable-stream) work without methods present.

        let iter_proto = self.alloc_object(Object::new_ordinary());
        let gen_proto = self.alloc_object(Object::new_ordinary());
        self.obj_mut(gen_proto).proto = Some(iter_proto);
        let gen_fn_proto = self.alloc_object(Object::new_ordinary());
        self.obj_mut(gen_fn_proto).dict_mut().insert(
            "prototype".into(),
            crate::value::PropertyDescriptor {
                value: Value::Object(gen_proto),
                writable: false,
                enumerable: false,
                configurable: false,
                getter: None,
                setter: None,
            },
        );
        self.iterator_prototype = Some(iter_proto);
        self.generator_prototype = Some(gen_proto);
        self.generator_function_prototype = Some(gen_fn_proto);

        let async_iter_proto = self.alloc_object(Object::new_ordinary());
        let async_gen_proto = self.alloc_object(Object::new_ordinary());
        self.obj_mut(async_gen_proto).proto = Some(async_iter_proto);
        let async_gen_fn_proto = self.alloc_object(Object::new_ordinary());
        self.obj_mut(async_gen_fn_proto).dict_mut().insert(
            "prototype".into(),
            crate::value::PropertyDescriptor {
                value: Value::Object(async_gen_proto),
                writable: false,
                enumerable: false,
                configurable: false,
                getter: None,
                setter: None,
            },
        );
        self.async_iterator_prototype = Some(async_iter_proto);
        self.async_generator_prototype = Some(async_gen_proto);
        self.async_generator_function_prototype = Some(async_gen_fn_proto);
    }
}

// ──────────────── %Object.prototype% ────────────────

fn install_object_proto(rt: &mut Runtime, host: ObjectRef) {
    // E35: Object.prototype.{toString, hasOwnProperty, valueOf} routed through IR.
    register_intrinsic_method(rt, host, "toString", 0, |rt, args| {
        crate::generated::object_prototype_to_string(rt, rt.current_this(), args)
    });
    register_intrinsic_method(rt, host, "hasOwnProperty", 1, |rt, args| {
        crate::generated::object_prototype_has_own_property(rt, rt.current_this(), args)
    });
    register_intrinsic_method(rt, host, "valueOf", 0, |rt, args| {
        crate::generated::object_prototype_value_of(rt, rt.current_this(), args)
    });
    // Tier-Ω.5.DDDDDDDD: Object.prototype.__defineGetter__/__defineSetter__
    // per ECMA Annex B.2.2.2/2.2.3 (legacy but ubiquitous — pg, slonik,
    // sockjs, mongoose use them at module-init for shape augmentation).
    // IR-EXT 56: __define*__ / __lookup*__ now route through generated.rs.
    register_intrinsic_method(rt, host, "__defineGetter__", 1, |rt, args| {
        crate::generated::object_proto_define_getter(rt, rt.current_this(), args)
    });
    register_intrinsic_method(rt, host, "__defineSetter__", 1, |rt, args| {
        crate::generated::object_proto_define_setter(rt, rt.current_this(), args)
    });
    register_intrinsic_method(rt, host, "__lookupGetter__", 1, |rt, args| {
        crate::generated::object_proto_lookup_getter(rt, rt.current_this(), args)
    });
    register_intrinsic_method(rt, host, "__lookupSetter__", 1, |rt, args| {
        crate::generated::object_proto_lookup_setter(rt, rt.current_this(), args)
    });
    // Tier-Ω.5.jjjj: Object.prototype.propertyIsEnumerable per ECMA-262
    // §20.1.3.4. Returns true if the receiver has an own enumerable
    // property at the given key. v1 returns true for any own property
    // (we don't track enumerable bit precisely).
    register_intrinsic_method(rt, host, "propertyIsEnumerable", 1, |rt, args| {
        crate::generated::object_prototype_property_is_enumerable(rt, rt.current_this(), args)
    });
    register_intrinsic_method(rt, host, "isPrototypeOf", 1, |rt, args| {
        crate::generated::object_prototype_is_prototype_of(rt, rt.current_this(), args)
    });
    register_intrinsic_method(rt, host, "toLocaleString", 0, |rt, args| {
        crate::generated::object_prototype_to_locale_string(rt, rt.current_this(), args)
    });
}

// ──────────────── %Array.prototype% ────────────────

fn install_array_proto(rt: &mut Runtime, host: ObjectRef) {
    // Ω.5.P61.E21: Array.prototype.toString per ECMA §23.1.3.34 —
    // dispatch to this.join, falling back to Object.prototype.toString
    // form when join is not callable. Pre-E21 cruftless inherited
    // Object.prototype.toString which returned "[object Array]",
    // breaking String([1])/ToPrimitive(arr) → joined-string and the
    // ToPropertyKey-on-Array test262 tests.
    register_intrinsic_method(rt, host, "toString", 0, |rt, args| {
        crate::generated::array_prototype_to_string(rt, rt.current_this(), args)
    });
    // E31: mutators routed through IR.
    register_intrinsic_method(rt, host, "push", 1, |rt, args| {
        crate::generated::array_prototype_push(rt, rt.current_this(), args)
    });
    register_intrinsic_method(rt, host, "pop", 0, |rt, args| {
        crate::generated::array_prototype_pop(rt, rt.current_this(), args)
    });
    register_intrinsic_method(rt, host, "shift", 0, |rt, args| {
        crate::generated::array_prototype_shift(rt, rt.current_this(), args)
    });
    register_intrinsic_method(rt, host, "unshift", 1, |rt, args| {
        crate::generated::array_prototype_unshift(rt, rt.current_this(), args)
    });
    // E30: routed through IR.
    register_intrinsic_method(rt, host, "indexOf", 1, |rt, args| {
        crate::generated::array_prototype_index_of(rt, rt.current_this(), args)
    });
    register_intrinsic_method(rt, host, "includes", 1, |rt, args| {
        crate::generated::array_prototype_includes(rt, rt.current_this(), args)
    });
    // Tier-Ω.5.cccccc: Array.prototype.reverse per ECMA-262 §23.1.3.21.
    // micromark slices events then reverses; without this, .reverse() was
    // undefined and every state-machine token finalization failed.
    register_intrinsic_method(rt, host, "reverse", 0, |rt, args| {
        crate::generated::array_prototype_reverse(rt, rt.current_this(), args)
    });
    // E32: slice/splice/concat/join/at/fill/lastIndexOf/reduceRight/copyWithin/flat/flatMap routed through IR.
    register_intrinsic_method(rt, host, "slice", 2, |rt, args| {
        crate::generated::array_prototype_slice(rt, rt.current_this(), args)
    });
    // Tier-Ω.5.xxx: Array.prototype.splice per ECMA-262 §23.1.3.31.
    // Removes deleteCount elements starting at start, optionally
    // inserting items in their place. Returns the removed elements.
    // object-hash uses splice on its internal stream buffer.
    register_intrinsic_method(rt, host, "splice", 2, |rt, args| {
        crate::generated::array_prototype_splice(rt, rt.current_this(), args)
    });
    register_intrinsic_method(rt, host, "concat", 1, |rt, args| {
        crate::generated::array_prototype_concat(rt, rt.current_this(), args)
    });
    register_intrinsic_method(rt, host, "join", 1, |rt, args| {
        crate::generated::array_prototype_join(rt, rt.current_this(), args)
    });
    register_intrinsic_method(rt, host, "at", 1, |rt, args| {
        crate::generated::array_prototype_at(rt, rt.current_this(), args)
    });
    register_intrinsic_method(rt, host, "fill", 1, |rt, args| {
        crate::generated::array_prototype_fill(rt, rt.current_this(), args)
    });
    register_intrinsic_method(rt, host, "flat", 0, |rt, args| {
        crate::generated::array_prototype_flat(rt, rt.current_this(), args)
    });
    register_intrinsic_method(rt, host, "flatMap", 1, |rt, args| {
        crate::generated::array_prototype_flat_map(rt, rt.current_this(), args)
    });
    // Ω.5.P63.E1 (rusty-js-ir Tier 1.5): Array.prototype.map per ECMA
    // §23.1.3.20 is now routed through the IR-lowered implementation in
    // src/generated.rs. The hand-written version is preserved in git
    // history; if a regression surfaces, the IR can be patched (and
    // re-lowered) rather than this site edited directly.
    register_intrinsic_method(rt, host, "map", 1, |rt, args| {
        crate::generated::array_prototype_map(rt, rt.current_this(), args)
    });
    register_intrinsic_method(rt, host, "forEach", 1, |rt, args| {
        // Ω.5.P63.E2: routed through IR-lowered generated::array_prototype_for_each.
        crate::generated::array_prototype_for_each(rt, rt.current_this(), args)
    });
    // Ω.5.P63.E2: filter routed through IR-lowered generated::array_prototype_filter.
    register_intrinsic_method(rt, host, "filter", 1, |rt, args| {
        crate::generated::array_prototype_filter(rt, rt.current_this(), args)
    });
    register_intrinsic_method(rt, host, "reduce", 1, |rt, args| {
        crate::generated::array_prototype_reduce(rt, rt.current_this(), args)
    });
    // Ω.5.P63.E3: find routed through IR-lowered generated::array_prototype_find.
    register_intrinsic_method(rt, host, "find", 1, |rt, args| {
        crate::generated::array_prototype_find(rt, rt.current_this(), args)
    });
    // Ω.5.P63.E2: some routed through IR-lowered generated::array_prototype_some.
    register_intrinsic_method(rt, host, "some", 1, |rt, args| {
        crate::generated::array_prototype_some(rt, rt.current_this(), args)
    });
    register_intrinsic_method(rt, host, "@@iterator", 0, |rt, _args| {
        let id = to_array_this(rt)?;
        Ok(Value::Object(crate::iterator::make_array_iterator(rt, id)))
    });
    // Tier-Ω.5.j.proto: Array.prototype.sort(comparator?).
    // Mutates `this` in place, returns `this`. Stable.
    // - No comparator: ToString each element, lexicographic compare.
    // - With comparator: call comparator(a,b); sign of return → Ordering.
    // v1 ignores spec's sparse-array semantics; sorts dense own indices 0..length-1.
    register_intrinsic_method(rt, host, "sort", 1, |rt, args| {
        crate::generated::array_prototype_sort(rt, rt.current_this(), args)
    });
    // Ω.5.P63.E2: every routed through IR-lowered generated::array_prototype_every.
    register_intrinsic_method(rt, host, "every", 1, |rt, args| {
        crate::generated::array_prototype_every(rt, rt.current_this(), args)
    });
    // Tier-Ω.5.P24.E2.array-proto-iter: Array.prototype.entries/keys/values
    // per ECMA §23.1.3.4/§23.1.3.16/§23.1.3.32. Eager-materialize an array
    // of [i, v] / i / v entries, matching the for-of-array-compatible shape
    // used by Map.prototype.entries above. Surfaces from the Ω.5.P24.E1
    // proto-chain probe: arktype's constraintKinds.entries() lands here.
    register_intrinsic_method(rt, host, "entries", 0, |rt, args| {
        crate::generated::array_prototype_entries(rt, rt.current_this(), args)
    });
    register_intrinsic_method(rt, host, "keys", 0, |rt, args| {
        crate::generated::array_prototype_keys(rt, rt.current_this(), args)
    });
    register_intrinsic_method(rt, host, "values", 0, |rt, args| {
        crate::generated::array_prototype_values(rt, rt.current_this(), args)
    });

    // Ω.5.P61.E6: complete the Array.prototype surface per ECMA §23.1.3.
    // Adds findIndex / findLast / findLastIndex / reduceRight / lastIndexOf /
    // copyWithin / toReversed / toSorted / toSpliced / with / toLocaleString.
    // Each spec-arity per §23.1.3 + ECMA 2023 additions.

    // Ω.5.P63.E3: findIndex routed through IR-lowered generated::array_prototype_find_index.
    register_intrinsic_method(rt, host, "findIndex", 1, |rt, args| {
        crate::generated::array_prototype_find_index(rt, rt.current_this(), args)
    });

    register_intrinsic_method(rt, host, "findLast", 1, |rt, args| {
        crate::generated::array_prototype_find_last(rt, rt.current_this(), args)
    });
    register_intrinsic_method(rt, host, "findLastIndex", 1, |rt, args| {
        crate::generated::array_prototype_find_last_index(rt, rt.current_this(), args)
    });

    register_intrinsic_method(rt, host, "reduceRight", 1, |rt, args| {
        crate::generated::array_prototype_reduce_right(rt, rt.current_this(), args)
    });
    register_intrinsic_method(rt, host, "lastIndexOf", 1, |rt, args| {
        crate::generated::array_prototype_last_index_of(rt, rt.current_this(), args)
    });
    register_intrinsic_method(rt, host, "copyWithin", 2, |rt, args| {
        crate::generated::array_prototype_copy_within(rt, rt.current_this(), args)
    });

    register_intrinsic_method(rt, host, "toReversed", 0, |rt, args| {
        crate::generated::array_prototype_to_reversed(rt, rt.current_this(), args)
    });
    register_intrinsic_method(rt, host, "toSorted", 1, |rt, args| {
        crate::generated::array_prototype_to_sorted(rt, rt.current_this(), args)
    });
    register_intrinsic_method(rt, host, "toSpliced", 2, |rt, args| {
        crate::generated::array_prototype_to_spliced(rt, rt.current_this(), args)
    });
    register_intrinsic_method(rt, host, "with", 2, |rt, args| {
        crate::generated::array_prototype_with(rt, rt.current_this(), args)
    });
    register_intrinsic_method(rt, host, "toLocaleString", 0, |rt, args| {
        crate::generated::array_prototype_to_locale_string(rt, rt.current_this(), args)
    });
}

fn clamp_index(i: i64, len: i64) -> i64 {
    let v = if i < 0 { (len + i).max(0) } else { i.min(len) };
    v
}

// ──────────────── %String.prototype% ────────────────

/// Ω.5.P62.E13: IsRegExp per ECMA §7.2.8 — checks @@match first
/// (allowing custom RegExp-like duck types to opt in/out) then falls
/// back to the [[RegExpMatcher]] internal slot (InternalKind::RegExp).
fn is_regexp_like(rt: &mut Runtime, v: &Value) -> Result<bool, RuntimeError> {
    let id = match v {
        Value::Object(id) => *id,
        _ => return Ok(false),
    };
    let matcher = rt.read_property(id, "@@match")?;
    match matcher {
        Value::Undefined => Ok(matches!(rt.obj(id).internal_kind, InternalKind::RegExp(_))),
        _ => Ok(crate::abstract_ops::to_boolean(&matcher)),
    }
}

fn install_string_proto(rt: &mut Runtime, host: ObjectRef) {
    // Ω.5.P63.E21: case family routed through IR.
    register_intrinsic_method(rt, host, "toUpperCase", 0, |rt, _args| {
        let this = rt.current_this();
        crate::generated::string_prototype_to_upper_case(rt, this, &[])
    });
    register_intrinsic_method(rt, host, "toLowerCase", 0, |rt, _args| {
        let this = rt.current_this();
        crate::generated::string_prototype_to_lower_case(rt, this, &[])
    });
    register_intrinsic_method(rt, host, "toLocaleLowerCase", 0, |rt, _args| {
        let this = rt.current_this();
        crate::generated::string_prototype_to_locale_lower_case(rt, this, &[])
    });
    register_intrinsic_method(rt, host, "toLocaleUpperCase", 0, |rt, _args| {
        let this = rt.current_this();
        crate::generated::string_prototype_to_locale_upper_case(rt, this, &[])
    });
    // Ω.5.P63.E22: trim family routed through IR.
    register_intrinsic_method(rt, host, "trim", 0, |rt, _args| {
        let this = rt.current_this();
        crate::generated::string_prototype_trim(rt, this, &[])
    });
    register_intrinsic_method(rt, host, "trimStart", 0, |rt, _args| {
        let this = rt.current_this();
        crate::generated::string_prototype_trim_start(rt, this, &[])
    });
    register_intrinsic_method(rt, host, "trimEnd", 0, |rt, _args| {
        let this = rt.current_this();
        crate::generated::string_prototype_trim_end(rt, this, &[])
    });
    register_intrinsic_method(rt, host, "trimLeft", 0, |rt, _args| {
        let this = rt.current_this();
        crate::generated::string_prototype_trim_left(rt, this, &[])
    });
    register_intrinsic_method(rt, host, "trimRight", 0, |rt, _args| {
        let this = rt.current_this();
        crate::generated::string_prototype_trim_right(rt, this, &[])
    });
    // normalize(form?) — Unicode normalization. v1 deviation: pass-through
    // (return source unchanged). Most consumer code only invokes when input
    // is already ASCII; the cluster's load-bearing test is presence, not
    // correctness of NFC/NFD/NFKC/NFKD conversion.
    register_intrinsic_method(rt, host, "normalize", 0, |rt, _args| {
        let this = rt.current_this();
        crate::generated::string_prototype_normalize(rt, this, &[])
    });
    // Ω.5.P63.E20: String.prototype.{charAt, charCodeAt, concat} routed through IR.
    register_intrinsic_method(rt, host, "charAt", 1, |rt, args| {
        let this = rt.current_this();
        let pos = args.first().cloned().unwrap_or(Value::Undefined);
        crate::generated::string_prototype_char_at(rt, this, std::slice::from_ref(&pos))
    });
    register_intrinsic_method(rt, host, "charCodeAt", 1, |rt, args| {
        let this = rt.current_this();
        let pos = args.first().cloned().unwrap_or(Value::Undefined);
        crate::generated::string_prototype_char_code_at(rt, this, std::slice::from_ref(&pos))
    });
    register_intrinsic_method(rt, host, "concat", 1, |rt, args| {
        let this = rt.current_this();
        crate::generated::string_prototype_concat(rt, this, args)
    });
    // Tier-Ω.5.EEEEEEEE: String.prototype.localeCompare per ECMA-262 §22.1.3.10.
    // Used by sort comparators throughout the corpus (read-pkg/spdx-correct
    // family, conventional-changelog, meow). v1 deviation: locale-insensitive
    // lexicographic compare (real impl needs full Intl Collator chain).
    register_intrinsic_method(rt, host, "localeCompare", 1, |rt, args| {
        let this = rt.current_this();
        let that = args.first().cloned().unwrap_or(Value::Undefined);
        crate::generated::string_prototype_locale_compare(rt, this, &[that])
    });
    // Tier-Ω.5.GGGGGGG: String.prototype.codePointAt per ECMA-262 §22.1.3.4.
    // Returns the full code point (handles surrogate pairs) at the given
    // UTF-16 index; returns undefined if the index is out of range.
    // cli-truncate/execa/multiformats/strip-final-newline/tar all read
    // codePointAt at module-init for ANSI / encoding detection.
    register_intrinsic_method(rt, host, "codePointAt", 1, |rt, args| {
        let this = rt.current_this();
        let pos = args.first().cloned().unwrap_or(Value::Undefined);
        crate::generated::string_prototype_code_point_at(rt, this, &[pos])
    });
    // Ω.5.P63.E24: slice/substring/substr/indexOf/lastIndexOf/includes/startsWith/endsWith routed through IR.
    register_intrinsic_method(rt, host, "slice", 2, |rt, args| {
        let this = rt.current_this();
        let a = args.first().cloned().unwrap_or(Value::Undefined);
        let b = args.get(1).cloned().unwrap_or(Value::Undefined);
        crate::generated::string_prototype_slice(rt, this, &[a, b])
    });
    register_intrinsic_method(rt, host, "substr", 2, |rt, args| {
        let this = rt.current_this();
        let a = args.first().cloned().unwrap_or(Value::Undefined);
        let b = args.get(1).cloned().unwrap_or(Value::Undefined);
        crate::generated::string_prototype_substr(rt, this, &[a, b])
    });
    register_intrinsic_method(rt, host, "substring", 2, |rt, args| {
        let this = rt.current_this();
        let a = args.first().cloned().unwrap_or(Value::Undefined);
        let b = args.get(1).cloned().unwrap_or(Value::Undefined);
        crate::generated::string_prototype_substring(rt, this, &[a, b])
    });
    register_intrinsic_method(rt, host, "indexOf", 1, |rt, args| {
        let this = rt.current_this();
        let a = args.first().cloned().unwrap_or(Value::Undefined);
        let b = args.get(1).cloned().unwrap_or(Value::Undefined);
        crate::generated::string_prototype_index_of(rt, this, &[a, b])
    });
    register_intrinsic_method(rt, host, "lastIndexOf", 1, |rt, args| {
        let this = rt.current_this();
        let a = args.first().cloned().unwrap_or(Value::Undefined);
        let b = args.get(1).cloned().unwrap_or(Value::Undefined);
        crate::generated::string_prototype_last_index_of(rt, this, &[a, b])
    });
    register_intrinsic_method(rt, host, "includes", 1, |rt, args| {
        let this = rt.current_this();
        let a = args.first().cloned().unwrap_or(Value::Undefined);
        let b = args.get(1).cloned().unwrap_or(Value::Undefined);
        crate::generated::string_prototype_includes(rt, this, &[a, b])
    });
    register_intrinsic_method(rt, host, "startsWith", 1, |rt, args| {
        let this = rt.current_this();
        let a = args.first().cloned().unwrap_or(Value::Undefined);
        let b = args.get(1).cloned().unwrap_or(Value::Undefined);
        crate::generated::string_prototype_starts_with(rt, this, &[a, b])
    });
    register_intrinsic_method(rt, host, "endsWith", 1, |rt, args| {
        let this = rt.current_this();
        let a = args.first().cloned().unwrap_or(Value::Undefined);
        let b = args.get(1).cloned().unwrap_or(Value::Undefined);
        crate::generated::string_prototype_ends_with(rt, this, &[a, b])
    });
    register_intrinsic_method(rt, host, "split", 2, |rt, args| {
        let this = rt.current_this();
        let sep = args.first().cloned().unwrap_or(Value::Undefined);
        let limit = args.get(1).cloned().unwrap_or(Value::Undefined);
        crate::generated::string_prototype_split(rt, this, &[sep, limit])
    });
    // Ω.5.P63.E23: repeat routed through IR.
    register_intrinsic_method(rt, host, "repeat", 1, |rt, args| {
        let this = rt.current_this();
        let count = args.first().cloned().unwrap_or(Value::Undefined);
        crate::generated::string_prototype_repeat(rt, this, std::slice::from_ref(&count))
    });
    // Tier-Ω.5.iiiiii: String.prototype.matchAll per ECMA §22.1.3.13.
    // Returns an iterator over all matches of a regex with the /g flag.
    register_intrinsic_method(rt, host, "matchAll", 1, |rt, args| {
        let s = rt.to_string_strict(&rt.current_this())?;
        let regex_v = args.first().cloned().unwrap_or(Value::Undefined);
        let regex_id = match &regex_v {
            Value::Object(id) => *id,
            _ => {
                return Err(crate::interp::RuntimeError::TypeError(
                    "matchAll requires a regex".into(),
                ))
            }
        };
        // SMGR-EXT 1: ECMA-262 §22.1.3.13 step 4 — TypeError when first
        // argument is a RegExp without the global (`/g`) flag.
        if let crate::value::InternalKind::RegExp(re) = &rt.obj(regex_id).internal_kind {
            if !re.flags.contains('g') {
                return Err(crate::interp::RuntimeError::TypeError(
                    "String.prototype.matchAll called with a non-global RegExp argument".into(),
                ));
            }
        }
        let out = rt.alloc_object(Object::new_array());
        // Iterate via repeated regex.exec, advancing lastIndex.
        rt.object_set(regex_id, "lastIndex".into(), Value::Number(0.0));
        let exec = rt.object_get(regex_id, "exec");
        if !matches!(exec, Value::Object(_)) {
            return Err(RuntimeError::TypeError(
                "matchAll: regex.exec not callable".into(),
            ));
        }
        let mut idx = 0usize;
        loop {
            let r = rt.call_function(
                exec.clone(),
                regex_v.clone(),
                vec![Value::String(Rc::new(s.clone()))],
            )?;
            match r {
                Value::Null | Value::Undefined => break,
                Value::Object(match_id) => {
                    rt.object_set(out, idx.to_string(), Value::Object(match_id));
                    idx += 1;
                }
                _ => break,
            }
            if idx > 100000 {
                break;
            } // safety
        }
        rt.object_set(out, "length".into(), Value::Number(idx as f64));
        Ok(Value::Object(out))
    });
    // Tier-Ω.5.ppppp: padStart / padEnd per ECMA-262 §22.1.3.16 / §22.1.3.17.
    // date-fns / left-pad / many formatting libs reach for these.
    // Ω.5.P63.E23: padStart / padEnd routed through IR.
    register_intrinsic_method(rt, host, "padStart", 1, |rt, args| {
        let this = rt.current_this();
        let target = args.first().cloned().unwrap_or(Value::Undefined);
        let pad = args.get(1).cloned().unwrap_or(Value::Undefined);
        crate::generated::string_prototype_pad_start(rt, this, &[target, pad])
    });
    register_intrinsic_method(rt, host, "padEnd", 1, |rt, args| {
        let this = rt.current_this();
        let target = args.first().cloned().unwrap_or(Value::Undefined);
        let pad = args.get(1).cloned().unwrap_or(Value::Undefined);
        crate::generated::string_prototype_pad_end(rt, this, &[target, pad])
    });
    register_intrinsic_method(rt, host, "replace", 2, |rt, args| {
        let this = rt.current_this();
        let search = args.first().cloned().unwrap_or(Value::Undefined);
        let repl = args.get(1).cloned().unwrap_or(Value::Undefined);
        crate::generated::string_prototype_replace(rt, this, &[search, repl])
    });
    register_intrinsic_method(rt, host, "replaceAll", 2, |rt, args| {
        let this = rt.current_this();
        let search = args.first().cloned().unwrap_or(Value::Undefined);
        let repl = args.get(1).cloned().unwrap_or(Value::Undefined);
        crate::generated::string_prototype_replace_all(rt, this, &[search, repl])
    });
    register_intrinsic_method(rt, host, "at", 1, |rt, args| {
        let this = rt.current_this();
        let idx = args.first().cloned().unwrap_or(Value::Undefined);
        crate::generated::string_prototype_at(rt, this, &[idx])
    });
    register_intrinsic_method(rt, host, "toString", 0, |rt, _args| {
        // Ω.5.P62.E1: unwrap String-wrapper [[StringData]] before coerce.
        let this = rt.current_this();
        let t = rt.unwrap_primitive(&this);
        match t {
            Value::String(s) => Ok(Value::String(s)),
            _ => Err(RuntimeError::TypeError(
                "String.prototype.toString: this is not a String".into(),
            )),
        }
    });
    register_intrinsic_method(rt, host, "valueOf", 0, |rt, _args| {
        let this = rt.current_this();
        let t = rt.unwrap_primitive(&this);
        match t {
            Value::String(s) => Ok(Value::String(s)),
            _ => Err(RuntimeError::TypeError(
                "String.prototype.valueOf: this is not a String".into(),
            )),
        }
    });
    register_intrinsic_method(rt, host, "@@iterator", 0, |rt, _args| {
        let this = rt.current_this();
        let s = abstract_ops::to_string(&rt.unwrap_primitive(&this))
            .as_str()
            .to_string();
        Ok(Value::Object(crate::iterator::make_string_iterator(rt, s)))
    });
}

// ──────────────── %Function.prototype% ────────────────

fn install_function_proto(rt: &mut Runtime, host: ObjectRef) {
    // Tier-Ω.5.yyy: Function.prototype.toString returns a generic
    // "function NAME() { [native code] }" string. Per ECMA-262
    // §20.2.3.5 real toString returns source for user functions and
    // "[native code]" for natives; v1 returns the native-shape for
    // all functions. object-hash detects native functions by regex-
    // matching this output. Sufficient for the duck-test.
    register_intrinsic_method(rt, host, "toString", 0, |rt, args| {
        crate::generated::function_prototype_to_string(rt, rt.current_this(), args)
    });
    register_intrinsic_method(rt, host, "call", 1, |rt, args| {
        let f = rt.current_this();
        let this_arg = args.first().cloned().unwrap_or(Value::Undefined);
        let rest: Vec<Value> = args.iter().skip(1).cloned().collect();
        rt.call_function(f, this_arg, rest)
    });
    register_intrinsic_method(rt, host, "apply", 2, |rt, args| {
        let f = rt.current_this();
        let this_arg = args.first().cloned().unwrap_or(Value::Undefined);
        let arr_v = args.get(1).cloned().unwrap_or(Value::Undefined);
        let call_args: Vec<Value> = match arr_v {
            Value::Object(aid) => {
                let len = rt.array_length(aid);
                (0..len)
                    .map(|i| rt.object_get(aid, &i.to_string()))
                    .collect()
            }
            Value::Null | Value::Undefined => Vec::new(),
            _ => {
                return Err(RuntimeError::TypeError(
                    "apply: argsArray must be an Array".into(),
                ))
            }
        };
        rt.call_function(f, this_arg, call_args)
    });
    register_intrinsic_method(rt, host, "bind", 1, |rt, args| {
        let target = match rt.current_this() {
            Value::Object(id) => id,
            _ => return Err(RuntimeError::TypeError("bind: this is not callable".into())),
        };
        let bound_this = args.first().cloned().unwrap_or(Value::Undefined);
        let bound_args: Vec<Value> = args.iter().skip(1).cloned().collect();
        // Ω.5.P50.E2 (C1 constraint): per ECMA-262 §20.2.5.5 BoundFunctionCreate
        // + §10.4.1, bound functions have spec-mandated name "bound "+target.name
        // and length max(0, target.length - boundArgs.length). Without these,
        // call-bind output (array.prototype.* polyfills, data-view-* shims,
        // hasown, queue-microtask, emoji-regex, etc.) emit module-namespaces
        // missing name/length where Bun has them — the kc-pm-1-2 cluster's
        // 18+5-pkg signature.
        let target_name = match rt.object_get(target, "name") {
            Value::String(s) => (*s).clone(),
            _ => String::new(),
        };
        let target_length = match rt.object_get(target, "length") {
            Value::Number(n) if n.is_finite() => n,
            _ => 0.0,
        };
        let n_bound = bound_args.len() as f64;
        let bound_length = (target_length - n_bound).max(0.0);
        let bound_name = format!("bound {}", target_name);
        let mut properties = indexmap::IndexMap::new();
        crate::value::install_function_meta_props(&mut properties, &bound_name, bound_length);
        let bf = Object {
            proto: None,
            extensible: true,
            properties,
            internal_kind: InternalKind::BoundFunction(BoundFunctionInternals {
                target,
                this: bound_this,
                args: bound_args,
            }),

            ..Default::default()
        };
        let id = rt.alloc_object(bf);
        Ok(Value::Object(id))
    });
}

// ──────────────── %Promise.prototype% ────────────────
//
// `then` / `catch` delegate to the static-form logic in promise.rs. The
// receiver is the source promise. Since the static Promise.then native
// already expects (source, onF, onR) as positional args, we synthesize
// that argument list here.

fn install_promise_proto(rt: &mut Runtime, host: ObjectRef) {
    // Ω.5.P63.E52: Promise.prototype.{then, catch} routed through IR.
    // Helper convention: args[0]=source (the promise instance, taken from
    // current_this), args[1..]=user-supplied handlers. Prepend then forward.
    register_intrinsic_method(rt, host, "then", 1, |rt, args| {
        let mut a: Vec<Value> = Vec::with_capacity(args.len() + 1);
        a.push(rt.current_this());
        a.extend(args.iter().cloned());
        crate::generated::promise_prototype_then(rt, rt.current_this(), &a)
    });
    register_intrinsic_method(rt, host, "catch", 1, |rt, args| {
        let mut a: Vec<Value> = Vec::with_capacity(args.len() + 1);
        a.push(rt.current_this());
        a.extend(args.iter().cloned());
        crate::generated::promise_prototype_catch(rt, rt.current_this(), &a)
    });
}

fn promise_then_impl(
    rt: &mut Runtime,
    source: ObjectRef,
    on_fulfilled: Option<Value>,
    on_rejected: Option<Value>,
) -> Result<Value, RuntimeError> {
    let chain = crate::promise::new_promise(rt);
    let (status, value) = {
        let s = rt.obj(source);
        match &s.internal_kind {
            InternalKind::Promise(ps) => (ps.status, ps.value.clone()),
            _ => {
                return Err(RuntimeError::TypeError(
                    "then: source is not a Promise".into(),
                ))
            }
        }
    };
    match status {
        PromiseStatus::Pending => {
            let src = rt.obj_mut(source);
            if let InternalKind::Promise(ps) = &mut src.internal_kind {
                ps.fulfill_reactions.push(PromiseReaction {
                    handler: on_fulfilled,
                    chain,
                });
                ps.reject_reactions.push(PromiseReaction {
                    handler: on_rejected,
                    chain,
                });
            }
        }
        PromiseStatus::Fulfilled => {
            enqueue_handler(rt, on_fulfilled, value, chain, false);
        }
        PromiseStatus::Rejected => {
            rt.pending_unhandled.remove(&source);
            enqueue_handler(rt, on_rejected, value, chain, true);
        }
    }
    Ok(Value::Object(chain))
}

fn enqueue_handler(
    rt: &mut Runtime,
    handler: Option<Value>,
    value: Value,
    chain: ObjectRef,
    is_rejected: bool,
) {
    rt.enqueue_microtask("PromiseReactionJob", move |rt| {
        match handler {
            Some(h) => match rt.call_function(h, Value::Undefined, vec![value]) {
                Ok(r) => {
                    crate::promise::resolve_promise(rt, chain, r);
                }
                Err(e) => {
                    let thrown = match e {
                        RuntimeError::Thrown(v) => v,
                        other => Value::String(Rc::new(format!("{:?}", other))),
                    };
                    crate::promise::reject_promise(rt, chain, thrown);
                }
            },
            None => {
                if is_rejected {
                    crate::promise::reject_promise(rt, chain, value);
                } else {
                    crate::promise::resolve_promise(rt, chain, value);
                }
            }
        }
        Ok(())
    });
}

// ──────────────── %Number.prototype% ────────────────

fn install_number_proto(rt: &mut Runtime, host: ObjectRef) {
    // Ω.5.P62.E1: Number.prototype.valueOf returns [[NumberData]] for
    // Number-exotic wrapper objects (modeled via __primitive__ slot).
    // Ω.5.P63.E19: Number.prototype.valueOf routed through IR.
    register_intrinsic_method(rt, host, "valueOf", 0, |rt, _args| {
        let this = rt.current_this();
        crate::generated::number_prototype_value_of(rt, this, &[])
    });
    register_intrinsic_method(rt, host, "toString", 0, |rt, args| {
        crate::generated::number_prototype_to_string(rt, rt.current_this(), args)
    });
    // Ω.5.P63.E18: Number.prototype.toFixed routed through IR.
    register_intrinsic_method(rt, host, "toFixed", 1, |rt, args| {
        let this = rt.current_this();
        let digits = args.first().cloned().unwrap_or(Value::Undefined);
        crate::generated::number_prototype_to_fixed(rt, this, std::slice::from_ref(&digits))
    });
    // Ω.5.P61.E10: toExponential, toPrecision, toLocaleString per
    // ECMA §21.1.3.
    // Ω.5.P63.E19: Number.prototype.toExponential routed through IR.
    register_intrinsic_method(rt, host, "toExponential", 1, |rt, args| {
        let this = rt.current_this();
        let digits = args.first().cloned().unwrap_or(Value::Undefined);
        crate::generated::number_prototype_to_exponential(rt, this, std::slice::from_ref(&digits))
    });
    // Ω.5.P63.E19: Number.prototype.toPrecision routed through IR.
    register_intrinsic_method(rt, host, "toPrecision", 1, |rt, args| {
        let this = rt.current_this();
        let precision = args.first().cloned().unwrap_or(Value::Undefined);
        crate::generated::number_prototype_to_precision(rt, this, std::slice::from_ref(&precision))
    });
    register_intrinsic_method(rt, host, "toLocaleString", 0, |rt, args| {
        crate::generated::number_prototype_to_locale_string(rt, rt.current_this(), args)
    });
    // Ω.5.P62.E19: removed duplicate valueOf install (line 1785's
    // brand-checked + __primitive__-unwrapping version is the canonical).
}

// ──────────────── helpers ────────────────

fn arg_string(args: &[Value], i: usize) -> String {
    args.get(i)
        .map(|v| abstract_ops::to_string(v).as_str().to_string())
        .unwrap_or_default()
}

fn register_method<F>(rt: &mut Runtime, host: ObjectRef, name: &str, f: F)
where
    F: Fn(&mut Runtime, &[Value]) -> Result<Value, RuntimeError> + 'static,
{
    let native: NativeFn = Rc::new(f);
    let mut properties = indexmap::IndexMap::new();
    crate::value::install_function_meta_props(&mut properties, name, 0.0);
    let fn_obj = Object {
        proto: None, // function_prototype not yet installed when called from install_prototypes
        extensible: true,
        properties,
        internal_kind: InternalKind::Function(FunctionInternals {
            name: name.to_string(),
            length: 0,
            native,
            is_constructor: true,
        }),

        ..Default::default()
    };
    let fn_id = rt.alloc_object(fn_obj);
    rt.object_set(host, name.into(), Value::Object(fn_id));
}

/// Ω.5.P61.E7: ToObject coercion for Array.prototype `this`. Per ECMA
/// §23.1.3 the Array.prototype methods are generic — they accept any
/// object with `.length` and indexed properties, plus boxed primitive
/// `this` (Boolean / Number / String wrappers). Pre-P61.E7 cruftless's
/// methods all rejected non-Array.Object this with TypeError, which
/// failed ~500 test262 tests that call `Array.prototype.X.call(true)`
/// / `.call(123)` / `.call("abc")` patterns. Returns the coerced
/// ObjectRef; throws TypeError on null/undefined per ToObject step 1.
pub(crate) fn to_array_this(rt: &mut Runtime) -> Result<ObjectRef, RuntimeError> {
    match rt.current_this() {
        Value::Object(id) => Ok(id),
        Value::Undefined | Value::Null => Err(RuntimeError::TypeError(
            "Array.prototype method called on null or undefined".into(),
        )),
        Value::Boolean(b) => {
            // Ω.5.P61.E13: Box as Boolean wrapper. Per ECMA §7.1.18
            // ToObject, the box has [[BooleanData]] internal slot and
            // [[Prototype]] = %Boolean.prototype%. The boxed object does
            // NOT shadow Boolean.prototype.length — tests like
            //   Boolean.prototype[1] = obj; Boolean.prototype.length = 2;
            //   Array.prototype.indexOf.call(true, obj)
            // rely on the boxed wrapper inheriting length from the
            // prototype. Pre-P61.E13 cruftless set length=0 on the box,
            // shadowing the prototype's length.
            let mut o = Object::new_ordinary();
            o.set_own_internal("__primitive".into(), Value::Boolean(b));
            // GBSU-EXT 4b: canonical lookup via unified globalThis.
            if let Value::Object(bid) = rt.global_get("Boolean") {
                if let Value::Object(p) = rt.object_get(bid, "prototype") {
                    o.proto = Some(p);
                }
            }
            Ok(rt.alloc_object(o))
        }
        Value::Number(n) => {
            let mut o = Object::new_ordinary();
            o.set_own_internal("__primitive".into(), Value::Number(n));
            if let Some(p) = rt.number_prototype {
                o.proto = Some(p);
            }
            Ok(rt.alloc_object(o))
        }
        Value::String(s) => {
            // String wrappers per §6.1.4 expose length + indexed chars.
            let mut o = Object::new_ordinary();
            let n = s.chars().count();
            o.set_own("length".into(), Value::Number(n as f64));
            for (i, c) in s.chars().enumerate() {
                o.set_own(i.to_string(), Value::String(Rc::new(c.to_string())));
            }
            if let Some(p) = rt.string_prototype {
                o.proto = Some(p);
            }
            Ok(rt.alloc_object(o))
        }
        Value::BigInt(_) | Value::Symbol(_) => Err(RuntimeError::TypeError(
            "Array.prototype method called on BigInt/Symbol".into(),
        )),
    }
}

/// Ω.5.P61.E5: prototype-local intrinsic-method installer. Same semantics
/// as the one in intrinsics.rs but kept here to avoid module-cycle issues
/// with the prototype.rs install paths that pre-date intrinsics.rs's
/// availability. Sets length, marks non-constructor, installs non-enum.
fn register_intrinsic_method<F>(rt: &mut Runtime, host: ObjectRef, name: &str, length: u32, f: F)
where
    F: Fn(&mut Runtime, &[Value]) -> Result<Value, RuntimeError> + 'static,
{
    let native: NativeFn = Rc::new(f);
    let mut properties = indexmap::IndexMap::new();
    crate::value::install_function_meta_props(&mut properties, name, length as f64);
    let fn_obj = Object {
        proto: None,
        extensible: true,
        properties,
        internal_kind: InternalKind::Function(FunctionInternals {
            name: name.to_string(),
            length,
            native,
            is_constructor: false,
        }),

        ..Default::default()
    };
    let fn_id = rt.alloc_object(fn_obj);
    rt.obj_mut(host).dict_mut().insert(
        crate::value::PropertyKey::String(name.to_string()),
        crate::value::PropertyDescriptor {
            value: Value::Object(fn_id),
            writable: true,
            enumerable: false,
            configurable: true,
            getter: None,
            setter: None,
        },
    );
}
