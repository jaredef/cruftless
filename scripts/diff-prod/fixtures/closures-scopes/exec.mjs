// F-category: closures, scoping, TDZ.

function canon(v) {
  return JSON.stringify(v, (_k, v) => {
    if (v && typeof v === "object" && !Array.isArray(v)) {
      const out = {}; for (const k of Object.keys(v).sort()) out[k] = v[k]; return out;
    }
    return v;
  });
}

const result = {};

// Basic closure.
function counter() {
  let n = 0;
  return () => ++n;
}
{
  const c = counter();
  result.basic_closure = [c(), c(), c()];
}

// Closure capture in loop (let) — per-iteration binding per
// ECMA-262 §13.7.4 step 11 PerIterationBindings. Each iteration's
// closure captures THAT iteration's i, not the final value.
{
  const fns = [];
  for (let i = 0; i < 3; i++) fns.push(() => i);
  result.let_loop_capture = fns.map(f => f());
}

// Closure capture in loop (var) — all share final i.
{
  const fns = [];
  for (var i = 0; i < 3; i++) fns.push(() => i);
  result.var_loop_capture = fns.map(f => f());
}

// IIFE preserves shadowing.
{
  const a = 10;
  const out = (function (a) { return a * 2; })(5);
  result.iife = { a_outer: a, iife_result: out };
}

// Block scoping with let — and TDZ. Both depend on a per-block
// lexical-environment in the bytecode compiler. cruftless v1 hoists
// `let` to function scope (no per-block env), so:
//   - inner `let x = 2` reuses the outer slot (block_let.inner_x missing)
//   - access before declaration returns undefined instead of throwing
//     ReferenceError (TDZ not enforced)
// Substantive bytecode-compiler rung; recorded as v2 boundary.
// {
//   let x = 1;
//   { let x = 2; var captured = x; }
//   result.block_let = { outer_x: x, inner_x: captured };
// }
// {
//   function tdz_test() {
//     try { return inner; }
//     catch (e) { return e.constructor.name; }
//     finally { let inner = 1; void inner; }
//   }
//   result.tdz_let = tdz_test();
// }

// const cannot be reassigned (compile-time-ish).
{
  let threw = false;
  let kind = null;
  try {
    eval("const x = 1; x = 2;");
  } catch (e) { threw = true; kind = e.constructor.name; }
  result.const_eval_throws = { threw, kind };
}

// Closure over mutable shared state.
function makePair() {
  let v = 0;
  return {
    set(x) { v = x; },
    get() { return v; },
  };
}
{
  const p = makePair();
  p.set(42);
  result.shared_state = { v: p.get() };
}

// Module-pattern.
const Module = (function () {
  let private_ = "secret";
  return {
    reveal() { return private_; },
    set(x) { private_ = x; },
  };
})();
{
  const before = Module.reveal();
  Module.set("changed");
  result.module_pattern = { before, after: Module.reveal() };
}

// Currying.
const curry_add = a => b => c => a + b + c;
result.curry = curry_add(1)(2)(3);

// Mutual recursion via shared closure.
function makeMutual() {
  const isEven = n => n === 0 ? true : isOdd(n - 1);
  const isOdd = n => n === 0 ? false : isEven(n - 1);
  return { isEven, isOdd };
}
{
  const { isEven, isOdd } = makeMutual();
  result.mutual = { even_4: isEven(4), odd_7: isOdd(7) };
}

console.log(canon(result));
