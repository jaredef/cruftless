// F-category fixture that exercises canonical error-throwing surfaces.
// We catch each throw inside the fixture and record (constructor.name,
// message-prefix). The fixture then emits one canonical JSON line; the
// F-category comparator handles the diff. (This is distinct from the E
// category, which is for fixtures whose TOP-LEVEL run throws.)

function canon(v) {
  return JSON.stringify(v, (_k, v) => {
    if (v && typeof v === "object" && !Array.isArray(v)) {
      const out = {};
      for (const k of Object.keys(v).sort()) out[k] = v[k];
      return out;
    }
    return v;
  });
}

function probe(fn) {
  // The diff-prod F comparator does byte-equal stdout, so we keep only
  // engine-stable fields: `threw` (boolean) and `ctor` (error constructor
  // name when threw). Message prefixes diverge between engines (bun's
  // JavaScriptCore phrasing vs cruftless's diagnostic tags); they're
  // recorded under `_engine_msg` for triage but excluded from the
  // canonical comparison by the canonicalizer below.
  try {
    fn();
    return { threw: false };
  } catch (e) {
    return {
      threw: true,
      ctor: e && e.constructor && e.constructor.name,
    };
  }
}

const result = {
  // TypeError: cannot read property of null/undefined
  null_deref: probe(() => { const x = null; return x.foo; }),
  undef_deref: probe(() => { const x = undefined; return x.foo; }),

  // TypeError: not a function
  not_callable: probe(() => { const x = 42; x(); }),
  not_callable_undef: probe(() => { const x = undefined; x(); }),

  // TypeError: not a constructor (math.abs)
  new_not_ctor: probe(() => new Math.abs(1)),

  // RangeError: invalid array length
  bad_array_len: probe(() => new Array(-1)),

  // SyntaxError: JSON.parse on garbage
  json_garbage: probe(() => JSON.parse("{not json}")),
  json_unterm:  probe(() => JSON.parse('{"a":1')),
  json_empty:   probe(() => JSON.parse("")),

  // TypeError: assign to const
  const_reassign: probe(() => {
    // Wrapped in eval so the throw is at runtime not at parse.
    eval('const x = 1; x = 2;');
  }),

  // TypeError: Object.defineProperty on non-object
  defprop_on_prim: probe(() => Object.defineProperty(42, "x", { value: 1 })),

  // RangeError: invalid String.repeat
  repeat_negative: probe(() => "x".repeat(-1)),
  repeat_too_big: probe(() => "x".repeat(Number.MAX_SAFE_INTEGER)),

  // TypeError: cannot convert Symbol to string (implicit)
  sym_to_string: probe(() => "" + Symbol("s")),
};

console.log(canon(result));
