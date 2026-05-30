#!/usr/bin/env node
// apparatus/scripts/caacp-codex-multi-bridge.mjs
//
// Multi-runner Codex Desktop / app-server bridge. Successor to the single-
// runner caacp-codex-app-bridge.mjs; manages N {role, instance_id, thread_id}
// runners in one process so the keeper does not have to track a process per
// agent. Each runner gets its own seen-cache + active-directive ledger,
// matching the single-bridge semantics exactly (including stop-continue
// re-injection per the watcher 2026-05-29 design).
//
// On startup the process POSTs /local/bridge-announce to the sidecar so the
// helmsman's GET /local/bridges surfaces the managed runner set. On graceful
// shutdown (SIGTERM/SIGINT) it POSTs /local/bridge-shutdown.
//
// Operator-started only. The bridge does not discover threads on its own;
// the operator supplies the config file.

import crypto from "node:crypto";
import fs from "node:fs/promises";
import net from "node:net";
import os from "node:os";
import path from "node:path";

const REPO_ROOT = path.resolve(import.meta.dirname, "../..");
const DATA_DIR = path.join(REPO_ROOT, "apparatus/caacp-server/data");

function usage() {
  console.error(`Usage: caacp-codex-multi-bridge.mjs <config-file>

The config file is a JSON document of the shape:

  {
    "codex_app_server_ws": "ws://...",        // optional, falls back to env
    "codex_app_token_file": "/path/to/token", // optional, falls back to env
    "default_poll_interval_seconds": 5,        // optional, default 5
    "runners": [
      { "role": "substrate-resolver", "instance_id": "codex-r1-...", "thread_id": "019e..." },
      { "role": "substrate-resolver", "instance_id": "codex-r2-...", "thread_id": "019e..." },
      { "role": "helmsman",           "instance_id": null,           "thread_id": "019e..." }
    ]
  }

Environment (sidecar):
  CAACP_SIDECAR_HOST      default 127.0.0.1
  CAACP_SIDECAR_PORT      default 7777

Environment (codex fallback when config omits the field):
  CODEX_APP_SERVER_WS     default ws://100.101.214.109:41337
  CODEX_APP_TOKEN_FILE    default ~/.codex/remote-control/ios-token
`);
  process.exit(1);
}

const [configPath] = process.argv.slice(2);
if (!configPath) usage();

const config = JSON.parse(await fs.readFile(configPath, "utf8"));
if (!Array.isArray(config.runners) || config.runners.length === 0) {
  console.error("[caacp-codex-multi-bridge] FATAL: config.runners must be a non-empty array");
  process.exit(2);
}

const SIDECAR_HOST = process.env.CAACP_SIDECAR_HOST ?? "127.0.0.1";
const SIDECAR_PORT = process.env.CAACP_SIDECAR_PORT ?? "7777";
const CODEX_APP_SERVER_WS = config.codex_app_server_ws ?? process.env.CODEX_APP_SERVER_WS ?? "ws://100.101.214.109:41337";
const CODEX_APP_TOKEN_FILE =
  config.codex_app_token_file ??
  process.env.CODEX_APP_TOKEN_FILE ??
  path.join(process.env.HOME ?? "", ".codex/remote-control/ios-token");
const DEFAULT_INTERVAL_MS = Math.max(1, Number(config.default_poll_interval_seconds ?? 5)) * 1000;

const CONTINUE_AFTER_MS = 60 * 1000;
const CONTINUE_INTERVAL_MS = 120 * 1000;
const CONTINUE_MAX_ATTEMPTS = 3;
const STOP_STATUSES = new Set(["idle", "notLoaded"]);

const HOST = os.hostname();
const PID = process.pid;
const STARTED_AT = new Date().toISOString();
const LOG_FILE = path.join(DATA_DIR, `multi-bridge-${HOST}-${PID}.log`);

