
mod data;
mod parsing;

fn main() {
    use std::io::{stdout, stdin, Write};

    loop {
        print!("> ");
        stdout().flush().unwrap();
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let x = parsing::lexer::lex(&input).unwrap();
        println!("{:?}", x);
    } 
}
