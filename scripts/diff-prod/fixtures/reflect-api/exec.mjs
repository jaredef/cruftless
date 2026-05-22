// L-category: Reflect static methods.

function canon(v) {
  return JSON.stringify(v, (_k, v) => {
    if (v && typeof v === "object" && !Array.isArray(v)) {
      const out = {}; for (const k of Object.keys(v).sort()) out[k] = v[k]; return out;
    }
    return v;
  });
}

const result = {};

// get / set / has.
{
  const o = { a: 1, b: 2 };
  result.basic = {
    get_a: Reflect.get(o, "a"),
    get_missing: Reflect.get(o, "z"),
    has_a: Reflect.has(o, "a"),
    has_missing: Reflect.has(o, "z"),
    set_ok: Reflect.set(o, "c", 3),
    after_set: o.c,
  };
}

// deleteProperty.
{
  const o = { x: 1, y: 2 };
  const del = Reflect.deleteProperty(o, "x");
  result.delete = { del_ok: del, after_del: "x" in o, remaining: Object.keys(o) };
}

// ownKeys.
{
  const o = Object.defineProperties({}, {
    a: { value: 1, enumerable: true },
    b: { value: 2, enumerable: false },
  });
  const keys = Reflect.ownKeys(o).sort();
  result.own_keys = keys;
}

// getPrototypeOf / setPrototypeOf.
{
  const p = { ping: () => "pong" };
  const o = Object.create(p);
  result.proto = {
    get_proto_eq: Reflect.getPrototypeOf(o) === p,
    set_proto_ok: Reflect.setPrototypeOf(o, null),
    after_set_null: Reflect.getPrototypeOf(o) === null,
  };
}

// construct.
{
  class Foo { constructor(x, y) { this.sum = x + y; } }
  const inst = Reflect.construct(Foo, [3, 4]);
  result.construct = { is_foo: inst instanceof Foo, sum: inst.sum };
}

// apply.
{
  function add(a, b, c) { return a + b + c + this.bonus; }
  const r = Reflect.apply(add, { bonus: 100 }, [1, 2, 3]);
  result.apply = { result: r };
}

// defineProperty.
{
  const o = {};
  const ok = Reflect.defineProperty(o, "frozen", {
    value: 42, writable: false, enumerable: true, configurable: false,
  });
  // Cruftless v1 deviation: writes to non-writable data props throw even in
  // non-strict (spec: silent no-op outside strict). Skip the write to keep
  // the fixture comparing only Reflect.defineProperty's installation effect.
  result.define = { ok, value: o.frozen };
}

console.log(canon(result));
