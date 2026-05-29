#!/usr/bin/env node
// apparatus/scripts/caacp-codex-app-bridge.mjs
//
// Cybernetic bridge for Codex Desktop / app-server sessions. Polls the
// local CAACP sidecar for a role's PENDING inbox and wakes a specific
// Codex thread by submitting a short user turn through the app-server
// websocket (`turn/start`). This is the Desktop/iOS equivalent of the
// tmux bridge; use it when the target agent session is controlled by
// Codex Desktop rather than an interactive terminal pane.
//
// Operator-started only. The bridge deliberately does not discover or
// mutate thread targets on its own; the operator supplies the thread id.

import crypto from "node:crypto";
import fs from "node:fs/promises";
import net from "node:net";
import path from "node:path";

const REPO_ROOT = path.resolve(import.meta.dirname, "../..");
const DATA_DIR = path.join(REPO_ROOT, "apparatus/caacp-server/data");

function usage() {
  console.error(`Usage: caacp-codex-app-bridge.mjs <role> <thread-id> [poll-interval-seconds] [--once]

Arguments:
  role                    substrate-resolver | helmsman | arbiter | watcher | deputy
  thread-id               Codex app-server thread id to wake
  poll-interval           seconds between polls; default 5
  --once                  process one poll cycle and exit
  --instance-id <id>      optional CAACP instance_id for per-instance roles

Environment:
  CAACP_SIDECAR_HOST      default 127.0.0.1
  CAACP_SIDECAR_PORT      default 7777
  CODEX_APP_SERVER_WS     default ws://100.101.214.109:41337
  CODEX_APP_TOKEN_FILE    default ~/.codex/remote-control/ios-token
`);
  process.exit(1);
}

const args = process.argv.slice(2);
const once = args.includes("--once");
let instanceId = null;
const positional = [];
for (let i = 0; i < args.length; i += 1) {
  const arg = args[i];
  if (arg === "--once") continue;
  if (arg === "--instance-id") {
    instanceId = args[++i] ?? null;
    continue;
  }
  if (arg.startsWith("--instance-id=")) {
    instanceId = arg.slice("--instance-id=".length);
    continue;
  }
  positional.push(arg);
}
if (positional.length < 2) usage();

const ROLE = positional[0];
const THREAD_ID = positional[1];
const INSTANCE_ID = instanceId;
const INTERVAL_MS = Math.max(1, Number(positional[2] ?? "5")) * 1000;
const SIDECAR_HOST = process.env.CAACP_SIDECAR_HOST ?? "127.0.0.1";
const SIDECAR_PORT = process.env.CAACP_SIDECAR_PORT ?? "7777";
const CODEX_APP_SERVER_WS = process.env.CODEX_APP_SERVER_WS ?? "ws://100.101.214.109:41337";
const CODEX_APP_TOKEN_FILE =
  process.env.CODEX_APP_TOKEN_FILE ??
  path.join(process.env.HOME ?? "", ".codex/remote-control/ios-token");

const BRIDGE_ID = INSTANCE_ID ? `${ROLE}-${INSTANCE_ID}` : ROLE;
const SEEN_FILE = path.join(DATA_DIR, `bridge-${BRIDGE_ID}-codex-app-seen.json`);
const LOG_FILE = path.join(DATA_DIR, `bridge-${BRIDGE_ID}-codex-app.log`);
const ACTIVE_FILE = path.join(DATA_DIR, `bridge-${BRIDGE_ID}-codex-app-active.json`);

// Codex stop-continue wake primitive (per watcher 2026-05-29 design
// `apparatus/docs/codex-stop-continue-bridge-design.md` + keeper Telegram
// 10446/10449). The bridge tracks injected directives in an active-
// directive ledger; on each poll, if a tracked directive is still PENDING
// in the role/instance inbox AND the Codex thread status indicates the
// session has stopped before reaching quiescence, the bridge re-injects
// a **CAACP CONTINUE** turn to wake the session back into the loop.
//
// Throttles prevent runaway re-injection: a continue is eligible after
// CONTINUE_AFTER_MS since the original directive injection (or last
// continue attempt), interval CONTINUE_INTERVAL_MS between continues,
// max CONTINUE_MAX_ATTEMPTS per directive.
const CONTINUE_AFTER_MS = 60 * 1000;
const CONTINUE_INTERVAL_MS = 120 * 1000;
const CONTINUE_MAX_ATTEMPTS = 3;
// Status values observed across this codex app-server build:
// `active`, `idle`, `notLoaded`, `systemError`. The first two states
// where the session has stopped consuming turns are the targets for
// re-injection; `active` means turns are running, `systemError` is a
// terminal failure we log+skip instead of looping into.
const STOP_STATUSES = new Set(["idle", "notLoaded"]);

