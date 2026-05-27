// F-category: Promise combinators.

function canon(v) {
  return JSON.stringify(v, (_k, v) => {
    if (v && typeof v === "object" && !Array.isArray(v)) {
      const out = {}; for (const k of Object.keys(v).sort()) out[k] = v[k]; return out;
    }
    return v;
  });
}

const result = {};

result.all_basic = await Promise.all([
  Promise.resolve(1),
  Promise.resolve(2),
  Promise.resolve(3),
]);

result.all_empty = await Promise.all([]);

try {
  await Promise.all([
    Promise.resolve("a"),
    Promise.reject(new Error("fail")),
    Promise.resolve("c"),
  ]);
  result.all_reject = "DID NOT THROW";
} catch (e) {
  result.all_reject = e.message;
}

result.allSettled_mixed = (await Promise.allSettled([
  Promise.resolve("ok"),
  Promise.reject(new Error("bad")),
  Promise.resolve(42),
])).map(r => ({
  status: r.status,
  has_value: "value" in r,
  has_reason: "reason" in r,
  payload: r.status === "fulfilled" ? r.value : r.reason.message,
}));

result.allSettled_empty = await Promise.allSettled([]);

result.allSettled_all_reject = (await Promise.allSettled([
  Promise.reject("e1"),
  Promise.reject("e2"),
])).map(r => ({ status: r.status, reason: r.reason }));

result.race_resolves = await Promise.race([
  Promise.resolve("first"),
  Promise.resolve("second"),
]);

try {
  await Promise.race([
    Promise.reject(new Error("race-fail")),
    Promise.resolve("late"),
  ]);
  result.race_reject = "DID NOT THROW";
} catch (e) {
  result.race_reject = e.message;
}

result.race_empty = await Promise.race([
  new Promise(r => setTimeout(r, 1, "fallback")),
  ...[],
]).then(() => "resolved-with-fallback");

result.any_basic = await Promise.any([
  Promise.reject("no1"),
  Promise.resolve("yes"),
  Promise.reject("no2"),
]);

result.any_first_wins = await Promise.any([
  Promise.resolve("alpha"),
  Promise.resolve("beta"),
]);

try {
  await Promise.any([
    Promise.reject(new Error("e1")),
    Promise.reject(new Error("e2")),
  ]);
  result.any_all_reject = "DID NOT THROW";
} catch (e) {
  result.any_all_reject = {
    name: e.constructor.name,
    message: e.message,
    errors_count: e.errors.length,
    errors_messages: e.errors.map(x => x.message),
  };
}

try {
  await Promise.any([]);
  result.any_empty = "DID NOT THROW";
} catch (e) {
  result.any_empty = {
    name: e.constructor.name,
    errors_count: e.errors.length,
  };
}

result.all_non_promise = await Promise.all([1, "two", true, null]);

result.race_non_promise = await Promise.race([42]);

console.log(canon(result));
