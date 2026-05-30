// apparatus/caacp-server/server.ts
//
// CAACP local sidecar — runs on each cruftless dev/agent machine. Single
// shared process across all local resolver-instances per keeper directive
// (Telegram 10257). Each agent registers its role/instance with the sidecar
// at session entry; the sidecar registers with jaredfoy.com via the admin
// CAACP_TOKEN_VERIFIER, stores the returned per-agent token locally, and
// runs a polling loop per registered token to surface incoming messages.
//
// Three notification channels per registered agent:
//   α — notification file at data/inbound-<role>[-<instance>].json,
//        watchable via Claude Code's Monitor tool or any file-watch.
//   β — agent's registered callback URL, POSTed on new-message arrival
//        (cross-tool agnostic; works for Codex / shell-driven setups).
//   γ — Telegram fallback if neither channel acknowledges within window
//        (deferred; not implemented in v1).
//
// HTTP surface (localhost only):
//   POST /local/register {role, instance_id?, callback_url?}
//     → {token, role, instance_id, sidecar_port, notification_file}
//   POST /local/send {sender, sender_token, recipient, intent, slug, body, related_to?, target_instance_id?}
//     → {message_id, state, server_timestamp}
//   POST /local/ack {original_message_id, ack_author_token, ack_state, body}
//     → {ack_id, message_id, state, server_timestamp}
//   GET  /local/inbox?role=<role>&instance_id=<instance>
//     → forwarded GET /api/caacp/v1/inbox/<role> via per-agent token
//   GET  /local/outbox?role=<role>&instance_id=<instance>
//     → forwarded GET /api/caacp/v1/outbox/<role> via per-agent token
//   GET  /local/health → {status, registered_agents, last_poll_at}
//
// Configuration via env:
//   CAACP_SIDECAR_PORT      default 7777
//   CAACP_SIDECAR_HOST      default 127.0.0.1
//   CAACP_TOKEN_VERIFIER    admin token for registering agents with jaredfoy.com
//   CAACP_ENDPOINT          default https://jaredfoy.com/api/caacp/v1
//   CAACP_POLL_INTERVAL_MS  default 5000

import path from "node:path";
import { promises as fs } from "node:fs";

const PORT = parseInt(process.env.CAACP_SIDECAR_PORT ?? "7777", 10);
const HOST = process.env.CAACP_SIDECAR_HOST ?? "127.0.0.1";
const ENDPOINT = process.env.CAACP_ENDPOINT ?? "https://jaredfoy.com/api/caacp/v1";
const ADMIN_TOKEN = process.env.CAACP_TOKEN_VERIFIER ?? "";
const POLL_INTERVAL_MS = parseInt(process.env.CAACP_POLL_INTERVAL_MS ?? "5000", 10);

const DATA_DIR = path.resolve(import.meta.dir, "./data");
const REGISTRY_FILE = path.join(DATA_DIR, "agent-registry.json");

if (!ADMIN_TOKEN) {
  console.error("[caacp-sidecar] FATAL: CAACP_TOKEN_VERIFIER unset. Cannot register agents with jaredfoy.com.");
  process.exit(2);
}

type AgentRecord = {
  token: string;
  role: string;
  instance_id: string | null;
  callback_url: string | null;
  registered_at: string;
  last_polled_at: string | null;
  last_seen_message_ids: string[];
};

type Registry = Record<string, AgentRecord>; // key: token

async function ensureDataDir() {
  await fs.mkdir(DATA_DIR, { recursive: true });
}

async function loadRegistry(): Promise<Registry> {
  try {
    const raw = await fs.readFile(REGISTRY_FILE, "utf8");
    return JSON.parse(raw);
  } catch {
    return {};
  }
}

async function saveRegistry(registry: Registry): Promise<void> {
  await fs.writeFile(REGISTRY_FILE, JSON.stringify(registry, null, 2), "utf8");
}

function notificationFileFor(agent: AgentRecord): string {
  const suffix = agent.instance_id ? `${agent.role}-${agent.instance_id}` : agent.role;
  return path.join(DATA_DIR, `inbound-${suffix}.json`);
}

const registry: Registry = await (async () => {
  await ensureDataDir();
  return loadRegistry();
})();

function logInfo(...args: unknown[]) {
  console.log("[caacp-sidecar]", new Date().toISOString(), ...args);
}

function logWarn(...args: unknown[]) {
  console.warn("[caacp-sidecar]", new Date().toISOString(), "WARN", ...args);
}

async function callRemote(method: string, path: string, body: unknown, token: string): Promise<Response> {
  return fetch(`${ENDPOINT}${path}`, {
    method,
    headers: {
      "X-CAACP-Token": token,
      "Content-Type": "application/json",
    },
    body: body !== undefined ? JSON.stringify(body) : undefined,
  });
}

// ── HTTP handlers ──────────────────────────────────────────────────────────

