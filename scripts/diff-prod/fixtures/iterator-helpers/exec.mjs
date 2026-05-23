// F-category: ES2025 Iterator Helpers (Iterator.prototype.{map,filter,take,drop,flatMap,reduce,toArray,forEach,some,every,find}).
// Surface every modern data-pipeline library will reach for.

function canon(v) {
  return JSON.stringify(v, (_k, v) => {
    if (v && typeof v === "object" && !Array.isArray(v)) {
      const out = {}; for (const k of Object.keys(v).sort()) out[k] = v[k]; return out;
    }
    return v;
  });
}

// NOTE: cruftless v1 has eager-collected generators (per diff-prod Rung-9);
// lazy generators / frame park+resume are deferred substrate work. Tests use
// finite ranges only — infinite-generator + .take(N) requires lazy generators
// to terminate iterator-creation. Iterator Helpers themselves are eager-
// consuming wrappers here, which composes correctly over finite generators.
function* range(lo, hi) { for (let i = lo; i < hi; i++) yield i; }

const result = {};

// presence + Iterator constructor surface
{
  const it = range(0, 3);
  const proto = Object.getPrototypeOf(it);
  result.surface = {
    has_map: typeof proto.map === "function",
    has_filter: typeof proto.filter === "function",
    has_take: typeof proto.take === "function",
    has_drop: typeof proto.drop === "function",
    has_flatMap: typeof proto.flatMap === "function",
    has_reduce: typeof proto.reduce === "function",
    has_toArray: typeof proto.toArray === "function",
    has_forEach: typeof proto.forEach === "function",
    has_some: typeof proto.some === "function",
    has_every: typeof proto.every === "function",
    has_find: typeof proto.find === "function",
  };
}

// .map + .take on a finite iterator
{
  const out = range(0, 10).map((x) => x * x).take(5).toArray();
  result.map_take = out;
}

// .filter + .drop + .take
{
  const out = range(0, 20).filter((x) => x % 2 === 0).drop(2).take(4).toArray();
  result.filter_drop_take = out;
}

// .flatMap
{
  const out = range(1, 4).flatMap((x) => [x, x * 10]).toArray();
  result.flatMap = out;
}

// .reduce with seed
{
  const sum = range(1, 6).reduce((a, b) => a + b, 0);
  const product = range(1, 6).reduce((a, b) => a * b, 1);
  result.reduce = { sum, product };
}

// .some / .every / .find (over finite ranges)
{
  result.short_circuit = {
    some_true: range(0, 10).some((x) => x === 3),
    every_false: range(0, 5).every((x) => x < 3),
    find_first_even: range(1, 10).find((x) => x % 2 === 0),
  };
}

// .forEach is side-effectful + returns undefined
{
  const seen = [];
  const rv = range(0, 3).forEach((x) => seen.push(x));
  result.for_each = { seen, rv };
}

// composition over a user-iterable
{
  const obj = {
    [Symbol.iterator]() {
      let i = 0;
      return { next: () => (i < 5 ? { value: i++, done: false } : { value: undefined, done: true }) };
    },
  };
  const out = Iterator.from(obj).map((x) => x + 100).toArray();
  result.from_user_iterable = out;
}

console.log(canon(result));
