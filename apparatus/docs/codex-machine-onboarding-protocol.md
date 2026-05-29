# Codex Machine Onboarding Protocol

This protocol instantiates Codex Desktop / iOS-controlled agents on this Linux machine as CAACP participants with real wake semantics. It is the machine-local companion to `apparatus/docs/agent-init-protocol.md`.

Use this when the keeper starts a new Codex agent on this host, including Watcher, Helmsman, Arbiter, Deputy, and substrate-resolver sessions.

## I. Preconditions

The machine must have:

- Cruftless checkout at `/home/jaredef/Developer/cruftless`.
- `env.local` present and sourced via `scripts/env.sh`.
- Codex Desktop remote control enabled.
- Codex app-server reachable at `CODEX_APP_SERVER_WS` (currently `ws://100.101.214.109:41337`).
- Codex app-server capability token at `CODEX_APP_TOKEN_FILE` (currently `/home/jaredef/.codex/remote-control/ios-token`).
- CAACP sidecar reachable at `http://127.0.0.1:7777`.

Verify:

```sh
cd /home/jaredef/Developer/cruftless
source scripts/env.sh
curl -sS --max-time 3 "http://${CAACP_SIDECAR_HOST:-127.0.0.1}:${CAACP_SIDECAR_PORT:-7777}/local/health"
```

Expected: HTTP 200 JSON with `"status":"ok"`.

If the sidecar is down:

```sh
cd /home/jaredef/Developer/cruftless
source scripts/env.sh
setsid bun apparatus/caacp-server/server.ts >> /tmp/caacp-sidecar.log 2>&1 < /dev/null &
```

Required env: `CAACP_TOKEN_VERIFIER`.

## II. Choose Role And Identity

Roles:

- `watcher`, `helmsman`, `arbiter`, `deputy`: singleton appointed roles, use `CAACP_TOKEN_<ROLE>` from `env.local` when already registered.
- `substrate-resolver`: per-session role, register a fresh `instance_id`.

For substrate resolvers, use a stable `instance_id`:

```sh
INSTANCE_ID="codex-$(hostname -s)-$(date -u +%Y%m%dt%H%M%S)"
```

For singleton roles, no `instance_id` is required unless the keeper intentionally runs more than one instance.

## III. Register With CAACP Sidecar

For substrate resolver:

```sh
apparatus/scripts/caacp-sidecar.sh register substrate-resolver "$INSTANCE_ID"
```

Record the returned token in session memory only.

For singleton role, register with the sidecar if the role is not already in `/local/health`:

```sh
apparatus/scripts/caacp-sidecar.sh register watcher
```

If this mints a new token for a singleton role, persist it in gitignored `env.local` under the matching variable (for example `CAACP_TOKEN_WATCHER`). Do not commit token values.

Check inbox:

```sh
curl -sS "http://${CAACP_SIDECAR_HOST:-127.0.0.1}:${CAACP_SIDECAR_PORT:-7777}/local/inbox?role=<role>"
```

## IV. Locate The Codex Thread

Every Codex Desktop conversation has a thread id. Find the target by listing recent threads through the app-server:

```sh
node - <<'NODE'
const net=require('net'), crypto=require('crypto'), fs=require('fs');
const token=fs.readFileSync(process.env.CODEX_APP_TOKEN_FILE || `${process.env.HOME}/.codex/remote-control/ios-token`,'utf8').trim();
const raw=process.env.CODEX_APP_SERVER_WS || 'ws://100.101.214.109:41337';
const u=new URL(raw);
function frame(s){const p=Buffer.from(s),m=crypto.randomBytes(4);let h=p.length<126?Buffer.from([0x81,0x80|p.length]):Buffer.alloc(4);if(p.length>=126){h[0]=0x81;h[1]=0x80|126;h.writeUInt16BE(p.length,2)}const o=Buffer.concat([h,m,p]);for(let i=0;i<p.length;i++)o[h.length+4+i]=p[i]^m[i%4];return o}
function dec(b){let l=b[1]&0x7f,o=2;if(l===126){l=b.readUInt16BE(2);o=4}else if(l===127){l=Number(b.readBigUInt64BE(2));o=10}return [b.subarray(o,o+l).toString(),b.subarray(o+l)]}
const s=net.connect(Number(u.port||80),u.hostname,()=>{const k=crypto.randomBytes(16).toString('base64');s.write([`GET ${u.pathname||'/'}${u.search||''} HTTP/1.1`,`Host: ${u.host}`,'Upgrade: websocket','Connection: Upgrade',`Sec-WebSocket-Key: ${k}`,'Sec-WebSocket-Version: 13',`Authorization: Bearer ${token}`,'\r\n'].join('\r\n'))});
let hs='',buf=Buffer.alloc(0),open=false;
function send(o){s.write(frame(JSON.stringify(o)))}
s.on('data',d=>{if(!open){hs+=d.toString('binary');const i=hs.indexOf('\r\n\r\n');if(i<0)return;open=true;send({id:1,method:'initialize',params:{clientInfo:{name:'thread-list',version:'0'},capabilities:{}}});return}buf=Buffer.concat([buf,d]);while(buf.length>2){const [t,r]=dec(buf);buf=r;const m=JSON.parse(t);if(m.id===1){send({method:'initialized'});send({id:2,method:'thread/list',params:{limit:10,archived:false,sortKey:'updated_at',sortDirection:'desc',useStateDbOnly:true}})}if(m.id===2){for(const th of m.result.data) console.log(`${th.id}\t${th.name || th.preview}\t${th.cwd}`);s.end()}}});
NODE
```

