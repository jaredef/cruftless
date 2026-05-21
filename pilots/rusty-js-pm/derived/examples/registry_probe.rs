fn main() {
    let urls = [
        "https://registry.npmjs.org/lodash/4.17.21",
        "https://registry.yarnpkg.com/lodash/4.17.21",
        "https://registry.npmmirror.com/lodash/4.17.21",
        "https://npm.pkg.github.com/lodash/4.17.21",
    ];
    for url in &urls {
        match rusty_js_pm::http::pm_http_get(url) {
            Ok(b) => println!("{} → OK ({} bytes)", url, b.len()),
            Err(e) => println!("{} → ERR {:?}", url, e),
        }
    }
}
