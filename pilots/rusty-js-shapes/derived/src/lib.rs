//! rusty-js-shapes — hidden classes (shapes) for cruftless's Object
//! representation. Substrate-introduction round per cruftless seed
//! §A8.13; consumed by Pilot LeJIT-Σ as the IC fast-path cache-key
//! supplier per LeJIT seed §I.2.
//!
//! See `pilots/rusty-js-shapes/docs/shape-design.md` for the design
//! decisions this crate implements. See seed.md §III for the
//! methodology staging Shape-EXT 0 through Shape-EXT 8+.
//!
//! This crate intentionally does NOT depend on `rusty-js-runtime`.
//! Doing so would create a cycle (rusty-js-runtime depends on this
//! crate via `Object::storage: ObjectStorage`). The dependency
//! direction is fixed: `Shape` is value-payload-agnostic and lives
//! here; `ObjectStorage` and its `Vec<Value>` payload live in
//! `rusty-js-runtime`. The IC consumer API on `Object` likewise
//! lives in `rusty-js-runtime` because `Object` does.

pub mod shape;

pub use shape::Shape;
