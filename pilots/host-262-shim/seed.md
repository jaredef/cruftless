# host-262-shim - Seed

**Locale tag**: `L.host-262-shim` (top-level; EPSUA reopened sub-locale #3).

**Status**: FOUNDED at H262S-EXT 0. Phase-2 baseline probe complete; substrate landing not yet authorized.

**Workstream**: test262 host-hook availability for `$262`-dependent tests. The current runner prepends the upstream harness, but Cruft does not expose the `$262` host object that several test262 subclusters use for detach-buffer probes, IsHTMLDDA emulation, realm construction, global-state helpers, and host GC/agent hooks.

**Trigger**: EPSUA parallel-4 adjudication alpha (keeper directive Telegram 10317 via helmsman CAACP `epsua-parallel-4-adjudication-alpha-approved`). R1 is authorized for locale spawn plus read-only baseline partition only. Target is the cheap additive shim path; `createRealm` remains separate-track unless keeper directs.

**Composes with**:
- [EPSUA](../../apparatus/arcs/2026-05-25-ecmascript-parity-shared-upstream/arc.md) - parent arc.
- [T262C](../apparatus/test262-categorize/seed.md) - matrix source for runner-harness projections.
- ECMA-262 host hooks as exercised by test262's `$262` harness object.
- EPSUA-EXT 0.5 - prior projection drift finding for `$262.createRealm`.

## I. Telos

Expose the minimal `$262` host object needed by cheap, additive test262 host-hook subclusters while avoiding realm-substrate work in this rung. The likely cheap subclusters are `$262.detachArrayBuffer` and `$262.IsHTMLDDA` / emulates-undefined; `createRealm` is explicitly out of scope for EXT 1 unless keeper reclassifies it.

## II. Apparatus + Methodology

R is not identified for substrate yet. H262S-EXT 0 performs spawn plus Phase-2 baseline inspection only:

1. Partition current `runner-harness/$262-or-host-hook` failures in `test262-full-2026-05-28-123833-p2/interpreted.jsonl`.
2. Identify whether a cheap shim subcluster is real and sized.
3. Propose an EXT 1 landing plan only after helmsman ack.

Candidate substrate surface after approval:

1. `cruftless/src/lib.rs` host install path, after intrinsics and before `install_global_this_refresh`.
2. A new host module such as `cruftless/src/test262_host.rs` that installs `globalThis.$262`.
3. Possibly no `pilots/rusty-js-runtime/derived/src/*` changes if existing host APIs can allocate object/function properties.

## III. Carve-outs

- `$262.createRealm` / cross-realm constructor identity: out of scope. EPSUA-EXT 0.5 already found this is realm substrate, not a cheap host shim.
- Agent / Atomics coordination hooks: out of scope unless a cheap no-op hook is empirically shown to be sufficient for a bounded subcluster.
- GC stress hooks: out of scope except for explicit no-op compatibility if a single probe confirms it is harmless.
- Test262 runner feature policy and skip policy are out of scope; this locale only covers host object availability.

## IV. Baseline Partition

Input: `pilots/apparatus/test262-categorize/full-suite/results/test262-full-2026-05-28-123833-p2/interpreted.jsonl`.

Rows selected by `projection == "runner-harness/$262-or-host-hook"` or reason containing `$262 is not defined`: 361.

Subclusters:

| Subcluster | Rows | Reading |
|---|---:|---|
| `detachArrayBuffer` | 229 | Cheap-shim candidate if runtime already supports buffer-detached state transitions. Requires exemplar confirmation. |
| `isHTMLDDA-emulates-undefined` | 34 | Cheap-shim candidate if a special host object can emulate Annex B undefined-like behavior closely enough for test262. |
| `global-hook-or-global-state` | 24 | Probably mixed; some may require global declaration semantics, not only `$262` availability. |
| `createRealm-realm` | 15 | Not cheap. Defer per EPSUA-EXT 0.5. |
| `gc` | 1 | Likely no-op-compatible, but too small to drive the first substrate. |
| `other-$262` | 58 | Heterogeneous tail; needs exemplar inspection before inclusion. |

## V. Verification Plan

Before substrate EXT 1, run focused exemplar probes from the top two cheap subclusters:

- `built-ins/ArrayBuffer/prototype/byteLength/detached-buffer.js`
- `built-ins/DataView/detached-buffer.js`
- `annexB/built-ins/Object/is/emulates-undefined.js`
- `annexB/built-ins/String/prototype/search/custom-searcher-emulates-undefined.js`

Regression check after any approved substrate landing: adjacent previously-passing ArrayBuffer/DataView/TypedArray harness tests plus Annex B emulates-undefined rows that do not depend on the new hook.

## VI. Status

H262S-EXT 0 founded the locale and completed the read-only baseline partition. Awaiting helmsman ack on the follow-up EXT 1 landing plan.
