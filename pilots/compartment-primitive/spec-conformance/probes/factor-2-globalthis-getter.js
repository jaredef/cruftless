// Factor 2: Compartment.prototype.globalThis should be a getter on the prototype,
// not a per-instance data property.
const protoGT = Object.getOwnPropertyDescriptor(Compartment.prototype, "globalThis");
const instGT = Object.getOwnPropertyDescriptor(new Compartment({}), "globalThis");
console.log("proto_desc:", protoGT ? JSON.stringify({get: typeof protoGT.get, value: typeof protoGT.value}) : "MISSING");
console.log("inst_desc:", instGT ? JSON.stringify({get: typeof instGT.get, value: typeof instGT.value, enumerable: instGT.enumerable}) : "MISSING");
console.log("REFUTED_IF_INST_HAS_DATA:", instGT && instGT.value !== undefined);
