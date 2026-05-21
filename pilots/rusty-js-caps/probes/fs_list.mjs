// PROBE: fs_list — Doc 736 §IV adjacent (directory enumeration ≈ info disclosure).
// Attempts to list /etc.
const fs = require('node:fs');
try {
  const entries = fs.readdirSync('/etc');
  console.log('PROBE:WINS:fs_list:' + entries.length + ':' + (entries[0] || ''));
} catch (e) {
  console.log('PROBE:LOSES:fs_list:' + (e.name || 'unknown') + ':' + (e.message || ''));
}
