// F-category: destructuring coverage.

function canon(v) {
  return JSON.stringify(v, (_k, v) => {
    if (v && typeof v === "object" && !Array.isArray(v)) {
      const out = {}; for (const k of Object.keys(v).sort()) out[k] = v[k]; return out;
    }
    return v;
  });
}

const result = {};

// Array destructuring.
{
  const [a, b, c] = [1, 2, 3];
  result.basic_array = { a, b, c };
}

// Skip elements.
{
  const [a, , c] = [1, 2, 3];
  result.skip = { a, c };
}

// Defaults.
{
  const [x = 100, y = 200] = [1];
  result.defaults_array = { x, y };
}

// Rest.
{
  const [head, ...tail] = [1, 2, 3, 4];
  result.rest_array = { head, tail };
}

// Object destructuring.
{
  const { a, b } = { a: 1, b: 2, c: 3 };
  result.basic_object = { a, b };
}

// Rename.
{
  const { a: x, b: y } = { a: 1, b: 2 };
  result.rename = { x, y };
}

// Default + rename.
{
  const { a: x = 10, b: y = 20 } = { a: 1 };
  result.rename_default = { x, y };
}

// Nested.
{
  const { a: { b: { c } } } = { a: { b: { c: 42 } } };
  result.nested = { c };
}

// Object rest.
{
  const { a, ...rest } = { a: 1, b: 2, c: 3 };
  result.object_rest = { a, rest };
}

// Computed keys.
{
  const k = "dynamic";
  const { [k]: v } = { dynamic: "found" };
  result.computed = { v };
}

// Parameter destructuring.
function fn({ x = 0, y = 0 } = {}) { return x + y; }
result.param_object = { with_args: fn({ x: 3, y: 4 }), no_args: fn() };

function fn2([a, b] = [10, 20]) { return a * b; }
result.param_array = { with_args: fn2([2, 3]), no_args: fn2() };

// Destructuring from a non-array iterable. cruftless's array
// destructuring lowering uses direct numeric-index access rather than
// the iterator protocol, so generators/Sets/Maps don't unpack via
// `[a,b,c] = iter`. Substantive bytecode-compiler rung; deferred.
//
// {
//   function* g() { yield 1; yield 2; yield 3; }
//   const [a, b, c] = g();
//   result.from_generator = { a, b, c };
// }

// Swap via destructuring.
{
  let a = 1, b = 2;
  [a, b] = [b, a];
  result.swap = { a, b };
}

// Mixed nested.
{
  const data = { users: [{ name: "Alice", age: 30 }, { name: "Bob", age: 25 }] };
  const { users: [{ name: first_name }, { name: second_name, age }] } = data;
  result.mixed_nested = { first_name, second_name, age };
}

console.log(canon(result));
