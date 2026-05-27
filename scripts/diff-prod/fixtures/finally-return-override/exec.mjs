// F-category: finally return override semantics.

function canon(v) {
  return JSON.stringify(v, (_k, v) => {
    if (v && typeof v === "object" && !Array.isArray(v)) {
      const out = {}; for (const k of Object.keys(v).sort()) out[k] = v[k]; return out;
    }
    return v;
  });
}

const result = {};

// Return in finally overrides return in try.
{
  function fn() {
    try { return "try"; }
    finally { return "finally"; }
  }
  result.override_try = fn();
}

// Return in finally overrides return in catch.
{
  function fn() {
    try { throw new Error("e"); }
    catch { return "catch"; }
    finally { return "finally"; }
  }
  result.override_catch = fn();
}

// Return in finally overrides throw in try.
{
  function fn() {
    try { throw new Error("thrown"); }
    finally { return "finally-wins"; }
  }
  result.override_throw = fn();
}

// Finally without return: try's return propagates.
{
  function fn() {
    let x = 0;
    try { return "try-val"; }
    finally { x = 1; }
  }
  result.passthrough = fn();
}

// Finally without return: throw propagates.
{
  function fn() {
    try { throw new Error("original"); }
    finally { /* no return, no throw */ }
  }
  let msg = null;
  try { fn(); } catch (e) { msg = e.message; }
  result.throw_passthrough = msg;
}

// Throw in finally overrides throw in try.
{
  function fn() {
    try { throw new Error("try-err"); }
    finally { throw new Error("finally-err"); }
  }
  let msg = null;
  try { fn(); } catch (e) { msg = e.message; }
  result.throw_override = msg;
}

// Throw in finally overrides return in try.
{
  function fn() {
    try { return "try-val"; }
    finally { throw new Error("finally-throws"); }
  }
  let msg = null;
  try { fn(); } catch (e) { msg = e.message; }
  result.throw_overrides_return = msg;
}

// Nested try-finally: inner finally, then outer finally.
{
  const log = [];
  function fn() {
    try {
      try { return "inner-try"; }
      finally { log.push("inner-finally"); }
    }
    finally { log.push("outer-finally"); }
  }
  const ret = fn();
  result.nested = { ret, log };
}

// Finally runs even on break in a loop.
{
  const log = [];
  for (let i = 0; i < 3; i++) {
    try {
      if (i === 1) break;
      log.push("iter-" + i);
    } finally {
      log.push("finally-" + i);
    }
  }
  result.break_finally = log;
}

// Finally runs on continue.
{
  const log = [];
  for (let i = 0; i < 3; i++) {
    try {
      if (i === 1) continue;
      log.push("body-" + i);
    } finally {
      log.push("finally-" + i);
    }
  }
  result.continue_finally = log;
}

// Return value: try returns an object, finally mutates it.
{
  function fn() {
    const obj = { x: 1 };
    try { return obj; }
    finally { obj.x = 99; }
  }
  result.mutate_in_finally = fn();
}

console.log(canon(result));
