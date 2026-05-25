//! node:http intrinsic stub — Tier-Ω.5.r.
//!
//! Exposes enough shape that `import http from "node:http"` /
//! `require("node:http")` succeeds and `Object.keys(http).length > 0`,
//! which unblocks shape-probe parity passes for packages like node-fetch
//! that import the module unconditionally even when not all code paths
//! exercise it.
//!
//! All callable surface throws TypeError("not yet implemented") — the
//! goal is import-time success, not runtime functionality. Real HTTP
//! lives behind a future Tier-Π wiring round.

use crate::register::{new_object, register_method, set_constant};
use rusty_js_runtime::caps::{self, ModuleId};
use rusty_js_runtime::value::ObjectRef;
use rusty_js_runtime::{Runtime, RuntimeError, Value};
use std::cell::RefCell;
use std::rc::Rc;
use std::time::{Duration, Instant};

const SERVER_SLOT: &str = "__cruftless_http_server_id";
const BODY_SLOT: &str = "__cruftless_http_body";
const HEADERS_SLOT: &str = "__cruftless_http_headers";
const MAX_REQUEST_BYTES: usize = 1024 * 1024;

#[derive(Clone)]
struct ActiveHttpServer {
    listener_handle: u64,
    bound_addr: String,
    handler: Value,
    handler_realm: usize,
    server_object: ObjectRef,
    closing: bool,
}

thread_local! {
    static HTTP_SERVERS: RefCell<Vec<Option<ActiveHttpServer>>> = RefCell::new(Vec::new());
}

fn next_server_id(server: ActiveHttpServer) -> usize {
    HTTP_SERVERS.with(|servers| {
        let mut servers = servers.borrow_mut();
        if let Some((idx, slot)) = servers.iter_mut().enumerate().find(|(_, s)| s.is_none()) {
            *slot = Some(server);
            idx
        } else {
            servers.push(Some(server));
            servers.len() - 1
        }
    })
}

fn get_server(id: usize) -> Option<ActiveHttpServer> {
    HTTP_SERVERS.with(|servers| servers.borrow().get(id).and_then(|s| s.clone()))
}

fn update_server<F>(id: usize, f: F)
where F: FnOnce(&mut ActiveHttpServer) {
    HTTP_SERVERS.with(|servers| {
        if let Some(Some(server)) = servers.borrow_mut().get_mut(id) {
            f(server);
        }
    });
}

fn remove_server(id: usize) -> Option<ActiveHttpServer> {
    HTTP_SERVERS.with(|servers| servers.borrow_mut().get_mut(id).and_then(|s| s.take()))
}

fn value_to_string(v: &Value) -> String {
    rusty_js_runtime::abstract_ops::to_string(v).as_str().to_string()
}

fn value_to_bytes(v: &Value) -> Vec<u8> {
    value_to_string(v).into_bytes()
}

fn current_server_id(rt: &mut Runtime) -> Result<usize, RuntimeError> {
    let this_id = match rt.current_this() {
        Value::Object(id) => id,
        _ => return Err(RuntimeError::TypeError("node:http Server method: invalid receiver".into())),
    };
    match rt.object_get(this_id, SERVER_SLOT) {
        Value::Number(n) => Ok(n as usize),
        _ => Err(RuntimeError::TypeError("node:http Server method: missing server id".into())),
    }
}

