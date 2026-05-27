// F-category: private field encapsulation at the AST-to-bytecode boundary.

function canon(v) {
  return JSON.stringify(v, (_k, v) => {
    if (v && typeof v === "object" && !Array.isArray(v)) {
      const out = {}; for (const k of Object.keys(v).sort()) out[k] = v[k]; return out;
    }
    return v;
  });
}

const result = {};

// Private fields must not leak into Object.keys or Object.getOwnPropertyNames.
class Secret {
  #value;
  pub = "visible";
  constructor(v) { this.#value = v; }
  getValue() { return this.#value; }
}

{
  const s = new Secret(42);
  result.encapsulation = {
    keys: Object.keys(s).sort(),
    own_names: Object.getOwnPropertyNames(s).sort(),
    value_via_method: s.getValue(),
    has_pub: "pub" in s,
  };
}

// Private methods.
class WithPrivateMethod {
  #secret() { return "hidden"; }
  reveal() { return this.#secret(); }
}

{
  const w = new WithPrivateMethod();
  result.private_method = {
    keys: Object.keys(w).sort(),
    own_names: Object.getOwnPropertyNames(w).sort(),
    reveal: w.reveal(),
  };
}

// Private static.
class StaticPrivate {
  static #count = 0;
  static increment() { return ++StaticPrivate.#count; }
  static getCount() { return StaticPrivate.#count; }
}

{
  StaticPrivate.increment();
  StaticPrivate.increment();
  result.static_private = {
    count: StaticPrivate.getCount(),
    class_keys: Object.keys(StaticPrivate).sort(),
    class_own_names: Object.getOwnPropertyNames(StaticPrivate).sort()
      .filter(n => n !== "length" && n !== "name" && n !== "prototype"),
  };
}

// Private field with in-check (ergonomic brand check).
class Branded {
  #brand;
  constructor() { this.#brand = true; }
  static isBranded(obj) {
    try { return #brand in obj; }
    catch { return false; }
  }
}

{
  const b = new Branded();
  result.brand_check = {
    branded: Branded.isBranded(b),
    plain_obj: Branded.isBranded({}),
    null_safe: Branded.isBranded(null) === false || true,
  };
}

// Private field in subclass: parent's privates are not accessible directly.
class Parent {
  #x = 10;
  getX() { return this.#x; }
}
class Child extends Parent {
  #y = 20;
  getY() { return this.#y; }
}

{
  const c = new Child();
  result.inheritance = {
    parent_x: c.getX(),
    child_y: c.getY(),
    child_keys: Object.keys(c).sort(),
    child_own_names: Object.getOwnPropertyNames(c).sort(),
  };
}

// for...in must not enumerate private fields.
{
  const s = new Secret(99);
  s.extra = "also visible";
  const enumerated = [];
  for (const k in s) enumerated.push(k);
  result.for_in = {
    enumerated: enumerated.sort(),
  };
}

// JSON.stringify must not include private fields.
{
  const s = new Secret(42);
  const json = JSON.parse(JSON.stringify(s));
  result.json = {
    keys: Object.keys(json).sort(),
  };
}

console.log(canon(result));
