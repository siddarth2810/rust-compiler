use std::{i32, usize};

pub struct Lexer {
    source: String,
    cur_pos: i32,
    cur_char: char,
}

impl Lexer {
    pub fn new(source: String) -> Self {
        let mut lexer = Lexer {
            source: source + "\n",
            cur_char: ' ',
            cur_pos: -1,
        };
        lexer.next_char();
        lexer
    }

    pub fn next_char(&mut self) {
        self.cur_pos += 1;

        if self.cur_pos >= self.source.len() as i32 {
            self.cur_char = '\0';
        } else {
            self.cur_char = self.source.chars().nth(self.cur_pos as usize).unwrap();
        }
    }

    pub fn peek(&self) -> char {
        if self.cur_pos + 1 >= self.source.len() as i32 {
            return '\0';
        }
        self.source
            .chars()
            .nth((self.cur_pos + 1) as usize)
            .unwrap()
    }

    pub fn abort(&self) {
        println!("Unexpected character: {}", self.cur_char);
        std::process::exit(1);
    }

    pub fn get_token(&mut self) -> Token {
        self.skip_white_space();
        self.skip_comment();
        let token = match self.cur_char {
            '+' => Token::new(self.cur_char.to_string(), TokenType::PLUS),
            '-' => Token::new(self.cur_char.to_string(), TokenType::MINUS),
            '*' => Token::new(self.cur_char.to_string(), TokenType::ASTERISK),
            '/' => Token::new(self.cur_char.to_string(), TokenType::SLASH),
            '\n' => Token::new(self.cur_char.to_string(), TokenType::NEWLINE),
            '\0' => Token::new(self.cur_char.to_string(), TokenType::EOF),
            '=' => {
                if self.peek() == '=' {
                    self.next_char(); // Consume the second '='
                    Token::new("==".to_string(), TokenType::EQEQ)
                } else {
                    Token::new(self.cur_char.to_string(), TokenType::EQ)
                }
            }
            '>' => {
                if self.peek() == '=' {
                    self.next_char();
                    Token::new(">=".to_string(), TokenType::GTEQ)
                } else {
                    Token::new(self.cur_char.to_string(), TokenType::GT)
                }
            }
            '<' => {
                if self.peek() == '=' {
                    self.next_char();
                    Token::new("<=".to_string(), TokenType::LTEQ)
                } else {
                    Token::new(self.cur_char.to_string(), TokenType::LT)
                }
            }
            '!' => {
                if self.peek() == '=' {
                    self.next_char();
                    Token::new("!=".to_string(), TokenType::NOTEQ)
                } else {
                    Token::new(self.cur_char.to_string(), TokenType::UNKNOWN)
                }
            }
            '\"' => {
                self.next_char();
                let start_pos = self.cur_pos;
                while self.cur_char != '\"' {
                    if self.cur_char == '\n'
                        || self.cur_char == '\t'
                        || self.cur_char == '\r'
                        || self.cur_char == '\\'
                        || self.cur_char == '%'
                    {
                        self.abort();
                    }
                    self.next_char();
                }
                let tok_text = self.source[start_pos as usize..self.cur_pos as usize].to_string();
                Token::new(tok_text, TokenType::STRING)
            }
            //check if the character is a number
            _ if self.cur_char.is_digit(10) => {
                let start_pos = self.cur_pos;
                while self.peek().is_digit(10) {
                    self.next_char();
                }
                if self.peek() == '.' {
                    self.next_char();
                    if !self.peek().is_digit(10) {
                        self.abort();
                    }
                    while self.peek().is_digit(10) {
                        self.next_char();
                    }
                }
                let tok_text =
                    self.source[start_pos as usize..(self.cur_pos + 1) as usize].to_string();
                Token::new(tok_text, TokenType::NUMBER)
            }
            _ if self.cur_char.is_alphabetic() => {
                let start_pos = self.cur_pos;
                while self.peek().is_alphanumeric() {
                    self.next_char();
                }
                let tok_text =
                    self.source[start_pos as usize..(self.cur_pos + 1) as usize].to_string();
                let keyword = match tok_text.as_str() {
                    "LABEL" => TokenType::LABEL,
                    "GOTO" => TokenType::GOTO,
                    "PRINT" => TokenType::PRINT,
                    "INPUT" => TokenType::INPUT,
                    "LET" => TokenType::LET,
                    "IF" => TokenType::IF,
                    "THEN" => TokenType::THEN,
                    "ENDIF" => TokenType::ENDIF,
                    "WHILE" => TokenType::WHILE,
                    "REPEAT" => TokenType::REPEAT,
                    "ENDWHILE" => TokenType::ENDWHILE,
                    _ => TokenType::IDENT,
                };
                Token::new(tok_text, keyword)
            }

            _ => Token::new(self.cur_char.to_string(), TokenType::UNKNOWN),
        };
        self.next_char();
        token
    }

    pub fn skip_white_space(&mut self) {
        while self.cur_char == ' ' || self.cur_char == '\t' || self.cur_char == '\r' {
            self.next_char();
        }
    }

    pub fn skip_comment(&mut self) {
        if self.cur_char == '#' {
            while self.cur_char != '\n' {
                self.next_char();
            }
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub text: String,
    pub kind: TokenType,
}

impl Token {
    pub fn new(text: String, kind: TokenType) -> Self {
        Token { text, kind }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    EOF,
    NEWLINE,
    NUMBER,
    IDENT,
    STRING,
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
    GTEQ,
    UNKNOWN,
}
