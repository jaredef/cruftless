// Factor 6: Error thrown inside compartment.evaluate should `instanceof Error`
// when caught by outer code (cross-realm brand check).
const c = new Compartment({});
let caught;
try { c.evaluate("throw new Error('x')"); } catch (e) { caught = e; }
console.log("caught_type:", typeof caught);
console.log("instanceof_Error:", caught instanceof Error);
console.log("REFUTED_IF_FALSE:", !(caught instanceof Error));
