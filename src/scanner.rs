pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: &str) -> Self {
        Self { source: source.to_string(), tokens: vec![],
            start: 0, current: 0, line: 1 }
    }

    pub fn scan_tokens(&mut self) -> Result<Vec<Token>, String> {
        let mut errors: Vec<String>  = vec![];
        while !self.is_at_end() {
            self.start = self.current;
            match self.scan_token() {
                Ok(_) => (),
                Err(msg) => errors.push(msg),
            };
        }

        self.tokens.push(Token { 
            token_type: TokenType::EOF, 
            lexeme: "".to_string(), 
            literal: None, 
            line_number: self.line });

        if errors.len() > 0 {
            let mut joined = String::new();
            for msg in errors {
                joined.push_str(&msg);
                joined.push_str("\n");
            }

            return Err(joined);
        }
        
        Ok(self.tokens.clone())
    }

    fn scan_token(&mut self) -> Result<(), String> {
        let c = self.advance();

        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            ';' => self.add_token(TokenType::Semicolon),
            '+' => self.add_token(TokenType::Plus),
            '*' => self.add_token(TokenType::Star),
            '/' => {
                if self.chmatch('/') {
                    loop {
                        if self.peak() == '\n' || self.is_at_end() {
                            break;
                        }
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash);
                }
            }, 
            '!' => {
                let token = if self.chmatch('=') {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                };

                self.add_token(token);
            }
            '=' => {
                let token = if self.chmatch('=') {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                };
                self.add_token(token);
            }
            '<' => {
                let token = if self.chmatch('=') {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                };
                self.add_token(token);
            }
            '>' => {
                let token = if self.chmatch('=') {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                };
                self.add_token(token);
            }

            ' ' | '\r' | '\t' => (),
            '\n' => self.line += 1,

            _ => return Err(format!("Unrecognized char: {c}, at line: {}", self.line)),
        }

        Ok(())
    }

    fn peak(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }

        self.source.as_bytes()[self.current] as char
    }

    fn chmatch(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        if self.source.as_bytes()[self.current] as char != expected {
            return false;
        } else {
            self.current += 1;
            return true;
        }
    }

    fn advance(&mut self) -> char {
        let c = self.source.as_bytes()[self.current];
        self.current += 1;

        c as char
    }

    fn add_token(&mut self, token_type: TokenType) {
        let mut text = String::new();
        let bytes = self.source.as_bytes();
        for i in self.start..self.current {
            text.push(bytes[i] as char);
        }

        self.tokens.push(Token {
            token_type,
            lexeme: text,
            literal: None,
            line_number: self.line,
        });
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // ONE CHAR TOKENS
    LeftParen, RightParen, LeftBrace, RightBrace, 
    Comma, Dot, Minus, Plus, Semicolon, Slash, Star,

    // ONE-TWO CHAR TOKENS AS ==
    Bang, BangEqual,
    Equal, EqualEqual,
    Greater, GreaterEqual,
    Less, LessEqual,

    // LITERALS
    Identifier, String, Number,

    // LANGUAGE KEYWORDS
    And, Class, Else, False, Fun, For, If, Nil, Or,
    Print, Return, Super, This, True, Var, While,

    EOF,
}

#[derive(Debug, Clone)]
pub enum LiteralType {
    IntValue(i64),
    FloatValue(f64),
    StringValue(String),
    IdentifierValue(String),
}

#[derive(Debug, Clone)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: Option<LiteralType>,
    line_number: usize,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, literal: Option<LiteralType>, line_number: usize) -> Self {
        Self {
            token_type,
            lexeme,
            literal,
            line_number,
        }
    }

    pub fn to_string(&self) -> String {
        format!("{:?} {:?} {:?}", self.token_type, self.lexeme, self.literal)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handle_one_char_tokens() {
        let source = "(( ))";
        let mut scanner = Scanner::new(source);
        let tokens: Vec<Token> = match scanner.scan_tokens() {
            Ok(tokens) => tokens,
            Err(_) => vec![],
        };

        assert_eq!(tokens.len(), 5);
        assert_eq!(tokens[0].token_type, TokenType::LeftParen);
        assert_eq!(tokens[1].token_type, TokenType::LeftParen);
        assert_eq!(tokens[2].token_type, TokenType::RightParen);
        assert_eq!(tokens[3].token_type, TokenType::RightParen);
        assert_eq!(tokens[4].token_type, TokenType::EOF);
    }

    #[test]
    fn handle_two_char_tokens() {
        let source = "! != == <=";
        let mut scanner: Scanner = Scanner::new(source);
        let tokens = match scanner.scan_tokens() {
            Ok(tokens) => tokens,
            Err(_) => vec![],
        };

        assert_eq!(tokens.len(), 5);
        assert_eq!(tokens[0].token_type, TokenType::Bang);
        assert_eq!(tokens[1].token_type, TokenType::BangEqual);
        assert_eq!(tokens[2].token_type, TokenType::EqualEqual);
        assert_eq!(tokens[3].token_type, TokenType::LessEqual);
        assert_eq!(tokens[4].token_type, TokenType::EOF);
    }
}