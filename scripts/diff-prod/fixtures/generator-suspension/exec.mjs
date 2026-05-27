// F-category: generator suspension semantics (AST-to-bytecode boundary).

function canon(v) {
  return JSON.stringify(v, (_k, v) => {
    if (v && typeof v === "object" && !Array.isArray(v)) {
      const out = {}; for (const k of Object.keys(v).sort()) out[k] = v[k]; return out;
    }
    return v;
  });
}

const result = {};

// Return value surfaces on the terminal {done:true} step.
{
  function* g() { yield 1; yield 2; return "final"; }
  const gen = g();
  const steps = [];
  let step;
  do {
    step = gen.next();
    steps.push({ value: step.value, done: step.done });
  } while (!step.done);
  result.return_value = steps;
}

// Bidirectional send: next(val) delivers val as the yield result.
{
  function* g() {
    const a = yield "first";
    const b = yield "second";
    return [a, b];
  }
  const gen = g();
  const r1 = gen.next();
  const r2 = gen.next("sent-a");
  const r3 = gen.next("sent-b");
  result.send = {
    r1: { value: r1.value, done: r1.done },
    r2: { value: r2.value, done: r2.done },
    r3: { value: r3.value, done: r3.done },
  };
}

// Throw into suspended generator: caught inside.
{
  function* g() {
    try {
      yield 1;
      yield 2;
    } catch (e) {
      yield "caught:" + e.message;
    }
  }
  const gen = g();
  const r1 = gen.next();
  const r2 = gen.throw(new Error("oops"));
  const r3 = gen.next();
  result.throw_caught = {
    r1: { value: r1.value, done: r1.done },
    r2: { value: r2.value, done: r2.done },
    r3: { value: r3.value, done: r3.done },
  };
}

// Throw into generator without try/catch: propagates to caller.
{
  function* g() { yield 1; yield 2; }
  const gen = g();
  gen.next();
  let caught_msg = null;
  try {
    gen.throw(new Error("uncaught-in-gen"));
  } catch (e) {
    caught_msg = e.message;
  }
  result.throw_uncaught = { caught_msg };
}

// Side effects prove lazy evaluation (not eager collection).
{
  const log = [];
  function* g() {
    log.push("before-1");
    yield 1;
    log.push("before-2");
    yield 2;
    log.push("before-3");
    yield 3;
    log.push("after-all");
  }
  const gen = g();
  result.lazy_step0 = [...log];

  gen.next();
  result.lazy_step1 = [...log];

  gen.next();
  result.lazy_step2 = [...log];

  gen.next();
  result.lazy_step3 = [...log];

  gen.next();
  result.lazy_step4 = [...log];
}

// Generator.prototype.return() terminates the generator.
{
  function* g() { yield 1; yield 2; yield 3; }
  const gen = g();
  const r1 = gen.next();
  const ret = gen.return("early");
  const r2 = gen.next();
  result.return_method = {
    r1: { value: r1.value, done: r1.done },
    ret: { value: ret.value, done: ret.done },
    after: { value: r2.value, done: r2.done },
  };
}

// Infinite generator consumed partially.
{
  function* naturals() { let n = 0; while (true) yield n++; }
  const gen = naturals();
  const first5 = [];
  for (let i = 0; i < 5; i++) first5.push(gen.next().value);
  result.infinite_partial = first5;
}

// yield* delegation with return value.
{
  function* inner() { yield "a"; yield "b"; return "inner-done"; }
  function* outer() {
    const r = yield* inner();
    yield r;
  }
  result.delegation_return = [...outer()];
}

console.log(canon(result));
