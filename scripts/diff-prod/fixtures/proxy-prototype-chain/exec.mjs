// F-category: Proxy in prototype chain — receiver threading and MOP traversal.

function canon(v) {
  return JSON.stringify(v, (_k, v) => {
    if (v && typeof v === "object" && !Array.isArray(v)) {
      const out = {}; for (const k of Object.keys(v).sort()) out[k] = v[k]; return out;
    }
    return v;
  });
}

const result = {};

// Get trap in prototype: receiver is the original object, not the proxy.
{
  const log = [];
  const handler = {
    get(target, prop, receiver) {
      log.push({
        prop: String(prop),
        receiver_is_child: receiver === child,
        receiver_is_proxy: receiver === proto,
      });
      return Reflect.get(target, prop, receiver);
    }
  };
  const base = { x: 42 };
  const proto = new Proxy(base, handler);
  const child = Object.create(proto);
  const val = child.x;
  result.get_receiver = { val, log };
}

// Set trap in prototype: receiver is the child, set should create own property on child.
{
  const base = {};
  const log = [];
  const proto = new Proxy(base, {
    set(target, prop, value, receiver) {
      log.push({
        prop: String(prop),
        receiver_is_child: receiver === child,
      });
      return Reflect.set(target, prop, value, receiver);
    }
  });
  const child = Object.create(proto);
  child.y = 100;
  result.set_receiver = {
    child_own_y: Object.hasOwn(child, "y"),
    child_y: child.y,
    base_has_y: "y" in base,
    log,
  };
}

// has trap: in operator traverses prototype chain through proxy.
{
  const log = [];
  const base = { found: true };
  const proto = new Proxy(base, {
    has(target, prop) {
      log.push(String(prop));
      return Reflect.has(target, prop);
    }
  });
  const child = Object.create(proto);
  child.own = 1;
  result.has_traversal = {
    own_no_trap: "own" in child,
    found_via_proxy: "found" in child,
    missing: "nope" in child,
    log,
  };
}

// Multi-level proxy chain: proxy → proxy → base.
{
  const base = { deep: "value" };
  const log = [];
  const proxy1 = new Proxy(base, {
    get(t, p, r) { log.push("p1:" + String(p)); return Reflect.get(t, p, r); }
  });
  const proxy2 = new Proxy(Object.create(proxy1), {
    get(t, p, r) { log.push("p2:" + String(p)); return Reflect.get(t, p, r); }
  });
  const child = Object.create(proxy2);
  const val = child.deep;
  result.multi_proxy = { val, log };
}

// Proxy as prototype with getPrototypeOf trap.
{
  const base = {};
  const fakeProto = { marker: "fake" };
  const proto = new Proxy(base, {
    getPrototypeOf() { return fakeProto; }
  });
  result.getprotoof_trap = {
    proto_of_proxy: Object.getPrototypeOf(proto) === fakeProto,
    marker: Object.getPrototypeOf(proto).marker,
  };
}

// Property descriptor via proxy in chain.
{
  const base = {};
  Object.defineProperty(base, "desc", {
    get() { return "from-getter"; },
    enumerable: true,
    configurable: true,
  });
  const proto = new Proxy(base, {});
  const child = Object.create(proto);
  result.accessor_through_proxy = {
    val: child.desc,
  };
}

// instanceof with proxy wrapping a function (has [[Call]] slot).
{
  function Ctor() {}
  const proxyCtor = new Proxy(Ctor, {});
  const inst = new proxyCtor();
  result.instanceof_proxy_ctor = {
    is_instance: inst instanceof proxyCtor,
    is_instance_original: inst instanceof Ctor,
  };
}

// Proxy with apply trap as a prototype method.
{
  const handler = {
    apply(target, thisArg, args) {
      return target.apply(thisArg, args) + " (intercepted)";
    }
  };
  function greet(name) { return "hello " + name; }
  const proxyFn = new Proxy(greet, handler);

  const obj = { greet: proxyFn };
  result.apply_as_method = {
    direct: proxyFn("world"),
    as_method: obj.greet("world"),
  };
}

// Revocable proxy in prototype chain: revoke kills access.
{
  const base = { val: 42 };
  const { proxy, revoke } = Proxy.revocable(base, {});
  const child = Object.create(proxy);

  const before = child.val;
  revoke();
  let threw = false;
  let err_name = null;
  try { child.val; } catch (e) { threw = true; err_name = e.constructor.name; }
  result.revocable_chain = { before, threw, err_name };
}

console.log(canon(result));
