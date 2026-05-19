//! Hand-translated ECMA-262 algorithm sections.
//!
//! Tier 1 scope per IR-DESIGN.md §7.1 — one representative section
//! (Array.prototype.map §23.1.3.20). Tier 2 will add the spec-XML parser
//! to derive these from ECMA-262 directly.

pub mod array_prototype_map;
pub mod array_prototype_iteration; // forEach, filter, every, some
pub mod array_prototype_find;      // find, findIndex, findLast, findLastIndex
pub mod array_prototype_index_search; // indexOf, includes (lastIndexOf queued)
pub mod array_prototype_reduce;       // reduce (reduceRight queued, awaits signed-Int)
