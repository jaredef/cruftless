// L-category: Object static methods.

function canon(v) {
  return JSON.stringify(v, (_k, v) => {
    if (v && typeof v === "object" && !Array.isArray(v)) {
      const out = {}; for (const k of Object.keys(v).sort()) out[k] = v[k]; return out;
    }
    return v;
  });
}

const result = {};

// assign.
{
  const target = { a: 1, b: 2 };
  const r = Object.assign(target, { b: 3, c: 4 }, { d: 5 });
  result.assign = { target_eq_r: target === r, value: target };
}

// keys / values / entries.
{
  const o = { c: 3, a: 1, b: 2 };
  result.kve = {
    keys: Object.keys(o).sort(),
    values: Object.values(o).sort(),
    entries_sorted: Object.entries(o).sort((a, b) => a[0].localeCompare(b[0])),
  };
}

// fromEntries.
{
  const o = Object.fromEntries([["a", 1], ["b", 2], ["c", 3]]);
  result.from_entries = o;
}

// freeze / isFrozen.
{
  const o = Object.freeze({ x: 1 });
  result.freeze = {
    is_frozen: Object.isFrozen(o),
    keys_preserved: Object.keys(o),
  };
}

// getOwnPropertyDescriptor.
{
  const o = { a: 1 };
  Object.defineProperty(o, "b", { value: 2, writable: false, enumerable: false, configurable: true });
  const da = Object.getOwnPropertyDescriptor(o, "a");
  const db = Object.getOwnPropertyDescriptor(o, "b");
  result.descriptor = {
    a: { value: da.value, writable: da.writable, enumerable: da.enumerable, configurable: da.configurable },
    b: { value: db.value, writable: db.writable, enumerable: db.enumerable, configurable: db.configurable },
  };
}

// hasOwn.
{
  const o = Object.create({ inherited: true });
  o.own = 1;
  result.has_own = {
    own: Object.hasOwn(o, "own"),
    inherited_no: Object.hasOwn(o, "inherited"),
    missing: Object.hasOwn(o, "missing"),
  };
}

// getOwnPropertyNames vs keys (incl. non-enumerable).
{
  const o = {};
  Object.defineProperty(o, "vis", { value: 1, enumerable: true });
  Object.defineProperty(o, "hidden", { value: 2, enumerable: false });
  result.names_vs_keys = {
    keys: Object.keys(o),
    names: Object.getOwnPropertyNames(o).sort(),
  };
}

console.log(canon(result));
