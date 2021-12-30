pub mod lexer {

    pub struct Lexer {
        pub source:  String,
        pub cur_char: char,
        pub cur_pos: isize
    }

    pub fn next_char(lexer: &mut Lexer) {
        println!("nextChar called");

        lexer.cur_pos += 1;
        if lexer.cur_pos >= lexer.source.len() as isize {
            lexer.cur_char = '\0' //EOF
        } else {
            lexer.cur_char = lexer.source.chars().nth(lexer.cur_pos as usize).unwrap();
        }
    }

    fn peek(lexer: &Lexer) -> char {
        println!("peek called");

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

    fn get_token(lexer: &Lexer) {
        println!("getToken called")
    }
}