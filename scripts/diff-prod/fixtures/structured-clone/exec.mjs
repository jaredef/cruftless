// F-category: structuredClone() coverage.

function canon(v) {
  return JSON.stringify(v, (_k, v) => {
    if (v && typeof v === "object" && !Array.isArray(v)) {
      const out = {}; for (const k of Object.keys(v).sort()) out[k] = v[k]; return out;
    }
    return v;
  });
}

const result = {};

// Plain object.
{
  const a = { x: 1, y: [2, 3], z: { w: "deep" } };
  const c = structuredClone(a);
  result.plain_object = {
    deep_eq: canon(a) === canon(c),
    not_same_ref: a !== c,
    nested_not_same: a.z !== c.z,
  };
}

// Array.
{
  const a = [1, [2, 3], { k: 4 }];
  const c = structuredClone(a);
  result.array = {
    isArray: Array.isArray(c),
    deep_eq: canon(a) === canon(c),
    nested_ref_isolated: a[2] !== c[2],
  };
}

// Date.
{
  const d = new Date("2026-05-22T00:00:00Z");
  const c = structuredClone(d);
  result.date = {
    is_date: c instanceof Date,
    same_time: d.getTime() === c.getTime(),
    not_same_ref: d !== c,
  };
}

// RegExp.
{
  const r = /abc/gi;
  const c = structuredClone(r);
  result.regexp = {
    source: c.source,
    flags: c.flags,
    is_regexp: c instanceof RegExp,
    not_same_ref: r !== c,
  };
}

// Map.
{
  const m = new Map([["a", 1], ["b", 2]]);
  const c = structuredClone(m);
  result.map = {
    is_map: c instanceof Map,
    size: c.size,
    entries: [...c.entries()].sort((a, b) => a[0].localeCompare(b[0])),
    not_same_ref: m !== c,
  };
}

// Set.
{
  const s = new Set([1, 2, 3]);
  const c = structuredClone(s);
  result.set = {
    is_set: c instanceof Set,
    size: c.size,
    values: [...c].sort(),
    not_same_ref: s !== c,
  };
}

// Cycle (a refers back to itself).
{
  const a = { name: "root" };
  a.self = a;
  const c = structuredClone(a);
  result.cycle = {
    self_ref_preserved: c.self === c,
    name: c.name,
    not_same_ref: a !== c,
  };
}

// Shared substructure (two parent keys point at same child).
{
  const shared = { tag: "shared" };
  const a = { left: shared, right: shared };
  const c = structuredClone(a);
  result.shared = {
    identity_preserved: c.left === c.right,
    not_same_as_original: a.left !== c.left,
  };
}

// Functions and Symbols are NOT cloneable.
{
  let threw_fn = false;
  try { structuredClone({ f: () => 1 }); } catch (e) { threw_fn = true; }
  result.function_throws = threw_fn;
}

console.log(canon(result));
