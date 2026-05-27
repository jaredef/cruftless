// Factor 3: __compartment_realm / __compartment_globalthis / __compartment_modules
// should NOT appear as own-enumerable properties on a Compartment instance.
const c = new Compartment({});
const keys = Object.keys(c);
const own = Object.getOwnPropertyNames(c);
console.log("keys:", JSON.stringify(keys));
console.log("own:", JSON.stringify(own));
console.log("REFUTED_IF_PRESENT:", keys.includes("__compartment_realm") || keys.includes("__compartment_globalthis") || keys.includes("__compartment_modules"));
