//! HTTPS GET for the PM, composed on the engagement's own TLS + HTTP
//! pilots (Doc 729 §V compositional-safety principle: resolver-
//! instance #0 composes on resolver-instance #1's existing surface
//! rather than pulling in a parallel stack).
//!
//! PM-EXT 4 first cut:
//! - HTTPS only (`https://` scheme). Plain HTTP and other schemes are
//!   rejected. The npm registry is HTTPS-only as of 2026.
//! - No redirect following. The registry's canonical URLs return 200
//!   directly; redirects are second-cut.
//! - `Connection: close` per request (no keep-alive); avoids tracking
//!   pool state for the first cut. PM-EXT N+1 may add pooling.
//! - System default CA bundle via TrustStore::load_system_default().
//!
//! Public surface: `pm_http_get(url) -> Result<Vec<u8>, HttpError>`.

use rusty_tls::driver::tls_connect;
use rusty_tls::store::TrustStore;
use rusty_http_codec::{parse_response, serialize_request};

#[derive(Debug)]
pub enum HttpError {
    UnsupportedScheme(String),
    MalformedUrl(String),
    TrustStore(String),
    Tls(String),
    Codec(String),
    Status { code: u16, body_prefix: String },
}

struct ParsedUrl {
    host: String,
    port: u16,
    path: String,
}

fn parse_url(url: &str) -> Result<ParsedUrl, HttpError> {
    // First cut: https only.
    let rest = url.strip_prefix("https://")
        .ok_or_else(|| HttpError::UnsupportedScheme(url.to_string()))?;
    let (authority, path) = match rest.find('/') {
        Some(i) => (&rest[..i], &rest[i..]),
        None => (rest, "/"),
    };
    if authority.is_empty() {
        return Err(HttpError::MalformedUrl(url.to_string()));
    }
    let (host, port) = match authority.find(':') {
        Some(i) => {
            let p: u16 = authority[i + 1..].parse()
                .map_err(|_| HttpError::MalformedUrl(url.to_string()))?;
            (&authority[..i], p)
        }
        None => (authority, 443),
    };
    Ok(ParsedUrl {
        host: host.to_string(),
        port,
        path: if path.is_empty() { "/".to_string() } else { path.to_string() },
    })
}

/// HTTPS GET. Returns the response body bytes on 2xx. Errors loudly
/// on TLS failure, non-2xx status, or malformed input. No redirects.
pub fn pm_http_get(url: &str) -> Result<Vec<u8>, HttpError> {
    let resp = pm_http_get_raw(url)?;
    finalize_raw(resp)
}

/// HTTPS GET with bounded redirect following. Used by the tarball
/// fetcher: registries commonly 302 to a CDN (e.g.
/// registry.npmmirror.com → cdn.npmmirror.com). Follows up to
/// `max_hops` 3xx responses with `Location:` headers; only https
/// targets accepted.
pub fn pm_http_get_follow(url: &str, max_hops: u8) -> Result<Vec<u8>, HttpError> {
    let mut current = url.to_string();
    for _ in 0..=max_hops {
        let resp = pm_http_get_raw(&current)?;
        if (300..400).contains(&resp.status) {
            let loc = header_value(&resp.headers, "location")
                .ok_or_else(|| HttpError::Status {
                    code: resp.status,
                    body_prefix: "3xx without Location header".into(),
                })?;
            current = resolve_location(&current, &loc)?;
            continue;
        }
        return finalize_raw(resp);
    }
    Err(HttpError::Status { code: 310, body_prefix: format!("too many redirects from {url}") })
}

fn header_value(headers: &[(String, String)], name: &str) -> Option<String> {
    headers.iter()
        .find(|(k, _)| k.eq_ignore_ascii_case(name))
        .map(|(_, v)| v.clone())
}

fn resolve_location(base: &str, loc: &str) -> Result<String, HttpError> {
    if loc.starts_with("https://") {
        Ok(loc.to_string())
    } else if loc.starts_with("http://") {
        Err(HttpError::UnsupportedScheme(loc.to_string()))
    } else if loc.starts_with('/') {
        // path-absolute on same authority
        let b = parse_url(base)?;
        let port_suffix = if b.port == 443 { String::new() } else { format!(":{}", b.port) };
        Ok(format!("https://{}{}{}", b.host, port_suffix, loc))
    } else {
        Err(HttpError::MalformedUrl(format!("relative redirect not supported: {loc}")))
    }
}

