// F-category: arguments object binding (IR/bytecode arguments_slot).

function canon(v) {
  return JSON.stringify(v, (_k, v) => {
    if (v && typeof v === "object" && !Array.isArray(v)) {
      const out = {}; for (const k of Object.keys(v).sort()) out[k] = v[k]; return out;
    }
    return v;
  });
}

const result = {};

// Basic arguments access.
{
  function fn() { return [...arguments]; }
  result.basic = fn(1, 2, 3);
}

// arguments.length.
{
  function fn() { return arguments.length; }
  result.length = { zero: fn(), one: fn("a"), three: fn(1, 2, 3) };
}

// arguments indexing.
{
  function fn() { return { first: arguments[0], second: arguments[1], oor: arguments[99] }; }
  result.indexing = fn("a", "b");
}

// arguments is array-like but not an Array.
{
  function fn() {
    return {
      is_array: Array.isArray(arguments),
      has_length: "length" in arguments,
      type: typeof arguments,
    };
  }
  result.array_like = fn(1, 2);
}

// Array.from(arguments).
{
  function fn() { return Array.from(arguments); }
  result.array_from = fn(10, 20, 30);
}

// Spread from arguments.
{
  function fn() { return [...arguments]; }
  result.spread = fn("x", "y", "z");
}

// arguments with named parameters.
{
  function fn(a, b, c) { return { a, b, c, args: [...arguments] }; }
  result.with_params = fn(1, 2, 3);
}

// Extra arguments beyond declared params.
{
  function fn(a) { return { a, extra: [...arguments].slice(1) }; }
  result.extra = fn(1, 2, 3, 4);
}

// Fewer arguments than params.
{
  function fn(a, b, c) { return { a, b, c, len: arguments.length }; }
  result.fewer = fn(1);
}

// Arrow function has no own arguments (inherits from outer).
{
  function outer() {
    const arrow = () => typeof arguments !== "undefined" ? arguments.length : "none";
    return arrow();
  }
  result.arrow_inherits = outer(1, 2, 3);
}

// Sloppy mode aliasing: arguments[0] aliases first param.
{
  const fn = new Function("a", "arguments[0] = 99; return a;");
  result.sloppy_alias = fn(42);
}

// Strict mode: no aliasing.
{
  function fn(a) { "use strict"; arguments[0] = 99; return a; }
  result.strict_no_alias = fn(42);
}

// arguments with rest parameter: arguments still works.
{
  function fn(a, ...rest) {
    return { args_len: arguments.length, rest_len: rest.length };
  }
  result.with_rest = fn(1, 2, 3, 4);
}

// arguments in nested function.
{
  function outer() {
    function inner() { return [...arguments]; }
    return inner("inner1", "inner2");
  }
  result.nested = outer("outer1");
}

// arguments.callee in sloppy mode.
{
  const fn = new Function("return typeof arguments.callee");
  result.callee_sloppy = fn();
}

console.log(canon(result));