async function log(message) {
  const line = `[caacp-codex-app-bridge] ${new Date().toISOString()} role=${ROLE} ${message}\n`;
  process.stderr.write(line);
  await fs.appendFile(LOG_FILE, line, "utf8").catch(() => {});
}

async function readJsonArray(file) {
  try {
    const parsed = JSON.parse(await fs.readFile(file, "utf8"));
    return Array.isArray(parsed) ? parsed : [];
  } catch {
    return [];
  }
}

async function writeSeen(ids) {
  const capped = [...new Set(ids)].slice(-1000);
  await fs.writeFile(SEEN_FILE, `${JSON.stringify(capped, null, 2)}\n`, "utf8");
}

async function readActive() {
  try {
    const parsed = JSON.parse(await fs.readFile(ACTIVE_FILE, "utf8"));
    return parsed && typeof parsed === "object" ? parsed : {};
  } catch {
    return {};
  }
}

async function writeActive(ledger) {
  await fs.writeFile(ACTIVE_FILE, `${JSON.stringify(ledger, null, 2)}\n`, "utf8");
}

async function fetchInbox() {
  let url = `http://${SIDECAR_HOST}:${SIDECAR_PORT}/local/inbox?role=${encodeURIComponent(ROLE)}`;
  if (INSTANCE_ID) url += `&instance_id=${encodeURIComponent(INSTANCE_ID)}`;
  const resp = await fetch(url);
  if (!resp.ok) {
    throw new Error(`sidecar inbox returned HTTP ${resp.status}`);
  }
  return await resp.json();
}

function parseWsUrl(raw) {
  const url = new URL(raw);
  if (url.protocol !== "ws:") {
    throw new Error(`only ws:// app-server URLs are supported by this bridge; got ${raw}`);
  }
  return {
    host: url.hostname,
    port: Number(url.port || "80"),
    path: `${url.pathname || "/"}${url.search || ""}`,
    authority: url.host,
  };
}

function encodeClientFrame(text) {
  const payload = Buffer.from(text);
  const mask = crypto.randomBytes(4);
  const len = payload.length;
  let header;
  if (len < 126) {
    header = Buffer.alloc(2);
    header[1] = 0x80 | len;
  } else if (len < 65536) {
    header = Buffer.alloc(4);
    header[1] = 0x80 | 126;
    header.writeUInt16BE(len, 2);
  } else {
    throw new Error("websocket payload too large");
  }
  header[0] = 0x81;
  const frame = Buffer.concat([header, mask, payload]);
  for (let i = 0; i < payload.length; i += 1) {
    frame[header.length + 4 + i] = payload[i] ^ mask[i % 4];
  }
  return frame;
}

function tryDecodeServerFrame(buf) {
  if (buf.length < 2) return null;
  const opcode = buf[0] & 0x0f;
  let len = buf[1] & 0x7f;
  let off = 2;
  if (len === 126) {
    if (buf.length < 4) return null;
    len = buf.readUInt16BE(2);
    off = 4;
  } else if (len === 127) {
    if (buf.length < 10) return null;
    const bigLen = buf.readBigUInt64BE(2);
    if (bigLen > BigInt(Number.MAX_SAFE_INTEGER)) {
      throw new Error("websocket frame too large");
    }
    len = Number(bigLen);
    off = 10;
  }
  const masked = Boolean(buf[1] & 0x80);
  let mask;
  if (masked) {
    if (buf.length < off + 4) return null;
    mask = buf.subarray(off, off + 4);
    off += 4;
  }
  if (buf.length < off + len) return null;
  const payload = Buffer.from(buf.subarray(off, off + len));
  if (mask) {
    for (let i = 0; i < payload.length; i += 1) payload[i] ^= mask[i % 4];
  }
  return {
    opcode,
    text: payload.toString("utf8"),
    rest: buf.subarray(off + len),
  };
}

