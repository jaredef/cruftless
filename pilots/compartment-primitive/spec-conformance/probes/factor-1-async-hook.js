// CSC-EXT 8: async importHook (returns Promise of {source}).
let hookCalled = false;
const c = new Compartment({
  modules: {},
  importHook: async (spec) => {
    hookCalled = true;
    return {source: "export default 42"};
  }
});
c.import("async-spec").then(
  ns => {
    console.log("hook_called:", hookCalled);
    console.log("ns_default:", ns && ns.default);
    console.log("HOLDS:", hookCalled && ns && ns.default === 42);
  },
  err => {
    console.log("hook_called:", hookCalled);
    console.log("rejected:", err && err.message ? err.message.slice(0, 100) : err);
    console.log("REFUTED:", true);
  }
);
