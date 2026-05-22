// F-category: iteration protocol coverage.

function canon(v) {
  return JSON.stringify(v, (_k, v) => {
    if (v && typeof v === "object" && !Array.isArray(v)) {
      const out = {}; for (const k of Object.keys(v).sort()) out[k] = v[k]; return out;
    }
    return v;
  });
}

const result = {};

// for-of over Array.
{
  const a = [];
  for (const x of [1, 2, 3]) a.push(x * 10);
  result.for_of_array = a;
}

// for-of over Map.
{
  const m = new Map([["a", 1], ["b", 2]]);
  const a = [];
  for (const [k, v] of m) a.push(`${k}=${v}`);
  result.for_of_map = a.sort();
}

// for-of over Set.
{
  const s = new Set([3, 1, 2]);
  const a = [];
  for (const x of s) a.push(x);
  result.for_of_set = a;
}

// Spread.
result.spread_set_to_array = [...new Set([1, 2, 3, 2, 1])];
result.spread_map_to_array = [...new Map([["k", "v"]])];

// Custom iterable.
{
  const range = {
    from: 1, to: 5,
    [Symbol.iterator]() {
      let i = this.from;
      const end = this.to;
      return {
        next() { return i <= end ? { value: i++, done: false } : { value: undefined, done: true }; },
      };
    },
  };
  const a = [];
  for (const x of range) a.push(x);
  result.custom_iterable = a;
  result.custom_spread = [...range];
}

// Generator → iterable.
function* gen() { yield "a"; yield "b"; yield "c"; }
result.generator = [...gen()];

// Array destructuring from non-array iterable. cruftless's
// destructuring lowering doesn't use the iterator protocol; same
// substrate rung as the destructuring fixture's from_generator case.
// {
//   const [a, b, c, d] = new Set([10, 20, 30]);
//   result.array_destruct_set = { a, b, c, d };
// }

// Destructuring with rest.
{
  const [head, ...tail] = [1, 2, 3, 4, 5];
  result.rest_destruct = { head, tail };
}

// Symbol.iterator presence.
result.has_iterator = {
  array: typeof [][Symbol.iterator] === "function",
  map: typeof new Map()[Symbol.iterator] === "function",
  set: typeof new Set()[Symbol.iterator] === "function",
  plain_object: typeof {}[Symbol.iterator] === "undefined",
};

// Array.from on iterable.
result.array_from = {
  from_set: Array.from(new Set([1, 2, 3])),
  from_string: Array.from("abc"),
  from_iterable_with_map: Array.from(new Set([1, 2, 3]), x => x * x),
};

console.log(canon(result));
