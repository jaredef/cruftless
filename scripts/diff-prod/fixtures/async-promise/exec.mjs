// F-category: async/await + Promise combinators.

function canon(v) {
  return JSON.stringify(v, (_k, v) => {
    if (v && typeof v === "object" && !Array.isArray(v)) {
      const out = {};
      for (const k of Object.keys(v).sort()) out[k] = v[k];
      return out;
    }
    return v;
  });
}

const result = {};

// Basic await.
result.basic_await = await Promise.resolve(42);

// Chain.
result.chain = await Promise.resolve(1)
  .then(x => x + 1)
  .then(x => x * 10)
  .then(x => x - 5);

// Promise.all (all resolve).
result.all_resolved = await Promise.all([
  Promise.resolve(1),
  Promise.resolve(2),
  Promise.resolve(3),
]);

// Promise.allSettled (mix).
result.all_settled = (await Promise.allSettled([
  Promise.resolve("ok"),
  Promise.reject(new Error("nope")),
  Promise.resolve(42),
])).map(r => ({
  status: r.status,
  value_or_msg: r.status === "fulfilled" ? r.value : r.reason.message,
}));

// Promise.race (first to settle wins).
result.race_first_resolve = await Promise.race([
  Promise.resolve("fast"),
  new Promise(r => setTimeout(() => r("slow"), 50)),
]);

// Promise.any (first to fulfill wins).
result.any_first_fulfill = await Promise.any([
  Promise.reject("err1"),
  Promise.resolve("got-it"),
  Promise.reject("err2"),
]);

// async function returning thrown.
async function thrower() { throw new Error("boom"); }
try {
  await thrower();
  result.async_throw = "DID NOT THROW";
} catch (e) {
  result.async_throw = e.message;
}

// Microtask ordering: resolved Promise's .then runs before setTimeout(0).
let order = [];
await new Promise(resolve => {
  order.push("sync");
  setTimeout(() => order.push("timer"), 0);
  Promise.resolve().then(() => order.push("microtask"));
  setTimeout(() => { resolve(); }, 10);
});
result.microtask_order = order;

console.log(canon(result));
