// F-category: Map/Set construction, iteration, conversion.

function canon(v) {
  return JSON.stringify(v, (_k, v) => {
    if (v && typeof v === "object" && !Array.isArray(v)) {
      const out = {};
      for (const k of Object.keys(v).sort()) out[k] = v[k];
      return out;
    }
    return v;
  });
}

const result = {};

// Map: insertion order preserved on iteration.
{
  const m = new Map();
  m.set("z", 1);
  m.set("a", 2);
  m.set("m", 3);
  result.map_size = m.size;
  result.map_has_a = m.has("a");
  result.map_has_q = m.has("q");
  result.map_get_z = m.get("z");
  result.map_keys = [...m.keys()];
  result.map_values = [...m.values()];
  result.map_entries = [...m.entries()];
  result.map_after_delete = (() => {
    const c = new Map(m);
    c.delete("a");
    return [...c.keys()];
  })();
}

// Set: dedup + iteration.
{
  const s = new Set([1, 2, 2, 3, 3, 3, "x", "x"]);
  result.set_size = s.size;
  result.set_values = [...s];
  result.set_has_2 = s.has(2);
  result.set_has_4 = s.has(4);
}

// Map ↔ Object roundtrip.
{
  const obj = { a: 1, b: 2, c: 3 };
  const m = new Map(Object.entries(obj));
  const back = Object.fromEntries(m);
  result.map_object_roundtrip = canon(obj) === canon(back);
}

// Set ↔ Array roundtrip (dedup).
{
  const a = [1, 1, 2, 3, 2, 4];
  const deduped = [...new Set(a)];
  result.set_array_roundtrip = canon(deduped);
}

// Map with object keys (identity).
{
  const k1 = { id: 1 };
  const k2 = { id: 2 };
  const m = new Map();
  m.set(k1, "first");
  m.set(k2, "second");
  result.map_obj_keys_size = m.size;
  result.map_obj_keys_get_k1 = m.get(k1);
  result.map_obj_keys_get_otherk1 = m.get({ id: 1 });  // different identity → undefined
}

console.log(canon(result));
