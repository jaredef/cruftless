fn main() {
    for url in &[
        "https://example.com/",
        "https://httpbin.org/get",
        "https://www.google.com/",
        "https://api.github.com/",
        "https://registry.npmjs.org/lodash/4.17.21",
    ] {
        print!("{} → ", url);
        match rusty_js_pm::http::pm_http_get(url) {
            Ok(b) => println!("OK ({} bytes)", b.len()),
            Err(e) => println!("ERR {:?}", e),
        }
    }
}
