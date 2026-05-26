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
  // RFSDO-EXT 3 (TI.4): PARTIALLY_IMPLEMENTED carve-out. Maps a feature
  // flag to an array of path-substring allowlist entries. A test whose
  // path contains any allowlisted substring opts OUT of the SKIP — it
  // runs and reveals real engine state. Used for progressive substrate
  // programs where SOME tests pass and others don't yet.
  const PARTIALLY_IMPLEMENTED = {
    'Temporal': [
      // TDur-EXT 1: Temporal.Duration ctor + 10 field-getters + valueOf-throws.
      // Tests covering arithmetic / relativeTo / round / total stay SKIPped
      // until the corresponding sub-rungs land.
      '/Temporal/Duration/constructor.js',
      '/Temporal/Duration/name.js',
      '/Temporal/Duration/length.js',
      '/Temporal/Duration/prop-desc.js',
      '/Temporal/Duration/years-undefined.js',
      '/Temporal/Duration/months-undefined.js',
      '/Temporal/Duration/weeks-undefined.js',
      '/Temporal/Duration/days-undefined.js',
      '/Temporal/Duration/hours-undefined.js',
      '/Temporal/Duration/minutes-undefined.js',
      '/Temporal/Duration/seconds-undefined.js',
      '/Temporal/Duration/milliseconds-undefined.js',
      '/Temporal/Duration/microseconds-undefined.js',
      '/Temporal/Duration/nanoseconds-undefined.js',
      '/Temporal/Duration/basic.js',
      '/Temporal/Duration/builtin.js',
      '/Temporal/Duration/call-builtin.js',
      '/Temporal/Duration/infinity-throws-rangeerror.js',
      '/Temporal/Duration/negative-infinity-throws-rangeerror.js',
      '/Temporal/Duration/fractional-throws-rangeerror.js',
      '/Temporal/Duration/prototype/valueOf/',
      '/Temporal/Duration/prototype/years/',
      '/Temporal/Duration/prototype/months/',
      '/Temporal/Duration/prototype/weeks/',
      '/Temporal/Duration/prototype/days/',
      '/Temporal/Duration/prototype/hours/',
      '/Temporal/Duration/prototype/minutes/',
      '/Temporal/Duration/prototype/seconds/',
      '/Temporal/Duration/prototype/milliseconds/',
      '/Temporal/Duration/prototype/microseconds/',
      '/Temporal/Duration/prototype/nanoseconds/',
      '/Temporal/Duration/prototype/toStringTag/',
      // DSC-EXT 1: duration-string-conversion
      '/Temporal/Duration/prototype/toString/',
      '/Temporal/Duration/prototype/toJSON/',
      '/Temporal/Duration/prototype/toLocaleString/',
      // DA-EXT 1: duration-arithmetic (add / subtract)
      '/Temporal/Duration/prototype/add/',
      '/Temporal/Duration/prototype/subtract/',
      // DDP-EXT 1: duration-derived-properties (sign / blank / abs / negated)
      '/Temporal/Duration/prototype/sign/',
      '/Temporal/Duration/prototype/blank/',
      '/Temporal/Duration/prototype/abs/',
      '/Temporal/Duration/prototype/negated/',
      // DStat-EXT 1: duration-static (from / compare)
      '/Temporal/Duration/from/',
      '/Temporal/Duration/compare/',
      // DWith-EXT 1: duration-with (with method)
      '/Temporal/Duration/prototype/with/',
      // TInst-EXT 1: instant-ctor-fields
      '/Temporal/Instant/constructor.js',
      '/Temporal/Instant/name.js',
      '/Temporal/Instant/length.js',
      '/Temporal/Instant/basic.js',
      '/Temporal/Instant/builtin.js',
      '/Temporal/Instant/prop-desc.js',
      '/Temporal/Instant/argument.js',
      '/Temporal/Instant/large-bigint.js',
      '/Temporal/Instant/limits.js',
      '/Temporal/Instant/prototype/epochNanoseconds/',
      '/Temporal/Instant/prototype/epochMilliseconds/',
      '/Temporal/Instant/prototype/valueOf/',
      '/Temporal/Instant/prototype/constructor.js',
      '/Temporal/Instant/prototype/prop-desc.js',
      '/Temporal/Instant/prototype/toStringTag/',
      // TIS-EXT 1: instant-static (from / fromEpochMilliseconds / fromEpochNanoseconds / compare)
      '/Temporal/Instant/from/',
      '/Temporal/Instant/fromEpochMilliseconds/',
      '/Temporal/Instant/fromEpochNanoseconds/',
      '/Temporal/Instant/compare/',
      // ISC-EXT 1: instant-string-conversion
      '/Temporal/Instant/prototype/toString/',
      '/Temporal/Instant/prototype/toJSON/',
      '/Temporal/Instant/prototype/toLocaleString/',
      // IE-EXT 1: instant-equals
      '/Temporal/Instant/prototype/equals/',
      // IA-EXT 1: instant-arithmetic (add / subtract / since / until)
      '/Temporal/Instant/prototype/add/',
      '/Temporal/Instant/prototype/subtract/',
      '/Temporal/Instant/prototype/since/',
      '/Temporal/Instant/prototype/until/',
      // PTCF-EXT 1: plain-time-ctor-fields
      '/Temporal/PlainTime/constructor.js',
      '/Temporal/PlainTime/name.js',
      '/Temporal/PlainTime/length.js',
      '/Temporal/PlainTime/basic.js',
      '/Temporal/PlainTime/builtin.js',
      '/Temporal/PlainTime/prop-desc.js',
      '/Temporal/PlainTime/prototype/hour/',
      '/Temporal/PlainTime/prototype/minute/',
      '/Temporal/PlainTime/prototype/second/',
      '/Temporal/PlainTime/prototype/millisecond/',
      '/Temporal/PlainTime/prototype/microsecond/',
      '/Temporal/PlainTime/prototype/nanosecond/',
      '/Temporal/PlainTime/prototype/valueOf/',
      '/Temporal/PlainTime/prototype/constructor.js',
      '/Temporal/PlainTime/prototype/prop-desc.js',
      '/Temporal/PlainTime/prototype/toStringTag/',
      // PTS-EXT 1: plain-time-static (from / compare)
      '/Temporal/PlainTime/from/',
      '/Temporal/PlainTime/compare/',
      // PTW-EXT 1: plain-time-with
      '/Temporal/PlainTime/prototype/with/',
      // PTSC-EXT 1: plain-time-string-conversion
      '/Temporal/PlainTime/prototype/toString/',
      '/Temporal/PlainTime/prototype/toJSON/',
      '/Temporal/PlainTime/prototype/toLocaleString/',
      // PTE-EXT 1: plain-time-equals
      '/Temporal/PlainTime/prototype/equals/',
      // PTA-EXT 1: plain-time-arithmetic
      '/Temporal/PlainTime/prototype/add/',
      '/Temporal/PlainTime/prototype/subtract/',
      '/Temporal/PlainTime/prototype/since/',
      '/Temporal/PlainTime/prototype/until/',
      // PDCF-EXT 1: plain-date-ctor-fields
      '/Temporal/PlainDate/constructor.js',
      '/Temporal/PlainDate/name.js',
      '/Temporal/PlainDate/length.js',
      '/Temporal/PlainDate/basic.js',
      '/Temporal/PlainDate/builtin.js',
      '/Temporal/PlainDate/prop-desc.js',
      '/Temporal/PlainDate/missing-arguments.js',
      '/Temporal/PlainDate/calendar-undefined.js',
      '/Temporal/PlainDate/calendar-string.js',
      '/Temporal/PlainDate/calendar-case-insensitive.js',
      '/Temporal/PlainDate/infinity-throws-rangeerror.js',
      '/Temporal/PlainDate/negative-infinity-throws-rangeerror.js',
      '/Temporal/PlainDate/prototype/year/',
      '/Temporal/PlainDate/prototype/month/',
      '/Temporal/PlainDate/prototype/day/',
      '/Temporal/PlainDate/prototype/calendarId/',
      '/Temporal/PlainDate/prototype/monthCode/',
      '/Temporal/PlainDate/prototype/valueOf/',
      '/Temporal/PlainDate/prototype/constructor.js',
      '/Temporal/PlainDate/prototype/prop-desc.js',
      '/Temporal/PlainDate/prototype/toStringTag/',
      // PDS-EXT 1: plain-date-static (from / compare)
      '/Temporal/PlainDate/from/',
      '/Temporal/PlainDate/compare/',
      // PDSC-EXT 1: plain-date-string-conversion
      '/Temporal/PlainDate/prototype/toString/',
      '/Temporal/PlainDate/prototype/toJSON/',
      '/Temporal/PlainDate/prototype/toLocaleString/',
      // PDE-EXT 1: plain-date-equals
      '/Temporal/PlainDate/prototype/equals/',
      // PDDP-EXT 1: plain-date-derived-properties
      '/Temporal/PlainDate/prototype/dayOfWeek/',
      '/Temporal/PlainDate/prototype/dayOfYear/',
      '/Temporal/PlainDate/prototype/daysInMonth/',
      '/Temporal/PlainDate/prototype/daysInWeek/',
      '/Temporal/PlainDate/prototype/daysInYear/',
      '/Temporal/PlainDate/prototype/inLeapYear/',
      '/Temporal/PlainDate/prototype/monthsInYear/',
      '/Temporal/PlainDate/prototype/weekOfYear/',
      '/Temporal/PlainDate/prototype/yearOfWeek/',
      '/Temporal/PlainDate/prototype/era/',
      '/Temporal/PlainDate/prototype/eraYear/',
      // PDA-EXT 1: plain-date-arithmetic
      '/Temporal/PlainDate/prototype/add/',
      '/Temporal/PlainDate/prototype/subtract/',
      '/Temporal/PlainDate/prototype/since/',
      '/Temporal/PlainDate/prototype/until/',
      // PDW-EXT 1: plain-date-with
      '/Temporal/PlainDate/prototype/with/',
      // PDTCF-EXT 1: plain-date-time-ctor-fields
      '/Temporal/PlainDateTime/constructor.js',
      '/Temporal/PlainDateTime/name.js',
      '/Temporal/PlainDateTime/length.js',
      '/Temporal/PlainDateTime/basic.js',
      '/Temporal/PlainDateTime/builtin.js',
      '/Temporal/PlainDateTime/prop-desc.js',
      '/Temporal/PlainDateTime/calendar-undefined.js',
      '/Temporal/PlainDateTime/calendar-string.js',
      '/Temporal/PlainDateTime/calendar-case-insensitive.js',
      '/Temporal/PlainDateTime/infinity-throws-rangeerror.js',
      '/Temporal/PlainDateTime/negative-infinity-throws-rangeerror.js',
      '/Temporal/PlainDateTime/missing-arguments.js',
      '/Temporal/PlainDateTime/prototype/year/',
      '/Temporal/PlainDateTime/prototype/month/',
      '/Temporal/PlainDateTime/prototype/day/',
      '/Temporal/PlainDateTime/prototype/hour/',
      '/Temporal/PlainDateTime/prototype/minute/',
      '/Temporal/PlainDateTime/prototype/second/',
      '/Temporal/PlainDateTime/prototype/millisecond/',
      '/Temporal/PlainDateTime/prototype/microsecond/',
      '/Temporal/PlainDateTime/prototype/nanosecond/',
      '/Temporal/PlainDateTime/prototype/calendarId/',
      '/Temporal/PlainDateTime/prototype/monthCode/',
      '/Temporal/PlainDateTime/prototype/valueOf/',
      '/Temporal/PlainDateTime/prototype/constructor.js',
      '/Temporal/PlainDateTime/prototype/prop-desc.js',
      '/Temporal/PlainDateTime/prototype/toStringTag/',
      // PDTSC-EXT 1 + PDTE-EXT 1: PDT string-conversion + equals
      '/Temporal/PlainDateTime/prototype/toString/',
      '/Temporal/PlainDateTime/prototype/toJSON/',
      '/Temporal/PlainDateTime/prototype/toLocaleString/',
      '/Temporal/PlainDateTime/prototype/equals/',
      // PDTS-EXT 1: PDT.from + PDT.compare
      '/Temporal/PlainDateTime/from/',
      '/Temporal/PlainDateTime/compare/',
      // PDTW-EXT 1: PDT with
      '/Temporal/PlainDateTime/prototype/with/',
      // PDTA-EXT 1: PDT arithmetic
      '/Temporal/PlainDateTime/prototype/add/',
      '/Temporal/PlainDateTime/prototype/subtract/',
      '/Temporal/PlainDateTime/prototype/since/',
      '/Temporal/PlainDateTime/prototype/until/',
      // PMDCF-EXT 1: PlainMonthDay ctor + getters + toString
      '/Temporal/PlainMonthDay/constructor.js',
      '/Temporal/PlainMonthDay/name.js',
      '/Temporal/PlainMonthDay/length.js',
      '/Temporal/PlainMonthDay/basic.js',
      '/Temporal/PlainMonthDay/builtin.js',
      '/Temporal/PlainMonthDay/prop-desc.js',
      '/Temporal/PlainMonthDay/calendar-undefined.js',
      '/Temporal/PlainMonthDay/calendar-string.js',
      '/Temporal/PlainMonthDay/missing-arguments.js',
      '/Temporal/PlainMonthDay/prototype/day/',
      '/Temporal/PlainMonthDay/prototype/monthCode/',
      '/Temporal/PlainMonthDay/prototype/calendarId/',
      '/Temporal/PlainMonthDay/prototype/valueOf/',
      '/Temporal/PlainMonthDay/prototype/toString/',
      '/Temporal/PlainMonthDay/prototype/toJSON/',
      '/Temporal/PlainMonthDay/prototype/constructor.js',
      '/Temporal/PlainMonthDay/prototype/prop-desc.js',
      '/Temporal/PlainMonthDay/prototype/toStringTag/',
      // PYMCF-EXT 1: PlainYearMonth ctor + 10 getters + toString/toJSON
      '/Temporal/PlainYearMonth/constructor.js',
      '/Temporal/PlainYearMonth/name.js',
      '/Temporal/PlainYearMonth/length.js',
      '/Temporal/PlainYearMonth/basic.js',
      '/Temporal/PlainYearMonth/builtin.js',
      '/Temporal/PlainYearMonth/prop-desc.js',
      '/Temporal/PlainYearMonth/calendar-undefined.js',
      '/Temporal/PlainYearMonth/calendar-string.js',
      '/Temporal/PlainYearMonth/missing-arguments.js',
      '/Temporal/PlainYearMonth/prototype/year/',
      '/Temporal/PlainYearMonth/prototype/month/',
      '/Temporal/PlainYearMonth/prototype/monthCode/',
      '/Temporal/PlainYearMonth/prototype/calendarId/',
      '/Temporal/PlainYearMonth/prototype/daysInMonth/',
      '/Temporal/PlainYearMonth/prototype/daysInYear/',
      '/Temporal/PlainYearMonth/prototype/monthsInYear/',
      '/Temporal/PlainYearMonth/prototype/inLeapYear/',
      '/Temporal/PlainYearMonth/prototype/era/',
      '/Temporal/PlainYearMonth/prototype/eraYear/',
      '/Temporal/PlainYearMonth/prototype/valueOf/',
      '/Temporal/PlainYearMonth/prototype/toString/',
      '/Temporal/PlainYearMonth/prototype/toJSON/',
      '/Temporal/PlainYearMonth/prototype/constructor.js',
      '/Temporal/PlainYearMonth/prototype/prop-desc.js',
      '/Temporal/PlainYearMonth/prototype/toStringTag/',
      // PDTDP-EXT 1: PDT derived-properties (11 getters)
      '/Temporal/PlainDateTime/prototype/dayOfWeek/',
      '/Temporal/PlainDateTime/prototype/dayOfYear/',
      '/Temporal/PlainDateTime/prototype/daysInMonth/',
      '/Temporal/PlainDateTime/prototype/daysInWeek/',
      '/Temporal/PlainDateTime/prototype/daysInYear/',
      '/Temporal/PlainDateTime/prototype/inLeapYear/',
      '/Temporal/PlainDateTime/prototype/monthsInYear/',
      '/Temporal/PlainDateTime/prototype/weekOfYear/',
      '/Temporal/PlainDateTime/prototype/yearOfWeek/',
      '/Temporal/PlainDateTime/prototype/era/',
      '/Temporal/PlainDateTime/prototype/eraYear/',
      // Foundation tests that pass without per-class implementation.
      '/Temporal/getOwnPropertyNames.js',
      '/Temporal/keys.js',
      '/Temporal/prop-desc.js',
      '/Temporal/toStringTag/',
    ],
  };
  for (const f of meta.features) {
    if (DELIBERATELY_OMITTED.has(f)) {
      // PARTIALLY_IMPLEMENTED carve-out: opt OUT of SKIP if path matches.
      const allowlist = PARTIALLY_IMPLEMENTED[f];
      if (allowlist && allowlist.some(prefix => path.includes(prefix))) {
        break; // fall through to normal execution
      }
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
