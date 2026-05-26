use rusty_tls::driver::tls_connect;
use rusty_tls::store::TrustStore;
fn main() {
    let pem = std::fs::read_to_string("/tmp/tls-ext-4/ec-cert.pem").expect("read");
    let mut t = TrustStore::new();
    t.add_pem_bundle(&pem).expect("add");
    println!("[ec-probe] connecting...");
    let r = tls_connect("localhost", 4443, &t);
    println!(
        "[ec-probe] result: {:?}",
        r.map(|_| "OK").map_err(|e| format!("{:?}", e))
    );
}
