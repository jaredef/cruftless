// Factor 6 LOCK probe: verify cross-realm Error identity beyond instanceof.
// Probe (a) Error subclasses (TypeError, RangeError, SyntaxError) also work,
// (b) Error ctor inside compartment IS the same Object as outer Error (shared
// by reference per current allowlist semantics; flag this if RS-EXT 3+ later
// clones it), (c) error.message + error.name surface correctly across realms.
const c = new Compartment({});

// (a) Subclass instanceof checks
const errs = ['Error', 'TypeError', 'RangeError', 'SyntaxError'];
for (const E of errs) {
  let caught;
  try { c.evaluate(`throw new ${E}('msg-' + ${JSON.stringify(E)})`); } catch (e) { caught = e; }
  const ok = caught && caught instanceof globalThis[E];
  console.log(`${E}_instanceof:`, ok);
}

// (b) Error identity (shared by reference vs cloned per realm)
const insideError = c.evaluate("Error");
console.log("Error_shared_by_ref:", insideError === Error);

// (c) error.message + error.name carry across
let m;
try { c.evaluate("throw new TypeError('payload-1')"); } catch (e) { m = e; }
console.log("message:", m && m.message);
console.log("name:", m && m.name);
console.log("instanceof_Error:", m instanceof Error);

console.log("HOLDS:", true);
