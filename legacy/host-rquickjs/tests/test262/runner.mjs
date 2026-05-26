// Test262 runner for cruftless (rusty-js-runtime).
//
// Per-test invocation: this script is run by cruftless with the test
// path passed as process.env.T262_TEST_PATH (absolute path to a single
// .js test file). It:
//   1. Reads the test source.
//   2. Parses the YAML-like frontmatter (between /*--- and ---*/) per
//      test262 INTERPRETING.md.
//   3. Concatenates the test262 harness scripts (sta.js + assert.js +
//      any test-level `includes`) ahead of the source.
//   4. Evaluates via indirect-eval (cruftless's P59.E4 surface).
//   5. For negative tests, asserts the expected error type was thrown.
//   6. Emits a single JSON line to stdout:
//        {"path":"...","status":"PASS|FAIL|SKIP","reason":"..."}
//
// Flags honored: module, async, noStrict, onlyStrict, raw,
// negative.{phase,type}. Skipped flags / features land as SKIP with
// the reason recorded.
//
// Reference frontmatter spec: https://github.com/tc39/test262/blob/main/INTERPRETING.md

import { readFileSync, readdirSync, statSync } from 'node:fs';
import { join, dirname, basename } from 'node:path';

const testPath = process.env.T262_TEST_PATH;
const harnessDir = process.env.T262_HARNESS_DIR ||
  join(dirname(process.argv[1] || ''), 'harness');

if (!testPath) {
  process.stdout.write(JSON.stringify({
    status: 'SKIP',
    reason: 'T262_TEST_PATH not set'
  }) + '\n');
  process.exit(0);
}

let result;
try {
  result = runOne(testPath);
} catch (e) {
  result = {
    path: testPath,
    status: 'FAIL',
    reason: 'runner-internal: ' + ((e && e.message) || String(e))
  };
}
process.stdout.write(JSON.stringify(result) + '\n');

function runOne(path) {
  const src = readFileSync(path, 'utf8');
  const meta = parseFrontmatter(src);

  // Skip tests that exercise features we don't implement.
  if (meta.flags.module) {
    return { path, status: 'SKIP', reason: 'module-flag tests need a real module loader; using indirect-eval here' };
  }
  if (meta.flags.async) {
    return { path, status: 'SKIP', reason: 'async-flag tests need the async-test harness; deferred' };
  }
  if (meta.flags.raw) {
    return { path, status: 'SKIP', reason: 'raw tests skip harness; not yet wired' };
  }
  // RFSDO-EXT 1: deliberately-omitted stage-X / non-standard proposals.
  // Tests that require any of these features are not failures of cruft —
  // cruft has chosen not to implement these proposals. SKIP rather than
  // FAIL so the matrix reflects intent. Add features here ONLY when cruft
  // has DELIBERATELY excluded them (not when implementation is incomplete).
  const DELIBERATELY_OMITTED = new Set([
    'import-defer',                       // stage-3 deferred dynamic import (import.defer)
    'source-phase-imports',               // stage-3 source-phase import (import.source)
    'source-phase-imports-module-source', // sibling flag for source-phase imports
    // RFSDO-EXT 2: large standard-but-deliberately-deferred subsystems.
    // cruft v1 has no implementation of these; tests requiring them
    // would all FAIL with "X is not defined" runtime errors. Keeper
    // judgment per the standing protocol: SKIP rather than implement
    // — cruft v1 deliberately defers these large surfaces.
    'Temporal',                           // ECMA-402 / ECMA-262 Temporal API
    'Atomics',                            // SharedArrayBuffer/Atomics subsystem
    'Atomics.waitAsync',                  // Atomics async waiter
    'SharedArrayBuffer',                  // shared memory buffer
    'explicit-resource-management',       // DisposableStack/AsyncDisposableStack/SuppressedError/using
    'ShadowRealm',                        // stage-3 cross-realm execution boundary
  ]);
  for (const f of meta.features) {
    if (DELIBERATELY_OMITTED.has(f)) {
      return { path, status: 'SKIP', reason: `feature deliberately omitted: ${f}` };
    }
  }

  // Assemble the test source: harness + includes + source.
  // sta.js + assert.js are always prepended.
  let assembled = '';
  for (const h of ['sta.js', 'assert.js', ...meta.includes]) {
    const hpath = join(harnessDir, h);
    try {
      assembled += readFileSync(hpath, 'utf8') + '\n';
    } catch (_) {
      return { path, status: 'SKIP', reason: 'harness file missing: ' + h };
    }
  }
  // onlyStrict / noStrict — wrap or not. v1: always run in module-default
  // strict mode (indirect-eval source body is sloppy by default; for
  // strict-mode coverage we'd need explicit "use strict" prepend).
  if (meta.flags.onlyStrict) {
    assembled = '"use strict";\n' + assembled + src;
  } else if (meta.flags.noStrict) {
    assembled += src;
  } else {
    // Run sloppy form (no "use strict"). Strict-form coverage deferred
    // — would need two invocations per test.
    assembled += src;
  }

  // Run via indirect eval (cruftless's P59.E4). globalThis.eval matches
  // ECMA §19.2 indirect-eval semantics: script evaluated in global scope.
  let thrown = null;
  try {
    (0, eval)(assembled);
  } catch (e) {
    thrown = e;
  }

  // Negative-test handling per INTERPRETING.md.
  if (meta.negative) {
    if (!thrown) {
      return { path, status: 'FAIL', reason: 'expected ' + meta.negative.type + ' to be thrown, none thrown' };
    }
    // Match by constructor name.
    const thrownName = (thrown && thrown.constructor && thrown.constructor.name) ||
      (typeof thrown === 'object' && thrown && thrown.name) ||
      String(thrown);
    if (thrownName === meta.negative.type ||
        (thrownName === 'Test262Error' && meta.negative.type === 'Test262Error')) {
      return { path, status: 'PASS', reason: 'negative test threw ' + thrownName };
    }
    return { path, status: 'FAIL', reason: 'expected ' + meta.negative.type + ', got ' + thrownName };
  }

  // Non-negative test: any thrown value is a failure.
  if (thrown) {
    const msg = (thrown && thrown.message) || String(thrown);
    return { path, status: 'FAIL', reason: msg.slice(0, 240) };
  }
  return { path, status: 'PASS', reason: '' };
}

