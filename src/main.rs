use crate::emitter::Emitter;
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::symbol_table::SymbolTable;

mod lexer;
mod token;
mod parser;
mod symbol_table;
mod emitter;
mod compiler;


fn main() {


    println!("Main function");
    let lexer = Lexer {
        source: String::from(r#"LET a = 0
WHILE a < 1 REPEAT
    PRINT "Enter number of scores: "
    INPUT a
ENDWHILE

LET b = 0
LET s = 0
PRINT "Enter one value at a time: "
WHILE b < a REPEAT
    INPUT c
    LET s = s + c
    LET b = b + 1
ENDWHILE

PRINT "Average: "
PRINT s / a

"#),
        cur_char: ' ',
        cur_pos: -1
    };


    let emitter = Emitter {
        file: "out.rs".to_string(),
        full_path: "C:/dev/rust/kääntäjän_lähtö/compiler_output/src/main.rs".to_string(),
        header: "".to_string(),
        code: "".to_string()
    };



    let symbol_table = SymbolTable::default();
    let mut parser = Parser {
        lexer,
        symbol_table,
        cur_token: None,
        peek_token: None,
        emitter
    };

    parser.next_token();
    parser.next_token();

    parser.program();

    parser.emitter.write_to_file();
}
