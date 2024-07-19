mod lexer;
mod token;

use lexer::Lexer;

fn main() {
    let input = "=+(){},;:".to_string();
    let mut l = Lexer::new(input);
    l.next_token();
}
