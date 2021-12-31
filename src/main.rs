use std::ptr::null;
use crate::lexer::lexer::{get_token, Lexer, next_char, peek};
use crate::token::token::TokenType;

mod lexer;
mod token;

fn main() {
    println!("Main function");
    let mut lexer = Lexer {
        source: String::from("+-123 9.8654*/"),
        cur_char: '\0',
        cur_pos: -1
    };


    /*while peek(&lexer) != '\0' {
        next_char(&mut lexer);
        println!("{}", lexer.cur_char);
    }*/

    next_char(&mut lexer);
    let mut token = get_token(&mut lexer);
    while token.as_ref().is_some() && !matches!(token.as_ref().unwrap().kind, TokenType::EOF) {
        println!("{:?}", token.as_ref().unwrap().kind);

        token = get_token(&mut lexer)
    }
}
