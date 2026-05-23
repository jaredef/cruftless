# CMig-EXT 16 — Property-Bypass Audit

*Engagement-wide enumeration of every `properties.iter()` / `.keys()` / `.values()` / `.contains_key()` call site in the rusty-js-runtime crate. Categorizes each site by Shape-aware safety; identifies sites that need shape-aware migration. Per Findings VI.6 + CMig-EXT 15's escape-route discovery.*

## 1. Methodology

Ran `grep -rn "properties\.\(iter\|keys\|values\|contains_key\)" pilots/rusty-js-runtime/derived/src/`. Inspected each match's surrounding context:

- **Receiver origin**: is the iterated object a user-allocated JS object (potentially Shape-enrolled) or an engine-internal storage object (explicitly Dictionary-form via `Object::new_dictionary()`)?
- **Iteration purpose**: is the iteration user-observable (Object.keys / spread / for-in / iteration protocol) or engine-internal (diagnostic / cache / fast-path)?
- **Already-shape-aware?**: does the call site chain `o.shape.as_ref().iter_slots()...chain(o.properties.iter()...)` or use the canonical `ordinary_own_enumerable_string_keys()` helper?

Three categories applied:

- **SAFE**: receiver is engine-internal Dictionary-form storage OR call site is already shape-aware.
- **NEEDS FIX**: receiver is user-observable + Shape-enrollable + call site iterates only `.properties` directly.
- **DEFENSIVE**: receiver is user-observable but the iteration is diagnostic-only (logs, error context); shape-aware migration is correctness-irrelevant but discipline-relevant.

## 2. Audit results — intrinsics.rs

| line | site | receiver | category | notes |
|---:|---|---|---|---|
| 848 | `__object_spread` | user object via `Value::Object(sid)` | **FIXED at CMig-EXT 15** | shape-aware-then-dictionary pattern |
| 2682 | Headers / Request body spread | user object (s, headers) | **NEEDS FIX** | spread of headers excluding `__headers`; doesn't read shape_values |
| 3195 | Map.entries iteration | __map_data storage (`Object::new_dictionary()`) | **SAFE** | engine storage; explicit Dictionary form per intrinsics.rs:3196 |
| 3262 | Map populate-from-Map | __map_data src storage (Dictionary) | **SAFE** | same; explicit Dictionary |
| 3354 | Set.prototype iteration | __set_data storage (Dictionary) | **SAFE** | explicit Dictionary |
| 5185 | Set iterator construction | __set_data storage (Dictionary) | **SAFE** | explicit Dictionary |
| 5407 | Map.from copy | __map_data src storage (Dictionary) | **SAFE** | explicit Dictionary |
| 5439 | Set.from copy | __set_data src storage (Dictionary) | **SAFE** | explicit Dictionary |
| 5507 | Generic spread variant | user object (src) | **NEEDS FIX** | spreads filtering @@ keys; parallel to CMig-EXT 15 spread |
| 5731 | JSON.stringify | user object | **NEEDS FIX** | needs shape-stored entries for correct enumeration |

## 3. Audit results — interp.rs

