# Capability API Design (CAPS-EXT 2)

**Date**: 2026-05-21
**Scope**: the structured capability surface that Pilot α's substrate moves implement. Defines the capability types, their methods, constructors at the host boundary, deputation/restriction operators, the dispatcher protocol, and the mode-aware enforcement contract per Doc 736 §IX.

## I. The dispatcher protocol

Every effectful operation in the runtime routes through a single dispatcher call:

```rust
// In rusty-js-runtime (Rust side)
fn require_capability<C: Capability>(
    rt: &Runtime,
    cap: &C,
    op: C::Op,
) -> Result<C::Granted, CapabilityError>;
```

The dispatcher consults the active `CapMode` and the calling module's provenance (application / dependency) to decide:

- **Mode 0**: always returns the ambient capability for `C::Op`; the `cap` argument is ignored. Behavior identical to current Cruftless.
- **Mode 1**: same as Mode 0 plus records the call to the thread-local audit log.
- **Mode 2**: if the calling module is the application, returns ambient; if it is under `node_modules/`, returns `cap.granted(op)` or `Err(CapabilityError)`.
- **Mode 3**: returns `cap.granted(op)` regardless of caller; CapabilityError if the capability does not authorize the op.

Every effectful JS method's Rust implementation begins with one `require_capability` call. The capability argument comes from either an ambient handle held by the host or a handle the JS caller explicitly passed.

JS side, every effectful method is split:

```javascript
// Mode 0 / Mode 1 (ambient): the cap argument is implicit
fs.readFileSync(path);

// Mode 2 / Mode 3 (explicit capability passing):
fs.readFileSync(myFsCap, path);
// or, the JS shim threads it:
require('fs', { caps: { fs: myFsCap } }).readFileSync(path);
```

The capability handle on the JS side is an opaque object whose internal slot is a Rust `Arc<dyn Capability>`. JS code cannot construct one. The host constructs them at `install_bun_host` time; the application receives them; the application passes them.

## II. The six capability types

Per the CAPS-EXT 1 audit, the currently-callable effectful surface clusters into six classes. Each is a distinct Rust type implementing the `Capability` trait. The types are:

1. **Fs** — filesystem read/write/stat/list/remove/mkdir/access
2. **Stdio** — stdout/stderr writes
3. **Clock** — Date.now, hrtime, performance.now
4. **Scheduler** — setTimeout/setInterval/setImmediate, nextTick, queueMicrotask
5. **Process** — exit, cwd, pid, env-var reads
6. **Env** — os.hostname/homedir/tmpdir/userInfo/cpus (system introspection)

Six rather than the five §III.3 of Doc 736 named, because the audit found `Scheduler` and `Clock` are operationally separable (a dep can have `Clock` without having `Scheduler` and vice versa, with distinct attack-surface implications).

## III. The capability types in detail

### III.1 `Fs`

```rust
pub struct Fs {
    pub read: PathPolicy,    // which paths can be read
    pub write: PathPolicy,   // which paths can be written
    pub list: PathPolicy,    // which directories can be enumerated
    pub stat: PathPolicy,    // which paths can be stat-ed
    pub mkdir: PathPolicy,   // where directories can be created
    pub remove: PathPolicy,  // where files/dirs can be removed
}

pub enum PathPolicy {
    None,                          // no path
    Any,                           // every path
    Prefix(PathBuf),               // any path starting with prefix
    Prefixes(Vec<PathBuf>),        // union of prefixes
    Exact(Vec<PathBuf>),           // exactly these paths
}

pub enum FsOp {
    Read(PathBuf),
    Write(PathBuf),
    List(PathBuf),
    Stat(PathBuf),
    Mkdir(PathBuf),
    Remove(PathBuf),
}
```

The path policies are coarse in the first cut. Refinement to glob matching, regex matching, or content-type policy is deferred. Each policy is a closed enum so a future variant can be added without breaking existing capability constructions.

JS-side shape (Mode 2/3 with explicit pass):

```javascript
const fsCap = host.fs;             // ambient root from host
const limited = fsCap.subDir('./data');  // restricts to ./data prefix
require('axios', { caps: { fs: limited } });
```

Deputation operators on `Fs`:

```rust
impl Fs {
    pub fn sub_dir(&self, sub: impl Into<PathBuf>) -> Fs;     // narrow to a prefix
    pub fn read_only(&self) -> Fs;                            // strip write/mkdir/remove
    pub fn write_only(&self) -> Fs;                           // strip read/list/stat
    pub fn union(&self, other: &Fs) -> Fs;                    // merge two policies
}
```

