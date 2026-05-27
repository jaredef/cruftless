// F-category: reference records and GetValue/PutValue semantics.

function canon(v) {
  return JSON.stringify(v, (_k, v) => {
    if (v && typeof v === "object" && !Array.isArray(v)) {
      const out = {}; for (const k of Object.keys(v).sort()) out[k] = v[k]; return out;
    }
    return v;
  });
}

const result = {};

// Unresolvable reference: ReferenceError on read (but not typeof).
{
  let threw = false;
  try { eval("undeclared_ref_xyz"); } catch (e) { threw = e.constructor.name; }
  result.unresolvable_read = threw;
  result.unresolvable_typeof = typeof undeclared_ref_xyz;
}

// Compound assignment to member expression: evaluates base once.
{
  let count = 0;
  const getObj = () => { count++; return obj; };
  const obj = { x: 10 };
  getObj().x += 5;
  result.compound_member = { x: obj.x, count };
}

// Compound assignment to computed member.
{
  const obj = { a: 1, b: 2 };
  const keys = [];
  const getKey = (k) => { keys.push(k); return k; };
  obj[getKey("a")] += 10;
  result.compound_computed = { a: obj.a, keys };
}

// Pre/post increment on member.
{
  const obj = { x: 5 };
  const pre = ++obj.x;
  const post = obj.x++;
  result.inc_member = { pre, post, final: obj.x };
}

// Pre/post increment on computed member.
{
  const arr = [10, 20, 30];
  const pre = ++arr[1];
  const post = arr[2]++;
  result.inc_computed = { pre, post, arr };
}

// this binding: method call vs extracted function.
{
  const obj = {
    x: 42,
    getX() { return this.x; },
  };
  const extracted = obj.getX;
  result.this_method = obj.getX();
  result.this_extracted = (() => { try { return extracted(); } catch { return "error"; } })();
}

// this binding: call/apply/bind.
{
  function fn() { return this?.x; }
  const obj = { x: 99 };
  result.this_call = fn.call(obj);
  result.this_apply = fn.apply(obj);
  result.this_bind = fn.bind(obj)();
}

// this in nested function (sloppy vs strict).
{
  const fn = new Function(`
    var self = this;
    function inner() { return this === self; }
    return inner();
  `);
  result.this_nested_sloppy = fn();

  function strictOuter() {
    "use strict";
    function inner() { return this; }
    return inner() === undefined;
  }
  result.this_nested_strict = strictOuter();
}

// Assignment to property of null/undefined: TypeError.
{
  let threw = false;
  let err_name = null;
  try { null.x = 1; } catch (e) { threw = true; err_name = e.constructor.name; }
  result.assign_null_prop = { threw, err_name };
}

// Property access on null/undefined: TypeError.
{
  let threw_null = false, threw_undef = false;
  try { null.x; } catch { threw_null = true; }
  try { undefined.x; } catch { threw_undef = true; }
  result.access_nullish = { null: threw_null, undef: threw_undef };
}

// Chained assignment.
{
  const a = {}, b = {};
  a.x = b.x = 42;
  result.chain_assign = { a_x: a.x, b_x: b.x };
}

// Destructuring assignment (not declaration).
{
  let a, b;
  ({ a, b } = { a: 10, b: 20, c: 30 });
  result.destructure_assign = { a, b };
}

// Destructuring with rest in assignment.
{
  let first, rest;
  [first, ...rest] = [1, 2, 3, 4, 5];
  result.destructure_rest_assign = { first, rest };
}

console.log(canon(result));
