// F-category: directive prologue and strict mode effects.
// .mjs is implicitly strict; we use new Function for sloppy-mode comparisons.

function canon(v) {
  return JSON.stringify(v, (_k, v) => {
    if (v && typeof v === "object" && !Array.isArray(v)) {
      const out = {}; for (const k of Object.keys(v).sort()) out[k] = v[k]; return out;
    }
    return v;
  });
}

const result = {};

// Strict mode this: undefined for plain calls.
{
  function strictFn() { "use strict"; return this; }
  result.strict_this = strictFn() === undefined;
}

// Sloppy mode this: coerced to globalThis.
{
  const sloppyFn = new Function("return this !== undefined && this !== null");
  result.sloppy_this_not_undefined = sloppyFn();
}

// Strict mode: assignment to undeclared throws ReferenceError.
{
  let threw = false;
  try {
    new Function('"use strict"; undeclared = 1')();
  } catch (e) {
    threw = e.constructor.name === "ReferenceError";
  }
  result.strict_undeclared = threw;
}

// Strict mode: delete of plain identifier fails.
{
  let threw = false;
  try {
    eval('"use strict"; var x = 1; delete x');
  } catch (e) {
    threw = true;
  }
  result.strict_delete_var = threw;
}

// Sloppy mode: delete of var returns false.
{
  const fn = new Function("var x = 1; return delete x");
  result.sloppy_delete_var = fn();
}

// Strict mode: duplicate property names in object literals (allowed since ES2015).
{
  const obj = { a: 1, a: 2 };
  result.duplicate_prop = obj.a;
}

// Strict mode: eval has its own scope.
{
  const fn = new Function('"use strict"; eval("var evalLocal = 42"); return typeof evalLocal');
  result.strict_eval_scope = fn();
}

// Sloppy mode: eval leaks var into enclosing scope.
{
  const fn = new Function('eval("var evalLocal = 42"); return typeof evalLocal !== "undefined" ? evalLocal : "not-found"');
  result.sloppy_eval_leak = fn();
}

// Strict mode: arguments object is not aliased to parameters.
{
  const fn = new Function('"use strict"; return function(a) { arguments[0] = 99; return a; }');
  result.strict_args_no_alias = fn()(42);
}

// Sloppy mode: arguments IS aliased to parameters.
{
  const fn = new Function('return function(a) { arguments[0] = 99; return a; }');
  result.sloppy_args_alias = fn()(42);
}

// Strict mode: writing to a non-writable property throws.
{
  let threw = false;
  try {
    new Function('"use strict"; var obj = {}; Object.defineProperty(obj, "x", {value: 1, writable: false}); obj.x = 2')();
  } catch { threw = true; }
  result.strict_nonwritable = threw;
}

// Module is implicitly strict (this at top level is undefined in ESM).
{
  result.module_strict = this === undefined;
}

// Directive must be a raw string literal, not a concatenation.
{
  const fn = new Function('"use " + "strict"; return this !== undefined');
  result.concat_not_directive = fn();
}

// Multiple directives.
{
  const fn = new Function('"use strict"; "use asm"; return typeof this');
  result.multiple_directives = fn();
}

console.log(canon(result));
