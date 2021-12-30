use std::ptr::null;
use crate::lexer::lexer::{Lexer, next_char};

mod lexer;

fn main() {
    println!("Main function");
    let mut lexer = Lexer {
        source: "LET foobar = 123".to_string(),
        cur_char: '\0',
        cur_pos: -1
    };

    next_char(&mut lexer);
}
