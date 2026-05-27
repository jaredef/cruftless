// F-category: hoisting semantics (IR scope analysis).

function canon(v) {
  return JSON.stringify(v, (_k, v) => {
    if (v && typeof v === "object" && !Array.isArray(v)) {
      const out = {}; for (const k of Object.keys(v).sort()) out[k] = v[k]; return out;
    }
    return v;
  });
}

const result = {};

// Function declaration is hoisted with its body.
{
  const fn = new Function(`
    var r = hoisted();
    function hoisted() { return 42; }
    return r;
  `);
  result.function_decl_hoist = fn();
}

// var is hoisted but initialized to undefined.
{
  const fn = new Function(`
    var before = x;
    var x = 42;
    return { before: before, after: x };
  `);
  result.var_hoist = fn();
}

// Function declaration overrides var with same name.
{
  const fn = new Function(`
    var x = 1;
    function x() { return "fn"; }
    return typeof x;
  `);
  result.fn_overrides_var = fn();
}

// Function expression is NOT hoisted.
{
  const fn = new Function(`
    var before = typeof notHoisted;
    var notHoisted = function() { return 42; };
    return before;
  `);
  result.fn_expr_not_hoisted = fn();
}

// Multiple function declarations: last wins.
{
  const fn = new Function(`
    function f() { return 1; }
    function f() { return 2; }
    function f() { return 3; }
    return f();
  `);
  result.last_fn_wins = fn();
}

// var hoists across blocks.
{
  const fn = new Function(`
    function test() {
      if (false) {
        var x = 42;
      }
      return x;
    }
    return test();
  `);
  result.var_across_blocks = fn();
}

// var in for-loop hoists to function scope.
{
  const fn = new Function(`
    for (var i = 0; i < 3; i++) {}
    return i;
  `);
  result.var_for_hoist = fn();
}

// let does NOT hoist across blocks.
{
  let threw = false;
  try {
    eval(`
      if (true) { let blockLocal = 42; }
      blockLocal;
    `);
  } catch { threw = true; }
  result.let_no_hoist = threw;
}

// Nested function declarations.
{
  const fn = new Function(`
    function outer() {
      function inner() { return "inner"; }
      return inner();
    }
    return outer();
  `);
  result.nested_fn_hoist = fn();
}

// var and function interaction in same scope.
{
  const fn = new Function(`
    var r1 = typeof f;
    var f = 1;
    function f() {}
    var r2 = typeof f;
    return { r1: r1, r2: r2 };
  `);
  result.var_fn_interaction = fn();
}

// Arguments and var interaction.
{
  const fn = new Function(`
    return (function(a) {
      var a = 99;
      return a;
    })(42);
  `);
  result.param_var_shadow = fn();
}

console.log(canon(result));
