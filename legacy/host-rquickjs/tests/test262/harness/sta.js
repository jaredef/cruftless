// Standard Test Assertions per test262/harness/sta.js.
// Minimal vendored stand-in: matches upstream's exported surface
// (Test262Error class + assert + assert.sameValue + assert.notSameValue +
// assert.throws). Upstream also exposes $262 for host-specific access;
// not needed for the curated subset.
//
// Reference: https://github.com/tc39/test262/blob/main/harness/sta.js

function Test262Error(message) {
  this.message = message || "";
}
Test262Error.prototype.toString = function () {
  return "Test262Error: " + this.message;
};
Test262Error.thrower = function (message) {
  throw new Test262Error(message);
};

function $DONOTEVALUATE() {
  throw "Test262: This statement should not be evaluated.";
}
