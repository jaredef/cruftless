// pilots/realm-substrate/probes/prototype_pollution_realm.mjs
//
// RS-EXT 2f: realm-scoped variant of the prototype-pollution probe.
//
// Pre-realm-scoping baseline (prototype_pollution.mjs): ATTACK_SUCCEEDED.
// With minimum-realm substrate via __cruftless_eval_realm: expect ATTACK_BLOCKED.
//
// The dep's mutation happens inside a fresh realm (with its own cloned
// Array.prototype). The application's Array.prototype is the primordial
// realm's, untouched.

// Run dep in isolated realm.
__cruftless_eval_realm('Array.prototype.map = function() { return "PWNED"; };');

// Application code uses primordial-realm Array.prototype.
const result = [1, 2, 3].map(function (x) { return x * 2; });

if (result === "PWNED") {
    console.log("ATTACK_SUCCEEDED");
} else if (Array.isArray(result) && result.length === 3 && result[0] === 2 && result[1] === 4 && result[2] === 6) {
    console.log("ATTACK_BLOCKED");
} else {
    console.log("ATTACK_UNDETERMINED: " + JSON.stringify(result));
}