fn make_server_object(rt: &mut Runtime, handler: Value) -> Result<ObjectRef, RuntimeError> {
    let server = new_object(rt);
    rt.object_set(server, "listening".into(), Value::Boolean(false));

    register_method(rt, server, "listen", |rt, args| {
        let this_id = match rt.current_this() {
            Value::Object(id) => id,
            _ => return Err(RuntimeError::TypeError("server.listen: invalid receiver".into())),
        };
        if matches!(rt.object_get(this_id, SERVER_SLOT), Value::Number(_)) {
            return Ok(Value::Object(this_id));
        }

        let port = match args.first() {
            Some(Value::Number(n)) => *n as u16,
            Some(Value::String(s)) => s.parse::<u16>().unwrap_or(0),
            _ => 0,
        };
        let host = match args.get(1) {
            Some(Value::String(s)) => s.as_str().to_string(),
            _ => "127.0.0.1".to_string(),
        };
        let callback = args.iter().find(|v| matches!(v, Value::Object(_))).cloned();

        rt.caps.require_net(
            &caps::Net::none(),
            caps::NetOp::Listen { host: host.clone(), port },
            &ModuleId::builtin("node:http"),
        ).map_err(|e| RuntimeError::TypeError(e.to_string()))?;

        let (listener_handle, bound_addr) = rusty_sockets::listener_bind_async(&format!("{host}:{port}"))
            .map_err(|e| RuntimeError::TypeError(format!("server.listen: {e:?}")))?;
        let handler = rt.object_get(this_id, "__cruftless_http_handler");
        let server_id = next_server_id(ActiveHttpServer {
            listener_handle,
            bound_addr: bound_addr.clone(),
            handler,
            handler_realm: rt.current_realm,
            server_object: this_id,
            closing: false,
        });
        rt.object_set(this_id, SERVER_SLOT.into(), Value::Number(server_id as f64));
        rt.object_set(this_id, "__cruftless_http_bound_addr".into(), Value::String(Rc::new(bound_addr)));
        rt.object_set(this_id, "listening".into(), Value::Boolean(true));
        if let Some(cb) = callback {
            let _ = rt.call_function(cb, Value::Object(this_id), Vec::new())?;
        }
        Ok(Value::Object(this_id))
    });

    register_method(rt, server, "address", |rt, _args| {
        let id = current_server_id(rt)?;
        let server = get_server(id).ok_or_else(|| RuntimeError::TypeError("server.address: closed".into()))?;
        let out = new_object(rt);
        let (host, port) = split_bound_addr(&server.bound_addr);
        rt.object_set(out, "address".into(), Value::String(Rc::new(host)));
        rt.object_set(out, "family".into(), Value::String(Rc::new("IPv4".into())));
        rt.object_set(out, "port".into(), Value::Number(port as f64));
        Ok(Value::Object(out))
    });

    register_method(rt, server, "close", |rt, args| {
        let id = current_server_id(rt)?;
        if let Some(server) = remove_server(id) {
            let _ = rusty_sockets::listener_stop_async(server.listener_handle);
            rt.object_set(server.server_object, "listening".into(), Value::Boolean(false));
        }
        if let Some(cb) = args.iter().find(|v| matches!(v, Value::Object(_))).cloned() {
            let _ = rt.call_function(cb, rt.current_this(), Vec::new())?;
        }
        Ok(rt.current_this())
    });

    register_method(rt, server, "on", |_rt, _args| Ok(Value::Undefined));
    register_method(rt, server, "once", |_rt, _args| Ok(Value::Undefined));

    rt.object_set(server, "__cruftless_http_handler".into(), handler);
    Ok(server)
}

fn split_bound_addr(addr: &str) -> (String, u16) {
    match addr.rsplit_once(':') {
        Some((host, port)) => (host.trim_matches(['[', ']']).to_string(), port.parse().unwrap_or(0)),
        None => (addr.to_string(), 0),
    }
}

fn make_request_object(rt: &mut Runtime, req: &rusty_http_codec::ParsedRequest) -> ObjectRef {
    let obj = new_object(rt);
    rt.object_set(obj, "method".into(), Value::String(Rc::new(req.method.clone())));
    rt.object_set(obj, "url".into(), Value::String(Rc::new(req.target.clone())));
    rt.object_set(obj, "httpVersion".into(), Value::String(Rc::new(req.version.trim_start_matches("HTTP/").to_string())));
    let headers = new_object(rt);
    for (name, value) in &req.headers {
        rt.object_set(headers, name.to_ascii_lowercase(), Value::String(Rc::new(value.clone())));
    }
    rt.object_set(obj, "headers".into(), Value::Object(headers));
    obj
}

