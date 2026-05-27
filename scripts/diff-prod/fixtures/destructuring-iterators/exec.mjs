// F-category: destructuring via iterator protocol (AST-to-bytecode boundary).

function canon(v) {
  return JSON.stringify(v, (_k, v) => {
    if (v && typeof v === "object" && !Array.isArray(v)) {
      const out = {}; for (const k of Object.keys(v).sort()) out[k] = v[k]; return out;
    }
    return v;
  });
}

const result = {};

// Destructuring from a generator.
{
  function* g() { yield 1; yield 2; yield 3; }
  const [a, b, c] = g();
  result.generator = { a, b, c };
}

// Destructuring from a Set (iteration order = insertion order).
{
  const s = new Set([10, 20, 30]);
  const [a, b, c] = s;
  result.set = { a, b, c };
}

// Destructuring from a Map (entries as [key, value] pairs).
{
  const m = new Map([["x", 1], ["y", 2]]);
  const [[k1, v1], [k2, v2]] = m;
  result.map = { k1, v1, k2, v2 };
}

// Destructuring from a string (char iteration).
{
  const [a, b, c, d, e] = "hello";
  result.string = { a, b, c, d, e };
}

// Destructuring with rest from a generator.
{
  function* g() { yield "a"; yield "b"; yield "c"; yield "d"; }
  const [first, ...rest] = g();
  result.rest_generator = { first, rest };
}

// Destructuring with skip from an iterable.
{
  function* g() { yield 1; yield 2; yield 3; yield 4; yield 5; }
  const [a, , c, , e] = g();
  result.skip_iterable = { a, c, e };
}

// Destructuring with default from an iterable that yields fewer values.
{
  function* g() { yield 1; }
  const [a, b = 99, c = 100] = g();
  result.defaults_iterable = { a, b, c };
}

// Custom iterable via Symbol.iterator.
{
  const iterable = {
    [Symbol.iterator]() {
      let i = 0;
      const vals = [100, 200, 300];
      return {
        next() {
          return i < vals.length
            ? { value: vals[i++], done: false }
            : { value: undefined, done: true };
        }
      };
    }
  };
  const [a, b, c] = iterable;
  result.custom_iterable = { a, b, c };
}

// for-of + destructuring from Map.
{
  const m = new Map([["a", 1], ["b", 2], ["c", 3]]);
  const entries = [];
  for (const [k, v] of m) entries.push(k + "=" + v);
  result.for_of_map = entries;
}

// Nested destructuring from iterables.
{
  function* inner() { yield 10; yield 20; }
  function* outer() { yield inner(); yield inner(); }
  const iterators = [...outer()];
  const [a, b] = iterators[0];
  const [c, d] = iterators[1];
  result.nested_iterators = { a, b, c, d };
}

// Parameter destructuring from iterable.
{
  function fn([a, b, c]) { return a + b + c; }
  result.param_iterable = fn(new Set([1, 2, 3]));
}

console.log(canon(result));
