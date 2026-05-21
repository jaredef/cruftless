# Ambient-Authority Audit (CAPS-EXT 1)

**Date**: 2026-05-21
**Method**: Static walk of `host-v2/src/` (node_stubs + install_* entry points) and `pilots/rusty-js-runtime/derived/src/` (intrinsics + module loader).
**Classification rules**:
- **pure**: returns a value computed only from arguments; no syscall, no I/O, no global state read beyond deterministic process structure
- **effectful**: makes a syscall, reads/writes filesystem/network/env/clock/process state, or causes any observable change
- **effectful-stub**: currently throws "not yet implemented" or returns a no-op; the surface name exists but no exploitable path
- **mixed**: dual-use; some paths through the method are pure and some are effectful

## Headline

| | count | share |
|---|---|---|
| pure | ~450 | 72% |
| effectful | ~40 | 6% |
| effectful-stub | ~130 | 21% |
| mixed | ~5 | 1% |
| **total enumerated** | **~625** | |

**Key finding**: only ~40 currently-callable effectful methods, dominated by filesystem (fs.*), timers (setTimeout/setInterval), stdout/stderr writes, process introspection (cwd/hrtime), os env-var reads (hostname/homedir/tmpdir), and the CJS require global. Many "scary" surfaces (child_process, http/https, crypto.randomBytes, dns, vm) are stubs that throw on call.

**The Move 1 gating budget is much smaller than Doc 736 §VI estimated.** The 2-3k LOC estimate assumed gating across the full Node surface; in practice we gate ~40 methods plus the dispatcher in `cjs_require`. Revised estimate: ~800-1200 LOC for Move 1's first cut.

## Per-namespace classification

### node:fs (the dominant effectful surface)

**Effectful (sync syscalls):**
- `readFileSync` — read(2)
- `writeFileSync` — write(2)
- `existsSync` — access(2)/stat(2)
- `statSync` — stat(2); exfil vector for mtime/size/mode
- `readdirSync` — readdir(2)
- `mkdirSync` — mkdir(2)
- `unlinkSync` — unlink(2)
- `accessSync` — access(2)
- `realpath` / `realpathSync` (+ `.native`) — stubs that return input; effectful classification because intent is path resolution

**Effectful (async, Promise-returning):** `readFile`, `writeFile`, `exists`, `fs.promises.{stat,readFile,writeFile,access,mkdir,unlink}` — all map onto the same syscall set as their sync counterparts.

**Effectful-stub** (~18 methods): `chmod[Sync]`, `chown[Sync]`, `fchmod[Sync]`, `fchown[Sync]`, `fstat[Sync]`, `lchmod[Sync]`, `lchown[Sync]`, `lstat[Sync]`, `stat`, `readv`/`writev`, `createReadStream`, `fs.watch`/`watchFile`/`unwatchFile`.

**Pure**: `fs.constants.*` (50+ numeric constants for file open / stat / IPC).

### node:process

**Effectful:**
- `process.pid` — reads `std::process::id()` at call time
- `process.cwd()` — getcwd(2)
- `process.hrtime()` + `process.hrtime.bigint()` — clock read (timing side-channel vector)
- `process.exit(code)` — terminates process (RCE-equivalent for the running app)
- `process.stdout.write` / `process.stderr.write` — write(2) to fd 1/2
- `process.stdin/stdout/stderr.isTTY` — would read fd properties (currently hardcoded false)
- `process.nextTick(cb)` — invokes callback immediately (v1 deviation); observable callback side effects

**Pure (constants captured at startup):**
- `process.argv`, `process.execArgv`, `process.env` (snapshot), `process.platform`, `process.arch`, `process.version`, `process.versions`, `process.stdin/stdout/stderr.fd`, `process.stdout.columns/rows`, `process.binding`

**Pure (EventEmitter stubs):** `process.{on,once,off,emit,removeListener,addListener,prependListener,prependOnceListener,listeners,rawListeners,listenerCount,eventNames,setMaxListeners,getMaxListeners}`.

**Pure (report stubs):** `process.report.*` all return empty objects/strings.

### node:os

**Effectful:**
- `os.hostname()` — reads `HOSTNAME` env var
- `os.homedir()` — reads `HOME` env var
- `os.tmpdir()` — reads `TMPDIR` env var
- `os.userInfo()` — reads `USER`/`HOME` env vars
- `os.cpus()` — reads `std::thread::available_parallelism()`

**Pure**: `os.platform`, `os.arch`, `os.type`, `os.endianness`, `os.EOL`, `os.release` (hardcoded), `os.totalmem`/`freemem`/`uptime`/`availableParallelism`/`loadavg`/`networkInterfaces` (hardcoded constants), `os.constants.signals.*` (35+ signal numbers), `os.constants.errno.*` (80+ errno values).

### globalThis (timers, console)

