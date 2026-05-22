// F-category: node:fs sync surface.

import * as fs from "node:fs";
import * as path from "node:path";
import * as os from "node:os";

function canon(v) {
  return JSON.stringify(v, (_k, v) => {
    if (v && typeof v === "object" && !Array.isArray(v)) {
      const out = {}; for (const k of Object.keys(v).sort()) out[k] = v[k]; return out;
    }
    return v;
  });
}

const result = {};

// Use a fixed temp file under os.tmpdir so cleanup is harmless across engines.
const tmp = os.tmpdir();
const file = path.join(tmp, "diff-prod-fs-fixture.txt");
const dir = path.join(tmp, "diff-prod-fs-dir");

// Cleanup any prior run.
try { fs.unlinkSync(file); } catch {}
try { fs.rmSync(dir, { recursive: true, force: true }); } catch {}

// existsSync before write.
result.exists_before = fs.existsSync(file);

// writeFileSync + readFileSync roundtrip (utf8 + buffer).
fs.writeFileSync(file, "hello from diff-prod\n");
result.exists_after = fs.existsSync(file);
result.read_utf8 = fs.readFileSync(file, "utf8");

// readFileSync buffer form.
{
  const buf = fs.readFileSync(file);
  result.read_buffer_length = buf.length;
  result.read_buffer_first_byte = buf[0];          // 104 = 'h'
}

// statSync.
{
  const s = fs.statSync(file);
  result.stat = {
    isFile: s.isFile(),
    isDirectory: s.isDirectory(),
    size_positive: s.size > 0,
  };
}

// mkdirSync + readdirSync.
fs.mkdirSync(dir);
fs.writeFileSync(path.join(dir, "a.txt"), "a");
fs.writeFileSync(path.join(dir, "b.txt"), "b");
fs.writeFileSync(path.join(dir, "c.txt"), "c");
{
  const entries = fs.readdirSync(dir).sort();
  result.readdir = entries;
}

// unlinkSync + verify gone.
fs.unlinkSync(file);
result.exists_after_unlink = fs.existsSync(file);

// rmSync recursive on the dir.
fs.rmSync(dir, { recursive: true, force: true });
result.dir_after_rm = fs.existsSync(dir);

console.log(canon(result));
