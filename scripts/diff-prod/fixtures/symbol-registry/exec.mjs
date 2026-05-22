// L-category: Symbol surface.

function canon(v) {
  return JSON.stringify(v, (_k, v) => {
    if (typeof v === "symbol") return "Symbol(" + (v.description ?? "") + ")";
    if (v && typeof v === "object" && !Array.isArray(v)) {
      const out = {}; for (const k of Object.keys(v).sort()) out[k] = v[k]; return out;
    }
    return v;
  });
}

const result = {};

// Description.
{
  const s = Symbol("hello");
  result.desc = { d: s.description, str: s.toString(), typeof: typeof s };
}

// Uniqueness.
{
  const a = Symbol("x");
  const b = Symbol("x");
  result.unique = { eq: a === b, same_desc: a.description === b.description };
}

// Registry: Symbol.for / Symbol.keyFor.
{
  const a = Symbol.for("shared");
  const b = Symbol.for("shared");
  result.registry = {
    same: a === b,
    key_a: Symbol.keyFor(a),
    local_no_key: Symbol.keyFor(Symbol("local")) === undefined,
  };
}

// Well-known: Symbol.iterator (custom).
{
  const obj = {
    [Symbol.iterator]() {
      let i = 0;
      return { next: () => i < 3 ? { value: i++, done: false } : { value: undefined, done: true } };
    },
  };
  result.iterator = { spread: [...obj], sum: [...obj].reduce((a, b) => a + b, 0) };
}

// Symbol.toPrimitive.
{
  const o = {
    [Symbol.toPrimitive](hint) {
      if (hint === "number") return 42;
      if (hint === "string") return "str";
      return "default";
    },
  };
  result.toprim = { num: +o, str: `${o}`, dflt: o + "" };
}

// Symbol-keyed property.
{
  const k = Symbol("key");
  const o = { [k]: "value", regular: "r" };
  result.symbol_keyed = {
    via_sym: o[k],
    own_sym_keys: Object.getOwnPropertySymbols(o).map(s => s.description),
    string_keys: Object.keys(o),
  };
}

console.log(canon(result));
