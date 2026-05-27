// F-category: super binding and new.target (PushNewTarget + PropagateNewTarget + SetThis).

function canon(v) {
  return JSON.stringify(v, (_k, v) => {
    if (v && typeof v === "object" && !Array.isArray(v)) {
      const out = {}; for (const k of Object.keys(v).sort()) out[k] = v[k]; return out;
    }
    return v;
  });
}

const result = {};

// super() in derived constructor.
{
  class Base {
    constructor() { this.base = true; }
  }
  class Child extends Base {
    constructor() { super(); this.child = true; }
  }
  const c = new Child();
  result.super_call = { base: c.base, child: c.child, is_child: c instanceof Child, is_base: c instanceof Base };
}

// super() with arguments.
{
  class Base {
    constructor(x, y) { this.sum = x + y; }
  }
  class Child extends Base {
    constructor() { super(10, 20); }
  }
  result.super_args = new Child().sum;
}

// super.method() in derived class.
{
  class Base {
    greet() { return "hello"; }
  }
  class Child extends Base {
    greet() { return super.greet() + " world"; }
  }
  result.super_method = new Child().greet();
}

// super.property access.
{
  class Base {
    get value() { return 42; }
  }
  class Child extends Base {
    getValue() { return super.value; }
  }
  result.super_property = new Child().getValue();
}

// super in static method.
{
  class Base {
    static hello() { return "base-static"; }
  }
  class Child extends Base {
    static hello() { return super.hello() + "-extended"; }
  }
  result.super_static = Child.hello();
}

// new.target in constructor (is the constructor being invoked).
{
  class A {
    constructor() { this.target_name = new.target.name; }
  }
  result.new_target_direct = new A().target_name;
}

// new.target in base constructor when called from derived.
{
  class Base {
    constructor() { this.target_name = new.target.name; }
  }
  class Derived extends Base {}
  result.new_target_derived = new Derived().target_name;
}

// new.target is undefined in regular function call.
{
  function fn() { return new.target; }
  result.new_target_regular = fn() === undefined;
}

// new.target in new Function() call.
{
  function fn() { return new.target === fn; }
  result.new_target_new = new fn();
}

// Arrow inherits new.target from enclosing.
{
  function Ctor() {
    this.arrow_target = (() => new.target)();
  }
  const obj = new Ctor();
  result.new_target_arrow = obj.arrow_target === Ctor;
}

// Multi-level inheritance: new.target is the most-derived constructor.
{
  class A {
    constructor() { this.target = new.target.name; }
  }
  class B extends A {}
  class C extends B {}
  result.new_target_chain = new C().target;
}

// super() must be called before this access in derived constructor.
{
  class Base {}
  class Bad extends Base {
    constructor() {
      let threw = false;
      try {
        void this;
      } catch (e) {
        threw = e.constructor.name;
      }
      super();
      return { threw, after_super: typeof this };
    }
  }
  result.this_before_super = new Bad();
}

// Constructor returning an object replaces this.
{
  class Base {
    constructor() { return { replaced: true }; }
  }
  class Child extends Base {
    constructor() { super(); }
  }
  const c = new Child();
  result.ctor_return_object = {
    replaced: c.replaced,
    not_child: !(c instanceof Child),
  };
}

console.log(canon(result));
