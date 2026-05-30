---
name: redis-package-json-resolution
description: Redis package load failure caused by package-relative dot resolution in CommonJS require.
type: project
---

# redis-package-json-resolution - Seed

## Telos

`import("redis")` should load the package namespace instead of failing when a
transitive CommonJS module does `require(".")` from inside the package's
compiled `dist/lib/client/` tree.

## Scope

This locale is a singleton package-resolution residual surfaced by the
top500-fast-residual-survey. The targeted failure was:

`package.json read failed at .../redis/node_modules/./package.json`

The relevant package path is `@redis/client/dist/lib/client/pool.js`, whose
compiled CommonJS code calls `require(".")`. The resolver must treat bare dot
specifier forms (`.` and `..`) like relative directory specifiers, not like
bare package names.

## Apparatus

- Baseline evidence:
  `/home/jaredef/Developer/cruftless-sidecar/results/citpt-ext3-five.json`
- Sidecar package sandbox:
  `/home/jaredef/Developer/cruftless-sidecar/results/citpt-ext-3-parity-sandbox/redis/`
- Resolver substrate:
  `pilots/rusty-js-runtime/derived/src/module.rs`

## Methodology

1. Reproduce `import("redis")` from a redis package sandbox.
2. Inspect `@redis/client` require sites and package metadata.
3. Close only the dot-directory resolution gap.
4. Verify the direct resolver unit test, runtime lib tests, host build, and
   redis import smoke.

## Status

FOUNDED for R2 redis package-json resolution singleton.
