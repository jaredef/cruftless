// F-category: OrdinaryOwnPropertyKeys ordering (ECMA-262 §10.1.11.1).

function canon(v) {
  return JSON.stringify(v, (_k, v) => {
    if (v && typeof v === "object" && !Array.isArray(v)) {
      const out = {}; for (const k of Object.keys(v).sort()) out[k] = v[k]; return out;
    }
    if (typeof v === "symbol") return v.toString();
    return v;
  });
}

const result = {};

// Basic ordering: integer indices first (ascending), then strings (insertion), then symbols.
{
  const s1 = Symbol("s1");
  const s2 = Symbol("s2");
  const obj = {};
  obj.b = 1;
  obj[2] = 2;
  obj.a = 3;
  obj[0] = 4;
  obj[s1] = 5;
  obj[1] = 6;
  obj[s2] = 7;
  obj.c = 8;

  result.basic = {
    keys: Object.keys(obj),
    own_names: Object.getOwnPropertyNames(obj),
    own_symbols: Object.getOwnPropertySymbols(obj).map(s => s.toString()),
    reflect_ownkeys: Reflect.ownKeys(obj).map(k => typeof k === "symbol" ? k.toString() : k),
  };
}

// Delete and re-add: string key moves to end of insertion order.
{
  const obj = { a: 1, b: 2, c: 3 };
  delete obj.b;
  obj.b = 4;
  result.delete_readd = {
    keys: Object.keys(obj),
  };
}

// Integer index ordering is numeric, not lexicographic.
{
  const obj = {};
  obj["10"] = 1;
  obj["9"] = 2;
  obj["0"] = 3;
  obj["100"] = 4;
  obj["2"] = 5;
  result.integer_numeric_order = {
    keys: Object.keys(obj),
  };
}

// Large integer indices still sort numerically.
{
  const obj = {};
  obj["4294967294"] = "max-array-index";
  obj["4294967295"] = "not-array-index";
  obj["0"] = "zero";
  obj["1"] = "one";
  obj.str = "string";
  result.large_indices = {
    keys: Object.keys(obj),
  };
}

// Non-integer numeric-looking strings are string keys (insertion order).
{
  const obj = {};
  obj["1.5"] = 1;
  obj["0"] = 2;
  obj["-1"] = 3;
  obj["00"] = 4;
  obj["1"] = 5;
  obj["Infinity"] = 6;
  result.non_integer_strings = {
    keys: Object.keys(obj),
  };
}

// Symbol ordering: insertion order among symbols.
{
  const s1 = Symbol("first");
  const s2 = Symbol("second");
  const s3 = Symbol("third");
  const obj = {};
  obj[s3] = 3;
  obj[s1] = 1;
  obj[s2] = 2;
  result.symbol_order = {
    symbols: Object.getOwnPropertySymbols(obj).map(s => s.toString()),
  };
}

// for...in order matches Object.keys for own properties.
{
  const obj = {};
  obj.z = 1;
  obj[2] = 2;
  obj.a = 3;
  obj[0] = 4;
  obj[1] = 5;
  const forin = [];
  for (const k in obj) forin.push(k);
  result.forin_matches_keys = {
    forin,
    keys: Object.keys(obj),
    match: JSON.stringify(forin) === JSON.stringify(Object.keys(obj)),
  };
}

// Object.entries preserves order.
{
  const obj = {};
  obj.b = 2;
  obj[1] = 1;
  obj.a = 0;
  obj[0] = -1;
  result.entries_order = {
    entries: Object.entries(obj),
  };
}

// defineProperty: property added via defineProperty follows same insertion rule.
{
  const obj = {};
  obj.first = 1;
  Object.defineProperty(obj, "second", { value: 2, enumerable: true });
  obj.third = 3;
  Object.defineProperty(obj, 0, { value: 0, enumerable: true });
  result.define_property_order = {
    keys: Object.keys(obj),
  };
}

// Spread preserves key order.
{
  const src = {};
  src[2] = "two";
  src.b = "b";
  src[0] = "zero";
  src.a = "a";
  src[1] = "one";
  const copy = { ...src };
  result.spread_order = {
    keys: Object.keys(copy),
  };
}

// Object.assign preserves source key order.
{
  const src = {};
  src.z = 1;
  src[5] = 2;
  src.a = 3;
  src[1] = 4;
  const dest = Object.assign({}, src);
  result.assign_order = {
    keys: Object.keys(dest),
  };
}

// JSON.stringify key order.
{
  const obj = {};
  obj.c = 3;
  obj[1] = 1;
  obj.a = 0;
  obj[0] = -1;
  obj.b = 2;
  result.json_order = JSON.stringify(obj);
}

console.log(canon(result));
