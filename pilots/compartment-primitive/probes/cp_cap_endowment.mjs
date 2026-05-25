// CP-EXT 5 probe: capability-handle endowment validation per Doc 736.
//
// The point: a compartment's ambient bindings are EMPTY by default
// (no host-tier globals like `process`, `require`, `console`, etc.).
// Capabilities flow in via the explicit `globals` endowment map. This
// is the JS-API expression of Doc 736's "ambient authority denied;
// capability handle required" property.
//
// (1) endowed capability invokable
// (2) ambient host globals (process / require / console) DENIED — typeof returns undefined
// (3) intrinsic constructors (Array, Object, JSON, Math) STILL visible
// (4) the cap handle's method does the privileged op; same op via ambient path throws ReferenceError

// Construct a synthetic capability handle modeling Doc 736's pattern.
// In a real engagement this would come from the host (install_bun_host
// in rusty-js-caps trajectory). For the probe we synthesize the shape.
const fsCap = {
    _audit: [],
    readSafePath(path) {
        if (typeof path !== "string") throw new TypeError("fsCap.readSafePath: path must be a string");
        if (!path.startsWith("/tmp/safe/")) throw new Error("CapabilityError: path outside cap scope: " + path);
        this._audit.push(path);
        return "cap-served:" + path;
    },
};

const c = new Compartment({ globals: { fs: fsCap, JSON } });

// (1) endowed cap invokable
const r1 = c.evaluate('fs.readSafePath("/tmp/safe/data.json")');
if (r1 !== "cap-served:/tmp/safe/data.json") {
    console.log("FAIL_CAP_INVOKE: " + r1);
    throw "stop";
}

// (1b) cap discipline enforced inside the compartment
let pathReject = false;
try {
    c.evaluate('fs.readSafePath("/etc/passwd")');
} catch (e) {
    if (e && (e.message || "").indexOf("outside cap scope") >= 0) pathReject = true;
}
if (!pathReject) { console.log("FAIL_CAP_DISCIPLINE_NOT_ENFORCED"); throw "stop"; }

// (2) ambient host globals denied
const processT = c.evaluate('typeof process');
if (processT !== "undefined") { console.log("FAIL_AMBIENT_PROCESS_VISIBLE: " + processT); throw "stop"; }
const requireT = c.evaluate('typeof require');
if (requireT !== "undefined") { console.log("FAIL_AMBIENT_REQUIRE_VISIBLE: " + requireT); throw "stop"; }
const consoleT = c.evaluate('typeof console');
if (consoleT !== "undefined") { console.log("FAIL_AMBIENT_CONSOLE_VISIBLE: " + consoleT); throw "stop"; }

// (3) intrinsic constructors visible
const arrayT = c.evaluate('typeof Array');
if (arrayT !== "function") { console.log("FAIL_INTRINSIC_ARRAY: " + arrayT); throw "stop"; }
const jsonStr = c.evaluate('JSON.stringify({a:1})');
if (jsonStr !== '{"a":1}') { console.log("FAIL_INTRINSIC_JSON: " + jsonStr); throw "stop"; }
const mathT = c.evaluate('typeof Math');
if (mathT !== "object") { console.log("FAIL_INTRINSIC_MATH: " + mathT); throw "stop"; }

// (4) attempting the privileged op via ambient (process.exit, fetch, etc.) fails
let ambientDenied = false;
try {
    c.evaluate('process.exit(0)');
} catch (e) {
    // expected — process is undefined
    if (e && e.constructor && e.constructor.name === "TypeError") ambientDenied = true;
    else if (e && e.constructor && e.constructor.name === "ReferenceError") ambientDenied = true;
}
if (!ambientDenied) { console.log("FAIL_AMBIENT_PRIV_OP_SUCCEEDED"); throw "stop"; }

// (5) audit log on cap captures the invocations
if (fsCap._audit.length !== 1 || fsCap._audit[0] !== "/tmp/safe/data.json") {
    console.log("FAIL_AUDIT_LOG: " + JSON.stringify(fsCap._audit));
    throw "stop";
}

console.log("CP_EXT_5_OK");
