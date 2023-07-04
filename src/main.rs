use std::io::{self, BufRead};
use std::io::Result;


#[derive(Debug, PartialEq)]
enum Token {
    LET,
    FUNCTION,
    
    EQUAL,
    NOT_EQUAL,

    IDENT(String),
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

    INT(String),
    EOF,
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
        let mut s = Self{
            input,
            position: 0,
            read_pos: 0,
            ch: 0,
        };

        s.read_char();
        return s;

    }

    pub fn next_token(&mut self) -> Result<Token> {
        self.skip_whitespace();

        let tok = match self.ch {
            0 => Token::EOF,
            b'0'..=b'9' => {
                let literal = self.read_digit();

                return Ok(Token::INT(literal))
            },
            b'=' => {
                if self.peek() == b'=' {
                    self.read_char();
                    Token::EQUAL
                } else {
                    Token::ASSIGN
                }
            },
            b'!' => {
                if self.peek() == b'=' {
                    self.read_char();
                    Token::NOT_EQUAL
                } else {
                    Token::BANG
                }
            },
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
                let literal = self.read_ident();

                return Ok(match literal.as_str() {
                    "fn" => Token::FUNCTION,
                    "let" => Token::LET,
                    "if" => Token::IF,
                    "return" => Token::RETURN,
                    "else" => Token::ELSE,
                    "true" => Token::TRUE,
                    "false" => Token::FALSE,
                    _ => Token::IDENT(literal),
                })
            }
            _ => Token::ILLEGAL,
        };

        self.read_char();
        return Ok(tok);
    }

    pub fn read_ident(&mut self) -> String {
        let pos = self.position;
        while self.ch.is_ascii_alphabetic() || self.ch == b'_' {
            self.read_char();
        }

        return String::from_utf8_lossy(&self.input[pos..self.position]).to_string();
    }

    pub fn read_digit(&mut self) -> String {
        let pos = self.position;
        while self.ch.is_ascii_digit() {
            self.read_char();
        }

        return String::from_utf8_lossy(&self.input[pos..self.position]).to_string();
    }

    pub fn read_char(&mut self) {
        if self.read_pos >= self.input.len() {
            self.ch = 0
        } else {
            self.ch = self.input[self.read_pos]
        }

        self.position = self.read_pos;
        self.read_pos = self.read_pos + 1; 
    }

    pub fn skip_whitespace(&mut self) {
        while self.ch.is_ascii_whitespace() {
            self.read_char();
        }
    }

    pub fn peek(&mut self) -> u8 {
        if self.read_pos >= self.input.len() {
            0
        } else {
            self.input[self.read_pos]
        }
    }
}

#[test]
fn test_lex() {
    let input = "let five = 5; 
let ten = 10;

let add = fn(x, y) {
  x + y;
};

let result = add(five, ten);
!-/*5;
5 < 10 > 5;

if (5 < 10) {
    return true;
} else {
    return false;
}

10 == 10;
10 != 9;
".to_string();
    let tests = vec![
        Token::LET, Token::IDENT("five".to_string()), Token::ASSIGN, Token::INT("5".to_string()), Token::SEMICOLON,
    
        Token::LET, Token::IDENT("ten".to_string()), Token::ASSIGN, Token::INT("10".to_string()), Token::SEMICOLON,


        Token::LET, Token::IDENT("add".to_string()), Token::ASSIGN, Token::FUNCTION, Token::LPAREN, Token::IDENT("x".to_string()), Token::COMMA,
        Token::IDENT("y".to_string()), Token::RPAREN, Token::LBRACE, Token::IDENT("x".to_string()), Token::PLUS, Token::IDENT("y".to_string()),
        Token::SEMICOLON, Token::RBRACE, Token::SEMICOLON,

        Token::LET, Token::IDENT("result".to_string()), Token::ASSIGN, Token::IDENT("add".to_string()), Token::LPAREN, Token::IDENT("five".to_string()),
        Token::COMMA, Token::IDENT("ten".to_string()), Token::RPAREN, Token::SEMICOLON,

        Token::BANG, Token::MINUS, Token::SLASH, Token::ASTERISK, Token::INT("5".to_string()),Token::SEMICOLON,

        Token::INT("5".to_string()), Token::LT, Token::INT("10".to_string()), Token::GT, Token::INT("5".to_string()),Token::SEMICOLON,

        Token::IF, Token::LPAREN, Token::INT("5".to_string()), Token::LT, Token::INT("10".to_string()), Token::RPAREN, Token::LBRACE,
        Token::RETURN, Token::TRUE, Token::SEMICOLON,
        Token::RBRACE, Token::ELSE, Token::LBRACE,
        Token::RETURN, Token::FALSE, Token::SEMICOLON,
        Token::RBRACE,

        Token::INT("10".to_string()), Token::EQUAL, Token::INT("10".to_string()), Token::SEMICOLON,
        Token::INT("10".to_string()), Token::NOT_EQUAL, Token::INT("9".to_string()), Token::SEMICOLON,
    ];

    let mut lexer = Lexer::new(input.as_bytes().to_vec());

    for t in tests {
        if let Ok(tok) = lexer.next_token() {
            print!("Token: {:?}\n", tok);
            assert_eq!(t, tok);
        }
    }
}


fn main() {
    let stdin = io::stdin();
    stdin.lines().for_each(|line| {
        if let Ok(line) = line {
            let mut lexer = Lexer::new(line.as_bytes().to_vec());
            print!("{:?}\n", line);
            while let Ok(token) = lexer.next_token() {
                print!("{:?}\n", token);

                if token == Token::EOF {
                    break;
                }
            }
        }


    });
}
