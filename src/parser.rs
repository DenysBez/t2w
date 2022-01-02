pub mod parser {
    use crate::{get_token, Lexer, TokenType};
    use crate::token::token::Token;

    pub struct Parser {
        pub lexer: Lexer,
        pub cur_token: Option<Token>,
        pub peek_token: Option<Token>
    }


    impl Parser {
        fn run(lexer: Lexer) {
        }

        fn check_token(&self, token_type: &TokenType) -> bool {
            return if self.cur_token.is_some() {
                self.cur_token.as_ref().unwrap().kind.eq(token_type)
            } else {
                return false
            }
        }

        fn check_peek(&self, token_type: TokenType) -> bool {
            return if self.peek_token.is_some() {
                self.peek_token.as_ref().unwrap().kind.eq(&token_type)
            } else {
                return false
            }
        }

        fn match_function(&mut self, token_type: &TokenType) {
            if !self.check_token(token_type) {

                let mut val ;
                if self.cur_token.is_some() {
                    val = self.cur_token.as_ref().unwrap();

                    panic!("Expected one {:?} got another: {:?}", token_type,  val.text);
                } else {

                    panic!("Expected one {:?} got another empty", token_type);
                }
            }
            self.next_token();
        }

       pub fn next_token(&mut self) {
           self.cur_token = self.peek_token.clone();
           self.peek_token = Some(get_token(&mut self.lexer).unwrap());

           //DBG println!("cur_token current {:?} peek: {:?}", self.cur_token,  self.peek_token);

       }

        fn abort(msg: String) {
            panic!("Error")
        }


        //Rules

        //program ::= {statement}
        pub fn program(&mut self) {
            println!("PROGRAM");

            while !self.check_token(&TokenType::EOF) {
                self.statement();
            }
        }

        fn statement(&mut self){
            // "PRINT" (expression | string)
            if self.check_token(&TokenType::PRINT) {
                println!("STATEMENT PRINT");
                self.next_token();

                if self.check_token(&TokenType::STRING) {
                    self.next_token();
                } else {
                    self.expression();
                }
            }
            self.nl();
        }

        //nl ::= '\n'+
        fn nl(&mut self) {
            println!("NEWLINE");

            self.match_function(&TokenType::NEWLINE);

            while self.check_token(&TokenType::NEWLINE) {
                self.next_token();
            }
        }
        fn expression(&self) {
            todo!()
        }
    }
}