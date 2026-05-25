# non-ctor-registration-discipline — Trajectory

## NACR-EXT 0+1 — founding + closure (2026-05-25)

**Trigger**: matrix probe of "not-a-constructor" cluster (16 tests) revealed two parallel `register_method` helpers in `regexp.rs` and `promise.rs` that defaulted to `is_constructor=true`, contradicting the canonical `intrinsics.rs::register_method` which uses `make_native_non_ctor`.

**Edits** (~6 LOC):
- `regexp.rs::register_method`: switch to `make_native_non_ctor`.
- `promise.rs::register_method`: flip inline `is_constructor: true` → `false`.

**Verification**:
- Probes (RegExp.test / String.replace / RegExp.exec / Promise.prototype.then): all isConstructor=false (were partially true)
- Exemplar (16 in-cluster, 5 in-scope): PASS 0 → 5
- Regression on 200 random RegExp/Promise/String previously-passing: 200/200 preserved

### Findings

**Finding NACR.1**: substrate-discipline coherence check. Three `register_method` helpers existed (intrinsics.rs, regexp.rs, promise.rs); only one was correct. The discipline-defining doc comment lived only in intrinsics.rs's version. The drift between copies surfaced via the not-a-constructor test cluster; per Finding T262C.6 the corpus-as-regression-instrument shape applies — multiple test failures with the same reason-shape across distinct modules pointed at a substrate-discipline gap, not per-module per-bug.

**Finding NACR.2 (carve-out documentation)**: 11/16 remaining not-a-constructor tests are for missing methods (newer Stage proposals: Math.f16round, JSON.rawJSON, Map.prototype.getOrInsert*, Promise.allKeyed/allSettledKeyed). Adding those is per-method stub work — separate sub-locales per proposal.

**Status**: CLOSED at NACR-EXT 1.
