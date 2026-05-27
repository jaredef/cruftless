// F-category: comma, grouping, eval completion, void, typeof.

function canon(v) {
  return JSON.stringify(v, (_k, v) => {
    if (v && typeof v === "object" && !Array.isArray(v)) {
      const out = {}; for (const k of Object.keys(v).sort()) out[k] = v[k]; return out;
    }
    return v;
  });
}

const result = {};

// Comma operator: evaluates left-to-right, returns last.
{
  const x = (1, 2, 3);
  result.comma_basic = x;

  let log = [];
  const y = (log.push("a"), log.push("b"), log.push("c"), "final");
  result.comma_side_effects = { y, log };
}

// Comma in for-loop update.
{
  let i, j;
  for (i = 0, j = 10; i < 3; i++, j--) {}
  result.comma_for = { i, j };
}

// Grouping: changes precedence but is not a syntactic form.
{
  result.grouping = {
    basic: (2 + 3) * 4,
    nested: ((1 + 2) * (3 + 4)),
    no_op: (42),
  };
}

// (0, fn)() pattern: indirect call loses this binding.
{
  const obj = {
    x: 42,
    fn() { return this?.x; }
  };
  const direct = obj.fn();
  const indirect = (0, obj.fn)();
  result.indirect_call = {
    direct,
    indirect_undefined: indirect === undefined || indirect === null,
  };
}

// void operator.
{
  result.void_op = {
    zero: void 0,
    expr: void (1 + 2),
    call: void console.log,
    type: typeof void 0,
  };
}

// typeof on various types.
{
  result.typeof_types = {
    number: typeof 42,
    string: typeof "s",
    boolean: typeof true,
    undefined: typeof undefined,
    null: typeof null,
    object: typeof {},
    array: typeof [],
    function: typeof function(){},
    arrow: typeof (() => {}),
    symbol: typeof Symbol(),
    bigint: typeof 1n,
    regex: typeof /x/,
  };
}

// typeof on undeclared variable: returns "undefined" (no throw).
{
  result.typeof_undeclared = typeof never_declared_xyz_123;
}

// eval returns the completion value of the last statement.
{
  result.eval_completion = {
    expr: eval("1 + 2"),
    multi: eval("1; 2; 3"),
    var: eval("var _ev = 42; _ev"),
    if_true: eval("if (true) 'yes'; else 'no'"),
    if_false: eval("if (false) 'yes'; else 'no'"),
    block: eval("{ 1; 2; 3; }"),
    empty: eval(""),
  };
}

// delete operator.
{
  const obj = { a: 1, b: 2 };
  result.delete_op = {
    own: delete obj.a,
    missing: delete obj.nonexistent,
    after: Object.keys(obj),
  };
}

// delete on non-configurable property.
{
  const obj = {};
  Object.defineProperty(obj, "fixed", { value: 1, configurable: false });
  let threw = false;
  try {
    "use strict";
    delete obj.fixed;
  } catch { threw = true; }
  result.delete_nonconfig = { threw, still_has: "fixed" in obj };
}

// Chained typeof.
{
  result.typeof_chain = typeof typeof 42;
}

// void in expression context.
{
  const arr = [1, 2, 3].map(x => void x);
  result.void_map = arr;
}

console.log(canon(result));
