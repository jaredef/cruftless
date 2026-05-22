// F-category: Error subclass coverage.

function canon(v) {
  return JSON.stringify(v, (_k, v) => {
    if (v && typeof v === "object" && !Array.isArray(v)) {
      const out = {}; for (const k of Object.keys(v).sort()) out[k] = v[k]; return out;
    }
    return v;
  });
}

const result = {};

const ctors = { Error, TypeError, RangeError, SyntaxError, ReferenceError };

result.shapes = {};
for (const [name, C] of Object.entries(ctors)) {
  const e = new C("the message");
  result.shapes[name] = {
    name: e.name,
    message: e.message,
    inst_C: e instanceof C,
    inst_Error: e instanceof Error,
    has_stack: typeof e.stack === "string",
  };
}

// Catch by constructor.
{
  function catchAs(C, fn) {
    try { fn(); return "no-throw"; }
    catch (e) { return e instanceof C ? "ok" : `wrong:${e.constructor.name}`; }
  }
  result.catch = {
    type: catchAs(TypeError, () => { throw new TypeError("t"); }),
    range: catchAs(RangeError, () => { throw new RangeError("r"); }),
    base: catchAs(Error, () => { throw new TypeError("inherits"); }),  // TypeError instanceof Error
  };
}

// Error cause (ES2022).
{
  const inner = new Error("inner");
  const outer = new Error("outer", { cause: inner });
  result.cause = {
    has_cause_field: "cause" in outer,
    cause_message: outer.cause && outer.cause.message,
  };
}

// User Error subclass.
class MyErr extends Error {
  constructor(msg, code) { super(msg); this.code = code; this.name = "MyErr"; }
}
{
  try { throw new MyErr("oops", 42); }
  catch (e) {
    result.subclass = {
      name: e.name,
      message: e.message,
      code: e.code,
      inst_MyErr: e instanceof MyErr,
      inst_Error: e instanceof Error,
    };
  }
}

// toString.
{
  const e = new TypeError("bad");
  result.toString = String(e);   // "TypeError: bad"
}

console.log(canon(result));
