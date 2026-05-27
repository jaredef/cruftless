// F-category: collection iteration order.

function canon(v) {
  return JSON.stringify(v, (_k, v) => {
    if (v && typeof v === "object" && !Array.isArray(v)) {
      const out = {}; for (const k of Object.keys(v).sort()) out[k] = v[k]; return out;
    }
    return v;
  });
}

const result = {};

{
  const m = new Map();
  m.set("c", 3);
  m.set("a", 1);
  m.set("b", 2);
  result.map_keys_order = Array.from(m.keys());
  result.map_values_order = Array.from(m.values());
  result.map_entries_order = Array.from(m.entries());
  const forEachLog = [];
  m.forEach((v, k) => forEachLog.push([k, v]));
  result.map_forEach_order = forEachLog;
}

{
  const s = new Set();
  s.add("x");
  s.add("z");
  s.add("y");
  result.set_values_order = Array.from(s.values());
  result.set_keys_order = Array.from(s.keys());
  result.set_entries_order = Array.from(s.entries());
  const forEachLog = [];
  s.forEach(v => forEachLog.push(v));
  result.set_forEach_order = forEachLog;
}

{
  const m = new Map();
  m.set("a", 1);
  m.set("b", 2);
  m.set("c", 3);
  m.delete("b");
  m.set("b", 20);
  result.map_delete_readd = Array.from(m.entries());
}

{
  const s = new Set();
  s.add(1);
  s.add(2);
  s.add(3);
  s.delete(2);
  s.add(2);
  result.set_delete_readd = Array.from(s);
}

{
  const m = new Map();
  m.set("a", 1);
  m.set("b", 2);
  result.map_size = m.size;
  result.map_has = { has_a: m.has("a"), has_c: m.has("c") };
  result.map_get = { get_a: m.get("a"), get_c: m.get("c") };
  m.delete("a");
  result.map_after_delete = { size: m.size, keys: Array.from(m.keys()) };
}

{
  const s = new Set([10, 20, 30]);
  result.set_size = s.size;
  result.set_has = { has_10: s.has(10), has_40: s.has(40) };
  s.delete(20);
  result.set_after_delete = { size: s.size, values: Array.from(s) };
}

{
  const m = new Map();
  m.set("a", 1);
  m.set("b", 2);
  m.set("c", 3);
  m.clear();
  result.map_clear = { size: m.size, keys: Array.from(m.keys()) };
}

{
  const s = new Set([1, 2, 3]);
  s.clear();
  result.set_clear = { size: s.size, values: Array.from(s) };
}

{
  const m = new Map();
  m.set(1, "number");
  m.set("1", "string");
  m.set(true, "bool");
  m.set(null, "null");
  m.set(undefined, "undef");
  result.map_key_types = {
    size: m.size,
    get_1: m.get(1),
    get_str1: m.get("1"),
    get_true: m.get(true),
    get_null: m.get(null),
    get_undef: m.get(undefined),
  };
}

try {
  if (typeof Map.groupBy === "function") {
    const items = [
      { name: "a", type: "x" },
      { name: "b", type: "y" },
      { name: "c", type: "x" },
      { name: "d", type: "y" },
      { name: "e", type: "z" },
    ];
    const grouped = Map.groupBy(items, item => item.type);
    result.map_groupBy = {
      available: true,
      is_map: grouped instanceof Map,
      keys: Array.from(grouped.keys()),
      x_count: grouped.get("x").length,
      y_count: grouped.get("y").length,
      z_count: grouped.get("z").length,
      x_names: grouped.get("x").map(i => i.name),
    };
  } else {
    result.map_groupBy = { available: false };
  }
} catch (e) {
  result.map_groupBy = { available: "error", error: e.message };
}

