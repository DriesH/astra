use core::fmt;

#[derive(Debug, PartialEq)]
pub enum Token {
    Illegal,

    Name { name: String },

    Int { value: String },
    Float { value: String },
    String { value: String },

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
    Assign,
    Equal,      // '=='
    NotEqual,   // '!='
    AmperAmper, // '&&'
    Dot,        // '.'
    EndOfFile,

    Fun,
    Let,
    Struct,
    Enum,
    Return,
    True,
    False,
    If,
    Else,
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
            Token::Assign => "=",
            Token::Equal => "==",
            Token::NotEqual => "!=",
            Token::AmperAmper => "&&",
            Token::Dot => ".",
            Token::EndOfFile => "EOF",

            Token::Fun => "fun",
            Token::Let => "let",
            Token::Struct => "struct",
            Token::Enum => "enum",
            Token::Return => "return",
            Token::True => "true",
            Token::False => "false",
            Token::If => "if",
            Token::Else => "else",
        };

        write!(f, "`{s}`")
    }
}
