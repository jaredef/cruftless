// Probe rusty-tls against a locally-running openssl s_server, which
// we control end-to-end. The server logs every record it receives;
// if our app-data send is malformed, the server's log says exactly
// how. This is the §XVI.b bidirectional engine-diff apparatus where
// engine A (openssl) is the spec-correct reference + an observer of
// engine B (rusty-tls).

use rusty_tls::driver::tls_connect;
use rusty_tls::store::TrustStore;

fn main() {
    // Load our self-signed cert as the trust anchor for this probe.
    let pem = std::fs::read_to_string("/tmp/tls-ext-4/cert.pem")
        .expect("read self-signed cert");
    let mut trust = TrustStore::new();
    let n = trust.add_pem_bundle(&pem).expect("add pem");
    println!("[1] loaded {} cert(s) into trust store", n);

    println!("[2] TLS connect → localhost:4443");
    let mut session = match tls_connect("localhost", 4443, &trust) {
        Ok(s) => { println!("    handshake OK"); s }
        Err(e) => { println!("    handshake FAILED: {:?}", e); return; }
    };

    let req = b"GET / HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n";
    println!("[3] send_application_data ({} bytes)", req.len());
    match session.send_application_data(req) {
        Ok(()) => println!("    send OK"),
        Err(e) => { println!("    send FAILED: {:?}", e); return; }
    }

    println!("[4] receive_application_data");
    let mut acc = Vec::new();
    let mut total = 0usize;
    let mut i = 0;
    loop {
        i += 1;
        match session.receive_application_data(&mut acc) {
            Ok(c) => { println!("    [{}] Ok({} bytes)", i, c.len()); total += c.len(); }
            Err(e) => { println!("    [{}] Err: {:?} (raw total {} bytes)", i, e, total); break; }
        }
        if i > 30 { break; }
    }
}
