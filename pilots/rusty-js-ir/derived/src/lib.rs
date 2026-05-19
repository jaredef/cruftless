//! rusty-js-ir — Spec-as-source-of-truth IR for ECMA-262 algorithm sections.
//!
//! This crate is resolver-instance #0b/#0c per IR-DESIGN.md §0: it accepts
//! an IR function (constructed from ECMA-262 algorithm steps) and lowers it
//! to Rust source that targets the rusty-js-runtime Runtime helper surface.
//!
//! Tier 1 scope per IR-DESIGN.md §7.1 — minimal viable IR:
//!   - Hand-constructed IR functions (no spec-XML parser yet).
//!   - Lowering to Rust source as String (no proc-macro yet).
//!   - Linter signature stubbed; full spec-vs-IR diff deferred to Tier 2.
//!
//! Composes with seed §A8.28–§A8.33.

pub mod ir;
pub mod lower;
pub mod lint;

pub mod sections; // Hand-translated spec sections live here.

pub use ir::{ErrorClass, Expr, IRFunction, IRNode, Slot, Step};
pub use lint::{LintFinding, LintReport};
pub use lower::lower_to_rust;
