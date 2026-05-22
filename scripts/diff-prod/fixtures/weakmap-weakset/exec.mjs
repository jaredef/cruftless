// F-category: WeakMap + WeakSet basics. cruftless v1 ships WeakMap/
// WeakSet as a thin variant of Map/Set sharing the same prototype
// methods; the strict spec surface (primitive rejection, no iteration,
// no size, distinct prototype identity) is a substantive substrate
// rung. Fixture tests the basic identity-keyed operations that DO
// behave correctly.

function canon(v) {
  return JSON.stringify(v, (_k, v) => {
    if (v && typeof v === "object" && !Array.isArray(v)) {
      const out = {}; for (const k of Object.keys(v).sort()) out[k] = v[k]; return out;
    }
    return v;
  });
}

const result = {};

// WeakMap identity-keyed get/has/delete.
{
  const k1 = { id: 1 };
  const k2 = { id: 2 };
  const wm = new WeakMap();
  wm.set(k1, "one");
  wm.set(k2, "two");
  result.weakmap = {
    get_k1: wm.get(k1),
    get_k2: wm.get(k2),
    has_k1: wm.has(k1),
    has_other: wm.has({ id: 1 }),       // false — identity, not equality
    deleted: wm.delete(k1),
    has_after_delete: wm.has(k1),
  };
}

// WeakMap chained set returns wm.
{
  const wm = new WeakMap();
  const k = {};
  result.weakmap_set_returns_wm = wm.set(k, "x") === wm;
}

// WeakMap construction without args.
{
  const wm = new WeakMap();
  const k = {};
  wm.set(k, 1);
  result.weakmap_no_args = wm.get(k);
}

// WeakSet equivalents.
{
  const a = {}, b = {};
  const ws = new WeakSet();
  ws.add(a);
  result.weakset = {
    has_a: ws.has(a),
    has_other: ws.has({}),              // false — identity
    deleted: ws.delete(a),
    has_after_delete: ws.has(a),
  };
}

// WeakSet chained add returns ws.
{
  const ws = new WeakSet();
  const o = {};
  result.weakset_add_returns_ws = ws.add(o) === ws;
}

// V2 BOUNDARIES (not tested here):
//   - primitive-rejection (set/add of string/number should throw TypeError)
//   - no iteration (Symbol.iterator should be undefined)
//   - no size property
//   - iterable constructor (new WeakMap([[k,v],...]))
//   All deferred; cruftless v1 inherits Map/Set surface verbatim.

console.log(canon(result));
