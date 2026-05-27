// F-category: Automatic Semicolon Insertion (ECMA-262 §12.9).

function canon(v) {
  return JSON.stringify(v, (_k, v) => {
    if (v && typeof v === "object" && !Array.isArray(v)) {
      const out = {}; for (const k of Object.keys(v).sort()) out[k] = v[k]; return out;
    }
    return v;
  });
}

const result = {};

// return + newline: ASI inserts semicolon after return, value is undefined.
{
  function fn() {
    return
    42
  }
  result.return_newline = fn();
}

// return on same line: value propagates.
{
  function fn() {
    return 42
  }
  result.return_sameline = fn();
}

// return with object on next line: ASI before {, not treated as block.
{
  function fn() {
    return {
      x: 1
    }
  }
  result.return_object_nextline = fn();
}

// throw + newline: ASI does NOT apply (throw is restricted production).
{
  let caught = null;
  try {
    eval("throw\nnew Error('msg')")
  } catch (e) {
    caught = e.constructor ? e.constructor.name : String(e);
  }
  result.throw_newline = caught;
}

// ++/-- across newline: ASI applies, ++ starts new expression.
{
  let a = 1
  let b = 2
  const fn = new Function(`
    var a = 1
    var b = 2
    a
    ++b
    return { a: a, b: b }
  `);
  result.inc_newline = fn();
}

// No ASI between for-header tokens.
{
  let sum = 0
  for (let i = 0
    ; i < 3
    ; i++) {
    sum += i
  }
  result.for_header_multiline = sum;
}

// ASI at end of block (no semicolon before }).
{
  function fn() { return 1 }
  result.block_end = fn();
}

// ASI between two statements on same line is NOT inserted.
{
  const fn = new Function("var x = 1; var y = 2; return x + y");
  result.same_line_semi = fn();
}

// do-while ASI: semicolon after ) is auto-inserted.
{
  let count = 0
  do {
    count++
  } while (count < 3)
  result.do_while_asi = count;
}

// Multiline string expression (no ASI inside expression).
{
  const x = "a" +
    "b" +
    "c"
  result.multiline_expr = x;
}

// Array literal across lines (no ASI inside).
{
  const arr = [
    1,
    2,
    3
  ]
  result.array_multiline = arr;
}

// Chained method calls across lines (no ASI before .).
{
  const r = [1, 2, 3]
    .map(x => x * 2)
    .filter(x => x > 2)
  result.chained_method = r;
}

// Dangerous ASI case: line starting with ( treated as call.
{
  const fn = new Function(`
    var a = 1
    var b = (function() { return 2 })()
    return a + b
  `);
  result.paren_start = fn();
}

// Dangerous ASI case: line starting with [ treated as member access.
{
  const fn = new Function(`
    var a = 1
    var b = [2, 3][0]
    return a + b
  `);
  result.bracket_start = fn();
}

console.log(canon(result));
