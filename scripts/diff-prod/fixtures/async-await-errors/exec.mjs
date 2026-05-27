// F-category: async/await error handling.

function canon(v) {
  return JSON.stringify(v, (_k, v) => {
    if (v && typeof v === "object" && !Array.isArray(v)) {
      const out = {}; for (const k of Object.keys(v).sort()) out[k] = v[k]; return out;
    }
    return v;
  });
}

const result = {};

async function throwsSync() { throw new Error("sync-throw"); }
try {
  await throwsSync();
  result.try_catch_sync_throw = "DID NOT THROW";
} catch (e) {
  result.try_catch_sync_throw = { msg: e.message, name: e.constructor.name };
}

async function rejectsPromise() { return Promise.reject(new Error("rejected")); }
try {
  await rejectsPromise();
  result.await_rejected = "DID NOT THROW";
} catch (e) {
  result.await_rejected = e.message;
}

result.await_number = await 42;
result.await_string = await "hello";
result.await_null = await null;
result.await_undefined = await undefined;
result.await_boolean = await true;

const thenable = { then(resolve) { resolve("thenable-value"); } };
result.await_thenable = await thenable;

const rejectThenable = { then(_resolve, reject) { reject(new Error("thenable-reject")); } };
try {
  await rejectThenable;
  result.await_thenable_reject = "DID NOT THROW";
} catch (e) {
  result.await_thenable_reject = e.message;
}

async function returnsValue() { return 99; }
const rv = returnsValue();
result.return_wrapping = {
  is_promise: rv instanceof Promise,
  value: await rv,
};

async function returnsPromise() { return Promise.resolve("inner"); }
result.return_promise_unwrap = await returnsPromise();

async function level3() { throw new Error("deep-error"); }
async function level2() { return await level3(); }
async function level1() { return await level2(); }
try {
  await level1();
  result.propagation_chain = "DID NOT THROW";
} catch (e) {
  result.propagation_chain = e.message;
}

async function catchAndRethrow() {
  try {
    throw new Error("original");
  } catch (e) {
    throw new Error("wrapped: " + e.message);
  }
}
try {
  await catchAndRethrow();
  result.catch_rethrow = "DID NOT THROW";
} catch (e) {
  result.catch_rethrow = e.message;
}

async function finallyRuns() {
  const log = [];
  try {
    log.push("try");
    throw new Error("err");
  } catch (e) {
    log.push("catch");
  } finally {
    log.push("finally");
  }
  return log;
}
result.finally_order = await finallyRuns();

async function catchReturns() {
  try {
    throw new Error("caught");
  } catch (e) {
    return "recovered: " + e.message;
  }
}
result.catch_returns = await catchReturns();

async function nestedAsync() {
  const inner = async () => { throw new Error("inner-err"); };
  try {
    await inner();
  } catch (e) {
    return e.message;
  }
}
result.nested_async = await nestedAsync();

async function multipleAwaits() {
  const a = await Promise.resolve(1);
  const b = await Promise.resolve(2);
  const c = await Promise.reject(new Error("third-fails")).catch(e => e.message);
  return [a, b, c];
}
result.multiple_awaits = await multipleAwaits();

console.log(canon(result));