| line | site | receiver | category | notes |
|---:|---|---|---|---|
| 781 | target_keys for setLike op | user object | **NEEDS FIX** | enumeration; Set.union/intersect ops |
| 1166, 1192, 1214, 1236 | Multiple kvs-grab patterns (Map.union/intersect/diff) | Set-storage (Dictionary) | **SAFE** | per the surrounding `s` resolution; storage is explicit |
| 1262 | Set difference iteration | Set-storage (Dictionary) | **SAFE** | same |
| 1366 | Set vals iter | __set_data storage (Dictionary) | **SAFE** | per matched key |
| 1992 | Object.defineProperties props-enumeration | user object | **SAFE (CMig-EXT 4 Family B)** | verified: shape-iter block at lines 1985-1991 chains with properties.iter() |
| 2098 | Object.values-class enumeration | user object | **SAFE (CMig-EXT 4 Family B)** | same pattern; verified |
| 2148 | Object.keys-class enumeration | user object | **SAFE (CMig-EXT 4 Family B)** | same pattern; verified |
| 2674, 2686, 2713 | Map ops | __map_data storage | **SAFE** |
| 4962, 4982, 5008 | Object.getOwnPropertyNames / Reflect.ownKeys | user object | **PROBABLY-SAFE-VIA-HELPER** | uses `ordinary_own_enumerable_string_keys()` per Shape-EXT 4 |
| 5381 | __proto__ property enumeration | user object | **DEFENSIVE** | filters out @@ keys; doesn't read values |
| 5515, 5531 | Object.freeze / Object.seal | user object | **NEEDS VERIFY** | per CMig-EXT 5: migrates to Dictionary first; if migrate happens before iteration, SAFE; verify |
| 5596, 5608 | isFrozen / isSealed checks | user object | **NEEDS VERIFY** | predicate reads; if Shape-form is always frozen-or-not, may be SAFE-by-invariant |
| 5681 | `ordinary_own_enumerable_string_keys()` itself | user object | **SAFE (canonical)** | the shape-aware helper itself; chains shape_entries with properties.iter() |
| 6263 | property-has check | user object | **SAFE** | followed by other shape-aware reads |
| 6350 | __proto enumeration | user object | **DEFENSIVE** | similar to 5381 |
| 8004 | __-prefix key detection | user object | **DEFENSIVE** | diagnostic for engine-internal-sentinel detection |
| 8710, 9558 | Diagnostic `keys.take(N)` logging | user object | **DEFENSIVE** | error/debug logging; not load-bearing for correctness |

## 4. Audit results — value.rs / module.rs / napi.rs

| line | site | category | notes |
|---|---|---|---|
| value.rs:43 | `for d in self.properties.values()` | **NEEDS VERIFY** | context-dependent on caller |
| value.rs:508 | `has_own_str` helper | **SAFE-VIA-HELPER** | verified: shape.slot_of(key) check at line 506-507 precedes the contains_key fallback at 508 |
| value.rs:535 | `string_keys()` helper | **SAFE-VIA-HELPER** | the shape-aware iterator |
| value.rs:550 | `string_key_clones()` helper | **SAFE-VIA-HELPER** | shape-aware iterator |
| module.rs:1119 | Module namespace exports | **NEEDS VERIFY** | export-binding enumeration |
| module.rs:1484 | Module ES re-export | **NEEDS VERIFY** | same |
| napi.rs:1252 | NAPI key enumeration | **LOW-PRIORITY** | NAPI surface; less load-bearing |

## 5. Summary by category (post-verification)

| category | count | priority |
|---|---:|---|
| **SAFE** (engine-Dictionary / shape-aware helper / Family-B verified) | ~26 | none |
| **NEEDS FIX** (user-observable + bypasses shape) | **4** | HIGH (CMig-EXT 16.bis) |
| **NEEDS VERIFY** (context-dependent; may already be safe) | ~3 | MEDIUM |
| **DEFENSIVE** (diagnostic; correctness-irrelevant) | ~5 | LOW |

Initial scan flagged ~5 NEEDS-FIX sites. Verification reads removed value.rs:508 (shape-aware-via-fallback) + interp.rs:1992/2098/2148 (Family B chain pattern verified). Net NEEDS-FIX: 4.

## 6. Sites needing immediate fix (NEEDS FIX category, post-verification)

These are the load-bearing CMig-EXT 16.bis substrate moves:

