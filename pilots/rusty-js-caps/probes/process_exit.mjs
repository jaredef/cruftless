// PROBE: process_exit — Doc 736 §IV class 1 (DoS-equivalent + RCE in some threat models).
// Forces the host to exit with a distinctive code. If the call succeeds,
// the harness sees exit code 42 and infers WINS. If CapabilityError fires
// before exit, the catch branch runs and writes LOSES to stdout.
console.log('PROBE:STARTED:process_exit');
try {
  process.exit(42);
} catch (e) {
  console.log('PROBE:LOSES:process_exit:' + (e.name || 'unknown') + ':' + (e.message || ''));
}