**Effectful:**
- `setTimeout(cb, ms)` / `setInterval(cb, ms)` — registers timer in `thread_local TIMERS`; clock-based scheduling
- `setImmediate(cb)` — macrotask scheduling (setTimeout with 0ms)
- `queueMicrotask(cb)` — microtask enqueue (observable callback side effects)
- `console.log/error/warn/info/debug` — write to stdout/stderr

**Pure:**
- `clearTimeout(id)` / `clearInterval(id)` / `clearImmediate(id)` — internal state update only
- `Timeout` object methods: `.ref()` / `.unref()` / `.hasRef()` / `.refresh()` — no-op state changes
- `console.assert/count/countReset/group/groupEnd/time/timeEnd/trace/table` — stubs

### globalThis (Date, Math)

**Effectful**: `Date.now()` — clock read (timing side-channel vector).

**Mixed**: `Math.random()` — deterministic PRNG output but internal state mutates; observable via tight-loop timing.

**Pure**: all other `Math.*`, `Date.parse`, all `Date.prototype` field accessors, `JSON.parse`/`stringify`, all Object/Array/String/RegExp/Number/Symbol/Map/Set/WeakMap/WeakSet methods, all error constructors, `parseInt`/`parseFloat`/`isNaN`/`isFinite`/`encodeURI*`/`decodeURI*`/`atob`/`btoa`.

### node:perf_hooks

**Effectful**: `performance.now()` — clock read (currently hardcoded 0, but intent is effectful).

**Pure-stub**: `performance.measure`, `performance.mark`, `PerformanceObserver`.

### node:module + globalThis require

**Effectful**:
- `require(spec)` — CJS loader; reads filesystem + executes loaded module code (so observable side effects include any effect the loaded module performs through its own capabilities)
- `require.resolve(spec)` — filesystem resolution walk

**Pure**: `module.createRequire(parent_url)` — returns a bound require function (the binding itself is pure; later invocations carry the effectfulness).

### node:crypto

**Pure**: `crypto.createHash(algo).update(...).digest(encoding)` for SHA-256, SHA-1, MD5 (real hand-rolled implementations; deterministic).

**Effectful-stub** (~9 methods): `randomBytes`, `randomUUID`, `getRandomValues`, `pbkdf2[Sync]`, `scrypt[Sync]`, `createCipheriv`, `createDecipheriv`, `createHmac`, `sign`, `verify`, `generateKeyPair`, all of `crypto.subtle.*`.

### node:http / node:https / node:http2 / node:dns / node:net / node:dgram

**Effectful-stub** (all callable methods): `request`, `get`, `createServer`, `connect`, `lookup`, `resolve`, all DNS/network operations. Pure constants (`STATUS_CODES`, `METHODS`) remain.

**Important**: cruftless currently has **no network surface at the JS-callable layer**. The TLS substrate from the engagement-internal pilots is wired into rusty-js-pm for install but not exposed as `node:http`/`node:https` to runtime JS code.

### node:child_process / node:cluster / node:worker_threads

**Effectful-stub** (all methods): `spawn[Sync]`, `exec[Sync]`, `execFile[Sync]`, `fork`. The RCE vector is name-present but not invocable.

### node:tls / node:net

**Effectful-stub**: `connect`, `createServer`, `createSecureContext`.
**Mixed**: `TLSSocket()` / `tls.Server()` constructors return stub instances with EventEmitter-shaped methods (no-op write/connect/destroy); constructors don't throw but operations are no-ops.

### node:vm

**Effectful-stub** (all methods): `runInThisContext`, `runInNewContext`, `runInContext`, `compileFunction`, `createContext`. Sandboxing surface absent.

### Pure-only namespaces

- **node:path** (and `node:path/posix`, `node:path/win32`): all 27 methods pure (string manipulation only)
- **node:url**: URL/URLSearchParams parsing and getters
- **node:buffer**: Buffer.from/alloc/concat/compare/etc., except `Blob` and `File` stubs
- **node:util**: format/inspect/inherits/types predicates (`util.deprecate` is mixed — emits warning on first call)
- **node:assert**: predicates that throw on failure
- **node:string_decoder**, **node:punycode**, **node:zlib** constants
- **node:events**: EventEmitter (`emit` is mixed — observes user-callback side effects)
- **node:domain**, **node:async_hooks**, **node:diagnostics_channel**: all stubs returning EventEmitter shapes
- **node:v8**, **node:inspector**: all stubs

## The mixed surface (warrants splitting in Move 1)

| Surface | Mixed reason | Recommended split |
|---|---|---|
| `EventEmitter.emit` | Pure if no listeners; invokes listeners (with their side effects) if any registered | Keep as-is; the side effects are *from* the listeners, which are caller-provided. Effect surface is whatever the listeners hold. |
| `tls.TLSSocket()` / `tls.Server()` | Constructor doesn't throw but socket methods no-op | Replace with explicit `CapabilityError` when no net capability provided |
| `AsyncResource.run` / `AsyncLocalStorage.run` / `Domain.run` | Invokes user callback | Same as EventEmitter.emit — caller-provided effect surface |
| `queueMicrotask` / `process.nextTick` | Enqueues caller callback for execution | Microtask scheduling itself is structural; the *callback* carries the effect surface |
| `Math.random()` | Output is deterministic on a per-call basis but internal state observable | Keep pure for first cut; mark as a side-channel concern (timing analysis defeats it anyway) |
| `util.deprecate` | Returns wrapped function; first call writes deprecation message to stderr | Capability-gate the stderr write; deprecation behavior degrades to silent |

