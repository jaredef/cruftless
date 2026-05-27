// Factor 5: dynamic import inside compartment.evaluate should resolve via
// the compartment's modules-map (and hooks if any), NOT via the outer
// realm's module loader.
const c = new Compartment({modules: {}});
const r = c.evaluate("import('fs')");
if (r && typeof r.then === "function") {
  r.then(
    ns => { console.log("resolved_via_outer:", typeof ns); console.log("REFUTED_IF_RESOLVED:", true); },
    err => { console.log("rejected_correctly:", true); }
  );
} else {
  console.log("not_a_promise:", typeof r);
}
