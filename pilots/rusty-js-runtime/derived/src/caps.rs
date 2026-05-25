//! Capability infrastructure (CAPS-EXT 3, Pilot α of Doc 736).
//!
//! This module introduces the capability types + dispatcher + mode
//! enum. In Mode 0 (the only mode wired at CAPS-EXT 3 close), the
//! dispatcher exists but every check resolves to the ambient capability
//! — behavior is unchanged from pre-pilot Cruftless.
//!
//! Per `pilots/rusty-js-caps/docs/capability-api.md`. The substrate
//! moves that route effectful methods through `require_capability`
//! arrive at CAPS-EXT 6+.

use std::path::PathBuf;
use std::time::Duration;
use std::sync::Mutex;

// ---------------------------------------------------------------- mode

/// Per-process capability enforcement mode. Default `Compat` (Mode 0).
///
/// See Doc 736 §IX for the slider design. Mode determines whether the
/// dispatcher consults the capability handle or returns the ambient
/// authority unconditionally.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CapMode {
    /// Mode 0: Node-equivalent ambient authority. Dispatcher always
    /// returns ambient. Default.
    Compat,
    /// Mode 1: ambient authority + audit log of every effectful call.
    Audit,
    /// Mode 2: ambient for application modules; sealed for modules
    /// under `node_modules/`.
    SealedDeps,
    /// Mode 3: sealed everywhere. The Doc 736 §IV impossibility claim.
    Sealed,
}

impl Default for CapMode {
    fn default() -> Self { Self::Compat }
}

impl CapMode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Compat => "compat",
            Self::Audit => "audit",
            Self::SealedDeps => "sealed-deps",
            Self::Sealed => "sealed",
        }
    }

    /// Parse a CLI / env-var mode name. Recognized: "compat", "audit",
    /// "sealed-deps", "sealed". Unknown names map to None.
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "compat" | "0" => Some(Self::Compat),
            "audit" | "1" => Some(Self::Audit),
            "sealed-deps" | "2" => Some(Self::SealedDeps),
            "sealed" | "3" => Some(Self::Sealed),
            _ => None,
        }
    }
}

// ------------------------------------------------------------ provenance

/// Where a module was loaded from. Determines Mode-2 enforcement: deps
/// are sealed; the application is ambient.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ModuleProvenance {
    /// Application code (outside `node_modules/`).
    Application,
    /// Loaded from `<project>/node_modules/`.
    Dependency,
    /// File:// URL outside the project root. Treated as Dependency
    /// (deny-by-default) under Mode 2.
    External,
    /// Host-builtin facade (the `node:fs` etc. stubs). Treated as
    /// Application — these are part of the runtime, not user code.
    Builtin,
}

impl ModuleProvenance {
    pub fn is_application(&self) -> bool {
        matches!(self, Self::Application | Self::Builtin)
    }
}

/// Identity of the calling module for error and audit-log purposes.
#[derive(Debug, Clone)]
pub struct ModuleId {
    pub url: String,
    pub provenance: ModuleProvenance,
}

impl ModuleId {
    pub fn application(url: impl Into<String>) -> Self {
        Self { url: url.into(), provenance: ModuleProvenance::Application }
    }

    pub fn dependency(url: impl Into<String>) -> Self {
        Self { url: url.into(), provenance: ModuleProvenance::Dependency }
    }

    pub fn builtin(name: impl Into<String>) -> Self {
        Self { url: name.into(), provenance: ModuleProvenance::Builtin }
    }
}

// ------------------------------------------------------------ error type

#[derive(Debug, Clone)]
pub struct CapabilityError {
    pub capability: &'static str,
    pub operation: String,
    pub calling_module: String,
    pub mode: CapMode,
    pub hint: Option<String>,
}

impl std::fmt::Display for CapabilityError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}: no {} capability granted to module '{}' (mode: {})",
            self.capability, self.operation, self.capability,
            self.calling_module, self.mode.as_str())?;
        if let Some(hint) = &self.hint {
            write!(f, " — hint: {hint}")?;
        }
        Ok(())
    }
}

impl std::error::Error for CapabilityError {}

// ----------------------------------------------------------- Fs capability

#[derive(Debug, Clone)]
pub enum PathPolicy {
    None,
    Any,
    Prefix(PathBuf),
    Prefixes(Vec<PathBuf>),
    Exact(Vec<PathBuf>),
}