fn make_response_object(rt: &mut Runtime) -> ObjectRef {
    let obj = new_object(rt);
    let headers = new_object(rt);
    rt.object_set(obj, "statusCode".into(), Value::Number(200.0));
    rt.object_set(obj, "statusMessage".into(), Value::String(Rc::new("OK".into())));
    rt.object_set(obj, "headersSent".into(), Value::Boolean(false));
    rt.object_set(obj, HEADERS_SLOT.into(), Value::Object(headers));
    rt.object_set(obj, BODY_SLOT.into(), Value::String(Rc::new(String::new())));

    register_method(rt, obj, "setHeader", |rt, args| {
        let this = match rt.current_this() { Value::Object(id) => id, _ => return Ok(Value::Undefined) };
        let headers = match rt.object_get(this, HEADERS_SLOT) { Value::Object(id) => id, _ => return Ok(Value::Undefined) };
        let name = args.first().map(value_to_string).unwrap_or_default().to_ascii_lowercase();
        let value = args.get(1).map(value_to_string).unwrap_or_default();
        rt.object_set(headers, name, Value::String(Rc::new(value)));
        Ok(rt.current_this())
    });
    register_method(rt, obj, "getHeader", |rt, args| {
        let this = match rt.current_this() { Value::Object(id) => id, _ => return Ok(Value::Undefined) };
        let headers = match rt.object_get(this, HEADERS_SLOT) { Value::Object(id) => id, _ => return Ok(Value::Undefined) };
        let name = args.first().map(value_to_string).unwrap_or_default().to_ascii_lowercase();
        Ok(rt.object_get(headers, &name))
    });
    register_method(rt, obj, "removeHeader", |rt, args| {
        let this = match rt.current_this() { Value::Object(id) => id, _ => return Ok(Value::Undefined) };
        let headers = match rt.object_get(this, HEADERS_SLOT) { Value::Object(id) => id, _ => return Ok(Value::Undefined) };
        let name = args.first().map(value_to_string).unwrap_or_default().to_ascii_lowercase();
        rt.object_set(headers, name, Value::Undefined);
        Ok(rt.current_this())
    });
    register_method(rt, obj, "writeHead", |rt, args| {
        let this = match rt.current_this() { Value::Object(id) => id, _ => return Ok(Value::Undefined) };
        if let Some(Value::Number(n)) = args.first() {
            rt.object_set(this, "statusCode".into(), Value::Number(*n));
        }
        let header_arg = if let Some(Value::String(s)) = args.get(1) {
            rt.object_set(this, "statusMessage".into(), Value::String(s.clone()));
            args.get(2).cloned()
        } else {
            args.get(1).cloned()
        };
        if let Some(Value::Object(hid)) = header_arg {
            let headers = match rt.object_get(this, HEADERS_SLOT) { Value::Object(id) => id, _ => return Ok(Value::Undefined) };
            for key in rt.ordinary_own_enumerable_string_keys(hid) {
                let value = value_to_string(&rt.object_get(hid, &key));
                rt.object_set(headers, key.to_ascii_lowercase(), Value::String(Rc::new(value)));
            }
        }
        Ok(rt.current_this())
    });
    register_method(rt, obj, "write", |rt, args| {
        let this = match rt.current_this() { Value::Object(id) => id, _ => return Ok(Value::Undefined) };
        let mut body = match rt.object_get(this, BODY_SLOT) {
            Value::String(s) => s.as_str().to_string(),
            _ => String::new(),
        };
        if let Some(chunk) = args.first() {
            body.push_str(&value_to_string(chunk));
        }
        rt.object_set(this, BODY_SLOT.into(), Value::String(Rc::new(body)));
        Ok(Value::Boolean(true))
    });
    register_method(rt, obj, "end", |rt, args| {
        let this = match rt.current_this() { Value::Object(id) => id, _ => return Ok(Value::Undefined) };
        if let Some(chunk) = args.first() {
            let mut body = match rt.object_get(this, BODY_SLOT) {
                Value::String(s) => s.as_str().to_string(),
                _ => String::new(),
            };
            body.push_str(&value_to_string(chunk));
            rt.object_set(this, BODY_SLOT.into(), Value::String(Rc::new(body)));
        }
        rt.object_set(this, "__cruftless_http_ended".into(), Value::Boolean(true));
        Ok(Value::Undefined)
    });

    obj
}

fn response_to_wire(rt: &mut Runtime, res: ObjectRef) -> Vec<u8> {
    let status = match rt.object_get(res, "statusCode") {
        Value::Number(n) => n as u16,
        _ => 200,
    };
    let reason = match rt.object_get(res, "statusMessage") {
        Value::String(s) => s.as_str().to_string(),
        _ => "OK".to_string(),
    };
    let body = match rt.object_get(res, BODY_SLOT) {
        Value::String(s) => s.as_bytes().to_vec(),
        v => value_to_bytes(&v),
    };
    let mut headers = Vec::new();
    if let Value::Object(hid) = rt.object_get(res, HEADERS_SLOT) {
        for key in rt.ordinary_own_enumerable_string_keys(hid) {
            if matches!(rt.object_get(hid, &key), Value::Undefined) { continue; }
            headers.push((key.clone(), value_to_string(&rt.object_get(hid, &key))));
        }
    }
    if !headers.iter().any(|(n, _)| n.eq_ignore_ascii_case("connection")) {
        headers.push(("connection".into(), "close".into()));
    }
    rusty_http_codec::serialize_response(status, &reason, &headers, &body)
}