fn finalize_raw(resp: rusty_http_codec::ParsedResponse) -> Result<Vec<u8>, HttpError> {
    if !(200..300).contains(&resp.status) {
        let prefix: String = String::from_utf8_lossy(&resp.body)
            .chars().take(200).collect();
        return Err(HttpError::Status { code: resp.status, body_prefix: prefix });
    }
    Ok(resp.body)
}

fn pm_http_get_raw(url: &str) -> Result<rusty_http_codec::ParsedResponse, HttpError> {
    let dbg = std::env::var("CRUFTLESS_TLS_DEBUG").is_ok();
    if dbg { eprintln!("[pm_http_get] start {}", url); }
    let u = parse_url(url)?;
    let trust_store = TrustStore::load_system_default()
        .map_err(|e| HttpError::TrustStore(format!("{e:?}")))?;
    if dbg { eprintln!("[pm_http_get] connecting → {}:{}", u.host, u.port); }
    let mut session = tls_connect(&u.host, u.port, &trust_store)
        .map_err(|e| HttpError::Tls(format!("connect {}:{}: {e:?}", u.host, u.port)))?;
    if dbg { eprintln!("[pm_http_get] handshake OK"); }

    let request = serialize_request(
        "GET",
        &u.path,
        &[
            ("Host".into(), u.host.clone()),
            ("User-Agent".into(), "cruftless-pm/0.1.0".into()),
            ("Accept".into(), "application/json,*/*".into()),
            ("Accept-Encoding".into(), "identity".into()),
            ("Connection".into(), "close".into()),
        ],
        &[],
    );
    if dbg { eprintln!("[pm_http_get] sending {} request bytes", request.len()); }
    session.send_application_data(&request)
        .map_err(|e| HttpError::Tls(format!("send: {e:?}")))?;
    if dbg { eprintln!("[pm_http_get] send OK; entering drain loop"); }

    // Drain the response. With Connection: close the server sends its
    // close_notify after the body; receive_application_data now
    // returns Err(CloseNotify) explicitly (TLS-EXT 2). CloseNotify is
    // benign: hand whatever we accumulated to parse_response. Other
    // errors propagate.
    let mut raw = Vec::<u8>::new();
    let mut accumulator = Vec::<u8>::new();
    loop {
        match session.receive_application_data(&mut accumulator) {
            Ok(chunk) => {
                if chunk.is_empty() && accumulator.is_empty() { break; }
                raw.extend_from_slice(&chunk);
                // Early termination once we have a complete parseable
                // response, so we don't hang waiting for close_notify
                // on servers that drop the connection without one.
                if let Ok(resp) = parse_response(&raw) {
                    if resp.status >= 100 { return Ok(resp); }
                }
            }
            Err(rusty_tls::record::TlsError::CloseNotify) => break,
            Err(rusty_tls::record::TlsError::UnexpectedEnd) => break,
            Err(e) => return Err(HttpError::Tls(format!("recv: {e:?}"))),
        }
    }

    parse_response(&raw).map_err(|e| HttpError::Codec(format!("{e:?}")))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn url_parse_basic() {
        let u = parse_url("https://registry.npmjs.org/lodash/4.17.21").unwrap();
        assert_eq!(u.host, "registry.npmjs.org");
        assert_eq!(u.port, 443);
        assert_eq!(u.path, "/lodash/4.17.21");
    }

    #[test]
    fn url_parse_no_path() {
        let u = parse_url("https://registry.npmjs.org").unwrap();
        assert_eq!(u.path, "/");
    }

    #[test]
    fn url_parse_explicit_port() {
        let u = parse_url("https://example.com:8443/foo").unwrap();
        assert_eq!(u.port, 8443);
    }

    #[test]
    fn url_parse_rejects_http() {
        assert!(matches!(parse_url("http://x/y"), Err(HttpError::UnsupportedScheme(_))));
    }

    /// Network-dependent. Gated behind --ignored. Run via:
    ///   cargo test -p rusty-js-pm --release -- --ignored fetch_lodash
    #[test]
    #[ignore]
    fn fetch_lodash_manifest() {
        let body = pm_http_get("https://registry.npmjs.org/lodash/4.17.21")
            .expect("registry fetch failed");
        let text = std::str::from_utf8(&body).expect("body not utf-8");
        assert!(text.contains("\"version\":\"4.17.21\""),
            "expected version in response, got {} bytes; first 200: {}",
            text.len(), &text[..text.len().min(200)]);
        assert!(text.contains("\"tarball\""), "expected dist.tarball in response");
    }
}
