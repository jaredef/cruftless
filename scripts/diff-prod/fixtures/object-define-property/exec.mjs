// F-category: Object.defineProperty surface.

function canon(v) {
  return JSON.stringify(v, (_k, v) => {
    if (v && typeof v === "object" && !Array.isArray(v)) {
      const out = {}; for (const k of Object.keys(v).sort()) out[k] = v[k]; return out;
    }
    return v;
  });
}

const result = {};

// Data descriptor.
{
  const obj = {};
  Object.defineProperty(obj, "x", { value: 42, writable: true, enumerable: true, configurable: true });
  result.data_desc = { x: obj.x };
  const desc = Object.getOwnPropertyDescriptor(obj, "x");
  result.data_desc_shape = {
    value: desc.value,
    writable: desc.writable,
    enumerable: desc.enumerable,
    configurable: desc.configurable,
  };
}

// Accessor descriptor.
{
  const obj = {};
  let backing = 10;
  Object.defineProperty(obj, "y", {
    get() { return backing; },
    set(v) { backing = v; },
    enumerable: true,
    configurable: true,
  });
  result.accessor_get = obj.y;
  obj.y = 20;
  result.accessor_set = obj.y;
  const desc = Object.getOwnPropertyDescriptor(obj, "y");
  result.accessor_desc_shape = {
    has_get: typeof desc.get === "function",
    has_set: typeof desc.set === "function",
    enumerable: desc.enumerable,
    configurable: desc.configurable,
    has_value: "value" in desc,
    has_writable: "writable" in desc,
  };
}

// Transition data -> accessor.
{
  const obj = { z: 1 };
  Object.defineProperty(obj, "z", {
    get() { return 99; },
    configurable: true,
  });
  result.data_to_accessor = obj.z;
  const desc = Object.getOwnPropertyDescriptor(obj, "z");
  result.data_to_accessor_desc = {
    has_get: typeof desc.get === "function",
    has_value: "value" in desc,
  };
}

// Transition accessor -> data.
{
  const obj = {};
  Object.defineProperty(obj, "w", {
    get() { return 5; },
    configurable: true,
  });
  Object.defineProperty(obj, "w", {
    value: 77,
    writable: true,
    configurable: true,
  });
  result.accessor_to_data = obj.w;
  const desc = Object.getOwnPropertyDescriptor(obj, "w");
  result.accessor_to_data_desc = {
    has_get: "get" in desc,
    has_value: "value" in desc,
    value: desc.value,
  };
}

// Non-configurable: cannot redefine.
{
  const obj = {};
  Object.defineProperty(obj, "nc", { value: 1, writable: false, configurable: false });
  try {
    Object.defineProperty(obj, "nc", { value: 2 });
    result.nonconfig_redefine = "no_throw";
  } catch (e) {
    result.nonconfig_redefine = e.constructor.name;
  }
}

// Non-configurable: cannot delete (throws in strict mode).
{
  const obj = {};
  Object.defineProperty(obj, "nd", { value: 1, configurable: false });
  let threw = false;
  try { delete obj.nd; } catch { threw = true; }
  result.nonconfig_delete = { threw, still_has: "nd" in obj, val: obj.nd };
}

// Non-writable: assignment throws in strict mode (.mjs is strict).
{
  const obj = {};
  Object.defineProperty(obj, "nw", { value: 10, writable: false, configurable: false });
  let threw = false;
  try { obj.nw = 20; } catch { threw = true; }
  result.nonwritable_assign = { threw, val: obj.nw };
}

// Object.defineProperties (multiple).
{
  const obj = {};
  Object.defineProperties(obj, {
    a: { value: 1, enumerable: true, configurable: true, writable: true },
    b: { value: 2, enumerable: true, configurable: true, writable: true },
    c: { value: 3, enumerable: false, configurable: true, writable: true },
  });
  result.define_multi = { a: obj.a, b: obj.b, c: obj.c, keys: Object.keys(obj).sort() };
}

// Object.getOwnPropertyDescriptors.
{
  const obj = { m: 1 };
  Object.defineProperty(obj, "n", { value: 2, enumerable: false });
  const descs = Object.getOwnPropertyDescriptors(obj);
  result.all_descs = {
    m_enum: descs.m.enumerable,
    m_val: descs.m.value,
    n_enum: descs.n.enumerable,
    n_val: descs.n.value,
  };
}

// Reflect.defineProperty returns boolean.
{
  const obj = {};
  const ok = Reflect.defineProperty(obj, "rp", { value: 42 });
  result.reflect_define_ok = ok;
  result.reflect_define_val = obj.rp;

  Object.defineProperty(obj, "rp2", { value: 1, configurable: false, writable: false });
  const fail = Reflect.defineProperty(obj, "rp2", { value: 2 });
  result.reflect_define_fail = fail;
}

// Descriptor defaults: missing fields.
{
  const obj = {};
  Object.defineProperty(obj, "def", { value: 5 });
  const desc = Object.getOwnPropertyDescriptor(obj, "def");
  result.defaults = {
    writable: desc.writable,
    enumerable: desc.enumerable,
    configurable: desc.configurable,
  };
}

// defineProperty returns the object.
{
  const obj = {};
  const ret = Object.defineProperty(obj, "ret", { value: 1 });
  result.define_returns_obj = ret === obj;
}

console.log(canon(result));
