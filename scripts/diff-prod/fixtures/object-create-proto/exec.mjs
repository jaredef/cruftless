// F-category: Object.create and prototype chain surface.

function canon(v) {
  return JSON.stringify(v, (_k, v) => {
    if (v && typeof v === "object" && !Array.isArray(v)) {
      const out = {}; for (const k of Object.keys(v).sort()) out[k] = v[k]; return out;
    }
    return v;
  });
}

const result = {};

// Object.create(proto): inheritance.
{
  const proto = { greet() { return "hello"; } };
  const child = Object.create(proto);
  result.create_inherit = {
    has_greet: typeof child.greet === "function",
    greet_val: child.greet(),
    own: Object.getOwnPropertyNames(child).length,
    proto_match: Object.getPrototypeOf(child) === proto,
  };
}

// Object.create(null): no prototype.
{
  const bare = Object.create(null);
  bare.x = 1;
  result.create_null = {
    proto_null: Object.getPrototypeOf(bare) === null,
    has_toString: typeof bare.toString,
    has_hasOwnProperty: typeof bare.hasOwnProperty,
    x: bare.x,
  };
}

// Object.create(null) does not inherit from Object.prototype.
{
  const bare = Object.create(null);
  try {
    const str = bare.toString();
    result.null_proto_toString = str;
  } catch (e) {
    result.null_proto_toString = e.constructor.name;
  }
}

// Object.create with property descriptors (second arg).
{
  const obj = Object.create(Object.prototype, {
    a: { value: 10, enumerable: true, writable: true, configurable: true },
    b: { value: 20, enumerable: false, writable: false, configurable: false },
  });
  result.create_with_descs = {
    a: obj.a,
    b: obj.b,
    keys: Object.keys(obj),
    all_props: Object.getOwnPropertyNames(obj).sort(),
  };
}

// Object.getPrototypeOf.
{
  result.getProto_array = Object.getPrototypeOf([]) === Array.prototype;
  result.getProto_obj = Object.getPrototypeOf({}) === Object.prototype;
  result.getProto_null = Object.getPrototypeOf(Object.prototype) === null;
}

// Object.setPrototypeOf.
{
  const a = { kind: "a" };
  const b = { kind: "b" };
  const obj = Object.create(a);
  result.set_proto_before = obj.kind;
  Object.setPrototypeOf(obj, b);
  result.set_proto_after = obj.kind;
  result.set_proto_verify = Object.getPrototypeOf(obj) === b;
}

// __proto__ in object literal.
{
  const parent = { from: "parent" };
  const child = { __proto__: parent, own: "child" };
  result.dunder_proto = {
    from: child.from,
    own: child.own,
    proto_match: Object.getPrototypeOf(child) === parent,
  };
}

// __proto__ set to null via literal.
{
  const obj = { __proto__: null, x: 1 };
  result.dunder_null = {
    proto_null: Object.getPrototypeOf(obj) === null,
    x: obj.x,
  };
}

// Reflect.getPrototypeOf.
{
  result.reflect_getProto = Reflect.getPrototypeOf({}) === Object.prototype;
  result.reflect_getProto_arr = Reflect.getPrototypeOf([]) === Array.prototype;
}

// Reflect.setPrototypeOf.
{
  const obj = {};
  const newProto = { tag: "new" };
  const ok = Reflect.setPrototypeOf(obj, newProto);
  result.reflect_setProto = { ok, tag: obj.tag };
}

// Reflect.setPrototypeOf returns false on non-extensible.
{
  const obj = Object.preventExtensions({});
  const ok = Reflect.setPrototypeOf(obj, { x: 1 });
  result.reflect_setProto_nonext = ok;
}

// Object.setPrototypeOf throws on non-extensible.
{
  const obj = {};
  Object.preventExtensions(obj);
  try {
    Object.setPrototypeOf(obj, { y: 2 });
    result.setProto_nonext_throw = "no_throw";
  } catch (e) {
    result.setProto_nonext_throw = e.constructor.name;
  }
}

// Prototype chain walk.
{
  const gp = { level: "gp" };
  const p = Object.create(gp);
  p.level = "p";
  const c = Object.create(p);
  result.chain_walk = {
    c_level: c.level,
    c_own: c.hasOwnProperty("level"),
    p_level: Object.getPrototypeOf(c).level,
    gp_level: Object.getPrototypeOf(Object.getPrototypeOf(c)).level,
  };
}

console.log(canon(result));
