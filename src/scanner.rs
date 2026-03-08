use std::result;

pub fn is_digit(c: char) -> bool {
    (c as u8) >= b'0' && (c as u8) <= b'9'
}

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
        let mut errors: Vec<String> = vec![];
        while !self.is_at_end() {
            self.start = self.current;
            match self.scan_token() {
                Ok(_) => (),
                Err(msg) => errors.push(msg),
            };
        }

        let eof = 
            Token::new(
                TokenType::EOF, 
                "".to_string(), 
                None, 
                self.line );
    

        self.tokens.push(eof);

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
                        if self.peek() == '\n' || self.is_at_end() {
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

            '"' => self.string()?,

            c => {
                if is_digit(c) {
                    if let Err(msg) = self.number() {
                        return Err(msg);
                    }
                } else {
                    return Err(format!("Unrecognized char: {c}, at line: {}", self.line));
                }
            }
        }

        Ok(())
    }



    fn number(&mut self) -> Result<(), String> {
        while is_digit(self.peek()) {
            self.advance();
        }

        if self.peek() == '.' && is_digit(self.peek_next()) {
            self.advance();

            while is_digit(self.peek()) {
                self.advance();
            }
        }

        let substring = &self.source[self.start..self.current];

        if let Ok(val) = substring.parse::<f64>() {
            self.add_token_lit(TokenType::Number, LiteralType::FloatValue(val));
            Ok(())
        } else {
            Err(format!("Could not parse: {}", substring))
        }
    }

    fn peek_next(&self) -> char {
        if (self.current + 1) >= self.source.len() {
            return '\0';
        }

        self.source.chars().nth(self.current + 1).unwrap()
    }

    fn string(&mut self) -> Result<(), String> {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            return Err("Unterminated string!".to_string());
        }

        self.advance();

        let value = &self.source[self.start + 1..self.current - 1];

        self.add_token_lit(TokenType::String, LiteralType::StringValue(value.to_string()));

        Ok(())
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }

        self.source.chars().nth(self.current).unwrap()
    }

    fn chmatch(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        if self.source.chars().nth(self.current).unwrap() != expected {
            return false;
        } else {
            self.current += 1;
            return true;
        }
    }

    fn advance(&mut self) -> char {
        let c = self.source.chars().nth(self.current);
        self.current += 1;

        c.unwrap()
    }

    fn add_token_lit(&mut self, token_type: TokenType, literal: LiteralType) {
        let mut text = String::new();
        let _lit= self.source[self.start..self.current].chars()
                        .map(|ch| text.push(ch));

        self.tokens.push(Token {
            token_type,
            lexeme: text,
            literal: Some(literal),
            line_number: self.line,
        });
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

#[derive(Debug, Clone, PartialEq)]
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
        let source = "(( )) {+";
        let mut scanner = Scanner::new(source);
        let tokens: Vec<Token> = match scanner.scan_tokens() {
            Ok(tokens) => tokens,
            Err(_) => vec![],
        };

        assert_eq!(tokens.len(), 7);
        assert_eq!(tokens[0].token_type, TokenType::LeftParen);
        assert_eq!(tokens[1].token_type, TokenType::LeftParen);
        assert_eq!(tokens[2].token_type, TokenType::RightParen);
        assert_eq!(tokens[3].token_type, TokenType::RightParen);
        assert_eq!(tokens[4].token_type, TokenType::LeftBrace);
        assert_eq!(tokens[5].token_type, TokenType::Plus);
        assert_eq!(tokens[6].token_type, TokenType::EOF);
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

    #[test]
    fn handle_string_literal() {
        let source = r#""Hello world!""#;
        let mut scanner: Scanner = Scanner::new(source);
        let tokens = match scanner.scan_tokens() {
            Ok(tokens) => tokens,
            Err(_) => vec![],
        };

        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].token_type, TokenType::String);
        match tokens[0].literal.as_ref().unwrap() {
            LiteralType::StringValue(val) => assert_eq!(val, "Hello world!"),
            _ => panic!("Incorrect literal type!"),
        };
    }

    #[test]
    fn handle_unterminated_string_literal() {
        let source = r#""Hello world!"#;
        let mut scanner: Scanner = Scanner::new(source);
        let tokens = match scanner.scan_tokens() {
            Ok(tokens) => tokens,
            Err(msg) => {
                assert_eq!(msg, "Unterminated string!\n");
                vec![]
            }
        };

        assert_eq!(tokens.len(), 0);
    }


    #[test]
    fn handle_string_literal_multiline() {
        let source = "\"Hello\nworld!\"";
        let mut scanner: Scanner = Scanner::new(source);
        let tokens = match scanner.scan_tokens() {
            Ok(tokens) => tokens,
            Err(_) => vec![],
        };

        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].token_type, TokenType::String);
        match tokens[0].literal.as_ref().unwrap() {
            LiteralType::StringValue(val) => assert_eq!(val, "Hello\nworld!"),
            _ => panic!("Incorrect literal type!"),
        };
    }

    #[test]
    fn handle_number_literals() {
        let source = "123.431\n321.0\n5";
        let mut scanner: Scanner = Scanner::new(source);
        let tokens = match scanner.scan_tokens() {
            Ok(tokens) => tokens,
            Err(_) => vec![],
        };
        assert_eq!(tokens.len(), 4);

        for i in 0..3 {
            assert_eq!(tokens[i].token_type, TokenType::Number);
        }

        match tokens[0].literal {
            Some(LiteralType::FloatValue(val)) => assert_eq!(val, 123.431),
            _ => panic!("Incorrect literal when expected 123.431"),
        };  
        
        match tokens[1].literal {
            Some(LiteralType::FloatValue(val)) => assert_eq!(val, 321.0),
            _ => panic!("Incorrect literal when expected 321.0"),
        };  

        match tokens[2].literal {
            Some(LiteralType::FloatValue(val)) => assert_eq!(val, 5.0),
            _ => panic!("Incorrect literal when expected 123.431"),
        };


    }
}