fn read_request(stream_id: u64) -> Result<Vec<u8>, String> {
    let deadline = Instant::now() + Duration::from_millis(500);
    let mut buf = Vec::new();
    while Instant::now() < deadline {
        let chunk = rusty_sockets::stream_read(stream_id, 8192).map_err(|e| format!("{e:?}"))?;
        if chunk.is_empty() { break; }
        buf.extend_from_slice(&chunk);
        if buf.len() > MAX_REQUEST_BYTES {
            return Err("request too large".into());
        }
        if request_complete(&buf) {
            return Ok(buf);
        }
    }
    Ok(buf)
}

fn request_complete(buf: &[u8]) -> bool {
    let Some(header_end) = find_header_end(buf) else { return false; };
    let headers = String::from_utf8_lossy(&buf[..header_end]);
    let content_length = headers
        .lines()
        .find_map(|line| {
            let (name, value) = line.split_once(':')?;
            if name.eq_ignore_ascii_case("content-length") {
                value.trim().parse::<usize>().ok()
            } else {
                None
            }
        })
        .unwrap_or(0);
    buf.len() >= header_end + content_length
}

fn find_header_end(buf: &[u8]) -> Option<usize> {
    buf.windows(4).position(|w| w == b"\r\n\r\n").map(|idx| idx + 4)
}

pub fn poll_io(rt: &mut Runtime) -> Result<bool, RuntimeError> {
    let ids: Vec<(usize, u64)> = HTTP_SERVERS.with(|servers| {
        servers.borrow().iter().enumerate()
            .filter_map(|(idx, s)| s.as_ref().filter(|srv| !srv.closing).map(|srv| (idx, srv.listener_handle)))
            .collect()
    });
    let has_active = !ids.is_empty();
    for (server_id, listener_handle) in ids {
        match rusty_sockets::listener_poll(listener_handle, 10) {
            Ok(Some(rusty_sockets::AsyncEvent::Connection { stream_id, .. })) => {
                rt.enqueue_macrotask("http server request", move |rt| {
                    handle_connection(rt, server_id, stream_id);
                    Ok(())
                });
                return Ok(true);
            }
            Ok(Some(rusty_sockets::AsyncEvent::Closed)) => {
                update_server(server_id, |srv| srv.closing = true);
            }
            Ok(Some(rusty_sockets::AsyncEvent::Error(_))) | Ok(None) => {}
            Err(e) => return Err(RuntimeError::TypeError(format!("http poll_io: {e:?}"))),
        }
    }
    Ok(has_active)
}

fn handle_connection(rt: &mut Runtime, server_id: usize, stream_id: u64) {
    let Some(server) = get_server(server_id) else {
        let _ = rusty_sockets::handle_close(stream_id);
        return;
    };
    let response = match read_request(stream_id)
        .and_then(|bytes| rusty_http_codec::parse_request(&bytes).map_err(|e| e.to_string()))
    {
        Ok(parsed) => {
            let req = make_request_object(rt, &parsed);
            let res = make_response_object(rt);
            let prior = rt.enter_realm(server.handler_realm);
            let call_result = rt.call_function(
                server.handler.clone(),
                Value::Object(server.server_object),
                vec![Value::Object(req), Value::Object(res)],
            );
            rt.exit_realm(prior);
            if call_result.is_err() {
                rusty_http_codec::serialize_response(500, "Internal Server Error", &[("connection".into(), "close".into())], b"")
            } else {
                response_to_wire(rt, res)
            }
        }
        Err(_) => rusty_http_codec::serialize_response(400, "Bad Request", &[("connection".into(), "close".into())], b""),
    };
    let _ = rusty_sockets::stream_write_all(stream_id, &response);
    let _ = rusty_sockets::handle_close(stream_id);
}

