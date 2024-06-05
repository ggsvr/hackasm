use hackasm::parser::parse_asm;
use std::io::prelude::*;

fn main() {
    let mut file = String::new();
    std::io::stdin().read_to_string(&mut file).unwrap();
    let res = parse_asm(&file);
    println!("{res:?}");
}
