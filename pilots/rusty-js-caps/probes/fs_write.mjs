// PROBE: fs_write — Doc 736 §IV class 6 (persist to filesystem).
// Attempts to write to /tmp/cruftless-probe-fs-write.marker.
const fs = require('node:fs');
const path = '/tmp/cruftless-probe-fs-write.marker';
try {
  fs.writeFileSync(path, 'WIN-' + Date.now() + '\n');
  console.log('PROBE:WINS:fs_write:' + path);
} catch (e) {
  console.error('PROBE:LOSES:fs_write:' + (e.name || 'unknown') + ':' + (e.message || ''));
}
