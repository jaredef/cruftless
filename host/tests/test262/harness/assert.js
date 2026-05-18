// Test262 assert helpers per test262/harness/assert.js.
// Minimal vendored stand-in covering the assertions the curated test
// subset uses: assert, assert.sameValue, assert.notSameValue,
// assert.throws, assert.compareArray (also lives in compareArray.js
// upstream; folded in here for simplicity).
//
// Reference: https://github.com/tc39/test262/blob/main/harness/assert.js

function assert(mustBeTrue, message) {
  if (mustBeTrue === true) return;
  if (message === undefined) {
    message = "Expected true but got " + assert._toString(mustBeTrue);
  }
  throw new Test262Error(message);
}

assert._isSameValue = function (a, b) {
  if (a === b) {
    // Handle +0 vs -0.
    return a !== 0 || 1 / a === 1 / b;
  }
  // Handle NaN.
  return a !== a && b !== b;
};

assert.sameValue = function (actual, expected, message) {
  try {
    if (assert._isSameValue(actual, expected)) return;
  } catch (e) {
    throw new Test262Error(
      (message ? message + " " : "") +
      "Comparison error: " + e.message
    );
  }
  throw new Test262Error(
    (message ? message + " " : "") +
    "Expected SameValue(" +
    assert._toString(actual) + ", " + assert._toString(expected) +
    ") to be true"
  );
};

assert.notSameValue = function (actual, unexpected, message) {
  if (!assert._isSameValue(actual, unexpected)) return;
  throw new Test262Error(
    (message ? message + " " : "") +
    "Expected SameValue(" +
    assert._toString(actual) + ", " + assert._toString(unexpected) +
    ") to be false"
  );
};

assert.throws = function (expectedErrorConstructor, func, message) {
  if (typeof func !== "function") {
    throw new Test262Error(
      "assert.throws requires two arguments: the error constructor " +
      "and a function to run"
    );
  }
  message = message || "";
  try {
    func();
  } catch (thrown) {
    if (typeof thrown !== "object" || thrown === null) {
      throw new Test262Error(
        message + " Thrown value was not an object: " + String(thrown)
      );
    }
    if (thrown.constructor !== expectedErrorConstructor) {
      throw new Test262Error(
        message +
        " Expected a " + expectedErrorConstructor.name +
        " but got a " + (thrown.constructor && thrown.constructor.name)
      );
    }
    return;
  }
  throw new Test262Error(
    message + " Expected a " + expectedErrorConstructor.name +
    " to be thrown but no exception was thrown at all"
  );
};

assert.compareArray = function (actual, expected, message) {
  if (actual.length !== expected.length) {
    throw new Test262Error(
      (message ? message + " " : "") +
      "Array lengths differ: " + actual.length + " !== " + expected.length
    );
  }
  for (var i = 0; i < expected.length; i++) {
    if (!assert._isSameValue(actual[i], expected[i])) {
      throw new Test262Error(
        (message ? message + " " : "") +
        "Array element [" + i + "] differs: " +
        assert._toString(actual[i]) + " !== " + assert._toString(expected[i])
      );
    }
  }
};

assert._toString = function (v) {
  try {
    if (v === 0 && 1 / v === -Infinity) return "-0";
    return String(v);
  } catch (_) {
    return "<unprintable>";
  }
};
