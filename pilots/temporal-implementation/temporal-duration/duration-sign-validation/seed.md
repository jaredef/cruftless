---
name: duration-sign-validation
description: Fifth sub-rung of temporal-duration. Cross-cutting validation that enforces Duration's uniform-sign invariant at ctor + from + with.
type: project
---

# duration-sign-validation — Seed

## Leaf sub-locale under `pilots/temporal-implementation/temporal-duration/`.

Per Finding DWith.2: when a validation rule is cross-cutting across multiple sibling rungs, it warrants its own rung even though it's not a method. This is the first such cross-cutting rung in the Temporal program.

## Telos

Per ECMA-262 §11.4.2.1 ToTemporalDuration step "validate uniform sign": all non-zero unit fields of a Temporal.Duration MUST share the same sign (either all positive or all negative). Mixed signs → RangeError.

Validation fires at every site that constructs a Duration from user input:
- `Temporal.Duration` constructor (DCF).
- `Temporal.Duration.from` property-bag path (DStat).
- `Temporal.Duration.prototype.with` post-merge (DWith).

(The Duration-instance clone path inherits its source's validation; no re-check needed.)

## Apparatus

- `pilots/rusty-js-runtime/derived/src/intrinsics.rs::install_temporal` — new `validate_uniform_sign(&[f64; 10])` inline helper + 3 call sites.
- **Exemplar suite**: none (cross-cutting validation; measured via sibling-rung yield deltas).

## Methodology

### DSV-EXT 1 — uniform-sign helper + 3 call sites (LANDED)

```rust
fn validate_uniform_sign(units: &[f64; 10]) -> Result<(), RuntimeError> {
    let mut sign: f64 = 0.0;
    for &u in units {
        if u == 0.0 { continue; }
        let s = u.signum();
        if sign == 0.0 { sign = s; }
        else if sign != s {
            return Err(RuntimeError::RangeError(
                "Temporal.Duration: all non-zero unit fields must share sign".into()
            ));
        }
    }
    Ok(())
}
```

Called after units are assembled at each of the 3 construction sites.

## R13 prospective C1-C4

- C1 (sibling): HOLDS — sign-validation is a one-pass scan analogous to integer-validation already in ctor.
- C2 (shape-compat): HOLDS — additive single-call insertion at 3 sites.
- C3 (cost-positive): HOLDS — ~25 LOC closes residuals across 3 sibling rungs.
- C4 (bail-safe): HOLDS — only fires on illegal mixed-sign user input.

## Status

DSV-EXT 1 LANDED 2026-05-26. +3 sibling yield (DStat 22→23, DWith 17→19, DCF unchanged at 64/67). Smaller than the +7 prediction in Finding DWith.2 — most sibling residuals weren't pure sign-validation issues.
