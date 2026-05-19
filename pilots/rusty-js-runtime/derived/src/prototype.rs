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
    FunctionInternals, InternalKind, NativeFn, Object, ObjectRef,
    PromiseReaction, PromiseStatus, Value,
    BoundFunctionInternals,
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
        let array_proto    = self.alloc_object(Object::new_ordinary());
        let function_proto = self.alloc_object(Object::new_ordinary());
        let promise_proto  = self.alloc_object(Object::new_ordinary());
        let string_proto   = self.alloc_object(Object::new_ordinary());
        let number_proto   = self.alloc_object(Object::new_ordinary());
        self.array_prototype    = Some(array_proto);
        self.function_prototype = Some(function_proto);
        self.promise_prototype  = Some(promise_proto);
        self.string_prototype   = Some(string_proto);
        self.number_prototype   = Some(number_proto);

        install_object_proto(self, object_proto);
        install_array_proto(self, array_proto);
        install_string_proto(self, string_proto);
        install_function_proto(self, function_proto);
        install_promise_proto(self, promise_proto);
        install_number_proto(self, number_proto);
    }
}

// ──────────────── %Object.prototype% ────────────────

fn install_object_proto(rt: &mut Runtime, host: ObjectRef) {
    register_intrinsic_method(rt, host, "toString", 0, |rt, _args| {
        // Tier-Ω.5.lllll: Object.prototype.toString per ECMA-262 §20.1.3.6.
        // Internal-slot tags drive the output; spec-named tags are
        // PascalCase. Prior impl returned "[object string]" / "[object
        // number]" for primitives (lowercase via type_of) and "[object
        // Object]" for RegExp instances, which broke isString/isRegExp
        // duck-tests in linkify-it / yup / many libs.
        let this = rt.current_this();
        let s = match this {
            Value::Undefined => "[object Undefined]".to_string(),
            Value::Null => "[object Null]".to_string(),
            Value::Boolean(_) => "[object Boolean]".to_string(),
            Value::Number(_) => "[object Number]".to_string(),
            Value::String(_) => "[object String]".to_string(),
            Value::BigInt(_) => "[object BigInt]".to_string(),
            Value::Symbol(_) => "[object Symbol]".to_string(),
            Value::Object(id) => {
                // Ω.5.P62.E4: ECMA §20.1.3.6 step 15 — read @@toStringTag
                // up the proto chain; a String value overrides the
                // internal-kind tag. Math/JSON set this to "Math"/"JSON";
                // user objects can set their own custom tag.
                let tag_val = rt.object_get(id, "@@toStringTag");
                let tag = if let Value::String(s) = &tag_val {
                    s.as_str().to_string()
                } else {
                    match &rt.obj(id).internal_kind {
                        InternalKind::Array => "Array",
                        InternalKind::Function(_)
                        | InternalKind::Closure(_)
                        | InternalKind::BoundFunction(_) => "Function",
                        InternalKind::Promise(_) => "Promise",
                        InternalKind::Error => "Error",
                        InternalKind::RegExp(_) => "RegExp",
                        _ => "Object",
                    }.to_string()
                };
                format!("[object {}]", tag)
            }
        };
        Ok(Value::String(Rc::new(s)))
    });
    register_intrinsic_method(rt, host, "hasOwnProperty", 1, |rt, args| {
        let key = arg_string(args, 0);
        let owns = match rt.current_this() {
            Value::Object(id) => rt.obj(id).properties.contains_key(&key),
            _ => false,
        };
        Ok(Value::Boolean(owns))
    });
    register_intrinsic_method(rt, host, "valueOf", 0, |rt, _args| Ok(rt.current_this()));
    // Tier-Ω.5.DDDDDDDD: Object.prototype.__defineGetter__/__defineSetter__
    // per ECMA Annex B.2.2.2/2.2.3 (legacy but ubiquitous — pg, slonik,
    // sockjs, mongoose use them at module-init for shape augmentation).
    register_intrinsic_method(rt, host, "__defineGetter__", 1, |rt, args| {
        let this = match rt.current_this() { Value::Object(id) => id, _ => return Ok(Value::Undefined) };
        let key = abstract_ops::to_string(&args.first().cloned().unwrap_or(Value::Undefined)).as_str().to_string();
        let getter = args.get(1).cloned().unwrap_or(Value::Undefined);
        if !matches!(getter, Value::Object(_)) {
            return Err(RuntimeError::TypeError("__defineGetter__: getter must be callable".into()));
        }
        rt.obj_mut(this).properties.insert(key, crate::value::PropertyDescriptor {
            value: Value::Undefined,
            writable: false, enumerable: true, configurable: true,
            getter: Some(getter), setter: None,
        });
        Ok(Value::Undefined)
    });
    register_intrinsic_method(rt, host, "__defineSetter__", 1, |rt, args| {
        let this = match rt.current_this() { Value::Object(id) => id, _ => return Ok(Value::Undefined) };
        let key = abstract_ops::to_string(&args.first().cloned().unwrap_or(Value::Undefined)).as_str().to_string();
        let setter = args.get(1).cloned().unwrap_or(Value::Undefined);
        if !matches!(setter, Value::Object(_)) {
            return Err(RuntimeError::TypeError("__defineSetter__: setter must be callable".into()));
        }
        let existing_getter = rt.obj(this).properties.get(&key).and_then(|d| d.getter.clone());
        rt.obj_mut(this).properties.insert(key, crate::value::PropertyDescriptor {
            value: Value::Undefined,
            writable: false, enumerable: true, configurable: true,
            getter: existing_getter, setter: Some(setter),
        });
        Ok(Value::Undefined)
    });
    register_intrinsic_method(rt, host, "__lookupGetter__", 1, |rt, args| {
        let this = match rt.current_this() { Value::Object(id) => id, _ => return Ok(Value::Undefined) };
        let key = abstract_ops::to_string(&args.first().cloned().unwrap_or(Value::Undefined)).as_str().to_string();
        Ok(rt.obj(this).properties.get(&key).and_then(|d| d.getter.clone()).unwrap_or(Value::Undefined))
    });
    register_intrinsic_method(rt, host, "__lookupSetter__", 1, |rt, args| {
        let this = match rt.current_this() { Value::Object(id) => id, _ => return Ok(Value::Undefined) };
        let key = abstract_ops::to_string(&args.first().cloned().unwrap_or(Value::Undefined)).as_str().to_string();
        Ok(rt.obj(this).properties.get(&key).and_then(|d| d.setter.clone()).unwrap_or(Value::Undefined))
    });
    // Tier-Ω.5.jjjj: Object.prototype.propertyIsEnumerable per ECMA-262
    // §20.1.3.4. Returns true if the receiver has an own enumerable
    // property at the given key. v1 returns true for any own property
    // (we don't track enumerable bit precisely).
    register_intrinsic_method(rt, host, "propertyIsEnumerable", 1, |rt, args| {
        let key = abstract_ops::to_string(&args.first().cloned().unwrap_or(Value::Undefined))
            .as_str().to_string();
        let owns = match rt.current_this() {
            Value::Object(id) => rt.obj(id).properties.contains_key(&key),
            _ => false,
        };
        Ok(Value::Boolean(owns))
    });
    register_intrinsic_method(rt, host, "isPrototypeOf", 1, |rt, args| {
        let target = match args.first() {
            Some(Value::Object(id)) => *id,
            _ => return Ok(Value::Boolean(false)),
        };
        let this_id = match rt.current_this() {
            Value::Object(id) => id,
            _ => return Ok(Value::Boolean(false)),
        };
        let mut cur = rt.obj(target).proto;
        while let Some(c) = cur {
            if c == this_id { return Ok(Value::Boolean(true)); }
            cur = rt.obj(c).proto;
        }
        Ok(Value::Boolean(false))
    });
    // Ω.5.P61.E10: Object.prototype.toLocaleString per ECMA §20.1.3.5.
    // Default is to invoke this.toString() — locale-aware variants live
    // on subclass prototypes (Number/Date/Array each override).
    register_intrinsic_method(rt, host, "toLocaleString", 0, |rt, _args| {
        let this = rt.current_this();
        if let Value::Object(id) = &this {
            let to_str = rt.object_get(*id, "toString");
            if matches!(to_str, Value::Object(_)) {
                return rt.call_function(to_str, this.clone(), Vec::new());
            }
        }
        Ok(Value::String(Rc::new(crate::abstract_ops::to_string(&this).as_str().to_string())))
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
    register_intrinsic_method(rt, host, "toString", 0, |rt, _args| {
        let this = rt.current_this();
        if let Value::Object(id) = this {
            let join = rt.object_get(id, "join");
            if matches!(join, Value::Object(_)) {
                return rt.call_function(join, Value::Object(id), Vec::new());
            }
        }
        Ok(Value::String(Rc::new("[object Array]".into())))
    });
    register_intrinsic_method(rt, host, "push", 1, |rt, args| {
        let id = to_array_this(rt)?;
        let mut len = rt.array_length(id);
        for a in args {
            rt.object_set(id, len.to_string(), a.clone());
            len += 1;
        }
        // Keep a synthetic length to outpace property-derivation in edge cases.
        rt.object_set(id, "length".into(), Value::Number(len as f64));
        Ok(Value::Number(len as f64))
    });
    register_intrinsic_method(rt, host, "pop", 0, |rt, _args| {
        let id = to_array_this(rt)?;
        let len = rt.array_length(id);
        if len == 0 { return Ok(Value::Undefined); }
        let last_key = (len - 1).to_string();
        let v = rt.object_get(id, &last_key);
        rt.obj_mut(id).properties.shift_remove(&last_key);
        rt.object_set(id, "length".into(), Value::Number((len - 1) as f64));
        Ok(v)
    });
    register_intrinsic_method(rt, host, "shift", 0, |rt, _args| {
        let id = to_array_this(rt)?;
        let len = rt.array_length(id);
        if len == 0 { return Ok(Value::Undefined); }
        let first = rt.object_get(id, "0");
        for i in 1..len {
            let v = rt.object_get(id, &i.to_string());
            rt.object_set(id, (i - 1).to_string(), v);
        }
        rt.obj_mut(id).properties.shift_remove(&(len - 1).to_string());
        rt.object_set(id, "length".into(), Value::Number((len - 1) as f64));
        Ok(first)
    });
    register_intrinsic_method(rt, host, "unshift", 1, |rt, args| {
        let id = to_array_this(rt)?;
        let n = args.len();
        let len = rt.array_length(id);
        // Shift existing right by n.
        for i in (0..len).rev() {
            let v = rt.object_get(id, &i.to_string());
            rt.object_set(id, (i + n).to_string(), v);
        }
        for (i, a) in args.iter().enumerate() {
            rt.object_set(id, i.to_string(), a.clone());
        }
        let new_len = len + n;
        rt.object_set(id, "length".into(), Value::Number(new_len as f64));
        Ok(Value::Number(new_len as f64))
    });
    register_intrinsic_method(rt, host, "indexOf", 1, |rt, args| {
        // Ω.5.P61.E14: sparse-skip per ECMA §23.1.3.16; honor fromIndex
        // (second arg, default 0; negative = len + arg).
        let id = to_array_this(rt)?;
        let needle = args.first().cloned().unwrap_or(Value::Undefined);
        let len = rt.array_length(id) as i64;
        let from = match args.get(1) {
            Some(v) if !matches!(v, Value::Undefined) => {
                let n = abstract_ops::to_number(v) as i64;
                if n < 0 { (len + n).max(0) } else { n.min(len) }
            }
            _ => 0,
        };
        let mut i = from;
        while i < len {
            let key = i.to_string();
            if rt.has_property(id, &key) {
                let v = rt.read_property(id, &key)?;
                if abstract_ops::is_strictly_equal(&v, &needle) {
                    return Ok(Value::Number(i as f64));
                }
            }
            i += 1;
        }
        Ok(Value::Number(-1.0))
    });
    register_intrinsic_method(rt, host, "includes", 1, |rt, args| {
        // Ω.5.P61.E14: includes does NOT skip sparse per ECMA §23.1.3.14
        // (treats holes as undefined and applies SameValueZero).
        let id = to_array_this(rt)?;
        let needle = args.first().cloned().unwrap_or(Value::Undefined);
        let len = rt.array_length(id);
        for i in 0..len {
            let key = i.to_string();
            let v = if rt.has_property(id, &key) {
                rt.read_property(id, &key)?
            } else {
                Value::Undefined
            };
            // SameValueZero: like strict equal except NaN === NaN.
            let eq = match (&v, &needle) {
                (Value::Number(a), Value::Number(b)) if a.is_nan() && b.is_nan() => true,
                _ => abstract_ops::is_strictly_equal(&v, &needle),
            };
            if eq { return Ok(Value::Boolean(true)); }
        }
        Ok(Value::Boolean(false))
    });
    // Tier-Ω.5.cccccc: Array.prototype.reverse per ECMA-262 §23.1.3.21.
    // micromark slices events then reverses; without this, .reverse() was
    // undefined and every state-machine token finalization failed.
    register_intrinsic_method(rt, host, "reverse", 0, |rt, _args| {
        let id = to_array_this(rt)?;
        let len = rt.array_length(id) as i64;
        let mid = len / 2;
        for i in 0..mid {
            let j = len - 1 - i;
            let a = rt.object_get(id, &i.to_string());
            let b = rt.object_get(id, &j.to_string());
            rt.object_set(id, i.to_string(), b);
            rt.object_set(id, j.to_string(), a);
        }
        Ok(Value::Object(id))
    });
    register_intrinsic_method(rt, host, "slice", 2, |rt, args| {
        let id = to_array_this(rt)?;
        let len = rt.array_length(id) as i64;
        let start_arg = match args.first().cloned() {
            Some(Value::Undefined) | None => 0,
            Some(v) => rt.coerce_to_number(&v)? as i64,
        };
        let end_arg = match args.get(1).cloned() {
            Some(Value::Undefined) | None => len,
            Some(v) => rt.coerce_to_number(&v)? as i64,
        };
        let start = clamp_index(start_arg, len);
        let end = clamp_index(end_arg, len);
        let out = rt.alloc_object(Object::new_array());
        let mut j: i64 = 0;
        let mut i = start;
        while i < end {
            let v = rt.object_get(id, &i.to_string());
            rt.object_set(out, j.to_string(), v);
            j += 1;
            i += 1;
        }
        rt.object_set(out, "length".into(), Value::Number(j as f64));
        Ok(Value::Object(out))
    });
    // Tier-Ω.5.xxx: Array.prototype.splice per ECMA-262 §23.1.3.31.
    // Removes deleteCount elements starting at start, optionally
    // inserting items in their place. Returns the removed elements.
    // object-hash uses splice on its internal stream buffer.
    register_intrinsic_method(rt, host, "splice", 2, |rt, args| {
        let id = to_array_this(rt)?;
        let len = rt.array_length(id) as i64;
        let start_arg = match args.first().cloned() {
            Some(Value::Undefined) | None => 0,
            Some(v) => rt.coerce_to_number(&v)? as i64,
        };
        let start = clamp_index(start_arg, len);
        let delete_count = match args.get(1).cloned() {
            Some(Value::Undefined) | None => len - start,
            Some(v) => (rt.coerce_to_number(&v)? as i64).max(0).min(len - start),
        };
        let items: Vec<Value> = args.iter().skip(2).cloned().collect();
        // Collect removed slice into a new array.
        let removed = rt.alloc_object(Object::new_array());
        for j in 0..delete_count {
            let v = rt.object_get(id, &(start + j).to_string());
            rt.object_set(removed, j.to_string(), v);
        }
        rt.object_set(removed, "length".into(), Value::Number(delete_count as f64));
        // Shift tail by (items.len() - delete_count).
        let new_len = len - delete_count + items.len() as i64;
        let shift = items.len() as i64 - delete_count;
        if shift > 0 {
            // Move tail right (iterate from end).
            let mut i = len - 1;
            while i >= start + delete_count {
                let v = rt.object_get(id, &i.to_string());
                rt.object_set(id, (i + shift).to_string(), v);
                i -= 1;
            }
        } else if shift < 0 {
            // Move tail left.
            let mut i = start + delete_count;
            while i < len {
                let v = rt.object_get(id, &i.to_string());
                rt.object_set(id, (i + shift).to_string(), v);
                i += 1;
            }
            // Remove trailing slots.
            for i in new_len..len {
                rt.obj_mut(id).properties.shift_remove(&i.to_string());
            }
        }
        // Insert items.
        for (k, v) in items.into_iter().enumerate() {
            rt.object_set(id, (start + k as i64).to_string(), v);
        }
        rt.object_set(id, "length".into(), Value::Number(new_len as f64));
        Ok(Value::Object(removed))
    });
    register_intrinsic_method(rt, host, "concat", 1, |rt, args| {
        // Ω.5.P62.E16: concat per ECMA §23.1.3.2 — intentionally generic.
        // IsConcatSpreadable per §23.1.3.2.1: check @@isConcatSpreadable;
        // if undefined, fall back to IsArray(E). Otherwise it's a single
        // element (including plain Object receivers).
        let this = rt.current_this();
        rt.require_object_coercible(&this)?;
        let out = rt.alloc_object(Object::new_array());
        let mut j = 0usize;
        // Collect this + args.
        let mut items: Vec<Value> = Vec::with_capacity(args.len() + 1);
        items.push(this);
        items.extend(args.iter().cloned());
        for e in items {
            let spreadable = match &e {
                Value::Object(eid) => {
                    let flag = rt.read_property(*eid, "@@isConcatSpreadable")?;
                    match flag {
                        Value::Undefined => matches!(
                            rt.obj(*eid).internal_kind, InternalKind::Array),
                        v => crate::abstract_ops::to_boolean(&v),
                    }
                }
                _ => false,
            };
            if spreadable {
                if let Value::Object(eid) = e {
                    let el = rt.array_length(eid);
                    for i in 0..el {
                        let key = i.to_string();
                        if rt.has_property(eid, &key) {
                            let v = rt.read_property(eid, &key)?;
                            rt.object_set(out, j.to_string(), v);
                        }
                        j += 1;
                    }
                }
            } else {
                rt.object_set(out, j.to_string(), e);
                j += 1;
            }
        }
        rt.object_set(out, "length".into(), Value::Number(j as f64));
        Ok(Value::Object(out))
    });
    register_intrinsic_method(rt, host, "join", 1, |rt, args| {
        let id = to_array_this(rt)?;
        let sep = match args.first() {
            Some(Value::Undefined) | None => ",".to_string(),
            Some(v) => abstract_ops::to_string(v).as_str().to_string(),
        };
        let len = rt.array_length(id);
        let mut parts = Vec::with_capacity(len);
        for i in 0..len {
            let v = rt.object_get(id, &i.to_string());
            let s = match v {
                Value::Undefined | Value::Null => String::new(),
                other => abstract_ops::to_string(&other).as_str().to_string(),
            };
            parts.push(s);
        }
        Ok(Value::String(Rc::new(parts.join(&sep))))
    });
    register_intrinsic_method(rt, host, "at", 1, |rt, args| {
        let id = to_array_this(rt)?;
        let len = rt.array_length(id) as i64;
        let i = args.first().map(abstract_ops::to_number).unwrap_or(0.0) as i64;
        let idx = if i < 0 { len + i } else { i };
        if idx < 0 || idx >= len { return Ok(Value::Undefined); }
        Ok(rt.object_get(id, &idx.to_string()))
    });
    // Tier-Ω.5.DDDDDDD: Array.prototype.fill per ECMA §23.1.3.7. Receiver
    // is this; fills positions [start, end) with the value. lru-cache's
    // ZeroArray ctor does `super(size); this.fill(0)` to zero-initialize.
    register_intrinsic_method(rt, host, "fill", 1, |rt, args| {
        // Ω.5.P62.E17: coerce_to_number on start/end so Symbol→TypeError
        // and abrupt valueOf/toString propagates per §23.1.3.7 steps 5/9.
        let id = to_array_this(rt)?;
        let value = args.first().cloned().unwrap_or(Value::Undefined);
        let len = rt.array_length(id);
        let start = match args.get(1).cloned() {
            Some(Value::Undefined) | None => 0,
            Some(v) => {
                let n = rt.coerce_to_number(&v)? as i64;
                if n < 0 { (len as i64 + n).max(0) as usize } else { (n as usize).min(len) }
            }
        };
        let end = match args.get(2).cloned() {
            Some(Value::Undefined) | None => len,
            Some(v) => {
                let n = rt.coerce_to_number(&v)? as i64;
                if n < 0 { (len as i64 + n).max(0) as usize } else { (n as usize).min(len) }
            }
        };
        for i in start..end {
            rt.object_set(id, i.to_string(), value.clone());
        }
        Ok(Value::Object(id))
    });
    // Tier-Ω.5.iiiiii: Array.prototype.flat per ECMA §23.1.3.10.
    register_intrinsic_method(rt, host, "flat", 0, |rt, args| {
        let id = to_array_this(rt)?;
        let depth = args.first().map(abstract_ops::to_number).unwrap_or(1.0) as i64;
        let out = rt.alloc_object(Object::new_array());
        fn flat_into(rt: &mut Runtime, src: ObjectRef, out: ObjectRef, mut out_idx: usize, depth: i64) -> usize {
            let len = rt.array_length(src);
            for i in 0..len {
                let v = rt.object_get(src, &i.to_string());
                if depth > 0 {
                    if let Value::Object(nid) = &v {
                        if matches!(rt.obj(*nid).internal_kind, InternalKind::Array) {
                            out_idx = flat_into(rt, *nid, out, out_idx, depth - 1);
                            continue;
                        }
                    }
                }
                rt.object_set(out, out_idx.to_string(), v);
                out_idx += 1;
            }
            out_idx
        }
        let final_len = flat_into(rt, id, out, 0, depth);
        rt.object_set(out, "length".into(), Value::Number(final_len as f64));
        Ok(Value::Object(out))
    });
    register_intrinsic_method(rt, host, "flatMap", 1, |rt, args| {
        let id = to_array_this(rt)?;
        let cb = args.first().cloned().ok_or_else(||
            RuntimeError::TypeError("Array.prototype.flatMap: callback required".into()))?;
        if !rt.is_callable(&cb) {
            return Err(RuntimeError::TypeError("Array.prototype.flatMap: callback is not callable".into()));
        }
        let this_arg = args.get(1).cloned().unwrap_or(Value::Undefined);
        let len = rt.array_length(id);
        let out = rt.alloc_object(Object::new_array());
        let mut out_idx = 0usize;
        for i in 0..len {
            let v = rt.object_get(id, &i.to_string());
            let mapped = rt.call_function(cb.clone(), this_arg.clone(),
                vec![v, Value::Number(i as f64), Value::Object(id)])?;
            if let Value::Object(nid) = &mapped {
                if matches!(rt.obj(*nid).internal_kind, InternalKind::Array) {
                    let n = rt.array_length(*nid);
                    for j in 0..n {
                        let nv = rt.object_get(*nid, &j.to_string());
                        rt.object_set(out, out_idx.to_string(), nv);
                        out_idx += 1;
                    }
                    continue;
                }
            }
            rt.object_set(out, out_idx.to_string(), mapped);
            out_idx += 1;
        }
        rt.object_set(out, "length".into(), Value::Number(out_idx as f64));
        Ok(Value::Object(out))
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
        // Ω.5.P61.E13: per ECMA §23.1.3.24 skip sparse holes (HasProperty
        // check before invoking callback) and dispatch accessor reads
        // through read_property. Empty-with-no-initial throws even if
        // there are no enumerable elements (the spec's "no initial value
        // and no present elements" condition).
        let id = to_array_this(rt)?;
        let cb = args.first().cloned().ok_or_else(||
            RuntimeError::TypeError("Array.prototype.reduce: callback required".into()))?;
        if !rt.is_callable(&cb) {
            return Err(RuntimeError::TypeError("Array.prototype.reduce: callback is not callable".into()));
        }
        let len = rt.array_length(id);
        let has_init = args.len() >= 2;
        let mut i = 0usize;
        let mut acc = if has_init {
            args[1].clone()
        } else {
            // Find first present index to seed the accumulator.
            let mut seed: Option<(usize, Value)> = None;
            while i < len {
                let key = i.to_string();
                if rt.has_property(id, &key) {
                    let v = rt.read_property(id, &key)?;
                    seed = Some((i, v));
                    break;
                }
                i += 1;
            }
            match seed {
                Some((start, v)) => { i = start + 1; v }
                None => return Err(RuntimeError::TypeError(
                    "reduce of empty array with no initial value".into())),
            }
        };
        while i < len {
            let key = i.to_string();
            if rt.has_property(id, &key) {
                let v = rt.read_property(id, &key)?;
                acc = rt.call_function(cb.clone(), Value::Undefined,
                    vec![acc, v, Value::Number(i as f64), Value::Object(id)])?;
            }
            i += 1;
        }
        Ok(acc)
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
        // Ω.5.P62.E16: §23.1.3.27 — comparefn must be callable or
        // undefined; non-callable non-undefined throws TypeError up-front.
        let id = to_array_this(rt)?;
        let cmp_arg = args.first().cloned();
        let comparator = match cmp_arg {
            None | Some(Value::Undefined) => None,
            Some(v) => {
                if !rt.is_callable(&v) {
                    return Err(RuntimeError::TypeError(
                        "Array.prototype.sort: comparefn must be callable".into()));
                }
                Some(v)
            }
        };
        let len = rt.array_length(id);
        let mut items: Vec<Value> = (0..len).map(|i| rt.object_get(id, &i.to_string())).collect();
        // Stable sort. With comparator, use call_function; on error propagate.
        // sort_by needs a non-fallible cmp, so collect errors via interior state.
        let mut err: Option<RuntimeError> = None;
        match comparator {
            None => {
                items.sort_by(|a, b| {
                    let sa = abstract_ops::to_string(a);
                    let sb = abstract_ops::to_string(b);
                    sa.as_str().cmp(sb.as_str())
                });
            }
            Some(cb) => {
                items.sort_by(|a, b| {
                    if err.is_some() { return std::cmp::Ordering::Equal; }
                    match rt.call_function(cb.clone(), Value::Undefined, vec![a.clone(), b.clone()]) {
                        Ok(v) => {
                            let n = abstract_ops::to_number(&v);
                            if n.is_nan() { std::cmp::Ordering::Equal }
                            else if n < 0.0 { std::cmp::Ordering::Less }
                            else if n > 0.0 { std::cmp::Ordering::Greater }
                            else { std::cmp::Ordering::Equal }
                        }
                        Err(e) => { err = Some(e); std::cmp::Ordering::Equal }
                    }
                });
            }
        }
        if let Some(e) = err { return Err(e); }
        for (i, v) in items.into_iter().enumerate() {
            rt.object_set(id, i.to_string(), v);
        }
        rt.object_set(id, "length".into(), Value::Number(len as f64));
        Ok(Value::Object(id))
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
    register_intrinsic_method(rt, host, "entries", 0, |rt, _args| {
        let id = to_array_this(rt)?;
        let len = rt.array_length(id);
        let out = rt.alloc_object(Object::new_array());
        for i in 0..len {
            let v = rt.object_get(id, &i.to_string());
            let pair = rt.alloc_object(Object::new_array());
            rt.object_set(pair, "0".into(), Value::Number(i as f64));
            rt.object_set(pair, "1".into(), v);
            rt.object_set(pair, "length".into(), Value::Number(2.0));
            rt.object_set(out, i.to_string(), Value::Object(pair));
        }
        rt.object_set(out, "length".into(), Value::Number(len as f64));
        Ok(Value::Object(out))
    });
    register_intrinsic_method(rt, host, "keys", 0, |rt, _args| {
        let id = to_array_this(rt)?;
        let len = rt.array_length(id);
        let out = rt.alloc_object(Object::new_array());
        for i in 0..len {
            rt.object_set(out, i.to_string(), Value::Number(i as f64));
        }
        rt.object_set(out, "length".into(), Value::Number(len as f64));
        Ok(Value::Object(out))
    });
    register_intrinsic_method(rt, host, "values", 0, |rt, _args| {
        let id = to_array_this(rt)?;
        let len = rt.array_length(id);
        let out = rt.alloc_object(Object::new_array());
        for i in 0..len {
            let v = rt.object_get(id, &i.to_string());
            rt.object_set(out, i.to_string(), v);
        }
        rt.object_set(out, "length".into(), Value::Number(len as f64));
        Ok(Value::Object(out))
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
        let id = to_array_this(rt)?;
        let cb = args.first().cloned().ok_or_else(||
            RuntimeError::TypeError("findLast: callback required".into()))?;
        if !rt.is_callable(&cb) {
            return Err(RuntimeError::TypeError("Array.prototype.findLast: callback is not callable".into()));
        }
        let this_arg = args.get(1).cloned().unwrap_or(Value::Undefined);
        let len = rt.array_length(id);
        for i in (0..len).rev() {
            let v = rt.object_get(id, &i.to_string());
            let r = rt.call_function(cb.clone(), this_arg.clone(),
                vec![v.clone(), Value::Number(i as f64), Value::Object(id)])?;
            if abstract_ops::to_boolean(&r) { return Ok(v); }
        }
        Ok(Value::Undefined)
    });

    register_intrinsic_method(rt, host, "findLastIndex", 1, |rt, args| {
        let id = to_array_this(rt)?;
        let cb = args.first().cloned().ok_or_else(||
            RuntimeError::TypeError("findLastIndex: callback required".into()))?;
        if !rt.is_callable(&cb) {
            return Err(RuntimeError::TypeError("Array.prototype.findLastIndex: callback is not callable".into()));
        }
        let this_arg = args.get(1).cloned().unwrap_or(Value::Undefined);
        let len = rt.array_length(id);
        for i in (0..len).rev() {
            let v = rt.object_get(id, &i.to_string());
            let r = rt.call_function(cb.clone(), this_arg.clone(),
                vec![v, Value::Number(i as f64), Value::Object(id)])?;
            if abstract_ops::to_boolean(&r) { return Ok(Value::Number(i as f64)); }
        }
        Ok(Value::Number(-1.0))
    });

    register_intrinsic_method(rt, host, "reduceRight", 1, |rt, args| {
        // Ω.5.P61.E13: sparse-skip + getter dispatch per ECMA §23.1.3.25.
        let id = to_array_this(rt)?;
        let cb = args.first().cloned().ok_or_else(||
            RuntimeError::TypeError("reduceRight: callback required".into()))?;
        if !rt.is_callable(&cb) {
            return Err(RuntimeError::TypeError("Array.prototype.reduceRight: callback is not callable".into()));
        }
        let len = rt.array_length(id);
        let has_init = args.len() >= 2;
        let mut i: i64 = (len as i64) - 1;
        let mut acc = if has_init { args[1].clone() } else {
            // Find last present index.
            let mut seed: Option<(i64, Value)> = None;
            while i >= 0 {
                let key = i.to_string();
                if rt.has_property(id, &key) {
                    let v = rt.read_property(id, &key)?;
                    seed = Some((i, v)); break;
                }
                i -= 1;
            }
            match seed {
                Some((start, v)) => { i = start - 1; v }
                None => return Err(RuntimeError::TypeError(
                    "reduce of empty array with no initial value".into())),
            }
        };
        while i >= 0 {
            let key = i.to_string();
            if rt.has_property(id, &key) {
                let v = rt.read_property(id, &key)?;
                acc = rt.call_function(cb.clone(), Value::Undefined,
                    vec![acc, v, Value::Number(i as f64), Value::Object(id)])?;
            }
            i -= 1;
        }
        Ok(acc)
    });

    register_intrinsic_method(rt, host, "lastIndexOf", 1, |rt, args| {
        // Ω.5.P61.E14: sparse-skip per ECMA §23.1.3.18.
        let id = to_array_this(rt)?;
        let needle = args.first().cloned().unwrap_or(Value::Undefined);
        let len = rt.array_length(id) as i64;
        let from = match args.get(1) {
            Some(v) if !matches!(v, Value::Undefined) => {
                let n = abstract_ops::to_number(v) as i64;
                if n < 0 { (len + n).max(-1) } else { (n.min(len - 1)).max(-1) }
            }
            _ => (len - 1).max(-1),
        };
        let mut i = from;
        while i >= 0 {
            let key = i.to_string();
            if rt.has_property(id, &key) {
                let v = rt.read_property(id, &key)?;
                if abstract_ops::is_strictly_equal(&v, &needle) {
                    return Ok(Value::Number(i as f64));
                }
            }
            i -= 1;
        }
        Ok(Value::Number(-1.0))
    });

    register_intrinsic_method(rt, host, "copyWithin", 2, |rt, args| {
        // ECMA §23.1.3.4: arr.copyWithin(target, start, end).
        // Ω.5.P62.E18: coerce_to_number on all three positional args
        // so Symbol→TypeError and Object→valueOf dispatch per spec.
        let id = to_array_this(rt)?;
        let len = rt.array_length(id) as i64;
        let arg_n = |rt: &mut Runtime, i: usize, default: i64| -> Result<i64, RuntimeError> {
            match args.get(i).cloned() {
                Some(Value::Undefined) | None => Ok(default),
                Some(v) => Ok(rt.coerce_to_number(&v)? as i64),
            }
        };
        let to = clamp_index(arg_n(rt, 0, 0)?, len);
        let from = clamp_index(arg_n(rt, 1, 0)?, len);
        let end = clamp_index(arg_n(rt, 2, len)?, len);
        let count = (end - from).min(len - to).max(0);
        // Read-then-write to handle overlap correctly.
        let buf: Vec<Value> = (0..count).map(|i|
            rt.object_get(id, &(from + i).to_string())).collect();
        for (i, v) in buf.into_iter().enumerate() {
            rt.object_set(id, (to + i as i64).to_string(), v);
        }
        Ok(Value::Object(id))
    });

    register_intrinsic_method(rt, host, "toReversed", 0, |rt, _args| {
        let id = to_array_this(rt)?;
        let len = rt.array_length(id);
        let out = rt.alloc_object(Object::new_array());
        for i in 0..len {
            let v = rt.object_get(id, &(len - 1 - i).to_string());
            rt.object_set(out, i.to_string(), v);
        }
        rt.object_set(out, "length".into(), Value::Number(len as f64));
        Ok(Value::Object(out))
    });

    register_intrinsic_method(rt, host, "toSorted", 1, |rt, args| {
        let id = to_array_this(rt)?;
        let len = rt.array_length(id);
        let out = rt.alloc_object(Object::new_array());
        for i in 0..len {
            rt.object_set(out, i.to_string(), rt.object_get(id, &i.to_string()));
        }
        rt.object_set(out, "length".into(), Value::Number(len as f64));
        // Reuse sort by setting this and invoking via call_function path —
        // simpler to inline the body here.
        let comparator = args.first().cloned().filter(|v| !matches!(v, Value::Undefined));
        let mut items: Vec<Value> = (0..len).map(|i| rt.object_get(out, &i.to_string())).collect();
        let mut err: Option<RuntimeError> = None;
        match comparator {
            Some(cmp) => {
                items.sort_by(|a, b| {
                    if err.is_some() { return std::cmp::Ordering::Equal; }
                    match rt.call_function(cmp.clone(), Value::Undefined, vec![a.clone(), b.clone()]) {
                        Ok(Value::Number(n)) => {
                            if n < 0.0 { std::cmp::Ordering::Less }
                            else if n > 0.0 { std::cmp::Ordering::Greater }
                            else { std::cmp::Ordering::Equal }
                        }
                        Ok(_) => std::cmp::Ordering::Equal,
                        Err(e) => { err = Some(e); std::cmp::Ordering::Equal }
                    }
                });
            }
            None => {
                items.sort_by(|a, b| {
                    let sa = abstract_ops::to_string(a);
                    let sb = abstract_ops::to_string(b);
                    sa.as_str().cmp(sb.as_str())
                });
            }
        }
        if let Some(e) = err { return Err(e); }
        for (i, v) in items.into_iter().enumerate() {
            rt.object_set(out, i.to_string(), v);
        }
        Ok(Value::Object(out))
    });

    register_intrinsic_method(rt, host, "toSpliced", 2, |rt, args| {
        let id = to_array_this(rt)?;
        let len = rt.array_length(id) as i64;
        let start = clamp_index(args.first().map(abstract_ops::to_number).unwrap_or(0.0) as i64, len);
        let del = match args.get(1) {
            Some(v) => {
                let n = abstract_ops::to_number(v) as i64;
                n.max(0).min(len - start)
            }
            None => len - start,
        };
        let inserts: Vec<Value> = args.iter().skip(2).cloned().collect();
        let new_len = len - del + inserts.len() as i64;
        let out = rt.alloc_object(Object::new_array());
        let mut k = 0i64;
        for i in 0..start {
            rt.object_set(out, k.to_string(), rt.object_get(id, &i.to_string()));
            k += 1;
        }
        for v in inserts {
            rt.object_set(out, k.to_string(), v);
            k += 1;
        }
        for i in (start + del)..len {
            rt.object_set(out, k.to_string(), rt.object_get(id, &i.to_string()));
            k += 1;
        }
        rt.object_set(out, "length".into(), Value::Number(new_len as f64));
        Ok(Value::Object(out))
    });

    register_intrinsic_method(rt, host, "with", 2, |rt, args| {
        let id = to_array_this(rt)?;
        let len = rt.array_length(id) as i64;
        let idx = args.first().map(abstract_ops::to_number).unwrap_or(0.0) as i64;
        let actual = if idx < 0 { len + idx } else { idx };
        if actual < 0 || actual >= len {
            return Err(RuntimeError::RangeError(format!("with: index {} out of bounds", idx)));
        }
        let val = args.get(1).cloned().unwrap_or(Value::Undefined);
        let out = rt.alloc_object(Object::new_array());
        for i in 0..len {
            let v = if i == actual { val.clone() } else { rt.object_get(id, &i.to_string()) };
            rt.object_set(out, i.to_string(), v);
        }
        rt.object_set(out, "length".into(), Value::Number(len as f64));
        Ok(Value::Object(out))
    });

    register_intrinsic_method(rt, host, "toLocaleString", 0, |rt, _args| {
        // v1: toLocaleString as locale-insensitive toString — comma-join.
        let id = to_array_this(rt)?;
        let len = rt.array_length(id);
        let mut out = String::new();
        for i in 0..len {
            if i > 0 { out.push(','); }
            let v = rt.object_get(id, &i.to_string());
            out.push_str(abstract_ops::to_string(&v).as_str());
        }
        Ok(Value::String(Rc::new(out)))
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
        Value::Undefined => {
            Ok(matches!(rt.obj(id).internal_kind, InternalKind::RegExp(_)))
        }
        _ => Ok(crate::abstract_ops::to_boolean(&matcher)),
    }
}

fn install_string_proto(rt: &mut Runtime, host: ObjectRef) {
    // Ω.5.P62.E15: case + trim families per ECMA §22.1.3.{27,28,29,30,31,32}.
    // All require RequireObjectCoercible + strict ToString(this).
    register_intrinsic_method(rt, host, "toUpperCase", 0, |rt, _args| {
        let this = rt.current_this(); rt.require_object_coercible(&this)?;
        Ok(Value::String(Rc::new(rt.to_string_strict(&this)?.to_uppercase())))
    });
    register_intrinsic_method(rt, host, "toLowerCase", 0, |rt, _args| {
        let this = rt.current_this(); rt.require_object_coercible(&this)?;
        Ok(Value::String(Rc::new(rt.to_string_strict(&this)?.to_lowercase())))
    });
    register_intrinsic_method(rt, host, "toLocaleLowerCase", 0, |rt, _args| {
        let this = rt.current_this(); rt.require_object_coercible(&this)?;
        Ok(Value::String(Rc::new(rt.to_string_strict(&this)?.to_lowercase())))
    });
    register_intrinsic_method(rt, host, "toLocaleUpperCase", 0, |rt, _args| {
        let this = rt.current_this(); rt.require_object_coercible(&this)?;
        Ok(Value::String(Rc::new(rt.to_string_strict(&this)?.to_uppercase())))
    });
    register_intrinsic_method(rt, host, "trim", 0, |rt, _args| {
        let this = rt.current_this(); rt.require_object_coercible(&this)?;
        Ok(Value::String(Rc::new(rt.to_string_strict(&this)?.trim().to_string())))
    });
    register_intrinsic_method(rt, host, "trimStart", 0, |rt, _args| {
        let this = rt.current_this(); rt.require_object_coercible(&this)?;
        Ok(Value::String(Rc::new(rt.to_string_strict(&this)?.trim_start().to_string())))
    });
    register_intrinsic_method(rt, host, "trimEnd", 0, |rt, _args| {
        let this = rt.current_this(); rt.require_object_coercible(&this)?;
        Ok(Value::String(Rc::new(rt.to_string_strict(&this)?.trim_end().to_string())))
    });
    register_intrinsic_method(rt, host, "trimLeft", 0, |rt, _args| {
        let this = rt.current_this(); rt.require_object_coercible(&this)?;
        Ok(Value::String(Rc::new(rt.to_string_strict(&this)?.trim_start().to_string())))
    });
    register_intrinsic_method(rt, host, "trimRight", 0, |rt, _args| {
        let this = rt.current_this(); rt.require_object_coercible(&this)?;
        Ok(Value::String(Rc::new(rt.to_string_strict(&this)?.trim_end().to_string())))
    });
    // normalize(form?) — Unicode normalization. v1 deviation: pass-through
    // (return source unchanged). Most consumer code only invokes when input
    // is already ASCII; the cluster's load-bearing test is presence, not
    // correctness of NFC/NFD/NFKC/NFKD conversion.
    register_intrinsic_method(rt, host, "normalize", 0, |rt, _args| {
        let this = rt.current_this(); rt.require_object_coercible(&this)?;
        Ok(Value::String(Rc::new(rt.to_string_strict(&this)?)))
    });
    register_intrinsic_method(rt, host, "charAt", 1, |rt, args| {
        let this = rt.current_this(); rt.require_object_coercible(&this)?;
        let s = rt.to_string_strict(&this)?;
        let i_n = match args.first().cloned() { Some(v) => rt.coerce_to_number(&v)?, None => 0.0 };
        // §22.1.3.1: out-of-range returns "" (matches existing behavior).
        if !i_n.is_finite() || i_n < 0.0 { return Ok(Value::String(Rc::new(String::new()))); }
        let i = i_n as usize;
        let c = s.chars().nth(i).map(|c| c.to_string()).unwrap_or_default();
        Ok(Value::String(Rc::new(c)))
    });
    register_intrinsic_method(rt, host, "charCodeAt", 1, |rt, args| {
        let this = rt.current_this(); rt.require_object_coercible(&this)?;
        let s = rt.to_string_strict(&this)?;
        let i_n = match args.first().cloned() { Some(v) => rt.coerce_to_number(&v)?, None => 0.0 };
        if !i_n.is_finite() || i_n < 0.0 { return Ok(Value::Number(f64::NAN)); }
        match s.chars().nth(i_n as usize) {
            Some(c) => Ok(Value::Number(c as u32 as f64)),
            None => Ok(Value::Number(f64::NAN)),
        }
    });
    // §22.1.3.3: concat receiver + ToString(args) variadic.
    register_intrinsic_method(rt, host, "concat", 1, |rt, args| {
        let this = rt.current_this(); rt.require_object_coercible(&this)?;
        let mut s = rt.to_string_strict(&this)?;
        for a in args {
            s.push_str(&rt.to_string_strict(a)?);
        }
        Ok(Value::String(Rc::new(s)))
    });
    // Tier-Ω.5.EEEEEEEE: String.prototype.localeCompare per ECMA-262 §22.1.3.10.
    // Used by sort comparators throughout the corpus (read-pkg/spdx-correct
    // family, conventional-changelog, meow). v1 deviation: locale-insensitive
    // lexicographic compare (real impl needs full Intl Collator chain).
    register_intrinsic_method(rt, host, "localeCompare", 1, |rt, args| {
        let this = rt.current_this(); rt.require_object_coercible(&this)?;
        let a = rt.to_string_strict(&this)?;
        let b = rt.to_string_strict(&args.first().cloned().unwrap_or(Value::Undefined))?;
        Ok(Value::Number(match a.cmp(&b) {
            std::cmp::Ordering::Less => -1.0,
            std::cmp::Ordering::Equal => 0.0,
            std::cmp::Ordering::Greater => 1.0,
        }))
    });
    // Tier-Ω.5.GGGGGGG: String.prototype.codePointAt per ECMA-262 §22.1.3.4.
    // Returns the full code point (handles surrogate pairs) at the given
    // UTF-16 index; returns undefined if the index is out of range.
    // cli-truncate/execa/multiformats/strip-final-newline/tar all read
    // codePointAt at module-init for ANSI / encoding detection.
    register_intrinsic_method(rt, host, "codePointAt", 1, |rt, args| {
        let this = rt.current_this(); rt.require_object_coercible(&this)?;
        let s = rt.to_string_strict(&this)?;
        let i_n = match args.first().cloned() { Some(v) => rt.coerce_to_number(&v)?, None => 0.0 };
        if !i_n.is_finite() || i_n < 0.0 { return Ok(Value::Undefined); }
        let i = i_n as i64;
        // The spec is UTF-16 indexed; our Rust strings are UTF-8 / chars().
        // Iterate chars accumulating UTF-16 code-units; when the cumulative
        // count crosses i, return the current char's code point.
        let mut u16_idx: i64 = 0;
        for c in s.chars() {
            let units = c.len_utf16() as i64;
            if u16_idx == i { return Ok(Value::Number(c as u32 as f64)); }
            if u16_idx < i && i < u16_idx + units {
                // Trail surrogate — return the low surrogate value.
                let cp = c as u32;
                let low = 0xDC00 + ((cp - 0x10000) & 0x3FF);
                return Ok(Value::Number(low as f64));
            }
            u16_idx += units;
        }
        Ok(Value::Undefined)
    });
    register_intrinsic_method(rt, host, "slice", 2, |rt, args| {
        // Ω.5.P62.E14: RequireObjectCoercible + strict ToString(this);
        // ToInteger args propagates abrupt completions.
        let this = rt.current_this();
        rt.require_object_coercible(&this)?;
        let s = rt.to_string_strict(&this)?;
        let chars: Vec<char> = s.chars().collect();
        let len = chars.len() as i64;
        let start_n = match args.first().cloned() {
            Some(v) => rt.coerce_to_number(&v)?,
            None => 0.0,
        };
        let end_n = match args.get(1).cloned() {
            Some(Value::Undefined) | None => len as f64,
            Some(v) => rt.coerce_to_number(&v)?,
        };
        let start = clamp_index(start_n as i64, len);
        let end = clamp_index(end_n as i64, len);
        if end <= start { return Ok(Value::String(Rc::new(String::new()))); }
        let out: String = chars[start as usize..end as usize].iter().collect();
        Ok(Value::String(Rc::new(out)))
    });
    // Tier-Ω.5.aaaaaa: String.prototype.substr (legacy Annex B.2.2.1).
    register_intrinsic_method(rt, host, "substr", 2, |rt, args| {
        let this = rt.current_this();
        rt.require_object_coercible(&this)?;
        let s = rt.to_string_strict(&this)?;
        let chars: Vec<char> = s.chars().collect();
        let len = chars.len() as i64;
        let mut start = match args.first().cloned() {
            Some(v) => rt.coerce_to_number(&v)? as i64, None => 0,
        };
        if start < 0 { start = (len + start).max(0); }
        let start = start.min(len) as usize;
        let count_n = match args.get(1).cloned() {
            Some(Value::Undefined) | None => (len - start as i64) as f64,
            Some(v) => rt.coerce_to_number(&v)?,
        };
        let count = (count_n as i64).max(0) as usize;
        let end = (start + count).min(chars.len());
        let out: String = chars[start..end].iter().collect();
        Ok(Value::String(Rc::new(out)))
    });
    register_intrinsic_method(rt, host, "substring", 2, |rt, args| {
        let this = rt.current_this();
        rt.require_object_coercible(&this)?;
        let s = rt.to_string_strict(&this)?;
        let chars: Vec<char> = s.chars().collect();
        let len = chars.len() as i64;
        let a_n = match args.first().cloned() {
            Some(v) => rt.coerce_to_number(&v)?, None => 0.0,
        };
        let b_n = match args.get(1).cloned() {
            Some(Value::Undefined) | None => len as f64,
            Some(v) => rt.coerce_to_number(&v)?,
        };
        let mut a = (a_n as i64).clamp(0, len);
        let mut b = (b_n as i64).clamp(0, len);
        if a > b { std::mem::swap(&mut a, &mut b); }
        let out: String = chars[a as usize..b as usize].iter().collect();
        Ok(Value::String(Rc::new(out)))
    });
    register_intrinsic_method(rt, host, "indexOf", 1, |rt, args| {
        let this = rt.current_this();
        rt.require_object_coercible(&this)?;
        let s = rt.to_string_strict(&this)?;
        let needle = rt.to_string_strict(&args.first().cloned().unwrap_or(Value::Undefined))?;
        if let Some(pv) = args.get(1).cloned() { let _ = rt.coerce_to_number(&pv)?; }
        match s.find(&needle) {
            Some(byte_off) => Ok(Value::Number(s[..byte_off].chars().count() as f64)),
            None => Ok(Value::Number(-1.0)),
        }
    });
    register_intrinsic_method(rt, host, "lastIndexOf", 1, |rt, args| {
        let this = rt.current_this();
        rt.require_object_coercible(&this)?;
        let s = rt.to_string_strict(&this)?;
        let needle = rt.to_string_strict(&args.first().cloned().unwrap_or(Value::Undefined))?;
        if let Some(pv) = args.get(1).cloned() { let _ = rt.coerce_to_number(&pv)?; }
        match s.rfind(&needle) {
            Some(byte_off) => Ok(Value::Number(s[..byte_off].chars().count() as f64)),
            None => Ok(Value::Number(-1.0)),
        }
    });
    register_intrinsic_method(rt, host, "includes", 1, |rt, args| {
        // Ω.5.P62.E13: spec §22.1.3.7 — RequireObjectCoercible, then
        // reject RegExp searchStr (IsRegExp), then ToString both sides,
        // then ToInteger position. Symbol surfaces as TypeError via
        // to_string_strict; throwing valueOf/toString propagates.
        let this = rt.current_this();
        rt.require_object_coercible(&this)?;
        let needle_v = args.first().cloned().unwrap_or(Value::Undefined);
        if is_regexp_like(rt, &needle_v)? {
            return Err(RuntimeError::TypeError(
                "String.prototype.includes: searchString cannot be a RegExp".into()));
        }
        let s = rt.to_string_strict(&this)?;
        let needle = rt.to_string_strict(&needle_v)?;
        if let Some(pv) = args.get(1).cloned() {
            // ToInteger via coerce_to_number — abrupt completion
            // propagates through ? naturally.
            let _ = rt.coerce_to_number(&pv)?;
        }
        Ok(Value::Boolean(s.contains(&needle)))
    });
    register_intrinsic_method(rt, host, "startsWith", 1, |rt, args| {
        let this = rt.current_this();
        rt.require_object_coercible(&this)?;
        let needle_v = args.first().cloned().unwrap_or(Value::Undefined);
        if is_regexp_like(rt, &needle_v)? {
            return Err(RuntimeError::TypeError(
                "String.prototype.startsWith: searchString cannot be a RegExp".into()));
        }
        let s = rt.to_string_strict(&this)?;
        let needle = rt.to_string_strict(&needle_v)?;
        if let Some(pv) = args.get(1).cloned() { let _ = rt.coerce_to_number(&pv)?; }
        Ok(Value::Boolean(s.starts_with(&needle)))
    });
    register_intrinsic_method(rt, host, "endsWith", 1, |rt, args| {
        let this = rt.current_this();
        rt.require_object_coercible(&this)?;
        let needle_v = args.first().cloned().unwrap_or(Value::Undefined);
        if is_regexp_like(rt, &needle_v)? {
            return Err(RuntimeError::TypeError(
                "String.prototype.endsWith: searchString cannot be a RegExp".into()));
        }
        let s = rt.to_string_strict(&this)?;
        let needle = rt.to_string_strict(&needle_v)?;
        if let Some(pv) = args.get(1).cloned() { let _ = rt.coerce_to_number(&pv)?; }
        Ok(Value::Boolean(s.ends_with(&needle)))
    });
    register_intrinsic_method(rt, host, "split", 2, |rt, args| {
        // §22.1.3.23: RequireObjectCoercible(this), then dispatch to
        // separator[@@split] if present (RegExp case), else ToString
        // both sides, then split with optional limit (ToUint32).
        let this = rt.current_this(); rt.require_object_coercible(&this)?;
        let sep_v = args.first().cloned().unwrap_or(Value::Undefined);
        // RegExp @@split dispatch.
        if let Value::Object(rx_id) = &sep_v {
            let split_method = rt.read_property(*rx_id, "@@split")?;
            if matches!(split_method, Value::Object(_)) {
                let limit = args.get(1).cloned().unwrap_or(Value::Undefined);
                let s = rt.to_string_strict(&this)?;
                return rt.call_function(split_method, sep_v.clone(),
                    vec![Value::String(Rc::new(s)), limit]);
            }
        }
        let s = rt.to_string_strict(&this)?;
        let limit_n = match args.get(1).cloned() {
            Some(Value::Undefined) | None => u32::MAX,
            Some(v) => {
                let n = rt.coerce_to_number(&v)?;
                if n.is_nan() || n <= 0.0 { 0 } else { n as u32 }
            }
        };
        let out = rt.alloc_object(Object::new_array());
        if limit_n == 0 {
            rt.object_set(out, "length".into(), Value::Number(0.0));
            return Ok(Value::Object(out));
        }
        let mut parts: Vec<String> = match &sep_v {
            Value::Undefined => vec![s.clone()],
            _ => {
                let sep = rt.to_string_strict(&sep_v)?;
                if sep.is_empty() {
                    // §22.1.3.23 step 12: if S is empty, return [].
                    if s.is_empty() {
                        rt.object_set(out, "length".into(), Value::Number(0.0));
                        return Ok(Value::Object(out));
                    }
                    s.chars().map(|c| c.to_string()).collect()
                } else if s.is_empty() {
                    vec![s.clone()]
                } else {
                    s.split(&sep).map(|s| s.to_string()).collect()
                }
            }
        };
        if (parts.len() as u32) > limit_n {
            parts.truncate(limit_n as usize);
        }
        for (i, p) in parts.iter().enumerate() {
            rt.object_set(out, i.to_string(), Value::String(Rc::new(p.clone())));
        }
        rt.object_set(out, "length".into(), Value::Number(parts.len() as f64));
        Ok(Value::Object(out))
    });
    register_intrinsic_method(rt, host, "repeat", 1, |rt, args| {
        // §22.1.3.21: negative or Infinity count throws RangeError.
        let this = rt.current_this(); rt.require_object_coercible(&this)?;
        let s = rt.to_string_strict(&this)?;
        let n_n = match args.first().cloned() { Some(v) => rt.coerce_to_number(&v)?, None => 0.0 };
        if n_n.is_nan() || n_n < 0.0 || n_n == f64::INFINITY {
            return Err(RuntimeError::RangeError(
                "Invalid count value".into()));
        }
        let n = n_n as usize;
        Ok(Value::String(Rc::new(s.repeat(n))))
    });
    // Tier-Ω.5.iiiiii: String.prototype.matchAll per ECMA §22.1.3.13.
    // Returns an iterator over all matches of a regex with the /g flag.
    register_intrinsic_method(rt, host, "matchAll", 1, |rt, args| {
        let s = abstract_ops::to_string(&rt.current_this()).as_str().to_string();
        let regex_v = args.first().cloned().unwrap_or(Value::Undefined);
        let regex_id = match &regex_v {
            Value::Object(id) => *id,
            _ => return Err(RuntimeError::TypeError("matchAll requires a regex".into())),
        };
        let out = rt.alloc_object(Object::new_array());
        // Iterate via repeated regex.exec, advancing lastIndex.
        rt.object_set(regex_id, "lastIndex".into(), Value::Number(0.0));
        let exec = rt.object_get(regex_id, "exec");
        if !matches!(exec, Value::Object(_)) {
            return Err(RuntimeError::TypeError("matchAll: regex.exec not callable".into()));
        }
        let mut idx = 0usize;
        loop {
            let r = rt.call_function(exec.clone(), regex_v.clone(), vec![Value::String(Rc::new(s.clone()))])?;
            match r {
                Value::Null | Value::Undefined => break,
                Value::Object(match_id) => {
                    rt.object_set(out, idx.to_string(), Value::Object(match_id));
                    idx += 1;
                }
                _ => break,
            }
            if idx > 100000 { break; } // safety
        }
        rt.object_set(out, "length".into(), Value::Number(idx as f64));
        Ok(Value::Object(out))
    });
    // Tier-Ω.5.ppppp: padStart / padEnd per ECMA-262 §22.1.3.16 / §22.1.3.17.
    // date-fns / left-pad / many formatting libs reach for these.
    register_intrinsic_method(rt, host, "padStart", 1, |rt, args| {
        let this = rt.current_this(); rt.require_object_coercible(&this)?;
        let s = rt.to_string_strict(&this)?;
        let target_n = match args.first().cloned() { Some(v) => rt.coerce_to_number(&v)?, None => 0.0 };
        let target = if target_n.is_nan() || target_n <= 0.0 { 0 } else { target_n as usize };
        let pad = match args.get(1).cloned() {
            Some(Value::Undefined) | None => " ".to_string(),
            Some(v) => rt.to_string_strict(&v)?,
        };
        if s.chars().count() >= target || pad.is_empty() {
            return Ok(Value::String(Rc::new(s)));
        }
        let need = target - s.chars().count();
        let mut prefix = String::new();
        while prefix.chars().count() < need { prefix.push_str(&pad); }
        let prefix: String = prefix.chars().take(need).collect();
        Ok(Value::String(Rc::new(prefix + &s)))
    });
    register_intrinsic_method(rt, host, "padEnd", 1, |rt, args| {
        let this = rt.current_this(); rt.require_object_coercible(&this)?;
        let s = rt.to_string_strict(&this)?;
        let target_n = match args.first().cloned() { Some(v) => rt.coerce_to_number(&v)?, None => 0.0 };
        let target = if target_n.is_nan() || target_n <= 0.0 { 0 } else { target_n as usize };
        let pad = match args.get(1).cloned() {
            Some(Value::Undefined) | None => " ".to_string(),
            Some(v) => rt.to_string_strict(&v)?,
        };
        if s.chars().count() >= target || pad.is_empty() {
            return Ok(Value::String(Rc::new(s)));
        }
        let need = target - s.chars().count();
        let mut suffix = String::new();
        while suffix.chars().count() < need { suffix.push_str(&pad); }
        let suffix: String = suffix.chars().take(need).collect();
        Ok(Value::String(Rc::new(s + &suffix)))
    });
    register_intrinsic_method(rt, host, "replace", 2, |rt, args| {
        // §22.1.3.15 (string-replacer subset; regex-replacer goes through
        // RegExp.prototype[@@replace] which has its own dispatch).
        let this = rt.current_this(); rt.require_object_coercible(&this)?;
        let s = rt.to_string_strict(&this)?;
        let needle_v = args.first().cloned().unwrap_or(Value::Undefined);
        // If searchValue is a RegExp, delegate to its @@replace method.
        if let Value::Object(rx_id) = &needle_v {
            let replace_method = rt.read_property(*rx_id, "@@replace")?;
            if matches!(replace_method, Value::Object(_)) {
                let replacer = args.get(1).cloned().unwrap_or(Value::Undefined);
                return rt.call_function(replace_method, needle_v.clone(),
                    vec![Value::String(Rc::new(s)), replacer]);
            }
        }
        let needle = rt.to_string_strict(&needle_v)?;
        let repl_v = args.get(1).cloned().unwrap_or(Value::Undefined);
        // If replacer is callable, invoke it with (match, position, source).
        if rt.is_callable(&repl_v) {
            match s.find(&needle) {
                Some(byte_off) => {
                    let pos = s[..byte_off].chars().count() as f64;
                    let r = rt.call_function(repl_v, Value::Undefined, vec![
                        Value::String(Rc::new(needle.clone())),
                        Value::Number(pos),
                        Value::String(Rc::new(s.clone())),
                    ])?;
                    let repl_str = rt.to_string_strict(&r)?;
                    let mut out = String::with_capacity(s.len());
                    out.push_str(&s[..byte_off]);
                    out.push_str(&repl_str);
                    out.push_str(&s[byte_off + needle.len()..]);
                    return Ok(Value::String(Rc::new(out)));
                }
                None => return Ok(Value::String(Rc::new(s))),
            }
        }
        let repl = rt.to_string_strict(&repl_v)?;
        Ok(Value::String(Rc::new(s.replacen(&needle, &repl, 1))))
    });
    register_intrinsic_method(rt, host, "at", 1, |rt, args| {
        let this = rt.current_this(); rt.require_object_coercible(&this)?;
        let s = rt.to_string_strict(&this)?;
        let chars: Vec<char> = s.chars().collect();
        let len = chars.len() as i64;
        let i_n = match args.first().cloned() { Some(v) => rt.coerce_to_number(&v)?, None => 0.0 };
        let i = i_n as i64;
        let idx = if i < 0 { len + i } else { i };
        if idx < 0 || idx >= len { return Ok(Value::Undefined); }
        Ok(Value::String(Rc::new(chars[idx as usize].to_string())))
    });
    register_intrinsic_method(rt, host, "toString", 0, |rt, _args| {
        // Ω.5.P62.E1: unwrap String-wrapper [[StringData]] before coerce.
        let this = rt.current_this();
        let t = rt.unwrap_primitive(&this);
        match t {
            Value::String(s) => Ok(Value::String(s)),
            _ => Err(RuntimeError::TypeError("String.prototype.toString: this is not a String".into())),
        }
    });
    register_intrinsic_method(rt, host, "valueOf", 0, |rt, _args| {
        let this = rt.current_this();
        let t = rt.unwrap_primitive(&this);
        match t {
            Value::String(s) => Ok(Value::String(s)),
            _ => Err(RuntimeError::TypeError("String.prototype.valueOf: this is not a String".into())),
        }
    });
    register_intrinsic_method(rt, host, "@@iterator", 0, |rt, _args| {
        let this = rt.current_this();
        let s = abstract_ops::to_string(&rt.unwrap_primitive(&this)).as_str().to_string();
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
    register_intrinsic_method(rt, host, "toString", 0, |rt, _args| {
        let this = rt.current_this();
        let s = match &this {
            Value::Object(id) => {
                let name = match &rt.obj(*id).internal_kind {
                    InternalKind::Function(f) => f.name.clone(),
                    InternalKind::Closure(c) => {
                        // FunctionProto carries no name field directly;
                        // use a generic placeholder for closures.
                        let _ = c;
                        "anonymous".to_string()
                    }
                    InternalKind::BoundFunction(_) => "bound".to_string(),
                    _ => return Err(RuntimeError::TypeError("Function.prototype.toString: not a function".into())),
                };
                format!("function {}() {{ [native code] }}", name)
            }
            _ => return Err(RuntimeError::TypeError("Function.prototype.toString: not a function".into())),
        };
        Ok(Value::String(Rc::new(s)))
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
                (0..len).map(|i| rt.object_get(aid, &i.to_string())).collect()
            }
            Value::Null | Value::Undefined => Vec::new(),
            _ => return Err(RuntimeError::TypeError("apply: argsArray must be an Array".into())),
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
    register_intrinsic_method(rt, host, "then", 1, |rt, args| {
        let source = match rt.current_this() {
            Value::Object(id) => id,
            _ => return Err(RuntimeError::TypeError("Promise.prototype.then: this is not a Promise".into())),
        };
        promise_then_impl(rt, source, args.first().cloned(), args.get(1).cloned())
    });
    register_intrinsic_method(rt, host, "catch", 1, |rt, args| {
        let source = match rt.current_this() {
            Value::Object(id) => id,
            _ => return Err(RuntimeError::TypeError("Promise.prototype.catch: this is not a Promise".into())),
        };
        promise_then_impl(rt, source, None, args.first().cloned())
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
            _ => return Err(RuntimeError::TypeError("then: source is not a Promise".into())),
        }
    };
    match status {
        PromiseStatus::Pending => {
            let src = rt.obj_mut(source);
            if let InternalKind::Promise(ps) = &mut src.internal_kind {
                ps.fulfill_reactions.push(PromiseReaction { handler: on_fulfilled, chain });
                ps.reject_reactions.push(PromiseReaction { handler: on_rejected, chain });
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
                Ok(r) => { crate::promise::resolve_promise(rt, chain, r); }
                Err(e) => {
                    let thrown = match e {
                        RuntimeError::Thrown(v) => v,
                        other => Value::String(Rc::new(format!("{:?}", other))),
                    };
                    crate::promise::reject_promise(rt, chain, thrown);
                }
            }
            None => if is_rejected {
                crate::promise::reject_promise(rt, chain, value);
            } else {
                crate::promise::resolve_promise(rt, chain, value);
            }
        }
        Ok(())
    });
}

// ──────────────── %Number.prototype% ────────────────

fn install_number_proto(rt: &mut Runtime, host: ObjectRef) {
    // Ω.5.P62.E1: Number.prototype.valueOf returns [[NumberData]] for
    // Number-exotic wrapper objects (modeled via __primitive__ slot).
    register_intrinsic_method(rt, host, "valueOf", 0, |rt, _args| {
        let this = rt.current_this();
        let t = rt.unwrap_primitive(&this);
        match t {
            Value::Number(n) => Ok(Value::Number(n)),
            _ => Err(RuntimeError::TypeError("Number.prototype.valueOf: this is not a Number".into())),
        }
    });
    register_intrinsic_method(rt, host, "toString", 0, |rt, args| {
        // Ω.5.P62.E19: ThisNumberValue per §21.1.3 — receiver must be a
        // Number primitive or Number-wrapper exotic ([[NumberData]]).
        let this = rt.current_this();
        let unwrapped = rt.unwrap_primitive(&this);
        let n = match unwrapped {
            Value::Number(n) => n,
            _ => return Err(RuntimeError::TypeError(
                "Number.prototype.toString: this is not a Number".into())),
        };
        // Radix: undefined → 10; else ToInteger and validate 2..=36 or throw RangeError.
        let radix = match args.first().cloned() {
            None | Some(Value::Undefined) => 10,
            Some(v) => {
                let n = rt.coerce_to_number(&v)? as i32;
                if n < 2 || n > 36 {
                    return Err(RuntimeError::RangeError(
                        "toString() radix must be between 2 and 36".into()));
                }
                n
            }
        };
        if radix == 10 {
            Ok(Value::String(Rc::new(abstract_ops::number_to_string(n))))
        } else if (2..=36).contains(&radix) && n.is_finite() && n.fract() == 0.0 {
            // Integer-radix only — fractional radix conversion is rare.
            let mut x = n as i64;
            if x == 0 { return Ok(Value::String(Rc::new("0".into()))); }
            let neg = x < 0;
            if neg { x = -x; }
            let mut digits = Vec::new();
            while x > 0 {
                let d = (x % radix as i64) as u32;
                let c = if d < 10 { (b'0' + d as u8) as char } else { (b'a' + (d - 10) as u8) as char };
                digits.push(c);
                x /= radix as i64;
            }
            if neg { digits.push('-'); }
            digits.reverse();
            Ok(Value::String(Rc::new(digits.into_iter().collect())))
        } else {
            Ok(Value::String(Rc::new(abstract_ops::number_to_string(n))))
        }
    });
    register_intrinsic_method(rt, host, "toFixed", 1, |rt, args| {
        // ThisNumberValue brand + RangeError on out-of-range digits.
        let this = rt.current_this();
        let n = match rt.unwrap_primitive(&this) {
            Value::Number(n) => n,
            _ => return Err(RuntimeError::TypeError(
                "Number.prototype.toFixed: this is not a Number".into())),
        };
        let digits_n = match args.first().cloned() {
            None | Some(Value::Undefined) => 0.0,
            Some(v) => rt.coerce_to_number(&v)?,
        };
        if digits_n.is_nan() || digits_n < 0.0 || digits_n > 100.0 {
            return Err(RuntimeError::RangeError(
                "toFixed() digits argument must be between 0 and 100".into()));
        }
        let digits = digits_n as usize;
        if n.is_nan() { return Ok(Value::String(Rc::new("NaN".into()))); }
        if !n.is_finite() {
            return Ok(Value::String(Rc::new(if n > 0.0 { "Infinity".into() } else { "-Infinity".into() })));
        }
        Ok(Value::String(Rc::new(format!("{:.*}", digits, n))))
    });
    // Ω.5.P61.E10: toExponential, toPrecision, toLocaleString per
    // ECMA §21.1.3.
    register_intrinsic_method(rt, host, "toExponential", 1, |rt, args| {
        // ThisNumberValue brand + RangeError on digits out of range.
        let this = rt.current_this();
        let n = match rt.unwrap_primitive(&this) {
            Value::Number(n) => n,
            _ => return Err(RuntimeError::TypeError(
                "Number.prototype.toExponential: this is not a Number".into())),
        };
        let digits = match args.first().cloned() {
            None | Some(Value::Undefined) => None,
            Some(v) => {
                let dn = rt.coerce_to_number(&v)?;
                if dn.is_nan() || dn < 0.0 || dn > 100.0 {
                    return Err(RuntimeError::RangeError(
                        "toExponential() digits argument must be between 0 and 100".into()));
                }
                Some(dn as usize)
            }
        };
        let s = match digits {
            Some(d) => format!("{:.*e}", d, n),
            None => format!("{:e}", n),
        };
        // Rust uses "1e0" form; JS uses "1e+0" — patch the e-sign.
        let s = s.replace("e0", "e+0").replace("e1", "e+1").replace("e2", "e+2")
                 .replace("e3", "e+3").replace("e4", "e+4").replace("e5", "e+5")
                 .replace("e6", "e+6").replace("e7", "e+7").replace("e8", "e+8")
                 .replace("e9", "e+9");
        Ok(Value::String(Rc::new(s)))
    });
    register_intrinsic_method(rt, host, "toPrecision", 1, |rt, args| {
        // ThisNumberValue brand + RangeError per §21.1.3.5.
        let this = rt.current_this();
        let n = match rt.unwrap_primitive(&this) {
            Value::Number(n) => n,
            _ => return Err(RuntimeError::TypeError(
                "Number.prototype.toPrecision: this is not a Number".into())),
        };
        match args.first().cloned() {
            None | Some(Value::Undefined) => Ok(Value::String(Rc::new(
                crate::abstract_ops::number_to_string(n)))),
            Some(v) => {
                let pn = rt.coerce_to_number(&v)?;
                if pn.is_nan() || pn < 1.0 || pn > 100.0 {
                    return Err(RuntimeError::RangeError(
                        "toPrecision() argument must be between 1 and 100".into()));
                }
                let p = pn as usize;
                if n.is_nan() { return Ok(Value::String(Rc::new("NaN".into()))); }
                if !n.is_finite() {
                    return Ok(Value::String(Rc::new(
                        if n > 0.0 { "Infinity".into() } else { "-Infinity".into() })));
                }
                Ok(Value::String(Rc::new(format!("{:.*}", p.saturating_sub(1), n))))
            }
        }
    });
    register_intrinsic_method(rt, host, "toLocaleString", 0, |rt, _args| {
        // ThisNumberValue brand.
        let this = rt.current_this();
        let n = match rt.unwrap_primitive(&this) {
            Value::Number(n) => n,
            _ => return Err(RuntimeError::TypeError(
                "Number.prototype.toLocaleString: this is not a Number".into())),
        };
        Ok(Value::String(Rc::new(crate::abstract_ops::number_to_string(n))))
    });
    // Ω.5.P62.E19: removed duplicate valueOf install (line 1785's
    // brand-checked + __primitive__-unwrapping version is the canonical).
}

// ──────────────── helpers ────────────────

fn arg_string(args: &[Value], i: usize) -> String {
    args.get(i).map(|v| abstract_ops::to_string(v).as_str().to_string()).unwrap_or_default()
}

fn register_method<F>(rt: &mut Runtime, host: ObjectRef, name: &str, f: F)
where F: Fn(&mut Runtime, &[Value]) -> Result<Value, RuntimeError> + 'static {
    let native: NativeFn = Rc::new(f);
    let mut properties = indexmap::IndexMap::new();
    crate::value::install_function_meta_props(&mut properties, name, 0.0);
    let fn_obj = Object {
        proto: None, // function_prototype not yet installed when called from install_prototypes
        extensible: true,
        properties,
        internal_kind: InternalKind::Function(FunctionInternals { name: name.to_string(), length: 0, native, is_constructor: true }),
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
            "Array.prototype method called on null or undefined".into())),
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
            if let Some(Value::Object(bid)) = rt.globals.get("Boolean").cloned() {
                if let Value::Object(p) = rt.object_get(bid, "prototype") {
                    o.proto = Some(p);
                }
            }
            Ok(rt.alloc_object(o))
        }
        Value::Number(n) => {
            let mut o = Object::new_ordinary();
            o.set_own_internal("__primitive".into(), Value::Number(n));
            if let Some(p) = rt.number_prototype { o.proto = Some(p); }
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
            if let Some(p) = rt.string_prototype { o.proto = Some(p); }
            Ok(rt.alloc_object(o))
        }
        Value::BigInt(_) | Value::Symbol(_) => Err(RuntimeError::TypeError(
            "Array.prototype method called on BigInt/Symbol".into())),
    }
}

/// Ω.5.P61.E5: prototype-local intrinsic-method installer. Same semantics
/// as the one in intrinsics.rs but kept here to avoid module-cycle issues
/// with the prototype.rs install paths that pre-date intrinsics.rs's
/// availability. Sets length, marks non-constructor, installs non-enum.
fn register_intrinsic_method<F>(rt: &mut Runtime, host: ObjectRef, name: &str, length: u32, f: F)
where F: Fn(&mut Runtime, &[Value]) -> Result<Value, RuntimeError> + 'static {
    let native: NativeFn = Rc::new(f);
    let mut properties = indexmap::IndexMap::new();
    crate::value::install_function_meta_props(&mut properties, name, length as f64);
    let fn_obj = Object {
        proto: None,
        extensible: true,
        properties,
        internal_kind: InternalKind::Function(FunctionInternals {
            name: name.to_string(), length, native, is_constructor: false,
        }),
    };
    let fn_id = rt.alloc_object(fn_obj);
    rt.obj_mut(host).properties.insert(name.to_string(), crate::value::PropertyDescriptor {
        value: Value::Object(fn_id),
        writable: true, enumerable: false, configurable: true,
        getter: None, setter: None,
    });
}