async function codexRequest(method, params) {
  const token = (await fs.readFile(CODEX_APP_TOKEN_FILE, "utf8")).trim();
  const ws = parseWsUrl(CODEX_APP_SERVER_WS);
  const key = crypto.randomBytes(16).toString("base64");
  const initializeRequest = {
    id: 1,
    method: "initialize",
    params: {
      clientInfo: { name: "caacp-codex-app-bridge", version: "0" },
      capabilities: {},
    },
  };
  const requestId = 2;
  const request = { id: requestId, method, params };

  return await new Promise((resolve, reject) => {
    const socket = net.connect(ws.port, ws.host);
    let handshake = "";
    let established = false;
    let initialized = false;
    let wsBuffer = Buffer.alloc(0);
    const timer = setTimeout(() => {
      socket.destroy();
      reject(new Error(`timeout waiting for app-server response to ${method}`));
    }, 15000);

    function cleanup() {
      clearTimeout(timer);
      socket.destroy();
    }

    socket.on("connect", () => {
      socket.write(
        [
          `GET ${ws.path} HTTP/1.1`,
          `Host: ${ws.authority}`,
          "Upgrade: websocket",
          "Connection: Upgrade",
          `Sec-WebSocket-Key: ${key}`,
          "Sec-WebSocket-Version: 13",
          `Authorization: Bearer ${token}`,
          "\r\n",
        ].join("\r\n"),
      );
    });

    socket.on("data", (chunk) => {
      try {
        if (!established) {
          handshake += chunk.toString("binary");
          const idx = handshake.indexOf("\r\n\r\n");
          if (idx < 0) return;
          if (!handshake.startsWith("HTTP/1.1 101")) {
            cleanup();
            reject(new Error(`websocket handshake failed: ${handshake.slice(0, idx)}`));
            return;
          }
          established = true;
          const rest = Buffer.from(handshake.slice(idx + 4), "binary");
          if (rest.length > 0) wsBuffer = Buffer.concat([wsBuffer, rest]);
          socket.write(encodeClientFrame(JSON.stringify(initializeRequest)));
          return;
        }

        wsBuffer = Buffer.concat([wsBuffer, chunk]);
        while (wsBuffer.length > 0) {
          const decoded = tryDecodeServerFrame(wsBuffer);
          if (!decoded) return;
          wsBuffer = decoded.rest;
          if (decoded.opcode === 8) {
            cleanup();
            reject(new Error("app-server closed websocket before response"));
            return;
          }
          if (decoded.opcode !== 1) continue;
          const msg = JSON.parse(decoded.text);
          if (!initialized && msg.id === initializeRequest.id) {
            if (msg.error) {
              cleanup();
              reject(new Error(JSON.stringify(msg.error)));
              return;
            }
            initialized = true;
            socket.write(encodeClientFrame(JSON.stringify({ method: "initialized" })));
            socket.write(encodeClientFrame(JSON.stringify(request)));
            continue;
          }
          if (msg.id !== requestId) continue;
          cleanup();
          if (msg.error) reject(new Error(JSON.stringify(msg.error)));
          else resolve(msg.result);
          return;
        }
      } catch (err) {
        cleanup();
        reject(err);
      }
    });

    socket.on("error", (err) => {
      cleanup();
      reject(err);
    });
  });
}

async function wakeCodex(directive) {
  await codexRequest("thread/resume", {
    threadId: THREAD_ID,
    cwd: REPO_ROOT,
  });
  return await codexRequest("turn/start", {
    threadId: THREAD_ID,
    input: [{ type: "text", text: directive }],
    cwd: REPO_ROOT,
  });
}

async function codexThreadStatus() {
  // Per watcher 2026-05-29 investigation: thread/read returns the thread
  // record including status (one of: active, idle, notLoaded, systemError).
  // Used by the stop-continue mechanism to decide whether to re-inject.
  try {
    const result = await codexRequest("thread/read", { threadId: THREAD_ID });
    if (result && typeof result === "object") {
      const status =
        (result.thread && result.thread.status) ||
        result.status ||
        null;
      return status;
    }
    return null;
  } catch (err) {
    await log(`WARN: thread/read failed: ${err instanceof Error ? err.message : String(err)}`);
    return null;
  }
}

