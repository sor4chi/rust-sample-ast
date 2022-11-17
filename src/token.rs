use phf::phf_map;

#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
    // Operators
    And,          // &
    Or,           // |
    Colon,        // :
    Comma,        // ,
    LeftParen,    // (
    RightParen,   // )
    LeftBrace,    // {
    RightBrace,   // }
    LeftBracket,  // [
    RightBracket, // ]

    // Keywords
    Struct,  // struct
    Enum,    // enum
    Number,  // number
    String,  // string
    Boolean, // boolean
    Null,    // null

    // Other
    Ident,
    Illegal, // Illegal
}

pub struct Token {
    pub kind: TokenKind,
    pub literal: String,
}

pub static KEYWORDS: phf::Map<&'static str, TokenKind> = phf_map! {
    "struct" => TokenKind::Struct,
    "enum" => TokenKind::Enum,
    "number" => TokenKind::Number,
    "string" => TokenKind::String,
    "boolean" => TokenKind::Boolean,
    "null" => TokenKind::Null,
};

pub fn keyword_or_ident(ident: &str) -> TokenKind {
    KEYWORDS.get(ident).unwrap_or(&TokenKind::Ident).clone()
}

pub static SYMBOLS: phf::Map<&'static str, TokenKind> = phf_map! {
    ":" => TokenKind::Colon,
    "," => TokenKind::Comma,
    "(" => TokenKind::LeftParen,
    ")" => TokenKind::RightParen,
    "{" => TokenKind::LeftBrace,
    "}" => TokenKind::RightBrace,
    "[" => TokenKind::LeftBracket,
    "]" => TokenKind::RightBracket,
    "&" => TokenKind::And,
    "|" => TokenKind::Or,
};

pub fn symbol_or_ident(symbol: &str) -> TokenKind {
    SYMBOLS.get(symbol).unwrap_or(&TokenKind::Ident).clone()
}
