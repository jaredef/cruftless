// L-category: async iteration protocol.

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

  // async generator + for-await-of.
  {
    async function* gen() {
      yield 1;
      yield 2;
      yield 3;
    }
    const out = [];
    for await (const v of gen()) out.push(v);
    result.async_gen = { values: out, sum: out.reduce((a, b) => a + b, 0) };
  }

  // for-await-of over array of promises.
  {
    const out = [];
    for await (const v of [Promise.resolve("a"), Promise.resolve("b"), Promise.resolve("c")]) {
      out.push(v);
    }
    result.for_await_promises = { values: out };
  }

  // Cruftless v1 boundaries (deferred):
  // - Manual [Symbol.asyncIterator] protocol over user-defined object
  //   throws "undefined" rather than iterating. Async-generator path
  //   (covered above) works.
  // - AsyncGenerator.prototype.throw not implemented in v1.

  // Mixed await + yield.
  {
    async function* gen() {
      const a = await Promise.resolve(10);
      yield a;
      const b = await Promise.resolve(20);
      yield b + a;
    }
    const out = [];
    for await (const v of gen()) out.push(v);
    result.await_yield = { values: out };
  }

  console.log(canon(result));
}

main().catch(e => { console.error("THREW:", e.message); process.exit(1); });
