// F-category: node:assert surface.

import assert from "node:assert";

function canon(v) {
  return JSON.stringify(v, (_k, v) => {
    if (v && typeof v === "object" && !Array.isArray(v)) {
      const out = {}; for (const k of Object.keys(v).sort()) out[k] = v[k]; return out;
    }
    return v;
  });
}

const result = {};

// assert.fail produces AssertionError, not a plain string.
{
  try {
    assert.fail("deliberate failure");
    result.fail = { threw: false };
  } catch (e) {
    result.fail = {
      threw: true,
      name: e.name,
      constructor_name: e.constructor.name,
      is_error: e instanceof Error,
      has_message: typeof e.message === "string",
      message_contains: e.message.includes("deliberate failure"),
    };
  }
}

// assert.fail with no arguments.
{
  try {
    assert.fail();
    result.fail_no_args = { threw: false };
  } catch (e) {
    result.fail_no_args = {
      threw: true,
      name: e.name,
      has_message: typeof e.message === "string",
    };
  }
}

// assert.ok.
{
  let ok_threw = false;
  try { assert.ok(true); } catch { ok_threw = true; }
  result.ok_true = { threw: ok_threw };

  try {
    assert.ok(false, "should be truthy");
    result.ok_false = { threw: false };
  } catch (e) {
    result.ok_false = {
      threw: true,
      name: e.name,
      is_error: e instanceof Error,
    };
  }
}

// assert.strictEqual.
{
  let eq_threw = false;
  try { assert.strictEqual(1, 1); } catch { eq_threw = true; }
  result.strict_equal_same = { threw: eq_threw };

  try {
    assert.strictEqual(1, "1");
    result.strict_equal_diff = { threw: false };
  } catch (e) {
    result.strict_equal_diff = {
      threw: true,
      name: e.name,
      is_error: e instanceof Error,
    };
  }
}

// assert.deepStrictEqual.
{
  let deep_threw = false;
  try { assert.deepStrictEqual({ a: 1, b: [2] }, { a: 1, b: [2] }); } catch { deep_threw = true; }
  result.deep_equal_same = { threw: deep_threw };

  try {
    assert.deepStrictEqual({ a: 1 }, { a: 2 });
    result.deep_equal_diff = { threw: false };
  } catch (e) {
    result.deep_equal_diff = {
      threw: true,
      name: e.name,
      is_error: e instanceof Error,
    };
  }
}

// assert.throws.
{
  let throws_ok = false;
  try {
    assert.throws(() => { throw new TypeError("boom"); }, TypeError);
    throws_ok = true;
  } catch { throws_ok = false; }
  result.throws_match = { passed: throws_ok };

  try {
    assert.throws(() => {}, Error);
    result.throws_no_throw = { threw: false };
  } catch (e) {
    result.throws_no_throw = {
      threw: true,
      name: e.name,
      is_error: e instanceof Error,
    };
  }
}

// assert module shape.
{
  result.shape = {
    has_fail: typeof assert.fail === "function",
    has_ok: typeof assert.ok === "function",
    has_strictEqual: typeof assert.strictEqual === "function",
    has_deepStrictEqual: typeof assert.deepStrictEqual === "function",
    has_throws: typeof assert.throws === "function",
    has_notStrictEqual: typeof assert.notStrictEqual === "function",
    has_rejects: typeof assert.rejects === "function",
  };
}

console.log(canon(result));
