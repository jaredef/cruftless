// F-category: Symbol.toPrimitive hint dispatch.

function canon(v) {
  return JSON.stringify(v, (_k, v) => {
    if (v && typeof v === "object" && !Array.isArray(v)) {
      const out = {}; for (const k of Object.keys(v).sort()) out[k] = v[k]; return out;
    }
    return v;
  });
}

const result = {};

// Basic Symbol.toPrimitive with all three hints.
{
  const obj = {
    [Symbol.toPrimitive](hint) {
      if (hint === "number") return 42;
      if (hint === "string") return "forty-two";
      return "default-42";
    }
  };

  result.basic = {
    number: +obj,
    string: `${obj}`,
    default: obj + "",
  };
}

// Hint values received by the callback.
{
  const hints = [];
  const obj = {
    [Symbol.toPrimitive](hint) {
      hints.push(hint);
      return 0;
    }
  };

  +obj;
  `${obj}`;
  obj + "";
  obj == 0;

  result.hint_values = hints;
}

// toPrimitive takes priority over valueOf and toString.
{
  const obj = {
    valueOf() { return "valueOf"; },
    toString() { return "toString"; },
    [Symbol.toPrimitive](hint) { return "toPrimitive:" + hint; },
  };

  result.priority = {
    number: +obj,
    string: `${obj}`,
    default: obj + "",
  };
}

// Without toPrimitive, valueOf is used for number hint.
{
  const obj = {
    valueOf() { return 100; },
    toString() { return "hundred"; },
  };

  result.fallback_valueof = {
    number: +obj,
    string: `${obj}`,
  };
}

// Date has a built-in toPrimitive that prefers string for default hint.
{
  const d = new Date(0);
  result.date = {
    has_toPrimitive: typeof d[Symbol.toPrimitive] === "function",
    number: +d,
    default_is_string: typeof (d + "") === "string",
  };
}

// toPrimitive returning an object should throw TypeError.
{
  const obj = {
    [Symbol.toPrimitive]() { return {}; }
  };
  let threw = false;
  let err_name = null;
  try { +obj; } catch (e) { threw = true; err_name = e.constructor.name; }
  result.object_return_throws = { threw, err_name };
}

// Comparison operators trigger default hint.
{
  const hints = [];
  const obj = {
    [Symbol.toPrimitive](hint) { hints.push(hint); return 5; }
  };
  obj < 10;
  obj > 0;
  obj == 5;
  result.comparison_hints = hints;
}

// toPrimitive on a class instance.
{
  class Money {
    constructor(amount, currency) {
      this.amount = amount;
      this.currency = currency;
    }
    [Symbol.toPrimitive](hint) {
      if (hint === "number") return this.amount;
      if (hint === "string") return `${this.amount} ${this.currency}`;
      return this.amount;
    }
  }

  const m = new Money(100, "USD");
  result.class_instance = {
    number: +m,
    string: `${m}`,
    default: m + 50,
  };
}

console.log(canon(result));
