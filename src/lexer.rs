use crate::token::Token;

pub struct Lexer {
    input: Vec<char>,
    pos: usize,      // current position in input (points to current char)
    read_pos: usize, // current reading position in input (after current char)
    ch: char,        // current char under examination
}

fn get_identifier(identifier: String) -> Token {
    match identifier.as_str() {
        "fun" => Token::Fun,
        "let" => Token::Let,
        "struct" => Token::Struct,
        _ => Token::Name {
            name: identifier.into(),
        },
    }
}

fn is_letter(ch: char) -> bool {
    'a' <= ch && ch <= 'z' || 'A' <= ch && ch <= 'Z' || ch == '_'
}

fn is_digit(ch: char) -> bool {
    '0' <= ch && ch <= '9'
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut lexer = Self {
            input: input.chars().collect(),
            pos: 0,
            read_pos: 0,
            ch: '\0',
        };

        // Initialize the read position for users of our lexer
        lexer.read_char();

        lexer
    }

    pub fn next_token(&mut self) -> Option<Token> {
        self.skip_whitespace();

        let token = match self.ch {
            '=' => Some(Token::Equal),
            ';' => Some(Token::SemiColon),
            '(' => Some(Token::LeftParen),
            ')' => Some(Token::RightParen),
            ',' => Some(Token::Comma),
            '+' => Some(Token::Plus),
            '{' => Some(Token::LeftBrace),
            '}' => Some(Token::RightBrace),
            '\0' => Some(Token::EndOfFile),
            _ => None,
        };

        if token == None {
            if is_letter(self.ch) {
                let name = self.read_name();
                Some(get_identifier(name))
            } else if is_digit(self.ch) {
                // TODO: extend for floats
                let integer = self.read_number();
                Some(Token::Int {
                    value: integer.into(),
                })
            } else {
                Some(Token::Illegal)
            }
        } else {
            self.read_char();
            return token;
        }
    }

    fn read_char(&mut self) {
        if self.read_pos >= self.input.len() {
            self.ch = '\0';
        } else {
            self.ch = self.input[self.read_pos];
        }

        self.pos = self.read_pos;
        self.read_pos += 1;
    }

    fn read_name(&mut self) -> String {
        let position = self.pos;
        while is_letter(self.ch) {
            self.read_char();
        }
        String::from_iter(self.input[position..self.pos].into_iter())
    }

    fn read_number(&mut self) -> String {
        let position = self.pos;
        while is_digit(self.ch) {
            self.read_char();
        }
        String::from_iter(self.input[position..self.pos].into_iter())
    }

    fn skip_whitespace(&mut self) {
        while self.ch == ' ' || self.ch == '\t' || self.ch == '\n' || self.ch == '\r' {
            self.read_char();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_token() {
        let input = String::from(
            r#"
            fun do_thing() {
    let x = 0;
    let b = 1;

    b + x;
}
        "#,
        );

        let mut lexer = Lexer::new(input);
        let expected = vec![
            Token::Fun,
            Token::Name {
                name: "do_thing".into(),
            },
            Token::LeftParen,
            Token::RightParen,
            Token::LeftBrace,
            Token::Let,
            Token::Name { name: "x".into() },
            Token::Equal,
            Token::Int { value: "0".into() },
            Token::SemiColon,
            Token::Let,
            Token::Name { name: "b".into() },
            Token::Equal,
            Token::Int { value: "1".into() },
            Token::SemiColon,
            Token::Name { name: "b".into() },
            Token::Plus,
            Token::Name { name: "x".into() },
            Token::SemiColon,
        ];

        for expected_token in expected {
            let result: Option<Token> = lexer.next_token();

            if let Some(result) = result {
                dbg!(&result);
                assert_eq!(result, expected_token);
            }
        }
    }
}