JS bindings: `fs_cap.subDir(s)`, `fs_cap.readOnly()`, `fs_cap.writeOnly()`, `fs_cap.union(other)`.

### III.2 `Stdio`

```rust
pub struct Stdio {
    pub stdout: bool,
    pub stderr: bool,
}

pub enum StdioOp {
    Stdout(Vec<u8>),
    Stderr(Vec<u8>),
}
```

Simpler than Fs because there are only two streams. Deputation: `stdout_only()`, `stderr_only()`, `silent()`.

The default application capability is `Stdio { stdout: true, stderr: true }`. The default dep capability under Mode 2 / Mode 3 is `Stdio { stdout: false, stderr: false }` unless the dep's manifest declared otherwise *and* the application explicitly passed.

### III.3 `Clock`

```rust
pub struct Clock {
    pub resolution: ClockResolution,
}

pub enum ClockResolution {
    Disabled,                    // CapabilityError on any read
    Coarse(Duration),            // rounded to nearest Duration
    Fine,                        // raw clock
}

pub enum ClockOp {
    Now,                         // wallclock millis
    HighResolution,              // performance.now nanos
}
```

The `Coarse` variant is the side-channel mitigation: timing attacks need fine-grained clock access. A dep that needs the clock for cache-expiry logic can receive `ClockResolution::Coarse(Duration::from_millis(100))` and never see a clock value useful for side-channel measurement.

Deputation: `Clock::disabled()`, `Clock::coarse(d)`, `Clock::fine()`.

### III.4 `Scheduler`

```rust
pub struct Scheduler {
    pub timers: bool,            // setTimeout / setInterval / setImmediate
    pub microtasks: bool,        // queueMicrotask / process.nextTick
    pub min_delay: Duration,     // 0 for none; > 0 floors all timer delays
}

pub enum SchedulerOp {
    Timer(Duration),
    Microtask,
}
```

`min_delay` is the rate-limit knob. A dep that wants to busy-wait on `setTimeout(cb, 0)` is bounded by the application's floor. Setting `min_delay` to 16ms approximates "no faster than animation frame."

### III.5 `Process`

```rust
pub struct Process {
    pub may_exit: bool,
    pub may_read_cwd: bool,
    pub may_read_pid: bool,
}

pub enum ProcessOp {
    Exit(i32),
    ReadCwd,
    ReadPid,
}
```

`may_exit` is the high-severity bit. A dep with `may_exit: true` can call `process.exit(1)` and terminate the host process. Default for deps under Mode 2/3: `false` everywhere.

### III.6 `Env`

```rust
pub struct Env {
    pub vars: EnvVarPolicy,      // which env vars are readable
    pub system_info: bool,       // os.cpus, os.userInfo, etc.
}

pub enum EnvVarPolicy {
    None,
    Any,
    Whitelist(Vec<String>),      // only these var names
}
```

`process.env` is currently a frozen snapshot captured at startup. The capability gates the *snapshot read*, not the live env. A dep with `Env::default()` (no vars) cannot read any env-var-derived value. A dep with `Whitelist(vec!["LANG", "TZ"])` can read those two and nothing else.

## IV. CapabilityError

```rust
pub struct CapabilityError {
    pub capability: &'static str,        // "fs", "stdio", "clock", etc.
    pub operation: String,               // human-readable op summary
    pub calling_module: ModuleId,        // identity of the caller
    pub mode: CapMode,                   // which mode was active
    pub hint: Option<String>,            // suggested cap declaration
}
```

JS side, this surfaces as a thrown Error with:

```javascript
{
    name: 'CapabilityError',
    message: "fs.readFileSync('/etc/passwd'): no fs capability granted to module 'sketchy-pkg' (mode: sealed-deps)",
    capability: 'fs',
    operation: "readFileSync('/etc/passwd')",
    callingModule: 'sketchy-pkg',
    mode: 'sealed-deps',
    hint: "Add to caps in package.json: { fs: { read: ['./safe-path'] } }"
}
```

The hint is the ergonomic load-bearing field. When the developer hits a CapabilityError, the error tells them exactly what declaration would unblock the call. The amendment's Mode 1 audit mode produces declarations automatically; the hint in CapabilityError makes Mode 2 self-explanatory.

## V. Mode-aware dispatcher

