# Runtime Rust LoC Audit — 2026-05-30

Measurement instrument: `tokei` (Rust only, code lines exclude comments + blanks).
Scope: every crate that builds into the `cruft` runtime. Apparatus / measurement
instruments (`derive-constraints`, `legacy/welch`, `test262-categorize`,
`ts-consumer-corpus`, `ts-execute-corpus`) are excluded — they are tooling, not
runtime.

## Headline

Measured against `origin/main` (post-pull, even with remote on 2026-05-30).

| Layer | Rust code LoC | Crates |
|---|---|---|
| Engine substrate + host | 101,086 | 10 |
| Web/Node platform surface | 23,967 | 24 |
| **Grand total** | **125,053** | **34** |

- ~100k LoC is the core runtime path: parser → bytecode → IR → runtime → JIT → host.
- ~24k LoC is the platform / standard-library surface (fetch, streams, crypto, tls, buffer, node-* shims, etc.).

## Engine substrate + host

| Crate | Code LoC |
|---|---|
| rusty-js-runtime | 57,581 |
| rusty-js-ir | 10,805 |
| cruftless (host) | 10,561 |
| rusty-js-parser | 8,492 |
| rusty-js-bytecode | 7,420 |
| rusty-js-jit | 3,206 |
| rusty-js-pm | 1,497 |
| rusty-js-ast | 878 |
| rusty-js-gc | 326 |
| rusty-js-shapes | 320 |
| **Subtotal** | **101,086** |

Note: the `rusty-js-esm`, `rusty-js-caps`, `rusty-js-json-fast`,
`rusty-js-regex-fast`, and `rusty-js-http-server` locales are analysis-only
(no derived Rust crate); their implementation lives in `rusty-js-runtime` or
elsewhere, so they contribute 0 standalone LoC.

## Web/Node platform surface

| Crate | Code LoC | Surface |
|---|---|---|
| web-crypto | 7,642 | WebCrypto primitives |
| tls | 2,225 | TLS 1.3 |
| ts-resolve | 1,622 | TypeScript resolver |
| streams | 1,027 | Web Streams |
| structured-clone | 1,017 | structuredClone |
| fetch-api | 938 | fetch |
| x509 | 775 | X.509 certs |
| buffer | 735 | Node Buffer |
| node-path | 734 | Node path |
| compression | 720 | zlib/compression |
| bun-serve | 684 | Bun.serve |
| sockets | 621 | TCP/UDP sockets |
| node-http | 541 | Node http |
| urlsearchparams | 521 | URLSearchParams |
| bun-spawn | 507 | Bun.spawn |
| textencoder | 459 | TextEncoder/Decoder |
| websocket | 451 | WebSocket |
| asn1-der | 446 | ASN.1 DER |
| node-fs | 428 | Node fs |
| abort-controller | 425 | AbortController |
| blob | 415 | Blob |
| http-codec | 410 | HTTP wire codec |
| bun-file | 386 | Bun.file |
| file | 238 | File |
| **Subtotal** | **23,967** | **24 crates / 91 files** |

### Sub-groupings

- **Web standard surface** (~13.4k): web-crypto, streams, structured-clone, fetch-api, urlsearchparams, textencoder, websocket, abort-controller, blob, file, compression
- **Node compat shims** (~2.7k): node-fs, node-http, node-path, buffer, http-codec
- **Networking / crypto plumbing** (~3.4k): tls, x509, asn1-der, sockets
- **Bun-compat + TS** (~3.2k): bun-serve, bun-spawn, bun-file, ts-resolve

## Re-measurement

```sh
# engine + host
tokei -t Rust \
  pilots/rusty-js-ast pilots/rusty-js-bytecode/derived pilots/rusty-js-gc/derived \
  pilots/rusty-js-parser/derived pilots/rusty-js-runtime/derived pilots/rusty-js-ir/derived \
  pilots/rusty-js-jit/derived pilots/rusty-js-shapes/derived pilots/rusty-js-pm/derived \
  cruftless

# platform surface
tokei -t Rust $(for c in abort-controller asn1-der blob buffer bun-file bun-serve \
  bun-spawn compression fetch-api file http-codec sockets node-fs node-http node-path \
  streams structured-clone textencoder tls ts-resolve urlsearchparams web-crypto \
  websocket x509; do echo pilots/$c/derived; done)
```
