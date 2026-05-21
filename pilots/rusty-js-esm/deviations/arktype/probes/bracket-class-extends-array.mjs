// §XV bracket probe for the arktype deviation.
// Substrate primitive under test: `class X extends Array`. ECMA-262 requires
// `new X(...)` to produce an X instance (proto chain X→Array→Object), with
// X.prototype distinguishable from Array.prototype, and instanceof X true.
//
// Bun (and any spec-compliant engine): the subclass identity is preserved.
// cruftless (current): the subclass identity is lost; new X(...) yields a
// plain Array — Array.isArray true, but instanceof X false and
// constructor.name === "Array".
//
// arktype's Disjoint class (`class Disjoint extends Array`) is the
// downstream consumer; its static init() relies on `instanceof Disjoint`
// for control flow in the union-reduction pipeline.
//
// Expected output on a spec-compliant engine:
//   ctor: MySub
//   instanceof MySub: true
//   instanceof Array: true
//   isArray: true
//
// Current cruftless output:
//   ctor: Array
//   instanceof MySub: false
//   instanceof Array: true
//   isArray: true
//
// Flip criterion: cruftless trace matches bun trace exactly.

class MySub extends Array {
  static init() { return new MySub({ k: 1 }); }
  hello() { return "hi"; }
}
const x = MySub.init();
console.log("ctor:", x.constructor && x.constructor.name);
console.log("instanceof MySub:", x instanceof MySub);
console.log("instanceof Array:", x instanceof Array);
console.log("isArray:", Array.isArray(x));
console.log("typeof hello:", typeof x.hello);
