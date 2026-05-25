use std::io::{BufRead, BufReader, Read, Write};
use std::net::TcpStream;
use std::os::unix::process::ExitStatusExt;
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

fn bin() -> &'static str {
    env!("CARGO_BIN_EXE_cruftless")
}

fn read_http(port: u16) -> String {
    read_http_path(port, "/")
}

fn read_http_path(port: u16, path: &str) -> String {
    let deadline = Instant::now() + Duration::from_secs(5);
    let mut stream = loop {
        match TcpStream::connect(("127.0.0.1", port)) {
            Ok(stream) => break stream,
            Err(err) if Instant::now() < deadline => {
                let _ = err;
                std::thread::sleep(Duration::from_millis(10));
            }
            Err(err) => panic!("connect http fixture: {err}"),
        }
    };
    stream
        .set_read_timeout(Some(Duration::from_secs(5)))
        .expect("read timeout");
    stream
        .write_all(
            format!("GET {path} HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n")
                .as_bytes(),
        )
        .expect("write request");
    let mut response = String::new();
    stream.read_to_string(&mut response).expect("read response");
    response
}

#[test]
fn http_server_hygiene_probe() {
    let fixture = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../pilots/rusty-js-http-server/fixtures/hygiene-node-http.mjs"
    );
    let mut child = Command::new(bin())
        .arg(fixture)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn cruftless http fixture");

    let stdout = child.stdout.take().expect("stdout pipe");
    let mut reader = BufReader::new(stdout);
    let mut first = String::new();
    reader.read_line(&mut first).expect("sentinel line");
    assert_eq!(first.trim(), "HS_SENTINEL_KEYS:");

    let mut port_line = String::new();
    reader.read_line(&mut port_line).expect("port line");
    let port: u16 = port_line
        .trim()
        .strip_prefix("HS_PORT:")
        .expect("HS_PORT line")
        .parse()
        .expect("numeric port");

    let response = read_http(port);

    assert!(
        response.starts_with("HTTP/1.1 200 OK"),
        "response: {response}"
    );
    assert!(
        response.contains("x-agent-object: coerced-header"),
        "response should use runtime ToString for object-valued headers: {response}"
    );
    assert!(
        response.ends_with("coerced-body"),
        "response should use runtime ToString for object body chunks: {response}"
    );

    let status = child.wait().expect("wait fixture");
    assert!(status.success(), "fixture failed with status {status}");
}

#[test]
fn http_server_eventemitter_request_probe() {
    let fixture = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../pilots/rusty-js-http-server/fixtures/eventemitter-node-http.mjs"
    );
    let mut child = Command::new(bin())
        .arg(fixture)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn cruftless http event fixture");

    let stdout = child.stdout.take().expect("stdout pipe");
    let mut reader = BufReader::new(stdout);
    let mut port_line = String::new();
    reader.read_line(&mut port_line).expect("event port line");
    let port: u16 = port_line
        .trim()
        .strip_prefix("HS_EVENT_PORT:")
        .expect("HS_EVENT_PORT line")
        .parse()
        .expect("numeric port");

    let response = read_http(port);

    assert!(
        response.starts_with("HTTP/1.1 200 OK"),
        "response: {response}"
    );
    assert!(
        response.contains("x-agent-event: once"),
        "response: {response}"
    );
    assert!(response.ends_with("event:1:/"), "response: {response}");

    let status = child.wait().expect("wait event fixture");
    assert!(
        status.success(),
        "event fixture failed with status {status}"
    );
}

#[test]
fn http_server_authority_modes() {
    let dynamic_fixture = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../pilots/rusty-js-http-server/fixtures/hygiene-node-http.mjs"
    );
    let audit_log = std::env::temp_dir().join(format!(
        "cruftless-http-audit-{}.log",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    let mut child = Command::new(bin())
        .arg("--audit")
        .arg("--audit-log")
        .arg(&audit_log)
        .arg(dynamic_fixture)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn audit fixture");
    let stdout = child.stdout.take().expect("stdout pipe");
    let mut reader = BufReader::new(stdout);
    let mut line = String::new();
    reader.read_line(&mut line).expect("sentinel line");
    let mut port_line = String::new();
    reader.read_line(&mut port_line).expect("port line");
    let port: u16 = port_line
        .trim()
        .strip_prefix("HS_PORT:")
        .expect("HS_PORT line")
        .parse()
        .expect("numeric port");
    let response = read_http(port);
    assert!(response.contains("coerced-body"), "response: {response}");
    let status = child.wait().expect("wait audit fixture");
    assert!(
        status.success(),
        "audit fixture failed with status {status}"
    );
    let audit = std::fs::read_to_string(&audit_log).expect("audit log");
    assert!(
        audit.contains("\tnet\tlisten(127.0.0.1:0)\t"),
        "audit log: {audit}"
    );
    let _ = std::fs::remove_file(&audit_log);

    let fixed_fixture = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../pilots/rusty-js-http-server/fixtures/authority-fixed-node-http.mjs"
    );
    let denied = Command::new(bin())
        .arg("--sealed")
        .arg(fixed_fixture)
        .output()
        .expect("run sealed-denied fixture");
    assert!(
        !denied.status.success(),
        "sealed run should deny ambient net"
    );
    let denied_stderr = String::from_utf8_lossy(&denied.stderr);
    assert!(
        denied_stderr.contains("no net capability granted"),
        "sealed stderr: {denied_stderr}"
    );

    let mut allowed = Command::new(bin())
        .arg("--sealed")
        .arg("--allow-net-loopback")
        .arg(fixed_fixture)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn sealed loopback fixture");
    let response = read_http(39731);
    assert!(response.ends_with("authority-ok"), "response: {response}");
    let status = allowed.wait().expect("wait sealed loopback fixture");
    assert!(
        status.success(),
        "sealed loopback fixture failed with status {status}"
    );
}