async function log(message) {
  const line = `[caacp-codex-multi-bridge] ${new Date().toISOString()} ${message}\n`;
  process.stderr.write(line);
  await fs.appendFile(LOG_FILE, line, "utf8").catch(() => {});
}

function runnerBridgeId(runner) {
  return runner.instance_id ? `${runner.role}-${runner.instance_id}` : runner.role;
}

function runnerSeenFile(runner) {
  return path.join(DATA_DIR, `bridge-${runnerBridgeId(runner)}-codex-app-seen.json`);
}

function runnerActiveFile(runner) {
  return path.join(DATA_DIR, `bridge-${runnerBridgeId(runner)}-codex-app-active.json`);
}

async function readJsonArray(file) {
  try {
    const parsed = JSON.parse(await fs.readFile(file, "utf8"));
    return Array.isArray(parsed) ? parsed : [];
  } catch {
    return [];
  }
}

async function readJsonObject(file) {
  try {
    const parsed = JSON.parse(await fs.readFile(file, "utf8"));
    return parsed && typeof parsed === "object" ? parsed : {};
  } catch {
    return {};
  }
}

async function fetchInbox(runner) {
  let url = `http://${SIDECAR_HOST}:${SIDECAR_PORT}/local/inbox?role=${encodeURIComponent(runner.role)}`;
  if (runner.instance_id) url += `&instance_id=${encodeURIComponent(runner.instance_id)}`;
  const resp = await fetch(url);
  if (!resp.ok) throw new Error(`sidecar inbox returned HTTP ${resp.status} for ${runner.role}`);
  return await resp.json();
}

function parseWsUrl(raw) {
  const url = new URL(raw);
  if (url.protocol !== "ws:") throw new Error(`only ws:// app-server URLs are supported; got ${raw}`);
  return { host: url.hostname, port: Number(url.port || "80"), path: `${url.pathname || "/"}${url.search || ""}`, authority: url.host };
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
  for (let i = 0; i < payload.length; i += 1) frame[header.length + 4 + i] = payload[i] ^ mask[i % 4];
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
    if (bigLen > BigInt(Number.MAX_SAFE_INTEGER)) throw new Error("websocket frame too large");
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
  if (mask) for (let i = 0; i < payload.length; i += 1) payload[i] ^= mask[i % 4];
  return { opcode, text: payload.toString("utf8"), rest: buf.subarray(off + len) };
}

async function codexRequest(method, params) {
  const token = (await fs.readFile(CODEX_APP_TOKEN_FILE, "utf8")).trim();
  const ws = parseWsUrl(CODEX_APP_SERVER_WS);
  const key = crypto.randomBytes(16).toString("base64");
  const initializeRequest = { id: 1, method: "initialize", params: { clientInfo: { name: "caacp-codex-multi-bridge", version: "0" }, capabilities: {} } };
  const requestId = 2;
  const request = { id: requestId, method, params };

  return await new Promise((resolve, reject) => {
    const socket = net.connect(ws.port, ws.host);
    let handshake = "";
    let established = false;
    let initialized = false;
    let wsBuffer = Buffer.alloc(0);
    const timer = setTimeout(() => { socket.destroy(); reject(new Error(`timeout waiting for app-server response to ${method}`)); }, 15000);
    function cleanup() { clearTimeout(timer); socket.destroy(); }

    socket.on("connect", () => {
      socket.write([
        `GET ${ws.path} HTTP/1.1`, `Host: ${ws.authority}`, "Upgrade: websocket", "Connection: Upgrade",
        `Sec-WebSocket-Key: ${key}`, "Sec-WebSocket-Version: 13", `Authorization: Bearer ${token}`, "\r\n",
      ].join("\r\n"));
    });

    socket.on("data", (chunk) => {
      try {
        if (!established) {
          handshake += chunk.toString("binary");
          const idx = handshake.indexOf("\r\n\r\n");
          if (idx < 0) return;
          if (!handshake.startsWith("HTTP/1.1 101")) { cleanup(); reject(new Error(`websocket handshake failed: ${handshake.slice(0, idx)}`)); return; }
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
          if (decoded.opcode === 8) { cleanup(); reject(new Error("app-server closed websocket before response")); return; }
          if (decoded.opcode !== 1) continue;
          const msg = JSON.parse(decoded.text);
          if (!initialized && msg.id === initializeRequest.id) {
            if (msg.error) { cleanup(); reject(new Error(JSON.stringify(msg.error))); return; }
            initialized = true;
            socket.write(encodeClientFrame(JSON.stringify({ method: "initialized" })));
            socket.write(encodeClientFrame(JSON.stringify(request)));
            continue;
          }
          if (msg.id !== requestId) continue;
          cleanup();
          if (msg.error) reject(new Error(JSON.stringify(msg.error))); else resolve(msg.result);
          return;
        }
      } catch (err) { cleanup(); reject(err); }
    });
    socket.on("error", (err) => { cleanup(); reject(err); });
  });
}

