use core::fmt;

use ecow::EcoString;

#[derive(Debug, PartialEq)]
pub enum Token {
    Illegal,

    Name { name: EcoString },

    Int { value: EcoString },
    Float { value: EcoString },
    String { value: EcoString },

    LeftParen,   // (
    RightParen,  // )
    LeftSquare,  // [
    RightSquare, // ]
    LeftBrace,   // {
    RightBrace,  // }

    Plus,
    Minus,
    Star,
    Slash,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,
    Percent,

    Colon,
    SemiColon,
    Comma,
    Bang, // '!'
    Equal,
    EqualEqual, // '=='
    NotEqual,   // '!='
    AmperAmper, // '&&'
    Dot,        // '.'
    EndOfFile,

    Fun,
    Let,
    Struct,
    Enum,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Token::Illegal => "illegal",
            Token::Name { name } => name.as_str(),
            Token::Int { value } | Token::Float { value } | Token::String { value } => {
                value.as_str()
            }

            Token::LeftParen => "(",
            Token::RightParen => ")",
            Token::LeftSquare => "[",
            Token::RightSquare => "]",
            Token::LeftBrace => "{",
            Token::RightBrace => "}",

            Token::Plus => "+",
            Token::Minus => "-",
            Token::Star => "*",
            Token::Slash => "/",
            Token::Less => "<",
            Token::Greater => ">",
            Token::LessEqual => "<=",
            Token::GreaterEqual => ">=",
            Token::Percent => "%",

            Token::Colon => ":",
            Token::SemiColon => ";",
            Token::Comma => ",",
            Token::Bang => "!",
            Token::Equal => "=",
            Token::EqualEqual => "==",
            Token::NotEqual => "!=",
            Token::AmperAmper => "&&",
            Token::Dot => ".",
            Token::EndOfFile => "EOF",

            Token::Fun => "fun",
            Token::Let => "let",
            Token::Struct => "struct",
            Token::Enum => "enum",
        };

        write!(f, "`{s}`")
    }
}
