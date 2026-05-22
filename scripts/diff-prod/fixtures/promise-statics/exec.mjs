// L-category: Promise static combinators.

function canon(v) {
  return JSON.stringify(v, (_k, v) => {
    if (v && typeof v === "object" && !Array.isArray(v)) {
      const out = {}; for (const k of Object.keys(v).sort()) out[k] = v[k]; return out;
    }
    return v;
  });
}

async function main() {
  const result = {};

  // Promise.all - all fulfilled.
  result.all_ok = await Promise.all([Promise.resolve(1), Promise.resolve(2), Promise.resolve(3)]);

  // Promise.all - rejection short-circuits.
  try {
    await Promise.all([Promise.resolve(1), Promise.reject(new Error("boom")), Promise.resolve(3)]);
    result.all_rej = "no-throw";
  } catch (e) {
    result.all_rej = "caught:" + e.message;
  }

  // Promise.allSettled.
  {
    const r = await Promise.allSettled([
      Promise.resolve("a"),
      Promise.reject(new Error("b-err")),
      Promise.resolve("c"),
    ]);
    result.all_settled = r.map(x => x.status === "fulfilled"
      ? { s: "f", v: x.value }
      : { s: "r", m: x.reason.message });
  }

  // Promise.race - first settled wins.
  {
    const r = await Promise.race([
      new Promise(res => setTimeout(() => res("slow"), 50)),
      Promise.resolve("fast"),
    ]);
    result.race = r;
  }

  // Promise.any - first fulfilled wins; all rejected throws AggregateError.
  {
    const r = await Promise.any([
      Promise.reject(new Error("e1")),
      Promise.resolve("got-it"),
      Promise.reject(new Error("e3")),
    ]);
    result.any_ok = r;

    try {
      await Promise.any([Promise.reject(new Error("a")), Promise.reject(new Error("b"))]);
      result.any_rej = "no-throw";
    } catch (e) {
      result.any_rej = {
        name: e.constructor.name,
        errors_len: e.errors ? e.errors.length : null,
      };
    }
  }

  // Promise.resolve / reject identity.
  {
    const p = Promise.resolve(42);
    result.resolve_passthrough = Promise.resolve(p) === p;
    result.resolved_value = await p;
  }

  // Promise.withResolvers.
  {
    if (typeof Promise.withResolvers === "function") {
      const { promise, resolve } = Promise.withResolvers();
      resolve("via-with-resolvers");
      result.with_resolvers = await promise;
    } else {
      result.with_resolvers = "absent";
    }
  }

  console.log(canon(result));
}

main().catch(e => { console.error("THREW:", e.message); process.exit(1); });
