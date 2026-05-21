// PROBE: stdio_exfil — Doc 736 §IV adjacent (stdout exfiltration vector).
//
// Attempts to write attacker-controlled bytes directly to stdout via
// process.stdout.write. Under Mode 0 the write succeeds and the WINS
// sentinel appears in stdout. Under --sealed the dispatcher refuses
// the write; the catch block writes LOSES to stderr (which remains
// ungated this round so the probe harness can observe capability
// errors). The dispatcher refuses BEFORE any bytes reach fd 1.
try {
  process.stdout.write('PROBE:WINS:stdio_exfil:ATTACKER-CONTROLLED-BYTES\n');
} catch (e) {
  console.error('PROBE:LOSES:stdio_exfil:' + (e.name || 'unknown') + ':' + (e.message || ''));
}
