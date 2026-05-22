// F-category: Number + Math coverage.

function canon(v) {
  return JSON.stringify(v, (_k, v) => {
    if (v && typeof v === "object" && !Array.isArray(v)) {
      const out = {}; for (const k of Object.keys(v).sort()) out[k] = v[k]; return out;
    }
    return v;
  });
}

const result = {};

// parseInt / parseFloat / Number constructor.
result.parsing = {
  parseInt_42: parseInt("42"),
  parseInt_abc: parseInt("abc"),
  parseInt_42_5: parseInt("42.5"),
  parseInt_hex: parseInt("0xff"),
  parseInt_octal_radix: parseInt("17", 8),
  parseInt_bin_radix: parseInt("1010", 2),
  parseInt_leading_ws: parseInt("   42"),
  parseInt_trailing: parseInt("42px"),

  parseFloat_3_14: parseFloat("3.14"),
  parseFloat_neg: parseFloat("-2.5"),
  parseFloat_sci: parseFloat("1e3"),
  parseFloat_inf: parseFloat("Infinity"),
  parseFloat_nan: Number.isNaN(parseFloat("abc")),

  Number_str_42: Number("42"),
  Number_str_empty: Number(""),     // 0 per spec
  Number_str_ws: Number("  "),      // 0
  Number_str_abc: Number.isNaN(Number("abc")),
  Number_null: Number(null),         // 0
  Number_undef: Number.isNaN(Number(undefined)),
  Number_true: Number(true),
  Number_false: Number(false),
};

// NaN / Infinity behavior.
result.specials = {
  nan_eq_nan: NaN === NaN,
  nan_isNaN: Number.isNaN(NaN),
  isFinite_inf: Number.isFinite(Infinity),
  isFinite_neg_inf: Number.isFinite(-Infinity),
  isFinite_42: Number.isFinite(42),
  isInteger_3_0: Number.isInteger(3.0),
  isInteger_3_5: Number.isInteger(3.5),
  isSafeInteger_2_53: Number.isSafeInteger(Math.pow(2, 53)),
  isSafeInteger_2_53_m1: Number.isSafeInteger(Math.pow(2, 53) - 1),
  MAX_SAFE: Number.MAX_SAFE_INTEGER,
  MIN_SAFE: Number.MIN_SAFE_INTEGER,
  MAX_VALUE_finite: Number.isFinite(Number.MAX_VALUE),
};

// Math functions.
result.math = {
  PI_round: Math.round(Math.PI * 1e6) / 1e6,
  E_round: Math.round(Math.E * 1e6) / 1e6,
  sqrt_4: Math.sqrt(4),
  sqrt_2_close: Math.abs(Math.sqrt(2) - 1.41421356) < 1e-5,
  abs_neg: Math.abs(-42),
  floor: Math.floor(3.7),
  ceil: Math.ceil(3.2),
  round_half: Math.round(0.5),     // 1 per JS spec (half-toward-positive)
  round_neg_half: Math.round(-0.5),// 0 per JS spec
  min_args: Math.min(3, 1, 4, 1, 5, 9, 2, 6),
  max_args: Math.max(3, 1, 4, 1, 5, 9, 2, 6),
  min_empty: Math.min(),            // Infinity
  max_empty: Math.max(),            // -Infinity
  pow_2_10: Math.pow(2, 10),
  log_e: Math.log(Math.E),
  log2_8: Math.log2(8),
  log10_1000: Math.log10(1000),
  exp_1_close_e: Math.abs(Math.exp(1) - Math.E) < 1e-10,
  hypot_3_4: Math.hypot(3, 4),
  sign_neg: Math.sign(-7),
  sign_zero: Math.sign(0),
  sign_pos: Math.sign(7),
  trunc_pos: Math.trunc(3.7),
  trunc_neg: Math.trunc(-3.7),
};

// Number method outputs.
result.format = {
  toString_radix: (255).toString(16),
  toString_bin: (10).toString(2),
  toFixed_2: (3.14159).toFixed(2),
  toFixed_0: (3.5).toFixed(0),
  toPrecision_3: (3.14159).toPrecision(3),
  toExponential: (12345).toExponential(2),
};

// BigInt basics.
result.bigint = {
  literal: typeof 100n,
  arith: String(2n ** 10n),
  cmp: 100n > 99n,
};

console.log(canon(result));
