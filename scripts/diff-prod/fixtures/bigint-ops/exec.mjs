// L-category: BigInt language surface.

function canon(v) {
  return JSON.stringify(v, (_k, v) => {
    if (typeof v === "bigint") return v.toString() + "n";
    if (v && typeof v === "object" && !Array.isArray(v)) {
      const out = {}; for (const k of Object.keys(v).sort()) out[k] = v[k]; return out;
    }
    return v;
  });
}

const result = {};

// Literals + arithmetic.
{
  const a = 10n;
  const b = 3n;
  result.arith = {
    add: a + b,
    sub: a - b,
    mul: a * b,
    div: a / b,
    mod: a % b,
    pow: a ** b,
    neg: -a,
  };
}

// Big magnitudes.
{
  const big = 2n ** 64n;
  result.big = {
    val: big,
    plus_one: big + 1n,
    minus_one: big - 1n,
    squared_first8: (big * big).toString().slice(0, 8),
  };
}

// Comparison.
{
  result.cmp = {
    lt: 1n < 2n,
    gt: 3n > 2n,
    eq: 5n === 5n,
    mixed_lt: 1n < 2,
    mixed_gt: 3n > 2,
    mixed_eq: 5n == 5,
    mixed_strict: 5n === 5,
  };
}

// Conversion.
{
  result.conv = {
    from_string: BigInt("123456789012345678901234567890").toString(),
    from_number: BigInt(42).toString(),
    to_number: Number(123n),
    to_string_hex: (255n).toString(16),
    to_string_bin: (10n).toString(2),
  };
}

// Throws on Number(BigInt) precision loss is NOT a throw — just convert.
// But BigInt(1.5) throws RangeError, BigInt(NaN) throws.
{
  let r1 = "ok"; try { BigInt(1.5); r1 = "no-throw"; } catch (e) { r1 = e.constructor.name; }
  let r2 = "ok"; try { 1n + 1; r2 = "no-throw"; } catch (e) { r2 = e.constructor.name; }
  result.throws = { bigint_from_float: r1, mixed_add: r2 };
}

// Bitwise.
{
  result.bitwise = {
    and: (0xffn & 0x0fn),
    or: (0x0fn | 0xf0n),
    xor: (0xffn ^ 0x0fn),
    not: ~0n,
    shl: 1n << 8n,
    shr: 256n >> 2n,
  };
}

// BigInt64Array roundtrip.
{
  const arr = new BigInt64Array([1n, 2n, 3n, -1n]);
  result.typed = {
    length: arr.length,
    sum: arr[0] + arr[1] + arr[2],
    neg: arr[3],
    byteLen: arr.byteLength,
  };
}

console.log(canon(result));
