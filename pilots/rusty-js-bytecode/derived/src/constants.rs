//! Constants pool for compiled modules. Per design spec §III.

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Constant {
    Number(f64),
    BigInt(String),
    String(String),
    Regex { body: String, flags: String },
    /// Nested function prototype. Holds its own CompiledModule shape.
    Function(Box<crate::compiler::FunctionProto>),
}

// Ω.5.P03.E2.const-intern-hash: dedup index alongside the entries Vec.
// Pre-substrate, `intern` did a linear scan over `entries` on every call
// (`self.entries.iter().position(...)`). For an object-literal expression
// with N distinct property keys the compiler interns each key once, so a
// 16k-key literal cost ≈ 128M string comparisons — observed as O(N^2.17)
// on synthetic sweeps and as the 11.4s compile phase on sentry's 3.8MB
// CJS bundle.
//
// The HashMap is keyed only on the dedupable Constant variants
// (Number/BigInt/String/Regex). Function constants are unique per
// declaration site (see same_constant: Function/Function → false) so
// they bypass the map and append directly — preserves the existing
// no-dedup-for-functions contract.
//
// Invariants:
//   - For every (k, i) in dedup_index, entries[i] == k under same_constant
//   - For every i where entries[i] is dedup-eligible, the corresponding
//     key appears in dedup_index with value i
//   - Function entries are absent from dedup_index by construction
#[derive(Debug, Default, Clone)]
pub struct ConstantsPool {
    entries: Vec<Constant>,
    dedup_index: HashMap<DedupKey, u16>,
}

/// HashMap key for dedupable constants. Mirrors `same_constant` for the
/// four dedup-eligible variants; Function is intentionally absent.
/// f64 NaN bit-patterns are preserved via to_bits, matching the existing
/// equality discipline.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum DedupKey {
    Number(u64),
    BigInt(String),
    String(String),
    Regex { body: String, flags: String },
}

impl DedupKey {
    fn from_constant(c: &Constant) -> Option<Self> {
        match c {
            Constant::Number(x) => Some(DedupKey::Number(x.to_bits())),
            Constant::BigInt(x) => Some(DedupKey::BigInt(x.clone())),
            Constant::String(x) => Some(DedupKey::String(x.clone())),
            Constant::Regex { body, flags } => Some(DedupKey::Regex {
                body: body.clone(), flags: flags.clone(),
            }),
            Constant::Function(_) => None,
        }
    }
}

impl ConstantsPool {
    pub fn new() -> Self { Self::default() }

    /// Intern a constant. Equal constants return the same index. Numbers
    /// compare bit-for-bit (NaN bit-patterns are distinguished — caller
    /// should handle if needed). Function constants are never deduped.
    pub fn intern(&mut self, c: Constant) -> u16 {
        if let Some(key) = DedupKey::from_constant(&c) {
            if let Some(&idx) = self.dedup_index.get(&key) {
                return idx;
            }
            let idx = self.entries.len();
            assert!(idx < u16::MAX as usize, "constants pool overflow");
            self.entries.push(c);
            self.dedup_index.insert(key, idx as u16);
            return idx as u16;
        }
        // Function path: no dedup, no index entry.
        let idx = self.entries.len();
        assert!(idx < u16::MAX as usize, "constants pool overflow");
        self.entries.push(c);
        idx as u16
    }

    pub fn get(&self, idx: u16) -> Option<&Constant> {
        self.entries.get(idx as usize)
    }

    pub fn entries(&self) -> &[Constant] { &self.entries }

    pub fn len(&self) -> usize { self.entries.len() }
}

fn same_constant(a: &Constant, b: &Constant) -> bool {
    match (a, b) {
        (Constant::Number(x), Constant::Number(y)) => x.to_bits() == y.to_bits(),
        (Constant::BigInt(x), Constant::BigInt(y)) => x == y,
        (Constant::String(x), Constant::String(y)) => x == y,
        (Constant::Regex { body: b1, flags: f1 }, Constant::Regex { body: b2, flags: f2 }) =>
            b1 == b2 && f1 == f2,
        // Functions are unique per declaration site; never deduplicated.
        (Constant::Function(_), Constant::Function(_)) => false,
        _ => false,
    }
}
