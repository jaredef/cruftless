//! Tier-Ω.5.c Stage 2 — iterator protocol helpers.
//!
//! v1 representation: a Symbol.iterator value is the well-known string
//! `"@@iterator"`. The `Symbol` global is an ordinary object whose
//! `iterator` slot holds that string, so JS code `obj[Symbol.iterator]`
//! evaluates to property-key `"@@iterator"`. The iterable protocols
//! (Array.prototype, String.prototype, etc.) register a method under
//! that key.
//!
//! String iteration walks the source by Unicode scalar values (Rust
//! `char` iteration) rather than UTF-16 code units. This is a deliberate
//! v1 deviation from ECMA-262 §22.1.5.1 which iterates by code points
//! but yields surrogate pairs as single elements only on the BMP path.
//! UTF-8/char iteration matches consumer expectations for ASCII +
//! single-codepoint inputs which cover the corpus.

use crate::interp::{Runtime, RuntimeError};
use crate::value::{
    FunctionInternals, InternalKind, NativeFn, Object, ObjectRef, PropertyDescriptor, Value,
};
use std::collections::HashMap;
use std::rc::Rc;

/// Make an Array iterator object — `{ next: () => {value, done}, __arr, __i }`.
/// The iterator carries its source array id and a current-index cursor
/// stored as engine-internal sentinels `__arr` / `__i` (v1 — a real engine
/// would intern in internal slots).
///
/// ESNE-EXT 3: all installed props are non-enumerable. CLAUDE.md's source-
/// identifier coordinate convention requires `__X` sentinels to be hidden;
/// `next`/`@@iterator`/`@@toStringTag` are spec-non-enumerable on built-in
/// instances. Pre-fix every install used rt.object_set's default
/// {w:t,e:t,c:t}, leaking 5 own keys per iterator instance.
pub fn make_array_iterator(rt: &mut Runtime, src: ObjectRef) -> ObjectRef {
    let iter = rt.alloc_object(Object::new_ordinary());
    rt.set_engine_sentinel(iter, "__arr", Value::Object(src));
    rt.set_engine_sentinel(iter, "__i", Value::Number(0.0));
    // §23.1.5.2 %ArrayIteratorPrototype%[@@toStringTag] = "Array Iterator".
    rt.set_engine_sentinel(iter, "@@toStringTag", Value::String(Rc::new("Array Iterator".into())));
    // §23.1.5.2.2 the iterator IS itself iterable — [@@iterator]() returns this.
    install_self_returning_iterator(rt, iter);
    install_next(rt, iter, |rt, _args| {
        let it = match rt.current_this() {
            Value::Object(id) => id,
            _ => return Err(RuntimeError::TypeError("array iterator next: this is not an iterator".into())),
        };
        let src_id = match rt.object_get(it, "__arr") {
            Value::Object(id) => id,
            _ => return Ok(iter_result_done(rt)),
        };
        let i = match rt.object_get(it, "__i") {
            Value::Number(n) => n as usize,
            _ => 0,
        };
        let len = rt.array_length(src_id);
        if i >= len {
            return Ok(iter_result_done(rt));
        }
        let v = rt.object_get(src_id, &i.to_string());
        rt.object_set(it, "__i".into(), Value::Number((i + 1) as f64));
        Ok(iter_result_value(rt, v))
    });
    iter
}

/// Make a String iterator. Pre-collects the chars into a Vec stored on
/// the iterator's _chars property (as an Array of single-character
/// strings) for simplicity. _i tracks the cursor.
pub fn make_string_iterator(rt: &mut Runtime, s: String) -> ObjectRef {
    let chars: Vec<char> = s.chars().collect();
    let arr = rt.alloc_object(Object::new_array());
    for (i, c) in chars.iter().enumerate() {
        rt.object_set(arr, i.to_string(), Value::String(Rc::new(c.to_string())));
    }
    rt.object_set(arr, "length".into(), Value::Number(chars.len() as f64));
    let it = make_array_iterator(rt, arr);
    // String iterator's @@toStringTag overrides Array Iterator label.
    rt.set_engine_sentinel(it, "@@toStringTag", Value::String(Rc::new("String Iterator".into())));
    it
}

fn install_self_returning_iterator(rt: &mut Runtime, host: ObjectRef) {
    let native: NativeFn = Rc::new(|rt, _args| Ok(rt.current_this()));
    let mut properties = indexmap::IndexMap::new();
    crate::value::install_function_meta_props(&mut properties, "[Symbol.iterator]", 0.0);
    let fn_obj = Object {
        proto: None,
        extensible: true,
        properties,
        internal_kind: InternalKind::Function(FunctionInternals { name: "[Symbol.iterator]".into(), length: 0, native, is_constructor: false }),
    
        ..Default::default()
    };
    let fn_id = rt.alloc_object(fn_obj);
    // ESNE-EXT 3: install @@iterator non-enumerable per §23.1.5.2.2's
    // prototype-method placement (we install on instance pending real
    // ArrayIteratorPrototype; non-enumerable matches built-in convention).
    rt.set_engine_sentinel(host, "@@iterator", Value::Object(fn_id));
}

fn install_next<F>(rt: &mut Runtime, host: ObjectRef, f: F)
where F: Fn(&mut Runtime, &[Value]) -> Result<Value, RuntimeError> + 'static {
    let native: NativeFn = Rc::new(f);
    let mut properties = indexmap::IndexMap::new();
    crate::value::install_function_meta_props(&mut properties, "next", 0.0);
    let fn_obj = Object {
        proto: None,
        extensible: true,
        properties,
        internal_kind: InternalKind::Function(FunctionInternals { name: "next".into(), length: 0, native, is_constructor: true }),

        ..Default::default()
    };
    let fn_id = rt.alloc_object(fn_obj);
    // ESNE-EXT 3: next() is a prototype method on real spec; hide on instance.
    rt.set_engine_sentinel(host, "next", Value::Object(fn_id));
}

/// Build `{ value, done: false }`.
pub fn iter_result_value(rt: &mut Runtime, v: Value) -> Value {
    let id = rt.alloc_object(Object::new_ordinary());
    rt.obj_mut(id).dict_mut().insert("value".into(), PropertyDescriptor {
        value: v, writable: true, enumerable: true, configurable: true, getter: None, setter: None,
    });
    rt.obj_mut(id).dict_mut().insert("done".into(), PropertyDescriptor {
        value: Value::Boolean(false), writable: true, enumerable: true, configurable: true, getter: None, setter: None,
    });
    Value::Object(id)
}

/// Build `{ value: undefined, done: true }`.
pub fn iter_result_done(rt: &mut Runtime) -> Value {
    let id = rt.alloc_object(Object::new_ordinary());
    rt.obj_mut(id).dict_mut().insert("value".into(), PropertyDescriptor {
        value: Value::Undefined, writable: true, enumerable: true, configurable: true, getter: None, setter: None,
    });
    rt.obj_mut(id).dict_mut().insert("done".into(), PropertyDescriptor {
        value: Value::Boolean(true), writable: true, enumerable: true, configurable: true, getter: None, setter: None,
    });
    Value::Object(id)
}
