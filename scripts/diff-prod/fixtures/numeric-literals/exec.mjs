// F-category: numeric literal parsing (lexer depth).

function canon(v) {
  return JSON.stringify(v, (_k, v) => {
    if (v && typeof v === "object" && !Array.isArray(v)) {
      const out = {}; for (const k of Object.keys(v).sort()) out[k] = v[k]; return out;
    }
    return v;
  });
}

const result = {};

// Binary literals.
{
  result.binary = {
    zero: 0b0,
    one: 0b1,
    ten: 0b1010,
    max_byte: 0b11111111,
    mixed_case: 0B1100,
  };
}

// Octal literals.
{
  result.octal = {
    zero: 0o0,
    eight: 0o10,
    max_byte: 0o377,
    mixed_case: 0O77,
  };
}

// Hex literals.
{
  result.hex = {
    zero: 0x0,
    ff: 0xFF,
    deadbeef: 0xDEADBEEF,
    mixed_case: 0Xff,
  };
}

// Numeric separators.
{
  result.separators = {
    decimal: 1_000_000,
    binary: 0b1010_0001,
    hex: 0xFF_FF,
    octal: 0o77_77,
    float: 1_234.567_89,
  };
}

// Exponent notation.
{
  result.exponent = {
    positive: 1e3,
    negative: 1e-3,
    explicit_plus: 1e+3,
    big: 1.5e10,
    tiny: 2.5e-20,
    uppercase: 1E3,
  };
}

// Special values.
{
  result.special = {
    positive_inf: Infinity,
    negative_inf: -Infinity,
    nan: NaN,
    neg_zero: Object.is(-0, -0),
    neg_zero_string: String(-0),
    neg_zero_eq_zero: -0 === 0,
    max_safe: Number.MAX_SAFE_INTEGER,
    min_safe: Number.MIN_SAFE_INTEGER,
    max_value: Number.MAX_VALUE,
    epsilon: Number.EPSILON,
  };
}

// Float precision edge cases.
{
  result.precision = {
    point_one_plus_point_two: 0.1 + 0.2,
    point_three: 0.3,
    not_equal: 0.1 + 0.2 !== 0.3,
    close_enough: Math.abs(0.1 + 0.2 - 0.3) < Number.EPSILON,
  };
}

// BigInt literals.
{
  result.bigint = {
    zero: String(0n),
    large: String(9007199254740993n),
    hex: String(0xFFn),
    binary: String(0b1010n),
    octal: String(0o77n),
    negative: String(-42n),
    typeof: typeof 1n,
  };
}

// BigInt operations.
{
  result.bigint_ops = {
    add: String(1n + 2n),
    mul: String(3n * 4n),
    div: String(10n / 3n),
    mod: String(10n % 3n),
    pow: String(2n ** 10n),
    compare: 1n < 2n,
    equal_strict: 1n === 1n,
    not_equal_number: 1n !== 1,
  };
}

// parseInt / parseFloat edge cases.
{
  result.parse = {
    hex_string: parseInt("0xFF", 16),
    octal_string: parseInt("077", 8),
    binary_string: parseInt("1010", 2),
    leading_space: parseFloat("  3.14  "),
    trailing_junk: parseInt("42abc"),
    empty: parseInt(""),
    nan_result: Number.isNaN(parseInt("abc")),
  };
}

console.log(canon(result));