```rust
pub enum CapMode {
    Compat,           // Mode 0
    Audit,            // Mode 1
    SealedDeps,       // Mode 2
    Sealed,           // Mode 3
}

pub struct CapDispatcher {
    mode: CapMode,
    ambient: AmbientCaps,            // the full-authority handles the host holds
    audit_log: Option<AuditLog>,     // populated in Mode 1
}

impl CapDispatcher {
    pub fn require<C: Capability>(
        &self,
        cap: &C,
        op: C::Op,
        caller: ModuleId,
    ) -> Result<C::Granted, CapabilityError> {
        if let Some(log) = &self.audit_log {
            log.record(caller, C::name(), &op);
        }
        match self.mode {
            CapMode::Compat | CapMode::Audit => Ok(self.ambient.granted(op)),
            CapMode::SealedDeps if caller.is_application() => {
                Ok(self.ambient.granted(op))
            }
            CapMode::SealedDeps | CapMode::Sealed => {
                cap.check(&op).map_err(|reason| CapabilityError {
                    capability: C::name(),
                    operation: op.describe(),
                    calling_module: caller,
                    mode: self.mode,
                    hint: Some(C::hint_for(&op)),
                })
            }
        }
    }
}
```

The dispatcher is the single point where mode policy and capability policy compose. Adding a new mode means adding a match arm; adding a new capability means implementing the `Capability` trait. No effectful method bypasses the dispatcher.

## VI. The `Capability` trait

```rust
pub trait Capability {
    type Op: OpDescribe;
    type Granted;

    fn name() -> &'static str;
    fn check(&self, op: &Self::Op) -> Result<Self::Granted, &'static str>;
    fn hint_for(op: &Self::Op) -> String;
}

pub trait OpDescribe {
    fn describe(&self) -> String;
}
```

`Granted` is typically `()` for boolean caps and `&CapabilityHandle` for ones that produce a deputation. The trait is intentionally minimal; complexity lives in per-capability implementations.

## VII. Module provenance

The dispatcher's Mode-2 branch keys on `ModuleId::is_application()`. The PM-side lockfile and the runtime-side module loader together produce this:

- A module loaded from a path under `<project>/node_modules/` is `Dependency`.
- A module loaded from a path under `<project>/` but outside `node_modules/` is `Application`.
- A module from `<project>/node_modules/.cruftless-staging/` (transient) is treated as `Dependency` to be safe.
- A module from a `file://` URL outside `<project>/` is `External`; treated as `Dependency` in Mode 2 (deny by default).

The `ModuleId` is established at load time and carried through the bytecode compiler so the dispatcher can read it without a stack walk. The path lives in `pilots/rusty-js-bytecode/derived/src/` (the compiler annotates each compiled module with its provenance flag).

## VIII. Host capability construction

The host (`host-v2/src/lib.rs`) constructs ambient capabilities at process start:

```rust
pub fn ambient_caps() -> AmbientCaps {
    AmbientCaps {
        fs: Fs::full(),                        // every path readable + writable
        stdio: Stdio { stdout: true, stderr: true },
        clock: Clock::fine(),                  // raw nanosecond clock
        scheduler: Scheduler { timers: true, microtasks: true, min_delay: Duration::ZERO },
        process: Process { may_exit: true, may_read_cwd: true, may_read_pid: true },
        env: Env { vars: EnvVarPolicy::Any, system_info: true },
    }
}
```

Under Mode 0 / Mode 1, every JS module's effectful call resolves to this ambient set via the dispatcher. Under Mode 2, application modules continue to resolve here; dep modules receive whatever their manifest declares + whatever the application passed. Under Mode 3, even the application receives only what `cruftless-caps.json` declared.

## IX. JS-side capability handles

Capability handles on the JS side are opaque objects with these guarantees:

1. **Cannot be constructed by JS code.** Only the host's intrinsics produce them.
2. **Are deputable but never broadenable.** `fsCap.subDir('./data')` produces a narrower cap; there is no operator that broadens.
3. **Survive serialization fences as opaque tokens.** JSON.stringify on a handle yields a sentinel string with no exploitable structure.
4. **Are reference-equal across passes within the same process.** `fsCap === childRequire('fs-passing-pkg').receivedCap` holds if the application passed the same handle.

A handle's runtime shape:

```javascript
{
    [Symbol.for('cruftless.cap')]: '<opaque-token>',
    subDir(path) { /* native, returns new handle */ },
    readOnly() { /* native, returns new handle */ },
    union(other) { /* native, returns new handle */ }
}
```

The `Symbol.for('cruftless.cap')` slot is the host's identity check. A dep that tries to forge a capability by constructing an object with that symbol fails: the host reads the underlying Rust pointer through a sealed slot, not the JS-visible field.

## X. The require() API extension

`require(spec, opts?)` accepts a second argument in Mode 2 / Mode 3:

