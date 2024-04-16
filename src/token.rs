#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub text: String,
    pub kind: TokenType,
}

impl Token {
    pub fn check_if_keyword(input: &str) -> Option<TokenType> {
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
pub enum TokenType {
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
