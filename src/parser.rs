use crate::{lexer::Lexer, token::Token};

struct Parser<'a> {
    lexer: Lexer<'a>,
    curr: Option<Token>,
    peek: Option<Token>,
}

impl<'a> Parser<'a> {
    fn new(lexer: Lexer<'a>) -> Parser<'a> {
        let mut p = Parser {
            lexer,
            curr: None,
            peek: None,
        };
        p.next_token();
        p.next_token();
        p
    }

    fn next_token(&mut self) {
        self.curr = self.peek.take();
        self.peek = Some(self.lexer.next_token());
    }

    fn expect_peek(&mut self, kind: &Token) -> bool {
        if self.peek == Some(kind.clone()) {
            self.next_token();
            true
        } else {
            false
        }
    }
}
