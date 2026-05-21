// PROBE: fs_stat — Doc 736 §IV adjacent (filesystem info disclosure: mtime, size, mode).
const fs = require('node:fs');
try {
  const s = fs.statSync('/etc/hostname');
  console.log('PROBE:WINS:fs_stat:size=' + (s.size ?? 0) + ':mode=' + (s.mode ?? 0));
} catch (e) {
  console.log('PROBE:LOSES:fs_stat:' + (e.name || 'unknown') + ':' + (e.message || ''));
}
