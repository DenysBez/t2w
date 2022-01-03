use std::ptr::null;
use crate::lexer::lexer::{get_token, Lexer, next_char, peek};
use crate::parser::parser::Parser;
use crate::symbol_table::symbol_table::Symbol_Table;
use crate::token::token::TokenType;

mod lexer;
mod token;
mod parser;
mod symbol_table;
mod emitter;

fn main() {
    println!("Main function");
    let mut lexer = Lexer {
        source: String::from(r#"PRINT "How many fibonacci numbers do you want?"
INPUT nums
PRINT ""

LET a = 0
LET b = 1
WHILE nums > 0 REPEAT
    PRINT a
    LET c = a + b
    LET a = b
    LET b = c
    LET nums = nums - 1
ENDWHILE
"#),
        cur_char: ' ',
        cur_pos: -1
    };



    let mut symbol_table = Symbol_Table::default();

   /* next_char(&mut lexer);
    let mut token = get_token(&mut lexer);
    while token.as_ref().is_some() && !matches!(token.as_ref().unwrap().kind, TokenType::EOF) {
        println!("{:?}", token.as_ref().unwrap().kind);

        token = get_token(&mut lexer)
    }*/

    let mut parser = Parser {
        lexer,
        symbol_table,
        cur_token: None,
        peek_token: None
    };

    parser.next_token();
    parser.next_token();
    //parser.next_token();

    parser.program();


}
