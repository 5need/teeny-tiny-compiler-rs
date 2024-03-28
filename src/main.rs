//https://austinhenley.com/blog/teenytinycompiler1.html

use std::fs::File;
use std::io::{self, Read};
use std::process::exit;

fn main() -> io::Result<()> {
    let file_path = "test.txt";
    let mut file = File::open(file_path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    println!(
        "input test.txt\n------------\n{}------------",
        content.to_string()
    );

    let mut lexx = Lexer {
        source: content.to_string(),
        cur_char: ' ',
        cur_pos: 0,
    };

    while lexx.cur_char != '\0' {
        lexx.next_char();
        let current_token = lexx.get_token();
        println!(
            "{:?}: {:?}",
            current_token.clone().unwrap().kind,
            current_token.unwrap().text
        );
    }
    // okay so the lexer is doing the correct things and whatever,
    // it's just the parser is being retarded with how to drive the lexer.

    // let mut parss = Parser {
    //     lexer: lexx,
    //     cur_token: None,
    //     peek_token: None,
    // };
    // parss.next_token();
    // parss.next_token();
    //
    // parss.program();

    Ok(())
}

#[derive(Debug, Clone)]
struct Lexer {
    source: String,
    cur_char: char,
    cur_pos: i32,
}

impl Lexer {
    //Process the next character.
    fn next_char(&mut self) {
        if self.cur_pos < self.source.len() as i32 {
            self.cur_char = self.source.chars().nth(self.cur_pos as usize).unwrap();
            self.cur_pos += 1;
        } else {
            self.cur_char = '\0';
        }
    }

    // Return the lookahead character.
    fn peek(&self) -> char {
        if self.cur_pos + 1 < self.source.len() as i32 {
            return self.source.chars().nth(self.cur_pos as usize).unwrap();
        } else {
            return '\0';
        }
    }

    // Invalid token found, print error message and exit.
    fn abort(&self, message: &str) {
        println!("{}", message);
        exit(0);
    }

    // Skip whitespace except newlines, which we will use to indicate the end of a statement.
    fn skip_whitespace(&mut self) {
        while self.cur_char == ' ' || self.cur_char == '\t' || self.cur_char == '\r' {
            self.next_char();
        }
    }

    // Skip comments in the code.
    fn skip_comment(&mut self) {
        if self.cur_char == '#' {
            while self.cur_char != '\n' {
                self.next_char();
            }
            self.next_char();
        }
    }

    // Return the next token.
    fn get_token(&mut self) -> Option<Token> {
        self.skip_whitespace();
        self.skip_comment();
        match self.cur_char {
            '+' => {
                return Some(Token {
                    text: self.cur_char.to_string(),
                    kind: TokenType::PLUS,
                });
            }
            '-' => {
                return Some(Token {
                    text: self.cur_char.to_string(),
                    kind: TokenType::MINUS,
                });
            }
            '*' => {
                return Some(Token {
                    text: self.cur_char.to_string(),
                    kind: TokenType::ASTERISK,
                });
            }
            '/' => {
                return Some(Token {
                    text: self.cur_char.to_string(),
                    kind: TokenType::SLASH,
                });
            }
            '\n' => {
                return Some(Token {
                    text: self.cur_char.to_string(),
                    kind: TokenType::NEWLINE,
                });
            }
            '\0' => {
                return Some(Token {
                    text: '\0'.to_string(),
                    kind: TokenType::EOF,
                });
            }
            '>' => {
                if self.peek() == '=' {
                    let last_char = self.cur_char.clone();
                    self.next_char();
                    return Some(Token {
                        text: last_char.to_string() + &self.cur_char.to_string(),
                        kind: TokenType::GTEQ,
                    });
                }
                return Some(Token {
                    text: '>'.to_string(),
                    kind: TokenType::GT,
                });
            }
            '<' => {
                if self.peek() == '=' {
                    let last_char = self.cur_char.clone();
                    self.next_char();
                    return Some(Token {
                        text: last_char.to_string() + &self.cur_char.to_string(),
                        kind: TokenType::LTEQ,
                    });
                }
                return Some(Token {
                    text: '<'.to_string(),
                    kind: TokenType::LT,
                });
            }
            '=' => {
                if self.peek() == '=' {
                    let last_char = self.cur_char.clone();
                    self.next_char();
                    return Some(Token {
                        text: last_char.to_string() + &self.cur_char.to_string(),
                        kind: TokenType::EQEQ,
                    });
                }
                return Some(Token {
                    text: '='.to_string(),
                    kind: TokenType::EQ,
                });
            }
            '!' => {
                if self.peek() == '=' {
                    let last_char = self.cur_char.clone();
                    self.next_char();
                    return Some(Token {
                        text: last_char.to_string() + &self.cur_char.to_string(),
                        kind: TokenType::NOTEQ,
                    });
                }
                return None;
            }
            '"' => {
                self.next_char();
                let start_pos = self.cur_pos.clone();

                while self.cur_char != '"' {
                    if self.cur_char == '\r'
                        || self.cur_char == '\n'
                        || self.cur_char == '\\'
                        || self.cur_char == '%'
                    {
                        self.abort(
                            &format!(
                                "Illegal character in string: {:?} at position {:?}",
                                self.cur_char, self.cur_pos
                            )
                            .to_string(),
                        );
                    }
                    self.next_char();
                }
                return Some(Token {
                    text: self.source[(start_pos - 1) as usize..(self.cur_pos - 1) as usize]
                        .to_string(),
                    kind: TokenType::STRING,
                });
            }
            '0'..='9' => {
                let start_pos = self.cur_pos;

                while self.cur_char.is_ascii_digit() {
                    self.next_char();
                }
                if self.cur_char == '.' {
                    self.next_char();
                    if !self.peek().is_ascii_digit() {
                        self.abort("Illegal character in number.");
                    }
                    while self.peek().is_ascii_digit() {
                        self.next_char();
                    }
                }
                return Some(Token {
                    text: self.source[(start_pos - 1) as usize..(self.cur_pos - 1) as usize]
                        .to_string(),
                    kind: TokenType::NUMBER,
                });
            }
            'A'..='Z' | 'a'..='z' => {
                let start_pos = self.cur_pos;

                while self.peek().is_ascii_alphanumeric() {
                    self.next_char();
                }

                let token_text = &self.source[(start_pos - 1) as usize..(self.cur_pos) as usize];

                let keyword = Token::check_if_keyword(&token_text);

                if keyword.is_none() {
                    return Some(Token {
                        text: token_text.to_string(),
                        kind: TokenType::IDENT,
                    });
                } else {
                    return Some(Token {
                        text: token_text.to_string(),
                        kind: keyword.unwrap(),
                    });
                }
            }
            _ => {
                return None;
            }
        };
    }
}

#[derive(Debug, Clone)]
struct Token {
    text: String,
    kind: TokenType,
}

impl Token {
    fn check_if_keyword(input: &str) -> Option<TokenType> {
        match input {
            "LABEL" => return Some(TokenType::LABEL),
            "GOTO" => return Some(TokenType::GOTO),
            "PRINT" => return Some(TokenType::PRINT),
            "INPUT" => return Some(TokenType::INPUT),
            "LET" => return Some(TokenType::LET),
            "IF" => return Some(TokenType::IF),
            "THEN" => return Some(TokenType::THEN),
            "ENDIF" => return Some(TokenType::ENDIF),
            "WHILE" => return Some(TokenType::WHILE),
            "REPEAT" => return Some(TokenType::REPEAT),
            "ENDWHILE" => return Some(TokenType::ENDWHILE),
            _ => return None,
        };
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum TokenType {
    EOF,
    NEWLINE,
    NUMBER,
    IDENT,
    STRING,
    // Keywords
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
    GTEQ,
}

#[derive(Debug)]
struct Parser {
    lexer: Lexer,
    cur_token: Option<Token>,
    peek_token: Option<Token>,
}

impl Parser {
    fn check_token(&self, token_type: TokenType) -> bool {
        // println!(
        //     "{:?} and {:?}",
        //     token_type,
        //     self.cur_token.as_ref().unwrap().kind
        // );
        // println!("{:?}", self.lexer.get_token());
        return token_type == self.cur_token.as_ref().unwrap().kind;
    }
    fn check_peek(&self, token_type: TokenType) -> bool {
        return token_type == self.peek_token.as_ref().unwrap().kind;
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
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.lexer.get_token();
    }

    fn abort(&self, message: String) {
        println!("{}", message);
        exit(0);
    }

    fn program(&mut self) {
        println!("PROGRAM");

        while !self.check_token(TokenType::EOF) {
            self.statement();
        }
    }

    fn statement(&mut self) {
        if self.check_token(TokenType::PRINT) {
            println!("STATEMENT-PRINT");
            self.next_token();
            if self.check_token(TokenType::STRING) {
                self.next_token();
            } else {
                self.expression();
            }
        } else if self.check_token(TokenType::IF) {
            println!("STATEMENT-IF");
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
            println!("STATEMENT-WHILE");
            self.next_token();
            self.comparison();

            self.match_token(TokenType::REPEAT);
            self.nl();

            while !self.check_token(TokenType::ENDWHILE) {
                self.statement();
            }
            self.match_token(TokenType::ENDWHILE)
        } else if self.check_token(TokenType::LABEL) {
            println!("STATEMENT-LABEL");
            self.next_token();
            self.match_token(TokenType::IDENT);
        } else if self.check_token(TokenType::GOTO) {
            println!("STATEMENT-GOTO");
            self.next_token();
            self.match_token(TokenType::IDENT);
        } else if self.check_token(TokenType::LET) {
            println!("STATEMENT-LET");
            self.next_token();
            self.match_token(TokenType::IDENT);
            self.match_token(TokenType::EQ);
            self.expression();
        } else if self.check_token(TokenType::INPUT) {
            println!("STATEMENT-INPUT");
            self.next_token();
            self.match_token(TokenType::IDENT);
        } else {
            self.abort(format!(
                "Invalid statement at {:?} ({:?})",
                self.cur_token.as_ref().unwrap().text,
                self.cur_token.as_ref().unwrap().kind
            ))
        }
        self.nl();
    }

    fn nl(&mut self) {
        println!("NEWLINE");
        self.match_token(TokenType::NEWLINE);
        self.match_token(TokenType::NEWLINE);
        self.match_token(TokenType::NEWLINE);
        self.match_token(TokenType::NEWLINE);
        self.match_token(TokenType::NEWLINE);
        while self.check_token(TokenType::NEWLINE) {
            self.next_token();
        }
    }

    fn comparison(&self) {
        todo!()
    }

    fn expression(&self) {
        todo!()
    }
}
