use crate::token::{keyword_or_ident, symbol_or_ident, Token, TokenKind};

#[derive(Debug)]
pub struct Lexer<'a> {
    input: &'a str,
    position: usize,
    read_position: usize,
    ch: u8,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &str) -> Lexer {
        let mut l = Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: 0,
        };
        l.read_token();
        l
    }

    fn new_token(kind: TokenKind, literal: &str) -> Token {
        Token {
            kind,
            literal: literal.to_string(),
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.read_whitespace();

        match self.ch {
            b':' | b',' | b'(' | b')' | b'{' | b'}' | b'[' | b']' | b'&' | b'|' => {
                let symbol = self.read_symbol_token();
                let kind = symbol_or_ident(&symbol);
                Lexer::new_token(kind, &symbol)
            }
            b'0'..=b'9' => Lexer::new_token(TokenKind::Number, &self.read_literal_token(is_number)),
            b'a'..=b'z' | b'A'..=b'Z' | b'_' => {
                let ident = self.read_literal_token(is_letter);
                let kind = keyword_or_ident(&ident);
                Lexer::new_token(kind, &ident)
            }
            _ => Lexer::new_token(TokenKind::Illegal, String::from(self.ch as char).as_str()),
        }
    }

    fn read_token(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = 0;
        } else {
            self.ch = self.input.as_bytes()[self.read_position];
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn read_whitespace(&mut self) {
        while self.ch == b' ' || self.ch == b'\n' || self.ch == b'\t' || self.ch == b'\r' {
            self.read_token();
        }
    }

    fn read_symbol_token(&mut self) -> &str {
        let position = self.position;
        self.read_token();
        &self.input[position..self.position]
    }

    fn read_literal_token(&mut self, literal_judger: fn(ch: u8) -> bool) -> &str {
        let position = self.position;
        while literal_judger(self.ch) {
            self.read_token();
        }
        &self.input[position..self.position]
    }
}

fn is_letter(ch: u8) -> bool {
    (b'a'..=b'z').contains(&ch) || (b'A'..=b'Z').contains(&ch) || ch == b'_'
}

fn is_number(ch: u8) -> bool {
    (b'0'..=b'9').contains(&ch)
}

#[test]
fn test_next_token() {
    let input = r#"
    struct Person {
        name: string | null,
    }
    "#;
    let tests = vec![
        Token {
            kind: TokenKind::Struct,
            literal: "struct".to_string(),
        },
        Token {
            kind: TokenKind::Ident,
            literal: "Person".to_string(),
        },
        Token {
            kind: TokenKind::LeftBrace,
            literal: "{".to_string(),
        },
        Token {
            kind: TokenKind::Ident,
            literal: "name".to_string(),
        },
        Token {
            kind: TokenKind::Colon,
            literal: ":".to_string(),
        },
        Token {
            kind: TokenKind::String,
            literal: "string".to_string(),
        },
        Token {
            kind: TokenKind::Or,
            literal: "|".to_string(),
        },
        Token {
            kind: TokenKind::Null,
            literal: "null".to_string(),
        },
        Token {
            kind: TokenKind::Comma,
            literal: ",".to_string(),
        },
        Token {
            kind: TokenKind::RightBrace,
            literal: "}".to_string(),
        },
    ];

    let mut l = Lexer::new(input);
    for tt in tests {
        let tok = l.next_token();
        assert_eq!(tok.kind, tt.kind);
        assert_eq!(tok.literal, tt.literal);
    }
}
