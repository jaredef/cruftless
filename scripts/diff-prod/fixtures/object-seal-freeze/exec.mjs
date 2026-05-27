// F-category: Object.seal / Object.preventExtensions / Object.freeze strict-mode enforcement.

"use strict";

function canon(v) {
  return JSON.stringify(v, (_k, v) => {
    if (v && typeof v === "object" && !Array.isArray(v)) {
      const out = {}; for (const k of Object.keys(v).sort()) out[k] = v[k]; return out;
    }
    return v;
  });
}

const result = {};

// Object.preventExtensions: adding a property in strict mode throws TypeError.
{
  const obj = { a: 1 };
  Object.preventExtensions(obj);
  let threw = false;
  let err_name = null;
  try { obj.b = 2; } catch (e) { threw = true; err_name = e.constructor.name; }
  result.prevent_ext_add = {
    threw,
    err_name,
    is_extensible: Object.isExtensible(obj),
    existing_still_writable: (() => { obj.a = 99; return obj.a; })(),
  };
}

// preventExtensions: delete still works.
{
  const obj = { x: 1, y: 2 };
  Object.preventExtensions(obj);
  delete obj.x;
  result.prevent_ext_delete = {
    keys: Object.keys(obj).sort(),
    has_x: "x" in obj,
  };
}

// Object.seal: configurable becomes false, but writable stays.
{
  const obj = { a: 1, b: 2 };
  Object.seal(obj);
  result.seal_descriptors = {
    is_sealed: Object.isSealed(obj),
    is_extensible: Object.isExtensible(obj),
    a_desc: Object.getOwnPropertyDescriptor(obj, "a"),
  };
}

// Seal: can still write to existing properties.
{
  const obj = { a: 1 };
  Object.seal(obj);
  obj.a = 42;
  result.seal_write = { a: obj.a };
}

// Seal: cannot add new properties (strict mode TypeError).
{
  const obj = { a: 1 };
  Object.seal(obj);
  let threw = false;
  let err_name = null;
  try { obj.b = 2; } catch (e) { threw = true; err_name = e.constructor.name; }
  result.seal_add = { threw, err_name };
}

// Seal: cannot delete properties (strict mode TypeError).
{
  const obj = { a: 1 };
  Object.seal(obj);
  let threw = false;
  let err_name = null;
  try { delete obj.a; } catch (e) { threw = true; err_name = e.constructor.name; }
  result.seal_delete = { threw, err_name, still_has_a: "a" in obj };
}

// Seal: cannot reconfigure (strict mode TypeError).
{
  const obj = { a: 1 };
  Object.seal(obj);
  let threw = false;
  let err_name = null;
  try {
    Object.defineProperty(obj, "a", { enumerable: false });
  } catch (e) { threw = true; err_name = e.constructor.name; }
  result.seal_reconfigure = { threw, err_name };
}

// Object.freeze: cannot write, add, delete, or reconfigure.
{
  const obj = { a: 1 };
  Object.freeze(obj);

  let write_threw = false;
  try { obj.a = 2; } catch { write_threw = true; }

  let add_threw = false;
  try { obj.b = 2; } catch { add_threw = true; }

  let del_threw = false;
  try { delete obj.a; } catch { del_threw = true; }

  result.freeze = {
    is_frozen: Object.isFrozen(obj),
    is_sealed: Object.isSealed(obj),
    is_extensible: Object.isExtensible(obj),
    write_threw,
    add_threw,
    del_threw,
    value_unchanged: obj.a,
  };
}

// Freeze: nested object is NOT deep-frozen.
{
  const obj = { inner: { x: 1 } };
  Object.freeze(obj);
  obj.inner.x = 99;
  result.freeze_shallow = {
    inner_x: obj.inner.x,
    inner_frozen: Object.isFrozen(obj.inner),
  };
}

// Array: freeze prevents push, pop, index assignment.
{
  const arr = [1, 2, 3];
  Object.freeze(arr);

  let push_threw = false;
  try { arr.push(4); } catch { push_threw = true; }

  let idx_threw = false;
  try { arr[0] = 99; } catch { idx_threw = true; }

  result.freeze_array = {
    push_threw,
    idx_threw,
    length: arr.length,
    values: [...arr],
  };
}

console.log(canon(result));
