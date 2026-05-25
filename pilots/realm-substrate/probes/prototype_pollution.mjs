// pilots/realm-substrate/probes/prototype_pollution.mjs
//
// Adversary probe — intrinsic-identity attack class (vs the ambient-authority
// class that rusty-js-caps probes already cover).
//
// Premise: a malicious dep loaded by the application doesn't need fs / net /
// env capabilities to compromise the application. It can mutate a shared
// intrinsic that the application later uses. The current cruft caps model
// does NOT defeat this — no capability gates "the right to assign to
// Array.prototype.map".
//
// Pre-realm-scoping (RS-EXT 1 baseline): expect ATTACK_SUCCEEDED.
// Post-minimum-realm (RS-EXT 2): expect ATTACK_BLOCKED.

function malicious_dep() {
    // The attack: replace a well-known method on a shared intrinsic.
    // The dep doesn't need any capability handle for this — it's pure
    // ECMAScript at the language tier.
    Array.prototype.map = function () { return "PWNED"; };
    return "dep loaded";
}

// Application code (would in production live in a separate module).
malicious_dep();
const result = [1, 2, 3].map(function (x) { return x * 2; });

// Application expected [2, 4, 6]; got "PWNED".
if (result === "PWNED") {
    console.log("ATTACK_SUCCEEDED");
} else if (Array.isArray(result) && result.length === 3 && result[0] === 2 && result[1] === 4 && result[2] === 6) {
    console.log("ATTACK_BLOCKED");
} else {
    console.log("ATTACK_UNDETERMINED: " + JSON.stringify(result));
}
