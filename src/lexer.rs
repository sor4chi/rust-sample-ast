use crate::token::{Token, TokenKind, KEYWORDS};

#[derive(Debug)]
pub struct Lexer<'a> {
    input: &'a str,
    position: usize,
    read_position: usize,
    ch: u8,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut l = Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: 0,
        };
        l.read_char();
        return l;
    }

    pub fn new_token(kind: TokenKind, ch: u8) -> Token {
        Token {
            kind,
            literal: String::from_utf8(vec![ch]).unwrap(),
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let token;
        match self.ch {
            b':' => token = Lexer::new_token(TokenKind::Colon, self.ch),
            b',' => token = Lexer::new_token(TokenKind::Comma, self.ch),
            b'(' => token = Lexer::new_token(TokenKind::LeftParen, self.ch),
            b')' => token = Lexer::new_token(TokenKind::RightParen, self.ch),
            b'{' => token = Lexer::new_token(TokenKind::LeftBrace, self.ch),
            b'}' => token = Lexer::new_token(TokenKind::RightBrace, self.ch),
            b'[' => token = Lexer::new_token(TokenKind::LeftBracket, self.ch),
            b']' => token = Lexer::new_token(TokenKind::RightBracket, self.ch),
            b'&' => token = Lexer::new_token(TokenKind::And, self.ch),
            b'|' => token = Lexer::new_token(TokenKind::Or, self.ch),
            b'0'..=b'9' => {
                token = Token {
                    kind: TokenKind::Number,
                    literal: self.read_number(),
                };
                return token;
            }
            b'a'..=b'z' | b'A'..=b'Z' | b'_' => {
                let ident = self.read_ident();
                let kind = KEYWORDS
                    .get(ident.as_str())
                    .unwrap_or(&TokenKind::Ident)
                    .clone();
                token = Token {
                    kind,
                    literal: ident,
                };
                return token;
            }
            _ => token = Lexer::new_token(TokenKind::Illegal, self.ch),
        }
        self.read_char();
        return token;
    }

    fn skip_whitespace(&mut self) {
        while self.ch == b' ' || self.ch == b'\n' || self.ch == b'\t' || self.ch == b'\r' {
            self.read_char();
        }
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = 0;
        } else {
            self.ch = self.input.as_bytes()[self.read_position];
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn read_number(&mut self) -> String {
        let position = self.position;
        while self.ch >= b'0' && self.ch <= b'9' {
            self.read_char();
        }
        self.input[position..self.position].to_string()
    }

    fn is_letter(&self, ch: u8) -> bool {
        (b'a'..=b'z').contains(&ch) || (b'A'..=b'Z').contains(&ch) || ch == b'_'
    }

    fn read_ident(&mut self) -> String {
        let position = self.position;
        while self.is_letter(self.ch) {
            self.read_char();
        }
        self.input[position..self.position].to_string()
    }
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
