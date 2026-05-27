// F-category: Array.from and Array.of.

function canon(v) {
  return JSON.stringify(v, (_k, v) => {
    if (v && typeof v === "object" && !Array.isArray(v)) {
      const out = {}; for (const k of Object.keys(v).sort()) out[k] = v[k]; return out;
    }
    return v;
  });
}

const result = {};

result.from_string = Array.from("hello");

result.from_set = Array.from(new Set([3, 1, 2, 1]));

result.from_map = Array.from(new Map([["a", 1], ["b", 2]]));

function* gen() { yield 10; yield 20; yield 30; }
result.from_generator = Array.from(gen());

result.from_arguments = (function () { return Array.from(arguments); })(1, "two", true);

result.from_mapFn = Array.from([1, 2, 3], x => x * x);

result.from_mapFn_index = Array.from([10, 20, 30], (x, i) => x + i);

const ctx = { multiplier: 5 };
result.from_thisArg = Array.from([1, 2, 3], function (x) { return x * this.multiplier; }, ctx);

result.of_basic = Array.of(1, 2, 3);

result.of_single = Array.of(7);

result.of_mixed = Array.of("a", null, undefined, true, 42);

result.of_empty = Array.of();

result.from_length_only = Array.from({ length: 3 });

result.from_length_mapFn = Array.from({ length: 4 }, (_, i) => i * 2);

result.from_array_like = Array.from({ 0: "x", 1: "y", 2: "z", length: 3 });

result.from_empty_iterable = Array.from([]);

result.from_empty_set = Array.from(new Set());

try {
  result.from_null = "no-throw";
  Array.from(null);
} catch (e) {
  result.from_null = { threw: true, name: e.constructor.name };
}

try {
  result.from_undefined = "no-throw";
  Array.from(undefined);
} catch (e) {
  result.from_undefined = { threw: true, name: e.constructor.name };
}

result.from_number_string = Array.from("123");

result.from_unicode = Array.from("\u{1F600}AB");

console.log(canon(result));
