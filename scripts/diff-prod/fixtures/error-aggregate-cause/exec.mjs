// F-category: AggregateError and Error cause chaining.

function canon(v) {
  return JSON.stringify(v, (_k, v) => {
    if (v && typeof v === "object" && !Array.isArray(v)) {
      const out = {}; for (const k of Object.keys(v).sort()) out[k] = v[k]; return out;
    }
    return v;
  });
}

const result = {};

{
  const e = new AggregateError([new Error("a"), new Error("b")], "two errors");
  result.basic = {
    message: e.message,
    name: e.name,
    errors_count: e.errors.length,
    errors_messages: e.errors.map(x => x.message),
    is_error: e instanceof Error,
    is_aggregate: e instanceof AggregateError,
  };
}

{
  const e = new AggregateError([], "empty");
  result.empty_errors = {
    message: e.message,
    errors_count: e.errors.length,
    is_array: Array.isArray(e.errors),
  };
}

{
  const e = new AggregateError(["string-error", 42, null]);
  result.non_error_items = {
    message: e.message,
    errors: e.errors,
  };
}

{
  const e = new AggregateError([new Error("x")]);
  result.no_message = {
    message: e.message,
    errors_count: e.errors.length,
  };
}

{
  const inner = new Error("root cause");
  const outer = new Error("wrapper", { cause: inner });
  result.cause_basic = {
    message: outer.message,
    has_cause: "cause" in outer,
    cause_message: outer.cause.message,
    cause_is_error: outer.cause instanceof Error,
  };
}

{
  const e = new Error("no cause");
  result.no_cause = {
    message: e.message,
    has_cause: "cause" in e,
  };
}

{
  const e = new Error("with non-error cause", { cause: "string-cause" });
  result.cause_non_error = {
    message: e.message,
    cause: e.cause,
    cause_type: typeof e.cause,
  };
}

{
  const e = new Error("null cause", { cause: null });
  result.cause_null = {
    message: e.message,
    has_cause: "cause" in e,
    cause: e.cause,
  };
}

{
  const e = new Error("undefined cause", { cause: undefined });
  result.cause_undefined = {
    message: e.message,
    has_cause: "cause" in e,
    cause: e.cause,
  };
}

{
  const level0 = new Error("level-0");
  const level1 = new Error("level-1", { cause: level0 });
  const level2 = new Error("level-2", { cause: level1 });
  result.cause_chain = {
    msg2: level2.message,
    msg1: level2.cause.message,
    msg0: level2.cause.cause.message,
    depth: "cause" in level2.cause.cause ? "deeper" : "terminal",
  };
}

{
  const inner = new Error("inner");
  const agg = new AggregateError([new Error("e1")], "agg-with-cause", { cause: inner });
  result.aggregate_with_cause = {
    message: agg.message,
    has_cause: "cause" in agg,
    cause_message: agg.cause.message,
    errors_count: agg.errors.length,
  };
}

{
  const te = new TypeError("type-err", { cause: new RangeError("range-cause") });
  result.subclass_cause = {
    name: te.name,
    message: te.message,
    cause_name: te.cause.name,
    cause_message: te.cause.message,
    is_type: te instanceof TypeError,
    is_error: te instanceof Error,
  };
}

{
  const re = new RangeError("range", { cause: new Error("cause") });
  result.range_cause = {
    name: re.name,
    has_cause: "cause" in re,
    cause_msg: re.cause.message,
  };
}

{
  const se = new SyntaxError("syntax", { cause: 42 });
  result.syntax_cause_number = {
    name: se.name,
    cause: se.cause,
    cause_type: typeof se.cause,
  };
}

try {
  if (typeof SuppressedError === "function") {
    const e = new SuppressedError(new Error("suppressed"), new Error("error"), "msg");
    result.suppressed = {
      available: true,
      message: e.message,
      name: e.name,
      is_error: e instanceof Error,
      suppressed_msg: e.suppressed.message,
      error_msg: e.error.message,
    };
  } else {
    result.suppressed = { available: false };
  }
} catch (e) {
  result.suppressed = { available: "error", error: e.message };
}

{
  const e = new AggregateError([new Error("a"), new Error("b")], "test");
  const descriptor = Object.getOwnPropertyDescriptor(e, "errors");
  result.errors_descriptor = {
    writable: descriptor ? descriptor.writable : "no-descriptor",
    configurable: descriptor ? descriptor.configurable : "no-descriptor",
    enumerable: descriptor ? descriptor.enumerable : "no-descriptor",
  };
}

{
  const proto = Object.getPrototypeOf(AggregateError.prototype);
  result.prototype_chain = {
    proto_is_error_proto: proto === Error.prototype,
    agg_name: AggregateError.prototype.name,
  };
}

console.log(canon(result));
