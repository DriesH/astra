use anyhow::Result;
use parser::{lexer::Lexer, parser::Parser};
use std::io::{stdin, stdout, Write};

fn eval(input: &String) {
    let l = Lexer::new(input.to_owned());
    let p = Parser::new(l);

    let program = p.parse_program();
}

fn main() -> Result<()> {
    println!("Astra REPL: 0.0.1\n");
    println!("Press CTRL+C or type 'exit' to quit\n\n\n");

    loop {
        print!(">> ");
        stdout().flush().unwrap();
        match stdin().lines().next() {
            Some(Ok(input)) => {
                if input.trim() == "exit" {
                    println!("\nBye.");
                    break;
                }

                if input.trim().is_empty() {
                    continue;
                }

                eval(&input);
            }
            _ => {
                println!("No input given.")
            }
        }
    }

    Ok(())
}
