pub mod parser {
    use crate::{get_token, Lexer, Symbol_Table, TokenType};
    use crate::token::token::Token;

    pub struct Parser {
        pub lexer: Lexer,
        pub symbol_table: Symbol_Table,
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

            //Since some newlines are required in our grammar, need to skip the excess.
            while self.check_token(&TokenType::NEWLINE) {
                self.statement();
            }

            while !self.check_token(&TokenType::EOF) {
                self.statement();
            }

            //Check that each label referenced in a GOTO is declared.

            for label in &self.symbol_table.labels_goto_ed {
                if !self.symbol_table.labels_declared.contains(label) {
                    panic!("Attempting to GOTO to undeclared label: {:?}", label);
                }
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
            //"IF" comparison "THEN" {statement} "ENDIF"
            else if self.check_token(&TokenType::IF) {
                println!("STATEMENT IF");
                self.next_token();
                self.comparison();

                self.match_function(&TokenType::THEN);
                self.nl();

                //Zero or more statements in the body.
                while !self.check_token(&TokenType::ENDIF) {
                    self.statement();
                }

                self.match_function(&TokenType::ENDIF);
            }
            //"WHILE" comparison "REPEAT" {statement} "ENDWHILE"
            else if self.check_token(&TokenType::WHILE) {
                println!("STATEMENT-WHILE");
                self.next_token();
                self.comparison();

                self.match_function(&TokenType::REPEAT);
                self.nl();

                //Zero or more statements in the body.
                while !self.check_token(&TokenType::ENDWHILE) {
                    self.statement();
                }

                self.match_function(&TokenType::ENDWHILE);
            }
            //"LABEL" ident
            else if self.check_token(&TokenType::LABEL) {
                println!("STATEMENT-LABEL");
                self.next_token();

                //Make sure this label doesn't already exist
                if self.symbol_table.labels_declared.contains(&self.cur_token.as_ref().unwrap().text) {
                    panic!("Label already exists: {:?}", self.cur_token.as_ref().unwrap().text);
                }
                self.symbol_table.labels_declared.insert(self.cur_token.as_ref().unwrap().text.clone());

                self.match_function(&TokenType::IDENT);
            }
            //"GOTO" ident
            else if self.check_token(&TokenType::GOTO) {
                println!("STATEMENT-GOTO");
                self.next_token();
                self.symbol_table.labels_goto_ed.insert(self.cur_token.as_ref().unwrap().text.clone());
                self.match_function(&TokenType::IDENT);
            }
            //"LET" ident = expression
            else if self.check_token(&TokenType::LET) {
                println!("STATEMENT-LET");
                self.next_token();

                //Check if ident exists in symbol table. If not, declare it.
                if !self.symbol_table.symbol_table.contains(&self.cur_token.as_ref().unwrap().text) {
                    self.symbol_table.symbol_table.insert(self.cur_token.as_ref().unwrap().text.clone());
                }

                self.match_function(&TokenType::IDENT);
                self.match_function(&TokenType::EQ);
                self.expression();
            }
            //"INPUT" ident
            else if self.check_token(&TokenType::INPUT) {
                println!("STATEMENT-INPUT");
                self.next_token();

                //Check if ident exists in symbol table. If not, declare it.
                if !self.symbol_table.symbol_table.contains(&self.cur_token.as_ref().unwrap().text) {
                    self.symbol_table.symbol_table.insert(self.cur_token.as_ref().unwrap().text.clone());
                }

                self.match_function(&TokenType::IDENT);
            } else {
                panic!("This is not a valid statement {:?}", self.cur_token.as_ref().unwrap().text);
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

        //expression ::= term {( "-" | "+" ) term}
        fn expression(&mut self) {
            println!("EXPRESSION");


            self.term();

            //Can have 0 or more +/- and expressions.
            while self.check_token(&TokenType::PLUS)
                || self.check_token(&TokenType::MINUS) {
                self.next_token();
                self.term();
            }
        }

        //term ::= unary {( "/" | "*" ) unary}
        fn term(&mut self) {
            println!("TERM");

            self.unary();

            while self.check_token(&TokenType::ASTERISK)
                || self.check_token(&TokenType::SLASH) {
                self.next_token();
                self.unary();
            }
        }

        //unary ::= ["+" | "-"] primary
        fn unary(&mut self) {
            println!("UNARY");

            //Optional unary +/-

            if self.check_token(&TokenType::PLUS) || self.check_token(&TokenType::MINUS) {
                self.next_token();
            }
            self.primary();
        }

        //comparison ::= expression (("==" | "!=" | ">" | ">=" | "<" | "<=") expression)+
        fn comparison(&mut self) {
            println!("COMPARISON");

            self.expression();

            //Must be at least one comparison operator and another expression

            if self.isComparisonOperator() {
                self.next_token();
                self.expression();
            } else {
                panic!("Expected comparison operator at {:?}", self.cur_token.as_ref().unwrap().text);
            }


            //Can have 0 or more comparison operator and expressions
            while self.isComparisonOperator() {
                self.next_token();
                self.expression();
            }

        }
        fn isComparisonOperator(&self) -> bool {
            return self.check_token(&TokenType::GT) ||
                self.check_token(&TokenType::GTEQ)  ||
                self.check_token(&TokenType::LT)  ||
                self.check_token(&TokenType::LTEQ)  ||
                self.check_token(&TokenType::EQEQ) ||
                self.check_token(&TokenType::NOTEQ);
        }

        //primary ::= number | ident
        fn primary(&mut self) {
            println!("PRIMARY {:?}", self.cur_token.as_ref().unwrap().text);

            if self.check_token(&TokenType::NUMBER) {
                self.next_token();
            } else if self.check_token(&TokenType::IDENT){
                //Ensure the variable already exists
                if !self.symbol_table.symbol_table.contains(&self.cur_token.as_ref().unwrap().text) {
                    panic!("Referencing variable before assignment:: {:?}", self.cur_token.as_ref().unwrap().text);
                }
                self.next_token();
            } else {
                panic!("Unexpected token at {:?}", self.cur_token.as_ref().unwrap().text);
            }
        }
    }
}