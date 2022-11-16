#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Number(f64),
    Plus,
    Minus,
    Asterisk,
    Slash,
    LParen,
    RParen,
    Whitespace,
    Illegal,
    Eof,
}
