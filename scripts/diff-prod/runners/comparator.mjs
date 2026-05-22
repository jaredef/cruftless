// Differential-prod comparator. Reads bun.json + cruftless.json from
// the per-fixture results dir; emits PASS/FAIL per enabled category.
// See specs/diff-prod-testing.md §V.
//
// Argv:  <fixture-name> <results-dir> <categories-space-sep>
// Exit:  0 = all enabled categories PASS; 1 = any FAIL.

import { readFileSync, writeFileSync } from "node:fs";

const [, , name, dir, catsArg] = process.argv;
const cats = (catsArg || "L").split(/\s+/).filter(Boolean);

const bun = JSON.parse(readFileSync(`${dir}/bun.json`, "utf8"));
const rb = JSON.parse(readFileSync(`${dir}/cruftless.json`, "utf8"));

const result = { fixture: name, categories: {}, overall: "PASS" };

function fail(cat, why, detail) {
  result.categories[cat] = { status: "FAIL", why, detail };
  result.overall = "FAIL";
}
function pass(cat, note) {
  result.categories[cat] = { status: "PASS", note: note || null };
}

function bunOut() { return bun.bun.stdout.trimEnd(); }
function rbOut()  { return rb.cruftless.stdout.trimEnd(); }
function bunErr() { return bun.bun.stderr.trimEnd(); }
function rbErr()  { return rb.cruftless.stderr.trimEnd(); }
function bunRc()  { return bun.bun.rc; }
function rbRc()   { return rb.cruftless.rc; }

// L-category: load-and-shape. The fixture must produce a JSON line with
// a `shape` field; comparator diffs the shape objects.
function compareL() {
  try {
    const b = JSON.parse(bunOut());
    const r = JSON.parse(rbOut());
    const bk = Object.keys(b.shape || {}).sort();
    const rk = Object.keys(r.shape || {}).sort();
    if (bk.length !== rk.length || bk.some((k, i) => k !== rk[i])) {
      return fail("L", "key-set mismatch", {
        bun_only: bk.filter(k => !rk.includes(k)),
        rb_only:  rk.filter(k => !bk.includes(k)),
      });
    }
    for (const k of bk) {
      if (b.shape[k] !== r.shape[k]) {
        return fail("L", `typeof mismatch on ${k}`, { bun: b.shape[k], rb: r.shape[k] });
      }
    }
    pass("L", `${bk.length} keys, shape identical`);
  } catch (e) {
    fail("L", "parse error", String(e));
  }
}

// F-category: pure-function output. Stdout must be a single
// JSON.stringify(result, canonicalizingReplacer). String-compare directly.
function compareF() {
  if (bunRc() !== rbRc()) return fail("F", "rc mismatch", { bun_rc: bunRc(), rb_rc: rbRc() });
  if (bunOut() !== rbOut()) {
    // Surface first diverging char position for quick triage.
    const b = bunOut(), r = rbOut();
    let i = 0;
    while (i < Math.min(b.length, r.length) && b[i] === r[i]) i++;
    return fail("F", "stdout mismatch", {
      diverge_at: i,
      bun_excerpt: b.slice(Math.max(0, i - 20), i + 80),
      rb_excerpt:  r.slice(Math.max(0, i - 20), i + 80),
    });
  }
  pass("F", `${bunOut().length} bytes identical`);
}

// E-category: error equivalence. If neither engine reports an error in
// stderr, this is a no-op pass. Otherwise compare (name, message-prefix).
function compareE() {
  const be = bunErr(), re = rbErr();
  if (!be && !re) { return pass("E", "no errors on either side"); }
  if (!be || !re) {
    return fail("E", "one engine errored, the other did not", { bun_err: be.slice(0, 200), rb_err: re.slice(0, 200) });
  }
  // Extract leading "Name: message" pattern; fall back to first line.
  const grab = (s) => {
    const line = (s.split("\n")[0] || "").trim();
    const m = line.match(/^(\w*Error)[:\s]+(.{0,32})/);
    if (m) return { name: m[1], prefix: m[2].trim() };
    return { name: "?", prefix: line.slice(0, 32) };
  };
  const bg = grab(be), rg = grab(re);
  if (bg.name !== rg.name) return fail("E", "error name mismatch", { bun: bg, rb: rg });
  if (!rg.prefix.startsWith(bg.prefix.slice(0, 16))) {
    return fail("E", "error message-prefix mismatch", { bun: bg, rb: rg });
  }
  pass("E", `both ${bg.name}, prefix matched`);
}

// S-category: side-effect trace. Stub for now; reads optional trace
// files emitted by the fixture's exec.mjs (e.g., recorder.jsonl).
function compareS() {
  pass("S", "stub — not yet implemented (Doc 736 capability-passing prereq)");
}

const dispatch = { L: compareL, F: compareF, E: compareE, S: compareS };
for (const c of cats) {
  if (dispatch[c]) dispatch[c]();
  else fail(c, `unknown category: ${c}`, null);
}

writeFileSync(`${dir}/result.json`, JSON.stringify(result, null, 2));
console.log(`${result.overall.padEnd(4)} ${name}`);
for (const [c, r] of Object.entries(result.categories)) {
  console.log(`  ${c}: ${r.status}${r.note ? ' — ' + r.note : ''}${r.why ? ' — ' + r.why : ''}`);
}
process.exit(result.overall === "PASS" ? 0 : 1);
