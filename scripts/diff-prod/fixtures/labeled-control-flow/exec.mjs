// F-category: labeled control flow (Jump target resolution in compiler).

function canon(v) {
  return JSON.stringify(v, (_k, v) => {
    if (v && typeof v === "object" && !Array.isArray(v)) {
      const out = {}; for (const k of Object.keys(v).sort()) out[k] = v[k]; return out;
    }
    return v;
  });
}

const result = {};

// Labeled break from block.
{
  let reached = false;
  outer: {
    reached = true;
    break outer;
    reached = false;
  }
  result.block_break = reached;
}

// Labeled break from nested for loop.
{
  const log = [];
  outer: for (let i = 0; i < 3; i++) {
    for (let j = 0; j < 3; j++) {
      if (i === 1 && j === 1) break outer;
      log.push(`${i},${j}`);
    }
  }
  result.nested_break = log;
}

// Labeled continue to outer loop.
{
  const log = [];
  outer: for (let i = 0; i < 3; i++) {
    for (let j = 0; j < 3; j++) {
      if (j === 1) continue outer;
      log.push(`${i},${j}`);
    }
  }
  result.nested_continue = log;
}

// Label on while loop.
{
  let count = 0;
  loop: while (true) {
    count++;
    if (count >= 5) break loop;
  }
  result.while_label = count;
}

// Label on do-while.
{
  let count = 0;
  loop: do {
    count++;
    if (count >= 3) break loop;
  } while (true);
  result.do_while_label = count;
}

// Labeled break from switch inside loop.
{
  const log = [];
  loop: for (let i = 0; i < 3; i++) {
    switch (i) {
      case 1:
        log.push("break-loop");
        break loop;
      default:
        log.push("default-" + i);
    }
  }
  result.switch_in_loop = log;
}

// Break vs continue disambiguation: break breaks the label, continue continues the loop.
{
  const log = [];
  outer: for (let i = 0; i < 3; i++) {
    inner: for (let j = 0; j < 3; j++) {
      if (j === 1) { log.push("skip-" + i); continue inner; }
      if (j === 2 && i === 1) { log.push("break-outer"); break outer; }
      log.push(`${i},${j}`);
    }
  }
  result.break_continue_mix = log;
}

// Triple-nested with labels.
{
  const log = [];
  a: for (let i = 0; i < 2; i++) {
    b: for (let j = 0; j < 2; j++) {
      c: for (let k = 0; k < 2; k++) {
        if (k === 1) continue b;
        log.push(`${i}${j}${k}`);
      }
    }
  }
  result.triple_nested = log;
}

// for-of with labeled break.
{
  const log = [];
  outer: for (const x of [1, 2, 3]) {
    for (const y of [10, 20, 30]) {
      if (x === 2 && y === 20) break outer;
      log.push(x * y);
    }
  }
  result.for_of_labeled = log;
}

console.log(canon(result));
