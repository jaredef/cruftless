// F-category: Proxy trap invariant enforcement (ECMA-262 §10.5).

function canon(v) {
  return JSON.stringify(v, (_k, v) => {
    if (v && typeof v === "object" && !Array.isArray(v)) {
      const out = {}; for (const k of Object.keys(v).sort()) out[k] = v[k]; return out;
    }
    return v;
  });
}

const result = {};

// [[Get]] invariant: if target property is non-configurable, non-writable data,
// the trap must return the target's value.
{
  const target = {};
  Object.defineProperty(target, "x", { value: 42, writable: false, configurable: false });
  const p = new Proxy(target, {
    get() { return 999; }
  });
  let threw = false;
  let err_name = null;
  let val = null;
  try { val = p.x; } catch (e) { threw = true; err_name = e.constructor.name; }
  result.get_nonconfig_nonwritable = { threw, err_name, val };
}

// [[Get]] invariant: non-configurable accessor with undefined getter must return undefined.
{
  const target = {};
  Object.defineProperty(target, "y", { get: undefined, set() {}, configurable: false });
  const p = new Proxy(target, {
    get() { return "lie"; }
  });
  let threw = false;
  let err_name = null;
  try { p.y; } catch (e) { threw = true; err_name = e.constructor.name; }
  result.get_nonconfig_no_getter = { threw, err_name };
}

// [[Set]] invariant: cannot set a non-configurable, non-writable data property
// to a different value.
{
  const target = {};
  Object.defineProperty(target, "z", { value: 10, writable: false, configurable: false });
  const p = new Proxy(target, {
    set() { return true; }
  });
  let threw = false;
  let err_name = null;
  try { p.z = 20; } catch (e) { threw = true; err_name = e.constructor.name; }
  result.set_nonconfig_nonwritable = { threw, err_name };
}

// [[Set]] invariant: cannot set a non-configurable accessor with undefined setter.
{
  const target = {};
  Object.defineProperty(target, "w", { get() { return 1; }, set: undefined, configurable: false });
  const p = new Proxy(target, {
    set() { return true; }
  });
  let threw = false;
  let err_name = null;
  try { p.w = 2; } catch (e) { threw = true; err_name = e.constructor.name; }
  result.set_nonconfig_no_setter = { threw, err_name };
}

// [[HasProperty]] invariant: cannot report non-configurable own property as non-existent.
{
  const target = {};
  Object.defineProperty(target, "a", { value: 1, configurable: false });
  const p = new Proxy(target, {
    has() { return false; }
  });
  let threw = false;
  let err_name = null;
  try { "a" in p; } catch (e) { threw = true; err_name = e.constructor.name; }
  result.has_nonconfig = { threw, err_name };
}

// [[HasProperty]] invariant: cannot report own property of non-extensible target as non-existent.
{
  const target = { b: 2 };
  Object.preventExtensions(target);
  const p = new Proxy(target, {
    has() { return false; }
  });
  let threw = false;
  let err_name = null;
  try { "b" in p; } catch (e) { threw = true; err_name = e.constructor.name; }
  result.has_nonextensible = { threw, err_name };
}

// [[Delete]] invariant: cannot delete a non-configurable property.
{
  const target = {};
  Object.defineProperty(target, "c", { value: 1, configurable: false });
  const p = new Proxy(target, {
    deleteProperty() { return true; }
  });
  let threw = false;
  let err_name = null;
  try { delete p.c; } catch (e) { threw = true; err_name = e.constructor.name; }
  result.delete_nonconfig = { threw, err_name };
}

// [[OwnPropertyKeys]] invariant: must include all non-configurable own keys.
{
  const target = {};
  Object.defineProperty(target, "fixed", { value: 1, configurable: false });
  target.flex = 2;
  const p = new Proxy(target, {
    ownKeys() { return ["flex"]; }
  });
  let threw = false;
  let err_name = null;
  try { Object.keys(p); } catch (e) { threw = true; err_name = e.constructor.name; }
  result.ownkeys_missing_nonconfig = { threw, err_name };
}

// [[OwnPropertyKeys]] invariant: non-extensible target — result must be exactly the target's own keys.
{
  const target = { a: 1, b: 2 };
  Object.preventExtensions(target);
  const p = new Proxy(target, {
    ownKeys() { return ["a", "b", "c"]; }
  });
  let threw = false;
  let err_name = null;
  try { Object.keys(p); } catch (e) { threw = true; err_name = e.constructor.name; }
  result.ownkeys_extra_nonextensible = { threw, err_name };
}

// [[DefineOwnProperty]] invariant: cannot define non-configurable on extensible target
// if the target doesn't have it (unless target is extensible — then it's ok).
// But: cannot make a property non-configurable if it's configurable on target.
{
  const target = {};
  Object.defineProperty(target, "d", { value: 1, configurable: true, writable: true });
  const p = new Proxy(target, {
    defineProperty() { return true; }
  });
  let threw = false;
  let err_name = null;
  try {
    Object.defineProperty(p, "d", { value: 2, configurable: false });
  } catch (e) { threw = true; err_name = e.constructor.name; }
  result.define_nonconfig_on_config = { threw, err_name };
}

// [[IsExtensible]] invariant: must agree with target.
{
  const target = {};
  Object.preventExtensions(target);
  const p = new Proxy(target, {
    isExtensible() { return true; }
  });
  let threw = false;
  let err_name = null;
  try { Object.isExtensible(p); } catch (e) { threw = true; err_name = e.constructor.name; }
  result.isextensible_disagree = { threw, err_name };
}

// [[PreventExtensions]] invariant: can only return true if target is actually non-extensible.
{
  const target = { x: 1 };
  const p = new Proxy(target, {
    preventExtensions() { return true; }
  });
  let threw = false;
  let err_name = null;
  try { Object.preventExtensions(p); } catch (e) { threw = true; err_name = e.constructor.name; }
  result.prevent_ext_lie = { threw, err_name };
}

// [[GetPrototypeOf]] invariant: non-extensible target — must return target's actual prototype.
{
  const target = Object.preventExtensions({});
  const p = new Proxy(target, {
    getPrototypeOf() { return Array.prototype; }
  });
  let threw = false;
  let err_name = null;
  try { Object.getPrototypeOf(p); } catch (e) { threw = true; err_name = e.constructor.name; }
  result.getproto_nonextensible = { threw, err_name };
}

console.log(canon(result));
