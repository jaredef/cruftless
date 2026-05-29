# ODP-EXT 3 R4 Landing

## Directive

`helmsman/request/odp-ext-3-arguments-mapped-prototype-shadow-r4`

## Disposition

Partial substrate closure landed for mapped arguments exotic defineProperty semantics and prototype non-writable assignment shadowing.

## Evidence

```text
cargo build --release --bin cruft -p cruftless: PASS

targeted residuals:
PASS 4-289-1, 4-292-1, 4-293-2, 4-293-3, 4-294-1, 4-295-1, 4-296-1, 4-301-1, 4-410, 4-415
FAIL 4-625gs
NO OUTPUT 4-116

descriptor-shape/property-semantics bucket: 41 PASS / 1 FAIL / 1 no-output
Object.defineProperty surface bucket: 52 PASS / 1 FAIL / 1 no-output
Adjacent first-80 Object.defineProperty sample: 80 PASS / 0 FAIL
```

## Residuals

- `4-625gs`: focused probe has `this.prop === 1002` and `prop === 1002`, but `this.hasOwnProperty("prop")` is false. Residual coordinate is script-global own-property reflection.
- `4-116`: `timeout 10s` exits status 124 with empty stdout/stderr. Residual coordinate is a no-output hang trace.
