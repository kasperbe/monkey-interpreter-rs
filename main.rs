#[derive(Debug)]
enum Token {
    LET,
    FUNCTION,
    
    EQUAL,
    NOT_EQUAL,

    IDENT,
    ASSIGN,
    PLUS ,
    MINUS    ,
    BANG     ,
    ASTERISK ,
    SLASH    ,

    LT ,
    GT ,

    COMMA ,
    SEMICOLON ,

    LPAREN ,
    RPAREN ,
    LBRACE ,
    RBRACE ,

    TRUE     ,
    FALSE    ,
    IF       ,
    ELSE     ,
    RETURN   ,

    INT(u32),
    //EOF,
    ILLEGAL,
}


struct Lexer {
    input: Vec<u8>,
    ch: u8,
    position: usize,
    read_pos: usize,
}

impl Lexer {
    pub fn new(input: Vec<u8>) -> Self {
        Self {
            input,
            position: 0,
            read_pos: 0,
            ch: 0,
        }
    }

    pub fn next_token(self: &Self) -> Token {

        let tok = match self.ch {
            b'0'..=b'9' => Token::INT(x.to_digit(10).unwrap()),
            b'=' => Token::ASSIGN,
            b'!' => Token::BANG,
            b'*' => Token::ASTERISK,
            b'+' => Token::PLUS,
            b'-' => Token::MINUS,
            b'/' => Token::SLASH,
            b'<' => Token::LT,
            b'>' => Token::GT,
            b',' => Token::COMMA,
            b';' => Token::SEMICOLON,
            b'(' => Token::LPAREN,
            b')' => Token::RPAREN,
            b'{' => Token::LBRACE,
            b'}' => Token::RBRACE,
            b'a'..=b'z' | b'A'..=b'Z' | b'_' => {
                Token::BANG
            }
            _ => Token::ILLEGAL,
        };

        self.read_char()
    }

    pub fn read_char(self: &Self) {
        if self.ch >= self.input.length() {
            self.ch = 0
        } else {
            self.ch = self.input[self.read_pos]
        }

        self.position = self.read_pos;
        self.read_pos = self.read_pos + 1; 
    }
}

fn lex(input: String) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut raw = input.chars().into_iter();
    while let Some(x) = raw.next() {

        let tok = match x {
            '0'..='9' => Token::INT(x.to_digit(10).unwrap()),
            '=' => Token::ASSIGN,
            '!' => Token::BANG,
            '*' => Token::ASTERISK,
            '+' => Token::PLUS,
            '-' => Token::MINUS,
            '/' => Token::SLASH,
            '<' => Token::LT,
            '>' => Token::GT,
            ',' => Token::COMMA,
            ';' => Token::SEMICOLON,
            '(' => Token::LPAREN,
            ')' => Token::RPAREN,
            '{' => Token::LBRACE,
            '}' => Token::RBRACE,
            'a'..='z' | 'A'..='Z' | '_' => {
                Token::BANG
            }
            _ => Token::ILLEGAL,
        };

        tokens.push(tok);
    }

    return tokens
}

#[test]
fn test_lex() {
    let tokens = lex("0123!=".to_string());

    assert_ne!(false, false);
}


fn main() {
    let tokens = lex("let five = 5;".to_string());
    println!("{:?}", tokens);    
    println!("Hello, world!");
}