function json(status: number, data: unknown): Response {
  return new Response(JSON.stringify(data), {
    status,
    headers: { "Content-Type": "application/json" },
  });
}

async function handleRegister(req: Request): Promise<Response> {
  const body = await req.json().catch(() => null);
  if (!body) return json(400, { error: "invalid JSON body" });
  const { role, instance_id, callback_url } = body as any;
  if (!role) return json(400, { error: "missing role" });

  // Forward registration to jaredfoy.com using the admin token.
  const remoteResp = await callRemote("POST", "/register", { role, instance_id, callback_url }, ADMIN_TOKEN);
  if (!remoteResp.ok) {
    const errBody = await remoteResp.text();
    return json(remoteResp.status, { error: "remote registration failed", remote: errBody });
  }
  const remoteData = (await remoteResp.json()) as any;
  const agent: AgentRecord = {
    token: remoteData.token,
    role,
    instance_id: instance_id ?? null,
    callback_url: callback_url ?? null,
    registered_at: remoteData.registered_at,
    last_polled_at: null,
    last_seen_message_ids: [],
  };
  registry[agent.token] = agent;
  await saveRegistry(registry);
  logInfo("registered agent", { role: agent.role, instance_id: agent.instance_id });

  // Initialize empty notification file so watchers see it.
  await fs.writeFile(notificationFileFor(agent), JSON.stringify({ messages: [] }, null, 2), "utf8");

  return json(201, {
    token: agent.token,
    role: agent.role,
    instance_id: agent.instance_id,
    sidecar_host: HOST,
    sidecar_port: PORT,
    notification_file: notificationFileFor(agent),
  });
}

function findAgentByToken(token: string): AgentRecord | undefined {
  return registry[token];
}

async function handleSend(req: Request): Promise<Response> {
  const body = await req.json().catch(() => null);
  if (!body) return json(400, { error: "invalid JSON body" });
  const { sender, sender_token, recipient, intent, slug, body: msgBody, related_to, target_instance_id } = body as any;
  if (!sender_token) return json(400, { error: "missing sender_token" });
  if (target_instance_id !== undefined && target_instance_id !== null && typeof target_instance_id !== "string") {
    return json(400, { error: "target_instance_id must be a string or null" });
  }
  const agent = findAgentByToken(sender_token);
  if (!agent) return json(403, { error: "unknown sender_token; agent not registered" });
  if (sender && sender !== agent.role) return json(403, { error: `sender (${sender}) does not match token role (${agent.role})` });

  // Compute content_sha
  const enc = new TextEncoder().encode(msgBody ?? "");
  const hashBuf = await crypto.subtle.digest("SHA-256", enc);
  const content_sha = Array.from(new Uint8Array(hashBuf)).map(b => b.toString(16).padStart(2, "0")).join("");

  const remoteResp = await callRemote("POST", "/messages", {
    sender: agent.role,
    recipient,
    intent,
    slug,
    related_to: related_to ?? null,
    target_instance_id: target_instance_id ?? null,
    content_sha,
    body: msgBody ?? null,
  }, agent.token);
  if (!remoteResp.ok) {
    const errBody = await remoteResp.text();
    return json(remoteResp.status, { error: "remote send failed", remote: errBody });
  }
  return json(201, await remoteResp.json());
}

async function handleAck(req: Request): Promise<Response> {
  const body = await req.json().catch(() => null);
  if (!body) return json(400, { error: "invalid JSON body" });
  const { original_message_id, ack_author_token, ack_state, body: msgBody, ack_slug } = body as any;
  if (!ack_author_token) return json(400, { error: "missing ack_author_token" });
  const agent = findAgentByToken(ack_author_token);
  if (!agent) return json(403, { error: "unknown ack_author_token" });
  if (!original_message_id || !ack_state || !ack_slug) {
    return json(400, { error: "missing original_message_id, ack_state, or ack_slug" });
  }

  const enc = new TextEncoder().encode(msgBody ?? "");
  const hashBuf = await crypto.subtle.digest("SHA-256", enc);
  const content_sha = Array.from(new Uint8Array(hashBuf)).map(b => b.toString(16).padStart(2, "0")).join("");

  const remoteResp = await callRemote("POST", `/messages/${original_message_id}/acknowledge`, {
    ack_author: agent.role,
    ack_intent: ack_state,
    ack_slug,
    content_sha,
    body: msgBody ?? null,
  }, agent.token);
  if (!remoteResp.ok) {
    const errBody = await remoteResp.text();
    return json(remoteResp.status, { error: "remote ack failed", remote: errBody });
  }
  return json(201, await remoteResp.json());
}

async function handleInbox(url: URL): Promise<Response> {
  const role = url.searchParams.get("role");
  const instance_id = url.searchParams.get("instance_id");
  if (!role) return json(400, { error: "missing role" });

  // Find a registered agent whose role+instance match (for the per-token call).
  const agent = Object.values(registry).find(a =>
    a.role === role && (instance_id ? a.instance_id === instance_id : true),
  );
  if (!agent) return json(404, { error: "no registered agent for role/instance" });

  const remoteResp = await callRemote("GET", `/inbox/${role}?state=PENDING`, undefined, agent.token);
  if (!remoteResp.ok) {
    return json(remoteResp.status, { error: "remote inbox failed", remote: await remoteResp.text() });
  }
  return json(200, await remoteResp.json());
}

