// F-category: Function.prototype.bind, name, length surface.

function canon(v) {
  return JSON.stringify(v, (_k, v) => {
    if (v && typeof v === "object" && !Array.isArray(v)) {
      const out = {}; for (const k of Object.keys(v).sort()) out[k] = v[k]; return out;
    }
    return v;
  });
}

const result = {};

// bind creates a bound function.
{
  function greet(greeting, name) {
    return greeting + " " + name;
  }
  try {
    const bound = greet.bind(null, "Hello");
    result.bind_creates_function = typeof bound === "function";
    result.bind_call = bound("World");
  } catch (e) {
    result.bind_basic = e.constructor.name;
  }
}

// Bound function .name is "bound " + original name.
{
  function foo() {}
  try {
    const bf = foo.bind(null);
    result.bound_name = bf.name;
    result.bound_name_prefix = bf.name.startsWith("bound ");
  } catch (e) {
    result.bound_name = e.constructor.name;
  }
}

// Double bind name.
{
  function bar() {}
  try {
    const b1 = bar.bind(null);
    const b2 = b1.bind(null);
    result.double_bound_name = b2.name;
  } catch (e) {
    result.double_bound_name = e.constructor.name;
  }
}

// Bound function .length accounts for pre-filled args.
{
  function add(a, b, c) { return a + b + c; }
  try {
    result.orig_length = add.length;
    const b0 = add.bind(null);
    result.bind0_length = b0.length;
    const b1 = add.bind(null, 1);
    result.bind1_length = b1.length;
    const b2 = add.bind(null, 1, 2);
    result.bind2_length = b2.length;
    const b3 = add.bind(null, 1, 2, 3);
    result.bind3_length = b3.length;
    const b4 = add.bind(null, 1, 2, 3, 4);
    result.bind4_length = b4.length;
  } catch (e) {
    result.bind_length = e.constructor.name;
  }
}

// bind with thisArg.
{
  const obj = { x: 42 };
  function getX() { return this.x; }
  try {
    const bound = getX.bind(obj);
    result.bind_this = bound();
  } catch (e) {
    result.bind_this = e.constructor.name;
  }
}

// bind with pre-filled args (partial application).
{
  function multiply(a, b) { return a * b; }
  try {
    const double = multiply.bind(null, 2);
    result.partial_5 = double(5);
    result.partial_10 = double(10);
    result.partial_name = double.name;
    result.partial_length = double.length;
  } catch (e) {
    result.partial = e.constructor.name;
  }
}

// Arrow function name.
{
  const arrowFn = () => {};
  result.arrow_name = arrowFn.name;
}

// Anonymous function expression.
{
  const anon = function() {};
  result.named_expr_name = anon.name;
}

// Named function expression.
{
  const fn = function myName() {};
  result.named_fn_name = fn.name;
}

// Class method names.
{
  class MyClass {
    myMethod() {}
    static myStatic() {}
    get myGetter() { return 1; }
    set mySetter(v) {}
  }
  try {
    const proto = MyClass.prototype;
    result.method_name = proto.myMethod.name;
    result.static_name = MyClass.myStatic.name;
    const getterDesc = Object.getOwnPropertyDescriptor(proto, "myGetter");
    result.getter_name = getterDesc.get.name;
    const setterDesc = Object.getOwnPropertyDescriptor(proto, "mySetter");
    result.setter_name = setterDesc.set.name;
  } catch (e) {
    result.class_names = e.constructor.name;
  }
}

// Class name.
{
  class Foo {}
  result.class_name = Foo.name;
  const Bar = class {};
  result.class_expr_name = Bar.name;
  const Baz = class QuxName {};
  result.class_named_expr = Baz.name;
}

// Function.prototype.length with defaults (stops counting at first default).
{
  function f0() {}
  function f1(a) {}
  function f2(a, b) {}
  function f3(a, b = 1, c) {}
  function f4(a, ...rest) {}
  function f5(a, b = 1, c = 2) {}
  function f6({x}, [y], z) {}
  result.length_0 = f0.length;
  result.length_1 = f1.length;
  result.length_2 = f2.length;
  result.length_default = f3.length;
  result.length_rest = f4.length;
  result.length_all_default = f5.length;
  result.length_destructuring = f6.length;
}

// Function.prototype.toString basic shape.
{
  function hello(a, b) { return a + b; }
  try {
    const str = hello.toString();
    result.toString_starts_function = str.startsWith("function");
    result.toString_includes_name = str.includes("hello");
    result.toString_includes_body = str.includes("return");
  } catch (e) {
    result.toString = e.constructor.name;
  }
}

// Arrow toString.
{
  const arr = (x) => x + 1;
  try {
    const str = arr.toString();
    result.arrow_toString_has_arrow = str.includes("=>");
  } catch (e) {
    result.arrow_toString = e.constructor.name;
  }
}

// Bound function toString.
{
  function orig() {}
  try {
    const bf = orig.bind(null);
    const str = bf.toString();
    result.bound_toString_is_native = str.includes("native code");
  } catch (e) {
    result.bound_toString = e.constructor.name;
  }
}

// Computed property name.
{
  const sym = Symbol("mySymbol");
  const obj = { [sym]() {} };
  try {
    result.symbol_method_name = obj[sym].name;
  } catch (e) {
    result.symbol_method_name = e.constructor.name;
  }
}

console.log(canon(result));
