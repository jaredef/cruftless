// F-category: node:events EventEmitter.

import { EventEmitter } from "node:events";

function canon(v) {
  return JSON.stringify(v, (_k, v) => {
    if (v && typeof v === "object" && !Array.isArray(v)) {
      const out = {}; for (const k of Object.keys(v).sort()) out[k] = v[k]; return out;
    }
    return v;
  });
}

const result = {};

// Basic on/emit.
{
  const ee = new EventEmitter();
  const calls = [];
  ee.on("greet", (name) => calls.push(`hello ${name}`));
  ee.emit("greet", "Alice");
  ee.emit("greet", "Bob");
  result.basic = calls;
}

// Multiple listeners.
{
  const ee = new EventEmitter();
  const calls = [];
  ee.on("evt", () => calls.push(1));
  ee.on("evt", () => calls.push(2));
  ee.on("evt", () => calls.push(3));
  ee.emit("evt");
  result.multi = calls;
}

// off removes listener.
{
  const ee = new EventEmitter();
  const calls = [];
  const fn = () => calls.push("first");
  ee.on("evt", fn);
  ee.emit("evt");
  ee.off("evt", fn);
  ee.emit("evt");
  result.off = calls;
}

// once fires only once.
{
  const ee = new EventEmitter();
  const calls = [];
  ee.once("evt", () => calls.push("once"));
  ee.emit("evt");
  ee.emit("evt");
  ee.emit("evt");
  result.once = calls;
}

// listenerCount / eventNames.
{
  const ee = new EventEmitter();
  ee.on("a", () => {});
  ee.on("a", () => {});
  ee.on("b", () => {});
  result.counts = {
    a: ee.listenerCount("a"),
    b: ee.listenerCount("b"),
    c: ee.listenerCount("c"),
    names: ee.eventNames().sort(),
  };
}

// removeAllListeners.
{
  const ee = new EventEmitter();
  ee.on("a", () => {});
  ee.on("b", () => {});
  ee.removeAllListeners("a");
  result.remove_all = {
    a_after: ee.listenerCount("a"),
    b_after: ee.listenerCount("b"),
  };
  ee.removeAllListeners();
  result.remove_all_full = {
    a: ee.listenerCount("a"),
    b: ee.listenerCount("b"),
    names: ee.eventNames(),
  };
}

// Subclass of EventEmitter.
class MyEmitter extends EventEmitter {
  ping() { this.emit("ping"); }
}
{
  const m = new MyEmitter();
  let count = 0;
  m.on("ping", () => count++);
  m.ping();
  m.ping();
  result.subclass = count;
}

console.log(canon(result));
