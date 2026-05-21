// PROBE: env_read — Doc 736 §IV class 4 (environment-variable secrets exfil vector).
//
// CAPS-EXT 9: under --sealed the runtime installs process.env as an
// empty object. The probe must distinguish "got the real env" (WINS)
// from "env is empty / variable absent" (LOSES).
try {
  const env = process.env;
  if (!env || typeof env !== 'object') {
    console.error('PROBE:LOSES:env_read:no-env-object');
  } else {
    const home = typeof env.HOME === 'string' ? env.HOME : '';
    const path = typeof env.PATH === 'string' ? env.PATH : '';
    if (home === '' && path === '') {
      console.error('PROBE:LOSES:env_read:env-empty');
    } else {
      console.log('PROBE:WINS:env_read:home=' + home.slice(0, 16) + ':path_len=' + path.length);
    }
  }
} catch (e) {
  console.error('PROBE:LOSES:env_read:' + (e.name || 'unknown') + ':' + (e.message || ''));
}
