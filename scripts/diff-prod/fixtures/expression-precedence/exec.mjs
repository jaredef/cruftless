// F-category: expression precedence and associativity (parser depth).

function canon(v) {
  return JSON.stringify(v, (_k, v) => {
    if (v && typeof v === "object" && !Array.isArray(v)) {
      const out = {}; for (const k of Object.keys(v).sort()) out[k] = v[k]; return out;
    }
    return v;
  });
}

const result = {};

// ** is right-associative.
{
  result.pow_assoc = {
    right: 2 ** 3 ** 2,
    explicit_left: (2 ** 3) ** 2,
    explicit_right: 2 ** (3 ** 2),
  };
}

// Arithmetic precedence.
{
  result.arith = {
    add_mul: 2 + 3 * 4,
    mul_add: (2 + 3) * 4,
    sub_div: 10 - 6 / 2,
    mod_add: 10 % 3 + 1,
    unary_minus_pow: -(2 ** 2),
  };
}

// Comparison chains (left-to-right, no chaining in JS).
{
  result.compare = {
    lt_result: (1 < 2) < 3,
    gt_result: (3 > 2) > 0,
    mixed: 1 < 2 === true,
  };
}

// Bitwise vs comparison precedence.
{
  result.bitwise_compare = {
    and_lt: 5 & 3 < 4,
    or_eq: 1 | 0 === 1,
    xor_ne: 1 ^ 0 !== 1,
    explicit: (5 & 3) < 4,
  };
}

// Logical vs bitwise.
{
  result.logical_bitwise = {
    and_bitor: true && 0 | 1,
    or_bitand: false || 1 & 1,
  };
}

// Ternary associativity (right-associative nesting).
{
  const a = true ? "a" : true ? "b" : "c";
  const b = true ? "a" : (true ? "b" : "c");
  const c = (true ? "a" : true) ? "b" : "c";
  result.ternary = { a, b, c };
}

// Ternary vs assignment.
{
  let x;
  x = true ? 1 : 2;
  result.ternary_assign = x;
}

// Comma operator precedence (lowest).
{
  const x = (1, 2, 3);
  result.comma = x;

  const arr = [1, 2, 3];
  result.comma_not_in_array = arr;
}

// typeof precedence.
{
  result.typeof_prec = {
    string: typeof "hello" + " world",
    number: typeof 42 + 1,
    paren: typeof ("hello" + " world"),
  };
}

// void precedence.
{
  result.void_prec = {
    basic: void 0,
    expr: void (1 + 2),
    with_add: (void 0) + 1,
  };
}

// delete precedence.
{
  const obj = { a: 1, b: 2 };
  const d = delete obj.a;
  result.delete_prec = { deleted: d, keys: Object.keys(obj) };
}

// Unary chain.
{
  result.unary_chain = {
    not_not: !!1,
    neg_neg: - -5,
    typeof_typeof: typeof typeof 1,
    void_typeof: typeof void 0,
  };
}

// Short-circuit evaluation order.
{
  let log = [];
  const a = (log.push("a"), false) && (log.push("b"), true);
  result.short_circuit_and = { a, log: [...log] };

  log = [];
  const b = (log.push("a"), true) || (log.push("b"), false);
  result.short_circuit_or = { b, log: [...log] };

  log = [];
  const c = (log.push("a"), null) ?? (log.push("b"), 42);
  result.short_circuit_nullish = { c, log: [...log] };
}

// Assignment is right-associative.
{
  let a, b, c;
  a = b = c = 42;
  result.assign_chain = { a, b, c };
}

// Increment/decrement vs property access.
{
  const obj = { x: 5 };
  const pre = ++obj.x;
  const post = obj.x++;
  result.inc_prop = { pre, post, final: obj.x };
}

console.log(canon(result));