pub fn install(rt: &mut Runtime) {
    let http = new_object(rt);

    register_method(rt, http, "request", |_rt, _args| {
        Err(RuntimeError::TypeError(
            "node:http http.request: not yet implemented (Tier-Ω.5.r stub)".into(),
        ))
    });
    register_method(rt, http, "get", |_rt, _args| {
        Err(RuntimeError::TypeError(
            "node:http http.get: not yet implemented (Tier-Ω.5.r stub)".into(),
        ))
    });
    register_method(rt, http, "createServer", |rt, args| {
        let handler = match args {
            [Value::Object(_)] => args[0].clone(),
            [_, Value::Object(_), ..] => args[1].clone(),
            _ => {
                return Err(RuntimeError::TypeError(
                    "node:http createServer: handler must be callable".into(),
                ));
            }
        };
        let server = make_server_object(rt, handler)?;
        Ok(Value::Object(server))
    });
    // Ω.5.P49.E4: parallel to https.Agent — benign at module-init.
    register_method(rt, http, "Agent", |rt, _args| {
        let id = rt.alloc_object(rusty_js_runtime::Object::new_ordinary());
        Ok(Value::Object(id))
    });

    // STATUS_CODES — partial. Enough entries that callers probing
    // `STATUS_CODES[200]` / `STATUS_CODES[404]` get sensible strings.
    let codes = new_object(rt);
    for (code, msg) in &[
        (100, "Continue"),
        (101, "Switching Protocols"),
        (200, "OK"),
        (201, "Created"),
        (202, "Accepted"),
        (204, "No Content"),
        (301, "Moved Permanently"),
        (302, "Found"),
        (304, "Not Modified"),
        (307, "Temporary Redirect"),
        (308, "Permanent Redirect"),
        (400, "Bad Request"),
        (401, "Unauthorized"),
        (403, "Forbidden"),
        (404, "Not Found"),
        (405, "Method Not Allowed"),
        (408, "Request Timeout"),
        (409, "Conflict"),
        (410, "Gone"),
        (429, "Too Many Requests"),
        (500, "Internal Server Error"),
        (501, "Not Implemented"),
        (502, "Bad Gateway"),
        (503, "Service Unavailable"),
        (504, "Gateway Timeout"),
    ] {
        set_constant(rt, codes, &code.to_string(), Value::String(Rc::new((*msg).into())));
    }
    set_constant(rt, http, "STATUS_CODES", Value::Object(codes));

    // METHODS — list of supported HTTP method names. node-fetch and
    // similar shims occasionally read this; express's lib/utils.js:30
    // calls `METHODS.map(m => m.toLowerCase())`, requiring METHODS to
    // be a real Array (not a plain Object with integer keys + length).
    // Ω.5.P58.E2: allocate as Array so Array.prototype.map (and the
    // rest of the iteration protocol) resolves through the prototype
    // chain. Pre-P58.E2 METHODS was a plain Object; consumers that
    // probed it as iterable broke at module-init.
    let methods = rt.alloc_object(rusty_js_runtime::value::Object::new_array());
    let names = [
        "ACL", "BIND", "CHECKOUT", "CONNECT", "COPY", "DELETE", "GET", "HEAD",
        "LINK", "LOCK", "M-SEARCH", "MERGE", "MKACTIVITY", "MKCALENDAR", "MKCOL",
        "MOVE", "NOTIFY", "OPTIONS", "PATCH", "POST", "PROPFIND", "PROPPATCH",
        "PURGE", "PUT", "QUERY", "REBIND", "REPORT", "SEARCH", "SOURCE",
        "SUBSCRIBE", "TRACE", "UNBIND", "UNLINK", "UNLOCK", "UNSUBSCRIBE",
    ];
    for (i, n) in names.iter().enumerate() {
        rt.object_set(methods, i.to_string(), Value::String(Rc::new((*n).into())));
    }
    rt.object_set(methods, "length".into(), Value::Number(names.len() as f64));
    set_constant(rt, http, "METHODS", Value::Object(methods));

    // Tier-Ω.5.xxxxxx: http.ServerResponse / IncomingMessage / Server class
    // stubs with .prototype. compression/on-headers/koa-style middleware read
    // `http.ServerResponse.prototype.appendHeader` at module-init; without
    // ServerResponse the lookup throws on `.prototype`. The class is a
    // stub: constructor errors if called, prototype is an empty object,
    // sufficient for `class X extends http.ServerResponse` and
    // `typeof http.ServerResponse.prototype.foo === 'function'` probes.
    for class_name in &["ServerResponse", "IncomingMessage", "Server", "ClientRequest"] {
        let proto = new_object(rt);
        let ctor = crate::register::make_callable(rt, class_name, |_rt, _args| {
            Err(RuntimeError::TypeError(format!(
                "node:http class constructor not yet implemented (Tier-Ω.5.xxxxxx stub)",
            )))
        });
        rt.obj_mut(ctor).set_own_frozen("prototype".into(), Value::Object(proto));
        rt.obj_mut(proto).set_own_internal("constructor".into(), Value::Object(ctor));
        set_constant(rt, http, class_name, Value::Object(ctor));
    }

    // Default export points at the namespace itself for CJS-interop
    // shape: `import http from "node:http"` reads `default` and falls
    // back to the namespace if absent, but writing it explicitly keeps
    // `http.default === http` round-trip honest for callers that probe.
    set_constant(rt, http, "default", Value::Object(http));

    rt.globals.insert("http".into(), Value::Object(http));
}
