# rusty-js-caps Probes — Synthetic-Adversary Harness

**Purpose**: the §XVI oracle for Pilot α. Each probe is a small `.mjs` file that simulates one attack class from Doc 736 §IV. The probe attempts the attack; the harness inspects exit code and stdout to determine whether the attack succeeded.

## States

Each probe has two reportable outcomes, written to stdout as a sentinel line:

- `PROBE:WINS:<class>:<evidence>` — the attack succeeded (capability not refused)
- `PROBE:LOSES:<class>:<reason>` — the attack was refused (CapabilityError thrown)

Under **Mode 0** (current default at CAPS-EXT 4 close), every probe WINS — Cruftless has no capability enforcement wired yet. The Mode-0 wins are the engagement's documented pre-state.

Under **Mode 3 + empty caps**, every probe will LOSE once CAPS-EXT 6+ wires route-through. The CAPS-EXT 13 closure gate is: every probe in this directory LOSES under `--sealed`.

## Probe naming

`<surface>_<verb>.mjs` — `fs_read.mjs`, `process_exit.mjs`, etc. Each probe is self-contained: it reads no input from environment, performs one attack with a hardcoded sentinel target, and reports the outcome on stdout.

## Why no node_modules layout

The probes live at the application tier in this round. Mode 0 / Mode 3 enforcement is global, so application-vs-dependency provenance does not affect outcome. **CAPS-EXT 8 (Mode 2 wiring)** will introduce a parallel probe layout where each probe is repackaged as a dep under `node_modules/probe-<name>/` and required by a small application stub. The Mode 2 round verifies the provenance branch in the dispatcher.

## Runner

`host-v2/tests/caps_probes.rs` invokes the cruftless binary on each probe and asserts the expected outcome per mode. Tests are network-free; safe to run locally.