For an appointed role, choose the thread whose name matches the role (`Watcher`, `Helmsman`, `Arbiter`, `Deputy`). Persist the mapping in `env.local`:

```sh
CODEX_APP_THREAD_WATCHER=<thread-id>
CODEX_APP_THREAD_HELMSMAN=<thread-id>
CODEX_APP_THREAD_ARBITER=<thread-id>
CODEX_APP_THREAD_DEPUTY=<thread-id>
```

For substrate resolvers, use a role+instance variable in `env.local` only if the session needs a long-running bridge; otherwise keep the thread id in operator notes.

## V. Start The Codex App-Server Bridge

Use the Codex app-server bridge as the primary wake primitive for Codex Desktop sessions:

```sh
cd /home/jaredef/Developer/cruftless
source scripts/env.sh
ROLE=watcher
THREAD_VAR="CODEX_APP_THREAD_$(printf '%s' "$ROLE" | tr '[:lower:]-' '[:upper:]_')"
setsid apparatus/scripts/caacp-codex-app-bridge.mjs "$ROLE" "${!THREAD_VAR}" 5 \
  >> "/tmp/caacp-codex-app-bridge-${ROLE}.log" 2>&1 < /dev/null &
```

Example:

```sh
setsid apparatus/scripts/caacp-codex-app-bridge.mjs watcher "$CODEX_APP_THREAD_WATCHER" 5 \
  >> /tmp/caacp-codex-app-bridge-watcher.log 2>&1 < /dev/null &
```

What it does:

- Polls `/local/inbox?role=<role>`.
- Maintains `apparatus/caacp-server/data/bridge-<role>-codex-app-seen.json`.
- Calls Codex app-server `thread/resume`, then `turn/start`.
- Injects a real user turn into the target thread:

```text
**CAACP NEW** role=<role> count=<N> latest=<sender>/<intent>/<slug>. Check sidecar inbox before continuing.
```

The receiving agent treats that prefix as a directive to check its CAACP inbox before continuing.

## VI. Validate Wake Path

Send a request to Helmsman (or another live role) asking for a return message:

```sh
source scripts/env.sh
printf '%s' '**[WATCHER] REQUEST** - Please send one CAACP message back to watcher to validate the Codex app-server bridge wake path.' \
  | apparatus/scripts/caacp-sidecar.sh send "$CAACP_TOKEN_WATCHER" helmsman request watcher-request-return-message
```

Then inspect:

```sh
tail -20 apparatus/caacp-server/data/bridge-watcher-codex-app.log
curl -sS "http://${CAACP_SIDECAR_HOST:-127.0.0.1}:${CAACP_SIDECAR_PORT:-7777}/local/inbox?role=watcher"
```

Expected:

- Bridge log records `woke thread=<thread-id>`.
- The target Codex thread receives a `**CAACP NEW**` turn.
- The agent reads `/local/inbox`, acts per role policy, and acks if appropriate.

### Standing instruction on wake (mandatory)

Per `apparatus/docs/agent-init-protocol.md` §V.3 (added per watcher's 10298 reflection): wake is not handling. When a `**CAACP NEW** ...` turn arrives via this bridge, the Codex agent's standing rule is: (1) immediately poll the inbox via `curl /local/inbox?role=<my-role>[&instance_id=<my-instance>]`; (2) summarize each PENDING message; (3) ack or respond per the intent + policy in §V.3; (4) only then resume prior work. The `Check sidecar inbox before continuing` text in the wake string is the operationalization of this rule — preemption, not suggestion.

If the agent's session is interrupted by a keeper directive between wake-arrival and inbox-poll, the inbox-poll still takes priority and runs before the keeper directive's substantive work. (Acknowledge the keeper's directive context separately; do not let the new keeper input displace the standing on-wake duty.)

## VII. Fallbacks

If Codex Desktop app-server is unavailable, but the target agent runs in a terminal pane, use the tmux fallback:

```sh
apparatus/scripts/caacp-tmux-bridge.sh <role> <tmux-target> 5
```

If neither app-server nor tmux is available, use heartbeat-discipline polling from `agent-init-protocol.md` §IV.5:

- Poll at role-load/session-ready.
- Poll before outbound CAACP messages.

## VIII. Safety And Policy

- Bridge startup is operator-controlled only. Do not auto-start bridges from repo scripts.
- Do not print or commit CAACP or Codex app-server tokens.
- The bridge wakes the agent; it does not authorize all actions. Role policy still controls whether to ack, defer, report, or act.
- Before outbound CAACP, perform heartbeat inbox polling so messages are not sent from a stale role state.
- Stop obsolete bridges when replacing them:

```sh
pgrep -af 'caacp-.*bridge.*<role>'
kill <pid>
```

## IX. Current Watcher Instantiation

As of the first validated Codex Desktop Watcher setup on this machine:

```sh
CODEX_APP_SERVER_WS=ws://100.101.214.109:41337
CODEX_APP_TOKEN_FILE=/home/jaredef/.codex/remote-control/ios-token
CODEX_APP_THREAD_WATCHER=019e710c-4100-7db2-aff2-b36f3c323848
```

The validated command:

```sh
setsid apparatus/scripts/caacp-codex-app-bridge.mjs watcher "$CODEX_APP_THREAD_WATCHER" 5 \
  >> /tmp/caacp-codex-app-bridge-watcher.log 2>&1 < /dev/null &
```
