//! Shape — the immutable shape descriptor identifying a property-layout class.
//!
//! Per shapes pilot docs/shape-design.md §2. Two Objects share an
//! `Rc<Shape>` (compared by `Rc::ptr_eq`) iff they were constructed
//! by the same property-addition history. This identity invariant is
//! the load-bearing property that makes `(shape_ptr, slot_offset)`
//! usable as the IC fast-path cache key.

use smallvec::SmallVec;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

/// Inline storage caps for the SmallOrLarge variants. Modal cases
/// per Shape-EXT 1 §8: most shapes carry < 16 slots and branch < 4
/// ways; the inline form covers the modal path with O(n) linear scan
/// (faster than HashMap probe at small n on the engagement's Pi
/// target).
const SLOTS_INLINE_CAP: usize = 8;
const TRANSITIONS_INLINE_CAP: usize = 4;

/// Maximum slot index. u32 covers any plausible Object far below the
/// IndexMap fallback ceiling. IC stubs encode slot indices as u32.
pub type SlotIndex = u32;

pub struct Shape {
    /// Eager-denormalized name → slot map. Every shape carries the
    /// FULL map (not a parent-delta) per design §2 lookup-cheap-over-
    /// memory-cheap decision.
    slots: SmallOrLargeSlotMap,

    /// Forward transitions to child shapes. Keyed on added property
    /// name. Interior-mutable per design §6 (transitions are added
    /// incrementally over the shape's lifetime, but the shape itself
    /// is shared `Rc<Shape>`).
    transitions: RefCell<SmallOrLargeTransitionMap>,

    /// Parent pointer for diagnosability and the potential closure-
    /// round refactor that drops eager-denorm in favor of parent walks.
    /// Not load-bearing for v1 lookup or enumeration.
    parent: Option<Rc<Shape>>,

    /// Cached slot count. Equal to `slots.len()`.
    slot_count: SlotIndex,
}

/// Two-form map for the slots: linear scan for small shapes, hashmap
/// for large. Migration is one-way (Small → Large) at INLINE_CAP + 1.
enum SmallOrLargeSlotMap {
    Small(SmallVec<[(String, SlotIndex); SLOTS_INLINE_CAP]>),
    Large(Vec<(String, SlotIndex)>, HashMap<String, SlotIndex>),
}

impl SmallOrLargeSlotMap {
    fn new() -> Self { Self::Small(SmallVec::new()) }

    fn len(&self) -> usize {
        match self {
            Self::Small(v) => v.len(),
            Self::Large(v, _) => v.len(),
        }
    }

    fn get(&self, name: &str) -> Option<SlotIndex> {
        match self {
            Self::Small(v) => v.iter().find(|(n, _)| n == name).map(|(_, s)| *s),
            Self::Large(_, h) => h.get(name).copied(),
        }
    }

    fn iter(&self) -> Box<dyn Iterator<Item = (&str, SlotIndex)> + '_> {
        match self {
            Self::Small(v) => Box::new(v.iter().map(|(n, s)| (n.as_str(), *s))),
            Self::Large(v, _) => Box::new(v.iter().map(|(n, s)| (n.as_str(), *s))),
        }
    }

    /// Insert at the next slot index. Returns the assigned slot.
    /// Caller guarantees `name` is not already present.
    fn push(&mut self, name: String) -> SlotIndex {
        let new_slot = self.len() as SlotIndex;
        match self {
            Self::Small(v) if v.len() < SLOTS_INLINE_CAP => {
                v.push((name, new_slot));
            }
            Self::Small(_) => {
                // Promote to Large at SLOTS_INLINE_CAP + 1.
                let Self::Small(old) = std::mem::replace(self, Self::Large(Vec::new(), HashMap::new())) else { unreachable!() };
                let Self::Large(vec, map) = self else { unreachable!() };
                for (n, s) in old {
                    map.insert(n.clone(), s);
                    vec.push((n, s));
                }
                map.insert(name.clone(), new_slot);
                vec.push((name, new_slot));
            }
            Self::Large(v, h) => {
                h.insert(name.clone(), new_slot);
                v.push((name, new_slot));
            }
        }
        new_slot
    }

    /// Deep-clone for child-shape construction. Used by transition_to.
    fn cloned(&self) -> Self {
        match self {
            Self::Small(v) => Self::Small(v.clone()),
            Self::Large(v, h) => Self::Large(v.clone(), h.clone()),
        }
    }
}

