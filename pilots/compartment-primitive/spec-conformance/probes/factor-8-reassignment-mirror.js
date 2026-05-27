// Factor 8: ES-EXT 2 v2 reassignment-mirror gap (ARC.M.7) inside compartment.
// var x = init; x = newVal; → globalThis.x should equal newVal, not init.
const c = new Compartment({});
c.evaluate("var x = 0; x = 7;");
const fromGT = c.globalThis.x;
console.log("globalThis.x:", fromGT);
console.log("REFUTED_IF_NOT_7:", fromGT !== 7);
