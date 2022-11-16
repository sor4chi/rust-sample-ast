use crate::token::Token;

struct Lexer {
    input: Vec<char>,
    position: usize,
}

impl Lexer {
    fn new(input: Vec<char>) -> Lexer {
        Lexer { input, position: 0 }
    }

    fn next_token(&mut self) -> Option<Token> {
        if self.position >= self.input.len() {
            return Some(Token::Eof);
        }

        let ch = self.input[self.position];
        self.position += 1;

        match ch {
            '+' => Some(Token::Plus),
            '-' => Some(Token::Minus),
            '*' => Some(Token::Asterisk),
            '/' => Some(Token::Slash),
            '(' => Some(Token::LParen),
            ')' => Some(Token::RParen),
            ' ' => Some(Token::Whitespace),
            '0'..='9' => Some(Token::Number(ch.to_digit(10).unwrap() as f64)),
            _ => Some(Token::Illegal),
        }
    }
}

#[test]
fn test_lexer() {
    let mut lexer = Lexer::new("1 + 2".chars().collect());
    assert_eq!(lexer.next_token(), Some(Token::Number(1.0)));
    assert_eq!(lexer.next_token(), Some(Token::Whitespace));
    assert_eq!(lexer.next_token(), Some(Token::Plus));
    assert_eq!(lexer.next_token(), Some(Token::Whitespace));
    assert_eq!(lexer.next_token(), Some(Token::Number(2.0)));
    assert_eq!(lexer.next_token(), Some(Token::Eof));
}
