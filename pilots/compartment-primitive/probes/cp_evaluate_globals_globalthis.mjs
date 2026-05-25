// CP-EXT 1+2+3 probe: Compartment ctor + evaluate + globals + globalThis.
//
// Three checks rolled into one probe (per combined sub-locale):
// (1) ctor returns a Compartment instance
// (2) evaluate runs source in isolated realm (intrinsic pollution doesn't leak)
// (3) globals endowments visible inside the compartment
// (4) globalThis returns the compartment's globalThis object (distinct from outer)

const c = new Compartment({ globals: { x: 42, greet: function() { return "hi"; } } });

// (1) ctor sanity
if (typeof c !== "object") { console.log("FAIL_CTOR_TYPE: " + typeof c); throw "stop"; }

// (2) intrinsic isolation
c.evaluate('Array.prototype.map = function() { return "PWNED"; };');
const outerMap = [1,2,3].map(function(x) { return x * 2; });
if (outerMap === "PWNED" || !(Array.isArray(outerMap)) || outerMap.length !== 3) {
    console.log("FAIL_INTRINSIC_LEAK: " + JSON.stringify(outerMap));
    throw "stop";
}

// (3) endowments visible
const xResult = c.evaluate('x');
if (xResult !== 42) { console.log("FAIL_GLOBALS_X: " + xResult); throw "stop"; }
const greetResult = c.evaluate('greet()');
if (greetResult !== "hi") { console.log("FAIL_GLOBALS_GREET: " + greetResult); throw "stop"; }

// (4) globalThis distinct from outer + identity stable across evaluates
const gt1 = c.globalThis();
const gt2 = c.globalThis();
if (gt1 !== gt2) { console.log("FAIL_GT_IDENTITY_UNSTABLE"); throw "stop"; }
if (gt1 === globalThis) { console.log("FAIL_GT_EQUAL_OUTER"); throw "stop"; }

// (5) ambient bindings NOT in compartment (only endowments)
let ambient_visible_inside = false;
try {
    c.evaluate('typeof outsideOnlyVar');
} catch (e) {
    // expected — outsideOnlyVar isn't endowed
}
// Define an outer variable; should NOT be visible inside.
globalThis.outsideOnlyVar = "leak";
const ambient = c.evaluate('typeof outsideOnlyVar');
if (ambient !== "undefined") {
    console.log("FAIL_AMBIENT_LEAK: " + ambient + " (expected 'undefined' since outsideOnlyVar is not endowed)");
    throw "stop";
}

console.log("CP_EXT_123_OK");
