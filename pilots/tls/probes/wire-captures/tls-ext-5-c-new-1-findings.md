# TLS-EXT 5 — C-NEW-1 (Record-Content Logging) Findings

**Tag**: `Ω.5.P06.E2.tls-c-new-1-hang-discovery` (TLS-EXT 5, partial — C-NEW-1 only)
**Date**: 2026-05-21
**Companion**: [bidirectional-pin-art-probe-design.md](../bidirectional-pin-art-probe-design.md); [tls-ext-4-pin-art-readout.md](./tls-ext-4-pin-art-readout.md).

## §1. Probe executed

C-NEW-1 from the TLS-EXT 4 plan: record-content logging in `receive_application_data` + step-by-step logging in `pm_http_get`. Gated behind `CRUFTLESS_TLS_DEBUG=1` env var. Plus a new `just_example.rs` that takes a URL as argument so single-endpoint isolation is possible.

## §2. Unexpected finding

The instrumented run surfaced a **different failure mode** than the earlier TLS-EXT 3 probe captured:

| endpoint | TLS-EXT 3 probe | TLS-EXT 5 C-NEW-1 probe |
|---|---|---|
| httpbin.org | CloseNotify mid-handshake | CloseNotify mid-handshake (reproducible) |
| api.github.com | Codec("Truncated") | **HANG (CPU-bound, indefinite)** |
| google.com | Codec("Truncated") | **HANG (CPU-bound)** |
| example.com | Codec("Truncated") | **HANG (CPU-bound)** |
| example.org | (not probed earlier) | **HANG (CPU-bound)** |

The hang is in `tls_connect` itself: the debug log shows `[pm_http_get] connecting → host:443` and then nothing more. `ps -p $PID -o stat` shows `R` (running), not `S` (sleeping on I/O). The process is in a CPU-bound loop, not waiting on the socket.

## §3. Interpretation

This is structurally significant. The TLS-EXT 3 reading of "Codec(Truncated) = TCP EOF before any response data" assumed `tls_connect` returned successfully. The hang finding falsifies that assumption: `tls_connect` does not return at all for these endpoints. The earlier Codec(Truncated) outcomes may have been:

- A different code path in `tls_probe.rs` that times out the hang and returns a different error
- Network/server variability across runs (CDN backends differ)
- Or — more concerning — the TLS-EXT 2 close_notify changes silently introduced a regression that creates the hang under specific server behaviors

The hang means the **TLS-EXT 3 hypothesis space (H1 transcript-hash drift / H2 cipher / H3 ServerHello extension) is partly invalidated.** Those hypotheses presupposed that handshake completes; they explain post-handshake decryption divergence. A handshake that doesn't complete at all points to a different cluster of bugs:

- **H5 — handshake state machine deadlock.** Our handshake loop is waiting for a message that the server sent but our decode rejected silently, then loops. Or our loop processes a server message in a way that doesn't advance state.
- **H6 — infinite read loop with progress.** Our loop reads bytes, decodes a record, the record is something we don't handle (e.g., a TLS 1.3 HelloRetryRequest), we silently `continue`, and the same bytes are re-read or we re-decode the same record.
- **H7 — TLS-EXT 2 regression.** The close_notify routing changes may have introduced a path where a warning alert during handshake triggers a non-terminating branch. Bisect required.

## §4. Local-server control still works

Against `openssl s_server -tls1_3 -www` on localhost (self-signed cert), the same instrumented binary completes the full exchange:
```
[pm_http_get] start https://localhost:4443/
[pm_http_get] connecting → localhost:4443
[pm_http_get] handshake OK
[pm_http_get] sending NN request bytes
[pm_http_get] send OK; entering drain loop
[tls-c-new-1] record: ct=ApplicationData frag_len=... seq=0
[tls-c-new-1]   decrypt OK: inner_ct=22 pt_len=...
[tls-c-new-1]   post-handshake handshake_type=0x04 (NewSessionTicket)
[...continued through full response and CloseNotify...]
```

The asymmetry holds: local works, CDN hangs.

## §5. Redirected hypothesis space (TLS-EXT 6 candidates)

H5/H6/H7 push the substrate work into the handshake state machine rather than the post-handshake data path. Three concrete next moves:

**C-NEW-4 — handshake-tier debug print.** Add `CRUFTLESS_TLS_DEBUG` gating to the handshake loop in `tls_connect`: print every record type seen, every handshake message extracted, every state transition. Re-run against api.github.com. The last printed event before the hang names the substrate site to fix.

**C-NEW-5 — bisect TLS-EXT 2.** Revert just the close_notify routing changes from TLS-EXT 2 in a scratch branch; re-run the 5-endpoint probe. If the hang disappears, H7 is confirmed. If it persists, H7 is falsified and H5/H6 take precedence.

**C-NEW-6 — strace to confirm CPU-bound loop.** Verify under strace that the process is in a tight read-decode-continue cycle (no syscalls) rather than spinning on syscalls. Distinguishes H5 (pure-CPU state-machine deadlock) from a network-syscall livelock.

## §6. Closes (partial TLS-EXT 5)

C-NEW-1 closes with the unexpected hang finding. C-NEW-2 (transcript-hash + app-key dump) and C-NEW-3 (ServerHello extension audit) are deferred to TLS-EXT 7+ — they presuppose handshake completion, which the current state does not always provide. The handshake-tier hang surfaced as a higher-priority work item.

The Pin-Art apparatus continues to function: C-NEW-1 was designed to surface H1/H2/H3 evidence; instead it surfaced H5/H6/H7 by exposing a precondition (handshake completion) that the earlier probes had silently assumed. The probe's reach is what surfaced the assumption. This is the §XVI bidirectional engine-diff oracle operating: the diff between expectation and observation pins the next move.
