// F-category: iterator close/return protocol (ECMA-262 §7.4.7 IteratorClose).

function canon(v) {
  return JSON.stringify(v, (_k, v) => {
    if (v && typeof v === "object" && !Array.isArray(v)) {
      const out = {}; for (const k of Object.keys(v).sort()) out[k] = v[k]; return out;
    }
    return v;
  });
}

function makeTrackingIterable(values) {
  const log = [];
  const iterable = {
    [Symbol.iterator]() {
      let i = 0;
      return {
        next() {
          if (i < values.length) {
            log.push("next:" + values[i]);
            return { value: values[i++], done: false };
          }
          log.push("next:done");
          return { value: undefined, done: true };
        },
        return(val) {
          log.push("return");
          return { value: val, done: true };
        },
      };
    },
    get log() { return log; },
  };
  return iterable;
}

const result = {};

// for-of break: .return() must be called.
{
  const it = makeTrackingIterable([1, 2, 3, 4, 5]);
  for (const x of it) {
    if (x === 3) break;
  }
  result.forof_break = { log: it.log };
}

// for-of normal completion: .return() is NOT called.
{
  const it = makeTrackingIterable([1, 2]);
  for (const x of it) { /* consume all */ }
  result.forof_complete = { log: it.log };
}

// for-of throw: .return() must be called.
{
  const it = makeTrackingIterable([1, 2, 3]);
  let caught = null;
  try {
    for (const x of it) {
      if (x === 2) throw new Error("bail");
    }
  } catch (e) { caught = e.message; }
  result.forof_throw = { log: it.log, caught };
}

// for-of return: .return() must be called when function returns mid-loop.
{
  const it = makeTrackingIterable([1, 2, 3]);
  function fn() {
    for (const x of it) {
      if (x === 2) return "early";
    }
    return "done";
  }
  const ret = fn();
  result.forof_return = { log: it.log, ret };
}

// Destructuring: partial consumption calls .return().
{
  const it = makeTrackingIterable([10, 20, 30, 40]);
  const [a, b] = it;
  result.destructure_partial = { a, b, log: it.log };
}

// Destructuring: full consumption does NOT call .return().
{
  const it = makeTrackingIterable([10, 20]);
  const [a, b] = it;
  result.destructure_full = { a, b, log: it.log };
}

// Destructuring with rest: consumes all, no .return().
{
  const it = makeTrackingIterable([1, 2, 3]);
  const [first, ...rest] = it;
  result.destructure_rest = { first, rest, log: it.log };
}

// yield* delegation: break in outer closes inner.
{
  const innerLog = [];
  function* inner() {
    try {
      yield 1;
      yield 2;
      yield 3;
    } finally {
      innerLog.push("inner-finally");
    }
  }
  function* outer() {
    yield* inner();
  }
  const g = outer();
  g.next();
  g.next();
  g.return("done");
  result.yield_star_close = { innerLog };
}

// yield* delegation: throw in outer triggers inner's catch.
{
  const innerLog = [];
  function* inner() {
    try {
      yield 1;
      yield 2;
    } catch (e) {
      innerLog.push("caught:" + e.message);
    }
  }
  function* outer() {
    yield* inner();
  }
  const g = outer();
  g.next();
  g.throw(new Error("from-outer"));
  result.yield_star_throw = { innerLog };
}

// Array.from with close on throw in mapFn.
{
  const it = makeTrackingIterable([1, 2, 3, 4]);
  let caught = null;
  try {
    Array.from(it, (x) => {
      if (x === 2) throw new Error("map-bail");
      return x * 10;
    });
  } catch (e) { caught = e.message; }
  result.array_from_throw = { log: it.log, caught };
}

// .return() itself throws: the error propagates.
{
  const iterable = {
    [Symbol.iterator]() {
      let i = 0;
      return {
        next() {
          return i++ < 3 ? { value: i, done: false } : { value: undefined, done: true };
        },
        return() {
          throw new Error("return-threw");
        },
      };
    }
  };
  let caught = null;
  try {
    for (const x of iterable) {
      if (x === 2) break;
    }
  } catch (e) { caught = e.message; }
  result.return_throws = { caught };
}

console.log(canon(result));
