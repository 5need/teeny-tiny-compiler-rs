use crate::lexer::*;
use crate::token::*;
use std::process::exit;

#[derive(Debug)]
pub struct Parser {
    pub lexer: Lexer,
    pub cur_token: Option<Token>,
    pub peek_token: Option<Token>,
    pub symbols: Vec<Token>,
    pub labels_declared: Vec<Token>,
    pub labels_gotoed: Vec<Token>,
}

impl Parser {
    fn check_token(&mut self, token_type: TokenType) -> bool {
        return token_type == self.cur_token.clone().unwrap().kind;
    }
    fn check_peek(&mut self, token_type: TokenType) -> bool {
        return token_type == self.peek_token.clone().unwrap().kind;
    }
    fn match_token(&mut self, token_type: TokenType) {
        if !self.check_token(token_type.clone()) {
            self.abort(format!(
                "Expected {:?}, got {:?}",
                token_type, self.cur_token
            ))
        }
        self.next_token();
    }
    fn next_token(&mut self) {
        self.lexer.next_char();
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.lexer.get_token();
    }
    fn abort(&self, message: String) {
        println!("{}", message);
        exit(0);
    }
    pub fn program(&mut self) {
        self.next_token();
        self.next_token();
        println!("PROGRAM");
        while self.check_token(TokenType::NEWLINE) {
            self.next_token();
        }
        while !self.check_token(TokenType::EOF) {
            self.statement();
        }
        for label in self.labels_gotoed.iter() {
            if !self.labels_declared.contains(label) {
                self.abort(format!(
                    "Attempting to GOTO to undeclared label: {:?}",
                    label
                ))
            }
        }
    }
    fn statement(&mut self) {
        if self.check_token(TokenType::PRINT) {
            println!("---STATEMENT-PRINT");
            self.next_token();
            if self.check_token(TokenType::STRING) {
                self.next_token();
            } else {
                self.expression();
            }
        } else if self.check_token(TokenType::IF) {
            println!("---STATEMENT-IF");
            self.next_token();
            self.comparison();
            self.match_token(TokenType::THEN);
            self.nl();
            // zero or more statements in the body
            while !self.check_token(TokenType::ENDIF) {
                self.statement()
            }
            self.match_token(TokenType::ENDIF)
        } else if self.check_token(TokenType::WHILE) {
            println!("---STATEMENT-WHILE");
            self.next_token();
            self.comparison();

            self.match_token(TokenType::REPEAT);
            self.nl();

            while !self.check_token(TokenType::ENDWHILE) {
                self.statement();
            }
            self.match_token(TokenType::ENDWHILE)
        } else if self.check_token(TokenType::LABEL) {
            println!("---STATEMENT-LABEL");
            self.next_token();

            if self
                .labels_declared
                .contains(&self.cur_token.as_ref().unwrap())
            {
                self.abort(format!(
                    "Label already exists: {:?}",
                    self.cur_token.as_ref().unwrap().text
                ))
            }
            self.labels_declared.push(self.cur_token.clone().unwrap());

            self.match_token(TokenType::IDENT);
        } else if self.check_token(TokenType::GOTO) {
            println!("---STATEMENT-GOTO");
            self.next_token();
            self.labels_gotoed.push(self.cur_token.clone().unwrap());
            self.match_token(TokenType::IDENT);
        } else if self.check_token(TokenType::LET) {
            println!("---STATEMENT-LET");
            self.next_token();
            if !self.symbols.contains(self.cur_token.as_ref().unwrap()) {
                self.symbols.push(self.cur_token.clone().unwrap())
            }
            self.match_token(TokenType::IDENT);
            self.match_token(TokenType::EQ);
            self.expression();
        } else if self.check_token(TokenType::INPUT) {
            println!("---STATEMENT-INPUT");
            self.next_token();
            if !self.symbols.contains(self.cur_token.as_ref().unwrap()) {
                self.symbols.push(self.cur_token.clone().unwrap())
            }
            self.match_token(TokenType::IDENT);
        } else {
            self.abort(format!(
                "Invalid statement at {:?} ({:?})",
                self.cur_token.clone().unwrap().text,
                self.cur_token.clone().unwrap().kind
            ))
        }
        self.nl();
    }
    fn nl(&mut self) {
        println!("---NEWLINE");
        while self.check_token(TokenType::NEWLINE) {
            self.next_token();
        }
    }
    fn comparison(&mut self) {
        println!("---COMPARISON");
        self.expression();
        // Must be at least one comparison operator and another expression.
        if self.is_comparison_operator() {
            self.next_token();
            self.expression();
        } else {
            self.abort(format!(
                "Expected comparison operator at: {:?}",
                self.cur_token.as_ref().unwrap().text,
            ));
        }

        // Can have 0 or more comparison operator and expressions.
        while self.is_comparison_operator() {
            self.next_token();
            self.expression();
        }
    }
    fn is_comparison_operator(&mut self) -> bool {
        return self.check_token(TokenType::GT)
            || self.check_token(TokenType::GTEQ)
            || self.check_token(TokenType::LT)
            || self.check_token(TokenType::LTEQ)
            || self.check_token(TokenType::EQEQ)
            || self.check_token(TokenType::NOTEQ);
    }
    fn expression(&mut self) {
        println!("---EXPRESSION");
        self.term();
        // Can have 0 or more +/- and expressions.
        while self.check_token(TokenType::PLUS) || self.check_token(TokenType::MINUS) {
            self.next_token();
            self.term();
        }
    }
    fn term(&mut self) {
        println!("---TERM");
        self.unary();
        // Can have 0 or more *// and expressions.
        while self.check_token(TokenType::ASTERISK) || self.check_token(TokenType::SLASH) {
            self.next_token();
            self.unary();
        }
    }
    fn unary(&mut self) {
        println!("---UNARY");
        // Optional unary +/-
        if self.check_token(TokenType::PLUS) || self.check_token(TokenType::MINUS) {
            self.next_token();
        }
        self.primary();
    }
    fn primary(&mut self) {
        println!("---PRIMARY");
        if self.check_token(TokenType::NUMBER) {
            self.next_token();
        } else if self.check_token(TokenType::IDENT) {
            if !self.symbols.contains(self.cur_token.as_ref().unwrap()) {
                self.abort(format!(
                    "Referencing variable before assignment: {:?}",
                    self.cur_token.as_ref().unwrap().text
                ))
            }
            self.next_token();
        } else {
            // Error!
            self.abort(format!(
                "Unexpected token at {:?}",
                self.cur_token.as_ref().unwrap().text
            ));
        }
    }
}
