// F-category: arrow function parsing and semantics edge cases.

function canon(v) {
  return JSON.stringify(v, (_k, v) => {
    if (v && typeof v === "object" && !Array.isArray(v)) {
      const out = {}; for (const k of Object.keys(v).sort()) out[k] = v[k]; return out;
    }
    return v;
  });
}

const result = {};

// Single param no parens.
{
  const fn = x => x * 2;
  result.single_param = fn(21);
}

// Expression body vs block body.
{
  const expr = x => x + 1;
  const block = x => { return x + 1; };
  result.body_forms = { expr: expr(1), block: block(1) };
}

// Object literal return requires parens.
{
  const fn = () => ({ a: 1, b: 2 });
  result.object_return = fn();
}

// Destructuring parameters.
{
  const fn = ({ x, y }) => x + y;
  result.destructure_param = fn({ x: 10, y: 20 });
}

// Array destructuring parameter.
{
  const fn = ([a, b, c]) => a + b + c;
  result.array_destructure_param = fn([1, 2, 3]);
}

// Default parameter values.
{
  const fn = (x = 10, y = 20) => x + y;
  result.defaults = { no_args: fn(), one_arg: fn(5), both: fn(5, 6) };
}

// Rest parameter.
{
  const fn = (first, ...rest) => ({ first, rest });
  result.rest = fn(1, 2, 3, 4);
}

// Lexical this: arrow captures outer this.
{
  const obj = {
    value: 42,
    getArrow() {
      const fn = () => this.value;
      return fn();
    },
    getRegular() {
      const fn = function() { return this; };
      return fn();
    }
  };
  result.lexical_this = {
    arrow: obj.getArrow(),
    regular_is_undefined: obj.getRegular() === undefined,
  };
}

// Arrow has no arguments object.
{
  function outer() {
    const arrow = () => typeof arguments !== "undefined" ? [...arguments] : "no-arguments";
    return arrow();
  }
  result.no_own_arguments = outer(1, 2, 3);
}

// Arrow has no prototype property.
{
  const fn = () => {};
  result.no_prototype = {
    has_prototype: "prototype" in fn,
  };
}

// Arrow cannot be used with new.
{
  const fn = () => {};
  let threw = false;
  let err_name = null;
  try { new fn(); } catch (e) { threw = true; err_name = e.constructor.name; }
  result.no_new = { threw, err_name };
}

// Arrow in method shorthand.
{
  const obj = {
    items: [1, 2, 3],
    doubled() { return this.items.map(x => x * 2); }
  };
  result.in_method = obj.doubled();
}

// Nested arrows.
{
  const add = a => b => a + b;
  result.nested = add(10)(20);
}

// Arrow returning arrow.
{
  const compose = f => g => x => f(g(x));
  const double = x => x * 2;
  const inc = x => x + 1;
  result.compose = compose(double)(inc)(5);
}

// IIFE arrow.
{
  result.iife = (x => x * x)(7);
}

// Arrow with computed property names.
{
  const key = "dynamic";
  const fn = () => ({ [key]: 42, static: 1 });
  result.computed_prop = fn();
}

console.log(canon(result));
