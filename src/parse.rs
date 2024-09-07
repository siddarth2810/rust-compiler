use std::collections::HashSet;
use std::fmt;

use crate::emit::Emitter;
use crate::lex::Lexer;
use crate::lex::Token;
use crate::lex::TokenType;

pub struct Parser {
    pub lexer: Lexer,
    pub emitter: Emitter,
    cur_token: Token,
    peek_token: Token,
    symbols: HashSet<String>,
    labels_declared: HashSet<String>,
    labels_gotoed: HashSet<String>,
}

#[derive(Debug)]
pub struct ParseError(String);

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ParseError: {}", self.0)
    }
}

impl Parser {
    pub fn new(lexer: Lexer, emitter: Emitter) -> Self {
        let mut parser = Parser {
            lexer,
            emitter,
            cur_token: Token::new("".to_string(), TokenType::EOF),
            peek_token: Token::new("".to_string(), TokenType::EOF),
            symbols: HashSet::new(),
            labels_declared: HashSet::new(),
            labels_gotoed: HashSet::new(),
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
        self.emitter.header_line("#include <stdio.h>");
        self.emitter.header_line("int main(void){");

        while self.check_token(&TokenType::NEWLINE) {
            self.next_token();
        }

        while !self.check_token(&TokenType::EOF) {
            self.statement()?;
        }

        //Wrap things
        self.emitter.emit_line("return 0;");
        self.emitter.emit_line("}");

        for label in self.labels_gotoed.clone() {
            if !self.labels_declared.contains(&label) {
                println!("Attempting to GOTO to undecalred label {:?} ", label);
                std::process::exit(1);
            }
        }

        Ok(())
    }

    pub fn statement(&mut self) -> Result<(), ParseError> {
        if self.check_token(&TokenType::PRINT) {
            self.next_token();

            if self.check_token(&TokenType::STRING) {
                self.emitter
                    .emit_line(&format!("printf(\"{}\\n\");", self.cur_token.text));
                self.next_token();
            } else {
                //Expect an expression
                self.emitter.emit("printf(\"%.2f\\n\", (float)(");
                self.expression()?;
                self.emitter.emit_line("));");
            }
        }
        //"IF" comparison "THEN" {statement} "ENDIF"
        else if self.check_token(&TokenType::IF) {
            self.next_token();
            self.emitter.emit("if(");
            self.comparison()?;

            self.match_token(&TokenType::THEN)?;
            self.nl()?;
            self.emitter.emit_line("){");

            while !self.check_token(&TokenType::ENDIF) {
                self.statement()?;
            }
            self.match_token(&TokenType::ENDIF)?;
            self.emitter.emit_line("}");
        }
        // "WHILE" comparison "REPEAT" {statement} "ENDWHILE"
        else if self.check_token(&TokenType::WHILE) {
            self.next_token();
            self.emitter.emit("while(");
            self.comparison()?; // Emit the condition

            self.match_token(&TokenType::REPEAT)?;
            self.nl()?;
            self.emitter.emit_line("){");

            while !self.check_token(&TokenType::ENDWHILE) {
                self.statement()?;
            }
            self.match_token(&TokenType::ENDWHILE)?;
            self.emitter.emit_line("}");
        }
        //  "LABEL" ident
        else if self.check_token(&TokenType::LABEL) {
            self.next_token();

            if self.labels_declared.contains(&self.cur_token.text) {
                return Err(ParseError(format!(
                    "Label already exists : {:?}",
                    self.cur_token.text
                )));
            }
            self.labels_declared.insert(self.cur_token.text.clone());
            self.emitter.emit_line(&format!("{}:", self.cur_token.text));
            self.match_token(&TokenType::IDENT)?;
        }
        // "GOTO" ident
        else if self.check_token(&TokenType::GOTO) {
            self.next_token();
            self.labels_gotoed.insert(self.cur_token.text.clone());
            //emit the goto ident;
            self.emitter
                .emit_line(&format!("goto {};", self.cur_token.text));
            self.match_token(&TokenType::IDENT)?;
        }
        //   # "LET" ident "=" expression
        else if self.check_token(&TokenType::LET) {
            self.next_token();

            if !self.symbols.contains(&self.cur_token.text) {
                self.symbols.insert(self.cur_token.text.clone());
                self.emitter
                    .header_line(&format!("float {};", self.cur_token.text));
            }
            self.emitter.emit(&format!("{} = ", self.cur_token.text));
            self.match_token(&TokenType::IDENT)?;
            self.match_token(&TokenType::EQ)?;

            self.expression()?;
            self.emitter.emit_line(";");
        }
        // "INPUT" ident
        else if self.check_token(&TokenType::INPUT) {
            self.next_token();
            if !self.symbols.contains(&self.cur_token.text) {
                self.symbols.insert(self.cur_token.text.clone());
                self.emitter
                    .header_line(&format!("float {};", self.cur_token.text));
            }

            //emit scanf and verify
            self.emitter.emit_line(&format!(
                "if(0 == scanf(\"%f\", &{})) {{",
                self.cur_token.text
            ));
            self.emitter
                .emit_line(&format!("{} = 0;", self.cur_token.text));
            self.emitter.emit("scanf(\"%");
            self.emitter.emit_line("*s\");");
            self.emitter.emit_line("}");
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
        self.match_token(&TokenType::NEWLINE)?;
        while self.check_token(&TokenType::NEWLINE) {
            self.next_token();
        }
        Ok(())
    }

    pub fn comparison(&mut self) -> Result<(), ParseError> {
        self.expression()?;
        if self.is_comparison_operator() {
            self.emitter.emit(&self.cur_token.text);
            self.next_token();
            self.expression()?;
        } else {
            return Err(ParseError(format!(
                "expected comparsion operator at : {:?}",
                self.cur_token.text
            )));
        }
        while self.is_comparison_operator() {
            self.emitter.emit(&self.cur_token.text);
            self.next_token();
            self.expression()?;
        }
        Ok(())
    }

    pub fn is_comparison_operator(&self) -> bool {
        return self.check_token(&TokenType::GT)
            || self.check_token(&TokenType::GTEQ)
            || self.check_token(&TokenType::LT)
            || self.check_token(&TokenType::LTEQ)
            || self.check_token(&TokenType::EQEQ)
            || self.check_token(&TokenType::NOTEQ);
    }

    pub fn expression(&mut self) -> Result<(), ParseError> {
        self.term()?;

        while self.check_token(&TokenType::PLUS) || self.check_token(&TokenType::MINUS) {
            self.emitter.emit(&self.cur_token.text);
            self.next_token();
            self.term()?;
        }
        Ok(())
    }

    pub fn term(&mut self) -> Result<(), ParseError> {
        println!("TERM");
        self.unary()?;

        while self.check_token(&TokenType::ASTERISK) || self.check_token(&TokenType::SLASH) {
            self.emitter.emit(&self.cur_token.text);
            self.next_token();
            self.unary()?;
        }
        Ok(())
    }

    pub fn unary(&mut self) -> Result<(), ParseError> {
        println!("UNARY");
        if self.check_token(&TokenType::PLUS) || self.check_token(&TokenType::MINUS) {
            self.emitter.emit(&self.cur_token.text);
            self.next_token();
        }
        self.primary()?;
        Ok(())
    }
    pub fn primary(&mut self) -> Result<(), ParseError> {
        // PRIMARY ::= NUMBER | IDENT

        println!("PRIMARY ( {:?} ) ", self.cur_token.text);

        if self.check_token(&TokenType::NUMBER) {
            self.emitter.emit(&self.cur_token.text);
            self.next_token();
        } else if self.check_token(&TokenType::IDENT) {
            if !self.symbols.contains(&self.cur_token.text) {
                println!(
                    "Referencing variable before assignment {:?}",
                    self.cur_token.text
                );
                std::process::exit(1);
            }
            self.emitter.emit(&self.cur_token.text);
            self.next_token();
        } else {
            return Err(ParseError(format!(
                "Unexpected token at {:?}",
                self.cur_token.text
            )));
        }
        Ok(())
    }
}
