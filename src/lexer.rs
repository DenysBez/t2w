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

    fn skip_whitespace(lexer: &mut Lexer) {
        while lexer.cur_char == ' ' ||  lexer.cur_char == '\t' ||  lexer.cur_char == '\r' {
            next_char(lexer);
        }
    }

    fn skip_comment(lexer: &mut Lexer) {
        if lexer.cur_char == '#' {
           while lexer.cur_char != '\n' {
               next_char(lexer);
           }
        }
    }

    pub fn get_token(lexer: &mut Lexer) -> Option<Token> {
        skip_whitespace(lexer);
        skip_comment(lexer);

        let mut token;
        if lexer.cur_char == '+' {
            token = Token {
                text: lexer.cur_char.to_string(),
                kind: TokenType::PLUS
            }
        } else if lexer.cur_char == '-' {
            token = Token {
                text: lexer.cur_char.to_string(),
                kind: TokenType::MINUS
            }
        } else if lexer.cur_char == '*' {
            token = Token {
                text: lexer.cur_char.to_string(),
                kind: TokenType::ASTERISK
            }
        } else if lexer.cur_char == '/' {
            token = Token {
                text: lexer.cur_char.to_string(),
                kind: TokenType::SLASH
            }
        } else if lexer.cur_char == '\n' {
            token = Token {
                text: lexer.cur_char.to_string(),
                kind: TokenType::NEWLINE
            }
        } else if lexer.cur_char == '\0' {
            token = Token {
                text: lexer.cur_char.to_string(),
                kind: TokenType::EOF
            }
        } else if lexer.cur_char == '=' {
            if peek(lexer) == '=' {
                let lastChar = lexer.cur_char;
                next_char(lexer);
                token = Token {
                    text: lastChar.to_string() + &*lexer.cur_char.to_string(),
                    kind: TokenType::EQEQ
                }
            } else {
                token = Token {
                    text: lexer.cur_char.to_string(),
                    kind: TokenType::EQ
                }
            }
        } else if lexer.cur_char == '>' {
            if peek(lexer) == '=' {
                let lastChar = lexer.cur_char;
                next_char(lexer);
                token = Token {
                    text: lastChar.to_string() + &*lexer.cur_char.to_string(),
                    kind: TokenType::GTEQ
                }
            } else {
                token = Token {
                    text: lexer.cur_char.to_string(),
                    kind: TokenType::GT
                }
            }
        } else if lexer.cur_char == '<' {
            if peek(lexer) == '=' {
                let lastChar = lexer.cur_char;
                next_char(lexer);
                token = Token {
                    text: lastChar.to_string() + &*lexer.cur_char.to_string(),
                    kind: TokenType::LTEQ
                }
            } else {
                token = Token {
                    text: lexer.cur_char.to_string(),
                    kind: TokenType::LT
                }
            }
        } else if lexer.cur_char == '!' {
            if peek(lexer) == '=' {
                let lastChar = lexer.cur_char;
                next_char(lexer);
                token = Token {
                    text: lastChar.to_string() + &*lexer.cur_char.to_string(),
                    kind: TokenType::NOTEQ
                }
            } else {
                panic!("Expected !=, got ! {}",  peek(lexer).to_string());
            }
        } else if lexer.cur_char == '\"' {

            next_char(lexer);
            let start_pos = lexer.cur_pos;

            while lexer.cur_char != '\"' {
                if lexer.cur_char == '\r' || lexer.cur_char == '\n' || lexer.cur_char == '\t' || lexer.cur_char == '\\' || lexer.cur_char == '%' {
                    panic!("illegal character in string")
                }
                next_char(lexer);
            }

            let token_text = lexer.source.chars().skip(start_pos as usize).take(lexer.cur_pos as usize).collect();

            token =  Token {
                text: token_text,
                kind: TokenType::STRING
            }

        } else if lexer.cur_char.is_digit(10) {
            let start_pos = lexer.cur_pos;
            while peek(lexer).is_digit(10) {
                next_char(lexer);
            }

            if peek(lexer) == '.' {
                next_char(lexer);

                if !peek(lexer).is_digit(10) {
                    panic!("Illegal character in number");
                }
                while peek(lexer).is_digit(10) {
                    next_char(lexer);
                }
            }


            let token_text = lexer.source.chars().skip(start_pos as usize).take((lexer.cur_pos + 1) as usize).collect();

            token =  Token {
                text: token_text,
                kind: TokenType::NUMBER
            }
        } else {
            panic!("Unknown token")
        }
        next_char(lexer);

        return Some(token);
    }
}