try {
  if (typeof Set.prototype.union === "function") {
    const a = new Set([1, 2, 3]);
    const b = new Set([3, 4, 5]);
    const u = a.union(b);
    result.set_union = {
      available: true,
      values: Array.from(u).sort((x, y) => x - y),
      size: u.size,
      is_set: u instanceof Set,
    };
  } else {
    result.set_union = { available: false };
  }
} catch (e) {
  result.set_union = { available: "error", error: e.message };
}

try {
  if (typeof Set.prototype.intersection === "function") {
    const a = new Set([1, 2, 3, 4]);
    const b = new Set([3, 4, 5, 6]);
    const i = a.intersection(b);
    result.set_intersection = {
      available: true,
      values: Array.from(i).sort((x, y) => x - y),
      size: i.size,
    };
  } else {
    result.set_intersection = { available: false };
  }
} catch (e) {
  result.set_intersection = { available: "error", error: e.message };
}

try {
  if (typeof Set.prototype.difference === "function") {
    const a = new Set([1, 2, 3, 4]);
    const b = new Set([3, 4, 5, 6]);
    const d = a.difference(b);
    result.set_difference = {
      available: true,
      values: Array.from(d).sort((x, y) => x - y),
      size: d.size,
    };
  } else {
    result.set_difference = { available: false };
  }
} catch (e) {
  result.set_difference = { available: "error", error: e.message };
}

try {
  if (typeof Set.prototype.symmetricDifference === "function") {
    const a = new Set([1, 2, 3]);
    const b = new Set([2, 3, 4]);
    const sd = a.symmetricDifference(b);
    result.set_symmetricDifference = {
      available: true,
      values: Array.from(sd).sort((x, y) => x - y),
      size: sd.size,
    };
  } else {
    result.set_symmetricDifference = { available: false };
  }
} catch (e) {
  result.set_symmetricDifference = { available: "error", error: e.message };
}

try {
  if (typeof Set.prototype.isSubsetOf === "function") {
    const a = new Set([1, 2]);
    const b = new Set([1, 2, 3, 4]);
    const c = new Set([2, 3]);
    result.set_isSubsetOf = {
      available: true,
      subset: a.isSubsetOf(b),
      not_subset: a.isSubsetOf(c),
    };
  } else {
    result.set_isSubsetOf = { available: false };
  }
} catch (e) {
  result.set_isSubsetOf = { available: "error", error: e.message };
}

{
  const obj1 = { id: 1 };
  const obj2 = { id: 2 };
  const wm = new WeakMap();
  wm.set(obj1, "val1");
  wm.set(obj2, "val2");
  result.weakmap = {
    has_obj1: wm.has(obj1),
    get_obj1: wm.get(obj1),
    has_obj2: wm.has(obj2),
    get_missing: wm.get({}),
    has_missing: wm.has({}),
  };
  wm.delete(obj1);
  result.weakmap_after_delete = {
    has_obj1: wm.has(obj1),
    has_obj2: wm.has(obj2),
  };
}

{
  const obj1 = { id: 1 };
  const obj2 = { id: 2 };
  const ws = new WeakSet();
  ws.add(obj1);
  ws.add(obj2);
  result.weakset = {
    has_obj1: ws.has(obj1),
    has_obj2: ws.has(obj2),
    has_missing: ws.has({}),
  };
  ws.delete(obj1);
  result.weakset_after_delete = {
    has_obj1: ws.has(obj1),
    has_obj2: ws.has(obj2),
  };
}

{
  const m = new Map([["a", 1], ["b", 2], ["c", 3]]);
  const iter = m[Symbol.iterator]();
  result.map_symbol_iterator = {
    first: iter.next(),
    second: iter.next(),
    third: iter.next(),
    done: iter.next(),
  };
}

{
  const s = new Set([10, 20, 30]);
  const iter = s[Symbol.iterator]();
  result.set_symbol_iterator = {
    first: iter.next(),
    second: iter.next(),
    third: iter.next(),
    done: iter.next(),
  };
}

console.log(canon(result));
