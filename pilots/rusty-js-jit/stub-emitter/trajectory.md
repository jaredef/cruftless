# LeJIT-Σ — Trajectory

Per-StubE-EXT log for the LeJIT-Σ stub-emitter pilot. Sub-workstream of `pilots/rusty-js-jit/`. Reads seed.md first; this file records the hand-rolled aarch64 IC stub emitter's substrate moves.

Format: one section per StubE-EXT. Same shape as other Pin-Art trajectory.md files.

---

## StubE-EXT 0 — 2026-05-23 (workstream founding)

### Headline

Apparatus-tier round. Pilot LeJIT-Σ founded per Doc 737 §IV + the keeper's standing "set up seeds at every fractal locale that requires it" directive. The LeJIT seed §I.2 (JIT-EXT 25) pre-filed this coordinate; the spawn is now explicit because the pilot has multi-rung shape (StubE-EXT 0-8 covers founding → bench-baseline → design → scaffold → synthetic-pointer test → translator wiring → bench measurement → fuzz → default-on flip).

### Substrate delivered

- `pilots/rusty-js-jit/stub-emitter/seed.md` (~155 lines) — telos, apparatus (composes with parent LeJIT crate + sibling shapes pilot at the `Object::shape_ptr_and_slot_for` API boundary), methodology with StubE-EXT 0-8 staging, carve-outs (aarch64 only, monomorphic only, GetProp only, shape-cache only), composition with shapes pilot's CMig-EXT 8 enrollment-flip gate, falsifiers Pred-stub.1-.5.
- `pilots/rusty-js-jit/stub-emitter/trajectory.md` (this file).
- `pilots/rusty-js-jit/stub-emitter/docs/` scaffold for StubE-EXT 1 (bench-baseline.md) + StubE-EXT 2 (stub-design.md) outputs.

### Locale registration

Per Doc 737 §IV: nested locale at coordinate `pilots/rusty-js-jit/stub-emitter/` (depth 2). Parent reference: `L.rusty-js-jit` (LeJIT). Sibling cross-reference: `L.rusty-js-shapes` and its nested `L.rusty-js-shapes/consumer-migration`. The pilot composes with the shapes substrate at the `Object::shape_ptr_and_slot_for` API.

This is the engagement's **second prospective-spawn case** (the first was consumer-migration earlier today). Both spawned because their multi-rung shape was known at spawn time per the keeper's standing directive sharpening Doc 737 §VII.

### §XVI / Doc 734 categorization

Per Doc 730 §XVI: not applicable (no probe gated; founding-round documentation only).

Per Doc 734 §V: growth mechanism (a) tier-relocation recursion — the LeJIT-Σ pilot was pre-filed at JIT-EXT 25 as a future coordinate; the spawn now makes it an active locale per Doc 737 §IV. Growth mechanism (b) negative-finding amendment (latent) — the shapes pilot's CMig-EXT 8 enrollment-flip gate is the empirical event that will validate or falsify the Pred-stub.1 3× speedup threshold; below threshold, the pilot's structural claim weakens to (P2.d) and the work is reverted with the boundary documented.

### Composition with prior corpus work

- **Doc 729 §A8.13 substrate-amortization**: LeJIT-Σ is the closure round consuming the shapes pilot's substrate-introduction round. Staging: substrate-first (shapes Shape-EXT 0-4 + consumer-migration CMig-EXT 0-8), closure-second (this pilot StubE-EXT 0-8). EXTs 0-3 here can land in advance of CMig-EXT 8; EXTs 4-8 gate on it.
- **Doc 731 §VII R1**: single-tier baseline JIT shape preserves under the hybrid Cranelift + hand-rolled stub emitter. The stub emitter is a sub-substrate of the same JIT tier; not a second tier. The corpus claim under test (per parent LeJIT seed §I.2) is that this hybrid composition achieves IC fast-path latency competitive with mainstream JITs while preserving R1.
- **Doc 735 §X.h three-probe-levels discipline**: the Pred-stub.1 (≥3× speedup) claim requires bench + consumer-route + fuzz probes per §X.h.c. Each probe activates at a different StubE-EXT (bench at EXT 1 + 6; consumer-route at EXT 5 via diff-prod; fuzz at EXT 7).
- **Doc 737 §IV locale-as-coordinate**: this is the engagement's second prospective-spawn case. The coordinate uniqueness is filesystem-structural; the parent reference is explicit at seed §VII.
- **Doc 738 §II source-tier conventions**: stub-emitter identifiers will fit the five-axis space. `__ic_*` prefix for stub-internal sentinels per §II.a; snake_case methods per §II.b; pillar-path `pilots/rusty-js-jit/derived/src/stub_aarch64.rs` per §II.e.

### Open scope at StubE-EXT 0 close

1. **StubE-EXT 1** — Pre-stub bench probe. Establish baseline measurement for the current extern-call IC dispatch on a 1M-iteration property-access loop on the Pi. Output: `docs/bench-baseline.md`. Test file: `pilots/rusty-js-jit/derived/tests/bench_ic.rs`. Apparatus + small-code round.
2. **StubE-EXT 2** — Stub emitter design. Output: `docs/stub-design.md`. Apparatus-tier; no code.
3. **StubE-EXT 3** — Stub emitter crate scaffold. `pilots/rusty-js-jit/derived/src/stub_aarch64.rs` module + tests. Test-only; not wired into the translator.
4. **StubE-EXTs 4-8** — Per the seed §III methodology.

### Cumulative status at StubE-EXT 0 close

LOC delta: 0 (apparatus-tier round). docs/ scaffold: 1 (empty dir). Locale registered.

The pilot's locale exists; the substrate work begins at StubE-EXT 1.

---

*StubE-EXT 0 closes. The pilot is founded; the next round is the bench baseline measurement.*
