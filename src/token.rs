use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct Token {
    pub text: String,
    pub kind: TokenType
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenType {
    EOF,
    NEWLINE,
    NUMBER,
    IDENT,
    STRING,
    //keywords
    LABEL,
    GOTO,
    PRINT,
    INPUT,
    LET,
    IF,
    THEN,
    ENDIF,
    WHILE,
    REPEAT,
    ENDWHILE,
    // Operators
    EQ,
    PLUS,
    MINUS,
    ASTERISK,
    SLASH,
    EQEQ,
    NOTEQ,
    LT,
    LTEQ,
    GT,
    GTEQ
}

impl TokenType {

    pub fn check_if_keyword(token_text: String) -> Option<TokenType> {
        let result = TokenType::from_str(&token_text);

        if result.is_ok() {

          let token_type = result.unwrap();

            match token_type {
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
            _      => Err(())
        }
    }

}
