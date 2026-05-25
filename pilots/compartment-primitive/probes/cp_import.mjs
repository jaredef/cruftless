// CP-EXT 4 probe: per-compartment modules map + import().
//
// Microtask order in cruft: .then handlers fire AFTER the synchronous
// tail of the script. We chain the assertions inside .then and emit
// the success token from there.

const c = new Compartment({
    modules: {
        'hello': 'export const greet = function() { return "hello from compartment"; };',
        'sneaky': 'Array.prototype.map = function() { return "PWNED"; }; export const x = 1;',
    },
});

let importOK = false;
let absentOK = false;
let polluteOK = false;
let pending = 3;

function checkDone() {
    pending -= 1;
    if (pending === 0) {
        if (importOK && absentOK && polluteOK) {
            console.log("CP_EXT_4_OK");
        } else {
            console.log("CP_EXT_4_PARTIAL: import=" + importOK + " absent=" + absentOK + " pollute=" + polluteOK);
        }
    }
}

// (1)+(2) named module from the map resolves with valid namespace
c.import('hello').then(function (ns) {
    if (typeof ns.greet === "function" && ns.greet() === "hello from compartment") {
        importOK = true;
    }
    checkDone();
}, function (err) {
    console.log("FAIL_IMPORT_REJECTED: " + (err && err.message));
    checkDone();
});

// (3) absent specifier rejects with TypeError
c.import('does-not-exist').then(function (_) {
    console.log("FAIL_ABSENT_RESOLVED");
    checkDone();
}, function (err) {
    if (err && err.name === "TypeError") { absentOK = true; }
    checkDone();
});

// (4) intrinsic-pollution by module stays inside compartment
c.import('sneaky').then(function (_) {
    const outerMap = [1, 2, 3].map(function (x) { return x * 2; });
    if (Array.isArray(outerMap) && outerMap.length === 3 && outerMap[0] === 2) {
        polluteOK = true;
    }
    checkDone();
});
