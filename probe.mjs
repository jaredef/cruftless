try {
  const M = await import('arktype');
  process.stdout.write(JSON.stringify({ok: true, keys: Object.keys(M).slice(0,5), kc: Object.keys(M).length}) + '\n');
} catch (e) {
  process.stdout.write(JSON.stringify({err: e?.message || String(e)}) + '\n');
}
