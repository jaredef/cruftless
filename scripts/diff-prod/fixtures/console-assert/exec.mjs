// F-category: console method surface beyond log/warn/error.

function canon(v) {
  return JSON.stringify(v, (_k, v) => {
    if (v && typeof v === "object" && !Array.isArray(v)) {
      const out = {}; for (const k of Object.keys(v).sort()) out[k] = v[k]; return out;
    }
    return v;
  });
}

const result = {};

// console.assert: true condition produces no output; false prints.
// We test shape only (doesn't throw in browsers/node/bun).
{
  result.assert = {
    present: typeof console.assert === "function",
  };
  if (typeof console.assert === "function") {
    let threw = false;
    try { console.assert(true, "should not appear"); } catch { threw = true; }
    result.assert.true_no_throw = !threw;

    threw = false;
    try { console.assert(false, "expected assertion"); } catch { threw = true; }
    result.assert.false_no_throw = !threw;
  }
}

// console.count / console.countReset.
{
  result.count = {
    present: typeof console.count === "function",
    reset_present: typeof console.countReset === "function",
  };
}

// console.time / console.timeEnd / console.timeLog.
{
  result.time = {
    time_present: typeof console.time === "function",
    timeEnd_present: typeof console.timeEnd === "function",
    timeLog_present: typeof console.timeLog === "function",
  };
}

// console.dir.
{
  result.dir = {
    present: typeof console.dir === "function",
  };
}

// console.table.
{
  result.table = {
    present: typeof console.table === "function",
  };
}

// console.trace.
{
  result.trace = {
    present: typeof console.trace === "function",
  };
}

// console.group / console.groupEnd.
{
  result.group = {
    group_present: typeof console.group === "function",
    groupEnd_present: typeof console.groupEnd === "function",
    groupCollapsed_present: typeof console.groupCollapsed === "function",
  };
}

// console.clear.
{
  result.clear = {
    present: typeof console.clear === "function",
  };
}

console.log(canon(result));
