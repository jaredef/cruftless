// F-category: Proxy trap coverage.

function canon(v) {
  return JSON.stringify(v, (_k, v) => {
    if (v && typeof v === "object" && !Array.isArray(v)) {
      const out = {}; for (const k of Object.keys(v).sort()) out[k] = v[k]; return out;
    }
    return v;
  });
}

const result = {};

// get trap: synthesize properties.
{
  const p = new Proxy({}, {
    get(_target, prop) { return `[${String(prop)}]`; },
  });
  result.get_trap = { foo: p.foo, bar: p.bar, num42: p[42] };
}

// set trap: log writes; pass through.
{
  const log = [];
  const obj = {};
  const p = new Proxy(obj, {
    set(target, prop, value) { log.push(`${String(prop)}=${value}`); target[prop] = value; return true; },
  });
  p.a = 1;
  p.b = 2;
  result.set_trap = { log, obj_a: obj.a, obj_b: obj.b };
}

// has trap: customize `in`.
{
  const p = new Proxy({}, {
    has(_target, prop) { return String(prop).startsWith("ok_"); },
  });
  result.has_trap = { ok_x: "ok_x" in p, bad_x: "bad_x" in p };
}

// deleteProperty trap.
{
  const log = [];
  const target = { x: 1, y: 2 };
  const p = new Proxy(target, {
    deleteProperty(t, prop) { log.push(String(prop)); delete t[prop]; return true; },
  });
  delete p.x;
  result.delete_trap = { log, target_keys: Object.keys(target).sort() };
}

// Reflect-based pass-through.
{
  const target = { a: 1, b: 2 };
  const p = new Proxy(target, {
    get(t, prop, recv) { return Reflect.get(t, prop, recv); },
    set(t, prop, value, recv) { return Reflect.set(t, prop, value, recv); },
  });
  p.c = 3;
  result.reflect_passthrough = { a: p.a, c: p.c, target_c: target.c };
}

// Default-trap (no handler) behavior — should pass through to target.
{
  const target = { x: 42 };
  const p = new Proxy(target, {});
  result.empty_handler = { x: p.x, ownKeys: Object.keys(p).sort() };
}

// Counter via closure-captured state.
{
  let calls = 0;
  const p = new Proxy({}, { get() { calls++; return calls; } });
  const a = p.x, b = p.y, c = p.z;
  result.closure_counter = { a, b, c, calls };
}

console.log(canon(result));
