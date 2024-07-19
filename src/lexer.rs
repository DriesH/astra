use crate::token::Token;

pub struct Lexer {
    input: Vec<char>,
    pos: usize,       // current position in input (points to current char)
    read_pos: usize,  // current reading position in input (after current char)
    ch: Option<char>, // current char under examination
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut lexer = Self {
            input: input.chars().collect(),
            pos: 0,
            read_pos: 0,
            ch: None,
        };

        // Initialize the read position for users of our lexer
        lexer.read_char();

        lexer
    }

    pub fn next_token(&mut self) -> Option<Token> {
        while is_whitespace(self.ch) {
            self.read_char();
        }

        if let Some(c) = self.ch {
            if is_name(c) {
                let name_token = self.lex_name();
                Some(name_token)
                // TODO: dot access
            } else if is_number(c) {
                let number_token = self.lex_number();
                Some(number_token)
            } else {
                self.consume_single_char(c)
            }
        } else {
            // End of file reached
            Some(Token::EndOfFile)
        }
    }

    fn read_char(&mut self) {
        if self.read_pos >= self.input.len() {
            self.ch = None;
        } else {
            self.ch = Some(self.input[self.read_pos]);
        }

        self.pos = self.read_pos;
        self.read_pos += 1;
    }

    fn lex_name(&mut self) -> Token {
        let mut name = String::new();

        while self.is_continuous_name() {
            name.push(self.ch.expect("lex_name continuous name failure"));
            self.read_char();
        }

        if let Some(token) = str_to_identifier(&name) {
            token
        } else {
            Token::Name { name: name.into() }
        }
    }

    fn is_continuous_name(&self) -> bool {
        self.ch
            .map(|c| matches!(c, '_' | '0'..='9' | 'a'..='z' | 'A'..='Z'))
            .unwrap_or(false)
    }

    // TODO: Support floats, if we run into a '.' after a number, this will blow up
    fn lex_number(&mut self) -> Token {
        let mut value = String::new();

        value.push_str(&self.radix_run(10));

        Token::Int {
            value: value.into(),
        }
    }

    // Consume a sequence of numbers with the given radix
    fn radix_run(&mut self, radix: u32) -> String {
        let mut value_text = String::new();

        loop {
            if let Some(c) = self.take_number(radix) {
                value_text.push(c);
            } else {
                break;
            }
        }
        value_text
    }

    // Consume a single character with the given radix.
    fn take_number(&mut self, radix: u32) -> Option<char> {
        let take_char = is_digit_of_radix(self.ch, radix);

        if take_char {
            let number = Some(self.ch.expect("take_number next char"));
            self.read_char();

            number
        } else {
            None
        }
    }

    fn consume_single_char(&mut self, c: char) -> Option<Token> {
        let token = match c {
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

        self.read_char();

        token
    }
}

pub fn str_to_identifier(identifier: &str) -> Option<Token> {
    match identifier {
        "fun" => Some(Token::Fun),
        "let" => Some(Token::Let),
        "struct" => Some(Token::Struct),
        "enum" => Some(Token::Enum),
        _ => None,
    }
}

fn is_name(ch: char) -> bool {
    matches!(ch, 'a'..='z' | 'A'..='Z')
}

fn is_number(ch: char) -> bool {
    matches!(ch, '0'..='9')
}

fn is_whitespace(ch: Option<char>) -> bool {
    match ch {
        Some(c) => matches!(c, ' ' | '\t' | '\n' | '\r'),
        None => false,
    }
}

// Test if a digit is of a certain radix.
fn is_digit_of_radix(c: Option<char>, radix: u32) -> bool {
    match radix {
        2 | 8 | 10 | 16 => c.filter(|c| c.is_digit(radix)).is_some(),
        other => panic!("Radix not implemented: {other}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_str_to_identifier() {
        let input = String::from("fun");
        let expected = Token::Fun;
        let result = str_to_identifier(&input).unwrap();

        assert_eq!(expected, result);

        let input = String::from("let");
        let expected = Token::Let;
        let result = str_to_identifier(&input).unwrap();

        assert_eq!(expected, result);

        let input = String::from("struct");
        let expected = Token::Struct;
        let result = str_to_identifier(&input).unwrap();

        assert_eq!(expected, result);

        let input = String::from("enum");
        let expected = Token::Enum;
        let result = str_to_identifier(&input).unwrap();

        assert_eq!(expected, result);

        let input = String::from("my_function_name");
        let expected: Option<Token> = None;
        let result = str_to_identifier(&input);

        assert_eq!(expected, result);
    }

    #[test]
    fn test_is_number() {
        let input = '8';
        let result = is_number(input);
        assert!(result == true);

        let input = 'r';
        let result = is_number(input);
        assert!(result == false);
    }

    #[test]
    fn test_is_name() {
        let input = 'r';
        let result = is_name(input);
        assert!(result == true);

        let input = '8';
        let result = is_name(input);
        assert!(result == false);

        let input = '-';
        let result = is_name(input);
        assert!(result == false);
    }

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
