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
