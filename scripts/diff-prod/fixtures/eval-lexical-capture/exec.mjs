// F-category: direct eval lexical capture at the AST-to-bytecode boundary.

function canon(v) {
  return JSON.stringify(v, (_k, v) => {
    if (v && typeof v === "object" && !Array.isArray(v)) {
      const out = {}; for (const k of Object.keys(v).sort()) out[k] = v[k]; return out;
    }
    return v;
  });
}

const result = {};

// Direct eval captures outer const.
{
  const x = 42;
  result.const_capture = eval("x");
}

// Direct eval captures outer let.
{
  let y = "hello";
  result.let_capture = eval("y");
}

// Direct eval captures outer var.
{
  var z = 99;
  result.var_capture = eval("z");
}

// Direct eval captures function parameters.
{
  function fn(a, b) { return eval("a + b"); }
  result.param_capture = fn(10, 20);
}

// Direct eval sees outer closure variables.
{
  function outer() {
    const secret = "from-outer";
    function inner() { return eval("secret"); }
    return inner();
  }
  result.closure_capture = outer();
}

// Direct eval with multiple lexical layers.
{
  const a = 1;
  {
    const b = 2;
    {
      const c = 3;
      result.nested_block_capture = eval("a + b + c");
    }
  }
}

// Direct eval can declare var that leaks into the enclosing function.
{
  function fn() {
    eval("var evalDeclared = 'leaked'");
    return typeof evalDeclared !== "undefined" ? evalDeclared : "not-leaked";
  }
  result.var_hoist = fn();
}

// Direct eval in strict mode: eval gets its own scope for var.
{
  function fn() {
    "use strict";
    eval("var strictLocal = 'scoped'");
    return typeof strictLocal === "undefined" ? "correctly-scoped" : "leaked";
  }
  result.strict_eval_scope = fn();
}

// Indirect eval does NOT capture lexical scope.
{
  const localVal = "should-not-see";
  const indirectEval = eval;
  let caught = false;
  try {
    indirectEval("localVal");
  } catch {
    caught = true;
  }
  result.indirect_no_capture = caught;
}

// eval of expression vs statement.
{
  result.eval_expr = eval("1 + 2 + 3");
  result.eval_stmt = eval("let _tmp = 10; _tmp * 2");
}

// eval with template literal referencing outer scope.
{
  const name = "world";
  result.eval_template = eval("`hello ${name}`");
}

console.log(canon(result));
