// §XV bracket probe for the §XVII iteration of the arktype deviation.
//
// Substrate pattern under test: the "explicit-return-from-derived-constructor
// + setPrototypeOf on a bound function" idiom used by @ark/util's Callable
// base class.
//
// class Callable {
//   constructor(fn, ...[opts]) {
//     return Object.assign(
//       Object.setPrototypeOf(fn.bind(opts?.bind ?? this), this.constructor.prototype),
//       opts?.attach
//     );
//   }
// }
// class BaseNode extends Callable { ... }
//
// Two ECMA-262 invariants combine:
//   1. §10.2.1 (Function [[Construct]]): if the constructor returns an
//      Object, that object IS the result of `new`, NOT the pre-allocated this.
//   2. §10.4.5 (bound function exotic objects): a bound function is itself
//      a callable function-typed object whose [[Prototype]] is settable.
//
// Bun (spec-compliant): the result of `new BaseNode(fn)` is a function-typed
// object whose proto is BaseNode.prototype. Calling its methods works.
//
// cruftless (current): the result is shaped as an Array(len=1) (per L7 trace),
// suggesting either (a) the explicit return is ignored, or (b) the bound-fn
// + setPrototypeOf chain doesn't compose, or (c) something further upstream.

class Base {
  constructor(fn) {
    return Object.setPrototypeOf(fn.bind(this), this.constructor.prototype);
  }
  ping() { return "pong"; }
}
class Sub extends Base {
  bark() { return "woof"; }
}

const inner = function inner(x) { return "inner(" + x + ")"; };
const s = new Sub(inner);

console.log("typeof s:", typeof s);
console.log("callable:", s(42));
console.log("instanceof Sub:", s instanceof Sub);
console.log("instanceof Base:", s instanceof Base);
console.log("instanceof Function:", s instanceof Function);
console.log("typeof s.ping:", typeof s.ping);
console.log("typeof s.bark:", typeof s.bark);
console.log("s.ping():", typeof s.ping === "function" ? s.ping() : "n/a");
console.log("s.bark():", typeof s.bark === "function" ? s.bark() : "n/a");
