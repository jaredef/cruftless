//! LeJIT-Τ TB-EXT 3a: TinyBaselineMetadata substrate-introduction.
//!
//! Per Doc 729 §A8.13 substrate-amortization staging: this module is
//! the substrate-introduction round. The metadata struct holds the
//! compile-time-resolvable facts that TB-EXT 3b's inline call thunk
//! will read once at thunk-build time instead of re-deriving them at
//! each call. No thunk emission in this round; the struct + table
//! is the apparatus that TB-EXT 3b consumes.
//!
//! Per Doc 738 §II.e: module path encodes substrate pillar (engine
//! optimization tier). Per §II.b: function names use post-§A8.32
//! receiver-discriminated form (no `_via`; these are JIT-emitter-
//! adjacent helpers, not Runtime-dispatching).
//!
//! Cost-reclaim classification per TB-EXT 2's decomposition:
//!  - compile-time-resolve: jit_fn_ptr, params, proto_id, bytecode_len
//!  - the per-closure facts (actual_this, is_arrow) stay on the
//!    closure side; the thunk (TB-EXT 3b) reads them via the closure
//!    pointer passed at call entry. Metadata is per-proto, closure
//!    facts are per-instance.
//!
//! Env flag `CRUFTLESS_LEJIT_TB=1` opts into metadata construction
//! at compile time. With the flag unset, no metadata is built and
//! no per-call cost is added (the metadata is None on CompiledFn).

/// Per-JIT-function metadata the TB-EXT 3b inline call thunk reads
/// once at thunk-build time. Substrate-introduction per TB-EXT 3a;
/// consumed by the thunk at TB-EXT 3b.
///
/// The struct holds proto-tier facts only. Per-closure facts
/// (`actual_this`, `is_arrow`, `bound_this`) live on the Closure
/// itself and are read by the thunk via the closure pointer.
#[derive(Debug, Clone)]
pub struct TinyBaselineMetadata {
    /// Raw pointer to the JitFn enum's discriminant. The TB-EXT 3b
    /// thunk reads this once at thunk-build to bake a direct call
    /// target into the inline preamble, bypassing the dispatcher's
    /// per-call `jit_cache.get(&proto_key)` HashMap lookup.
    ///
    /// Lifetime: the JitFn lives in the leaked JITModule per
    /// translator.rs:655 (`_module: &'static mut JITModule`); the
    /// pointer is stable for the process lifetime.
    pub jit_fn_ptr: usize,

    /// Param count baked at compile time. Used by the thunk's
    /// eligibility check (TB-EXT 3b fires only for params ∈ {1, 2})
    /// and by per-arg dispatch arm selection.
    pub params: u16,

    /// Bytecode length at compile time. Used by the TB eligibility
    /// gate (per seed §IV: ≤20 ops for first-cut tiny-baseline).
    /// A function whose bytecode exceeds the threshold stays on
    /// the standard dispatcher path.
    pub bytecode_len: usize,

    /// True iff the function's bytecode length is within the TB
    /// eligibility threshold. Computed at metadata-build time so
    /// the thunk's per-call eligibility check is a single bool load.
    pub tb_eligible: bool,
}

/// TB-EXT 3a first-cut eligibility threshold per seed §IV
/// (`≤20-op functions` carve-out). Bytecode byte count is a
/// conservative proxy for op count (each op is 1-3 bytes); the
/// threshold here is 60 bytes ≈ 20-30 ops upper bound.
pub const TB_BYTECODE_LEN_THRESHOLD: usize = 60;

impl TinyBaselineMetadata {
    /// Construct metadata from compile-time-known proto facts plus
    /// the freshly-compiled JitFn pointer. Called from
    /// `translator::compile_function` when `CRUFTLESS_LEJIT_TB=1`
    /// AND the function is param-eligible (1-2 args).
    pub fn build(jit_fn_ptr: usize, params: u16, bytecode_len: usize) -> Self {
        let tb_eligible = bytecode_len <= TB_BYTECODE_LEN_THRESHOLD && (params == 1 || params == 2);
        Self {
            jit_fn_ptr,
            params,
            bytecode_len,
            tb_eligible,
        }
    }