```javascript
require('lodash');                            // no caps; dep gets sealed default
require('axios', { caps: { net: netCap } });  // explicit pass
require('plugin', { caps: { fs: fsCap.subDir('./data') } });
```

In Mode 0 / Mode 1, the `opts.caps` field is recorded but does not affect behavior. The recording is what produces the audit log: the dispatcher knows what the application *intended* to pass even when the runtime is ambient.

Symmetric ESM form:

```javascript
import lodash from 'lodash';
import axios from 'axios' with { caps: { net: netCap } };
```

The `with { caps }` attribute is a Cruftless extension to the import-attributes proposal. ESM static analysis lets the compiler thread the capability without runtime dispatch.

## XI. Backward compatibility

Mode 0 is the default. Mode 0 preserves every existing call site. No JS source change is required to upgrade Cruftless from CAPS-EXT 0 to any post-Pilot-α version, as long as the user keeps Mode 0 active.

The PM-EXT 11 / 12 / 13 test gates run in Mode 0 in CI by default. A second CI lane runs the same gates in Mode 1 to populate the audit log. Once Move 2 (manifest schema) lands, a third CI lane runs in Mode 2 to verify that the lockfile-declared capabilities match observed audit behavior.

## XII. What this contract does not specify

Deferred to subsequent EXT rounds:

- **Async capability passing across await boundaries.** A capability handed to an async function survives the await; the implementation detail (whether the handle is captured in the async-fn closure environment or threaded through Promise reactions) is a CAPS-EXT N decision.
- **Capability handles in serialized data structures.** Structured-clone of an object containing a capability fails (sentinel sub-object). Workarounds (export the cap separately, re-bind on the other side) are a CAPS-EXT N concern.
- **Cross-realm capabilities.** Cruftless does not implement realms in the SES sense. If it ever does, capabilities will cross realm boundaries with explicit ferrying.
- **Capability revocation.** A capability is currently a value type; revocation requires a level of indirection (`Rc<RefCell<Option<C>>>`) that is straightforward but deferred. The use case (revoke after a phase change) is rare enough that the first cut omits it.
- **Persistent capability storage.** Capabilities are process-lifetime. A capability cannot be serialized to disk and reloaded. The lockfile records declarations, not handles.

## XIII. The implementation order

Per CAPS-EXT 1's substrate-move ordering, the implementation lands in this sequence:

1. **CAPS-EXT 3 (capability infrastructure, Mode 0 wiring)**: introduce `Capability` trait, `CapDispatcher`, `CapMode` enum, the six capability types, and the host's `ambient_caps()`. Wire the dispatcher into the runtime but route every check to ambient. PM-EXT 11/12/13 gates remain green; no observable behavior change.

2. **CAPS-EXT 4 (audit recorder, Mode 1)**: implement `AuditLog` + sidecar file writing + module-id-resolution helper. Add `--audit` CLI flag. Run the PM gates under `--audit` and confirm the audit log captures what we expect.

3. **CAPS-EXT 5 (synthetic-adversary probes)**: write `.mjs` files under `pilots/rusty-js-caps/probes/` for each attack class in Doc 736 §IV. Run under Mode 0 (every probe SUCCEEDS = attacker wins, the pre-state we're measuring against). The probe suite becomes the standing regression check for the rest of Pilot α.

4. **CAPS-EXT 6-7 (Fs capability enforcement, Mode 3)**: route all fs methods through `require_capability(fs_cap, FsOp::*)`. Add `--sealed` CLI flag. Add `cruftless-caps.json` parsing. Confirm: Mode-3-with-empty-caps run of the probe suite flips every fs probe to PASS (CapabilityError).

5. **CAPS-EXT 8 (Mode 2 wiring)**: add `--sealed-deps` flag, dispatcher branch on module provenance. Confirm: Mode-2 run with empty per-dep caps but ambient application caps flips dep-attempted-fs probes to PASS while leaving application-attempted-fs probes ambient.

6. **CAPS-EXT 9-12 (remaining capability classes)**: Process.exit, Stdio, Clock, Scheduler, Env, each in their own round.

7. **CAPS-EXT 13 (closure)**: run the full probe suite under Mode 3 with empty-caps. Every probe PASSES. The Doc 736 §IV impossibility claim is mechanically realized.

Verifier moves (Doc 736 §III Moves 2-5) follow Pilot α's closure as their own EXT rounds.

---

*The capability-API design closes CAPS-EXT 2. Next move: CAPS-EXT 3, the capability infrastructure with Mode 0 wiring. The dispatcher exists; behavior is unchanged from CAPS-EXT 0.*
