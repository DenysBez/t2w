pub mod token {
    use std::str::FromStr;

    pub struct Token {
        pub text: String,
        pub kind: TokenType
    }

    #[derive(Debug, PartialEq)]
    pub enum TokenType {
        EOF = -1,
        NEWLINE = 0,
        NUMBER = 1,
        IDENT = 2,
        STRING = 3,
        //keywords
        LABEL = 101,
        GOTO = 102,
        PRINT = 103,
        INPUT = 104,
        LET = 105,
        IF = 106,
        THEN = 107,
        ENDIF = 108,
        WHILE = 109,
        REPEAT = 110,
        ENDWHILE = 111,
        // Operators
        EQ = 201,
        PLUS = 202,
        MINUS = 203,
        ASTERISK = 204,
        SLASH = 205,
        EQEQ = 206,
        NOTEQ = 207,
        LT = 208,
        LTEQ = 209,
        GT = 210,
        GTEQ = 211
    }

    impl TokenType {

        pub fn check_if_keyword(token_text: String) -> Option<TokenType> {
            let result = TokenType::from_str(&token_text);

            if result.is_ok() {

              let token_type = result.unwrap();

                return match token_type {
                    //keywords
                    TokenType::LABEL => Some(TokenType::LABEL),
                    TokenType::GOTO => Some(TokenType::GOTO),
                    TokenType::PRINT => Some(TokenType::PRINT),
                    TokenType::INPUT => Some(TokenType::INPUT),
                    TokenType::LET => Some(TokenType::LET),
                    TokenType::IF => Some(TokenType::IF),
                    TokenType::THEN => Some(TokenType::THEN),
                    TokenType::ENDIF => Some(TokenType::ENDIF),
                    TokenType::WHILE => Some(TokenType::WHILE),
                    TokenType::REPEAT => Some(TokenType::REPEAT),
                    TokenType::ENDWHILE => Some(TokenType::ENDWHILE),
                    _ => None
                }

            } else {
                None
            }
        }
    }

    impl FromStr for TokenType {
        type Err = ();

        fn from_str(input: &str) -> Result<TokenType, Self::Err> {
            match input {
                "EOF"  => Ok(TokenType::EOF),
                "NEWLINE" => Ok(TokenType::NEWLINE),
                "NUMBER" => Ok(TokenType::NUMBER),
                "IDENT" => Ok(TokenType::IDENT),
                "STRING" => Ok(TokenType::STRING),

                //keywords
                "LABEL" => Ok(TokenType::LABEL),
                "GOTO" => Ok(TokenType::GOTO),
                "PRINT" => Ok(TokenType::PRINT),
                "INPUT" => Ok(TokenType::INPUT),
                "LET" => Ok(TokenType::LET),
                "IF" => Ok(TokenType::IF),
                "THEN" => Ok(TokenType::THEN),
                "ENDIF" => Ok(TokenType::ENDIF),
                "WHILE" => Ok(TokenType::WHILE),
                "REPEAT" => Ok(TokenType::REPEAT),
                "ENDWHILE" => Ok(TokenType::ENDWHILE),

                // Operators
                "EQ" => Ok(TokenType::EQ),
                "PLUS" => Ok(TokenType::PLUS),
                "MINUS" => Ok(TokenType::MINUS),
                "ASTERISK" => Ok(TokenType::ASTERISK),
                "SLASH" => Ok(TokenType::SLASH),
                "EQEQ" => Ok(TokenType::EQEQ),
                "NOTEQ" => Ok(TokenType::NOTEQ),
                "LT" => Ok(TokenType::LT),
                "LTEQ" => Ok(TokenType::LTEQ),
                "GT" => Ok(TokenType::GT),
                "GTEQ" => Ok(TokenType::GTEQ),
                _      => Err(()),
            }
        }

    }
}