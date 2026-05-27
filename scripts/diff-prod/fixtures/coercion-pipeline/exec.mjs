// F-category: type coercion pipeline (ToPrimitive → ToNumber/ToString/ToBoolean).

function canon(v) {
  return JSON.stringify(v, (_k, v) => {
    if (v && typeof v === "object" && !Array.isArray(v)) {
      const out = {}; for (const k of Object.keys(v).sort()) out[k] = v[k]; return out;
    }
    return v;
  });
}

const result = {};

// + operator string concatenation vs numeric addition.
{
  result.plus = {
    num_num: 1 + 2,
    str_str: "a" + "b",
    str_num: "1" + 2,
    num_str: 1 + "2",
    null_num: null + 1,
    undef_num: undefined + 1,
    bool_num: true + 1,
    bool_str: true + "x",
    null_str: null + "x",
    arr_str: [1, 2] + "",
    obj_str: {} + "",
    arr_num: +[],
    arr_one: +[42],
  };
}

// ToNumber.
{
  result.to_number = {
    string_num: Number("42"),
    string_float: Number("3.14"),
    string_hex: Number("0xFF"),
    string_empty: Number(""),
    string_spaces: Number("  42  "),
    string_nan: Number("abc"),
    bool_true: Number(true),
    bool_false: Number(false),
    null: Number(null),
    undefined: Number(undefined),
    empty_array: Number([]),
    single_array: Number([42]),
    multi_array: Number.isNaN(Number([1, 2])),
  };
}

// ToString.
{
  result.to_string = {
    number: String(42),
    float: String(3.14),
    neg_zero: String(-0),
    infinity: String(Infinity),
    neg_infinity: String(-Infinity),
    nan: String(NaN),
    bool_true: String(true),
    bool_false: String(false),
    null: String(null),
    undefined: String(undefined),
    array: String([1, 2, 3]),
    empty_array: String([]),
    nested_array: String([1, [2, [3]]]),
  };
}

// ToBoolean (truthiness).
{
  result.to_boolean = {
    zero: Boolean(0),
    neg_zero: Boolean(-0),
    nan: Boolean(NaN),
    empty_str: Boolean(""),
    null: Boolean(null),
    undefined: Boolean(undefined),
    false: Boolean(false),
    one: Boolean(1),
    neg_one: Boolean(-1),
    str: Boolean("hello"),
    zero_str: Boolean("0"),
    obj: Boolean({}),
    arr: Boolean([]),
    fn: Boolean(function(){}),
  };
}

// Template literal coercion (ToString).
{
  result.template_coerce = {
    number: `${42}`,
    null: `${null}`,
    undefined: `${undefined}`,
    boolean: `${true}`,
    array: `${[1, 2, 3]}`,
    object_plain: `${{}}`,
  };
}

// Object ToPrimitive via valueOf and toString.
{
  const valObj = { valueOf() { return 42; } };
  const strObj = { toString() { return "hello"; } };
  const bothObj = { valueOf() { return 10; }, toString() { return "ten"; } };

  result.obj_coerce = {
    valueof_num: +valObj,
    tostring_str: `${strObj}`,
    both_num: +bothObj,
    both_str: `${bothObj}`,
    both_add: bothObj + "",
  };
}

// Unary + as ToNumber shorthand.
{
  result.unary_plus = {
    string: +"42",
    bool: +true,
    null: +null,
    undefined: +undefined,
    empty: +"",
    whitespace: +"  ",
    hex: +"0xFF",
  };
}

// parseInt vs Number.
{
  result.parse_vs_number = {
    trailing: { parseInt: parseInt("42px"), number: Number("42px") },
    hex: { parseInt: parseInt("0xFF"), number: Number("0xFF") },
    empty: { parseInt: parseInt(""), number: Number("") },
    float: { parseInt: parseInt("3.14"), number: Number("3.14") },
  };
}

console.log(canon(result));
