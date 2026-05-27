// Factor 4: endowment property descriptors should match the spec shape
// (configurable; writable; enumerable per CreateDataPropertyOrThrow).
// Specifically check enumerability is consistent with primordial globalThis.
const c = new Compartment({globals: {x: 42}});
const xDesc = Object.getOwnPropertyDescriptor(c.globalThis, "x");
const arrayDesc = Object.getOwnPropertyDescriptor(c.globalThis, "Array");
console.log("endowment_x_enumerable:", xDesc && xDesc.enumerable);
console.log("intrinsic_Array_enumerable:", arrayDesc && arrayDesc.enumerable);
console.log("REFUTED_IF_INCONSISTENT:", xDesc && arrayDesc && xDesc.enumerable !== arrayDesc.enumerable);