async function handleOutbox(url: URL): Promise<Response> {
  const role = url.searchParams.get("role");
  const instance_id = url.searchParams.get("instance_id");
  if (!role) return json(400, { error: "missing role" });

  // Find a registered agent whose role+instance match (for the per-token call).
  const agent = Object.values(registry).find(a =>
    a.role === role && (instance_id ? a.instance_id === instance_id : true),
  );
  if (!agent) return json(404, { error: "no registered agent for role/instance" });

  const remoteResp = await callRemote("GET", `/outbox/${role}`, undefined, agent.token);
  if (!remoteResp.ok) {
    return json(remoteResp.status, { error: "remote outbox failed", remote: await remoteResp.text() });
  }
  return json(200, await remoteResp.json());
}

async function handleHealth(): Promise<Response> {
  return json(200, {
    status: "ok",
    registered_agents: Object.values(registry).map(a => ({
      role: a.role,
      instance_id: a.instance_id,
      last_polled_at: a.last_polled_at,
    })),
    endpoint: ENDPOINT,
    poll_interval_ms: POLL_INTERVAL_MS,
  });
}

// ── Polling loop ───────────────────────────────────────────────────────────

async function pollOnce(agent: AgentRecord): Promise<void> {
  try {
    const resp = await callRemote("GET", `/inbox/${agent.role}?state=PENDING`, undefined, agent.token);
    if (!resp.ok) {
      logWarn("poll failed", { role: agent.role, instance: agent.instance_id, status: resp.status });
      return;
    }
    const data = (await resp.json()) as any;
    const messages: any[] = data.messages ?? [];

    // Filter to messages the agent hasn't seen yet.
    const newMessages = messages.filter(m => !agent.last_seen_message_ids.includes(m.message_id));
    if (newMessages.length === 0) {
      agent.last_polled_at = new Date().toISOString();
      return;
    }

    // Write the full PENDING set to the notification file (so file-watch
    // sees a change). Caller distinguishes new vs already-seen via
    // last_seen_message_ids if needed.
    await fs.writeFile(notificationFileFor(agent), JSON.stringify({
      polled_at: new Date().toISOString(),
      messages,
      new_message_ids: newMessages.map(m => m.message_id),
    }, null, 2), "utf8");

    // POST to callback_url if registered.
    if (agent.callback_url) {
      try {
        await fetch(agent.callback_url, {
          method: "POST",
          headers: { "Content-Type": "application/json" },
          body: JSON.stringify({
            role: agent.role,
            instance_id: agent.instance_id,
            new_message_ids: newMessages.map(m => m.message_id),
            notification_file: notificationFileFor(agent),
          }),
        });
      } catch (e: any) {
        logWarn("callback_url POST failed", { url: agent.callback_url, error: e.message });
      }
    }

    agent.last_seen_message_ids = [...agent.last_seen_message_ids, ...newMessages.map(m => m.message_id)].slice(-200);
    agent.last_polled_at = new Date().toISOString();
    await saveRegistry(registry);

    logInfo("new messages", { role: agent.role, instance: agent.instance_id, count: newMessages.length });
  } catch (e: any) {
    logWarn("poll exception", { role: agent.role, instance: agent.instance_id, error: e.message });
  }
}

async function pollAll(): Promise<void> {
  await Promise.all(Object.values(registry).map(pollOnce));
}

// ── Bootstrap ──────────────────────────────────────────────────────────────

setInterval(() => { pollAll().catch(e => logWarn("pollAll error", e.message)); }, POLL_INTERVAL_MS);

const server = Bun.serve({
  hostname: HOST,
  port: PORT,
  async fetch(req) {
    const url = new URL(req.url);
    const p = url.pathname;
    try {
      if (req.method === "POST" && p === "/local/register") return await handleRegister(req);
      if (req.method === "POST" && p === "/local/send") return await handleSend(req);
      if (req.method === "POST" && p === "/local/ack") return await handleAck(req);
      if (req.method === "GET" && p === "/local/inbox") return await handleInbox(url);
      if (req.method === "GET" && p === "/local/outbox") return await handleOutbox(url);
      if (req.method === "GET" && p === "/local/health") return await handleHealth();
      return json(404, { error: "unknown sidecar path" });
    } catch (e: any) {
      return json(500, { error: e.message ?? String(e) });
    }
  },
});

logInfo(`CAACP sidecar listening on http://${HOST}:${PORT}`);
logInfo(`Endpoint: ${ENDPOINT}; poll interval ${POLL_INTERVAL_MS}ms; ${Object.keys(registry).length} registered agents`);
