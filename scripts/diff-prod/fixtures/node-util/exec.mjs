// F-category: node:util surface.

import * as util from "node:util";

function canon(v) {
  return JSON.stringify(v, (_k, v) => {
    if (v && typeof v === "object" && !Array.isArray(v)) {
      const out = {}; for (const k of Object.keys(v).sort()) out[k] = v[k]; return out;
    }
    return v;
  });
}

const result = {};

// isDeepStrictEqual.
{
  const has = typeof util.isDeepStrictEqual === "function";
  result.deep_strict_equal = { present: has };
  if (has) {
    result.deep_strict_equal.same_obj = util.isDeepStrictEqual({ a: 1, b: [2, 3] }, { a: 1, b: [2, 3] });
    result.deep_strict_equal.diff_obj = util.isDeepStrictEqual({ a: 1 }, { a: 2 });
    result.deep_strict_equal.nested = util.isDeepStrictEqual(
      { x: { y: { z: [1, 2, 3] } } },
      { x: { y: { z: [1, 2, 3] } } }
    );
    result.deep_strict_equal.type_mismatch = util.isDeepStrictEqual(1, "1");
    result.deep_strict_equal.nan = util.isDeepStrictEqual(NaN, NaN);
    result.deep_strict_equal.map = util.isDeepStrictEqual(
      new Map([["a", 1]]),
      new Map([["a", 1]])
    );
    result.deep_strict_equal.set = util.isDeepStrictEqual(
      new Set([1, 2, 3]),
      new Set([1, 2, 3])
    );
  }
}

// util.types.
{
  const t = util.types;
  const has = t != null && typeof t === "object";
  result.types = { present: has };
  if (has) {
    result.types.isDate_date = typeof t.isDate === "function" ? t.isDate(new Date()) : "absent";
    result.types.isDate_obj = typeof t.isDate === "function" ? t.isDate({}) : "absent";
    result.types.isRegExp_re = typeof t.isRegExp === "function" ? t.isRegExp(/abc/) : "absent";
    result.types.isRegExp_str = typeof t.isRegExp === "function" ? t.isRegExp("abc") : "absent";
    result.types.isMap = typeof t.isMap === "function" ? t.isMap(new Map()) : "absent";
    result.types.isMap_obj = typeof t.isMap === "function" ? t.isMap({}) : "absent";
    result.types.isSet = typeof t.isSet === "function" ? t.isSet(new Set()) : "absent";
    result.types.isSet_arr = typeof t.isSet === "function" ? t.isSet([]) : "absent";
    result.types.isPromise = typeof t.isPromise === "function" ? t.isPromise(Promise.resolve()) : "absent";
    result.types.isPromise_obj = typeof t.isPromise === "function" ? t.isPromise({}) : "absent";
  }
}

// util.format.
{
  const has = typeof util.format === "function";
  result.format = { present: has };
  if (has) {
    result.format.string = util.format("hello %s", "world");
    result.format.number = util.format("n=%d", 42);
    result.format.json = util.format("obj=%j", { a: 1 });
    result.format.percent = util.format("100%%");
    result.format.extra_args = util.format("a", "b", "c");
  }
}

console.log(canon(result));