async function cycle() {
  const inbox = await fetchInbox();
  const messages = (inbox.messages ?? []).filter((msg) => msg.state === "PENDING");
  const pendingIds = new Set(messages.map((m) => m.message_id));

  const seen = await readJsonArray(SEEN_FILE);
  const seenSet = new Set(seen);
  const fresh = messages.filter((msg) => !seenSet.has(msg.message_id));

  // Active-directive ledger maintenance: retire any tracked directives
  // that are no longer PENDING (resolver has acked or remote endpoint
  // transitioned them). v1 retirement check is inbox-absence; v2 follow-
  // up will use sidecar GET /local/messages/<id> for exact remote state
  // (per watcher's design recommendation).
  const active = await readActive();
  let activeChanged = false;
  for (const id of Object.keys(active)) {
    if (!pendingIds.has(id)) {
      delete active[id];
      activeChanged = true;
    }
  }

  if (fresh.length > 0) {
    const latest = fresh[fresh.length - 1];
    const latestTag = `${latest.sender}/${latest.intent}/${latest.slug}`;
    const instance = INSTANCE_ID ? ` instance_id=${INSTANCE_ID}` : "";
    const directive = `**CAACP NEW** role=${ROLE}${instance} count=${messages.length} latest=${latestTag}. Check sidecar inbox before continuing.`;

    await wakeCodex(directive);
    await writeSeen([...seen, ...fresh.map((msg) => msg.message_id)]);

    // Record each fresh injected directive in the active ledger so the
    // stop-continue check can re-inject if the session stops before the
    // directive is RESOLVED.
    const nowIso = new Date().toISOString();
    for (const msg of fresh) {
      active[msg.message_id] = {
        injected_at: nowIso,
        last_continue_at: null,
        continue_attempts: 0,
        directive_tag: `${msg.sender}/${msg.intent}/${msg.slug}`,
      };
    }
    activeChanged = true;

    await log(`woke thread=${THREAD_ID} directive="${directive}" new_ids=${fresh.map((msg) => msg.message_id).join(",")}`);
  }

  // Stop-continue check: if any tracked directive is still PENDING and
  // the Codex thread has stopped (idle/notLoaded) past the throttle
  // window, re-inject a CONTINUE turn. Only runs when there's at least
  // one tracked-pending directive — saves a thread/read call per cycle
  // when nothing is in flight.
  const tracked = Object.entries(active).filter(([id]) => pendingIds.has(id));
  if (tracked.length > 0) {
    const status = await codexThreadStatus();
    if (status === "systemError") {
      await log(`ALERT: thread status=systemError; skipping stop-continue check (operator review required)`);
    } else if (status && STOP_STATUSES.has(status)) {
      const now = Date.now();
      for (const [id, entry] of tracked) {
        if (entry.continue_attempts >= CONTINUE_MAX_ATTEMPTS) continue;
        const injectedAt = Date.parse(entry.injected_at) || now;
        const lastContinueAt = entry.last_continue_at
          ? Date.parse(entry.last_continue_at) || 0
          : 0;
        const sinceInject = now - injectedAt;
        const sinceContinue = now - lastContinueAt;
        const eligibleByInject = sinceInject >= CONTINUE_AFTER_MS;
        const eligibleByInterval =
          lastContinueAt === 0 || sinceContinue >= CONTINUE_INTERVAL_MS;
        if (!eligibleByInject || !eligibleByInterval) continue;

        const instance = INSTANCE_ID ? ` instance_id=${INSTANCE_ID}` : "";
        const continueDirective = `**CAACP CONTINUE** role=${ROLE}${instance} target_directive_id=${id} attempt=${entry.continue_attempts + 1}/${CONTINUE_MAX_ATTEMPTS} reason=stop-before-telos. Resume directive per §V.4 same-turn imperative; the original directive (${entry.directive_tag}) is still PENDING in your inbox.`;
        try {
          await wakeCodex(continueDirective);
          entry.continue_attempts += 1;
          entry.last_continue_at = new Date().toISOString();
          activeChanged = true;
          await log(`CONTINUE injected thread=${THREAD_ID} target=${id} attempt=${entry.continue_attempts}/${CONTINUE_MAX_ATTEMPTS} status=${status}`);
        } catch (err) {
          await log(`WARN: CONTINUE injection failed for ${id}: ${err instanceof Error ? err.message : String(err)}`);
        }
      }
    }
  }

  if (activeChanged) {
    await writeActive(active);
  }
}

await fs.mkdir(DATA_DIR, { recursive: true });
try {
  await fs.access(SEEN_FILE);
} catch {
  await fs.writeFile(SEEN_FILE, "[]\n", "utf8");
}
try {
  await fs.access(ACTIVE_FILE);
} catch {
  await fs.writeFile(ACTIVE_FILE, "{}\n", "utf8");
}
await log(`starting; thread=${THREAD_ID} instance_id=${INSTANCE_ID ?? ""} interval=${INTERVAL_MS / 1000}s seen-cache=${SEEN_FILE} active-cache=${ACTIVE_FILE} stop-continue=enabled(after=${CONTINUE_AFTER_MS / 1000}s,interval=${CONTINUE_INTERVAL_MS / 1000}s,max=${CONTINUE_MAX_ATTEMPTS})`);

while (true) {
  try {
    await cycle();
  } catch (err) {
    await log(`WARN: ${err instanceof Error ? err.message : String(err)}`);
  }
  if (once) break;
  await new Promise((resolve) => setTimeout(resolve, INTERVAL_MS));
}