impl PathPolicy {
    pub fn allows(&self, path: &std::path::Path) -> bool {
        match self {
            Self::None => false,
            Self::Any => true,
            Self::Prefix(p) => path.starts_with(p),
            Self::Prefixes(ps) => ps.iter().any(|p| path.starts_with(p)),
            Self::Exact(ps) => ps.iter().any(|p| p == path),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Fs {
    pub read: PathPolicy,
    pub write: PathPolicy,
    pub list: PathPolicy,
    pub stat: PathPolicy,
    pub mkdir: PathPolicy,
    pub remove: PathPolicy,
}

impl Fs {
    /// Full ambient filesystem access. Returned by `AmbientCaps::fs`
    /// under Mode 0/1 and used by the host's root capability set.
    pub fn full() -> Self {
        Self {
            read: PathPolicy::Any,
            write: PathPolicy::Any,
            list: PathPolicy::Any,
            stat: PathPolicy::Any,
            mkdir: PathPolicy::Any,
            remove: PathPolicy::Any,
        }
    }

    /// No capability at all. Default for sealed deps without
    /// declarations.
    pub fn none() -> Self {
        Self {
            read: PathPolicy::None,
            write: PathPolicy::None,
            list: PathPolicy::None,
            stat: PathPolicy::None,
            mkdir: PathPolicy::None,
            remove: PathPolicy::None,
        }
    }

    /// Narrow every policy to paths under `prefix`.
    pub fn sub_dir(&self, prefix: impl Into<PathBuf>) -> Self {
        let prefix = prefix.into();
        let narrow = |p: &PathPolicy| -> PathPolicy {
            match p {
                PathPolicy::None => PathPolicy::None,
                _ => PathPolicy::Prefix(prefix.clone()),
            }
        };
        Self {
            read: narrow(&self.read),
            write: narrow(&self.write),
            list: narrow(&self.list),
            stat: narrow(&self.stat),
            mkdir: narrow(&self.mkdir),
            remove: narrow(&self.remove),
        }
    }

    pub fn read_only(&self) -> Self {
        Self {
            read: self.read.clone(),
            list: self.list.clone(),
            stat: self.stat.clone(),
            write: PathPolicy::None,
            mkdir: PathPolicy::None,
            remove: PathPolicy::None,
        }
    }
}

#[derive(Debug, Clone)]
pub enum FsOp {
    Read(PathBuf),
    Write(PathBuf),
    List(PathBuf),
    Stat(PathBuf),
    Mkdir(PathBuf),
    Remove(PathBuf),
}

impl FsOp {
    fn describe(&self) -> String {
        match self {
            Self::Read(p) => format!("read({})", p.display()),
            Self::Write(p) => format!("write({})", p.display()),
            Self::List(p) => format!("list({})", p.display()),
            Self::Stat(p) => format!("stat({})", p.display()),
            Self::Mkdir(p) => format!("mkdir({})", p.display()),
            Self::Remove(p) => format!("remove({})", p.display()),
        }
    }

    fn policy<'a>(&self, cap: &'a Fs) -> (&'a PathPolicy, &'static str, &std::path::Path) {
        match self {
            Self::Read(p) => (&cap.read, "read", p),
            Self::Write(p) => (&cap.write, "write", p),
            Self::List(p) => (&cap.list, "list", p),
            Self::Stat(p) => (&cap.stat, "stat", p),
            Self::Mkdir(p) => (&cap.mkdir, "mkdir", p),
            Self::Remove(p) => (&cap.remove, "remove", p),
        }
    }
}

// -------------------------------------------------------- Stdio capability

#[derive(Debug, Clone, Copy)]
pub struct Stdio {
    pub stdout: bool,
    pub stderr: bool,
}

impl Stdio {
    pub fn full() -> Self { Self { stdout: true, stderr: true } }
    pub fn none() -> Self { Self { stdout: false, stderr: false } }
}

#[derive(Debug, Clone)]
pub enum StdioOp {
    Stdout(Vec<u8>),
    Stderr(Vec<u8>),
}

// ---------------------------------------------------------- Clock capability

#[derive(Debug, Clone, Copy)]
pub enum ClockResolution {
    Disabled,
    Coarse(Duration),
    Fine,
}

#[derive(Debug, Clone, Copy)]
pub struct Clock {
    pub resolution: ClockResolution,
}

impl Clock {
    pub fn fine() -> Self { Self { resolution: ClockResolution::Fine } }
    pub fn disabled() -> Self { Self { resolution: ClockResolution::Disabled } }
    pub fn coarse(d: Duration) -> Self { Self { resolution: ClockResolution::Coarse(d) } }
}

#[derive(Debug, Clone, Copy)]
pub enum ClockOp {
    Now,
    HighResolution,
}

// ------------------------------------------------------ Scheduler capability

#[derive(Debug, Clone, Copy)]
pub struct Scheduler {
    pub timers: bool,
    pub microtasks: bool,
    pub min_delay: Duration,
}

impl Scheduler {
    pub fn full() -> Self {
        Self { timers: true, microtasks: true, min_delay: Duration::ZERO }
    }
    pub fn none() -> Self {
        Self { timers: false, microtasks: false, min_delay: Duration::ZERO }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum SchedulerOp {
    Timer(Duration),
    Microtask,
}

// -------------------------------------------------------- Process capability

#[derive(Debug, Clone, Copy)]
pub struct Process {
    pub may_exit: bool,
    pub may_read_cwd: bool,
    pub may_read_pid: bool,
}

impl Process {
    pub fn full() -> Self {
        Self { may_exit: true, may_read_cwd: true, may_read_pid: true }
    }
    pub fn none() -> Self {
        Self { may_exit: false, may_read_cwd: false, may_read_pid: false }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ProcessOp {
    Exit(i32),
    ReadCwd,
    ReadPid,
}

// ------------------------------------------------------------ Env capability

#[derive(Debug, Clone)]
pub enum EnvVarPolicy {
    None,
    Any,
    Whitelist(Vec<String>),
}

impl EnvVarPolicy {
    pub fn allows(&self, name: &str) -> bool {
        match self {
            Self::None => false,
            Self::Any => true,
            Self::Whitelist(ws) => ws.iter().any(|w| w == name),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Env {
    pub vars: EnvVarPolicy,
    pub system_info: bool,
}

impl Env {
    pub fn full() -> Self { Self { vars: EnvVarPolicy::Any, system_info: true } }
    pub fn none() -> Self { Self { vars: EnvVarPolicy::None, system_info: false } }
}

#[derive(Debug, Clone)]
pub enum EnvOp {
    ReadVar(String),
    SystemInfo(&'static str),  // e.g. "cpus", "hostname", "homedir"
}

// ------------------------------------------------------------ Net capability

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NetEndpoint {
    pub host: String,
    pub port: u16,
}

impl NetEndpoint {
    pub fn new(host: impl Into<String>, port: u16) -> Self {
        Self { host: host.into(), port }
    }

    fn matches(&self, host: &str, port: u16) -> bool {
        self.host == host && self.port == port
    }
}

#[derive(Debug, Clone)]
pub enum NetListenPolicy {
    None,
    Any,
    LoopbackAnyPort,
    Exact(Vec<NetEndpoint>),
}

impl NetListenPolicy {
    fn allows(&self, host: &str, port: u16) -> bool {
        match self {
            Self::None => false,
            Self::Any => true,
            Self::LoopbackAnyPort => is_loopback_host(host),
            Self::Exact(endpoints) => endpoints.iter().any(|ep| ep.matches(host, port)),
        }
    }
}

#[derive(Debug, Clone)]
pub enum NetConnectPolicy {
    None,
    Any,
    Hosts(Vec<String>),
    Exact(Vec<NetEndpoint>),
}

impl NetConnectPolicy {
    fn allows(&self, host: &str, port: u16) -> bool {
        match self {
            Self::None => false,
            Self::Any => true,
            Self::Hosts(hosts) => hosts.iter().any(|h| h == host),
            Self::Exact(endpoints) => endpoints.iter().any(|ep| ep.matches(host, port)),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Net {
    pub listen: NetListenPolicy,
    pub connect: NetConnectPolicy,
}

impl Net {
    pub fn full() -> Self {
        Self { listen: NetListenPolicy::Any, connect: NetConnectPolicy::Any }
    }

    pub fn none() -> Self {
        Self { listen: NetListenPolicy::None, connect: NetConnectPolicy::None }
    }

    pub fn loopback_server() -> Self {
        Self { listen: NetListenPolicy::LoopbackAnyPort, connect: NetConnectPolicy::None }
    }

    pub fn listen_exact(host: impl Into<String>, port: u16) -> Self {
        Self {
            listen: NetListenPolicy::Exact(vec![NetEndpoint::new(host, port)]),
            connect: NetConnectPolicy::None,
        }
    }
}

#[derive(Debug, Clone)]
pub enum NetOp {
    Listen { host: String, port: u16 },
    Connect { host: String, port: u16 },
}

impl NetOp {
    fn describe(&self) -> String {
        match self {
            Self::Listen { host, port } => format!("listen({host}:{port})"),
            Self::Connect { host, port } => format!("connect({host}:{port})"),
        }
    }

    fn allows(&self, cap: &Net) -> bool {
        match self {
            Self::Listen { host, port } => cap.listen.allows(host, *port),
            Self::Connect { host, port } => cap.connect.allows(host, *port),
        }
    }

    fn hint(&self) -> String {
        match self {
            Self::Listen { host, port } if is_loopback_host(host) => {
                format!("add to caps in package.json: {{ net: {{ listen: ['{host}:{port}', 'loopback:*'] }} }}")
            }
            Self::Listen { host, port } => {
                format!("add to caps in package.json: {{ net: {{ listen: ['{host}:{port}'] }} }}")
            }
            Self::Connect { host, port } => {
                format!("add to caps in package.json: {{ net: {{ connect: ['{host}:{port}'] }} }}")
            }
        }
    }
}

fn is_loopback_host(host: &str) -> bool {
    matches!(host, "127.0.0.1" | "localhost" | "::1" | "[::1]")
}

// ----------------------------------------------------------- AmbientCaps

/// The full-authority capability set the host holds. Mode 0 / Mode 1
/// return these unconditionally; Mode 2 returns these for application
/// modules; Mode 3 ignores them in favor of the explicitly-passed
/// dep capability.
#[derive(Debug, Clone)]
pub struct AmbientCaps {
    pub fs: Fs,
    pub stdio: Stdio,
    pub clock: Clock,
    pub scheduler: Scheduler,
    pub process: Process,
    pub env: Env,
    pub net: Net,
}

impl AmbientCaps {
    pub fn full() -> Self {
        Self {
            fs: Fs::full(),
            stdio: Stdio::full(),
            clock: Clock::fine(),
            scheduler: Scheduler::full(),
            process: Process::full(),
            env: Env::full(),
            net: Net::full(),
        }
    }
}

impl Default for AmbientCaps {
    fn default() -> Self { Self::full() }
}

// -------------------------------------------------------------- AuditLog

/// Records the (caller, capability, operation) tuple for every
/// effectful call in Mode 1. The first cut writes to stderr; a sidecar
/// file destination is a follow-on substrate move (CAPS-EXT 4
/// refinement).
#[derive(Debug, Default)]
pub struct AuditLog {
    pub records: Vec<AuditRecord>,
}

#[derive(Debug, Clone)]
pub struct AuditRecord {
    pub caller: String,
    pub capability: &'static str,
    pub operation: String,
    pub timestamp_micros: u128,
}

impl AuditLog {
    pub fn record(&mut self, caller: &ModuleId, capability: &'static str, op: &str) {
        let timestamp_micros = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_micros())
            .unwrap_or(0);
        self.records.push(AuditRecord {
            caller: caller.url.clone(),
            capability,
            operation: op.to_string(),
            timestamp_micros,
        });
    }
}

// ------------------------------------------------------------ Dispatcher

/// The capability dispatcher. Single point where mode policy and
/// capability policy compose. Every effectful operation in the runtime
/// routes through one of the `require_*` methods.
///
/// At CAPS-EXT 3 close, no effectful method calls these yet — the
/// dispatcher exists and is wired into the Runtime, but route-through
/// lands at CAPS-EXT 6+ per the implementation order in
/// `pilots/rusty-js-caps/docs/capability-api.md` §XIII.
pub struct CapDispatcher {
    pub mode: CapMode,
    pub ambient: AmbientCaps,
    pub audit: Mutex<AuditLog>,
}

impl CapDispatcher {
    pub fn new(mode: CapMode) -> Self {
        Self {
            mode,
            ambient: AmbientCaps::full(),
            audit: Mutex::new(AuditLog::default()),
        }
    }

    /// Default constructor — Mode 0 with full ambient. Identical to
    /// pre-pilot behavior; safe to construct unconditionally at host
    /// init.
    pub fn compat() -> Self {
        Self::new(CapMode::Compat)
    }

    /// Mode 1: Mode 0 + audit recording.
    pub fn audit_mode() -> Self {
        Self::new(CapMode::Audit)
    }

    /// Drain the audit log (Mode 1). Returns an empty vec under any
    /// other mode.
    pub fn drain_audit(&self) -> Vec<AuditRecord> {
        let mut g = self.audit.lock().expect("audit log poisoned");
        std::mem::take(&mut g.records)
    }

    fn record_audit(&self, caller: &ModuleId, capability: &'static str, op: &str) {
        if !matches!(self.mode, CapMode::Audit) { return; }
        if let Ok(mut g) = self.audit.lock() {
            g.record(caller, capability, op);
        }
    }

    pub fn require_fs(
        &self,
        cap: &Fs,
        op: FsOp,
        caller: &ModuleId,
    ) -> Result<(), CapabilityError> {
        let op_desc = op.describe();
        self.record_audit(caller, "fs", &op_desc);
        match self.mode {
            CapMode::Compat | CapMode::Audit => Ok(()),
            CapMode::SealedDeps if caller.provenance.is_application() => Ok(()),
            CapMode::SealedDeps | CapMode::Sealed => {
                let (policy, action, path) = op.policy(cap);
                if policy.allows(path) {
                    Ok(())
                } else {
                    Err(CapabilityError {
                        capability: "fs",
                        operation: op_desc,
                        calling_module: caller.url.clone(),
                        mode: self.mode,
                        hint: Some(format!(
                            "add to caps in package.json: {{ fs: {{ {action}: ['{}'] }} }}",
                            path.display())),
                    })
                }
            }
        }
    }

    pub fn require_stdio(
        &self,
        cap: &Stdio,
        op: StdioOp,
        caller: &ModuleId,
    ) -> Result<(), CapabilityError> {
        let (allowed, stream) = match &op {
            StdioOp::Stdout(_) => (cap.stdout, "stdout"),
            StdioOp::Stderr(_) => (cap.stderr, "stderr"),
        };
        let op_desc = format!("write({stream})");
        self.record_audit(caller, "stdio", &op_desc);
        match self.mode {
            CapMode::Compat | CapMode::Audit => Ok(()),
            CapMode::SealedDeps if caller.provenance.is_application() => Ok(()),
            CapMode::SealedDeps | CapMode::Sealed => {
                if allowed { Ok(()) } else {
                    Err(CapabilityError {
                        capability: "stdio",
                        operation: op_desc,
                        calling_module: caller.url.clone(),
                        mode: self.mode,
                        hint: Some(format!(
                            "add to caps in package.json: {{ stdio: {{ {stream}: true }} }}")),
                    })
                }
            }
        }
    }

    pub fn require_clock(
        &self,
        cap: &Clock,
        op: ClockOp,
        caller: &ModuleId,
    ) -> Result<(), CapabilityError> {
        let op_desc = match op {
            ClockOp::Now => "now()".to_string(),
            ClockOp::HighResolution => "highResolution()".to_string(),
        };
        self.record_audit(caller, "clock", &op_desc);
        match self.mode {
            CapMode::Compat | CapMode::Audit => Ok(()),
            CapMode::SealedDeps if caller.provenance.is_application() => Ok(()),
            CapMode::SealedDeps | CapMode::Sealed => {
                match cap.resolution {
                    ClockResolution::Disabled => Err(CapabilityError {
                        capability: "clock",
                        operation: op_desc,
                        calling_module: caller.url.clone(),
                        mode: self.mode,
                        hint: Some("add to caps in package.json: { clock: 'fine' } or { clock: { coarse: 100 } }".into()),
                    }),
                    _ => Ok(()),
                }
            }
        }
    }

    pub fn require_scheduler(
        &self,
        cap: &Scheduler,
        op: SchedulerOp,
        caller: &ModuleId,
    ) -> Result<(), CapabilityError> {
        let (allowed, kind, op_desc) = match op {
            SchedulerOp::Timer(d) => (cap.timers, "timers",
                format!("timer({}ms)", d.as_millis())),
            SchedulerOp::Microtask => (cap.microtasks, "microtasks", "microtask".to_string()),
        };
        self.record_audit(caller, "scheduler", &op_desc);
        match self.mode {
            CapMode::Compat | CapMode::Audit => Ok(()),
            CapMode::SealedDeps if caller.provenance.is_application() => Ok(()),
            CapMode::SealedDeps | CapMode::Sealed => {
                if allowed { Ok(()) } else {
                    Err(CapabilityError {
                        capability: "scheduler",
                        operation: op_desc,
                        calling_module: caller.url.clone(),
                        mode: self.mode,
                        hint: Some(format!(
                            "add to caps in package.json: {{ scheduler: {{ {kind}: true }} }}")),
                    })
                }
            }
        }
    }

    pub fn require_process(
        &self,
        cap: &Process,
        op: ProcessOp,
        caller: &ModuleId,
    ) -> Result<(), CapabilityError> {
        let (allowed, field, op_desc) = match op {
            ProcessOp::Exit(c) => (cap.may_exit, "may_exit", format!("exit({c})")),
            ProcessOp::ReadCwd => (cap.may_read_cwd, "may_read_cwd", "cwd()".into()),
            ProcessOp::ReadPid => (cap.may_read_pid, "may_read_pid", "pid".into()),
        };
        self.record_audit(caller, "process", &op_desc);
        match self.mode {
            CapMode::Compat | CapMode::Audit => Ok(()),
            CapMode::SealedDeps if caller.provenance.is_application() => Ok(()),
            CapMode::SealedDeps | CapMode::Sealed => {
                if allowed { Ok(()) } else {
                    Err(CapabilityError {
                        capability: "process",
                        operation: op_desc,
                        calling_module: caller.url.clone(),
                        mode: self.mode,
                        hint: Some(format!("add to caps in package.json: {{ process: {{ {field}: true }} }}")),
                    })
                }
            }
        }
    }

    pub fn require_env(
        &self,
        cap: &Env,
        op: EnvOp,
        caller: &ModuleId,
    ) -> Result<(), CapabilityError> {
        let op_desc = match &op {
            EnvOp::ReadVar(name) => format!("readVar({name})"),
            EnvOp::SystemInfo(field) => format!("systemInfo({field})"),
        };
        self.record_audit(caller, "env", &op_desc);
        match self.mode {
            CapMode::Compat | CapMode::Audit => Ok(()),
            CapMode::SealedDeps if caller.provenance.is_application() => Ok(()),
            CapMode::SealedDeps | CapMode::Sealed => {
                let allowed = match &op {
                    EnvOp::ReadVar(name) => cap.vars.allows(name),
                    EnvOp::SystemInfo(_) => cap.system_info,
                };
                if allowed { Ok(()) } else {
                    let hint = match &op {
                        EnvOp::ReadVar(name) => format!(
                            "add to caps in package.json: {{ env: {{ vars: ['{name}'] }} }}"),
                        EnvOp::SystemInfo(_) => "add to caps in package.json: { env: { systemInfo: true } }".into(),
                    };
                    Err(CapabilityError {
                        capability: "env",
                        operation: op_desc,
                        calling_module: caller.url.clone(),
                        mode: self.mode,
                        hint: Some(hint),
                    })
                }
            }
        }
    }

    pub fn require_net(
        &self,
        cap: &Net,
        op: NetOp,
        caller: &ModuleId,
    ) -> Result<(), CapabilityError> {
        let op_desc = op.describe();
        self.record_audit(caller, "net", &op_desc);
        match self.mode {
            CapMode::Compat | CapMode::Audit => Ok(()),
            CapMode::SealedDeps if caller.provenance.is_application() => Ok(()),
            CapMode::SealedDeps | CapMode::Sealed => {
                if op.allows(cap) {
                    Ok(())
                } else {
                    Err(CapabilityError {
                        capability: "net",
                        operation: op_desc,
                        calling_module: caller.url.clone(),
                        mode: self.mode,
                        hint: Some(op.hint()),
                    })
                }
            }
        }
    }
}

impl Default for CapDispatcher {
    fn default() -> Self { Self::compat() }
}

// ----------------------------------------------------------------- tests

#[cfg(test)]
mod tests {
    use super::*;

    fn app_caller() -> ModuleId { ModuleId::application("file:///proj/app.mjs") }
    fn dep_caller() -> ModuleId { ModuleId::dependency("file:///proj/node_modules/lodash/index.js") }

    #[test]
    fn mode_default_is_compat() {
        assert_eq!(CapMode::default(), CapMode::Compat);
    }

    #[test]
    fn cap_mode_parse() {
        assert_eq!(CapMode::from_str("compat"), Some(CapMode::Compat));
        assert_eq!(CapMode::from_str("audit"), Some(CapMode::Audit));
        assert_eq!(CapMode::from_str("sealed-deps"), Some(CapMode::SealedDeps));
        assert_eq!(CapMode::from_str("sealed"), Some(CapMode::Sealed));
        assert_eq!(CapMode::from_str("nope"), None);
    }

    #[test]
    fn compat_mode_allows_everything_no_cap() {
        let d = CapDispatcher::compat();
        // Even an empty Fs capability is ignored under Mode 0.
        let cap = Fs::none();
        let result = d.require_fs(
            &cap,
            FsOp::Read("/etc/passwd".into()),
            &dep_caller(),
        );
        assert!(result.is_ok());
    }

    #[test]
    fn audit_mode_allows_and_records() {
        let d = CapDispatcher::audit_mode();
        let cap = Fs::none();
        let r = d.require_fs(&cap, FsOp::Read("/etc/passwd".into()), &dep_caller());
        assert!(r.is_ok());
        let records = d.drain_audit();
        assert_eq!(records.len(), 1);
        assert_eq!(records[0].capability, "fs");
        assert!(records[0].operation.contains("/etc/passwd"));
    }

    #[test]
    fn sealed_deps_dep_blocked() {
        let d = CapDispatcher::new(CapMode::SealedDeps);
        let cap = Fs::none();
        let r = d.require_fs(&cap, FsOp::Read("/etc/passwd".into()), &dep_caller());
        assert!(r.is_err());
        let e = r.unwrap_err();
        assert_eq!(e.capability, "fs");
        assert!(e.hint.is_some());
    }

    #[test]
    fn sealed_deps_app_passes() {
        let d = CapDispatcher::new(CapMode::SealedDeps);
        let cap = Fs::none();
        // Application caller — Mode 2 returns ambient regardless of cap.
        let r = d.require_fs(&cap, FsOp::Read("/etc/passwd".into()), &app_caller());
        assert!(r.is_ok());
    }

    #[test]
    fn sealed_blocks_app_too() {
        let d = CapDispatcher::new(CapMode::Sealed);
        let cap = Fs::none();
        // Mode 3: even the application must hold the capability.
        let r = d.require_fs(&cap, FsOp::Read("/etc/passwd".into()), &app_caller());
        assert!(r.is_err());
    }

    #[test]
    fn fs_prefix_policy() {
        let cap = Fs {
            read: PathPolicy::Prefix("/proj/data".into()),
            ..Fs::none()
        };
        let d = CapDispatcher::new(CapMode::Sealed);
        let ok = d.require_fs(&cap, FsOp::Read("/proj/data/x.txt".into()), &dep_caller());
        assert!(ok.is_ok());
        let denied = d.require_fs(&cap, FsOp::Read("/etc/passwd".into()), &dep_caller());
        assert!(denied.is_err());
    }

    #[test]
    fn fs_sub_dir_narrows() {
        let cap = Fs::full().sub_dir("/proj/data");
        let d = CapDispatcher::new(CapMode::Sealed);
        assert!(d.require_fs(&cap, FsOp::Read("/proj/data/x".into()), &dep_caller()).is_ok());
        assert!(d.require_fs(&cap, FsOp::Read("/proj/secrets".into()), &dep_caller()).is_err());
    }

    #[test]
    fn fs_read_only_strips_writes() {
        let cap = Fs::full().read_only();
        let d = CapDispatcher::new(CapMode::Sealed);
        assert!(d.require_fs(&cap, FsOp::Read("/x".into()), &dep_caller()).is_ok());
        assert!(d.require_fs(&cap, FsOp::Write("/x".into()), &dep_caller()).is_err());
    }

    #[test]
    fn stdio_sealed_blocks_unless_granted() {
        let d = CapDispatcher::new(CapMode::Sealed);
        let none = Stdio::none();
        assert!(d.require_stdio(&none, StdioOp::Stdout(b"x".to_vec()), &dep_caller()).is_err());
        let stdout_only = Stdio { stdout: true, stderr: false };
        assert!(d.require_stdio(&stdout_only, StdioOp::Stdout(b"x".to_vec()), &dep_caller()).is_ok());
        assert!(d.require_stdio(&stdout_only, StdioOp::Stderr(b"x".to_vec()), &dep_caller()).is_err());
    }

    #[test]
    fn clock_disabled_blocks() {
        let d = CapDispatcher::new(CapMode::Sealed);
        let cap = Clock::disabled();
        assert!(d.require_clock(&cap, ClockOp::Now, &dep_caller()).is_err());
        let cap2 = Clock::fine();
        assert!(d.require_clock(&cap2, ClockOp::Now, &dep_caller()).is_ok());
    }

    #[test]
    fn process_exit_gated() {
        let d = CapDispatcher::new(CapMode::Sealed);
        let none = Process::none();
        assert!(d.require_process(&none, ProcessOp::Exit(1), &dep_caller()).is_err());
        let full = Process::full();
        assert!(d.require_process(&full, ProcessOp::Exit(1), &dep_caller()).is_ok());
    }

    #[test]
    fn env_whitelist() {
        let cap = Env {
            vars: EnvVarPolicy::Whitelist(vec!["LANG".into(), "TZ".into()]),
            system_info: false,
        };
        let d = CapDispatcher::new(CapMode::Sealed);
        assert!(d.require_env(&cap, EnvOp::ReadVar("LANG".into()), &dep_caller()).is_ok());
        assert!(d.require_env(&cap, EnvOp::ReadVar("AWS_KEY".into()), &dep_caller()).is_err());
        assert!(d.require_env(&cap, EnvOp::SystemInfo("cpus"), &dep_caller()).is_err());
    }

    #[test]
    fn net_loopback_listen_policy() {
        let d = CapDispatcher::new(CapMode::Sealed);
        let cap = Net::loopback_server();
        assert!(d.require_net(&cap, NetOp::Listen { host: "127.0.0.1".into(), port: 0 }, &dep_caller()).is_ok());
        assert!(d.require_net(&cap, NetOp::Listen { host: "localhost".into(), port: 3000 }, &dep_caller()).is_ok());
        assert!(d.require_net(&cap, NetOp::Listen { host: "0.0.0.0".into(), port: 0 }, &dep_caller()).is_err());
    }

    #[test]
    fn net_exact_listen_policy() {
        let d = CapDispatcher::new(CapMode::Sealed);
        let cap = Net::listen_exact("127.0.0.1", 8080);
        assert!(d.require_net(&cap, NetOp::Listen { host: "127.0.0.1".into(), port: 8080 }, &dep_caller()).is_ok());
        assert!(d.require_net(&cap, NetOp::Listen { host: "127.0.0.1".into(), port: 8081 }, &dep_caller()).is_err());
    }

    #[test]
    fn net_audit_records_listen() {
        let d = CapDispatcher::audit_mode();
        let cap = Net::none();
        let r = d.require_net(&cap, NetOp::Listen { host: "127.0.0.1".into(), port: 0 }, &dep_caller());
        assert!(r.is_ok());
        let records = d.drain_audit();
        assert_eq!(records.len(), 1);
        assert_eq!(records[0].capability, "net");
        assert_eq!(records[0].operation, "listen(127.0.0.1:0)");
    }

    #[test]
    fn net_sealed_deps_app_passes() {
        let d = CapDispatcher::new(CapMode::SealedDeps);
        let cap = Net::none();
        assert!(d.require_net(&cap, NetOp::Listen { host: "0.0.0.0".into(), port: 0 }, &app_caller()).is_ok());
        assert!(d.require_net(&cap, NetOp::Listen { host: "0.0.0.0".into(), port: 0 }, &dep_caller()).is_err());
    }

    #[test]
    fn capability_error_display() {
        let d = CapDispatcher::new(CapMode::Sealed);
        let cap = Fs::none();
        let e = d.require_fs(&cap, FsOp::Read("/etc/passwd".into()), &dep_caller()).unwrap_err();
        let s = format!("{e}");
        assert!(s.contains("fs"));
        assert!(s.contains("/etc/passwd"));
        assert!(s.contains("sealed"));
        assert!(s.contains("hint"));
    }
}
