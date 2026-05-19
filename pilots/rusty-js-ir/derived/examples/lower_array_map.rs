//! Lower §23.1.3.20 Array.prototype.map IR to Rust and print to stdout.
//!
//! Run with: `cargo run --example lower_array_map -p rusty-js-ir`

use rusty_js_ir::lower_to_rust;
use rusty_js_ir::sections::array_prototype_map;

fn main() {
    let f = array_prototype_map::build();
    let rust = lower_to_rust(&f);
    println!("{}", rust);
}
