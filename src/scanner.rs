pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
}

impl Scanner {
    pub fn new(source: &str) -> Self {
        Self { source: source.to_string(), tokens: vec![] }
    }

    pub fn scan_tokens(&self) -> Result<Vec<Token>, String> {
        todo!();
    }
}

#[derive(Debug)]
pub enum TokenType {
    // ONE CHAR TOKENS
    LeftParen, RightParen, LeftBrace, RightBrace, 
    Comma, Dot, Minus, Plus, Semicolon, Slash, Start,

    // ONE-TWO CHAR TOKENS AS ==
    Bang, BangEqual,
    Equal, EqualEqual,
    Greater, GreateEqual,
    Less, LessEqual,

    // LITERALS
    Identifier, String, Number,

    // LANGUAGE KEYWORDS
    And, Class, Else, False, Fun, For, If, Nil, Or,
    Print, Return, Super, This, True, Var, While,

    EOF,
}

#[derive(Debug)]
pub enum LiteralType {
    IntValue(i64),
    FloatValue(f64),
    StringValue(String),
    IdentifierValue(String),
}

#[derive(Debug)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: Option<LiteralType>,
    line_number: u64,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, literal: Option<LiteralType>, line_number: u64) -> Self {
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