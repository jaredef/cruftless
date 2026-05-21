// PROBE: clock_read — Doc 736 §IV class 4-adjacent (timing side-channel surface).
try {
  const t1 = Date.now();
  let i = 0;
  for (; i < 100000; i++) {}
  const t2 = Date.now();
  console.log('PROBE:WINS:clock_read:dt=' + (t2 - t1) + 'ms:i=' + i);
} catch (e) {
  console.log('PROBE:LOSES:clock_read:' + (e.name || 'unknown') + ':' + (e.message || ''));
}
