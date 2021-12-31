pub mod lexer {
    use crate::token::token::{Token, TokenType};

    pub struct Lexer {
        pub source:  String,
        pub cur_char: char,
        pub cur_pos: isize
    }

    pub fn next_char(lexer: &mut Lexer) {
        lexer.cur_pos += 1;
        if lexer.cur_pos >= lexer.source.len() as isize {
            lexer.cur_char = '\0' //EOF
        } else {
            lexer.cur_char = lexer.source.chars().nth(lexer.cur_pos as usize).unwrap();
        }
    }

    pub fn peek(lexer: &Lexer) -> char {
        if lexer.cur_pos + 1 >= lexer.source.len() as isize {
            return '\0';
        }

        return lexer.source.chars().nth((lexer.cur_pos + 1) as usize).unwrap();
    }

    fn abort(lexer: &Lexer) {
        println!("abort called")
    }

    fn skip_whitespace(lexer: &Lexer) {
        println!("skipWhitespace called")
    }

    fn skip_comment(lexer: &Lexer) {
        println!("skipComment called")
    }

    pub fn get_token(lexer: &mut Lexer) -> Option<Token> {

        let mut token;
        if lexer.cur_char == '+' {
            token = Token {
                text: lexer.cur_char,
                kind: TokenType::PLUS
            }
        } else if lexer.cur_char == '-' {
            token = Token {
                text: lexer.cur_char,
                kind: TokenType::MINUS
            }
        } else if lexer.cur_char == '*' {
            token = Token {
                text: lexer.cur_char,
                kind: TokenType::ASTERISK
            }
        } else if lexer.cur_char == '/' {
            token = Token {
                text: lexer.cur_char,
                kind: TokenType::SLASH
            }
        } else if lexer.cur_char == '\n' {
            token = Token {
                text: lexer.cur_char,
                kind: TokenType::NEWLINE
            }
        } else if lexer.cur_char == '\0' {
            token = Token {
                text: lexer.cur_char,
                kind: TokenType::EOF
            }
        } else {
            panic!("Unknown token")
        }
        next_char(lexer);

        return Some(token);
    }
}