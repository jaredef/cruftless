fn main() {
    let body =
        rusty_js_pm::http::pm_http_get("https://registry.npmmirror.com/lodash/4.17.21").unwrap();
    let s = String::from_utf8_lossy(&body);
    println!("body length: {}", s.len());
    println!("first 500 chars: {}", &s[..s.len().min(500)]);
}
