use std::ptr::null;
use crate::lexer::lexer::{get_token, Lexer, next_char, peek};
use crate::parser::parser::Parser;
use crate::token::token::TokenType;

mod lexer;
mod token;
mod parser;

fn main() {
    println!("Main function");
    let mut lexer = Lexer {
        source: String::from(r#"PRINT "hello, world!"
        "#),
        cur_char: ' ',
        cur_pos: -1
    };



   /* next_char(&mut lexer);
    let mut token = get_token(&mut lexer);
    while token.as_ref().is_some() && !matches!(token.as_ref().unwrap().kind, TokenType::EOF) {
        println!("{:?}", token.as_ref().unwrap().kind);

        token = get_token(&mut lexer)
    }*/

    let mut parser = Parser {
        lexer,
        cur_token: None,
        peek_token: None
    };

    parser.next_token();
    parser.next_token();
    //parser.next_token();

    parser.program();


}
