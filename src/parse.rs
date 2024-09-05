use std::fmt;

use crate::lex::Lexer;
use crate::lex::Token;
use crate::lex::TokenType;

pub struct Parser {
    lexer: Lexer,
    cur_token: Token,
    peek_token: Token,
}

#[derive(Debug)]
pub struct ParseError(String);

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ParseError: {}", self.0)
    }
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        let mut parser = Parser {
            lexer,
            cur_token: Token::new("".to_string(), TokenType::EOF),
            peek_token: Token::new("".to_string(), TokenType::EOF),
        };
        parser.next_token();
        parser.next_token();
        parser
    }
    pub fn check_token(&self, kind: &TokenType) -> bool {
        &self.cur_token.kind == kind
    }
    pub fn check_peek(&self, kind: &TokenType) -> bool {
        &self.peek_token.kind == kind
    }
    pub fn match_token(&mut self, kind: &TokenType) -> Result<(), ParseError> {
        if !self.check_token(kind) {
            return Err(ParseError(format!(
                "Expected token {:?}, got {:?}",
                kind, self.cur_token.kind
            )));
        }
        self.next_token();
        Ok(())
    }
    pub fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.lexer.get_token();
    }

    //PRODUCTION RULES
    pub fn program(&mut self) -> Result<(), ParseError> {
        println!("PROGRAM");

        while self.check_token(&TokenType::NEWLINE) {
            self.next_token();
        }

        while !self.check_token(&TokenType::EOF) {
            self.statement()?;
        }
        Ok(())
    }

    pub fn statement(&mut self) -> Result<(), ParseError> {
        if self.check_token(&TokenType::PRINT) {
            println!("STATEMENT-PRINT");
            self.next_token();

            if self.check_token(&TokenType::STRING) {
                self.next_token();
            } else {
                //Expect an expression
                //self.expression();
            }
        }
        //"IF" comparison "THEN" {statement} "ENDIF"
        else if self.check_token(&TokenType::IF) {
            println!("STATEMENT-IF");
            self.next_token();
            //self.comparison();
            self.match_token(&TokenType::THEN)?;
            self.nl()?;

            while !self.check_token(&TokenType::ENDIF) {
                self.statement()?;
            }
            self.match_token(&TokenType::ENDIF)?;
        }
        // "WHILE" comparison "REPEAT" {statement} "ENDWHILE"
        else if self.check_token(&TokenType::WHILE) {
            println!("STATEMENT-WHILE");
            self.next_token();
            //self.comparison();
            self.match_token(&TokenType::REPEAT)?;
            self.nl()?;

            while !self.check_token(&TokenType::ENDWHILE) {
                self.statement()?;
            }
            self.match_token(&TokenType::ENDWHILE)?;
        }
        //  "LABEL" ident
        else if self.check_token(&TokenType::LABEL) {
            println!("STATEMENT-LABEL");
            self.next_token();
            self.match_token(&TokenType::IDENT)?;
        }
        // "GOTO" ident
        else if self.check_token(&TokenType::GOTO) {
            println!("STATEMENT-GOTO");
            self.next_token();
            self.match_token(&TokenType::IDENT)?;
        }
        //   # "LET" ident "=" expression
        else if self.check_token(&TokenType::LET) {
            println!("STATEMENT-LABEL");
            self.next_token();
            self.match_token(&TokenType::IDENT)?;
            self.match_token(&TokenType::EQ)?;
            //self.expression();
        }
        // "INPUT" ident
        else if self.check_token(&TokenType::INPUT) {
            println!("STATEMENT-INPUT");
            self.next_token();
            self.match_token(&TokenType::IDENT)?;
        } else {
            return Err(ParseError(format!(
                "Unexpected token: {:?}",
                self.cur_token.text
            )));
        }

        self.nl()?;
        Ok(())
    }

    pub fn nl(&mut self) -> Result<(), ParseError> {
        println!("NEWLINE");
        self.match_token(&TokenType::NEWLINE)?;
        while self.check_token(&TokenType::NEWLINE) {
            self.next_token();
        }
        Ok(())
    }
}
