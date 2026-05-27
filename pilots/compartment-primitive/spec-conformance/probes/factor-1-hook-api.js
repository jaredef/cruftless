// Factor 1: Compartment ctor should accept importHook.
// CSC-EXT 7 supports SYNCHRONOUS importHook (returns {source}); ASYNC
// (returns Promise<{source}>) deferred to CSC-EXT 8.
let hookCalled = false;
const c = new Compartment({
  globals: {},
  modules: {},
  importHook: (spec) => { hookCalled = true; return {source: "export default 1"}; }
});
c.import("unregistered").then(
  ns => {
    console.log("hook_called:", hookCalled);
    console.log("ns_default:", ns && ns.default);
    console.log("REFUTED_IF_NOT_CALLED:", !hookCalled);
  },
  err => {
    console.log("hook_called:", hookCalled);
    console.log("rejected:", err && err.message ? err.message.slice(0, 80) : err);
    console.log("REFUTED_IF_NOT_CALLED:", !hookCalled);
  }
);