/// Two-form map for the transitions: linear scan for small fanout,
/// hashmap for large.
enum SmallOrLargeTransitionMap {
    Small(SmallVec<[(String, Rc<Shape>); TRANSITIONS_INLINE_CAP]>),
    Large(HashMap<String, Rc<Shape>>),
}

impl SmallOrLargeTransitionMap {
    fn new() -> Self { Self::Small(SmallVec::new()) }

    fn len(&self) -> usize {
        match self {
            Self::Small(v) => v.len(),
            Self::Large(h) => h.len(),
        }
    }

    fn get(&self, name: &str) -> Option<Rc<Shape>> {
        match self {
            Self::Small(v) => v.iter().find(|(n, _)| n == name).map(|(_, s)| Rc::clone(s)),
            Self::Large(h) => h.get(name).cloned(),
        }
    }

    fn insert(&mut self, name: String, shape: Rc<Shape>) {
        match self {
            Self::Small(v) if v.len() < TRANSITIONS_INLINE_CAP => {
                v.push((name, shape));
            }
            Self::Small(_) => {
                let Self::Small(old) = std::mem::replace(self, Self::Large(HashMap::new())) else { unreachable!() };
                let Self::Large(h) = self else { unreachable!() };
                for (n, s) in old {
                    h.insert(n, s);
                }
                h.insert(name, shape);
            }
            Self::Large(h) => {
                h.insert(name, shape);
            }
        }
    }
}

impl Shape {
    /// Root shape per design §4: empty slots, empty transitions,
    /// no parent. Thread-local singleton so every caller sees the same
    /// Rc<Shape> root; identity invariant requires this (per Pred-shape.2).
    /// Cruftless's runtime is single-threaded so thread-local is sufficient.
    pub fn root() -> Rc<Shape> {
        thread_local! {
            static ROOT: Rc<Shape> = Rc::new(Shape {
                slots: SmallOrLargeSlotMap::new(),
                transitions: RefCell::new(SmallOrLargeTransitionMap::new()),
                parent: None,
                slot_count: 0,
            });
        }
        ROOT.with(|r| Rc::clone(r))
    }

    /// Return the shape that results from adding `name` to this shape.
    ///
    /// Identity invariant: reuses an existing child shape if the
    /// transition already exists. THIS REUSE IS THE LOAD-BEARING
    /// IDENTITY GATE per design §3. A bug here breaks Pred-shape.2
    /// (same-history Objects sharing `Rc::ptr_eq` shape pointers).
    pub fn transition_to(self: &Rc<Shape>, name: &str) -> Rc<Shape> {
        // Identity gate: check parent's transitions for an existing
        // child shape with this addition. RefCell::borrow scope is
        // narrow so the .borrow_mut below doesn't conflict.
        if let Some(existing) = self.transitions.borrow().get(name) {
            return existing;
        }
        // Allocate child. Deep-clone the slots map + insert; assign
        // next slot index; carry forward parent pointer.
        let mut child_slots = self.slots.cloned();
        let _new_slot = child_slots.push(name.to_string());
        let child = Rc::new(Shape {
            slots: child_slots,
            transitions: RefCell::new(SmallOrLargeTransitionMap::new()),
            parent: Some(Rc::clone(self)),
            slot_count: self.slot_count + 1,
        });
        // Register in this shape's transition table.
        self.transitions.borrow_mut().insert(name.to_string(), Rc::clone(&child));
        child
    }

    /// Lookup the slot index for `name`. None if not in this shape.
    pub fn slot_of(&self, name: &str) -> Option<SlotIndex> {
        self.slots.get(name)
    }

    /// Number of slots in this shape.
    pub fn slot_count(&self) -> SlotIndex {
        self.slot_count
    }

