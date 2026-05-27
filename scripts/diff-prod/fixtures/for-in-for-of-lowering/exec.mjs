// F-category: for-in/for-of lowering (IterInit/IterNext/IterClose opcodes).

function canon(v) {
  return JSON.stringify(v, (_k, v) => {
    if (v && typeof v === "object" && !Array.isArray(v)) {
      const out = {}; for (const k of Object.keys(v).sort()) out[k] = v[k]; return out;
    }
    return v;
  });
}

const result = {};

// for-in: enumerates own and inherited enumerable string keys.
{
  const proto = { inherited: true };
  const obj = Object.create(proto);
  obj.own = 1;
  obj.another = 2;
  const keys = [];
  for (const k in obj) keys.push(k);
  result.for_in_inherited = keys.sort();
}

// for-in: only enumerable properties.
{
  const obj = {};
  Object.defineProperty(obj, "hidden", { value: 1, enumerable: false });
  obj.visible = 2;
  const keys = [];
  for (const k in obj) keys.push(k);
  result.for_in_enumerable = keys;
}

// for-in: integer-indexed properties first.
{
  const obj = {};
  obj.b = 1;
  obj[2] = 2;
  obj.a = 3;
  obj[0] = 4;
  obj[1] = 5;
  const keys = [];
  for (const k in obj) keys.push(k);
  result.for_in_order = keys;
}

// for-in skips symbol keys.
{
  const s = Symbol("sym");
  const obj = { a: 1, [s]: 2, b: 3 };
  const keys = [];
  for (const k in obj) keys.push(k);
  result.for_in_no_symbols = keys;
}

// for-of with array.
{
  const vals = [];
  for (const x of [10, 20, 30]) vals.push(x);
  result.for_of_array = vals;
}

// for-of with string (character iteration).
{
  const chars = [];
  for (const c of "hello") chars.push(c);
  result.for_of_string = chars;
}

// for-of with Map.
{
  const entries = [];
  for (const [k, v] of new Map([["a", 1], ["b", 2]])) entries.push(k + "=" + v);
  result.for_of_map = entries;
}

// for-of with Set.
{
  const vals = [];
  for (const x of new Set([3, 1, 2])) vals.push(x);
  result.for_of_set = vals;
}

// for-of with custom iterable.
{
  const iterable = {
    [Symbol.iterator]() {
      let n = 0;
      return { next() { return n < 3 ? { value: n++, done: false } : { done: true }; } };
    }
  };
  const vals = [];
  for (const x of iterable) vals.push(x);
  result.for_of_custom = vals;
}

// for-of break.
{
  const vals = [];
  for (const x of [1, 2, 3, 4, 5]) {
    if (x > 3) break;
    vals.push(x);
  }
  result.for_of_break = vals;
}

// for-of continue.
{
  const vals = [];
  for (const x of [1, 2, 3, 4, 5]) {
    if (x % 2 === 0) continue;
    vals.push(x);
  }
  result.for_of_continue = vals;
}

// for-in with deletion during iteration.
{
  const obj = { a: 1, b: 2, c: 3, d: 4 };
  const keys = [];
  for (const k in obj) {
    keys.push(k);
    if (k === "b") delete obj.d;
  }
  result.for_in_delete = { keys, remaining: Object.keys(obj).sort() };
}

// for-of with destructuring.
{
  const pairs = [[1, "a"], [2, "b"], [3, "c"]];
  const log = [];
  for (const [num, letter] of pairs) {
    log.push(letter + num);
  }
  result.for_of_destructure = log;
}

// Nested for-of.
{
  const matrix = [[1, 2], [3, 4], [5, 6]];
  const flat = [];
  for (const row of matrix) {
    for (const cell of row) {
      flat.push(cell);
    }
  }
  result.nested_for_of = flat;
}

console.log(canon(result));
