use crate::token::Token;
use core::fmt::Debug;

trait Node {
    fn token_literal(&self) -> Token;
}

trait Statement: Node {
    fn statement_node(&self);
}

trait Expression: Node {
    fn expression_node(&self);
}

struct LetStatement {
    token: Token,
    name: Identifier,
    value: Box<dyn Expression>,
}

impl Node for LetStatement {
    fn token_literal(&self) -> Token {
        self.token.clone()
    }
}

impl Statement for LetStatement {
    fn statement_node(&self) {}
}

#[derive(Debug)]
struct Identifier {
    token: Token,
    value: String,
}

impl Node for Identifier {
    fn token_literal(&self) -> Token {
        self.token.clone()
    }
}

impl Expression for Identifier {
    fn expression_node(&self) {}
}

pub struct Program {
    statements: Vec<Box<dyn Statement>>,
}

impl Node for Program {
    fn token_literal(&self) -> Token {
        if self.statements.len() > 0 {
            self.statements[0].token_literal()
        } else {
            Token::EndOfFile
        }
    }
}
