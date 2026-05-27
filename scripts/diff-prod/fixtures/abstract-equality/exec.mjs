// F-category: abstract equality == algorithm (ECMA-262 §7.2.14).

function canon(v) {
  return JSON.stringify(v, (_k, v) => {
    if (v && typeof v === "object" && !Array.isArray(v)) {
      const out = {}; for (const k of Object.keys(v).sort()) out[k] = v[k]; return out;
    }
    return v;
  });
}

const result = {};

// null == undefined (and vice versa).
{
  result.null_undef = {
    null_eq_undef: null == undefined,
    undef_eq_null: undefined == null,
    null_eq_null: null == null,
    undef_eq_undef: undefined == undefined,
    null_ne_false: null != false,
    null_ne_zero: null != 0,
    null_ne_empty: null != "",
    undef_ne_false: undefined != false,
    undef_ne_zero: undefined != 0,
  };
}

// Number == String: string coerced to number.
{
  result.num_str = {
    num_str: 1 == "1",
    num_str_float: 1.5 == "1.5",
    zero_empty: 0 == "",
    zero_str_zero: 0 == "0",
    nan_str: NaN == "NaN",
    inf_str: Infinity == "Infinity",
  };
}

// Boolean == anything: boolean coerced to number first.
{
  result.bool_coerce = {
    true_eq_1: true == 1,
    false_eq_0: false == 0,
    true_eq_str1: true == "1",
    false_eq_str0: false == "0",
    true_ne_2: true != 2,
    false_eq_empty: false == "",
    true_ne_str_true: true != "true",
    false_ne_null: false != null,
    false_ne_undef: false != undefined,
  };
}

// Object == primitive: object coerced via ToPrimitive.
{
  result.obj_prim = {
    arr_zero: [0] == 0,
    arr_str: [1] == "1",
    arr_empty: [] == "",
    arr_false: [] == false,
    arr_zero_str: [0] == false,
    obj_nan: ({}) == NaN,
    obj_str: ({ toString() { return "42"; } }) == 42,
    obj_valueof: ({ valueOf() { return 1; } }) == 1,
  };
}

// Reflexivity and identity.
{
  result.reflexive = {
    nan_ne_nan: NaN != NaN,
    zero_eq_neg_zero: 0 == -0,
    inf_eq_inf: Infinity == Infinity,
    obj_ne_same_shape: ({}) != ({}),
    arr_ne_same_shape: ([]) != ([]),
  };
}

// Strict equality comparison.
{
  result.strict = {
    num_str: 1 === "1",
    null_undef: null === undefined,
    true_1: true === 1,
    zero_empty: 0 === "",
    nan_nan: NaN === NaN,
    zero_neg_zero: 0 === -0,
    same_obj: (() => { const o = {}; return o === o; })(),
  };
}

// != and !== follow the negation.
{
  result.not_equal = {
    ne: 1 != "1",
    sne: 1 !== "1",
    ne_null: null != undefined,
    sne_null: null !== undefined,
  };
}

// Edge cases with Symbol.
{
  const s = Symbol("test");
  result.symbol = {
    sym_eq_sym: s == s,
    sym_ne_other: s != Symbol("test"),
    sym_ne_str: s != "Symbol(test)",
  };
}

// Edge cases with BigInt.
{
  result.bigint = {
    bigint_eq_num: 1n == 1,
    bigint_ne_str: 1n == "1",
    bigint_ne_bool: 1n == true,
    bigint_ne_2: 1n != 2,
    bigint_strict_ne_num: 1n !== 1,
  };
}

console.log(canon(result));
