//! Parse a struct from stdin and dump it to stdout

use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    match structparse::parse_struct(&mut input.as_str()) {
        Ok(s) => {
            println!("{s:#?}");
        }
        Err(e) => eprintln!("Parse error: {e}"),
    }
}
