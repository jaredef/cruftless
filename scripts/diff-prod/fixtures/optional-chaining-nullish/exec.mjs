// F-category: optional chaining + nullish coalescing (JumpIfNullish opcode).

function canon(v) {
  return JSON.stringify(v, (_k, v) => {
    if (v && typeof v === "object" && !Array.isArray(v)) {
      const out = {}; for (const k of Object.keys(v).sort()) out[k] = v[k]; return out;
    }
    return v;
  });
}

const result = {};

// Optional property access.
{
  const obj = { a: { b: { c: 42 } } };
  result.prop = {
    deep: obj?.a?.b?.c,
    missing: obj?.x?.y?.z,
    null_base: null?.x,
    undef_base: undefined?.x,
  };
}

// Optional method call.
{
  const obj = { greet(name) { return "hello " + name; } };
  result.method = {
    exists: obj.greet?.("world"),
    missing: obj.missing?.("world"),
    null_base: null?.greet?.("world"),
  };
}

// Optional computed access.
{
  const obj = { items: [10, 20, 30] };
  result.computed = {
    exists: obj?.items?.[1],
    missing: obj?.nothing?.[0],
    null_base: null?.[0],
  };
}

// Stacking optional chains.
{
  const data = { users: [{ name: "Alice", address: { city: "NYC" } }] };
  result.stacked = {
    city: data?.users?.[0]?.address?.city,
    missing: data?.users?.[1]?.address?.city,
    deep_null: data?.users?.[0]?.phone?.number,
  };
}

// Short-circuit: RHS not evaluated.
{
  let called = false;
  const fn = () => { called = true; return 42; };
  const r = null?.[fn()];
  result.short_circuit = { r, called };
}

// Nullish coalescing.
{
  result.nullish = {
    null: null ?? "default",
    undef: undefined ?? "default",
    zero: 0 ?? "default",
    empty: "" ?? "default",
    false: false ?? "default",
    nan: NaN ?? "default",
    value: 42 ?? "default",
  };
}

// ?? vs || for falsy values.
{
  result.nullish_vs_or = {
    zero_nullish: 0 ?? 99,
    zero_or: 0 || 99,
    empty_nullish: "" ?? "default",
    empty_or: "" || "default",
    false_nullish: false ?? true,
    false_or: false || true,
  };
}

// Logical assignment: ??=, &&=, ||=.
{
  let a = null;
  a ??= 42;
  let b = 0;
  b ??= 42;
  result.nullish_assign = { a, b };
}

{
  let a = 1;
  a &&= 42;
  let b = 0;
  b &&= 42;
  result.and_assign = { a, b };
}

{
  let a = 0;
  a ||= 42;
  let b = 1;
  b ||= 42;
  result.or_assign = { a, b };
}

// Short-circuit assignment: RHS not evaluated when not needed.
{
  let count = 0;
  const bump = () => { count++; return 99; };

  let a = 1;
  a ??= bump();
  result.nullish_assign_skip = { a, count };

  let b = 0;
  b &&= bump();
  result.and_assign_skip = { b, count };

  let c = 1;
  c ||= bump();
  result.or_assign_skip = { c, count };
}

// delete with optional chaining.
{
  const obj = { a: { b: 1 } };
  const r1 = delete obj?.a?.b;
  const r2 = delete obj?.missing?.prop;
  result.delete_optional = { r1, r2, obj_a: obj.a };
}

console.log(canon(result));
