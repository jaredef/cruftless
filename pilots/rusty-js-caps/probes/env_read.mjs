// PROBE: env_read — Doc 736 §IV class 4 (environment-variable secrets exfil vector).
try {
  const home = process.env.HOME || '';
  const path = process.env.PATH || '';
  console.log('PROBE:WINS:env_read:home=' + home.slice(0, 16) + ':path_len=' + path.length);
} catch (e) {
  console.log('PROBE:LOSES:env_read:' + (e.name || 'unknown') + ':' + (e.message || ''));
}