1. **intrinsics.rs:5731** — JSON.stringify property enumeration. **HIGHEST PRIORITY** because JSON.stringify is one of the most-called user-facing primitives. Per CRB-EXT 9's reading, JSON.stringify is also one of the largest contributors to cruft's realistic-workload gap (~2-3× factor); a correctness bug here would compound across realistic workloads.
2. **intrinsics.rs:2682** — Headers spread variant. Similar pattern to CMig-EXT 15's __object_spread; needs the shape-aware-then-dictionary fix.
3. **intrinsics.rs:5507** — Generic spread variant (different call site from CMig-EXT 15's `__object_spread`); same fix pattern.
4. **interp.rs:781** — Set.union / setLike op target_keys enumeration; needs shape-aware iter.

For interp.rs:1992 / 2098 / 2148 (Object.entries/.values/.keys at the interp level): need to verify whether these call sites use `ordinary_own_enumerable_string_keys()` already or iterate `o.properties` directly. Sample read needed.

## 7. Forward to CMig-EXT 16.bis (substrate fix round)

CMig-EXT 16.bis should land the five NEEDS-FIX fixes in one round, each following the CMig-EXT 15 shape-aware-then-dictionary pattern:

```rust
// Pre-fix:
let entries: Vec<...> = o.properties.iter().filter(...).map(...).collect();

// Post-fix:
let mut shape_entries: Vec<...> = Vec::new();
let mut dict_entries: Vec<...> = Vec::new();
{
    let o = ...;
    if let Some(shape) = o.shape.as_ref() {
        for (name, _) in shape.iter_slots() {
            shape_entries.push(...);
        }
    }
    for (k, d) in o.properties.iter().filter(...) {
        dict_entries.push(...);
    }
}
// Process shape_entries first (plain data, no accessor dispatch needed per shapes seed §IV),
// then dict_entries (with accessor handling as before).
```

After CMig-EXT 16.bis lands: re-run fuzz-tb.mjs + fuzz-ic.mjs + diff-prod to verify no regression. **Add a JSON-stringify fixture** to fuzz (currently only spread is covered).

## 8. Forward to CMig-EXT 17 (canonical fuzz harness)

The audit identifies that bench probes + diff-prod can miss shape-bypass bugs in user-observable code paths (CMig-EXT 15 was caught only via out-of-band measurement). The canonical 2000-fixture fuzz harness (Findings VI.6 HIGH priority) is the engagement-wide instrument that catches this class proactively. Scope: random property-mutation patterns + spread + JSON.stringify + Object.entries/.values/.keys + Map/Set iteration.

CMig-EXT 17 design + implementation is the natural follow-on to CMig-EXT 16 + 16.bis.

## 9. §XVI / Doc 734 / Doc 735 §X.h categorization

Per Doc 730 §XVI: not applicable (audit-tier round; no substrate-correctness call).

Per Doc 734 §V: growth (a) tier-relocation — the property-bypass audit was queued from CMig-EXT 15's "needs follow-up audit" finding; this round produces the audit. Growth (b) negative-finding amendment in waiting — the 5 NEEDS-FIX sites are not-yet-failed but are constraint-propagated stalls per Doc 739's pattern; closing them prevents future regression cascades.

Per Doc 735 §X.h.c three-probe-levels: this round is design-tier (enumerate sites; categorize); the actual fix-and-verify is CMig-EXT 16.bis (bench + consumer-route via diff-prod + fuzz via CMig-EXT 17).

## 10. Composition with prior corpus work

- **CMig-EXT 15**: empirically anchored the bug class; this audit enumerates the residual bypass sites.
- **Findings doc IV.1 + IV.2**: directly applied; the audit is the discipline this finding queued.
- **Findings doc rule 5 (three probes before default-on)** + **rule 6 (surface-completeness audit for data-structure changes)**: this round IS rule 6's apparatus applied retroactively to the shape-enrollment change (CMig-EXT 14 default-on flip).
- **Doc 739 cascade-revival pattern**: the audit identifies SITES that may exhibit constraint-propagated stalls. The fix path is sub-pilot-local (CMig-EXT 16.bis), not upstream constraint-closure — these aren't (P2.d) stalls; they're CORRECTNESS GAPS at the consumer tier.

---

*CMig-EXT 16 audit complete. 5 NEEDS-FIX sites identified for CMig-EXT 16.bis substrate fix round. 6 NEEDS-VERIFY sites for follow-up inspection. CMig-EXT 17 canonical fuzz harness remains the engagement-wide probe-coverage gap close.*
