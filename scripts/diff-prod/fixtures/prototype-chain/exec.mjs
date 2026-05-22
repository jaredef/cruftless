// F-category: prototype-chain mechanics.

function canon(v) {
  return JSON.stringify(v, (_k, v) => {
    if (v && typeof v === "object" && !Array.isArray(v)) {
      const out = {}; for (const k of Object.keys(v).sort()) out[k] = v[k]; return out;
    }
    return v;
  });
}

const result = {};

// Object.create.
{
  const parent = { greet() { return "hello"; } };
  const child = Object.create(parent);
  child.name = "Alice";
  result.create = {
    greet: child.greet(),
    name: child.name,
    proto_is_parent: Object.getPrototypeOf(child) === parent,
    parent_has_greet: "greet" in child,
    own_has_name: Object.prototype.hasOwnProperty.call(child, "name"),
    own_has_greet: Object.prototype.hasOwnProperty.call(child, "greet"),
  };
}

// Object.create(null).
{
  const o = Object.create(null);
  o.x = 1;
  result.null_proto = {
    proto: Object.getPrototypeOf(o),  // null
    has_x: o.x,
    has_toString: typeof o.toString,  // undefined (no Object.prototype)
  };
}

// setPrototypeOf.
{
  const parent = { tag: "parent" };
  const o = {};
  Object.setPrototypeOf(o, parent);
  result.set_proto = {
    tag: o.tag,
    proto_is_parent: Object.getPrototypeOf(o) === parent,
  };
}

// Property lookup walks the chain.
{
  const a = { x: 1 };
  const b = Object.create(a);
  const c = Object.create(b);
  c.y = 3;
  result.chain_walk = { x: c.x, y: c.y, z: c.z };
}

// hasOwnProperty distinguishes own vs inherited.
{
  const parent = { inherited: 1 };
  const child = Object.create(parent);
  child.own = 2;
  result.hasOwn = {
    own_yes: Object.prototype.hasOwnProperty.call(child, "own"),
    inherited_no: Object.prototype.hasOwnProperty.call(child, "inherited"),
    inherited_in: "inherited" in child,
  };
}

// Object.keys = own enumerable only.
{
  const parent = { p_inherited: "i" };
  const child = Object.create(parent);
  child.c_own_enum = "e";
  Object.defineProperty(child, "c_own_nonenum", { value: "ne", enumerable: false });
  result.keys = {
    keys: Object.keys(child).sort(),
    own_names: Object.getOwnPropertyNames(child).sort(),
  };
}

// Object.assign.
{
  const a = { x: 1 };
  const b = { y: 2 };
  const c = { z: 3 };
  const merged = Object.assign({}, a, b, c);
  result.assign = { merged, returns_target: Object.assign(a, b) === a };
}

// Object.freeze + isFrozen.
{
  const o = { x: 1 };
  Object.freeze(o);
  let threw = false;
  try { o.x = 2; } catch (e) { threw = true; }
  result.freeze = {
    isFrozen: Object.isFrozen(o),
    value_unchanged: o.x,
  };
}

// Object.entries + fromEntries roundtrip.
{
  const o = { a: 1, b: 2, c: 3 };
  const e = Object.entries(o).sort((a, b) => a[0].localeCompare(b[0]));
  const back = Object.fromEntries(e);
  result.entries = { entries: e, roundtrip_eq: canon(o) === canon(back) };
}

// Class instances have ctor.prototype = the instance proto.
{
  class C { f() { return 1; } }
  const c = new C();
  result.class_proto = {
    proto_is_ctor_proto: Object.getPrototypeOf(c) === C.prototype,
    has_f: typeof c.f,
    inst: c instanceof C,
  };
}

console.log(canon(result));