    /// Iterate (name, slot_index) pairs in insertion order. The
    /// ECMA §10.1.11 enumeration-order primary source.
    pub fn iter_slots(&self) -> impl Iterator<Item = (&str, SlotIndex)> + '_ {
        self.slots.iter()
    }

    /// Parent shape, if any. None for the root shape.
    pub fn parent(self: &Rc<Shape>) -> Option<Rc<Shape>> {
        self.parent.as_ref().map(Rc::clone)
    }

    /// Raw pointer for IC stub cache-key emission. Per design §11:
    /// valid as long as some Rc<Shape> references the allocation. IC
    /// stub must keep `Rc<Shape>` alongside the cached pointer.
    pub fn as_raw_ptr(self: &Rc<Shape>) -> *const Shape {
        Rc::as_ptr(self)
    }

    /// Diagnostic-only transition-table size. Closure-round usage:
    /// Pilot LeJIT-Σ's bench harness can use this to characterize
    /// transition-tree fanout in real workloads.
    pub fn transition_count(&self) -> usize {
        self.transitions.borrow().len()
    }
}

impl std::fmt::Debug for Shape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Shape")
            .field("slot_count", &self.slot_count)
            .field("transition_count", &self.transition_count())
            .field("slots", &self.slots.iter().collect::<Vec<_>>())
            .finish()
    }
}

