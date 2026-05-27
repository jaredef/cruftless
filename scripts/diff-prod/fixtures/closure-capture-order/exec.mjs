// F-category: closure capture semantics (ResetLocalCell + CaptureLocal/CaptureUpvalue).

function canon(v) {
  return JSON.stringify(v, (_k, v) => {
    if (v && typeof v === "object" && !Array.isArray(v)) {
      const out = {}; for (const k of Object.keys(v).sort()) out[k] = v[k]; return out;
    }
    return v;
  });
}

const result = {};

// Classic for-loop closure with let: each iteration fresh binding.
{
  const fns = [];
  for (let i = 0; i < 5; i++) {
    fns.push(() => i);
  }
  result.for_let = fns.map(f => f());
}

// for-of with let: each iteration fresh binding.
{
  const fns = [];
  for (const x of [10, 20, 30]) {
    fns.push(() => x);
  }
  result.for_of_const = fns.map(f => f());
}

// Shared closure over same variable: mutations visible.
{
  let counter = 0;
  const inc = () => ++counter;
  const get = () => counter;
  inc(); inc(); inc();
  result.shared_mutation = { get: get(), counter };
}

// Nested closure: transitive capture three levels deep.
{
  function outer() {
    let x = "outer";
    function middle() {
      function inner() { return x; }
      return inner;
    }
    return middle();
  }
  result.transitive = outer()();
}

// Closure captures variable, not value.
{
  let val = "before";
  const fn = () => val;
  val = "after";
  result.capture_variable = fn();
}

// IIFE captures snapshot at call time.
{
  const fns = [];
  for (var i = 0; i < 3; i++) {
    fns.push((function(captured) { return () => captured; })(i));
  }
  result.iife_snapshot = fns.map(f => f());
}

// Two closures sharing the same let binding.
{
  function make() {
    let shared = 0;
    return {
      inc: () => ++shared,
      get: () => shared,
    };
  }
  const pair = make();
  pair.inc();
  pair.inc();
  pair.inc();
  result.shared_cell = pair.get();
}

// Closure over function parameter.
{
  function make(x) {
    return () => x;
  }
  result.param_capture = make(42)();
}

// Closure over destructured parameter.
{
  function make({ a, b }) {
    return () => a + b;
  }
  result.destructure_capture = make({ a: 10, b: 20 })();
}

// Closure over default parameter.
{
  function make(x = 99) {
    return () => x;
  }
  result.default_capture = { with_arg: make(1)(), no_arg: make()() };
}

// Closure in array method callback.
{
  const multiplier = 3;
  const arr = [1, 2, 3, 4].map(x => x * multiplier);
  result.callback_capture = arr;
}

// Nested closures with same-name variables (shadowing).
{
  function outer() {
    let x = "outer";
    function inner() {
      let x = "inner";
      return () => x;
    }
    return { inner: inner()(), outer: (() => x)() };
  }
  result.shadow_capture = outer();
}

console.log(canon(result));
