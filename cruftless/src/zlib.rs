//! node:zlib stub — Tier-Ω.5.y. Import-time + shape probe only;
//! every method throws a clear "not yet implemented" message.

use crate::register::{new_object, register_method};
use rusty_js_runtime::{Runtime, RuntimeError, Value};
use std::rc::Rc;

fn stub(name: &'static str) -> impl Fn(&mut Runtime, &[Value]) -> Result<Value, RuntimeError> {
    move |_rt, _args| {
        Err(RuntimeError::Thrown(Value::String(Rc::new(format!(
            "TypeError: node:zlib.{name} not yet implemented (Tier-Ω.5.y stub)"
        )))))
    }
}

pub fn install(rt: &mut Runtime) {
    let z = new_object(rt);
    for name in &[
        "deflate",
        "deflateSync",
        "deflateRaw",
        "deflateRawSync",
        "inflate",
        "inflateSync",
        "inflateRaw",
        "inflateRawSync",
        "gzip",
        "gzipSync",
        "gunzip",
        "gunzipSync",
        "brotliCompress",
        "brotliDecompress",
        "brotliCompressSync",
        "brotliDecompressSync",
        "createDeflate",
        "createInflate",
        "createGzip",
        "createGunzip",
        "createDeflateRaw",
        "createInflateRaw",
        "createBrotliCompress",
        "createBrotliDecompress",
    ] {
        register_method(rt, z, name, stub(name));
    }
    // Constructor placeholders for `util.inherits(X, zlib.Inflate)` and
    // `class X extends zlib.Inflate {}` patterns (pngjs Inflate, etc.).
    // Each is an Object with a `prototype` carrying a `constructor` backref;
    // util.inherits reads `super_.prototype` and that's the shape it needs.
    // Call/construct semantics are not wired — consumer code that actually
    // instantiates these will fail downstream, but module-load substrate
    // (the import-and-shape parity layer) only needs the slots to exist.
    for name in &[
        "Zlib",
        "Deflate",
        "Inflate",
        "DeflateRaw",
        "InflateRaw",
        "Gzip",
        "Gunzip",
        "BrotliCompress",
        "BrotliDecompress",
    ] {
        let ctor = new_object(rt);
        let proto = new_object(rt);
        rt.obj_mut(ctor)
            .set_own_frozen("prototype".into(), Value::Object(proto));
        rt.obj_mut(proto)
            .set_own_internal("constructor".into(), Value::Object(ctor));
        rt.object_set(z, name.to_string(), Value::Object(ctor));
    }
    // Ω.5.P51.E3: zlib.constants — flush flags, return codes, compression
    // levels, strategies, and the full Brotli operation/parameter set. These
    // are integer constants from zlib.h / Node's zlib bindings, read at
    // module-init by axios's http adapter (`flush: zlib.constants.Z_SYNC_FLUSH`,
    // `flush: zlib.constants.BROTLI_OPERATION_FLUSH`) and many other
    // compression-adjacent consumers (got, undici, etc.). Pure data,
    // no behavior — consumers that act on these values still hit the
    // method stubs above.
    let constants = new_object(rt);
    let pairs: &[(&str, f64)] = &[
        // Allowed flush values
        ("Z_NO_FLUSH", 0.0),
        ("Z_PARTIAL_FLUSH", 1.0),
        ("Z_SYNC_FLUSH", 2.0),
        ("Z_FULL_FLUSH", 3.0),
        ("Z_FINISH", 4.0),
        ("Z_BLOCK", 5.0),
        ("Z_TREES", 6.0),
        // Return codes
        ("Z_OK", 0.0),
        ("Z_STREAM_END", 1.0),
        ("Z_NEED_DICT", 2.0),
        ("Z_ERRNO", -1.0),
        ("Z_STREAM_ERROR", -2.0),
        ("Z_DATA_ERROR", -3.0),
        ("Z_MEM_ERROR", -4.0),
        ("Z_BUF_ERROR", -5.0),
        ("Z_VERSION_ERROR", -6.0),
        // Compression levels
        ("Z_NO_COMPRESSION", 0.0),
        ("Z_BEST_SPEED", 1.0),
        ("Z_BEST_COMPRESSION", 9.0),
        ("Z_DEFAULT_COMPRESSION", -1.0),
        // Compression strategies
        ("Z_FILTERED", 1.0),
        ("Z_HUFFMAN_ONLY", 2.0),
        ("Z_RLE", 3.0),
        ("Z_FIXED", 4.0),
        ("Z_DEFAULT_STRATEGY", 0.0),
        // Data type
        ("Z_BINARY", 0.0),
        ("Z_TEXT", 1.0),
        ("Z_ASCII", 1.0),
        ("Z_UNKNOWN", 2.0),
        ("Z_DEFLATED", 8.0),
        // Engine IDs
        ("DEFLATE", 1.0),
        ("INFLATE", 2.0),
        ("GZIP", 3.0),
        ("GUNZIP", 4.0),
        ("DEFLATERAW", 5.0),
        ("INFLATERAW", 6.0),
        ("UNZIP", 7.0),
        ("BROTLI_DECODE", 8.0),
        ("BROTLI_ENCODE", 9.0),
        // Default window/mem/level
        ("Z_DEFAULT_WINDOWBITS", 15.0),
        ("Z_MIN_WINDOWBITS", 8.0),
        ("Z_MAX_WINDOWBITS", 15.0),
        ("Z_MIN_CHUNK", 64.0),
        ("Z_MAX_CHUNK", f64::INFINITY),
        ("Z_DEFAULT_CHUNK", 16384.0),
        ("Z_MIN_MEMLEVEL", 1.0),
        ("Z_MAX_MEMLEVEL", 9.0),
        ("Z_DEFAULT_MEMLEVEL", 8.0),
        ("Z_MIN_LEVEL", -1.0),
        ("Z_MAX_LEVEL", 9.0),
        ("Z_DEFAULT_LEVEL", -1.0),
        // Brotli operations
        ("BROTLI_OPERATION_PROCESS", 0.0),
        ("BROTLI_OPERATION_FLUSH", 1.0),
        ("BROTLI_OPERATION_FINISH", 2.0),
        ("BROTLI_OPERATION_EMIT_METADATA", 3.0),
        // Brotli parameters
        ("BROTLI_PARAM_MODE", 0.0),
        ("BROTLI_MODE_GENERIC", 0.0),
        ("BROTLI_MODE_TEXT", 1.0),
        ("BROTLI_MODE_FONT", 2.0),
        ("BROTLI_DEFAULT_MODE", 0.0),
        ("BROTLI_PARAM_QUALITY", 1.0),
        ("BROTLI_MIN_QUALITY", 0.0),
        ("BROTLI_MAX_QUALITY", 11.0),
        ("BROTLI_DEFAULT_QUALITY", 11.0),
        ("BROTLI_PARAM_LGWIN", 2.0),
        ("BROTLI_MIN_WINDOW_BITS", 10.0),
        ("BROTLI_MAX_WINDOW_BITS", 24.0),
        ("BROTLI_LARGE_MAX_WINDOW_BITS", 30.0),
        ("BROTLI_DEFAULT_WINDOW", 22.0),
        ("BROTLI_PARAM_LGBLOCK", 3.0),
        ("BROTLI_MIN_INPUT_BLOCK_BITS", 16.0),
        ("BROTLI_MAX_INPUT_BLOCK_BITS", 24.0),
        ("BROTLI_PARAM_DISABLE_LITERAL_CONTEXT_MODELING", 4.0),
        ("BROTLI_PARAM_SIZE_HINT", 5.0),
        ("BROTLI_PARAM_LARGE_WINDOW", 6.0),
        ("BROTLI_PARAM_NPOSTFIX", 7.0),
        ("BROTLI_PARAM_NDIRECT", 8.0),
        // Brotli decoder
        ("BROTLI_DECODER_RESULT_ERROR", 0.0),
        ("BROTLI_DECODER_RESULT_SUCCESS", 1.0),
        ("BROTLI_DECODER_RESULT_NEEDS_MORE_INPUT", 2.0),
        ("BROTLI_DECODER_RESULT_NEEDS_MORE_OUTPUT", 3.0),
        ("BROTLI_DECODER_PARAM_DISABLE_RING_BUFFER_REALLOCATION", 0.0),
        ("BROTLI_DECODER_PARAM_LARGE_WINDOW", 1.0),
    ];
    for (name, val) in pairs {
        rt.object_set(constants, name.to_string(), Value::Number(*val));
    }
    rt.object_set(z, "constants".into(), Value::Object(constants));

    rt.define_global_property("zlib", Value::Object(z));
}
