use crate::{ast::Program, lexer::Lexer, token::Token};

pub struct Parser {
    lexer: Lexer,

    cur_token: Option<Token>,
    peek_token: Option<Token>,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        let mut p = Parser {
            lexer,
            cur_token: None,
            peek_token: None,
        };

        p.next_token();
        p.next_token();

        p
    }

    pub fn parse_program(&self) -> Option<Program> {
        None
    }

    fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }
}
