use std::io::{stdin, stdout, Write};
use interpreter::{Lexer, TokenType};

fn main() {
    
    println!("Hello! this is the Monkney programming language!");
    println!("Feel free to type in commands\n");

    let prefix = String::from(">>> ");
    let mut input = String::new();

    loop {
        print!("{}", prefix);
        let _ = stdout().flush();
        let _ = stdin().read_line(&mut input);

        let mut l = Lexer::new(&input);
        loop {
            let tok = l.next_token();
            match tok.ttype {
                TokenType::Eof => break,
                _ => println!("{:?}", tok)
            }
        }

        input.clear();
    }
}