    /// Returns true iff this metadata's function is eligible for
    /// thunk dispatch under TB-EXT 3b. The dispatcher under TB=1
    /// reads this once per call to decide whether to route through
    /// the thunk or fall through to the standard path.
    pub fn eligible(&self) -> bool {
        self.tb_eligible
    }
}

/// Read the `CRUFTLESS_LEJIT_TB` env flag.
///
/// LeJIT-Τ TB-EXT 8 (2026-05-23): default-on flip authorized by
/// keeper after TB-EXT 7's three-probe-levels gate satisfied
/// (bench: TB+STUB 81 ns on bench_ic; consumer-route: diff-prod
/// 42/42 under default + TB=1; fuzz: fuzz-tb.mjs 3/3 configs
/// byte-identical post-Box-wrap fix). Opt out via
/// `CRUFTLESS_LEJIT_TB=0`.
pub fn lejit_tb_enabled() -> bool {
    std::env::var("CRUFTLESS_LEJIT_TB")
        .map(|v| !(v == "0" || v.eq_ignore_ascii_case("false")))
        .unwrap_or(true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_minimal_eligible() {
        let m = TinyBaselineMetadata::build(0xdeadbeef, 1, 20);
        assert_eq!(m.jit_fn_ptr, 0xdeadbeef);
        assert_eq!(m.params, 1);
        assert_eq!(m.bytecode_len, 20);
        assert!(m.tb_eligible);
        assert!(m.eligible());
    }

    #[test]
    fn ineligible_due_to_size() {
        let m = TinyBaselineMetadata::build(0x1000, 1, 100);
        assert!(!m.eligible(), "100-byte function exceeds threshold");
    }

    #[test]
    fn ineligible_due_to_arity() {
        let m0 = TinyBaselineMetadata::build(0x1000, 0, 10);
        let m3 = TinyBaselineMetadata::build(0x1000, 3, 10);
        assert!(!m0.eligible(), "0-arg ineligible at first cut");
        assert!(!m3.eligible(), "3+-arg ineligible at first cut");
    }

    #[test]
    fn boundary_at_threshold() {
        let m_at = TinyBaselineMetadata::build(0x1000, 1, TB_BYTECODE_LEN_THRESHOLD);
        let m_over = TinyBaselineMetadata::build(0x1000, 1, TB_BYTECODE_LEN_THRESHOLD + 1);
        assert!(m_at.eligible(), "at-threshold is inclusive");
        assert!(!m_over.eligible(), "over-threshold ineligible");
    }

    #[test]
    fn two_arg_eligible() {
        let m = TinyBaselineMetadata::build(0x1000, 2, 10);
        assert!(m.eligible());
    }

    #[test]
    fn env_flag_default_on_post_tb_ext_8() {
        // TB-EXT 8 default-on flip (2026-05-23): no flag set → ON.
        // Mirrors STUB's StubE-EXT 8 default-on flip pattern.
        std::env::remove_var("CRUFTLESS_LEJIT_TB");
        assert!(lejit_tb_enabled());
    }

    #[test]
    fn env_flag_opt_out_via_zero() {
        std::env::set_var("CRUFTLESS_LEJIT_TB", "0");
        assert!(!lejit_tb_enabled());
        std::env::remove_var("CRUFTLESS_LEJIT_TB");
    }

    #[test]
    fn env_flag_opt_out_via_false_case_insensitive() {
        std::env::set_var("CRUFTLESS_LEJIT_TB", "FaLsE");
        assert!(!lejit_tb_enabled());
        std::env::remove_var("CRUFTLESS_LEJIT_TB");
    }

    #[test]
    fn env_flag_on_via_one_explicit() {
        std::env::set_var("CRUFTLESS_LEJIT_TB", "1");
        assert!(lejit_tb_enabled());
        std::env::remove_var("CRUFTLESS_LEJIT_TB");
    }
}
