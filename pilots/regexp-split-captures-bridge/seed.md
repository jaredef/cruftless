# regexp-split-captures-bridge — Seed

## Telos

Per ECMA-262 §22.2.5.13 RegExp.prototype[@@split], when a regex separator has capture groups, the captured strings (or `undefined` for non-participating) must be **interleaved** between the split chunks. Currently cruft's `split` delegates to `CompiledRegex::split_str` which discards captures.

Identified by RES audit-2 (Gap A): `"a1b2c3".split(/(\d)/)` returns `["a","b","c",""]` instead of `["a","1","b","2","c","3",""]`.

## Apparatus

- `pilots/rusty-js-runtime/derived/src/regexp.rs::String.prototype.split` (line 942, the regex branch at line 965).
- `pilots/rusty-js-runtime/derived/src/value.rs::CompiledRegex` (already has `captures_at` and `captures_positions_at`).

## Methodology

Replace the regex branch's `rx.split_str(&s)` call with a custom loop:
1. cursor = 0; loop until end of input.
2. captures_at(s, cursor) → if None, push remaining tail; break.
3. If match starts at cursor && match is empty: spec advances cursor by 1; don't push.
4. Else: push `s[cursor..mstart]`. For each capture (skip [0]), push the string or undefined. cursor = mend.
5. Push tail.
6. Honor limit truncation (existing mechanism).

## Carve-outs

- Spec edge: zero-width match handling (`split("ab", /(?:)/)` etc.) — keep existing semantics for now if they diverge; targeted in a follow-up.
- Non-regex split (plain-string sep): unchanged.

## Resume protocol

Read `trajectory.md` tail.
