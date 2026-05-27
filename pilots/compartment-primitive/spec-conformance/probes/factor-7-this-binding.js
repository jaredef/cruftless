// Factor 7: sloppy-mode top-level `this` inside compartment.evaluate
// should equal the compartment's globalThis.
const c = new Compartment({});
const sloppyThis = c.evaluate("this");
console.log("this_eq_globalThis:", sloppyThis === c.globalThis);
console.log("REFUTED_IF_NOT:", sloppyThis !== c.globalThis);
