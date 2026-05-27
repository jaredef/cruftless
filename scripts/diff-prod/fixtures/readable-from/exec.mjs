// F-category: stream.Readable.from surface.

import { Readable } from "node:stream";

function canon(v) {
  return JSON.stringify(v, (_k, v) => {
    if (v && typeof v === "object" && !Array.isArray(v)) {
      const out = {}; for (const k of Object.keys(v).sort()) out[k] = v[k]; return out;
    }
    return v;
  });
}

const result = {};

// Readable.from presence.
{
  result.present = typeof Readable.from === "function";
}

// Readable.from with array.
if (typeof Readable.from === "function") {
  const chunks = [];
  const r = Readable.from(["hello", " ", "world"]);
  for await (const chunk of r) {
    chunks.push(String(chunk));
  }
  result.from_array = {
    joined: chunks.join(""),
    count: chunks.length,
  };
}

// Readable.from with generator.
if (typeof Readable.from === "function") {
  function* gen() {
    yield "a";
    yield "b";
    yield "c";
  }
  const chunks = [];
  const r = Readable.from(gen());
  for await (const chunk of r) {
    chunks.push(String(chunk));
  }
  result.from_generator = {
    joined: chunks.join(""),
    count: chunks.length,
  };
}

// Readable.from with async generator.
if (typeof Readable.from === "function") {
  async function* agen() {
    yield "x";
    yield "y";
  }
  const chunks = [];
  const r = Readable.from(agen());
  for await (const chunk of r) {
    chunks.push(String(chunk));
  }
  result.from_async_gen = {
    joined: chunks.join(""),
    count: chunks.length,
  };
}

// Readable instance checks.
if (typeof Readable.from === "function") {
  const r = Readable.from(["test"]);
  result.instance = {
    is_readable: r instanceof Readable,
    has_pipe: typeof r.pipe === "function",
    has_read: typeof r.read === "function",
  };
}

console.log(canon(result));
