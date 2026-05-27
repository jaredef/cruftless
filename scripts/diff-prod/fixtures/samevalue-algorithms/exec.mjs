// F-category: SameValue / SameValueZero / strict equality (§7.2.10–§7.2.12).

function canon(v) {
  return JSON.stringify(v, (_k, v) => {
    if (v && typeof v === "object" && !Array.isArray(v)) {
      const out = {}; for (const k of Object.keys(v).sort()) out[k] = v[k]; return out;
    }
    return v;
  });
}

const result = {};

// Object.is (SameValue): NaN === NaN, +0 !== -0.
{
  result.same_value = {
    nan_nan: Object.is(NaN, NaN),
    zero_neg_zero: Object.is(0, -0),
    neg_zero_zero: Object.is(-0, 0),
    zero_zero: Object.is(0, 0),
    one_one: Object.is(1, 1),
    str_str: Object.is("a", "a"),
    null_null: Object.is(null, null),
    undef_undef: Object.is(undefined, undefined),
    null_undef: Object.is(null, undefined),
    inf_inf: Object.is(Infinity, Infinity),
    neg_inf_inf: Object.is(-Infinity, Infinity),
  };
}

// Strict equality (===): NaN !== NaN, +0 === -0.
{
  result.strict_eq = {
    nan_nan: NaN === NaN,
    zero_neg_zero: 0 === -0,
    neg_zero_zero: -0 === 0,
  };
}

// SameValueZero: NaN === NaN, +0 === -0 (used by includes, Map, Set).
// Tested indirectly through Array.prototype.includes and Set.
{
  result.same_value_zero_includes = {
    nan: [NaN, 1, 2].includes(NaN),
    zero: [0, 1, 2].includes(-0),
    neg_zero: [-0, 1, 2].includes(0),
  };
}

// Map uses SameValueZero for key comparison.
{
  const m = new Map();
  m.set(NaN, "nan-val");
  m.set(0, "zero-val");
  m.set(-0, "neg-zero-val");
  result.map_svz = {
    nan_get: m.get(NaN),
    zero_get: m.get(0),
    neg_zero_get: m.get(-0),
    size: m.size,
    zero_and_neg_zero_same_key: m.get(0) === m.get(-0),
  };
}

// Set uses SameValueZero.
{
  const s = new Set();
  s.add(NaN);
  s.add(NaN);
  s.add(0);
  s.add(-0);
  result.set_svz = {
    size: s.size,
    has_nan: s.has(NaN),
    has_zero: s.has(0),
    has_neg_zero: s.has(-0),
  };
}

// indexOf uses strict equality (NaN not found).
{
  result.indexof_strict = {
    nan: [NaN, 1, 2].indexOf(NaN),
    zero: [0, 1, 2].indexOf(-0),
    neg_zero: [-0, 1, 2].indexOf(0),
  };
}

// Object identity: same reference.
{
  const obj = {};
  const arr = [];
  result.identity = {
    same_obj: Object.is(obj, obj),
    diff_obj: Object.is({}, {}),
    same_arr: Object.is(arr, arr),
    diff_arr: Object.is([], []),
    same_fn: Object.is(canon, canon),
  };
}

// Edge: symbols.
{
  const s = Symbol("test");
  result.symbol_sv = {
    same: Object.is(s, s),
    diff: Object.is(Symbol("test"), Symbol("test")),
  };
}

console.log(canon(result));
