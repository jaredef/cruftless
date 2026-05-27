// F-category: microtask ordering.

function canon(v) {
  return JSON.stringify(v, (_k, v) => {
    if (v && typeof v === "object" && !Array.isArray(v)) {
      const out = {}; for (const k of Object.keys(v).sort()) out[k] = v[k]; return out;
    }
    return v;
  });
}

const result = {};

{
  const log = [];
  await new Promise(resolve => {
    log.push("sync1");
    Promise.resolve().then(() => log.push("micro1"));
    log.push("sync2");
    Promise.resolve().then(() => {
      log.push("micro2");
      resolve();
    });
  });
  result.basic_then_order = log;
}

{
  const log = [];
  await new Promise(resolve => {
    Promise.resolve().then(() => log.push("promise-then"));
    queueMicrotask(() => log.push("queueMicrotask"));
    Promise.resolve().then(() => {
      log.push("promise-then-2");
      resolve();
    });
  });
  result.queue_vs_promise = log;
}

{
  const log = [];
  await new Promise(resolve => {
    Promise.resolve()
      .then(() => { log.push("chain-1"); })
      .then(() => { log.push("chain-2"); })
      .then(() => { log.push("chain-3"); resolve(); });
  });
  result.nested_chain = log;
}

{
  const log = [];
  await new Promise(resolve => {
    let remaining = 2;
    const check = () => { if (--remaining === 0) resolve(); };
    Promise.resolve()
      .then(() => { log.push("A1"); })
      .then(() => { log.push("A2"); })
      .then(() => { log.push("A3"); check(); });
    Promise.resolve()
      .then(() => { log.push("B1"); })
      .then(() => { log.push("B2"); })
      .then(() => { log.push("B3"); check(); });
  });
  result.interleaved_chains = log;
}

{
  const log = [];
  await new Promise(resolve => {
    queueMicrotask(() => {
      log.push("outer-qm");
      queueMicrotask(() => {
        log.push("inner-qm");
        resolve();
      });
    });
    Promise.resolve().then(() => log.push("promise-after-qm"));
  });
  result.nested_queueMicrotask = log;
}

{
  const log = [];
  await new Promise(resolve => {
    Promise.resolve().then(() => {
      log.push("then-1");
      Promise.resolve().then(() => {
        log.push("nested-then");
        resolve();
      });
    });
    Promise.resolve().then(() => log.push("then-2"));
  });
  result.nested_then_scheduling = log;
}

{
  const log = [];
  await new Promise(resolve => {
    Promise.resolve().then(() => {
      log.push("micro-a");
      queueMicrotask(() => log.push("qm-from-micro"));
    });
    queueMicrotask(() => {
      log.push("qm-b");
      Promise.resolve().then(() => {
        log.push("micro-from-qm");
        resolve();
      });
    });
  });
  result.mixed_nesting = log;
}

{
  const log = [];
  await Promise.resolve()
    .then(() => log.push("flat-1"))
    .then(() => log.push("flat-2"))
    .then(() => log.push("flat-3"));
  result.flat_chain = log;
}

console.log(canon(result));
