const DEVIATIONS = [
  "function-not-constructor-relax",
  "to-object-coerce-nullish",
];

if (typeof __cruftless_tolerate === "function") {
  for (const d of DEVIATIONS) {
    try { __cruftless_tolerate(d); } catch (_) {}
  }
}

const pkg = process.env.PARITY_PROBE_PKG;
if (!pkg) {
  process.stdout.write('{"status":"NO_PKG"}\n');
  process.exit(0);
}

try {
  const m = await import(pkg);
  const keys = Object.keys(m).sort();
  const shape = {};
  for (const k of keys) {
    const v = m[k];
    shape[k] = typeof v;
  }
  process.stdout.write(JSON.stringify({
    status: "OK", pkg, keyCount: keys.length, shape,
  }) + "\n");
} catch (e) {
  process.stdout.write(JSON.stringify({
    status: "ERR", pkg,
    error: (e && e.constructor && e.constructor.name) || "Error",
    message: ((e && e.message) || String(e)).slice(0, 800),
  }) + "\n");
}
