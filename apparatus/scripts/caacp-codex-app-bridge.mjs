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
const positional = args.filter((arg) => arg !== "--once");
if (positional.length < 2) usage();

const ROLE = positional[0];
const THREAD_ID = positional[1];
const INTERVAL_MS = Math.max(1, Number(positional[2] ?? "5")) * 1000;
const SIDECAR_HOST = process.env.CAACP_SIDECAR_HOST ?? "127.0.0.1";
const SIDECAR_PORT = process.env.CAACP_SIDECAR_PORT ?? "7777";
const CODEX_APP_SERVER_WS = process.env.CODEX_APP_SERVER_WS ?? "ws://100.101.214.109:41337";
const CODEX_APP_TOKEN_FILE =
  process.env.CODEX_APP_TOKEN_FILE ??
  path.join(process.env.HOME ?? "", ".codex/remote-control/ios-token");

const SEEN_FILE = path.join(DATA_DIR, `bridge-${ROLE}-codex-app-seen.json`);
const LOG_FILE = path.join(DATA_DIR, `bridge-${ROLE}-codex-app.log`);

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

async function fetchInbox() {
  const url = `http://${SIDECAR_HOST}:${SIDECAR_PORT}/local/inbox?role=${encodeURIComponent(ROLE)}`;
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

async function cycle() {
  const inbox = await fetchInbox();
  const messages = (inbox.messages ?? []).filter((msg) => msg.state === "PENDING");
  if (messages.length === 0) return;

  const seen = await readJsonArray(SEEN_FILE);
  const seenSet = new Set(seen);
  const fresh = messages.filter((msg) => !seenSet.has(msg.message_id));
  if (fresh.length === 0) return;

  const latest = fresh[fresh.length - 1];
  const latestTag = `${latest.sender}/${latest.intent}/${latest.slug}`;
  const directive = `**CAACP NEW** role=${ROLE} count=${messages.length} latest=${latestTag}. Check sidecar inbox before continuing.`;

  await wakeCodex(directive);
  await writeSeen([...seen, ...fresh.map((msg) => msg.message_id)]);
  await log(`woke thread=${THREAD_ID} directive="${directive}" new_ids=${fresh.map((msg) => msg.message_id).join(",")}`);
}

await fs.mkdir(DATA_DIR, { recursive: true });
try {
  await fs.access(SEEN_FILE);
} catch {
  await fs.writeFile(SEEN_FILE, "[]\n", "utf8");
}
await log(`starting; thread=${THREAD_ID} interval=${INTERVAL_MS / 1000}s seen-cache=${SEEN_FILE}`);

while (true) {
  try {
    await cycle();
  } catch (err) {
    await log(`WARN: ${err instanceof Error ? err.message : String(err)}`);
  }
  if (once) break;
  await new Promise((resolve) => setTimeout(resolve, INTERVAL_MS));
}
