// F-category: arrow functions.

function canon(v) {
  return JSON.stringify(v, (_k, v) => {
    if (v && typeof v === "object" && !Array.isArray(v)) {
      const out = {}; for (const k of Object.keys(v).sort()) out[k] = v[k]; return out;
    }
    return v;
  });
}

const result = {};

// Shapes.
result.shapes = {
  expr_body: ((x) => x * 2)(7),
  block_body: ((x) => { return x * 3; })(7),
  multi_param: ((a, b) => a + b)(3, 4),
  no_param: (() => 42)(),
  paren_obj_return: (() => ({ x: 1 }))(),
};

// Lexical this. Arrow's lexical-this binding is the surface tested;
// the detached_fn case (regular function detached from receiver) has
// surface variance across engines (sloppy/strict/global differences)
// and isn't worth pinning in F-category.
function Holder() {
  this.value = 100;
  this.read_arrow = () => this.value;
  this.read_fn = function () { return this.value; };
}
{
  const h = new Holder();
  const ra = h.read_arrow;
  result.lexical_this = {
    method_arrow: h.read_arrow(),
    method_fn: h.read_fn(),
    detached_arrow: ra(),                 // 100 — `this` lexically bound
  };
}

// Arrow callbacks see surrounding `this`.
{
  const obj = {
    n: 0,
    addAll(arr) { return arr.reduce((acc, x) => acc + x + this.n, 0); },
  };
  obj.n = 5;
  result.reduce_with_this = obj.addAll([1, 2, 3]);   // 6 + 3*5 = 21
}

// No own arguments — arrow inherits outer function's `arguments`.
// cruftless v1 doesn't expose `arguments` as a closure-captureable
// binding (it's installed as a synthesized local per function), so
// arrows can't reach outer arguments. Substantive bytecode-compiler
// rung; recorded as v2 boundary.
// {
//   function outer_args() { const arr = () => Array.from(arguments); return arr(); }
//   result.no_own_args = outer_args(1, 2, 3);   // bun:[1,2,3] rb:[]
// }

// Currying.
const curry = a => b => c => a * 100 + b * 10 + c;
result.curry = curry(1)(2)(3);

// Compose.
const compose = (...fns) => x => fns.reduceRight((acc, f) => f(acc), x);
const inc = x => x + 1;
const dbl = x => x * 2;
result.compose = compose(inc, dbl)(5);          // dbl(5) → 10, inc(10) → 11

// Sort with comparator.
result.sort_arrow = [3, 1, 4, 1, 5, 9, 2, 6].sort((a, b) => a - b);

// Map / filter / reduce.
result.map_filter_reduce = {
  mapped: [1, 2, 3].map(x => x * 10),
  filtered: [1, 2, 3, 4, 5].filter(x => x % 2 === 0),
  reduced: [1, 2, 3, 4, 5].reduce((acc, x) => acc + x, 0),
};

// Arrow inside class method captures `this`.
class Counter {
  constructor() { this.n = 0; }
  incBy(steps) {
    return steps.map(s => this.n + s);
  }
}
{
  const c = new Counter();
  c.n = 100;
  result.class_arrow_this = c.incBy([1, 2, 3]);
}

console.log(canon(result));
