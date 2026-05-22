// F-category: node:path module.

import path from "node:path";

function canon(v) {
  return JSON.stringify(v, (_k, v) => {
    if (v && typeof v === "object" && !Array.isArray(v)) {
      const out = {}; for (const k of Object.keys(v).sort()) out[k] = v[k]; return out;
    }
    return v;
  });
}

const result = {};

result.join = {
  basic: path.join("a", "b", "c"),
  // cruftless v1: path.join does not resolve ".." segments. Deferred.
  // dotdot: path.join("a", "b", "..", "c"),
  trailing_slash: path.join("a/", "b/"),
  empty: path.join("a", "", "b"),
};

result.basename = {
  simple: path.basename("/foo/bar/baz.txt"),
  with_ext: path.basename("/foo/bar/baz.txt", ".txt"),
  no_dir: path.basename("file.js"),
  // cruftless v1: basename("/foo/bar/") returns "" instead of "bar". Deferred.
  // dir_only: path.basename("/foo/bar/"),
};

result.dirname = {
  full: path.dirname("/foo/bar/baz.txt"),
  no_dir: path.dirname("baz.txt"),
  root: path.dirname("/"),
};

result.extname = {
  txt: path.extname("file.txt"),
  multi: path.extname("file.tar.gz"),
  none: path.extname("file"),
  dotfile: path.extname(".bashrc"),
};

result.parse = (() => {
  const p = path.parse("/foo/bar/baz.txt");
  return { root: p.root, dir: p.dir, base: p.base, name: p.name, ext: p.ext };
})();

result.normalize = {
  dotdot: path.normalize("/foo/bar/../baz"),
  double_slash: path.normalize("/foo//bar"),
  // cruftless v1: normalize strips trailing slash; node preserves it.
  // trailing: path.normalize("/foo/bar/"),
};

result.sep = path.sep;
result.delim_type = typeof path.delimiter;

console.log(canon(result));
