fn main() {
    let src = std::env::args().nth(1).expect("source as arg");
    match rusty_js_parser::parse_module(&src) {
        Ok(m) => println!("{:#?}", m),
        Err(e) => eprintln!("ERR: {:?}", e),
    }
}
