// F-category: abstract relational comparison (ECMA-262 §7.2.13).

function canon(v) {
  return JSON.stringify(v, (_k, v) => {
    if (v && typeof v === "object" && !Array.isArray(v)) {
      const out = {}; for (const k of Object.keys(v).sort()) out[k] = v[k]; return out;
    }
    return v;
  });
}

const result = {};

// Numeric comparison.
{
  result.numeric = {
    basic: 1 < 2,
    equal: 1 < 1,
    negative: -1 < 0,
    float: 1.5 < 2.5,
    zero: 0 < -0,
    neg_zero: -0 < 0,
  };
}

// NaN comparisons: always false.
{
  result.nan = {
    lt: NaN < 1,
    gt: NaN > 1,
    le: NaN <= 1,
    ge: NaN >= 1,
    lt_nan: 1 < NaN,
    nan_nan: NaN < NaN,
    nan_le_nan: NaN <= NaN,
  };
}

// Infinity.
{
  result.infinity = {
    inf_gt_num: Infinity > 999999,
    neg_inf_lt_num: -Infinity < -999999,
    inf_lt_inf: Infinity < Infinity,
    inf_le_inf: Infinity <= Infinity,
    neg_inf_lt_inf: -Infinity < Infinity,
  };
}

// String comparison (lexicographic by code unit).
{
  result.string = {
    abc_lt_abd: "abc" < "abd",
    abc_lt_b: "abc" < "b",
    a_lt_b: "a" < "b",
    empty_lt_a: "" < "a",
    a_lt_aa: "a" < "aa",
    upper_lt_lower: "A" < "a",
    nine_lt_A: "9" < "A",
  };
}

// Mixed types: both coerced to number (unless both strings).
{
  result.mixed = {
    str_num: "2" < 10,
    num_str: 2 < "10",
    bool_num: true < 2,
    false_lt_1: false < 1,
    null_lt_1: null < 1,
    null_ge_0: null >= 0,
    undef_lt_0: undefined < 0,
    undef_ge_0: undefined >= 0,
  };
}

// Object ToPrimitive for comparison.
{
  const obj = { valueOf() { return 5; } };
  result.obj_compare = {
    lt: obj < 10,
    gt: obj > 3,
    le: obj <= 5,
    ge: obj >= 5,
  };
}

// <= and >= are NOT just !(>) and !(<) due to NaN.
{
  result.le_ge_nan = {
    not_lt_is_ge_for_nan: !(NaN < 1) !== (NaN >= 1),
    both_false: { lt: NaN < 1, ge: NaN >= 1 },
  };
}

// BigInt relational.
{
  result.bigint = {
    bigint_lt_bigint: 1n < 2n,
    bigint_lt_num: 1n < 2,
    num_lt_bigint: 1 < 2n,
    bigint_le_num: 1n <= 1,
    equal_mixed: 1n >= 1,
  };
}

// String numeric comparison (NOT numeric, lexicographic).
{
  result.string_numeric = {
    nine_gt_ten: "9" > "10",
    two_gt_ten: "2" > "10",
    hundred_lt_two: "100" < "2",
  };
}

console.log(canon(result));
