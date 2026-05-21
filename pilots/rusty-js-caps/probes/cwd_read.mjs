// PROBE: cwd_read — Doc 736 §IV class 4-adjacent (process introspection).
try {
  const cwd = process.cwd();
  console.log('PROBE:WINS:cwd_read:' + cwd.slice(0, 64));
} catch (e) {
  console.log('PROBE:LOSES:cwd_read:' + (e.name || 'unknown') + ':' + (e.message || ''));
}