async function wakeCodex(threadId, directive) {
  await codexRequest("thread/resume", { threadId, cwd: REPO_ROOT });
  return await codexRequest("turn/start", { threadId, input: [{ type: "text", text: directive }], cwd: REPO_ROOT });
}

async function codexThreadStatus(threadId) {
  try {
    const result = await codexRequest("thread/read", { threadId });
    if (result && typeof result === "object") return (result.thread && result.thread.status) || result.status || null;
    return null;
  } catch (err) {
    await log(`WARN: thread/read failed for ${threadId}: ${err instanceof Error ? err.message : String(err)}`);
    return null;
  }
}

async function cycleRunner(runner) {
  const SEEN_FILE = runnerSeenFile(runner);
  const ACTIVE_FILE = runnerActiveFile(runner);

  const inbox = await fetchInbox(runner);
  const messages = (inbox.messages ?? []).filter((msg) => msg.state === "PENDING");
  const pendingIds = new Set(messages.map((m) => m.message_id));

  const seen = await readJsonArray(SEEN_FILE);
  const seenSet = new Set(seen);
  const fresh = messages.filter((msg) => !seenSet.has(msg.message_id));

  const active = await readJsonObject(ACTIVE_FILE);
  let activeChanged = false;
  for (const id of Object.keys(active)) {
    if (!pendingIds.has(id)) { delete active[id]; activeChanged = true; }
  }

  if (fresh.length > 0) {
    const latest = fresh[fresh.length - 1];
    const latestTag = `${latest.sender}/${latest.intent}/${latest.slug}`;
    const instance = runner.instance_id ? ` instance_id=${runner.instance_id}` : "";
    const directive = `**CAACP NEW** role=${runner.role}${instance} count=${messages.length} latest=${latestTag}. Check sidecar inbox before continuing.`;
    await wakeCodex(runner.thread_id, directive);
    const capped = [...new Set([...seen, ...fresh.map((m) => m.message_id)])].slice(-1000);
    await fs.writeFile(SEEN_FILE, `${JSON.stringify(capped, null, 2)}\n`, "utf8");
    const nowIso = new Date().toISOString();
    for (const msg of fresh) {
      active[msg.message_id] = { injected_at: nowIso, last_continue_at: null, continue_attempts: 0, directive_tag: `${msg.sender}/${msg.intent}/${msg.slug}` };
    }
    activeChanged = true;
    await log(`woke role=${runner.role}${instance} thread=${runner.thread_id} new_ids=${fresh.map((m) => m.message_id).join(",")}`);
  }

  const tracked = Object.entries(active).filter(([id]) => pendingIds.has(id));
  if (tracked.length > 0) {
    const status = await codexThreadStatus(runner.thread_id);
    if (status === "systemError") {
      await log(`ALERT role=${runner.role} thread=${runner.thread_id} status=systemError; skipping stop-continue`);
    } else if (status && STOP_STATUSES.has(status)) {
      const now = Date.now();
      for (const [id, entry] of tracked) {
        if (entry.continue_attempts >= CONTINUE_MAX_ATTEMPTS) continue;
        const injectedAt = Date.parse(entry.injected_at) || now;
        const lastContinueAt = entry.last_continue_at ? (Date.parse(entry.last_continue_at) || 0) : 0;
        if (now - injectedAt < CONTINUE_AFTER_MS) continue;
        if (lastContinueAt !== 0 && now - lastContinueAt < CONTINUE_INTERVAL_MS) continue;
        const instance = runner.instance_id ? ` instance_id=${runner.instance_id}` : "";
        const continueDirective = `**CAACP CONTINUE** role=${runner.role}${instance} target_directive_id=${id} attempt=${entry.continue_attempts + 1}/${CONTINUE_MAX_ATTEMPTS} reason=stop-before-telos. Resume directive per §V.4 same-turn imperative; the original directive (${entry.directive_tag}) is still PENDING in your inbox.`;
        try {
          await wakeCodex(runner.thread_id, continueDirective);
          entry.continue_attempts += 1;
          entry.last_continue_at = new Date().toISOString();
          activeChanged = true;
          await log(`CONTINUE role=${runner.role} thread=${runner.thread_id} target=${id} attempt=${entry.continue_attempts}/${CONTINUE_MAX_ATTEMPTS}`);
        } catch (err) {
          await log(`WARN: CONTINUE injection failed for ${runner.role} target=${id}: ${err instanceof Error ? err.message : String(err)}`);
        }
      }
    }
  }

  if (activeChanged) await fs.writeFile(ACTIVE_FILE, `${JSON.stringify(active, null, 2)}\n`, "utf8");
}

