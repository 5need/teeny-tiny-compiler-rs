use crate::Token;
use crate::TokenType;
use std::process::exit;

#[derive(Debug, Clone)]
pub struct Lexer {
    pub source: String,
    pub cur_char: char,
    pub cur_pos: i32,
}

impl Lexer {
    //Process the next character.
    pub fn next_char(&mut self) {
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
            self.source.chars().nth(self.cur_pos as usize).unwrap()
        } else {
            '\0'
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
    pub fn get_token(&mut self) -> Option<Token> {
        self.skip_whitespace();
        self.skip_comment();
        match self.cur_char {
            '+' => Some(Token {
                text: self.cur_char.to_string(),
                kind: TokenType::PLUS,
            }),
            '-' => Some(Token {
                text: self.cur_char.to_string(),
                kind: TokenType::MINUS,
            }),
            '*' => Some(Token {
                text: self.cur_char.to_string(),
                kind: TokenType::ASTERISK,
            }),
            '/' => Some(Token {
                text: self.cur_char.to_string(),
                kind: TokenType::SLASH,
            }),
            '\n' => Some(Token {
                text: self.cur_char.to_string(),
                kind: TokenType::NEWLINE,
            }),
            '\0' => Some(Token {
                text: '\0'.to_string(),
                kind: TokenType::EOF,
            }),
            '>' => {
                if self.peek() == '=' {
                    let last_char = self.cur_char.clone();
                    self.next_char();
                    Some(Token {
                        text: last_char.to_string() + &self.cur_char.to_string(),
                        kind: TokenType::GTEQ,
                    })
                } else {
                    Some(Token {
                        text: '>'.to_string(),
                        kind: TokenType::GT,
                    })
                }
            }
            '<' => {
                if self.peek() == '=' {
                    let last_char = self.cur_char.clone();
                    self.next_char();
                    Some(Token {
                        text: last_char.to_string() + &self.cur_char.to_string(),
                        kind: TokenType::LTEQ,
                    })
                } else {
                    Some(Token {
                        text: '<'.to_string(),
                        kind: TokenType::LT,
                    })
                }
            }
            '=' => {
                if self.peek() == '=' {
                    let last_char = self.cur_char.clone();
                    self.next_char();
                    Some(Token {
                        text: last_char.to_string() + &self.cur_char.to_string(),
                        kind: TokenType::EQEQ,
                    })
                } else {
                    Some(Token {
                        text: '='.to_string(),
                        kind: TokenType::EQ,
                    })
                }
            }
            '!' => {
                if self.peek() == '=' {
                    let last_char = self.cur_char.clone();
                    self.next_char();
                    Some(Token {
                        text: last_char.to_string() + &self.cur_char.to_string(),
                        kind: TokenType::NOTEQ,
                    })
                } else {
                    None
                }
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
                Some(Token {
                    text: self.source[(start_pos - 1) as usize..(self.cur_pos - 1) as usize]
                        .to_string(),
                    kind: TokenType::STRING,
                })
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
                Some(Token {
                    text: self.source[(start_pos - 1) as usize..(self.cur_pos - 1) as usize]
                        .to_string(),
                    kind: TokenType::NUMBER,
                })
            }
            'A'..='Z' | 'a'..='z' => {
                let start_pos = self.cur_pos;

                while self.peek().is_ascii_alphanumeric() {
                    self.next_char();
                }

                let token_text = &self.source[(start_pos - 1) as usize..(self.cur_pos) as usize];

                let keyword = Token::check_if_keyword(&token_text);

                if keyword.is_none() {
                    Some(Token {
                        text: token_text.to_string(),
                        kind: TokenType::IDENT,
                    })
                } else {
                    Some(Token {
                        text: token_text.to_string(),
                        kind: keyword.unwrap(),
                    })
                }
            }
            _ => None,
        }
    }
}
