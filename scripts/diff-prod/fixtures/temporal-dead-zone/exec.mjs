// F-category: Temporal Dead Zone enforcement (IR/bytecode lowering).

function canon(v) {
  return JSON.stringify(v, (_k, v) => {
    if (v && typeof v === "object" && !Array.isArray(v)) {
      const out = {}; for (const k of Object.keys(v).sort()) out[k] = v[k]; return out;
    }
    return v;
  });
}

const result = {};

// let before initialization: ReferenceError.
{
  let threw = false;
  let err_name = null;
  try {
    eval("x; let x = 1;");
  } catch (e) { threw = true; err_name = e.constructor.name; }
  result.let_before_init = { threw, err_name };
}

// const before initialization: ReferenceError.
{
  let threw = false;
  let err_name = null;
  try {
    eval("y; const y = 1;");
  } catch (e) { threw = true; err_name = e.constructor.name; }
  result.const_before_init = { threw, err_name };
}

// var does NOT have TDZ (hoisted with undefined).
{
  const fn = new Function("return typeof z; var z = 1;");
  result.var_no_tdz = fn();
}

// TDZ in same block: function sees uninitialized let.
{
  let threw = false;
  try {
    eval(`
      (function() { return a; })();
      let a = 1;
    `);
  } catch (e) { threw = true; }
  result.function_sees_tdz = threw;
}

// TDZ with typeof: typeof in TDZ still throws (unlike typeof for undeclared).
{
  let threw = false;
  let err_name = null;
  try {
    eval("typeof tdz_var; let tdz_var = 1;");
  } catch (e) { threw = true; err_name = e.constructor.name; }
  result.typeof_in_tdz = { threw, err_name };
}

// typeof of undeclared variable: does NOT throw.
{
  result.typeof_undeclared = typeof completely_undeclared_var_xyz;
}

// TDZ in default parameter: later param can reference earlier, but not self.
{
  const fn = (a = 1, b = a + 1) => [a, b];
  result.default_param_order = fn();

  let threw = false;
  try {
    eval("(function(a = b, b = 1) { return [a, b]; })()")
  } catch { threw = true; }
  result.default_param_tdz = threw;
}

// TDZ for class declarations.
{
  let threw = false;
  try {
    eval("new MyClass(); class MyClass {}");
  } catch { threw = true; }
  result.class_tdz = threw;
}

// let in block scope: outer let accessible, inner let shadows.
{
  let x = "outer";
  {
    let x = "inner";
    result.block_shadow = x;
  }
  result.block_outer = x;
}

// for-loop let: each iteration gets fresh binding.
{
  const fns = [];
  for (let i = 0; i < 3; i++) {
    fns.push(() => i);
  }
  result.for_let_binding = fns.map(f => f());
}

// for-loop var: shared binding.
{
  const fns = [];
  for (var j = 0; j < 3; j++) {
    fns.push(new Function("j", "return function() { return j; }")(j));
  }
  result.for_var_binding_workaround = fns.map(f => f());
}

// const assignment after initialization throws TypeError.
{
  let threw = false;
  let err_name = null;
  try {
    eval("const c = 1; c = 2;");
  } catch (e) { threw = true; err_name = e.constructor.name; }
  result.const_reassign = { threw, err_name };
}

console.log(canon(result));
