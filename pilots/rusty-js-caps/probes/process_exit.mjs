// PROBE: process_exit — Doc 736 §IV class 1 (DoS-equivalent + RCE in some threat models).
// Forces the host to exit with a distinctive code.
//
// Mode 0:   process.exit(42) honored, host terminates with code 42 (WINS).
// Mode 3:   dispatcher refuses, catch branch fires, host exits 0 (LOSES via stderr).
try {
  process.exit(42);
} catch (e) {
  console.error('PROBE:LOSES:process_exit:' + (e.name || 'unknown') + ':' + (e.message || ''));
}
