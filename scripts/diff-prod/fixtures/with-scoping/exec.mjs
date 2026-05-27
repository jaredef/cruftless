// F-category: with-statement environment record lookup.
// .mjs is strict mode, so we use new Function() for sloppy-mode execution.

function canon(v) {
  return JSON.stringify(v, (_k, v) => {
    if (v && typeof v === "object" && !Array.isArray(v)) {
      const out = {}; for (const k of Object.keys(v).sort()) out[k] = v[k]; return out;
    }
    return v;
  });
}

const result = {};

// Basic with: property lookup in the with-object's scope.
{
  const fn = new Function("obj", `
    with (obj) {
      return x + y;
    }
  `);
  result.basic = fn({ x: 10, y: 20 });
}

// with shadows outer variables.
{
  const fn = new Function(`
    var x = 1;
    with ({ x: 99 }) {
      return x;
    }
  `);
  result.shadow = fn();
}

// with falls through to outer scope when property is missing.
{
  const fn = new Function(`
    var outer = "from-outer";
    with ({}) {
      return outer;
    }
  `);
  result.fallthrough = fn();
}

// with sees prototype chain of the binding object.
{
  const fn = new Function("obj", `
    with (obj) {
      return inherited;
    }
  `);
  const proto = { inherited: "from-proto" };
  const obj = Object.create(proto);
  result.prototype_chain = fn(obj);
}

// Assignment inside with: assigns to the with-object's property.
{
  const fn = new Function("obj", `
    with (obj) {
      x = 42;
    }
    return obj.x;
  `);
  const obj = { x: 0 };
  result.assign = fn(obj);
}

// Assignment inside with: new property goes to outer scope if not in with-object.
{
  const fn = new Function("obj", `
    with (obj) {
      newVar = "created";
    }
    return typeof newVar !== "undefined" ? newVar : "not-found";
  `);
  result.assign_fallthrough = fn({});
}

// Symbol.unscopables: properties listed are excluded from with binding.
{
  const fn = new Function("obj", `
    var copyWithin = "outer-copyWithin";
    with (obj) {
      return { resolved: typeof copyWithin === "string" ? copyWithin : "from-obj" };
    }
  `);
  result.unscopables_present = typeof Symbol.unscopables === "symbol";

  const obj = { copyWithin: "from-obj" };
  obj[Symbol.unscopables] = { copyWithin: true };
  result.unscopables = fn(obj);
}

// Symbol.unscopables on Array.prototype (real-world usage).
{
  const fn = new Function(`
    var values = "outer-values";
    with ([1, 2, 3]) {
      return values;
    }
  `);
  result.array_unscopables = fn();
}

// Nested with statements.
{
  const fn = new Function("a", "b", `
    with (a) {
      with (b) {
        return x + y;
      }
    }
  `);
  result.nested = fn({ x: 10 }, { y: 20 });
}

// Nested with: inner shadows outer.
{
  const fn = new Function("a", "b", `
    with (a) {
      with (b) {
        return x;
      }
    }
  `);
  result.nested_shadow = fn({ x: "outer" }, { x: "inner" });
}

// with + delete: deleting a property reveals outer scope.
{
  const fn = new Function("obj", `
    var x = "outer-x";
    with (obj) {
      var before = x;
      delete obj.x;
      var after = x;
      return { before: before, after: after };
    }
  `);
  result.delete_reveals_outer = fn({ x: "with-x" });
}

// with + Proxy: has trap is consulted for binding lookup.
{
  const fn = new Function("obj", `
    with (obj) {
      return x;
    }
  `);
  const log = [];
  const proxy = new Proxy({ x: 42 }, {
    has(target, prop) {
      log.push(String(prop));
      return prop in target;
    },
    get(target, prop) { return target[prop]; }
  });
  const val = fn(proxy);
  result.proxy_has_trap = {
    val,
    x_checked: log.includes("x"),
  };
}

console.log(canon(result));