#[test]
fn http_server_compartment_facade_authority() {
    let no_ambient_fixture = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../pilots/rusty-js-http-server/fixtures/compartment-no-ambient-http.mjs"
    );
    let no_ambient = Command::new(bin())
        .arg(no_ambient_fixture)
        .output()
        .expect("run no-ambient compartment fixture");
    assert!(no_ambient.status.success(), "no-ambient fixture failed");
    let no_ambient_stdout = String::from_utf8_lossy(&no_ambient.stdout);
    assert!(
        no_ambient_stdout.contains("HS_COMPARTMENT_AMBIENT:undefined:undefined"),
        "no-ambient stdout: {no_ambient_stdout}"
    );

    let allowed_fixture = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../pilots/rusty-js-http-server/fixtures/compartment-facade-loopback-fixed.mjs"
    );
    let mut allowed = Command::new(bin())
        .arg("--sealed")
        .arg(allowed_fixture)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn sealed compartment loopback fixture");
    let response = read_http(39732);
    assert!(response.ends_with("realm-ok"), "response: {response}");
    let status = allowed
        .wait()
        .expect("wait sealed compartment loopback fixture");
    assert!(
        status.success(),
        "sealed compartment loopback fixture failed with status {status}"
    );

    let denied_fixture = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../pilots/rusty-js-http-server/fixtures/compartment-facade-deny-wide.mjs"
    );
    let denied = Command::new(bin())
        .arg("--sealed")
        .arg(denied_fixture)
        .output()
        .expect("run sealed compartment wide-bind fixture");
    assert!(!denied.status.success(), "wide bind should be denied");
    let denied_stderr = String::from_utf8_lossy(&denied.stderr);
    assert!(
        denied_stderr.contains("no net capability granted"),
        "wide-bind stderr: {denied_stderr}"
    );
}

#[test]
fn http_server_express_minimal_probe() {
    let fixture_root = std::env::var("CRUFTLESS_EXPRESS_FIXTURE_ROOT").unwrap_or_else(|_| {
        concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../pilots/rusty-js-http-server/fixtures/express-minimal"
        )
        .to_string()
    });
    if !std::path::Path::new(&fixture_root)
        .join("node_modules/express")
        .is_dir()
    {
        eprintln!(
            "skipping Express probe; install deps with `npm install --prefix {fixture_root}`"
        );
        return;
    }
    let fixture = std::path::Path::new(&fixture_root).join("express-minimal.mjs");
    let mut child = Command::new(bin())
        .arg(fixture)
        .current_dir(&fixture_root)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn Express fixture");

    let response = read_http(39733);
    assert!(
        response.starts_with("HTTP/1.1 200 OK"),
        "response: {response}"
    );
    assert!(response.contains("x-from: express"), "response: {response}");
    assert!(response.ends_with("hello express"), "response: {response}");

    let _ = child.kill();
    let status = child.wait().expect("wait Express fixture");
    assert!(
        status.success() || status.signal().is_some(),
        "Express fixture failed with status {status}"
    );
}

#[test]
fn http_server_express_middleware_probe() {
    let fixture_root = std::env::var("CRUFTLESS_EXPRESS_FIXTURE_ROOT").unwrap_or_else(|_| {
        concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../pilots/rusty-js-http-server/fixtures/express-minimal"
        )
        .to_string()
    });
    if !std::path::Path::new(&fixture_root)
        .join("node_modules/express")
        .is_dir()
    {
        eprintln!(
            "skipping Express middleware probe; install deps with `npm install --prefix {fixture_root}`"
        );
        return;
    }
    let fixture = std::path::Path::new(&fixture_root).join("express-middleware.mjs");
    let mut child = Command::new(bin())
        .arg(fixture)
        .current_dir(&fixture_root)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn Express middleware fixture");

    let response = read_http_path(39734, "/user/42?q=abc");
    assert!(
        response.starts_with("HTTP/1.1 201 OK"),
        "response: {response}"
    );
    assert!(response.contains("x-mid: seen"), "response: {response}");
    assert!(response.contains("x-route: 42"), "response: {response}");
    assert!(
        response.ends_with("user:42:q=abc:mid=yes"),
        "response: {response}"
    );

    let _ = child.kill();
    let status = child.wait().expect("wait Express middleware fixture");
    assert!(
        status.success() || status.signal().is_some(),
        "Express middleware fixture failed with status {status}"
    );
}
