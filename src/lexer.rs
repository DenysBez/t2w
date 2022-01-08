use crate::token::{Token, TokenType};

pub struct Lexer {
    pub source:  String,
    pub cur_char: char,
    pub cur_pos: isize
}

impl Lexer {
    pub fn next_char(&mut self) {
        self.cur_pos += 1;
        if self.cur_pos >= self.source.len() as isize {
            self.cur_char = '\0' //EOF
        } else {
            self.cur_char = self.source.chars().nth(self.cur_pos as usize).unwrap();
        }
    }

    pub fn peek(&mut self) -> char {
        if self.cur_pos + 1 >= self.source.len() as isize {
            return '\0';
        }

        return self.source.chars().nth((self.cur_pos + 1) as usize).unwrap();
    }

    fn skip_whitespace(&mut self) {
        while self.cur_char == ' ' ||  self.cur_char == '\t' ||  self.cur_char == '\r' {
            self.next_char();
        }
    }

    fn skip_comment(&mut self) {
        if self.cur_char == '#' {
            while self.cur_char != '\n' {
                self.next_char();
            }
        }
    }

    pub fn get_token(&mut self) -> Option<Token> {
        self.skip_whitespace();
        self.skip_comment();

        let token;
        if self.cur_char == '+' {
            token = Token {
                text: self.cur_char.to_string(),
                kind: TokenType::PLUS
            }
        } else if self.cur_char == '-' {
            token = Token {
                text: self.cur_char.to_string(),
                kind: TokenType::MINUS
            }
        } else if self.cur_char == '*' {
            token = Token {
                text: self.cur_char.to_string(),
                kind: TokenType::ASTERISK
            }
        } else if self.cur_char == '/' {
            token = Token {
                text: self.cur_char.to_string(),
                kind: TokenType::SLASH
            }
        } else if self.cur_char == '\n' {
            token = Token {
                text: self.cur_char.to_string(),
                kind: TokenType::NEWLINE
            }
        } else if self.cur_char == '\0' {
            token = Token {
                text: self.cur_char.to_string(),
                kind: TokenType::EOF
            }
        } else if self.cur_char == '=' {
            if self.peek() == '=' {
                let last_char = self.cur_char;
                self.next_char();
                token = Token {
                    text: last_char.to_string() + &*self.cur_char.to_string(),
                    kind: TokenType::EQEQ
                }
            } else {
                token = Token {
                    text: self.cur_char.to_string(),
                    kind: TokenType::EQ
                }
            }
        } else if self.cur_char == '>' {
            if self.peek() == '=' {
                let last_char = self.cur_char;
                self.next_char();
                token = Token {
                    text: last_char.to_string() + &*self.cur_char.to_string(),
                    kind: TokenType::GTEQ
                }
            } else {
                token = Token {
                    text: self.cur_char.to_string(),
                    kind: TokenType::GT
                }
            }
        } else if self.cur_char == '<' {
            if self.peek() == '=' {
                let last_char = self.cur_char;
                self.next_char();
                token = Token {
                    text: last_char.to_string() + &*self.cur_char.to_string(),
                    kind: TokenType::LTEQ
                }
            } else {
                token = Token {
                    text: self.cur_char.to_string(),
                    kind: TokenType::LT
                }
            }
        } else if self.cur_char == '!' {
            if self.peek() == '=' {
                let last_char = self.cur_char;
                self.next_char();
                token = Token {
                    text: last_char.to_string() + &*self.cur_char.to_string(),
                    kind: TokenType::NOTEQ
                }
            } else {
                panic!("Expected !=, got ! {}",  self.peek().to_string());
            }
        } else if self.cur_char == '\"' {
            self.next_char();
            let start_pos = self.cur_pos;

            while self.cur_char != '\"' {
                if self.cur_char == '\r' || self.cur_char == '\n' || self.cur_char == '\t' || self.cur_char == '\\' || self.cur_char == '%' {
                    panic!("illegal character in string {:?}", self.cur_char)
                }
                self.next_char();
            }

            let token_text = self.source.chars().skip(start_pos as usize).take((self.cur_pos - start_pos) as usize).collect();

            token =  Token {
                text: token_text,
                kind: TokenType::STRING
            }
        } else if self.cur_char.is_digit(10) {
            let start_pos = self.cur_pos;
            while self.peek().is_digit(10) {
                self.next_char();
            }

            if self.peek() == '.' {
                self.next_char();

                if !self.peek().is_digit(10) {
                    panic!("Illegal character in number");
                }
                while self.peek().is_digit(10) {
                    self.next_char();
                }
            }


            let token_text = self.source.chars().skip(start_pos as usize).take((self.cur_pos + 1 - start_pos) as usize).collect();

            token =  Token {
                text: token_text,
                kind: TokenType::NUMBER
            }
        } else if self.cur_char.is_alphabetic() {
            let start_pos = self.cur_pos;
            while self.peek().is_alphanumeric() {
                self.next_char();
            }

            let token_text = self.source.chars().skip(start_pos as usize).take((self.cur_pos + 1 - start_pos) as usize).collect();

            let keyword = TokenType::check_if_keyword(String::from(&token_text));

            match keyword {
                Some(ref x) => {
                    token =  Token {
                        text: token_text,
                        kind: keyword.unwrap()
                    }
                },
                None =>  token =  Token {
                    text: token_text,
                    kind: TokenType::IDENT
                }
            }

        } else {
            panic!("Unknown token")
        }
        self.next_char();

        return Some(token);
    }
}


