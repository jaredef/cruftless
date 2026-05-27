// F-category: computed property key evaluation order.

function canon(v) {
  return JSON.stringify(v, (_k, v) => {
    if (v && typeof v === "object" && !Array.isArray(v)) {
      const out = {}; for (const k of Object.keys(v).sort()) out[k] = v[k]; return out;
    }
    return v;
  });
}

const result = {};

// Object literal: computed keys evaluated left-to-right.
{
  const log = [];
  const key = (n) => { log.push(n); return n; };
  const obj = {
    [key("a")]: 1,
    [key("b")]: 2,
    [key("c")]: 3,
  };
  result.obj_literal_order = { log, keys: Object.keys(obj) };
}

// Object literal: computed key with side effect.
{
  let counter = 0;
  const obj = {
    [counter++]: "first",
    [counter++]: "second",
    [counter++]: "third",
  };
  result.counter_keys = { obj, counter };
}

// Computed method names.
{
  const log = [];
  const key = (n) => { log.push("key:" + n); return n; };
  const obj = {
    [key("greet")]() { return "hello"; },
    [key("farewell")]() { return "bye"; },
  };
  result.method_order = { log, greet: obj.greet(), farewell: obj.farewell() };
}

// Class with computed method names.
{
  const log = [];
  const key = (n) => { log.push(n); return n; };
  class C {
    [key("a")]() { return 1; }
    [key("b")]() { return 2; }
  }
  const c = new C();
  result.class_method_order = { log, a: c.a(), b: c.b() };
}

// Computed property with Symbol keys.
{
  const s1 = Symbol("first");
  const s2 = Symbol("second");
  const obj = { [s1]: 1, [s2]: 2, plain: 3 };
  result.symbol_computed = {
    keys: Object.keys(obj),
    symbols: Object.getOwnPropertySymbols(obj).map(s => s.toString()),
    s1: obj[s1],
    s2: obj[s2],
  };
}

// Computed property that returns different types for ToPropertyKey.
{
  const obj = {
    [1]: "number",
    [true]: "boolean",
    [null]: "null",
    [undefined]: "undefined",
  };
  result.coerced_keys = {
    keys: Object.keys(obj).sort(),
    by_1: obj["1"],
    by_true: obj["true"],
    by_null: obj["null"],
    by_undef: obj["undefined"],
  };
}

// Computed property in destructuring target.
{
  const key = "dynamic";
  const { [key]: val } = { dynamic: 42 };
  result.destructure_computed = val;
}

// Computed property with template literal key.
{
  const prefix = "data";
  const obj = {
    [`${prefix}_1`]: "one",
    [`${prefix}_2`]: "two",
  };
  result.template_keys = Object.keys(obj).sort();
}

// Getter/setter with computed name.
{
  const prop = "value";
  const obj = {
    _inner: 0,
    get [prop]() { return this._inner; },
    set [prop](v) { this._inner = v * 2; },
  };
  obj.value = 5;
  result.computed_accessor = { get: obj.value, inner: obj._inner };
}

// Spread with computed properties.
{
  const a = { x: 1 };
  const key = "y";
  const b = { ...a, [key]: 2, z: 3 };
  result.spread_computed = { keys: Object.keys(b).sort(), b };
}

console.log(canon(result));
