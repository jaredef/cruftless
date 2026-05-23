# StubE-EXT 1 — Pre-stub IC bench baseline

*Establishes the baseline measurement of the current extern-call IC dispatch path for `Op::GetPropOnObject`. The stub-emitter pilot's Pred-stub.1 target (≥3× per-hit speedup over this baseline per LeJIT seed §I.2 + Doc 735 §X.h.b) reads against this number.*

## 1. Bench protocol

**Workload**: 1,000,000 iterations of `getx(obj)` where `function getx(obj) { return obj.x; }` and `obj = { x: 42 }`. The receiver is a single Object instance reused across all iterations (monomorphic IC site shape).

**Bytecode**: `Op::LoadLocal 0` (load arg-0 / receiver) → `Op::GetPropOnObject 0` (constant idx 0 = "x") → `Op::Return`. Hand-built FunctionProto bypasses the upstream parser (which doesn't yet emit `Op::GetPropOnObject` per JIT-EXT 24 open scope item 2).

**Dispatch path**:
```
Op::GetPropOnObject
  → JIT-emitted Cranelift `call jit_getprop_on_object`
    → extern "C" fn jit_getprop_on_object (in rusty-js-jit/deopt.rs)
      → runtime_getprop_on_object (in rusty-js-runtime, reads TLS Runtime + Proto)
        → rt.object_get(obj_id, "x")
          → shape-fast-path (None pre-CMig-EXT 8) → falls through to properties IndexMap probe
        → encodes f64::to_bits as i64 sentinel
      → returns to JIT-emitted code
    → JIT returns Value::Number(42.0)
```

**Warmup**: 10 calls before measurement to amortize JIT compile (`jit_threshold = 1`).

**Source**: `cruftless/examples/bench_ic.rs`. Run via `cargo build --release --example bench_ic -p cruftless && target/release/examples/bench_ic`.

## 2. Pi baseline measurement (2026-05-23)

```
elapsed:     270.986 ms
per-iter:    271.0 ns
```

Hardware: Raspberry Pi (the engagement's reference target). Single-run; not yet variance-characterized — StubE-EXT 6's post-stub measurement should re-measure with at least 5 runs and report mean ± stdev to be robust under variance.

## 3. Cost breakdown (estimated, ground truth pending profiling)

The 271 ns/iter decomposes approximately as:

| component | est. ns | source |
|---|---:|---|
| `call_function` Rust dispatcher (closure + arg copy + Frame setup) | ~120 | invariant; not in scope for LeJIT-Σ to optimize |
| JIT-emitted preamble + arg coercion (Number/Object unbox) | ~30 | Cranelift-generated; partly amenable to value-tag inline (sibling pilot per LeJIT seed §I.2 item 4) |
| Cranelift `call` to `jit_getprop_on_object` (extern boundary, TLS read) | ~50 | **the target of LeJIT-Σ stub emission** |
| `runtime_getprop_on_object` body (object_get probe through properties) | ~50 | **partly amenable to shape-fast-path** post CMig-EXT 8 (slot index vs IndexMap probe) |
| JIT return + reboxing | ~20 | Cranelift-generated |

The ~50 ns "Cranelift call to extern" component is what LeJIT-Σ targets directly: replace the indirect `call` with an inlined 2-3-instruction shape-check + slot-load. Plus the ~50 ns object_get body collapses to a direct slot load post-CMig-EXT 8. Net per-iter cost should drop to ~170-180 ns after both substrate moves, which is well past the 3× threshold of ≤90.3 ns/iter from Pred-stub.1.

The cost breakdown above is estimated, not measured. StubE-EXT 6's profiling should validate or refine the decomposition.

## 4. Pred-stub.1 falsifier reading

Per LeJIT seed §I.2 + Doc 735 §X.h.b:

- **(P2.a) strict win**: stub-emitted dispatch achieves ≤90.3 ns/iter (≥3× speedup).
- **(P2.d) correct-but-losing**: stub-emitted dispatch is correct but > 90.3 ns/iter. Revert; document boundary; cost-stratum decision Cranelift-extern-call is the right pick for this engagement.
- **(P2.c) illegal-speed**: stub-emitted dispatch is < 90.3 ns/iter but fuzz probe (StubE-EXT 7) finds correctness violations (e.g., cache stale after shape transition). Cautionary tale per Doc 735 §X.h.b WC-EXT 21 precedent — bench-fixture passing alone is necessary-but-not-sufficient evidence.
- **(P2.b) slow-stratum implementation**: a hand-rolled stub that's algorithmically right but composed from primitives at a worse cost-stratum (e.g., a generic memory fence where a narrower aarch64 `dmb ish` suffices). Refine the implementation per X.h.b precedent.

## 5. Comparison points (informational)

- Bun (V8 TurboFan + Sparkplug + Maglev) IC fast-path: low-single-digit ns/iter, per general V8 IC literature. cruftless's first-cut JIT runs at 1.5× of Bun on numeric loops (per LeJIT seed §VIII bench data); for property-access the gap is wider currently because Bun's IC fast-path inlines what cruftless routes through extern call.
- Pre-JIT interpreter: not measured in this round (the bench harness calls the JIT-compiled path; the interpreter path for this workload would be a separate harness with `jit_threshold = u32::MAX`). Expected ~3-5× slower than the JIT path per the seed §VIII bench precedent.

## 6. Forward to StubE-EXT 2

StubE-EXT 2's design round chooses:
- **Cache layout**: inline literal in JIT-emitted code (one shape pointer + one slot offset patched per IC site) vs side-table indexed by call-site id (one global Vec, IC site index baked into JIT code).
- **Patching mechanism**: aarch64 instruction-cache flush sequence (`dc cvau` + `ic ivau` + `dsb ish` per ARMv8.0; or `__builtin___clear_cache` if calling out to libgcc).
- **State machine**: cold (no cache; first hit fills) → warm-monomorphic (one cached shape) → cold-after-miss (patch with new shape, return to warm-monomorphic) → degraded (after N misses, abandon stub and call extern path forever).

Each design choice has a measurable per-iter cost implication; the StubE-EXT 2 doc should estimate each choice's contribution against the 271 ns baseline.

---

*StubE-EXT 1 closes. The baseline is 271 ns/iter on the Pi for the current extern-call dispatch. StubE-EXT 2's design round chooses the stub's cache + patching shape; StubE-EXTs 3-5 implement; StubE-EXT 6 re-measures against this baseline.*
