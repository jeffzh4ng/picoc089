use din::{lexer, parser::Parser};
use std::fs;

fn main() {
    println!(
        "
    ⠀⠀⠀⠀⠀⣼⣧⠀⠀⠀⠀⠀
    ⠀⠀⠀⠀⣼⣿⣿⣧⠀⠀⠀⠀
    ⠀⠀⠀⠾⠿⠿⠿⠿⠷⠀⠀⠀
    ⠀⠀⣼⣆⠀⠀⠀⠀⣰⣧⠀⠀
    ⠀⣼⣿⣿⣆⠀⠀⣰⣿⣿⣧⠀
    ⠾⠟⠿⠿⠿⠧⠼⠿⠿⠿⠻⠷
    din: C89 -> RISC V
    "
    );
    let chars = fs::read("tests/valid/hello.c")
        .expect("Should have been able to read the file")
        .iter()
        .map(|b| *b as char)
        .collect();
    let tokens = lexer::scan(chars);
    // let tree = Parser::parse(tokens);
    println!("{:?}", tokens);
}
