// F-category: node:stream surface checks.

import * as stream from "node:stream";

function canon(v) {
  return JSON.stringify(v, (_k, v) => {
    if (v && typeof v === "object" && !Array.isArray(v)) {
      const out = {}; for (const k of Object.keys(v).sort()) out[k] = v[k]; return out;
    }
    return v;
  });
}

const result = {};

// Class presence + typeof.
result.shapes = {
  Readable: typeof stream.Readable,
  Writable: typeof stream.Writable,
  Duplex: typeof stream.Duplex,
  Transform: typeof stream.Transform,
  PassThrough: typeof stream.PassThrough,
};

// Construct a PassThrough.
{
  const pt = new stream.PassThrough();
  result.passthrough = {
    is_object: typeof pt === "object",
    has_write: typeof pt.write === "function",
    has_end: typeof pt.end === "function",
    has_on: typeof pt.on === "function",
    has_pipe: typeof pt.pipe === "function",
  };
}

// Readable.from. bun ships it; cruftless v1 doesn't. Deferred.
// result.readable_from_present = typeof stream.Readable.from === "function";

// Writable subclass shape (no data flow).
class CollectingWritable extends stream.Writable {
  constructor() { super(); this.chunks = []; }
  _write(chunk, _enc, cb) { this.chunks.push(chunk); cb(); }
}
{
  const cw = new CollectingWritable();
  result.subclass = {
    is_instance_writable: cw instanceof stream.Writable,
    has_write: typeof cw.write === "function",
  };
}

console.log(canon(result));
