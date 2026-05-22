// L-category: Math static methods (precision-sensitive).

function canon(v) {
  return JSON.stringify(v, (_k, v) => {
    if (typeof v === "number" && Number.isFinite(v)) return Number(v.toPrecision(12));
    if (v && typeof v === "object" && !Array.isArray(v)) {
      const out = {}; for (const k of Object.keys(v).sort()) out[k] = v[k]; return out;
    }
    return v;
  });
}

const result = {};

// Trig.
result.trig = {
  sin0: Math.sin(0),
  cos0: Math.cos(0),
  tan_pi4: Math.tan(Math.PI / 4),
  asin1: Math.asin(1),
  atan2: Math.atan2(1, 1),
};

// Hyperbolic.
result.hyperbolic = {
  sinh0: Math.sinh(0),
  cosh0: Math.cosh(0),
  tanh1: Math.tanh(1),
  asinh1: Math.asinh(1),
};

// Log family.
result.log = {
  log_e: Math.log(Math.E),
  log2_8: Math.log2(8),
  log10_1000: Math.log10(1000),
  log1p_0: Math.log1p(0),
};

// Exp.
result.exp = {
  exp0: Math.exp(0),
  exp1: Math.exp(1),
  expm1_0: Math.expm1(0),
};

// Hypot / cbrt.
result.misc = {
  hypot_3_4: Math.hypot(3, 4),
  hypot_5_12: Math.hypot(5, 12),
  cbrt_27: Math.cbrt(27),
  cbrt_neg_8: Math.cbrt(-8),
};

// Sign / trunc / fround.
result.basics = {
  sign_neg: Math.sign(-5),
  sign_zero: Math.sign(0),
  sign_pos: Math.sign(7),
  trunc_pos: Math.trunc(3.7),
  trunc_neg: Math.trunc(-3.7),
  fround_pi: Math.fround(Math.PI),
};

// Integer math.
result.imath = {
  clz32_1: Math.clz32(1),
  clz32_full: Math.clz32(0xffffffff),
  imul: Math.imul(3, 4),
  imul_neg: Math.imul(-2, 3),
};

console.log(canon(result));
