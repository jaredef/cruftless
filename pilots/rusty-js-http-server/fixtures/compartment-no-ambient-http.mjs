const c = new Compartment();

console.log(`HS_COMPARTMENT_AMBIENT:${c.evaluate("typeof http")}:${c.evaluate("typeof require")}`);
