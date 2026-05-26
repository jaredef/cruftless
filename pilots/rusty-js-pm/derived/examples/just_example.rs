fn main() {
    let url = std::env::args()
        .nth(1)
        .unwrap_or("https://example.com/".into());
    let r = rusty_js_pm::http::pm_http_get(&url);
    println!("RESULT: {:?}", r.map(|b| b.len()));
}