## Absent surface (no gating required)

These npm-expected surfaces are **not exposed** in cruftless today:

- `node:child_process` — all stubs (no RCE vector)
- `node:net` / `node:dgram` — not exposed
- `node:fetch` — global fetch not exposed (undici pilot exists but not wired)
- `node:fs` open/close/read/write fd surface — not exposed (only path-based ops)
- `node:fs` symlink ops (`symlink`, `readlink`, `lutimes`) — not exposed
- `crypto.randomBytes` / `crypto.getRandomValues` / WebCrypto — all stubs
- `node:vm` sandboxing — all stubs
- `node:tls` actual socket layer — stubs
- `node:dns` — stubs
- `node:zlib` compression — stubs
- `node:readline` — stubs
- `node:inspector` debugging — stubs

This is good news for Pilot α scope: a substantial chunk of historically-dangerous Node surface is *already* unreachable in cruftless because the substrate hasn't been built. We gate what exists, document the absent surface, and ensure that any future implementation of the absent surface lands behind a capability gate from the start.

## The effective Move 1 gating budget

Distilled from the audit, the methods Pilot α must gate are:

**Filesystem (single `Fs` capability):**
- `fs.{readFileSync, writeFileSync, existsSync, statSync, readdirSync, mkdirSync, unlinkSync, accessSync, realpath, realpathSync}`
- `fs.{readFile, writeFile, exists}` (async forms)
- `fs.promises.{stat, readFile, writeFile, access, mkdir, unlink}`
- `require(spec)` — filesystem walk inside the loader

**Stdio (`Stdout` + `Stderr` capabilities, or a coarser `Stdio`):**
- `process.stdout.write` / `process.stderr.write`
- `console.{log, error, warn, info, debug}`

**Time + scheduler (`Clock` + `Scheduler` capabilities):**
- `Date.now`
- `process.hrtime` / `process.hrtime.bigint`
- `performance.now`
- `setTimeout` / `setInterval` / `setImmediate`
- `process.nextTick` / `queueMicrotask`

**Process (`Process` capability):**
- `process.exit`
- `process.cwd`
- `process.pid`

**Env (`Env` capability):**
- `os.hostname` / `os.homedir` / `os.tmpdir` / `os.userInfo`
- `os.cpus` (system-introspection)
- Access to `process.env` (currently a frozen snapshot, but conceptually env-read)

**Total surfaces to gate**: ~40 methods.

**LOC estimate (revised)**: ~600 LOC for the gating + ~300 LOC for capability constructors/deputation/restriction + ~200 LOC for `install_bun_host` rework = **~1100 LOC**. Below Doc 736 §VI's original 2-3k estimate; supports Pred-736.1 (retrofit, not rewrite).

## Substrate move ordering (CAPS-EXT 2 onward)

The audit suggests a priority order keyed on attack-severity × implementation-cost:

1. **`Fs` capability** (CAPS-EXT 4-5): largest surface, highest exfil risk, biggest single win. Two rounds because the sync/async paths share underlying syscalls but have separate dispatch sites.
2. **`Process.exit` gating** (CAPS-EXT 6): trivial; one method; high severity (denial of host process).
3. **`Stdio` capability** (CAPS-EXT 7): observable side channel via stdout writes; ergonomically tricky because console.log is everywhere. Default-grant for the application; explicit pass for deps.
4. **`Clock` + `Scheduler`** (CAPS-EXT 8-9): timing side channels.
5. **`Env`** (CAPS-EXT 10): low severity (process.env is a snapshot of public-ish data), but completes the model.

Move 3 (load-time SRI re-verifier) can land in parallel with any of the above as a no-regret hardening.

## Outstanding gaps

The audit covered host-v2 + the runtime's surfaceful entry points. **Not yet enumerated**:

- `pilots/rusty-js-runtime/derived/src/intrinsics.rs` per-method walk (the explorer reports ~100 methods classified at the namespace level: Math, JSON, Date, Promise, etc. mostly pure). A per-method pass would surface any effectful intrinsic the namespace-level classification missed.
- The `module.rs` resolver's `resolve_builtin_namespace` path — currently the only "ambient" route to node:* facades. Move 4 (closed import graphs) constrains this from the parser side.

These gaps are queued for CAPS-EXT 1.1 (a deeper walk if Pred-736.3's LOC estimate appears wrong during Move 1 implementation).

---

*The audit closes CAPS-EXT 1's deliverable. Next move (CAPS-EXT 2) is the capability-API design document: structured types, constructors at the host boundary, deputation/restriction operators, CapabilityError shape.*