async function announceBridge() {
  const resp = await fetch(`http://${SIDECAR_HOST}:${SIDECAR_PORT}/local/bridge-announce`, {
    method: "POST", headers: { "Content-Type": "application/json" },
    body: JSON.stringify({ host: HOST, pid: PID, started_at: STARTED_AT, runners: config.runners }),
  });
  if (!resp.ok) await log(`WARN: bridge-announce failed (${resp.status}); continuing`);
  else await log(`announced to sidecar bridge_id=${HOST}-${PID} runners=${config.runners.length}`);
}

async function shutdownBridge() {
  try {
    await fetch(`http://${SIDECAR_HOST}:${SIDECAR_PORT}/local/bridge-shutdown`, {
      method: "POST", headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ host: HOST, pid: PID }),
    });
    await log(`shutdown notice sent to sidecar`);
  } catch (err) {
    await log(`WARN: shutdown notice failed: ${err instanceof Error ? err.message : String(err)}`);
  }
}

let shuttingDown = false;
for (const sig of ["SIGTERM", "SIGINT"]) {
  process.on(sig, async () => {
    if (shuttingDown) return;
    shuttingDown = true;
    await log(`received ${sig}; shutting down`);
    await shutdownBridge();
    process.exit(0);
  });
}

await fs.mkdir(DATA_DIR, { recursive: true });
for (const runner of config.runners) {
  const seen = runnerSeenFile(runner);
  const active = runnerActiveFile(runner);
  try { await fs.access(seen); } catch { await fs.writeFile(seen, "[]\n", "utf8"); }
  try { await fs.access(active); } catch { await fs.writeFile(active, "{}\n", "utf8"); }
}
await announceBridge();
await log(`starting multi-bridge pid=${PID} runners=${config.runners.length} interval=${DEFAULT_INTERVAL_MS / 1000}s`);

while (true) {
  for (const runner of config.runners) {
    try {
      await cycleRunner(runner);
    } catch (err) {
      await log(`WARN runner=${runner.role}${runner.instance_id ? `/${runner.instance_id}` : ""}: ${err instanceof Error ? err.message : String(err)}`);
    }
  }
  await new Promise((resolve) => setTimeout(resolve, DEFAULT_INTERVAL_MS));
}
