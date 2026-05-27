// F-category: WeakRef and FinalizationRegistry (observable GC surface).

function canon(v) {
  return JSON.stringify(v, (_k, v) => {
    if (v && typeof v === "object" && !Array.isArray(v)) {
      const out = {}; for (const k of Object.keys(v).sort()) out[k] = v[k]; return out;
    }
    return v;
  });
}

const result = {};

// WeakRef presence and shape.
{
  result.weakref_shape = {
    present: typeof WeakRef === "function",
    proto_deref: typeof WeakRef === "function" ? typeof WeakRef.prototype.deref === "function" : false,
  };
}

// WeakRef.deref() returns target while reachable.
{
  const obj = { id: "alive" };
  const ref = new WeakRef(obj);
  result.deref_reachable = {
    value: ref.deref(),
    same_object: ref.deref() === obj,
  };
}

// WeakRef with different target types.
{
  const arr = [1, 2, 3];
  const fn = () => 42;
  const map = new Map();

  const refs = {
    arr: new WeakRef(arr),
    fn: new WeakRef(fn),
    map: new WeakRef(map),
  };

  result.target_types = {
    arr_alive: refs.arr.deref() === arr,
    fn_alive: refs.fn.deref() === fn,
    map_alive: refs.map.deref() === map,
  };
}

// WeakRef rejects non-object targets.
{
  const cases = {};
  for (const [name, val] of [["number", 42], ["string", "s"], ["boolean", true], ["null", null], ["undefined", undefined]]) {
    try {
      new WeakRef(val);
      cases[name] = "no-throw";
    } catch (e) {
      cases[name] = e.constructor.name;
    }
  }
  result.reject_primitives = cases;
}

// WeakRef with Symbol target (ES2023: symbols as WeakRef targets).
{
  const sym = Symbol("weak-target");
  let threw = false;
  let err_name = null;
  let alive = false;
  try {
    const ref = new WeakRef(sym);
    alive = ref.deref() === sym;
  } catch (e) {
    threw = true;
    err_name = e.constructor.name;
  }
  result.symbol_target = { threw, err_name, alive };
}

// FinalizationRegistry presence and shape.
{
  result.registry_shape = {
    present: typeof FinalizationRegistry === "function",
    proto_register: typeof FinalizationRegistry === "function"
      ? typeof FinalizationRegistry.prototype.register === "function" : false,
    proto_unregister: typeof FinalizationRegistry === "function"
      ? typeof FinalizationRegistry.prototype.unregister === "function" : false,
  };
}

// FinalizationRegistry: register does not throw for valid args.
if (typeof FinalizationRegistry === "function") {
  const log = [];
  const registry = new FinalizationRegistry((heldValue) => {
    log.push(heldValue);
  });

  const target = {};
  let threw = false;
  try {
    registry.register(target, "held-value", target);
  } catch { threw = true; }
  result.register_ok = { threw };
}

// FinalizationRegistry: register rejects non-object target.
if (typeof FinalizationRegistry === "function") {
  const registry = new FinalizationRegistry(() => {});
  let threw = false;
  let err_name = null;
  try {
    registry.register(42, "val");
  } catch (e) { threw = true; err_name = e.constructor.name; }
  result.register_reject_primitive = { threw, err_name };
}

// FinalizationRegistry: unregister returns boolean.
if (typeof FinalizationRegistry === "function") {
  const registry = new FinalizationRegistry(() => {});
  const token = {};
  const target = {};
  registry.register(target, "val", token);
  const removed = registry.unregister(token);
  const removed_again = registry.unregister(token);
  result.unregister = {
    first: removed,
    second: removed_again,
    first_type: typeof removed,
  };
}

// FinalizationRegistry: unregister with non-registered token.
if (typeof FinalizationRegistry === "function") {
  const registry = new FinalizationRegistry(() => {});
  const unrelated = {};
  const removed = registry.unregister(unrelated);
  result.unregister_unrelated = { removed };
}

// WeakRef constructor.name and prototype chain.
{
  result.weakref_identity = {
    name: WeakRef.name,
    proto_of_instance: Object.getPrototypeOf(new WeakRef({})) === WeakRef.prototype,
  };
}

// FinalizationRegistry constructor.name.
if (typeof FinalizationRegistry === "function") {
  result.registry_identity = {
    name: FinalizationRegistry.name,
  };
}

console.log(canon(result));