// =========================================================================
// Tests
// =========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn root_is_empty() {
        let root = Shape::root();
        assert_eq!(root.slot_count(), 0);
        assert!(root.slot_of("any").is_none());
        assert!(root.parent().is_none());
        assert_eq!(root.iter_slots().count(), 0);
    }

    #[test]
    fn single_transition_assigns_slot_zero() {
        let root = Shape::root();
        let s = root.transition_to("x");
        assert_eq!(s.slot_count(), 1);
        assert_eq!(s.slot_of("x"), Some(0));
        assert!(s.parent().is_some());
        assert!(Rc::ptr_eq(&s.parent().unwrap(), &root));
    }

    /// Pred-shape.2 corroboration: two transition_to calls with the
    /// same name from the same parent return Rc::ptr_eq shapes. This
    /// is the load-bearing identity gate.
    #[test]
    fn same_transition_same_shape() {
        let root = Shape::root();
        let a = root.transition_to("x");
        let b = root.transition_to("x");
        assert!(Rc::ptr_eq(&a, &b), "same transition must reuse shape (Pred-shape.2)");
    }

    /// Different-name transitions produce distinct shapes.
    #[test]
    fn different_transitions_distinct_shapes() {
        let root = Shape::root();
        let a = root.transition_to("x");
        let b = root.transition_to("y");
        assert!(!Rc::ptr_eq(&a, &b));
        assert_eq!(a.slot_of("x"), Some(0));
        assert_eq!(b.slot_of("y"), Some(0));
        assert!(a.slot_of("y").is_none());
        assert!(b.slot_of("x").is_none());
    }

    /// Chain of N transitions: same property-addition history yields
    /// the same shape pointer, slot indices in insertion order.
    #[test]
    fn chain_preserves_insertion_order_and_identity() {
        let root = Shape::root();
        let path_a = root.transition_to("x").transition_to("y").transition_to("z");
        let path_b = root.transition_to("x").transition_to("y").transition_to("z");
        assert!(Rc::ptr_eq(&path_a, &path_b));
        assert_eq!(path_a.slot_count(), 3);
        assert_eq!(path_a.slot_of("x"), Some(0));
        assert_eq!(path_a.slot_of("y"), Some(1));
        assert_eq!(path_a.slot_of("z"), Some(2));
        let names: Vec<&str> = path_a.iter_slots().map(|(n, _)| n).collect();
        assert_eq!(names, vec!["x", "y", "z"]);
    }

    /// Order-divergent chains produce distinct shapes (Object{x,y} !=
    /// Object{y,x} at the shape tier).
    #[test]
    fn order_divergent_chains_distinct() {
        let root = Shape::root();
        let xy = root.transition_to("x").transition_to("y");
        let yx = root.transition_to("y").transition_to("x");
        assert!(!Rc::ptr_eq(&xy, &yx));
        assert_eq!(xy.slot_of("x"), Some(0));
        assert_eq!(xy.slot_of("y"), Some(1));
        assert_eq!(yx.slot_of("y"), Some(0));
        assert_eq!(yx.slot_of("x"), Some(1));
    }

    /// SmallOrLargeSlotMap promotion: at SLOTS_INLINE_CAP + 1 the
    /// map transitions from Small to Large. Verify both forms behave
    /// identically.
    #[test]
    fn slot_map_promotes_past_inline_cap() {
        let root = Shape::root();
        let mut cur = root;
        for i in 0..(SLOTS_INLINE_CAP + 2) {
            cur = cur.transition_to(&format!("p{}", i));
        }
        assert_eq!(cur.slot_count() as usize, SLOTS_INLINE_CAP + 2);
        for i in 0..(SLOTS_INLINE_CAP + 2) {
            assert_eq!(cur.slot_of(&format!("p{}", i)), Some(i as SlotIndex));
        }
        // Insertion order preserved through the promotion.
        let names: Vec<String> = cur.iter_slots().map(|(n, _)| n.to_string()).collect();
        let expected: Vec<String> = (0..(SLOTS_INLINE_CAP + 2)).map(|i| format!("p{}", i)).collect();
        assert_eq!(names, expected);
    }

    /// SmallOrLargeTransitionMap promotion: at TRANSITIONS_INLINE_CAP + 1
    /// the per-shape transition map promotes. Identity invariant must
    /// hold across the promotion.
    #[test]
    fn transition_map_promotes_past_inline_cap() {
        let root = Shape::root();
        let mut children: Vec<Rc<Shape>> = Vec::new();
        for i in 0..(TRANSITIONS_INLINE_CAP + 2) {
            children.push(root.transition_to(&format!("k{}", i)));
        }
        // Re-request each transition; identity must hold.
        for (i, child) in children.iter().enumerate() {
            let again = root.transition_to(&format!("k{}", i));
            assert!(Rc::ptr_eq(child, &again), "identity must hold across map promotion");
        }
        assert_eq!(root.transition_count(), TRANSITIONS_INLINE_CAP + 2);
    }

    /// Pred-shape.3 bounded corroboration: a randomized workload's
    /// shape count grows linearly in unique transition paths, not
    /// exponentially. Bounded probe; full corroboration awaits
    /// Shape-EXT 4+ with the diff-prod gate active.
    #[test]
    fn shape_count_linear_in_unique_paths() {
        let root = Shape::root();
        // 100 objects, each adds 5 properties from a 10-name pool in
        // arbitrary but order-stable sequences. Worst-case shape
        // count is the number of distinct ordered 5-prefixes of the
        // property-addition sequences, bounded by the count of
        // distinct sequences themselves.
        let pool: Vec<String> = (0..10).map(|i| format!("p{}", i)).collect();
        // 5 distinct sequences exercised across 100 "objects":
        let sequences: Vec<Vec<&String>> = vec![
            vec![&pool[0], &pool[1], &pool[2], &pool[3], &pool[4]],
            vec![&pool[0], &pool[1], &pool[2], &pool[3], &pool[5]],
            vec![&pool[0], &pool[1], &pool[2], &pool[6], &pool[7]],
            vec![&pool[0], &pool[1], &pool[8], &pool[9], &pool[4]],
            vec![&pool[5], &pool[6], &pool[7], &pool[8], &pool[9]],
        ];
        let mut leaf_shapes: Vec<Rc<Shape>> = Vec::new();
        for _obj in 0..100 {
            for seq in &sequences {
                let mut cur = Rc::clone(&root);
                for name in seq {
                    cur = cur.transition_to(name);
                }
                leaf_shapes.push(cur);
            }
        }
        // 5 distinct sequences → at most 5 distinct leaf shapes.
        let mut distinct_leaves: Vec<*const Shape> = leaf_shapes.iter().map(|s| Rc::as_ptr(s)).collect();
        distinct_leaves.sort();
        distinct_leaves.dedup();
        assert_eq!(distinct_leaves.len(), 5, "leaf shape count must equal distinct-sequence count (Pred-shape.3)");
    }

    /// IC consumer API: as_raw_ptr returns the Rc's pointer; stable
    /// while any Rc holds the allocation.
    #[test]
    fn as_raw_ptr_is_rc_pointer() {
        let root = Shape::root();
        let s = root.transition_to("x");
        let ptr = s.as_raw_ptr();
        assert_eq!(ptr, Rc::as_ptr(&s));
        // Hold another Rc; pointer stable.
        let s2 = Rc::clone(&s);
        assert_eq!(s2.as_raw_ptr(), ptr);
    }
}
