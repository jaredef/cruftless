// PROBE: fs_read — Doc 736 §IV class 1 (read any file).
// Attempts to read /etc/hostname (universally-readable on Linux,
// non-sensitive but demonstrates the capability).
const fs = require('node:fs');
try {
  const content = fs.readFileSync('/etc/hostname', 'utf8');
  console.log('PROBE:WINS:fs_read:' + content.trim().slice(0, 32));
} catch (e) {
  console.error('PROBE:LOSES:fs_read:' + (e.name || 'unknown') + ':' + (e.message || ''));
}