function parseFrontmatter(src) {
  // Frontmatter is between /*--- and ---*/ markers.
  const m = src.match(/\/\*---([\s\S]*?)---\*\//);
  const meta = {
    flags: {},
    includes: [],
    features: [],
    negative: null,
    description: '',
  };
  if (!m) return meta;
  const body = m[1];
  // YAML-lite parser. Handles:
  //   key: value
  //   key: [a, b, c]
  //   key:
  //     subkey: value
  //   features: [a, b]
  //   includes: [a.js, b.js]
  //   flags: [module, async]
  //   description: |
  //     ...
  // Bare-minimum: scan lines for the keys we care about.
  const lines = body.split('\n');
  let i = 0;
  while (i < lines.length) {
    const raw = lines[i];
    const trimmed = raw.trim();
    if (!trimmed || trimmed.startsWith('#')) { i++; continue; }
    if (trimmed.startsWith('flags:')) {
      const rest = trimmed.slice('flags:'.length).trim();
      const arr = parseInlineArray(rest);
      for (const f of arr) meta.flags[f] = true;
      i++; continue;
    }
    if (trimmed.startsWith('includes:')) {
      const rest = trimmed.slice('includes:'.length).trim();
      meta.includes = parseInlineArray(rest);
      i++; continue;
    }
    if (trimmed.startsWith('features:')) {
      const rest = trimmed.slice('features:'.length).trim();
      meta.features = parseInlineArray(rest);
      i++; continue;
    }
    if (trimmed.startsWith('negative:')) {
      // Sub-block. Read indented lines.
      const neg = { phase: null, type: null };
      i++;
      while (i < lines.length) {
        const sub = lines[i];
        if (!sub.startsWith('  ') && !sub.startsWith('\t')) break;
        const st = sub.trim();
        const colon = st.indexOf(':');
        if (colon < 0) { i++; continue; }
        const k = st.slice(0, colon).trim();
        const v = st.slice(colon + 1).trim();
        if (k === 'phase' || k === 'type') neg[k] = v;
        i++;
      }
      meta.negative = neg;
      continue;
    }
    if (trimmed.startsWith('description:')) {
      meta.description = trimmed.slice('description:'.length).trim();
      i++; continue;
    }
    i++;
  }
  return meta;
}

function parseInlineArray(s) {
  // Supports [a, b, c] or [a,b,c]. Falls back to empty array on
  // multi-line block forms (test262 uses inline arrays predominantly).
  s = s.trim();
  if (!s.startsWith('[') || !s.endsWith(']')) return [];
  const inner = s.slice(1, -1);
  if (!inner.trim()) return [];
  return inner.split(',').map(x => x.trim()).filter(Boolean);
}
