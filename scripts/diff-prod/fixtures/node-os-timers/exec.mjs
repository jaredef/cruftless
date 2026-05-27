// F-category: node:os and timer/microtask globals.

import os from "node:os";

function canon(v) {
  return JSON.stringify(v, (_k, v) => {
    if (v && typeof v === "object" && !Array.isArray(v)) {
      const out = {}; for (const k of Object.keys(v).sort()) out[k] = v[k]; return out;
    }
    return v;
  });
}

const result = {};

// os.platform.
{
  try {
    result.platform_typeof = typeof os.platform();
    result.platform_is_string = typeof os.platform() === "string";
    result.platform_nonempty = os.platform().length > 0;
  } catch (e) {
    result.platform = e.constructor.name;
  }
}

// os.arch.
{
  try {
    result.arch_typeof = typeof os.arch();
    result.arch_is_string = typeof os.arch() === "string";
    result.arch_nonempty = os.arch().length > 0;
  } catch (e) {
    result.arch = e.constructor.name;
  }
}

// os.tmpdir.
{
  try {
    result.tmpdir_typeof = typeof os.tmpdir();
    result.tmpdir_is_string = typeof os.tmpdir() === "string";
    result.tmpdir_nonempty = os.tmpdir().length > 0;
  } catch (e) {
    result.tmpdir = e.constructor.name;
  }
}

// os.homedir.
{
  try {
    result.homedir_typeof = typeof os.homedir();
    result.homedir_is_string = typeof os.homedir() === "string";
    result.homedir_nonempty = os.homedir().length > 0;
  } catch (e) {
    result.homedir = e.constructor.name;
  }
}

// os.hostname.
{
  try {
    result.hostname_typeof = typeof os.hostname();
    result.hostname_is_string = typeof os.hostname() === "string";
  } catch (e) {
    result.hostname = e.constructor.name;
  }
}

// os.cpus.
{
  try {
    const cpus = os.cpus();
    result.cpus_is_array = Array.isArray(cpus);
    result.cpus_length_gt0 = cpus.length > 0;
    if (cpus.length > 0) {
      result.cpu0_has_model = typeof cpus[0].model === "string";
      result.cpu0_has_speed = typeof cpus[0].speed === "number";
      result.cpu0_has_times = typeof cpus[0].times === "object" && cpus[0].times !== null;
    }
  } catch (e) {
    result.cpus = e.constructor.name;
  }
}

// os.type, os.release, os.totalmem, os.freemem.
{
  try {
    result.type_typeof = typeof os.type();
    result.type_is_string = typeof os.type() === "string";
  } catch (e) {
    result.os_type = e.constructor.name;
  }
  try {
    result.release_typeof = typeof os.release();
    result.release_is_string = typeof os.release() === "string";
  } catch (e) {
    result.os_release = e.constructor.name;
  }
  try {
    result.totalmem_typeof = typeof os.totalmem();
    result.totalmem_is_number = typeof os.totalmem() === "number";
    result.totalmem_gt0 = os.totalmem() > 0;
  } catch (e) {
    result.totalmem = e.constructor.name;
  }
  try {
    result.freemem_typeof = typeof os.freemem();
    result.freemem_is_number = typeof os.freemem() === "number";
    result.freemem_gt0 = os.freemem() > 0;
  } catch (e) {
    result.freemem = e.constructor.name;
  }
}

// os.EOL.
{
  result.eol_typeof = typeof os.EOL;
  result.eol_is_string = typeof os.EOL === "string";
  result.eol_length = os.EOL.length;
}

// setTimeout / clearTimeout shape.
{
  result.setTimeout_typeof = typeof setTimeout;
  result.clearTimeout_typeof = typeof clearTimeout;
  result.setTimeout_is_function = typeof setTimeout === "function";
  result.clearTimeout_is_function = typeof clearTimeout === "function";
}

// setInterval / clearInterval shape.
{
  result.setInterval_typeof = typeof setInterval;
  result.clearInterval_typeof = typeof clearInterval;
}

// setImmediate presence.
{
  result.setImmediate_typeof = typeof setImmediate;
  result.setImmediate_available = typeof setImmediate === "function";
  if (typeof clearImmediate !== "undefined") {
    result.clearImmediate_typeof = typeof clearImmediate;
  }
}

// queueMicrotask presence.
{
  result.queueMicrotask_typeof = typeof queueMicrotask;
  result.queueMicrotask_is_function = typeof queueMicrotask === "function";
}

// await setTimeout to prove resolution.
{
  try {
    const val = await new Promise(resolve => {
      setTimeout(() => resolve("timer_ok"), 10);
    });
    result.setTimeout_resolves = val;
  } catch (e) {
    result.setTimeout_resolves = e.constructor.name;
  }
}

// performance.now returns number.
{
  try {
    const t = performance.now();
    result.perf_now_typeof = typeof t;
    result.perf_now_is_number = typeof t === "number";
    result.perf_now_gt0 = t > 0;
  } catch (e) {
    result.perf_now = e.constructor.name;
  }
}

console.log(canon(result));
